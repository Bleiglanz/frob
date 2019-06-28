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
    fn max_apery(&self)-> usize;
    fn sum_apery(&self)-> usize;
    fn count_set(&self) -> usize;
    fn count_gap(&self) -> usize;
    fn f_over_m(&self) -> f64 {
        self.f() as f64 / self.m() as f64
    }
}