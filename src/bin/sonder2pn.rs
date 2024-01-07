extern crate crossbeam;

use frob::modules::fast_semigroup::{Fast, fast};
use frob::modules::Semigroup;
use std::io::Write;

fn prepare_generators(modul: usize, residue: usize, allgen: &[usize]) -> Vec<usize> {
    allgen.iter().filter(|x| { residue == *x % modul }).map(|x| { *x }).collect()
}

fn frobenius(modul: usize, residue: usize, start: usize) {
    let raw: Vec<usize> = primal::Primes::all().take(8000000).collect();
    let full = prepare_generators(modul, residue, &raw);
    let mut out = std::fs::File::create(format!("./ausreisserfuer2pn_mn{}.csv", start)).expect("Unable to create file");
    let head = "  n  ;     p_n;   p_n+k;    m(S);    e(S);  #(S<F);    f(S);f(S)-3m(S);   stable;    u(S);  fne(S);pi(2p_n); e_n*s_n-c;  anzerz;erz\n";
    out.write_all(head.as_bytes()).expect("head?");
    print!("{}", head);

    let begin_slice = start;
    let mut end_slice = begin_slice;
    let p = full[begin_slice];
    loop {
        if full[end_slice + 1] < 2 * p {
            end_slice += 1;
        } else {
            break;
        }
    }

    let pi_2pn = raw.iter().filter(|x| { **x < 2 * p }).count();
    let gens0: &[usize] = &full[begin_slice..end_slice + 1];
    let res0: Fast = { fast(&gens0) };
    let mut menge: Vec<usize> = Vec::new();
    let mut i = 1;
    while full[end_slice + i] < res0.f() {
        menge.push(full[end_slice + i]);
        i += 1;
    }
    let power = powerset(&menge);
    for neue in power {
        let mut gens: Vec<usize> = gens0.to_vec().clone();
        for x in neue {
            gens.push(x);
        }
        gens.sort();

        //println!("---versuch {:?}", gens);
        //println!("start {} end_slice {} prime p {}; pi(2pn) {} gen {:?} menge{:?}", start, end_slice, p, pi_2pn, gens0, menge);
        //println!("--");

        let res2: Fast = { fast(&gens) };


        let saturated: bool = res2.f() + 1 <= full[end_slice + 1];

        let max_even_gap = res2.max_even_gap();

        let g_n = res2.count_gap;

        let d_min = (2 * res2.sum_apery() / (res2.m())) - (res2.f() + res2.m());

        assert_eq!(res2.f() + res2.m(), res2.max_apery(), "max apery ist fn+p");
        assert_eq!(0, res2.f() + d_min + 1 - 2 * g_n);
        assert_eq!(d_min, res2.count_gap - res2.count_set);
        let ausgabe = format!("{:5};{:8};{:8};{:8};{:8};{:8};{:8};{:10};{};{:8};{:8};{:8};{:10};{:8};{:?}\n",
                              begin_slice + 2,
                              full[begin_slice], full[end_slice],
                              res2.m(), res2.e(),
                              res2.count_set, res2.f(), res2.f() as i64 - 3 * res2.m() as i64,
                              if saturated { "saturated" } else { "         " },
                              res2.u, max_even_gap,
                              pi_2pn, res2.e()*res2.count_set - res2.c() ,gens.len(),gens);

        print!("{}", ausgabe);
        out.write_all(ausgabe.as_bytes()).expect("ausgabe??");
    }
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}

fn main() {
    let ns: Vec<usize> = vec![8, 9, 10, 11, 15];
    for n in ns {
        frobenius(1, 0, n - 1);
    }
}
