extern crate clap;
extern crate crossbeam;

use clap::{Arg, App};
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;

fn t4list(max:usize)->Vec<usize>{
    let mut res:Vec<usize> = Vec::with_capacity(200000);
    let mut zaehler = 0;
    for t in 286807..u64::max_value() as u64 {
        if primal::is_prime(1+4*t) && primal::is_prime(3+4*t) && primal::is_prime(1+6*t){
            res.push(t as usize);
            zaehler+=1;
            println!("teste t={} von max={}=",t,max);
        }
        if zaehler>=max { break; }
    }
    res
}


fn frobenius(ts:Vec<usize>) {

    let raw:Vec<usize>= primal::Primes::all().take(8000000).collect();
    let mut out = std::fs::File::create(format!("./schrittw_t4_frage_nach_prim_{}.csv",ts.len())).expect("Unable to create file");
    let head = "      t; erste;   letzte;    m(S);     e(S);    f(S); f(S)/m(S) ; size ; <5.5; 6t-1prim?; 4t-1prim?; 4t-3prim? \n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);

    for t in ts {

        let mut full:Vec<usize> = Vec::with_capacity(20000);
        let mut i = 0;
        let first = 4*t + 1;
        let mut last : usize =0;
        loop{
            let p = raw[i];
            if p>=4*t+1 && 2*p<=3*(4*t+1) {
                full.push(p);
            }
            if 2*p<=3*(4*t+1) && 2*raw[i+1]>3*(4*t+1) { last = p; }
            if 2*p>3*(4*t+1) { break; }
            i=i+1;
        }
        let gens:&[usize] = &full;
        if gens.len() < 2 { continue; };
        let res2: Fast = { fast(&gens) };
        let max_even_gap = res2.max_even_gap();
        let g_n = res2.count_gap;
        let d_min = (2*res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());
        assert_eq!(res2.f()+res2.m(),res2.max_apery(),"max apery ist fn+p");
        assert_eq!(0, res2.f()+d_min +1 - 2*g_n);
        assert_eq!(d_min,res2.count_gap - res2.count_set);
        let state = if (res2.f() as f64 / res2.m() as f64) < 5.5 {
            "AUSREISSER"
        } else {
            ""
        };
        let sixtm1 = if primal::is_prime(6*t as u64 -1){
            "6t-1 prim"
        } else {
            ""
        };
        let viertm1 = if primal::is_prime(4*t as u64 -1){
            "4t-1 prim"
        } else {
            ""
        };
        let viertm3 = if primal::is_prime(4*t as u64 -3){
            "4t-3 prim"
        } else {
            ""
        };


        let ausgabe = format!("{:8};{:8};{:8};{:8};{:8};{:8};{:1.8};{:5};{};{};{};{}\n",
                              t,
                              first, last,
                              res2.m(), res2.e(),
                              res2.f(), res2.f() as f64 / res2.m() as f64, full.len(), state, sixtm1,viertm1,viertm3);

        print!("{}", ausgabe);
        if res2.f_over_m() < 20f64 {
            out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
        }
    }
}

fn main() {
    let matches = App::new("semiprog")
        .version("0.0")
        .author("Anton Rechenauer")
        .about("compute frobenius")
        .arg(Arg::with_name("stop")
            .help("where to stop, a n th prime")
            .required(true)
            .default_value("10")
        )
        .get_matches();

    let stop: usize = matches.value_of("stop").unwrap().parse().unwrap();
    let ts = t4list(stop);
    println!("t4 list bis {} ist {:?}",stop,ts);
    frobenius( ts );
}
