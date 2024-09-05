use num::Float;

use crate::{conf::{All, HighPass, LowPass}, param::{Chebyshev2FilterParam, ChebyshevFilterConf, FilterParamSecondOrder}, params::OmegaEpsilonCheb2SecondOrder, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///            √(ε^2T_2^2(ω/s))
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(ω/s))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///            √(ε^2T_2^2(s/ω))
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(s/ω))
        /// ```
    }
    SecondOrderChebyshev2Filter
    {
        type Conf: ChebyshevFilterConf;
        type Param: Chebyshev2FilterParam = OmegaEpsilonCheb2SecondOrder;

        const O_BUFFERS: usize = <CC as ChebyshevFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        where {
            P: FilterParamSecondOrder
        }
        {
            let omega = param.omega();
            let epsilon = param.epsilon();

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
            (
                ([], [], [
                    second_order_chebyshev2_low_pass_filter_b(four_rate2, eight_rate2, two_omega2, four_omega2, epsilon_inv),
                    second_order_chebyshev2_high_pass_filter_b(eight_rate2, sixteen_rate2, omega2, two_omega2, epsilon_inv)
                ]),
                [([], [
                    second_order_chebyshev2_low_pass_filter_a(four_rate2, two_omega2, four_omega2, cosh_2alpha, four_sqrt2_rate_omega_sinh_alpha),
                    second_order_chebyshev2_high_pass_filter_a(eight_rate2, sixteen_rate2, omega2, cosh_2alpha, four_sqrt2_rate_omega_sinh_alpha)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        where {
            P: FilterParamSecondOrder
        }
        {
            let omega = param.omega();
            let epsilon = param.epsilon();

            let one = F::one();
            let two = one + one;
    
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let four_rate = two_rate + two_rate;
            let eight_rate2 = four_rate2*two_rate;
            let omega2 = omega*omega;
            let two_omega2 = omega2 + omega2;
            let four_omega2 = two_omega2 + two_omega2;
            let epsilon_inv = epsilon.recip();
            let two_alpha = epsilon_inv.asinh();
            let alpha = two_alpha/two;
            let cosh_2alpha = two_alpha.cosh();
            let sinh_alpha = alpha.sinh();
            let four_sqrt2_rate_omega_sinh_alpha = F::SQRT_2()*four_rate*omega*sinh_alpha;
            (
                ([], [], [
                    second_order_chebyshev2_low_pass_filter_b(four_rate2, eight_rate2, two_omega2, four_omega2, epsilon_inv)
                ]),
                [([], [
                    second_order_chebyshev2_low_pass_filter_a(four_rate2, two_omega2, four_omega2, cosh_2alpha, four_sqrt2_rate_omega_sinh_alpha)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        where {
            P: FilterParamSecondOrder
        }
        {
            let omega = param.omega();
            let epsilon = param.epsilon();

            let one = F::one();
            let two = one + one;
    
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let four_rate = two_rate + two_rate;
            let eight_rate2 = four_rate2*two_rate;
            let sixteen_rate2 = four_rate*four_rate;
            let omega2 = omega*omega;
            let two_omega2 = omega2 + omega2;
            let epsilon_inv = epsilon.recip();
            let two_alpha = epsilon_inv.asinh();
            let alpha = two_alpha/two;
            let cosh_2alpha = two_alpha.cosh();
            let sinh_alpha = alpha.sinh();
            let four_sqrt2_rate_omega_sinh_alpha = F::SQRT_2()*four_rate*omega*sinh_alpha;
            (
                ([], [], [
                    second_order_chebyshev2_high_pass_filter_b(eight_rate2, sixteen_rate2, omega2, two_omega2, epsilon_inv)
                ]),
                [([], [
                    second_order_chebyshev2_high_pass_filter_a(eight_rate2, sixteen_rate2, omega2, cosh_2alpha, four_sqrt2_rate_omega_sinh_alpha)
                ])]
            )
        }
    }
    where
        P: FilterParamSecondOrder,
        [(); <CC as ChebyshevFilterConf>::OUTPUTS]:
);


pub(crate) fn second_order_chebyshev2_low_pass_filter_b<F>(four_rate2: F, eight_rate2: F, two_omega2: F, four_omega2: F, epsilon_inv: F) -> [F; 3]
where
    F: Float
{
    let four_rate2_p_two_omega2_epsilon_inv = (four_rate2 + two_omega2)*epsilon_inv;
    [
        four_rate2_p_two_omega2_epsilon_inv,
        (four_omega2 - eight_rate2)*epsilon_inv,
        four_rate2_p_two_omega2_epsilon_inv
    ]
}
pub(crate) fn second_order_chebyshev2_high_pass_filter_b<F>(eight_rate2: F, sixteen_rate2: F, omega2: F, two_omega2: F, epsilon_inv: F) -> [F; 3]
where
    F: Float
{
    let eight_rate2_p_omega2_epsilon_inv = (eight_rate2 + omega2)*epsilon_inv;
    [
        eight_rate2_p_omega2_epsilon_inv,
        (two_omega2 - sixteen_rate2)*epsilon_inv,
        eight_rate2_p_omega2_epsilon_inv
    ]
}
pub(crate) fn second_order_chebyshev2_low_pass_filter_a<F>(
    four_rate2: F,
    two_omega2: F,
    four_omega2: F,
    cosh_2alpha: F,
    four_sqrt2_rate_omega_sinh_alpha: F
) -> [F; 3]
where
    F: Float
{
    let four_rate2_cosh_2alpha = four_rate2*cosh_2alpha;
    let four_rate2_cosh_2alpha_p_two_omega2 = four_rate2_cosh_2alpha + two_omega2;
    let eight_rate2_cosh_2alpha = four_rate2_cosh_2alpha + four_rate2_cosh_2alpha;
    [
        four_rate2_cosh_2alpha_p_two_omega2 + four_sqrt2_rate_omega_sinh_alpha,
        four_omega2 - eight_rate2_cosh_2alpha,
        four_rate2_cosh_2alpha_p_two_omega2 - four_sqrt2_rate_omega_sinh_alpha
    ]
}
pub(crate) fn second_order_chebyshev2_high_pass_filter_a<F>(
    eight_rate2: F,
    sixteen_rate2: F,
    omega2: F,
    cosh_2alpha: F,
    four_sqrt2_rate_omega_sinh_alpha: F
) -> [F; 3]
where
    F: Float
{
    let omega2_cosh_2alpha = omega2*cosh_2alpha;
    let eight_rate2_p_omega2_cosh_2alpha = eight_rate2 + omega2_cosh_2alpha;
    let two_omega2_cosh_2alpha = omega2_cosh_2alpha + omega2_cosh_2alpha;
    [
        eight_rate2_p_omega2_cosh_2alpha + four_sqrt2_rate_omega_sinh_alpha,
        two_omega2_cosh_2alpha - sixteen_rate2,
        eight_rate2_p_omega2_cosh_2alpha - four_sqrt2_rate_omega_sinh_alpha,
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, params::OmegaEpsilon};

    use super::SecondOrderChebyshev2Filter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev2Filter::new::<All>(OmegaEpsilon::new(10000.0*TAU, 1.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}