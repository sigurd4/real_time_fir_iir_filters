use crate::iir::IIRFilter;

pub struct PIDController
{
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub w: [f32; 2]
}

impl PIDController
{
    pub fn new(p: f32, i: f32, d: f32) -> Self
    {
        Self {
            p,
            i,
            d,
            w: [0.0; 2]
        }
    }
}

impl IIRFilter<2, 1> for PIDController
{
    fn a(&self, rate: f32) -> [f32; 3]
    {
        [
            2.0*rate,
            0.0,
            -2.0*rate
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 3]; 1]
    {
        let rate2 = rate*rate;
        [
            [
                4.0*rate2*self.d + 2.0*rate*self.p + self.i,
                -8.0*rate2*self.d + 2.0*self.i,
                4.0*rate2*self.d - 2.0*rate*self.p + self.i,
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 2]
    {
        &mut self.w
    }
}