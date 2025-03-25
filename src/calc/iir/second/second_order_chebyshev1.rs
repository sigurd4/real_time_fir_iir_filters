use crate::param::{FilterFloat, OmegaEpsilon, OmegaEpsilonCheb1SecondOrder};

pub struct SecondOrderChebyshev1Calc<F>
where
    F: FilterFloat
{
    omega2: F,
    four_rate2: F,
    epsilon_inv: F,
    cosh_2alpha: F,
    four_sqrt2_rate_omega_sinh_alpha: F
}
impl<F> SecondOrderChebyshev1Calc<F>
where
    F: FilterFloat
{
    pub fn new(omega_epsilon: OmegaEpsilonCheb1SecondOrder<F>, rate: F) -> Self
    {
        let OmegaEpsilon {omega, epsilon, _m: _} = omega_epsilon;

        let one = F::one();
        let two = one + one;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let four_rate = two_rate + two_rate;
        let omega2 = omega*omega;
        let epsilon_inv = epsilon.recip();
        let alpha = epsilon_inv.asinh()/two;
        let cosh_2alpha = (alpha + alpha).cosh();
        let sinh_alpha = alpha.sinh();
        let four_sqrt2_rate_omega_sinh_alpha = F::SQRT_2()*four_rate*omega*sinh_alpha;

        Self {
            omega2,
            four_rate2,
            epsilon_inv,
            cosh_2alpha,
            four_sqrt2_rate_omega_sinh_alpha
        }
    }

    pub fn b_low(&self) -> [F; 3]
    {
        let omega2_d_epsilon = self.omega2*self.epsilon_inv;
        [
            omega2_d_epsilon,
            omega2_d_epsilon + omega2_d_epsilon,
            omega2_d_epsilon
        ]
    }
    pub fn b_high(&self) -> [F; 3]
    {
        let four_rate2_d_epsilon = self.four_rate2*self.epsilon_inv;
        [
            four_rate2_d_epsilon,
            -(four_rate2_d_epsilon + four_rate2_d_epsilon),
            four_rate2_d_epsilon
        ]
    }

    pub fn a_low(&self) -> [F; 3]
    {
        let eight_rate2 = self.four_rate2 + self.four_rate2;
        let omega2_cosh_2alpha = self.omega2*self.cosh_2alpha;
        let omega2_cosh_2alpha_p_eight_rate2 = omega2_cosh_2alpha + eight_rate2;
        let omega2_cosh_2alpha_m_eight_rate2 = omega2_cosh_2alpha - eight_rate2;
        [
            omega2_cosh_2alpha_p_eight_rate2 + self.four_sqrt2_rate_omega_sinh_alpha,
            omega2_cosh_2alpha_m_eight_rate2 + omega2_cosh_2alpha_m_eight_rate2,
            omega2_cosh_2alpha_p_eight_rate2 - self.four_sqrt2_rate_omega_sinh_alpha
        ]
    }
    pub fn a_high(&self) -> [F; 3]
    {
        let four_rate2_cosh_2alpha = self.four_rate2*self.cosh_2alpha;
        let two_omega2 = self.omega2 + self.omega2;
        let four_rate2_cosh_2alpha_p_two_omega2 = four_rate2_cosh_2alpha + two_omega2;
        let two_omega2_m_four_rate2_cosh_2alpha = two_omega2 - four_rate2_cosh_2alpha;
        [
            four_rate2_cosh_2alpha_p_two_omega2 + self.four_sqrt2_rate_omega_sinh_alpha,
            two_omega2_m_four_rate2_cosh_2alpha + two_omega2_m_four_rate2_cosh_2alpha,
            four_rate2_cosh_2alpha_p_two_omega2 - self.four_sqrt2_rate_omega_sinh_alpha
        ]
    }
}