use num::Float;

use crate::{conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, ButterworthFilterParam, FilterParamThirdOrder, ThirdOrderButterworthFilterConf}, params::OmegaThirdOrder, real_time_fir_iir_filters};

// TODO: Do it in SOS
crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [Peak](crate::conf::Peak),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak)<1>, [Peak](crate::conf::Peak)<2>, [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                  ω^3
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 1) PEAK 1:
        /// 
        ///                  ω^2s
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 2) PEAK 2:
        /// 
        ///                  ωs^2
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 3) HIGH-PASS:
        /// 
        ///                  s^3
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// ```
    }
    ThirdOrderButterworthFilter
    {
        type Conf: ThirdOrderButterworthFilterConf as ButterworthFilterConf<3>;
        type Param: ButterworthFilterParam = OmegaThirdOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<Peak<1>>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter1_b(two_omega2_rate)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<Peak<2>>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter2_b(four_omega_rate2)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter1_b(two_omega2_rate)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<Peak>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak<1>, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak<2>, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter2_b(four_omega_rate2),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_low_pass_filter_b(omega3, three_omega3),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        where {
            P: FilterParamThirdOrder,
            <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>
        }
        {
            let two_rate = rate + rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let four_omega_rate2 = omega*four_rate2;
            let two_omega2_rate = omega2*two_rate;
            let three_omega3 = three*omega3;
            let twenty_four_rate3 = three*eight_rate3;
            (
                ([], [], [
                    third_order_butterworth_peak_filter1_b(two_omega2_rate),
                    third_order_butterworth_peak_filter2_b(four_omega_rate2),
                    third_order_butterworth_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_butterworth_filter_a(
                        eight_rate3,
                        four_omega_rate2,
                        omega3,
                        two_omega2_rate,
                        three_omega3,
                        twenty_four_rate3
                    )
                ])]
            )
        }
    }
    where
        P: FilterParamThirdOrder,
        <P as ButterworthFilterParam<C>>::Conf: ButterworthFilterConf<3>,
        [(); <CC as ButterworthFilterConf<3>>::OUTPUTS]:
);

pub(crate) fn third_order_butterworth_low_pass_filter_b<F>(omega3: F, three_omega3: F) -> [F; 4]
where
    F: Float
{
    [
        omega3,
        three_omega3,
        three_omega3,
        omega3
    ]
}
pub(crate) fn third_order_butterworth_peak_filter1_b<F>(two_omega2_rate: F) -> [F; 4]
where
    F: Float
{
    let m_two_omega2_rate = -two_omega2_rate;
    [
        two_omega2_rate,
        two_omega2_rate,
        m_two_omega2_rate,
        m_two_omega2_rate
    ]
}
pub(crate) fn third_order_butterworth_peak_filter2_b<F>(four_omega_rate2: F) -> [F; 4]
where
    F: Float
{
    let m_four_omega_rate2 = -four_omega_rate2;
    [
        four_omega_rate2,
        m_four_omega_rate2,
        m_four_omega_rate2,
        four_omega_rate2
    ]
}
pub(crate) fn third_order_butterworth_high_pass_filter_b<F>(eight_rate3: F, twenty_four_rate3: F) -> [F; 4]
where
    F: Float
{
    [
        eight_rate3,
        -twenty_four_rate3,
        twenty_four_rate3,
        -eight_rate3
    ]
}

pub(crate) fn third_order_butterworth_filter_a<F>(
    eight_rate3: F,
    four_omega_rate2: F,
    omega3: F,
    two_omega2_rate: F,
    three_omega3: F,
    twenty_four_rate3: F
) -> [F; 4]
where
    F: Float
{
    let eight_omega_rate2 = four_omega_rate2 + four_omega_rate2;
    let four_omega2_rate = two_omega2_rate + two_omega2_rate;
    let eight_omega_rate2_p_omega3 = eight_omega_rate2 + omega3;
    let eight_rate3_p_four_omega2_rate = eight_rate3 + four_omega2_rate;
    let three_omega3_m_eight_omega_rate2 = three_omega3 - eight_omega_rate2;
    let four_omega2_rate_m_twenty_four_rate3 = four_omega2_rate - twenty_four_rate3;
    [
        eight_omega_rate2_p_omega3 + eight_rate3_p_four_omega2_rate,
        three_omega3_m_eight_omega_rate2 + four_omega2_rate_m_twenty_four_rate3,
        three_omega3_m_eight_omega_rate2 - four_omega2_rate_m_twenty_four_rate3,
        eight_omega_rate2_p_omega3 - eight_rate3_p_four_omega2_rate
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, params::Omega};

    use super::ThirdOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderButterworthFilter::new::<All>(Omega::new(10000.0*TAU));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}