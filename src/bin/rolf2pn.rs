extern crate clap;
extern crate crossbeam;

use clap::Parser;
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

    for j in 0..1 {
        let a: usize = 1 * a_start + j * b_start;
        let b: usize = 1 * b_start;
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
    let afak: usize = 33;
    let bfak: usize = 33;
    frobenius(args.module, args.residue, args.start, args.stop, afak, bfak);
}
