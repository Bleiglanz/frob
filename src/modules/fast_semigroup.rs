use super::Semigroup;
use crate::modules::math::gcd_vec;

#[derive(Debug, Clone)]
pub struct Fast {
    pub apery: Vec<usize>,
    pub max_a: usize,
    pub sum_a: usize,
    pub g1: usize,
    pub count_set: usize,
    pub count_gap: usize,
    pub e: usize,
    pub u: usize,
    pub genset:Vec<usize>,
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
    fn genset(&self)->Vec<usize> { self.genset.clone() }
}


impl Fast {
    fn new(count_set: usize, max_a: usize, g1: usize, mingencount:usize, sum:usize, apery:Vec<usize>, u:usize, genset:Vec<usize>) -> Self {
        //println!("sum {}, g1 {}, count_set {}, apery {:?}",sum,g1,count_set,apery);
        let count_gap = (sum - ((g1 - 1) * g1) / 2) / g1;
        Fast {
            max_a,
            sum_a: sum,
            g1,
            count_set,
            count_gap,
            e: mingencount,
            apery,
            u,
            genset
        }
    }
}


pub fn fast(input: &[usize]) -> Fast {
    let d = gcd_vec(input);
    let mut inputnumbers: Vec<usize> = input.iter().map(|x| (x / d) as usize).collect();
    inputnumbers.sort();
    let maximal_input: usize = *inputnumbers.last().unwrap();
    let width=2*maximal_input;
    let m: usize = *inputnumbers.first().unwrap();
    let mut aperyset: Vec<usize> = vec![0; m];
    let mut count_set = 1usize; // 0 schon dabei!
    let mut window = vec![-1isize; width]; // fenster hat die länge 2max
    let mut i: usize = m; // startindex
    let mut windowindex = m; // am anfang = i
    let mut runlength = 0usize; // anzahl aufeinanderfoldender hits
    let mut hit: bool = false;
    let mut max_apery:usize = m;
    let mut sum_apery:usize = 0;
    let mut minimal_generators:usize = 1;
    let mut max_atom = m;
    let mut genset:Vec<usize> = Vec::new();
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
                    if 0==window[windowindex - *k] {
                        minimal_generators+=1;
                        genset.push(i);
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
            let (dst, src) = window.split_at_mut(maximal_input);
            dst[0..maximal_input].clone_from_slice(&src[..maximal_input]);
            windowindex = maximal_input;
        } else {
            windowindex += 1;
        }
    }
    genset.push(m);
    Fast::new(count_set-m, max_apery, m, minimal_generators, sum_apery,aperyset,max_atom, genset)
}

#[cfg(test)]
mod test {
    use crate::modules::fast_semigroup::{fast};
    use crate::modules::Semigroup;

    #[test]
    fn zwo_drei(){
        let s= fast(&vec![2,3]);
        assert_eq!(1,s.f(),"semigroup 2,3 hat f=1");
    }
    #[test]
    fn others(){
        assert_eq!(250,fast(&vec![21,35,44,67]).f());
        assert_eq!(1,fast(&vec![2,3,44,67]).f());
        assert_eq!(5494,fast(&vec![121,235,444,9867]).f());
    }
}