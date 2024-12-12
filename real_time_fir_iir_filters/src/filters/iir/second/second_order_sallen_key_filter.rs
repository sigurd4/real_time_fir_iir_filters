use num::Float;

use crate::{conf::{All, BandPass, HighPass, LowPass}, param::{SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam}, params::RC2GSallenKey, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [BandPass](crate::conf::BandPass),
        /// [LowPass](crate::conf::LowPass), [BandPass](crate::conf::BandPass)<1>, [BandPass](crate::conf::BandPass)<2>, [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///            o------------o
        ///            |            |
        ///           [C1]          |
        ///            |            |
        ///     X-[R1]-o-[R2]-o-[G>-Y
        ///                   |
        ///                  [C2]
        ///                   |
        ///                  GND
        /// 1) BAND-PASS 1:
        ///            o------------o
        ///            |            |
        ///           [R1]          |
        ///            |            |
        ///     X-[C1]-o-[R2]-o-[G>-Y
        ///                   |
        ///                  [C2]
        ///                   |
        ///                  GND
        /// 2) BAND-PASS 2:
        ///            o------------o
        ///            |            |
        ///           [C1]          |
        ///            |            |
        ///     X-[R1]-o-[C2]-o-[G>-Y
        ///                   |
        ///                  [R2]
        ///                   |
        ///                  GND
        /// 3) HIGH-PASS:
        ///            o------------o
        ///            |            |
        ///           [R1]          |
        ///            |            |
        ///     X-[C1]-o-[C2]-o-[G>-Y
        ///                   |
        ///                  [R2]
        ///                   |
        ///                  GND
        /// ```
    }
    SecondOrderSallenKeyFilter
    {
        type Conf: SecondOrderSallenKeyFilterConf;
        type Param: SecondOrderSallenKeyFilterParam = RC2GSallenKey;

        const O_BUFFERS: usize = <CC as SecondOrderSallenKeyFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<BandPass<1>>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<BandPass<2>>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(BandPass<1>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(BandPass<2>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_low_pass_filter_b(g, two_g),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_low_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c1_r1_g_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();

            let zero = F::zero();
            let one = F::one();
            let two = one + one;

            let two_rate = two*rate;
            let two_g = two*g;
            
            let two_c1_r1_rate = c1*r1*two_rate;
            let two_c2_r2_rate = c2*r2*two_rate;

            let two_c1_r1_g_rate = two_c1_r1_rate*g;
            let two_c2_r2_g_rate = two_c2_r2_rate*g;

            let four_c1_c2_r1_r2_rate2 = two_c1_r1_rate*two_c2_r2_rate;
            let four_c1_c2_r1_r2_g_rate2 = four_c1_c2_r1_r2_rate2*g;
            let eight_c1_c2_r1_r2_g_rate2 = two*four_c1_c2_r1_r2_g_rate2;

            let one_p_four_c1_c2_r1_r2_rate2 = one + four_c1_c2_r1_r2_rate2;
            let two_m_eight_c1_c2_r1_r2_rate2 = two - two*four_c1_c2_r1_r2_rate2;

            let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate =
                c2*r1*two_rate + two_c2_r2_rate + two_c1_r1_rate;
            (
                ([], [], [
                    second_order_sallen_key_band_pass_filter1_b(zero, two_c1_r1_g_rate),
                    second_order_sallen_key_band_pass_filter2_b(zero, two_c2_r2_g_rate),
                    second_order_sallen_key_high_pass_filter_b(four_c1_c2_r1_r2_g_rate2, eight_c1_c2_r1_r2_g_rate2)
                ]),
                [([], [
                    second_order_sallen_key_band_pass_filter1_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        g,
                        two_g,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_band_pass_filter2_a(
                        two,
                        one_p_four_c1_c2_r1_r2_rate2,
                        four_c1_c2_r1_r2_g_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate
                    ),
                    second_order_sallen_key_high_pass_filter_a(
                        one_p_four_c1_c2_r1_r2_rate2,
                        two_m_eight_c1_c2_r1_r2_rate2,
                        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
                        two_c2_r2_g_rate
                    )
                ])]
            )
        }
    }
    where
        [(); <CC as SecondOrderSallenKeyFilterConf>::OUTPUTS]:
);

