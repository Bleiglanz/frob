use super::Semigroup;

#[derive(Debug, Clone)]
pub struct Fast {
    pub max_a: usize,
    pub sum_a: usize,
    pub g1: usize,
    pub count_set: usize,
    pub count_gap: usize,
    pub e: usize,
}

impl Semigroup for Fast {
    fn e(&self) -> usize { self.e }
    fn f(&self) -> usize { self.max_a - self.g1 }
    fn c(&self) -> usize { self.max_a - self.g1 +1 }
    fn m(&self) -> usize { self.g1 }
    fn max_apery(&self)-> usize { self.max_a }
    fn sum_apery(&self)-> usize { self.sum_a }
    fn count_set(&self) -> usize { self.count_set }
    fn count_gap(&self) -> usize { self.count_gap }
}


impl Fast {
    fn new(count_set: usize, max_a: usize, g1: usize, mingencount:usize, sum:usize) -> Self {
        let count_gap = (sum - ((g1 - 1) * g1) / 2) / g1;
        Fast {
            max_a,
            sum_a: sum,
            g1,
            count_set,
            count_gap,
            e: mingencount,
        }
    }
}


pub fn fast(inputnumbers: &[usize]) -> Fast {

    // nicht nötig wenn nur primzahlen richtig sortiert reinkommen
    // teilerfremd machen und sortieren
    // let d = gcd_vec(inputnumbers);
    // let mut input: Vec<usize> = inputnumbers.iter().map(|x| (x / d) as usize).collect();
    // input.sort();
    let maximal_input: usize = *inputnumbers.last().unwrap();
    let width=2*maximal_input;
    let m: usize = *inputnumbers.first().unwrap();
    let mut aperyset: Vec<usize> = vec![0; m];
    let mut count_set = 1usize; // 0 schon dabei!
    let mut window = vec![-1isize; width]; // fenster hat die länge 2max
    let mut i: usize = m; // startindex
    let mut windowindex = m; // am anfang = i
    let mut runlength = 1usize; // anzahl aufeinanderfoldender hits
    let mut hit: bool = false;
    let mut max_apery:usize = m;
    let mut sum_apery:usize = 0;
    let mut minimal_generators:usize = 1;
    window[0]=0;
    while runlength < m {
        let residue = i % m;
        if 0 == residue {
            count_set += 1;
            runlength += 1;
            hit = true;
            window[windowindex] = i as isize;
        } else if aperyset[residue]>0 && i > aperyset[residue] {
            count_set += 1;
            runlength += 1;
            hit = true;
            window[windowindex] = i as isize;
        }
        else {
            for k in inputnumbers[1..].iter() {
                if windowindex >= *k && window[windowindex - k] >= 0 {
                    count_set += 1;
                    runlength += 1;
                    hit = true;
                    window[windowindex] = i as isize;
                    aperyset[residue] = i;
                    sum_apery+=i;
                    if i>max_apery { max_apery = i}
                    if 0==window[windowindex - k] {
                        minimal_generators+=1;
                    }
                    break;
                }
            }
        }
        if !hit { runlength = 0 };
        hit = false;
        i += 1;
        if windowindex == width - 1 {
            for j in 0..maximal_input {
                window[j] = window[j + maximal_input];
            }
            windowindex = maximal_input;
        } else {
            windowindex += 1;
        }
    }

    Fast::new(count_set-m, max_apery, m, minimal_generators, sum_apery)
}
