extern crate clap;
extern crate crossbeam;
extern crate ndarray;

use ndarray::{Array2};

use clap::Parser;
use std::io::Write;

fn cellvalue(gen:&[usize],i:usize,j:usize) -> usize {
    let diff:i64 = i as i64 -j as i64;
    if gen.iter().map(|x|{x-1}).any(|x|{x as i64 ==diff}) || -1==diff { 1 } else { 0 }
}

fn dijkstra(gen:&[usize])-> usize {
    println!("Dijkstra...nie implementiert??");

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value="2")]
    module: usize,
    #[arg(short, long, default_value="1")]
    residue: usize,
    #[arg(short, long, default_value="2")]
    start: usize,
    #[arg(short, long, default_value="10")]
    stop: usize,
}

fn main() {
    let args = Args::parse();
    frobenius(args.module, args.residue, args.start, args.stop);
}

