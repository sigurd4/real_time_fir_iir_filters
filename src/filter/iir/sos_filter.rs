pub trait IIRSOSFilter<const N: usize>
{
    fn w(&mut self, n: usize) -> &mut [f32; 2];

    fn a(&self, rate: f32, n: usize) -> [f32; 3];
    fn b(&self, rate: f32, n: usize) -> [f32; 3];

    fn filter(&mut self, rate: f32, x: f32) -> f32
    {
        let mut y = x;
        for n in 0..N
        {
            let a = self.a(rate, n);
            let b = self.b(rate, n);
            let w: &mut [f32; 2] = self.w(n);
            if a[0] == 0.0
            {
                return 0.0
            }
            let w0 = y - (0..2).map(|i| w[i]*a[i + 1]/a[0]).reduce(|a, b| a + b).unwrap_or(0.0);
            y = w0*b[0]/a[0] + (0..2).map(|i| w[i]*b[i + 1]/a[0]).reduce(|a, b| a + b).unwrap_or(0.0);
            
            for i in (1..w.len()).rev()
            {
                w[i] = w[i - 1];
            }
            w[0] = w0;
        }
        return y;
    }
}
