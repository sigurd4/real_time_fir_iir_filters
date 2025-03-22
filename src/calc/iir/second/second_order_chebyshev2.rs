use crate::{param::{FilterFloat, OmegaEpsilon, OmegaEpsilonCheb2SecondOrder}, util};

pub struct SecondOrderChebyshev2Calc<F, L = (), H = ()>
where
    F: FilterFloat
{
    pub omega2: F,
    pub two_omega2: F,
    pub four_omega2: L,
    pub four_rate2: F,
    pub eight_rate2: F,
    pub sixteen_rate2: H,
    pub epsilon_inv: F,
    pub cosh_2alpha: F,
    pub four_sqrt2_rate_omega_sinh_alpha: F
}

impl<F, L, H> SecondOrderChebyshev2Calc<F, L, H>
where
    F: FilterFloat,
    L: Default,
    H: Default
{
    pub fn new(omega_epsilon: OmegaEpsilonCheb2SecondOrder<F>, rate: F) -> Self
    {
        let OmegaEpsilon {omega, epsilon} = omega_epsilon;

        let one = F::one();
        let two = one + one;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let four_rate = two_rate + two_rate;
        let eight_rate2 = four_rate2*two_rate;
        let sixteen_rate2 = four_rate*four_rate;
        let omega2 = omega*omega;
        let two_omega2 = omega2 + omega2;
        let four_omega2 = two_omega2 + two_omega2;
        let epsilon_inv = epsilon.recip();
        let alpha = epsilon_inv.asinh()/two;
        let cosh_2alpha = (alpha + alpha).cosh();
        let sinh_alpha = alpha.sinh();
        let four_sqrt2_rate_omega_sinh_alpha = F::SQRT_2()*four_rate*omega*sinh_alpha;

        Self {
            omega2,
            two_omega2,
            four_omega2: util::same::eval_if_same(|| two_omega2 + two_omega2, Default::default()),
            four_rate2,
            eight_rate2,
            sixteen_rate2: util::same::eval_if_same(|| eight_rate2 + eight_rate2, Default::default()),
            epsilon_inv,
            cosh_2alpha,
            four_sqrt2_rate_omega_sinh_alpha
        }
    }
}
impl<F, H> SecondOrderChebyshev2Calc<F, F, H>
where
    F: FilterFloat
{
    pub fn b_low(&self) -> [F; 3]
    {
        let four_rate2_p_two_omega2_epsilon_inv = (self.four_rate2 + self.two_omega2)*self.epsilon_inv;
        [
            four_rate2_p_two_omega2_epsilon_inv,
            (self.four_omega2 - self.eight_rate2)*self.epsilon_inv,
            four_rate2_p_two_omega2_epsilon_inv
        ]
    }
    pub fn a_low(&self) -> [F; 3]
    {
        let four_rate2_cosh_2alpha = self.four_rate2*self.cosh_2alpha;
        let four_rate2_cosh_2alpha_p_two_omega2 = four_rate2_cosh_2alpha + self.two_omega2;
        let eight_rate2_cosh_2alpha = four_rate2_cosh_2alpha + four_rate2_cosh_2alpha;
        [
            four_rate2_cosh_2alpha_p_two_omega2 + self.four_sqrt2_rate_omega_sinh_alpha,
            self.four_omega2 - eight_rate2_cosh_2alpha,
            four_rate2_cosh_2alpha_p_two_omega2 - self.four_sqrt2_rate_omega_sinh_alpha
        ]
    }
}
impl<F, L> SecondOrderChebyshev2Calc<F, L, F>
where
    F: FilterFloat
{
    pub fn b_high(&self) -> [F; 3]
    {
        let eight_rate2_p_omega2_epsilon_inv = (self.eight_rate2 + self.omega2)*self.epsilon_inv;
        [
            eight_rate2_p_omega2_epsilon_inv,
            (self.two_omega2 - self.sixteen_rate2)*self.epsilon_inv,
            eight_rate2_p_omega2_epsilon_inv
        ]
    }
    pub fn a_high(&self) -> [F; 3]
    {
        let omega2_cosh_2alpha = self.omega2*self.cosh_2alpha;
        let eight_rate2_p_omega2_cosh_2alpha = self.eight_rate2 + omega2_cosh_2alpha;
        let two_omega2_cosh_2alpha = omega2_cosh_2alpha + omega2_cosh_2alpha;
        [
            eight_rate2_p_omega2_cosh_2alpha + self.four_sqrt2_rate_omega_sinh_alpha,
            two_omega2_cosh_2alpha - self.sixteen_rate2,
            eight_rate2_p_omega2_cosh_2alpha - self.four_sqrt2_rate_omega_sinh_alpha,
        ]
    }
}