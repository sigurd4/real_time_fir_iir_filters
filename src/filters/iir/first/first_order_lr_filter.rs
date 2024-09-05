use num::Float;

use crate::{conf::{All, HighPass, LowPass}, param::{FirstOrderLRFilterConf, FirstOrderLRFilterParam}, params::LR, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[L]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// 
        ///           R
        /// H(s) = ------
        ///        Ls + R
        /// 
        /// 1) HIGH-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          GND
        /// 
        ///          Ls
        /// H(s) = ------
        ///        Ls + R
        /// ```
    }
    FirstOrderLRFilter
    {
        type Conf: FirstOrderLRFilterConf;
        type Param: FirstOrderLRFilterParam = LR;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            
            let rate_l = rate*l;
            let two_rate_l = rate_l + rate_l;
            (
                ([], [], [
                    first_order_lr_low_pass_filter_b(r),
                    first_order_lr_high_pass_filter_b(two_rate_l)
                ]),
                [([], [
                    first_order_lr_filter_a(r, two_rate_l)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            
            let rate_l = rate*l;
            let two_rate_l = rate_l + rate_l;
            (
                ([], [], [
                    first_order_lr_low_pass_filter_b(r)
                ]),
                [([], [
                    first_order_lr_filter_a(r, two_rate_l)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            
            let rate_l = rate*l;
            let two_rate_l = rate_l + rate_l;
            (
                ([], [], [
                    first_order_lr_high_pass_filter_b(two_rate_l)
                ]),
                [([], [
                    first_order_lr_filter_a(r, two_rate_l)
                ])]
            )
        }
    }
    where
        [(); <CC as FirstOrderLRFilterConf>::OUTPUTS]:
);

pub(crate) fn first_order_lr_low_pass_filter_b<F>(r: F) -> [F; 2]
where
    F: Float
{
    [
        r,
        r
    ]
}
pub(crate) fn first_order_lr_high_pass_filter_b<F>(two_rate_l: F) -> [F; 2]
where
    F: Float
{
    [
        two_rate_l,
        -two_rate_l
    ]
}
pub(crate) fn first_order_lr_filter_a<F>(r: F, two_rate_l: F) -> [F; 2]
where
    F: Float
{
    [
        r + two_rate_l,
        r - two_rate_l
    ]
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderLRFilter, LR};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderLRFilter::new::<All>(LR::new(100e-3, 10e3));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}