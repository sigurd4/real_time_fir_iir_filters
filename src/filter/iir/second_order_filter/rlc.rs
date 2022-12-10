use crate::iir::IIRFilter;

#[derive(Copy, Clone)]
pub struct SecondOrderRLCFilter
{
    pub w: [f32; 2],
    pub r: f32,
    pub l: f32,
    pub c: f32
}

impl SecondOrderRLCFilter
{
    pub fn new(r: f32, l: f32, c: f32) -> Self
    {
        Self {
            w: [0.0; 2],
            r,
            l,
            c
        }
    }

    pub fn omega(&self) -> f32
    {
        1.0/(self.l*self.c).sqrt()
    }

    pub fn zeta(&self) -> f32
    {
        0.5*self.r/self.l*(self.l*self.c).sqrt()
    }
}

impl IIRFilter<2, 3> for SecondOrderRLCFilter
{
    fn a(&self, rate: f32) -> [f32; 3]
    {
        let omega = self.omega();
        let zeta = self.zeta();
        [
            4.0*rate.powi(2) + 4.0*rate*zeta*omega + omega.powi(2),
            2.0*omega.powi(2) - 8.0*rate.powi(2),
            4.0*rate.powi(2) - 4.0*rate*zeta*omega + omega.powi(2)
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 3]; 3]
    {
        let omega = self.omega();
        [
            [
                omega.powi(2),
                2.0*omega.powi(2),
                omega.powi(2)
            ],
            [
                2.0*rate*omega,
                0.0,
                -2.0*rate*omega,
            ],
            [
                4.0*rate.powi(2),
                -8.0*rate.powi(2),
                4.0*rate.powi(2)
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 2]
    {
        &mut self.w
    }
}