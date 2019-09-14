extern crate rand;

pub mod math;
pub mod semigroup;
pub mod wilf;
pub mod fast_semigroup;

pub trait Semigroup {
    fn e(&self) -> usize;
    fn f(&self) -> usize;
    fn c(&self) -> usize;
    fn m(&self) -> usize;
    fn u(&self) -> usize;
    fn max_apery(&self)-> usize;
    fn sum_apery(&self)-> usize;
    fn count_set(&self) -> usize;
    fn count_gap(&self) -> usize;
    fn f_over_m(&self) -> f64 {
        self.f() as f64 / self.m() as f64
    }
    fn distance_to_f_over_m(&self) -> i64 {
        let d1 = self.f() as i64 - (self.f() as i64/ self.m() as i64) * self.m() as i64;
        let d2 = self.f() as i64 - ((self.f() as i64/ self.m() as i64)+1) * self.m() as i64;
        if d2.abs() < d1 { d2 } else { d1 }
    }
    fn contains(&self, n:usize) -> bool;
    fn max_even_gap(&self) -> usize;
}