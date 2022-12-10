use super::IIRFilter;

pub mod sallen_key;

#[derive(Copy, Clone)]
pub struct SecondOrderFilter
{
    pub omega: f32,
    pub zeta: f32,
    pub w: [f32; 2]
}

impl SecondOrderFilter
{
    pub fn new(omega: f32, zeta: f32) -> Self
    {
        Self {
            omega,
            zeta,
            w: [0.0; 2]
        }
    }
}

impl IIRFilter<2, 3> for SecondOrderFilter
{
    fn a(&self, rate: f32) -> [f32; 3]
    {
        [
            4.0*rate.powi(2) + 4.0*rate*self.zeta*self.omega + self.omega.powi(2),
            2.0*self.omega.powi(2) - 8.0*rate.powi(2),
            4.0*rate.powi(2) - 4.0*rate*self.zeta*self.omega + self.omega.powi(2)
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 3]; 3]
    {
        [
            [
                self.omega.powi(2),
                2.0*self.omega.powi(2),
                self.omega.powi(2)
            ],
            [
                2.0*rate*self.omega,
                0.0,
                -2.0*rate*self.omega,
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