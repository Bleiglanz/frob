extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;


fn prepare_generators(modul: usize, residue: usize, allgen:&[usize]) -> Vec<usize>{

    allgen.iter().filter(|x| { residue == *x % modul }).map(|x|{*x}).collect()

}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();

    //let raw:Vec<usize> = (1..8000000).map(|x|{(x*(3*x-1))/2}).collect();

    let full = prepare_generators(modul,residue,&raw);

    let mut out = std::fs::File::create(format!("./outsat_mod{}residue{},p_n{}to{}.csv", modul, residue, start, stop)).expect("Unable to create file");
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-..m(S); stable; f/p\n";
    out.write_all(head.as_bytes()).expect("head?");

    print!("{}", head);

    let mut end_slice=start+3;

    for begin_slice in start..stop {

        loop {

            let gens: &[usize] = &full[begin_slice..end_slice];

            if gens.len() < 3 { continue; };

            let res2: Fast = { fast(&gens) };

            let saturated:bool = res2.f() + 1 <= full[end_slice];

            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:.6}\n",
                                  modul, residue,
                                  begin_slice, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.distance_to_f_over_m(),
                                  if saturated { "saturated S" } else { "          " },
                                  res2.f_over_m(),
            );


            print!("{}", ausgabe);

            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            if saturated {
                break;
            }
            end_slice+=1;
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
            .default_value("10")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime")
            .required(true)
            .default_value("12")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    frobenius(modul, residue, start, stop);
}
