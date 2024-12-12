use num::Float;

use crate::{conf::{All, BandPass, BandStop, HighPass, LowPass}, param::{SecondOrderRLCFilterConf, SecondOrderRLCFilterParam}, params::RLC, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [BandStop](crate::conf::BandStop), [BandPass](crate::conf::BandPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[R]-[L]-Y
        ///               |
        ///              [C]
        ///               |
        ///              GND
        /// 
        /// 1) BAND-STOP:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 
        /// 2) BAND-PASS:
        ///     X-[C]-[L]-Y
        ///               |
        ///              [R]
        ///               |
        ///              GND
        /// 
        /// 3) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// ```
    }
    SecondOrderRLCFilter
    {
        type Conf: SecondOrderRLCFilterConf;
        type Param: SecondOrderRLCFilterParam = RLC;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2),
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<BandStop>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_pass_filter_b(two_c_r_rate)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(BandStop, BandPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(BandStop, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_pass_filter_b(two_c_r_rate),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop, BandPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_low_pass_filter_b(one, two),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
        fn make_coeffs<(BandStop, BandPass, HighPass)>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let c = param.c();

            let one = F::one();
            let two = one + one;

            let c_rate = c*rate;
            let l_rate = l*rate;
            let two_c_rate = c_rate + c_rate;
            let two_l_rate = l_rate + l_rate;

            let two_c_r_rate = two_c_rate*r;
            let four_c_l_rate2 = two_c_rate*two_l_rate;
            let eight_c_l_rate2 = four_c_l_rate2 + four_c_l_rate2;
            let one_p_four_c_l_rate2 = one + four_c_l_rate2;
            let two_m_eight_c_l_rate2 = two - eight_c_l_rate2;
            (
                ([], [], [
                    second_order_rlc_band_stop_filter_b(one_p_four_c_l_rate2, two_m_eight_c_l_rate2),
                    second_order_rlc_band_pass_filter_b(two_c_r_rate),
                    second_order_rlc_high_pass_filter_b(four_c_l_rate2, two_c_r_rate, eight_c_l_rate2)
                ]),
                [([], [
                    second_order_rlc_filter_a(one_p_four_c_l_rate2, two_c_r_rate, two_m_eight_c_l_rate2)
                ])]
            )
        }
    }
    where
        [(); <CC as SecondOrderRLCFilterConf>::OUTPUTS]:
);

pub(crate) fn second_order_rlc_low_pass_filter_b<F>(one: F, two: F) -> [F; 3]
where
    F: Float
{
    [
        one,
        two,
        one,
    ]
}
pub(crate) fn second_order_rlc_band_stop_filter_b<F>(one_p_four_c_l_rate2: F, two_m_eight_c_l_rate2: F) -> [F; 3]
where
    F: Float
{
    [
        one_p_four_c_l_rate2,
        two_m_eight_c_l_rate2,
        one_p_four_c_l_rate2,
    ]
}
pub(crate) fn second_order_rlc_band_pass_filter_b<F>(two_c_r_rate: F) -> [F; 3]
where
    F: Float
{
    [
        two_c_r_rate,
        F::zero(),
        -two_c_r_rate,
    ]
}
pub(crate) fn second_order_rlc_high_pass_filter_b<F>(four_c_l_rate2: F, two_c_r_rate: F, eight_c_l_rate2: F) -> [F; 3]
where
    F: Float
{
    [
        four_c_l_rate2 + two_c_r_rate,
        -eight_c_l_rate2,
        four_c_l_rate2 + two_c_r_rate,
    ]
}
pub(crate) fn second_order_rlc_filter_a<F>(one_p_four_c_l_rate2: F, two_c_r_rate: F, two_m_eight_c_l_rate2: F) -> [F; 3]
where
    F: Float
{
    [
        one_p_four_c_l_rate2 + two_c_r_rate,
        two_m_eight_c_l_rate2,
        one_p_four_c_l_rate2 - two_c_r_rate,
    ]
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderRLCFilter, RLC};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRLCFilter::new::<All>(RLC::new(1000.0, 0.01, 0.000000033));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}