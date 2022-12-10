use super::IIRFilter;

#[derive(Copy, Clone)]
pub struct SecondOrderButterworthFilter
{
    pub omega: f32,
    pub w: [f32; 2]
}

impl SecondOrderButterworthFilter
{
    pub fn new(omega: f32) -> Self
    {
        Self {
            omega,
            w: [0.0; 2]
        }
    }

    pub fn zeta() -> f32
    {
        (0.5f32).sqrt()
    }
}

impl IIRFilter<2, 3> for SecondOrderButterworthFilter
{
    fn a(&self, rate: f32) -> [f32; 3]
    {
        let zeta = Self::zeta();
        [
            4.0*rate.powi(2) + 4.0*rate*zeta*self.omega + self.omega.powi(2),
            2.0*self.omega.powi(2) - 8.0*rate.powi(2),
            4.0*rate.powi(2) - 4.0*rate*zeta*self.omega + self.omega.powi(2)
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