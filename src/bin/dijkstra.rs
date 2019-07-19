extern crate clap;
extern crate crossbeam;
extern crate ndarray;

use ndarray::{Array2};

use clap::{Arg, App};
use std::io::Write;

fn cellvalue(gen:&[usize],i:usize,j:usize) -> usize {
    let diff:i64 = i as i64 -j as i64;
    if gen.iter().map(|x|{x-1}).any(|x|{x as i64 ==diff}) || -1==diff { 1 } else { 0 }
}

fn dijkstra(gen:&[usize])-> usize {
    println!("Dijkstra...");

    let maxgen = *gen.iter().max().unwrap();
    use frob::modules::fast_semigroup::fast;
    use frob::modules::Semigroup;
    fast(gen).f()
}

fn prepare_generators(modul: usize, residue: usize, allgen:&[usize]) -> Vec<usize>{
    allgen.iter().filter(|x| { residue == *x % modul }).map(|x|{*x}).collect()
}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();
    let full = prepare_generators(modul,residue,&raw);

    let mut out = std::fs::File::create(format!("./heaplynnoutsat_mod{}residue{},p_n{}to{}.csv", modul, residue, start, stop)).expect("Unable to create file");
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k; f(S)\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);

    for begin_slice in start..stop {

        let mut end_slice = begin_slice + 3;

        loop {

            let gens: &[usize] = &full[begin_slice..end_slice];

            let dijk_frob = dijkstra(gens);

            let saturated:bool = dijk_frob + 1 <= full[end_slice];

            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};\n",
                                  modul, residue,
                                  begin_slice, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  dijk_frob,
                                  if saturated { "saturated " } else { "          " },
            );
            print!("{} \n\n", ausgabe);
            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            if saturated {
                break;
            }
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
            .default_value("2")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime after the filter")
            .required(true)
            .default_value("5")
        )
        .get_matches();

    let modul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let residue: usize = matches.value_of("residue").unwrap().parse().unwrap();

    let start: usize = matches.value_of("start").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();

    assert!(start>=1,"startindex must be bigger than 2, because <2,3> is trivial");

    frobenius(modul, residue, start - 1usize, stop);
}
