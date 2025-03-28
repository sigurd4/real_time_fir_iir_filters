use num::Zero;

use crate::{param::{FilterFloat, RC2GSallenKey}, util};

pub struct SecondOrderSallenKeyCalc<F, LB1 = (), B = (), B2H = ()>
where
    F: FilterFloat
{
    g: F,
    two: F,
    one_p_four_c1_c2_r1_r2_rate2: F,
    two_m_eight_c1_c2_r1_r2_rate2: F,
    two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate: F,
    two_g: LB1,
    two_c1_r1_g_rate: LB1,
    zero: B,
    two_c2_r2_g_rate: B2H,
    four_c1_c2_r1_r2_g_rate2: B2H,
    eight_c1_c2_r1_r2_g_rate2: B2H
}

impl<F, LB1, B, B2H> SecondOrderSallenKeyCalc<F, LB1, B, B2H>
where
    F: FilterFloat,
    LB1: Default,
    B: Default,
    B2H: Default
{
    pub fn new(rc2g: RC2GSallenKey<F>, rate: F) -> Self
    {
        let RC2GSallenKey {r1, c1, r2, c2, g} = rc2g;

        let one = F::one();
        let two = one + one;

        let two_rate = two*rate;
        
        let two_c1_r1_rate = c1*r1*two_rate;
        let two_c2_r2_rate = c2*r2*two_rate;

        let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
        let (four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2) = util::same::eval_if_same(
            || {
                let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
                let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;
                (four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
            },
            Default::default()
        );

        let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
        let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

        let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate = c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;

        Self {
            g,
            two,
            one_p_four_c1_c2_r1_r2_rate2,
            two_m_eight_c1_c2_r1_r2_rate2,
            two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
            two_g: util::same::eval_if_same(|| two*g, Default::default()),
            two_c1_r1_g_rate: util::same::eval_if_same(|| two_c1_r1_rate*g, Default::default()),
            zero: util::same::eval_if_same(<F as Zero>::zero, Default::default()),
            two_c2_r2_g_rate: util::same::eval_if_same(|| two_c2_r2_rate*g, Default::default()),
            four_c1_c2_r1_r2_g_rate2,
            eight_c1_c2_r1_r2_g_rate2
        }
    }
}

impl<F, B2H> SecondOrderSallenKeyCalc<F, F, F, B2H>
where
    F: FilterFloat
{
    pub fn b_band1(&self) -> [F; 3] // LB1 + B
    {
        [
            self.two_c1_r1_g_rate,
            self.zero,
            -self.two_c1_r1_g_rate,
        ]
    }
}
impl<F, B, B2H> SecondOrderSallenKeyCalc<F, F, B, B2H>
where
    F: FilterFloat
{
    pub fn b_low(&self) -> [F; 3] // LB1
    {
        let g2 = self.g*self.g;
        [
            g2,
            self.two_g,
            g2,
        ]
    }
    pub fn a_low(&self) -> [F; 3] // LB1
    {
        let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate = self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate - self.two_c1_r1_g_rate;
        [
            self.one_p_four_c1_c2_r1_r2_rate2 + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate,
            self.two_m_eight_c1_c2_r1_r2_rate2,
            self.one_p_four_c1_c2_r1_r2_rate2 - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate,
        ]
    }
    pub fn a_band1(&self) -> [F; 3] // LB1
    {
        let one_p_four_c1_c2_r1_r2_rate2_m_g = self.one_p_four_c1_c2_r1_r2_rate2 - self.g;
        [
            one_p_four_c1_c2_r1_r2_rate2_m_g + self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
            self.two_m_eight_c1_c2_r1_r2_rate2 - self.two_g,
            one_p_four_c1_c2_r1_r2_rate2_m_g - self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
        ]
    }
}
impl<F, LB1> SecondOrderSallenKeyCalc<F, LB1, F, F>
where
    F: FilterFloat
{
    pub fn b_band2(&self) -> [F; 3] // B2H + B
    {
        [
            self.two_c2_r2_g_rate,
            self.zero,
            -self.two_c2_r2_g_rate,
        ]
    }
}
impl<F, LB1, B> SecondOrderSallenKeyCalc<F, LB1, B, F>
where
    F: FilterFloat
{
    pub fn b_high(&self) -> [F; 3] // B2H
    {
        [
            self.four_c1_c2_r1_r2_g_rate2,
            -self.eight_c1_c2_r1_r2_g_rate2,
            self.four_c1_c2_r1_r2_g_rate2,
        ]
    }
    pub fn a_band2(&self) -> [F; 3] // B2H
    {
        let one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 = self.one_p_four_c1_c2_r1_r2_rate2 - self.four_c1_c2_r1_r2_g_rate2;
        [
            one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 + self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
            self.two_m_eight_c1_c2_r1_r2_rate2 + self.two*self.four_c1_c2_r1_r2_g_rate2,
            one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 - self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
        ]
    }
    pub fn a_high(&self) -> [F; 3] // B2H
    {
        let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate = self.two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate - self.two_c2_r2_g_rate;
        [
            self.one_p_four_c1_c2_r1_r2_rate2 + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate,
            self.two_m_eight_c1_c2_r1_r2_rate2,
            self.one_p_four_c1_c2_r1_r2_rate2 - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate,
        ]
    }
}