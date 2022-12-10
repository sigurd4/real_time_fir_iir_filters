use crate::iir::IIRFilter;

#[derive(Copy, Clone)]
pub struct SecondOrderSallenKeyFilter
{
    pub w: [f32; 2],
    pub r1: f32,
    pub r2: f32,
    pub c1: f32,
    pub c2: f32
}

impl SecondOrderSallenKeyFilter
{
    pub fn new(r1: f32, r2: f32, c1: f32, c2: f32) -> Self
    {
        Self {
            w: [0.0; 2],
            r1,
            r2,
            c1,
            c2
        }
    }

    pub fn omega(&self) -> f32
    {
        1.0/(self.r1*self.r2*self.c1*self.c2).sqrt()
    }

    pub fn zeta(&self) -> f32
    {
        0.5*(1.0/self.r2/self.c1 + 1.0/self.r1/self.c1)*(self.r1*self.r2*self.c1*self.c2).sqrt()
    }
}

impl IIRFilter<2, 3> for SecondOrderSallenKeyFilter
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