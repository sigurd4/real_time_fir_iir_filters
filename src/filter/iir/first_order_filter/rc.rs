use crate::iir::IIRFilter;

#[derive(Clone, Copy)]
pub struct FirstOrderRCFilter
{
    pub w: [f32; 1],
    pub r: f32,
    pub c: f32
}

impl FirstOrderRCFilter
{
    pub fn new(r: f32, c: f32) -> Self
    {
        Self {
            w: [0.0; 1],
            r,
            c
        }
    }

    pub fn omega(&self) -> f32
    {
        1.0/self.r/self.c
    }
}

impl IIRFilter<1, 2> for FirstOrderRCFilter
{
    fn a(&self, rate: f32) -> [f32; 2]
    {
        let omega = self.omega();
        [
            omega + 2.0*rate,
            omega - 2.0*rate,
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 2]; 2]
    {
        let omega = self.omega();
        [
            [
                omega,
                omega
            ],
            [
                2.0*rate,
                -2.0*rate
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 1]
    {
        &mut self.w
    }
}