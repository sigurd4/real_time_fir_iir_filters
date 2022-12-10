pub mod fir;
pub mod iir;

pub trait Filter<const M: usize>
{
    fn filter(&mut self, rate: f32, x: f32) -> [f32; M];
}