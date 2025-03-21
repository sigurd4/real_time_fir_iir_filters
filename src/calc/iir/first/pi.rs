use crate::param::{FilterFloat, PI};

pub struct PICalc<F>
where
    F: FilterFloat
{
    two_rate: F,
    two_rate_p: F,
    i: F
}

impl<F> PICalc<F>
where
    F: FilterFloat
{
    pub fn new(pi: PI<F>, rate: F) -> Self
    {
        let PI {p, i} = pi;
        let two_rate = rate + rate;
        let two_rate_p = two_rate*p;
        Self {
            two_rate,
            two_rate_p,
            i
        }
    }

    pub fn b(&self) -> [F; 2]
    {
        [
            self.i + self.two_rate_p,
            self.i - self.two_rate_p
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.two_rate,
            -self.two_rate
        ]
    }
}