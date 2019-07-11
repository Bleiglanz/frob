extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;
use frob::modules::math::gcd;
use std::fs::File;


fn prepare_generators(modul: usize, residue: usize, allgen:&[usize]) -> Vec<usize>{

    allgen.iter().filter(|x| { residue == *x % modul }).map(|x|{*x}).collect()

}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize, out:&mut File) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();
    let full = prepare_generators(modul,residue,&raw);
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-..m(S); stable; f/p\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);
    let mut end_slice=start+20;
    let mut begin_slice = start;
    let mut prime_lt_stop = true;
    while prime_lt_stop {

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
                                  res2.f_over_m()
                                  ,
            );
            if saturated {
                print!("{}", ausgabe);
                out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
                begin_slice += 1;
                break;
            }
            end_slice+=end_slice/10; // wächst um 10%
            prime_lt_stop = full[begin_slice] <= stop;
        }

    }
}

fn main() {
    let matches = App::new("semiprog")
        .version("0.0")
        .author("Anton Rechenauer")
        .about("compute frobenius")
        .arg(Arg::with_name("modul")
            .help("the max modulus, in which arithmetic progression to search")
            .required(true)
            .default_value("2")
        )
        .arg(Arg::with_name("stop")
            .help("where to stop, a number the n th prime shall be below")
            .required(true)
            .default_value("12")
        )
        .get_matches();

    let maxmodul: usize = matches.value_of("modul").unwrap().parse().unwrap();
    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();
    let filename = format!("sample_semigroups_maxmod{}_upto{}.csv",maxmodul,stop);
    let mut out = std::fs::File::create(filename).expect("Unable to create file");
    for m in 2..maxmodul+1 {
        for a in 1..m {
            if 1==gcd(a,m) {
                frobenius(m, a, 1, stop, &mut out);
            }
        }
    }



}
