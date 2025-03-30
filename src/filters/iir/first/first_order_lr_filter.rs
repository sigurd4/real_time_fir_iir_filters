use crate::{calc::iir::first::FirstOrderLRCalc, conf::{All, HighPass, LowPass}, param::{FirstOrderLRFilterConf, FirstOrderLRFilterParam, Param, LR}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`LowPass`](crate::conf::LowPass), [`HighPass`](crate::conf::HighPass)
        /// ```md
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
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// L = 100 mH
        /// 
        /// R = 10 kΩ
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="First order low-pass LR-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_l_r_filter0.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="First order high-pass LR-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_l_r_filter1.png" height="500">
        /// </div>
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
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <<<Param<P> as FirstOrderLRFilterParam<C>>::Conf as FirstOrderLRFilterConf>::Conf as FirstOrderLRFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderLRFilter, LR};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderLRFilter::new::<All>(LR {l: 100e-3, r: 10e3});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}