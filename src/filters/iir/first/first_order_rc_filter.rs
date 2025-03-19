use num::Float;

use crate::{conf::{All, HighPass, LowPass}, param::{FirstOrderRCFilterConf, FirstOrderRCFilterParam, RCVal}, params::RC, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 
        ///           1
        /// H(s) = -------
        ///        RCs + 1
        /// 
        /// 1) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// 
        ///          RCs
        /// H(s) = -------
        ///        RCs + 1
        /// ```
    }
    FirstOrderRCFilter
    {
        type Conf: FirstOrderRCFilterConf;
        type Param: FirstOrderRCFilterParam = RC;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let RCVal {r, c} = param.rc();

            let rate_d_omega = rate*r*c;
            let two_rate_d_omega = rate_d_omega + rate_d_omega;
            let one = F::one();
            (
                ([], [], [
                    first_order_rc_low_pass_filter_b(one),
                    first_order_rc_high_pass_filter_b(two_rate_d_omega)
                ]),
                [([], [
                    first_order_rc_filter_a(one, two_rate_d_omega)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let RCVal {r, c} = param.rc();

            let rate_d_omega = rate*r*c;
            let two_rate_d_omega = rate_d_omega + rate_d_omega;
            let one = F::one();
            (
                ([], [], [
                    first_order_rc_low_pass_filter_b(one)
                ]),
                [([], [
                    first_order_rc_filter_a(one, two_rate_d_omega)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let RCVal {r, c} = param.rc();

            let rate_d_omega = rate*r*c;
            let two_rate_d_omega = rate_d_omega + rate_d_omega;
            let one = F::one();
            (
                ([], [], [
                    first_order_rc_high_pass_filter_b(two_rate_d_omega)
                ]),
                [([], [
                    first_order_rc_filter_a(one, two_rate_d_omega)
                ])]
            )
        }
    }
    where
        [(); <CC as FirstOrderRCFilterConf>::OUTPUTS]:
);

pub(crate) fn first_order_rc_low_pass_filter_b<F>(one: F) -> [F; 2]
where
    F: Float
{
    [
        one,
        one
    ]
}
pub(crate) fn first_order_rc_high_pass_filter_b<F>(two_rate_d_omega: F) -> [F; 2]
where
    F: Float
{
    [
        two_rate_d_omega,
        -two_rate_d_omega
    ]
}
pub(crate) fn first_order_rc_filter_a<F>(one: F, two_rate_d_omega: F) -> [F; 2]
where
    F: Float
{
    [
        one + two_rate_d_omega,
        one - two_rate_d_omega,
    ]
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderRCFilter, RC};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::<_, _, All>::new(RC::new(10000.0, 0.000000033));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}