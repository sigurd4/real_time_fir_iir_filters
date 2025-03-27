use num::{One, Zero};

use crate::{param::{FilterFloat, RC2}, util};

pub struct SecondOrderRCCalc<F, B = ()>
where
    F: FilterFloat
{
    one: F,
    zero: B,
    two_c1_r1_rate: F,
    two_c2_r2_rate: F,
    two_c2_r1_rate: F
}

impl<F, B> SecondOrderRCCalc<F, B>
where
    F: FilterFloat,
    B: Default
{
    pub fn new(rc2: RC2<F>, rate: F) -> Self
    {
        let RC2 {r1, c1, r2, c2} = rc2;

        let two_rate = rate + rate;
        let two_r1_rate = r1*two_rate;
        let two_c1_r1_rate = c1*two_r1_rate;
        let two_c2_r2_rate = c2*r2*two_rate;
        let two_c2_r1_rate = c2*two_r1_rate;

        let one = One::one();

        Self {
            one,
            zero: util::same::eval_if_same(<F as Zero>::zero, Default::default()),
            two_c1_r1_rate,
            two_c2_r2_rate,
            two_c2_r1_rate
        }
    }

    pub fn b_low(&self) -> [F; 3]
    {
        [
            self.one,
            self.one + self.one,
            self.one
        ]
    }
    pub fn b_high(&self) -> [F; 3]
    {
        let four_c1_c2_r1_r2_rate2 = self.two_c1_r1_rate*self.two_c2_r2_rate;
        let eight_c1_c2_r1_r2_rate2 = four_c1_c2_r1_r2_rate2 + four_c1_c2_r1_r2_rate2;
        [
            four_c1_c2_r1_r2_rate2,
            -eight_c1_c2_r1_r2_rate2,
            four_c1_c2_r1_r2_rate2,
        ]
    }
    pub fn a(&self) -> [F; 3]
    {
        let two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate = self.two_c2_r2_rate + self.two_c2_r1_rate + self.two_c1_r1_rate;
        let four_c1_c2_r1_r2_rate2 = self.two_c1_r1_rate*self.two_c2_r2_rate;
        let one_p_four_c1_c2_r1_r2_rate2 = self.one + four_c1_c2_r1_r2_rate2;
        let one_m_four_c1_c2_r1_r2_rate2 = self.one - four_c1_c2_r1_r2_rate2;
        [
            one_p_four_c1_c2_r1_r2_rate2 + two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate,
            one_m_four_c1_c2_r1_r2_rate2 + one_m_four_c1_c2_r1_r2_rate2,
            one_p_four_c1_c2_r1_r2_rate2 - two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate,
        ]
    }
}
impl<F> SecondOrderRCCalc<F, F>
where
    F: FilterFloat
{
    pub fn b_band1(&self) -> [F; 3]
    {
        [
            self.two_c1_r1_rate,
            self.zero,
            -self.two_c1_r1_rate
        ]
    }
    pub fn b_band2(&self) -> [F; 3]
    {
        [
            self.two_c2_r2_rate,
            self.zero,
            -self.two_c2_r2_rate,
        ]
    }
}