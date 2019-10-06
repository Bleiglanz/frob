use super::Semigroup;

#[derive(Debug, Clone)]
pub struct Fast {
    apery: Vec<usize>,
    pub max_a: usize,
    pub sum_a: usize,
    pub g1: usize,
    pub count_set: usize,
    pub count_gap: usize,
    pub e: usize,
    pub u: usize,
}

impl Semigroup for Fast {
    fn e(&self) -> usize { self.e }
    fn f(&self) -> usize { self.max_a - self.g1 }
    fn c(&self) -> usize { self.max_a - self.g1 +1 }
    fn m(&self) -> usize { self.g1 }
    fn u(&self) -> usize { self.u }
    fn max_apery(&self)-> usize { self.max_a }
    fn sum_apery(&self)-> usize { self.sum_a }
    fn count_set(&self) -> usize { self.count_set }
    fn count_gap(&self) -> usize { self.count_gap }
    fn contains(&self,n:usize) -> bool { self.apery[n % self.g1] <= n }
    fn max_even_gap(&self) -> usize {
        let mut meg = 0;
        for k in 0..self.c() {
           meg = if !self.contains(k) && 0==k%2 { k } else { meg }
        }
        meg
    }
    fn aquer(&self)->usize { self.sum_a / self.g1 }
}


impl Fast {
    fn new(count_set: usize, max_a: usize, g1: usize, mingencount:usize, sum:usize, apery:Vec<usize>, u:usize) -> Self {
        let count_gap = (sum - ((g1 - 1) * g1) / 2) / g1;
        Fast {
            max_a,
            sum_a: sum,
            g1,
            count_set,
            count_gap,
            e: mingencount,
            apery,
            u
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
    let mut max_atom = m;
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
                        if max_atom < i {max_atom=i};
                    }
                    break;
                }
            }
        }
        if !hit { runlength = 0 };
        hit = false;
        i += 1;
        if windowindex == width - 1 {
            //for j in 0..maximal_input {
            //    window[j] = window[j + maximal_input];
            //}
            //copy_within_a_slice(&mut window,maximal_input, 0, maximal_input);
            let (dst, src) = window.split_at_mut(maximal_input);
            dst[0..maximal_input].clone_from_slice(&src[..maximal_input]);
            windowindex = maximal_input;
        } else {
            windowindex += 1;
        }
    }
    Fast::new(count_set-m, max_apery, m, minimal_generators, sum_apery,aperyset,max_atom)
}

//fn copy_within_a_slice<T: Clone>(v: &mut [T], from: usize, to: usize, len: usize) {
//    if from > to {
//        let (dst, src) = v.split_at_mut(from);
//        dst[to..to + len].clone_from_slice(&src[..len]);
//    } else {
//        let (src, dst) = v.split_at_mut(to);
//        dst[..len].clone_from_slice(&src[from..from + len]);
//    }
//}