use crate::param::{FilterFloat, Omega, OmegaThirdOrder};

pub struct ThirdOrderButterworthCalc<F>
where
    F: FilterFloat
{
    omega3: F,
    three_omega3: F,
    two_omega2_rate: F,
    four_omega_rate2: F,
    eight_rate3: F,
    twenty_four_rate3: F
}
impl<F> ThirdOrderButterworthCalc<F>
where
    F: FilterFloat
{
    pub fn new(omega: OmegaThirdOrder<F>, rate: F) -> Self
    {       
        let Omega {omega} = omega;
        let omega2 = omega*omega;
        let omega3 = omega2*omega;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let eight_rate3 = four_rate2*two_rate;

        let one = F::one();
        let two = one + one;
        let three = two + one;

        let four_omega_rate2 = omega*four_rate2;
        let two_omega2_rate = omega2*two_rate;
        let three_omega3 = three*omega3;
        let twenty_four_rate3 = three*eight_rate3;

        Self {
            omega3,
            three_omega3,
            two_omega2_rate,
            four_omega_rate2,
            eight_rate3,
            twenty_four_rate3
        }
    }

    pub fn b_low(&self) -> [F; 4]
    {
        [
            self.omega3,
            self.three_omega3,
            self.three_omega3,
            self.omega3
        ]
    }
    pub fn b_peak1(&self) -> [F; 4]
    {
        let m_two_omega2_rate = -self.two_omega2_rate;
        [
            self.two_omega2_rate,
            self.two_omega2_rate,
            m_two_omega2_rate,
            m_two_omega2_rate
        ]
    }
    pub fn b_peak2(&self) -> [F; 4]
    {
        let m_four_omega_rate2 = -self.four_omega_rate2;
        [
            self.four_omega_rate2,
            m_four_omega_rate2,
            m_four_omega_rate2,
            self.four_omega_rate2
        ]
    }
    pub fn b_high(&self) -> [F; 4]
    {
        [
            self.eight_rate3,
            -self.twenty_four_rate3,
            self.twenty_four_rate3,
            -self.eight_rate3
        ]
    }
    pub fn a(&self) -> [F; 4]
    {
        let eight_omega_rate2 = self.four_omega_rate2 + self.four_omega_rate2;
        let four_omega2_rate = self.two_omega2_rate + self.two_omega2_rate;
        let eight_omega_rate2_p_omega3 = eight_omega_rate2 + self.omega3;
        let eight_rate3_p_four_omega2_rate = self.eight_rate3 + four_omega2_rate;
        let three_omega3_m_eight_omega_rate2 = self.three_omega3 - eight_omega_rate2;
        let four_omega2_rate_m_twenty_four_rate3 = four_omega2_rate - self.twenty_four_rate3;
        [
            eight_omega_rate2_p_omega3 + eight_rate3_p_four_omega2_rate,
            three_omega3_m_eight_omega_rate2 + four_omega2_rate_m_twenty_four_rate3,
            three_omega3_m_eight_omega_rate2 - four_omega2_rate_m_twenty_four_rate3,
            eight_omega_rate2_p_omega3 - eight_rate3_p_four_omega2_rate
        ]
    }
}