extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;
use frob::modules::wilf::{WilfSet, generatewilf};


fn prepare_generators(modul: usize, residue: usize, allgen:&[usize]) -> Vec<usize>{
    allgen.iter().filter(|x| { residue == *x % modul }).map(|x|{*x}).collect()
}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();
    let full = prepare_generators(modul,residue,&raw);
    let mut out = std::fs::File::create(format!("./schrittw_mod{}residue{},p_n{}to{}.csv", modul, residue, start, stop)).expect("Unable to create file");
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S);   stable;    u(S); fne(S);   sum(A)\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);


    for begin_slice in start..stop {

        let mut end_slice=begin_slice+10;

        loop {

            let gens: &[usize] = &full[begin_slice..end_slice];

            if gens.len() < 3 { continue; };

            let res2: Fast = { fast(&gens) };

            let saturated:bool = res2.f() + 1 <= full[end_slice];

            let max_even_gap = res2.max_even_gap();

            let g_n = res2.count_gap;

            let d_min = (2*res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());

            //let pi3p = raw.iter().filter(|x|{**x<=(3*res2.m())}).count();
            assert_eq!(res2.f()+res2.m(),res2.max_apery(),"max apery ist fn+p");
            assert_eq!(0, res2.f()+d_min +1 - 2*g_n);
            assert_eq!(d_min,res2.count_gap - res2.count_set);
            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:8};{:8};{:8}\n",
                                  modul, residue,
                                  begin_slice+2, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.f() as i64 -3*res2.m() as i64,
                                  if saturated { "saturated" } else { "         " },
                                  res2.u, max_even_gap,
                                  res2.sum_a);

            print!("{}", ausgabe);
            //let ws:WilfSet = generatewilf(&gens);
            //println!("Wilf-e {} neu-e {}",ws.e,res2.e());
            //let html_title = &format!("./{}_Primzahl_{}.html",begin_slice+2,res2.m());
            //let mut outhtml = std::fs::File::create(html_title).expect("Unable to create file");
            //outhtml.write_all(ws.to_html(html_title).as_bytes()).expect("html?");
            if res2.f_over_m() < 20f64 {
                out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            }
            if saturated {
                break;
            }
            end_slice += 1 ;
        }
    }
}

fn main() {
    let matches = App::new("semiprog")
        .version("0.0")
        .author("Anton Rechenauer")
        .about("compute frobenius")
        .arg(Arg::with_name("modul")
            .help("the modulus, in which arithmetic progression to search")
            .required(true)
            .default_value("2")
        )
        .arg(Arg::with_name("residue")
            .help("the residue, consider only primes congruent this mod modul")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("start")
            .help("where to begin, a n th prime")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime")
            .required(true)
            .default_value("10")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    frobenius(modul, residue, start, stop);
}
