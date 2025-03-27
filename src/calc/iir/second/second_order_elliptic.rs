use num::Complex;

use crate::param::{FilterFloat, OmegaEpsilonXi, OmegaEpsilonXiSecondOrder};

pub struct SecondOrderEllipticCalc<F>
where
    F: FilterFloat
{
    omega2: F,
    four_rate2: F,
    tm1: F,
    g: F,
    two_rate_s1ps2_omega: F,
    s1_s2: F
}

impl<F> SecondOrderEllipticCalc<F>
where
    F: FilterFloat
{
    pub fn new(omega_epsilon_xi: OmegaEpsilonXiSecondOrder<F>, rate: F) -> Self
    {
        let OmegaEpsilonXi {omega, epsilon, xi} = omega_epsilon_xi;

        let xi2 = xi*xi;
        let xi3 = xi2*xi;

        let one = F::one();

        let t = (one - xi3.recip()).sqrt();
        let tm1 = t - one;
        let tp1 = t + one;

        let s1 = -(Complex::new(epsilon, one)/Complex::new(-epsilon*tp1, tm1)).sqrt();
        let s2 = s1.conj();
        let s1_s2 = (s1*s2).re;
        let s1ps2 = (s1 + s2).re;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let omega2 = omega*omega;
        let epsilon2 = epsilon*epsilon;
        let two_rate_s1ps2_omega = two_rate*s1ps2*omega;
        
        let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();

        Self {
            omega2,
            four_rate2,
            tm1,
            g,
            two_rate_s1ps2_omega,
            s1_s2
        }
    }

    pub fn b_low(&self) -> [F; 3]
    {
        let four_rate2_tm1 = self.four_rate2*self.tm1;
        let omega2_m_four_rate2_tm1_g = (self.omega2 - four_rate2_tm1)*self.g;
        let omega2_p_four_rate2_tm1_g = (self.omega2 + four_rate2_tm1)*self.g;
        [
            omega2_m_four_rate2_tm1_g,
            omega2_p_four_rate2_tm1_g + omega2_p_four_rate2_tm1_g,
            omega2_m_four_rate2_tm1_g
        ]
    }
    pub fn b_high(&self) -> [F; 3]
    {
        let tm1_omega2 = self.tm1*self.omega2;
        let tm1_omega2_m_four_rate2_g = (tm1_omega2 - self.four_rate2)*self.g;
        let tm1_omega2_p_four_rate2_g = (tm1_omega2 + self.four_rate2)*self.g;
        [
            tm1_omega2_m_four_rate2_g,
            tm1_omega2_p_four_rate2_g + tm1_omega2_p_four_rate2_g,
            tm1_omega2_m_four_rate2_g
        ]
    }
    pub fn a_low(&self) -> [F; 3]
    {
        let s1_s2_omega2 = self.s1_s2*self.omega2;
        let s1_s2_omega2_p_four_rate2 = s1_s2_omega2 + self.four_rate2;
        let s1_s2_omega2_m_four_rate2 = s1_s2_omega2 - self.four_rate2;
        [
            s1_s2_omega2_p_four_rate2 - self.two_rate_s1ps2_omega,
            s1_s2_omega2_m_four_rate2 + s1_s2_omega2_m_four_rate2,
            s1_s2_omega2_p_four_rate2 + self.two_rate_s1ps2_omega
        ]
    }
    pub fn a_high(&self) -> [F; 3]
    {
        let four_rate2_s1_s2 = self.four_rate2*self.s1_s2;
        let omega2_p_four_rate2_s1_s2 = self.omega2 + four_rate2_s1_s2;
        let omega2_m_four_rate2_s1_s2 = self.omega2 - four_rate2_s1_s2;
        [
            omega2_p_four_rate2_s1_s2 - self.two_rate_s1ps2_omega,
            omega2_m_four_rate2_s1_s2 + omega2_m_four_rate2_s1_s2,
            omega2_p_four_rate2_s1_s2 + self.two_rate_s1ps2_omega
        ]
    }
}