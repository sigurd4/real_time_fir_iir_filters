use num::{Complex, Float};

use crate::{conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, EllipticFilterParam, FilterParamSecondOrder}, params::OmegaEpsilonXiSecondOrder, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                     1
        /// |H(s)| = -----------------------
        ///          √(1 + ε^2R_2^2(ξ, s/ω))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///                     1
        /// |H(s)| = -----------------------
        ///          √(1 + ε^2R_2^2(ξ, ω/s))
        /// ```
    }
    SecondOrderEllipticFilter
    {
        type Conf: EllipticFilterConf;
        type Param: EllipticFilterParam = OmegaEpsilonXiSecondOrder;

        const O_BUFFERS: usize = <CC as EllipticFilterConf>::OUTPUTS;
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
            let xi = param.xi();
    
            let xi2 = xi*xi;
            let xi3 = xi2*xi;
    
            let t = (F::one() - xi3.recip()).sqrt();
            let tm1 = t - F::one();
            let tp1 = t + F::one();

            let s1 = -(Complex::new(epsilon, F::one())/Complex::new(-epsilon*tp1, tm1)).sqrt();
            let s2 = s1.conj();
            let s1_s2 = (s1*s2).re;
            let s1ps2 = (s1 + s2).re;
    
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let omega2 = omega*omega;
            let epsilon2 = epsilon*epsilon;
            let two_rate_s1ps2_omega = two_rate*s1ps2*omega;
            
            let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();
            (
                ([], [], [
                    second_order_elliptic_low_pass_filter_b(omega2, four_rate2, tm1, g),
                    second_order_elliptic_high_pass_filter_b(omega2, four_rate2, tm1, g)
                ]),
                [([], [
                    second_order_elliptic_low_pass_filter_a(omega2, four_rate2, two_rate_s1ps2_omega, s1_s2),
                    second_order_elliptic_high_pass_filter_a(omega2, four_rate2, two_rate_s1ps2_omega, s1_s2)
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
            let xi = param.xi();
    
            let xi2 = xi*xi;
            let xi3 = xi2*xi;
    
            let t = (F::one() - xi3.recip()).sqrt();
            let tm1 = t - F::one();
            let tp1 = t + F::one();

            let s1 = -(Complex::new(epsilon, F::one())/Complex::new(-epsilon*tp1, tm1)).sqrt();
            let s2 = s1.conj();
            let s1_s2 = (s1*s2).re;
            let s1ps2 = (s1 + s2).re;
    
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let omega2 = omega*omega;
            let epsilon2 = epsilon*epsilon;
            let two_rate_s1ps2_omega = two_rate*s1ps2*omega;
            
            let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();
            (
                ([], [], [
                    second_order_elliptic_low_pass_filter_b(omega2, four_rate2, tm1, g)
                ]),
                [([], [
                    second_order_elliptic_low_pass_filter_a(omega2, four_rate2, two_rate_s1ps2_omega, s1_s2)
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
            let xi = param.xi();
    
            let xi2 = xi*xi;
            let xi3 = xi2*xi;
    
            let t = (F::one() - xi3.recip()).sqrt();
            let tm1 = t - F::one();
            let tp1 = t + F::one();

            let s1 = -(Complex::new(epsilon, F::one())/Complex::new(-epsilon*tp1, tm1)).sqrt();
            let s2 = s1.conj();
            let s1_s2 = (s1*s2).re;
            let s1ps2 = (s1 + s2).re;
    
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let omega2 = omega*omega;
            let epsilon2 = epsilon*epsilon;
            let two_rate_s1ps2_omega = two_rate*s1ps2*omega;
            
            let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();
            (
                ([], [], [
                    second_order_elliptic_high_pass_filter_b(omega2, four_rate2, tm1, g)
                ]),
                [([], [
                    second_order_elliptic_high_pass_filter_a(omega2, four_rate2, two_rate_s1ps2_omega, s1_s2)
                ])]
            )
        }
    }
    where
        P: FilterParamSecondOrder,
        [(); <CC as EllipticFilterConf>::OUTPUTS]:
);

pub(crate) fn second_order_elliptic_low_pass_filter_b<F>(omega2: F, four_rate2: F, tm1: F, g: F) -> [F; 3]
where
    F: Float
{
    let four_rate2_tm1 = four_rate2*tm1;
    let omega2_m_four_rate2_tm1_g = (omega2 - four_rate2_tm1)*g;
    let omega2_p_four_rate2_tm1_g = (omega2 + four_rate2_tm1)*g;
    [
        omega2_m_four_rate2_tm1_g,
        omega2_p_four_rate2_tm1_g + omega2_p_four_rate2_tm1_g,
        omega2_m_four_rate2_tm1_g
    ]
}
pub(crate) fn second_order_elliptic_high_pass_filter_b<F>(omega2: F, four_rate2: F, tm1: F, g: F) -> [F; 3]
where
    F: Float
{
    let tm1_omega2 = tm1*omega2;
    let tm1_omega2_m_four_rate2_g = (tm1_omega2 - four_rate2)*g;
    let tm1_omega2_p_four_rate2_g = (tm1_omega2 + four_rate2)*g;
    [
        tm1_omega2_m_four_rate2_g,
        tm1_omega2_p_four_rate2_g + tm1_omega2_p_four_rate2_g,
        tm1_omega2_m_four_rate2_g
    ]
}
pub(crate) fn second_order_elliptic_low_pass_filter_a<F>(omega2: F, four_rate2: F, two_rate_s1ps2_omega: F, s1_s2: F) -> [F; 3]
where
    F: Float
{
    let s1_s2_omega2 = s1_s2*omega2;
    let four_rate2_p_s1_s2_omega2 = four_rate2 + s1_s2_omega2;
    let s1_s2_omega2_m_four_rate2 = s1_s2_omega2 - four_rate2;
    [
        four_rate2_p_s1_s2_omega2 - two_rate_s1ps2_omega,
        s1_s2_omega2_m_four_rate2 + s1_s2_omega2_m_four_rate2,
        four_rate2_p_s1_s2_omega2 + two_rate_s1ps2_omega
    ]
}
pub(crate) fn second_order_elliptic_high_pass_filter_a<F>(omega2: F, four_rate2: F, two_rate_s1ps2_omega: F, s1_s2: F) -> [F; 3]
where
    F: Float
{
    let four_rate2_s1_s2 = four_rate2*s1_s2;
    let omega2_p_four_rate2_s1_s2 = omega2 + four_rate2_s1_s2;
    let omega2_m_four_rate2_s1_s2 = omega2 - four_rate2_s1_s2;
    [
        omega2_p_four_rate2_s1_s2 - two_rate_s1ps2_omega,
        omega2_m_four_rate2_s1_s2 + omega2_m_four_rate2_s1_s2,
        omega2_p_four_rate2_s1_s2 + two_rate_s1ps2_omega
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, params::OmegaEpsilonXi};

    use super::SecondOrderEllipticFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderEllipticFilter::new::<All>(OmegaEpsilonXi::new(10000.0*TAU, 0.5, 1.5));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}