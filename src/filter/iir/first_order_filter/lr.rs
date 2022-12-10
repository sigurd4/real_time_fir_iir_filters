use crate::iir::IIRFilter;

#[derive(Clone, Copy)]
pub struct FirstOrderLRFilter
{
    pub w: [f32; 1],
    pub l: f32,
    pub r: f32
}

impl FirstOrderLRFilter
{
    pub fn new(l: f32, r: f32) -> Self
    {
        Self {
            w: [0.0; 1],
            l,
            r
        }
    }

    pub fn omega(&self) -> f32
    {
        self.r/self.l
    }
}

impl IIRFilter<1, 2> for FirstOrderLRFilter
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