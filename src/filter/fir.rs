pub mod sos_filter;

pub use sos_filter::*;

pub trait FIRFilter<const N: usize, const M: usize>
where
    [(); N + 1]:
{
    fn w(&mut self) -> &mut [f32; N];
    fn b(&self, rate: f32) -> [[f32; N + 1]; M];
    fn filter(&mut self, rate: f32, x: f32) -> [f32; M]
    {
        let b: [[f32; N + 1]; M] = self.b(rate);
        let w: &mut [f32; N] = self.w();
        let y = array_init::array_init(|k| x*b[k][0] + (0..N).map(|i| w[i]*b[k][i + 1]).reduce(|a, b| a + b).unwrap_or(0.0));
        
        for i in (1..w.len()).rev()
        {
            w[i] = w[i - 1];
        }
        w[0] = x;
        return y;
    }
}