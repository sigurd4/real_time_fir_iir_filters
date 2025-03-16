use num::Float;

use crate::{conf::{All, HighPass, LowPass, Peak}, param::{ThirdOrderFilterConf, ThirdOrderFilterParam}, params::Omega2Zeta, real_time_fir_iir_filters};

// TODO: Do it in SOS
crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [Peak](crate::conf::Peak),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak)<1>, [Peak](crate::conf::Peak)<2>, [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                   ω₁ω₂^2
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 1) PEAK 1:
        /// 
        ///              (ω₁ω₂^2)^(2/3)s
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 2) PEAK 2:
        /// 
        ///              (ω₁ω₂^2)^(1/3)s^2
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 3) HIGH-PASS:
        /// 
        ///                    s^3
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// ```
    }
    ThirdOrderFilter
    {
        type Conf: ThirdOrderFilterConf;
        type Param: ThirdOrderFilterParam = Omega2Zeta;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_peak_filter2_b(four_rate2, k),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<Peak<1>>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter1_b(k2, two_rate)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<Peak<2>>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter2_b(four_rate2, k)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter1_b(k2, two_rate)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter2_b(four_rate2, k)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<Peak>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_peak_filter2_b(four_rate2, k)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak<1>, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak<2>, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter2_b(four_rate2, k),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_peak_filter2_b(four_rate2, k)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;
            let k3 = k2*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_low_pass_filter_b(three, k3),
                    third_order_peak_filter2_b(four_rate2, k),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        {
            let omega1 = param.omega1();
            let omega2 = param.omega2();
            let zeta = param.zeta();

            let omega2_2 = omega2*omega2;

            let k = (omega1*omega2_2).cbrt();
            let k2 = k*k;

            let one = F::one();
            let two = one + one;
            let three = two + one;

            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            let twenty_four_rate3 = three*eight_rate3;

            (
                ([], [], [
                    third_order_peak_filter1_b(k2, two_rate),
                    third_order_peak_filter2_b(four_rate2, k),
                    third_order_high_pass_filter_b(eight_rate3, twenty_four_rate3)
                ]),
                [([], [
                    third_order_filter_a(
                        three,
                        rate,
                        two_rate,
                        four_rate,
                        eight_rate3,
                        twenty_four_rate3,
                        omega1,
                        omega2,
                        omega2_2,
                        zeta
                    )
                ])]
            )
        }
    }
    where
        [(); <CC as ThirdOrderFilterConf>::OUTPUTS]:
);

pub(crate) fn third_order_low_pass_filter_b<F>(three: F, k3: F) -> [F; 4]
where
    F: Float
{
    let three_k3 = three*k3;
    [
        k3,
        three_k3,
        three_k3,
        k3
    ]
}
pub(crate) fn third_order_peak_filter1_b<F>(k2: F, two_rate: F) -> [F; 4]
where
    F: Float
{
    let two_k2_rate = k2*two_rate;
    let m_two_k2_rate = -two_k2_rate;
    [
        two_k2_rate,
        two_k2_rate,
        m_two_k2_rate,
        m_two_k2_rate
    ]
}
pub(crate) fn third_order_peak_filter2_b<F>(four_rate2: F, k: F) -> [F; 4]
where
    F: Float
{
    let four_k_rate2 = k*four_rate2;
    let m_four_k_rate2 = -four_k_rate2;
    [
        four_k_rate2,
        m_four_k_rate2,
        m_four_k_rate2,
        four_k_rate2
    ]
}
pub(crate) fn third_order_high_pass_filter_b<F>(eight_rate3: F, twenty_four_rate3: F) -> [F; 4]
where
    F: Float
{
    [
        eight_rate3,
        -twenty_four_rate3,
        twenty_four_rate3,
        -eight_rate3,
    ]
}

pub(crate) fn third_order_filter_a<F>(
    three: F,
    rate: F,
    two_rate: F,
    four_rate: F,
    eight_rate3: F,
    twenty_four_rate3: F,
    omega1: F,
    omega2: F,
    omega2_2: F,
    zeta: F
) -> [F; 4]
where
    F: Float
{
    let four_rate_omega1 = four_rate*omega1;
    let omega2_zeta = omega2*zeta;
    let omega2_2_omega1 = omega2_2*omega1;

    let a_lhs_rhs = four_rate_omega1*rate + four_rate*two_rate*omega2_zeta;
    let a_rhs_lhs = omega2_2*two_rate + four_rate_omega1*omega2_zeta;

    let a0_lhs = omega2_2_omega1 + a_lhs_rhs;
    let a0_rhs = a_rhs_lhs + eight_rate3;

    let a1_lhs = three*omega2_2_omega1 - a_lhs_rhs;
    let a1_rhs = a_rhs_lhs - twenty_four_rate3;

    [
        a0_lhs + a0_rhs,
        a1_lhs + a1_rhs,
        a1_lhs - a1_rhs,
        a0_lhs - a0_rhs,
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, params::Omega2Zeta};

    use super::ThirdOrderFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderFilter::new::<All>(Omega2Zeta::new(1000.0*TAU, 10000.0*TAU, 0.1));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}