pub(crate) fn second_order_sallen_key_low_pass_filter_b<F>(g: F, two_g: F) -> [F; 3]
where
    F: Float
{
    let g2 = g*g;
    [
        g2,
        two_g,
        g2,
    ]
}
pub(crate) fn second_order_sallen_key_band_pass_filter1_b<F>(zero: F, two_c1_r1_g_rate: F) -> [F; 3]
where
    F: Float
{
    [
        two_c1_r1_g_rate,
        zero,
        -two_c1_r1_g_rate,
    ]
}
pub(crate) fn second_order_sallen_key_band_pass_filter2_b<F>(zero: F, two_c2_r2_g_rate: F) -> [F; 3]
where
    F: Float
{
    [
        two_c2_r2_g_rate,
        zero,
        -two_c2_r2_g_rate,
    ]
}
pub(crate) fn second_order_sallen_key_high_pass_filter_b<F>(four_c1_c2_r1_r2_g_rate2: F, eight_c1_c2_r1_r2_g_rate2: F) -> [F; 3]
where
    F: Float
{
    [
        four_c1_c2_r1_r2_g_rate2,
        -eight_c1_c2_r1_r2_g_rate2,
        four_c1_c2_r1_r2_g_rate2,
    ]
}

pub(crate) fn second_order_sallen_key_low_pass_filter_a<F>(
    one_p_four_c1_c2_r1_r2_rate2: F,
    two_m_eight_c1_c2_r1_r2_rate2: F,
    two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate: F,
    two_c1_r1_g_rate: F
) -> [F; 3]
where
    F: Float
{
    let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate =
        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate - two_c1_r1_g_rate;
    [
        one_p_four_c1_c2_r1_r2_rate2 + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate,
        two_m_eight_c1_c2_r1_r2_rate2,
        one_p_four_c1_c2_r1_r2_rate2 - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_p_two_c1_r1_g_rate,
    ]
}
pub(crate) fn second_order_sallen_key_band_pass_filter1_a<F>(
    one_p_four_c1_c2_r1_r2_rate2: F,
    two_m_eight_c1_c2_r1_r2_rate2: F,
    g: F,
    two_g: F,
    two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate: F
) -> [F; 3]
where
    F: Float
{
    let one_p_four_c1_c2_r1_r2_rate2_m_g =
        one_p_four_c1_c2_r1_r2_rate2 - g;
    [
        one_p_four_c1_c2_r1_r2_rate2_m_g + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
        two_m_eight_c1_c2_r1_r2_rate2 - two_g,
        one_p_four_c1_c2_r1_r2_rate2_m_g - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
    ]
}
pub(crate) fn second_order_sallen_key_band_pass_filter2_a<F>(
    two: F,
    one_p_four_c1_c2_r1_r2_rate2: F,
    four_c1_c2_r1_r2_g_rate2: F,
    two_m_eight_c1_c2_r1_r2_rate2: F,
    two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate: F
) -> [F; 3]
where
    F: Float
{
    let one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 =
        one_p_four_c1_c2_r1_r2_rate2 - four_c1_c2_r1_r2_g_rate2;
    [
        one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
        two_m_eight_c1_c2_r1_r2_rate2 + two*four_c1_c2_r1_r2_g_rate2,
        one_p_four_c1_c2_r1_r2_rate2_m_four_c1_c2_r1_r2_g_rate2 - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate,
    ]
}
pub(crate) fn second_order_sallen_key_high_pass_filter_a<F>(
    one_p_four_c1_c2_r1_r2_rate2: F,
    two_m_eight_c1_c2_r1_r2_rate2: F,
    two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate: F,
    two_c2_r2_g_rate: F
) -> [F; 3]
where
    F: Float
{
    let two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate =
        two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate - two_c2_r2_g_rate;
    [
        one_p_four_c1_c2_r1_r2_rate2 + two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate,
        two_m_eight_c1_c2_r1_r2_rate2,
        one_p_four_c1_c2_r1_r2_rate2 - two_c2_r1_rate_p_two_c2_r2_rate_p_two_c1_r1_rate_m_two_c2_r2_g_rate,
    ]
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderSallenKeyFilter, RC2GSallenKey};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderSallenKeyFilter::new::<All>(RC2GSallenKey::new(15.0e3, 2.7e-9, 15.0e3, 2.7e-9, 2.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}