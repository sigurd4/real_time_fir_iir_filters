use crate::{calc::iir::first::FirstOrderRCCalc, conf::{All, HighPass, LowPass}, param::{FirstOrderRCFilterConf, FirstOrderRCFilterParam, Param, RC}, real_time_fir_iir_filters};

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
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
        [(); <<<Param<P> as FirstOrderRCFilterParam<C>>::Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderRCFilter, RC};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::new::<All>(RC {r: 10000.0, c: 0.000000033});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}