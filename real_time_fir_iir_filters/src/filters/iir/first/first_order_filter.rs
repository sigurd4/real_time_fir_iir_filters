use num::Float;

use crate::{conf::{All, HighPass, LowPass}, param::{FirstOrderFilterConf, FirstOrderFilterParam}, params::OmegaFirstOrder, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///          ω
        /// H(s) = -----
        ///        s + ω
        /// 
        /// 1) HIGH-PASS
        /// 
        ///          s
        /// H(s) = -----
        ///        s + ω
        /// 
        /// ```
    }
    FirstOrderFilter
    {
        type Conf: FirstOrderFilterConf;
        type Param: FirstOrderFilterParam = OmegaFirstOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let omega = param.omega();
            let two_rate = rate + rate;
            (
                ([], [], [
                    first_order_low_pass_filter_b(omega),
                    first_order_high_pass_filter_b(two_rate)
                ]),
                [([], [
                    first_order_filter_a(omega, two_rate)
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let omega = param.omega();
            let two_rate = rate + rate;
            (
                ([], [], [
                    first_order_low_pass_filter_b(omega)
                ]),
                [([], [
                    first_order_filter_a(omega, two_rate)
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let omega = param.omega();
            let two_rate = rate + rate;
            (
                ([], [], [
                    first_order_high_pass_filter_b(omega)
                ]),
                [([], [
                    first_order_filter_a(omega, two_rate)
                ])]
            )
        }
    }
    where
        [(); <CC as FirstOrderFilterConf>::OUTPUTS]:
);

pub(crate) fn first_order_low_pass_filter_b<F>(omega: F) -> [F; 2]
where
    F: Float
{
    [
        omega,
        omega
    ]
}
pub(crate) fn first_order_high_pass_filter_b<F>(two_rate: F) -> [F; 2]
where
    F: Float
{
    [
        two_rate,
        -two_rate
    ]
}
pub(crate) fn first_order_filter_a<F>(omega: F, two_rate: F) -> [F; 2]
where
    F: Float
{
    [
        omega + two_rate,
        omega - two_rate,
    ]
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::FirstOrderFilter;

    use crate::{conf::All, params::Omega};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderFilter::new::<All>(Omega::new(10000.0*TAU));
        //let mut filter = FirstOrderFilter::new::<All>(RC::new(100.0e3, 47.0e-9));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}