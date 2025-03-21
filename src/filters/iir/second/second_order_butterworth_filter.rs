use num::{traits::FloatConst, Float};

use crate::{conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, ButterworthFilterParam, OmegaSecondOrder, SecondOrderButterworthFilterConf}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                ω^2
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// 
        /// 1) PEAK:
        /// 
        ///                ωs
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///                s^2
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// ```
    }
    SecondOrderButterworthFilter
    {
        type Conf: SecondOrderButterworthFilterConf as ButterworthFilterConf<2>;
        type Param: ButterworthFilterParam = OmegaSecondOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_low_pass_filter_b(omega2, two_omega2),
                    second_order_butterworth_peak_filter_b(rate, omega),
                    second_order_butterworth_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_low_pass_filter_b(omega2, two_omega2)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<Peak>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_peak_filter_b(rate, omega)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_low_pass_filter_b(omega2, two_omega2),
                    second_order_butterworth_peak_filter_b(rate, omega)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_low_pass_filter_b(omega2, two_omega2),
                    second_order_butterworth_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        where {
            P: FilterParamSecondOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>
        }
        {
            let omega = param.omega();
            let two_omega = omega + omega;
            let omega2 = omega*omega;
            let two_omega2 = two_omega*omega;
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate2 = four_rate2 + four_rate2;
            (
                ([], [], [
                    second_order_butterworth_peak_filter_b(rate, omega),
                    second_order_butterworth_high_pass_filter_b(four_rate2, eight_rate2)
                ]),
                [([], [
                    second_order_butterworth_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2)
                ])]
            )
        }
    }
    where
        P: FilterParamSecondOrder,
        <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<2>,
        [(); <CC as ButterworthFilterConf<2>>::OUTPUTS]:
);

pub fn second_order_butterworth_low_pass_filter_b<F>(omega2: F, two_omega2: F) -> [F; 3]
where
    F: Float
{
    super::second_order_low_pass_filter_b(omega2, two_omega2)
}
pub fn second_order_butterworth_peak_filter_b<F>(rate: F, omega: F) -> [F; 3]
where
    F: Float
{
    super::second_order_peak_filter_b(rate, omega)
}
pub fn second_order_butterworth_high_pass_filter_b<F>(four_rate2: F, eight_rate2: F) -> [F; 3]
where
    F: Float
{
    super::second_order_high_pass_filter_b(four_rate2, eight_rate2)
}
pub fn second_order_butterworth_filter_a<F>(two_rate: F, four_rate2: F, eight_rate2: F, two_omega: F, omega2: F, two_omega2: F) -> [F; 3]
where
    F: Float + FloatConst
{
    super::second_order_filter_a(two_rate, four_rate2, eight_rate2, two_omega, omega2, two_omega2, F::FRAC_1_SQRT_2())
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, params::Omega};

    use super::SecondOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderButterworthFilter::new::<All>(Omega::new(10000.0*TAU));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}