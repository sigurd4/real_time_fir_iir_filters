use crate::param::{FilterFloat, Omega, OmegaFirstOrder};

pub struct FirstOrderCalc<F>
where
    F: FilterFloat
{
    omega: F,
    two_rate: F
}

impl<F> FirstOrderCalc<F>
where
    F: FilterFloat
{
    pub fn new(omega: OmegaFirstOrder<F>, rate: F) -> Self
    {
        let Omega {omega} = omega;
        let two_rate = rate + rate;
        Self {
            omega,
            two_rate
        }
    }

    pub fn b_low(&self) -> [F; 2]
    {
        [
            self.omega,
            self.omega
        ]
    }

    pub fn b_high(&self) -> [F; 2]
    {
        [
            self.two_rate,
            -self.two_rate
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.omega + self.two_rate,
            self.omega - self.two_rate
        ]
    }
}