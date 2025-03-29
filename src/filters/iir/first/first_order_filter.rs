use crate::{calc::iir::first::FirstOrderCalc, conf::{All, HighPass, LowPass}, param::{FirstOrderFilterConf, FirstOrderFilterParam, OmegaFirstOrder, Param}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```md
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
            let calc = FirstOrderCalc::new(param.omega(), rate);
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
            let calc = FirstOrderCalc::new(param.omega(), rate);
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
            let calc = FirstOrderCalc::new(param.omega(), rate);
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
        [(); <<<Param<P> as FirstOrderFilterParam<C>>::Conf as FirstOrderFilterConf>::Conf as FirstOrderFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::FirstOrderFilter;

    use crate::{conf::All, param::Omega};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderFilter::new::<All>(Omega {omega: 10000.0*TAU});
        //let mut filter = FirstOrderFilter::new(RC::new(100.0e3, 47.0e-9));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}