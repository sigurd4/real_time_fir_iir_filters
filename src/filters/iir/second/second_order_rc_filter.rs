use num::Float;

use crate::{conf::{All, BandPass, HighPass, LowPass}, param::{SecondOrderRCFilterConf, SecondOrderRCFilterParam}, params::RC2, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [BandPass](crate::conf::BandPass),
        /// [LowPass](crate::conf::LowPass), [BandPass](crate::conf::BandPass)<1>, [BandPass](crate::conf::BandPass)<2>, [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[R1]-o-[R2]-Y
        ///            |      |
        ///           [C1]   [C2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 1) BAND-PASS 1:
        ///     X-[C1]-o-[R2]-Y
        ///            |      |
        ///           [R1]   [C2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 2) BAND-PASS 2
        ///     X-[R1]-o-[C2]-Y
        ///            |      |
        ///           [C1]   [R2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 3) HIGH-PASS
        ///     X-[C1]-o-[C2]-Y
        ///            |      |
        ///           [R1]   [R2]
        ///            |      |
        ///           GND    GND
        /// ```
    }
    SecondOrderRCFilter
    {
        type Conf: SecondOrderRCFilterConf;
        type Param: SecondOrderRCFilterParam = RC2;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<BandPass<1>>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<BandPass<2>>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(BandPass<1>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(BandPass<2>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let one = F::one();
            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_low_pass_filter_b(one),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();

            let zero = F::zero();

            let two_rate = rate + rate;
            let two_r1_rate = r1*two_rate;
            let two_c1_r1_rate = c1*two_r1_rate;
            let two_c2_r2_rate = c2*r2*two_rate;
            let two_c2_r1_rate = c2*two_r1_rate;
            (
                ([], [], [
                    second_order_rc_band_pass_filter1_b(two_c1_r1_rate, zero),
                    second_order_rc_band_pass_filter2_b(two_c2_r2_rate, zero),
                    second_order_rc_high_pass_filter_b(two_c1_r1_rate, two_c2_r2_rate)
                ]),
                [([], [
                    second_order_rc_filter_a(two_c1_r1_rate, two_c2_r2_rate, two_c2_r1_rate)
                ])]
            )
        }
    }
    where
        [(); <CC as SecondOrderRCFilterConf>::OUTPUTS]:
);

pub(crate) fn second_order_rc_low_pass_filter_b<F>(one: F) -> [F; 3]
where
    F: Float
{
    [
        one,
        one + one,
        one,
    ]
}
pub(crate) fn second_order_rc_band_pass_filter1_b<F>(two_c1_r1_rate: F, zero: F) -> [F; 3]
where
    F: Float
{
    [
        two_c1_r1_rate,
        zero,
        -two_c1_r1_rate,
    ]
}
pub(crate) fn second_order_rc_band_pass_filter2_b<F>(two_c2_r2_rate: F, zero: F) -> [F; 3]
where
    F: Float
{
    [
        two_c2_r2_rate,
        zero,
        -two_c2_r2_rate,
    ]
}
pub(crate) fn second_order_rc_high_pass_filter_b<F>(two_c1_r1_rate: F, two_c2_r2_rate: F) -> [F; 3]
where
    F: Float
{
    let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
    let eight_c1_c2_r1_r2_rate2 = four_c1_c2_r1_r2_rate2 + four_c1_c2_r1_r2_rate2;
    [
        four_c1_c2_r1_r2_rate2,
        -eight_c1_c2_r1_r2_rate2,
        four_c1_c2_r1_r2_rate2,
    ]
}
pub(crate) fn second_order_rc_filter_a<F>(two_c1_r1_rate: F, two_c2_r2_rate: F, two_c2_r1_rate: F) -> [F; 3]
where
    F: Float
{
    let one = F::one();
    let two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate = two_c2_r2_rate + two_c2_r1_rate + two_c1_r1_rate;
    let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
    let four_c1_c2_r1_r2_rate2_p_one = four_c1_c2_r1_r2_rate2 + one;
    let one_m_four_c1_c2_r1_r2_rate2 = one - four_c1_c2_r1_r2_rate2;
    [
        four_c1_c2_r1_r2_rate2_p_one + two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate,
        one_m_four_c1_c2_r1_r2_rate2 + one_m_four_c1_c2_r1_r2_rate2,
        four_c1_c2_r1_r2_rate2_p_one - two_c2_r2_rate_p_two_c2_r1_rate_p_two_c1_r1_rate,
    ]
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderRCFilter, RC2};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRCFilter::new::<All>(RC2::new(390e3, 100e-9, 4.7e3, 47e-12));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}