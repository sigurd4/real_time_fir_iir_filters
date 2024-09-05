use num::Float;

use crate::{conf::{All, HighPass, LowPass, Peak}, param::{SecondOrderFilterConf, SecondOrderFilterParam}, params::OmegaZeta, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///              ω^2
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// 
        /// 1) PEAK:
        /// 
        ///               ωs
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///              s^2
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// ```
    }
    SecondOrderFilter
    {
        type Conf: SecondOrderFilterConf;
        type Param: SecondOrderFilterParam = OmegaZeta;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_low_pass_filter_b(omega2, two_omega2),
                    second_order_peak_filter_b(rate, omega),
                    second_order_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_low_pass_filter_b(omega2, two_omega2)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<Peak>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_peak_filter_b(rate, omega)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_low_pass_filter_b(omega2, two_omega2),
                    second_order_peak_filter_b(rate, omega)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_low_pass_filter_b(omega2, two_omega2),
                    second_order_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let zeta = param.zeta();
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_peak_filter_b(rate, omega),
                    second_order_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, zeta)
                ])]
            )
        }
    }
    where
        [(); <<<P as SecondOrderFilterParam<C>>::Conf as SecondOrderFilterConf>::Conf as SecondOrderFilterConf>::OUTPUTS]:
);

pub(crate) fn second_order_low_pass_filter_b<F>(omega2: F, two_omega2: F) -> [F; 3]
where
    F: Float
{
    [
        omega2,
        two_omega2,
        omega2
    ]
}
pub(crate) fn second_order_peak_filter_b<F>(rate: F, omega: F) -> [F; 3]
where
    F: Float
{
    let rate_omega = rate*omega;
    let two_rate_omega = rate_omega + rate_omega;
    [
        two_rate_omega,
        F::zero(),
        -two_rate_omega
    ]
}
pub(crate) fn second_order_high_pass_filter_b<F>(four_rate2: F, eight_rate2: F) -> [F; 3]
where
    F: Float
{
    [
        four_rate2,
        -eight_rate2,
        four_rate2
    ]
}
pub(crate) fn second_order_filter_a<F>(two_rate: F, four_rate2: F, eight_rate2: F, two_omega: F, omega2: F, two_omega2: F, zeta: F) -> [F; 3]
where
    F: Float
{
    let four_rate2_p_omega2 = four_rate2 + omega2;
    let four_rate_zeta_omega = two_rate*zeta*two_omega;
    [
        four_rate2_p_omega2 + four_rate_zeta_omega,
        two_omega2 - eight_rate2,
        four_rate2_p_omega2 - four_rate_zeta_omega
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::conf::All;

    use super::{OmegaZeta, SecondOrderFilter};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderFilter::new::<All>(OmegaZeta::new(10000.0*TAU, 0.05));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}