extern crate clap;
extern crate crossbeam;

use clap::Parser;
use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;
use frob::modules::wilf::{WilfSet, generatewilf};


fn prepare_generators(allgen:&[usize],power:usize) -> Vec<usize>{
    allgen.iter().
           map(|x|{
               let mut res:usize = 1;
               for _i in 0..power {
                   res = res*(*x);
               }
               res
           }).collect()
}

fn frobenius(start: usize, stop: usize, power:usize) {

    let raw:Vec<usize>= (1..8000000).collect();
    let full = prepare_generators(&raw,power);
    let mut out = std::fs::File::create(format!("./schritt_waring_power_{}_n{}to{}.csv", power, start, stop)).expect("Unable to create file");
    let head = "begin_slice;end_slice; x_n;   x_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S);   stable;    u(S); fne(S);   sum(A)\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);


    for begin_slice in start..stop {

        let mut end_slice=begin_slice+10;

        loop {

            let gens: &[usize] = &full[begin_slice..end_slice];

            if gens.len() < 3 { continue; };

            let res2: Fast = { fast(&gens) };

            let saturated:bool = res2.f() + 1 <= full[end_slice];

            let max_even_gap = res2.max_even_gap();

            let g_n = res2.count_gap;

            let d_min = (2*res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());

            //let pi3p = raw.iter().filter(|x|{**x<=(3*res2.m())}).count();
            assert_eq!(res2.f()+res2.m(),res2.max_apery(),"max apery ist fn+p");
            assert_eq!(0, res2.f()+d_min +1 - 2*g_n);
            assert_eq!(d_min,res2.count_gap - res2.count_set);
            let ausgabe = format!("{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:8};{:8};{:8}\n",
                                  begin_slice, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  res2.m(), res2.e(),
                                  res2.count_set, res2.f(), res2.f() as i64 -3*res2.m() as i64,
                                  if saturated { "saturated" } else { "         " },
                                  res2.u, max_even_gap,
                                  res2.sum_a);

            print!("{}", ausgabe);
            //let ws:WilfSet = generatewilf(&gens);
            //println!("Wilf-e {} neu-e {}",ws.e,res2.e());
            //let html_title = &format!("./{}_Primzahl_{}.html",begin_slice+2,res2.m());
            //let mut outhtml = std::fs::File::create(html_title).expect("Unable to create file");
            //outhtml.write_all(ws.to_html(html_title).as_bytes()).expect("html?");
            if res2.f_over_m() < 20f64 {
                out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
            }
            if saturated {
                break;
            }
            end_slice += 1 ;
        }
    }
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value="2")]
    power:usize,
    #[arg(short, long, default_value="2")]
    start: usize,
    #[arg(short, long, default_value="10")]
    stop: usize,
}

fn main() {
    let args = Args::parse();
    frobenius(args.start, args.stop, args.power);
}
