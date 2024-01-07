extern crate clap;
extern crate crossbeam;

use clap::Parser;
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;
use frob::modules::wilf::{WilfSet, generatewilf};


fn prepare_generators(modul: usize, residue: usize, allgen:&[usize], start:usize) -> Vec<usize>{
    let primes :Vec<usize> = allgen.iter().filter(|x| { (residue == *x % modul) && (start <= **x)}).map(|x|{*x}).collect();
    let mut gens  = if primes[0]==start { primes } else {
        let mut f = vec![start];
        for x in primes {
            f.push(x);
        }
        f
    };
    gens
}

fn frobenius(modul: usize, residue: usize, start: usize, stop: usize) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();

    let mut out = std::fs::File::create(format!("./outsat_mod{}residue{},p_n{}to{}.csv", modul, residue, start, stop)).expect("Unable to create file");
    let head = "modul; resi;begin_slice;end_slice; p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S);   stable;    u(S); fne(S);   sum(A); d_min; 2g_n - fn\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);


    for x in start..stop {
        let full = prepare_generators(modul,residue,&raw,x);

        loop {
            let mut end_slice = stop;

            let gens : &[usize] = &full[0..end_slice];

            //dbg!(gens);

            if gens.len() <= 1 { continue; };

            let res2: Fast = { fast(&gens) };

            let saturated:bool = res2.f() + 1 <= full[end_slice];

            let max_even_gap = res2.max_even_gap();

            let g_n = res2.count_gap;

            let d_min = (2*res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());

            //let pi3p = raw.iter().filter(|x|{**x<=(3*res2.m())}).count();
            assert_eq!(res2.f()+res2.m(),res2.max_apery(),"max apery ist fn+p");
            assert_eq!(0, res2.f()+d_min +1 - 2*g_n);
            assert_eq!(d_min,res2.count_gap - res2.count_set);
            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:8};{:8};{:8};{:8};{:8.2}\n",
                                  modul, residue,
                                  0, end_slice,
                                  full[0], full[end_slice - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.f() as i64 -3*res2.m() as i64,
                                  if saturated { "saturated" } else { "         " },
                                  res2.u, max_even_gap,
                                  res2.sum_a, d_min, res2.f() as f64/res2.m() as f64);
            print!("{}", ausgabe);
            if saturated {
                //out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            }
            if saturated {
               let mut erzeuger = res2.genset.clone().to_vec();
                erzeuger.sort();
                assert_eq!(erzeuger.len(),res2.e);
                println!("Semigroup S_{} hat f={}, p={}, Lücken {},  \n  Erzeuger={:?}, \n  Apery={:?}",
                         x,
                         res2.f(),
                         res2.m(),
                         res2.count_gap(),
                         erzeuger, res2.apery);

                out.write_all(res2.m().to_string().as_bytes());
                out.write_all(";".as_bytes()).unwrap();
                for x in 0usize..1000usize {
                    //print!("{:3};",hoehe(&res2,x));
                    let column = format!("{:3};",hoehe(&res2,x));
                    out.write_all(column.as_bytes()).unwrap();
                }
                out.write("\n".as_bytes());
                println!();
                break;
            }
            end_slice+=end_slice/10; // wächst um 10%
        }
    }
}

fn hoehe(s: &Fast, x: usize) -> i64 {
    let i = x % s.m();
    let apery = s.apery[i] as i64;
    (x as i64 - apery) / (s.m() as i64)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value="1")]
    module: usize,
    #[arg(short, long, default_value="0")]
    residue: usize,
    #[arg(short, long, default_value="2")]
    start: usize,
    #[arg(long, default_value="50")]
    stop: usize,
}

fn main() {
    let args = Args::parse();
    frobenius(args.module, args.residue, args.start, args.stop);
}
