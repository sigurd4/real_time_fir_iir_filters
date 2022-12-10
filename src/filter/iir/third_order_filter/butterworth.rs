use crate::Filter;
use crate::iir::{FirstOrderFilter, SecondOrderFilter, IIRFilter};

#[derive(Copy, Clone)]
pub struct ThirdOrderButterworthFilter
{
    pub w: [f32; 3],
    pub omega: f32,
}

impl ThirdOrderButterworthFilter
{
    pub fn zeta() -> f32
    {
        0.5
    }
}

impl IIRFilter<3, 4> for ThirdOrderButterworthFilter
{
    fn a(&self, rate: f32) -> [f32; 4]
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let omega2 = self.omega*self.omega;
        let omega3 = omega2*self.omega;
        [
            8.0*rate3 + 4.0*rate2*self.omega + 2.0*rate*omega2 + omega3,
            -24.0*rate3 - 4.0*rate2*self.omega + 2.0*rate*omega2 + 3.0*omega3,
            24.0*rate3 - 4.0*rate2*self.omega - 2.0*rate*omega2 + 3.0*omega3,
            -8.0*rate3 + 4.0*rate2*self.omega - 2.0*rate*omega2 + omega3,

        ]
    }
    fn b(&self, rate: f32) -> [[f32; 4]; 4]
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;
        
        let omega2 = self.omega*self.omega;
        let omega3 = omega2*self.omega;
        [
            [
                omega3,
                3.0*omega3,
                3.0*omega3,
                omega3
            ],
            [
                2.0*rate*omega2,
                2.0*rate*omega2,
                -2.0*rate*omega2,
                -2.0*rate*omega2
            ],
            [
                4.0*rate2*self.omega,
                -4.0*rate2*self.omega,
                -4.0*rate2*self.omega,
                4.0*rate2*self.omega
            ],
            [
                8.0*rate3,
                -24.0*rate3,
                24.0*rate3,
                -8.0*rate3
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 3]
    {
        &mut self.w
    }
}