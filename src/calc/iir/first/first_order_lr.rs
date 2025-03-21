use crate::param::{FilterFloat, LR};

pub struct FirstOrderLRCalc<F>
where
    F: FilterFloat
{
    r: F,
    two_rate_l: F
}

impl<F> FirstOrderLRCalc<F>
where
    F: FilterFloat
{
    pub fn new(lr: LR<F>, rate: F) -> Self
    {
        let LR {l, r} = lr;
        let two_rate = rate + rate;
        let two_rate_l = two_rate*l;
        Self {
            r,
            two_rate_l
        }
    }

    pub fn b_low(&self) -> [F; 2]
    {
        [
            self.r,
            self.r
        ]
    }

    pub fn b_high(&self) -> [F; 2]
    {
        [
            self.two_rate_l,
            -self.two_rate_l
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.r + self.two_rate_l,
            self.r - self.two_rate_l
        ]
    }
}