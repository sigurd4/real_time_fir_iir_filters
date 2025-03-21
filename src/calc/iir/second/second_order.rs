use crate::param::{FilterFloat, Omega, OmegaZeta};

pub struct SecondOrderCalc<F>
where
    F: FilterFloat
{
    omega: F,
    two_omega: F,
    omega2: F,
    two_omega2: F,
    rate: F,
    two_rate: F,
    four_rate2: F,
    eight_rate2: F,
    zeta: F
}

impl<F> SecondOrderCalc<F>
where
    F: FilterFloat
{
    pub fn new_butterworth(omega: Omega<F>, rate: F) -> Self
    {
        let Omega {omega} = omega;
        let zeta = F::FRAC_1_SQRT_2();
        Self::new(OmegaZeta {omega, zeta}, rate)
    }

    pub fn new(omega_zeta: OmegaZeta<F>, rate: F) -> Self
    {
        let OmegaZeta {omega, zeta} = omega_zeta;
        let two_omega = omega + omega;
        let omega2 = omega*omega;
        let two_omega2 = omega2 + omega2;
        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let eight_rate2 = four_rate2 + four_rate2;
        Self {
            omega,
            two_omega: todo!(),
            omega2,
            two_omega2,
            rate,
            two_rate,
            four_rate2,
            eight_rate2,
            zeta,
        }
    }

    pub fn b_low(&self) -> [F; 3]
    {
        [
            self.omega2,
            self.two_omega2,
            self.omega2
        ]
    }
    pub fn b_peak(&self) -> [F; 3]
    {
        let rate_omega = self.rate*self.omega;
        let two_rate_omega = rate_omega + rate_omega;
        [
            two_rate_omega,
            F::zero(),
            -two_rate_omega
        ]
    }
    pub fn b_high(&self) -> [F; 3]
    {
        [
            self.four_rate2,
            -self.eight_rate2,
            self.four_rate2
        ]
    }

    pub fn a(&self) -> [F; 3]
    {
        let four_rate2_p_omega2 = self.four_rate2 + self.omega2;
        let four_rate_zeta_omega = self.two_rate*self.zeta*self.two_omega;
        [
            four_rate2_p_omega2 + four_rate_zeta_omega,
            self.two_omega2 - self.eight_rate2,
            four_rate2_p_omega2 - four_rate_zeta_omega
        ]
    }
}