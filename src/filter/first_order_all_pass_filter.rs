use super::IIRFilter;

pub struct FirstOrderAllPassFilter
{
    pub w: [f32; 1],
    pub tau: f32
}

impl FirstOrderAllPassFilter
{
    pub fn new(tau: f32) -> Self
    {
        Self {
            w: [0.0; 1],
            tau
        }
    }
}

impl IIRFilter<1, 1> for FirstOrderAllPassFilter
{
    fn a(&self, rate: f32) -> [f32; 2]
    {
        [
            1.0 + 2.0*self.tau*rate,
            1.0 - 2.0*self.tau*rate
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 2]; 1]
    {
        [
            [
                2.0*self.tau*rate - 1.0,
                1.0 - 2.0*self.tau*rate
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 1]
    {
        &mut self.w
    }
}