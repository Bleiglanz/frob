extern crate clap;
extern crate crossbeam;

use clap::Parser;
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
