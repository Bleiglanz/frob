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
    let full = prepare_generators(modul,residue,&raw);
    let mut out = std::fs::File::create(format!("./stabioutsat_mod{}residue{},p_n{}to{}.csv", modul, residue, start, stop)).expect("Unable to create file");
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-..m(S); stable;    f/p;delta(f)\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);


    for begin_slice in start..stop {

        let mut frobenius = full[begin_slice]*full[begin_slice+1]-full[begin_slice]-full[begin_slice+1];

        let frobenius_null = frobenius.clone();

        let mut end_slice = begin_slice + 3;

        let first:usize = full[begin_slice];

        let stable_max_index = full.iter().skip(start).enumerate().take_while(|(_i,v)|{*v <= &(first*6)}).map(|(i,_v)|{i}).last().unwrap();
        let stable_max_semi = fast(&full[begin_slice..stable_max_index]);

        loop {

            let gens: &[usize] = &full[begin_slice..end_slice];

            assert!(gens.len()>2);

            let res2: Fast = { fast(&gens) };

            let saturated:bool = res2.f() + 1 <= full[end_slice];

            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:9};{:9};{:.8};{:8}:{:8}\n",
                                  modul, residue,
                                  begin_slice, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.distance_to_f_over_m(),
                                  if saturated { "saturated " } else { "          " },
                                  res2.f_over_m(),
                                  if 0==(frobenius_null-res2.f())%res2.m() {"GOOD"} else {""},
                                  if 0==(res2.f()-stable_max_semi.f())%res2.m() {"NICE"} else {"STRUGGLE"}
            );

            print!("{}", ausgabe);

            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            if saturated {
                assert_eq!(stable_max_semi.f(),res2.f(),"stable frobenius = saturated");
                break;
            }
            frobenius = res2.f();
            end_slice +=1//if end_slice > 10 {end_slice/10} else {10}; // wächst um 10%
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
            .help("where to begin, a n th prime after the filter")
            .required(true)
            .default_value("10")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime after the filter")
            .required(true)
            .default_value("12")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    assert!(start>=1,"startindex must be bigger than 2, because <2,3> is trivial");

    frobenius(modul, residue, start - 1usize, stop);
}
