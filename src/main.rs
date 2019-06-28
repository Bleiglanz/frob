extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;


fn prepare_generators(modul: usize, residue: usize, allgen:&[usize]) -> Vec<usize>{

    allgen.iter().filter(|x| { residue == *x % modul }).map(|x|{*x}).collect()

}

fn slice_generators(skip:usize, factor1: usize, factor2: usize, iterate: bool, allgen:&[usize])->(usize,usize,usize){

    fn findindex(s: &[usize], start: usize, factor: usize) -> usize {
        if 1 == factor {
            start
        } else {
            let mut max = start;
            loop {
                if s[max] < factor * s[start] {
                    max = max + 1;
                } else {
                    break;
                }
            }
            max
        }
    }

    let minindex: usize = findindex(allgen, skip, factor1);

    let maxindex: usize = findindex(allgen, skip, factor2) + 1;

    let startindex: usize = if iterate { minindex + 1 } else { maxindex - 1 };

    (startindex,minindex,maxindex)
}



fn frobenius(modul: usize, residue: usize, start: usize, stop: usize, factor1: usize, factor2: usize, iterate: bool) {

    //let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();

    let raw:Vec<usize> = (1..8000000).map(|x|{x*x*x}).collect();

    let full = prepare_generators(modul,residue,&raw);

    let mut out = std::fs::File::create(format!("./outsat_mod{}residue{},{}pto{}p_n{}to{}.csv", modul, residue, factor1, factor2, start, stop)).expect("Unable to create file");
    let head = "       n;     n+k;       k;modul; resi;fak1;fak2;     p_n;    p_n+1;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-..m(S); stable; f/p\n";
    out.write_all(head.as_bytes()).expect("head?");

    print!("{}", head);

    for skip in start..stop {

        let (startindex,minindex,maxindex) = slice_generators(skip,factor1,factor2,iterate,&full);

        for i in startindex..maxindex {

            let gens: &[usize] = &full[minindex..i];

            if gens.len() < 2 { continue; };

            let res2: Fast = { fast(&gens) };

            let ausgabe = format!("{:8};{:8};{:8};{:5};{:5};{:4};{:4};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:.6}\n",
                                  skip + 1, i, i - (skip + 1),
                                  modul, residue,
                                  factor1, factor2,
                                  full[minindex], full[minindex + 1], full[i - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.distance_to_f_over_m(),
                                  if res2.f() + 1 <= full[i] { "saturated S" } else { "" },
                                  res2.f_over_m(),
            );

            print!("{}", ausgabe);

            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            if res2.f() + 1 <= full[i] {
                break;
            }
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
        .arg(Arg::with_name("factor1")
            .help("take all primes as generators factor1*p_start <= gen  < factor2*p_start")
            .required(true)
            .default_value("1")
        )
        .arg(Arg::with_name("factor2")
            .help("take all primes as generators factor1*p_start <= gen  < factor2*p_start")
            .required(true)
            .default_value("6")
        )
        .arg(Arg::with_name("iterate")
            .help("if 1, take all intermediate semigroups p_n....p_n+k")
            .required(true)
            .default_value("0")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    let factor1: usize = matches.value_of("factor1").unwrap().parse().unwrap();
    let factor2: usize = matches.value_of("factor2").unwrap().parse().unwrap();

    let iterate: usize = matches.value_of("iterate").unwrap().parse().unwrap();

    frobenius(modul, residue, start, stop, factor1, factor2, iterate != 0);
}
