extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;
use frob::modules::wilf::{WilfSet, generatewilf};


fn prepare_generators(modul: usize, residue: usize, allgen: &[usize]) -> Vec<usize> {
    allgen.iter().filter(|x| { residue == *x % modul }).map(|x| { *x }).collect()
}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize, a_start: usize, b_start: usize) {
    let raw: Vec<usize> = primal::Primes::all().take(8000000).collect();

    let full = prepare_generators(modul, residue, &raw);

    let mut out = std::fs::File::create(format!("./fuer2pn_runner_mod{}residue{}_lt{}div{}_nfrom{}to{}.csv", modul, residue, a_start, b_start, start, stop)).expect("Unable to create file");
    let head = "a      ;b     ;modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S);   stable;    u(S); fne(S);   pi((a/b)p_n)\n";
    out.write_all(head.as_bytes()).expect("head?");

    print!("{}", head);

    for j in 0..20 {
        let a: usize = 20 * a_start + j * b_start;
        let b: usize = 20 * b_start;
        for begin_slice in start..stop {
            let mut end_slice = begin_slice;
            let p = full[begin_slice];
            loop {
                if full[end_slice + 1] * b < a * p {
                    end_slice += 1;
                } else {
                    break;
                }
                if end_slice > 10000000 { panic!("overflow") };
            }
            let pi_a_b_pn = raw.iter().filter(|x| { **x * b < a * p }).count();
            let gens: &[usize] = &full[begin_slice..end_slice + 1];
            let res2: Fast = { fast(&gens) };
            let saturated: bool = res2.f() + 1 <= full[end_slice + 1];
            let max_even_gap = res2.max_even_gap();
            let g_n = res2.count_gap;
            let d_min = (2 * res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());

            //let pi3p = raw.iter().filter(|x|{**x<=(3*res2.m())}).count();
            assert_eq!(res2.f() + res2.m(), res2.max_apery(), "max apery ist fn+p");
            assert_eq!(0, res2.f() + d_min + 1 - 2 * g_n);
            assert_eq!(d_min, res2.count_gap - res2.count_set);
            let ausgabe = format!("{:8};{:8};{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:8};{:8};{:8}\n",
                                  a,b,modul, residue,
                                  begin_slice + 1, end_slice + 1,
                                  full[begin_slice], full[end_slice],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.f() as i64 - 3 * res2.m() as i64,
                                  if saturated { "saturated" } else { "         " },
                                  res2.u, max_even_gap,
                                  pi_a_b_pn);


            print!("{}", ausgabe);
            //let ws:WilfSet = generatewilf(&gens);
            //println!("Wilf-e {} neu-e {}",ws.e,res2.e());
            //let html_title = &format!("./{}_Primzahl_{}.html",begin_slice+2,res2.m());
            //let mut outhtml = std::fs::File::create(html_title).expect("Unable to create file");
            //outhtml.write_all(ws.to_html(html_title).as_bytes()).expect("html?");

            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
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
            .default_value("1")
        )
        .arg(Arg::with_name("residue")
            .help("the residue, consider only primes congruent this mod modul")
            .required(true)
            .default_value("0")
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
        .arg(Arg::with_name("afak")
            .help("a for search all q st q < (a/b) p_n")
            .required(true)
            .default_value("2")
        )
        .arg(Arg::with_name("bfak")
            .help("b for search all q st q < (a/b) p_n")
            .required(true)
            .default_value("1")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    let afak: usize = matches.value_of("afak").unwrap().parse().unwrap();
    let bfak: usize = matches.value_of("bfak").unwrap().parse().unwrap();

    frobenius(modul, residue, start - 1, stop - 1, afak, bfak);
}
