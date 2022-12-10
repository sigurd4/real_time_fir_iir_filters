pub mod first_order_filter;
pub mod first_order_all_pass_filter;
pub mod second_order_filter;
pub mod wah_filter;

pub use first_order_filter::*;
pub use first_order_all_pass_filter::*;
pub use second_order_filter::*;
pub use wah_filter::*;

pub trait IIRFilter<const N: usize, const M: usize>
where
    [(); N + 1]:
{
    fn w(&mut self) -> &mut [f32; N];
    fn a(&self, rate: f32) -> [f32; N + 1];
    fn b(&self, rate: f32) -> [[f32; N + 1]; M];
    fn filter(&mut self, rate: f32, x: f32) -> [f32; M]
    {
        let a: [f32; N + 1] = self.a(rate);
        let b: [[f32; N + 1]; M] = self.b(rate);
        let w: &mut [f32; N] = self.w();
        if a[0] == 0.0
        {
            return [0.0; M]
        }
        let w0 = x - (0..N).map(|i| w[i]*a[i + 1]/a[0]).reduce(|a, b| a + b).unwrap_or(0.0);
        let y = array_init::array_init(|k| w0*b[k][0]/a[0] + (0..N).map(|i| w[i]*b[k][i + 1]/a[0]).reduce(|a, b| a + b).unwrap_or(0.0));
        
        for i in (1..w.len()).rev()
        {
            w[i] = w[i - 1];
        }
        w[0] = w0;
        return y;
    }
}