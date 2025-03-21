use num::One;

use crate::param::{FilterFloat, RC};

pub struct FirstOrderRCCalc<F>
where
    F: FilterFloat
{
    one: F,
    two_rate_d_omega: F
}

impl<F> FirstOrderRCCalc<F>
where
    F: FilterFloat
{
    pub fn new(rc: RC<F>, rate: F) -> Self
    {
        let RC {r, c} = rc;
        let rate_d_omega = rate*r*c;
        let two_rate_d_omega = rate_d_omega + rate_d_omega;
        let one = One::one();
        Self {
            one,
            two_rate_d_omega
        }
    }

    pub fn b_low(&self) -> [F; 2]
    {
        [
            self.one,
            self.one
        ]
    }

    pub fn b_high(&self) -> [F; 2]
    {
        [
            self.two_rate_d_omega,
            -self.two_rate_d_omega
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.one + self.two_rate_d_omega,
            self.one - self.two_rate_d_omega
        ]
    }
}