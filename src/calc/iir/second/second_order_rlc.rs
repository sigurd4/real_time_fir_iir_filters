use crate::param::{FilterFloat, RLC};

pub struct SecondOrderRLCCalc<F>
where
    F: FilterFloat
{
    one: F,
    two: F,
    one_p_four_c_l_rate2: F,
    two_m_eight_c_l_rate2: F,
    two_c_r_rate: F,
    four_c_l_rate2: F,
    eight_c_l_rate2: F
}
impl<F> SecondOrderRLCCalc<F>
where
    F: FilterFloat
{
    pub fn new(rlc: RLC<F>, rate: F) -> Self
    {
        let RLC {r, l, c} = rlc;

        let one = F::one();
        let two = one + one;

        let c_rate = c*rate;
        let l_rate = l*rate;
        let two_c_rate = c_rate + c_rate;
        let two_l_rate = l_rate + l_rate;

        let two_c_r_rate = two_c_rate*r;
        let four_c_l_rate2 = two_c_rate*two_l_rate;
        let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
        let one_p_four_c_l_rate2 = one + four_c_l_rate2;
        let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;

        Self {
            one,
            two,
            one_p_four_c_l_rate2,
            two_m_eight_c_l_rate2,
            two_c_r_rate,
            four_c_l_rate2,
            eight_c_l_rate2
        }
    }
    pub fn b_low(&self) -> [F; 3]
    {
        [
            self.one,
            self.two,
            self.one
        ]
    }
    pub fn b_band_stop(&self) -> [F; 3]
    {
        [
            self.one_p_four_c_l_rate2,
            self.two_m_eight_c_l_rate2,
            self.one_p_four_c_l_rate2
        ]
    }
    pub fn b_band_pass(&self) -> [F; 3]
    {
        [
            self.two_c_r_rate,
            F::zero(),
            -self.two_c_r_rate
        ]
    }
    pub fn b_high(&self) -> [F; 3]
    {
        [
            self.four_c_l_rate2 + self.two_c_r_rate,
            -self.eight_c_l_rate2,
            self.four_c_l_rate2 - self.two_c_r_rate
        ]
    }
    pub fn a(&self) -> [F; 3]
    {
        [
            self.one_p_four_c_l_rate2 + self.two_c_r_rate,
            self.two_m_eight_c_l_rate2,
            self.one_p_four_c_l_rate2 - self.two_c_r_rate
        ]
    }
}