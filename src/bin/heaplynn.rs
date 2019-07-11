extern crate clap;
extern crate crossbeam;
extern crate ndarray;

use ndarray::{Array2};

use clap::{Arg, App};
use std::io::Write;

type Board = Array2<usize>;

fn binexp(m:Board,mut k:usize)->Board{

    let size:usize = *m.shape().first().unwrap();cd
    let mut res:Board = Array2::zeros((size,size));
    for i in 0..size { res[[i,i]]=1; };
    let mut x = m.clone();
    while k > 0 {
        if k % 2 == 1 {
            res = res.dot(&x);
        }
        x = x.dot(&x);
        k = k / 2; //Ganzzahlige Division (das Ergebnis wird abgerundet)
    }
    res
}


fn cellvalue(gen:&[usize],i:usize,j:usize) -> usize {
    let diff:i64 = i as i64 -j as i64;
    if gen.iter().map(|x|{x-1}).any(|x|{x as i64 ==diff}) || -1==diff { 1 } else { 0 }
}

fn heaplynn(gen:&[usize])-> usize {


    let maxgen = *gen.iter().max().unwrap();
    let mut m:Board = Board::from_elem((maxgen,maxgen),0);
    for i in 0..maxgen {
        for j in 0..maxgen {
            let newvalue = cellvalue(gen,i,j);
            m[[i,j]]=newvalue;
        }
    }
    println!("Generating set {:?}",gen);
    //println!("Matrix \n{:?}",m);
    let mut fertig = false;
    let mut a:Board = m.clone();
    let mut potenz = 1;
    while !fertig {
        a = a.dot(&m);
        potenz += 1;
        fertig = !a.iter().any(|x|{*x==0});
        //println!("Matrix^{}\n {:?}",potenz,a);
    }
    assert_eq!(a,binexp(m,potenz));
    println!("      Erste-Non-Null-Potenz {:4} berechnet f zu {:8}",potenz,potenz-maxgen);
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

            let lynn_frob = heaplynn(gens);

            let saturated:bool = lynn_frob + 1 <= full[end_slice];

            let ausgabe = format!("{:5};{:5};{:8};{:8};{:8};{:8};{:8};{:8};\n",
                                  modul, residue,
                                  begin_slice, end_slice,
                                  full[begin_slice], full[end_slice - 1],
                                  lynn_frob,
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
