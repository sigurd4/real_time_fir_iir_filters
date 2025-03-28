use crate::{calc::iir::third::ThirdOrderButterworthCalc, conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, OmegaThirdOrder, ThirdOrderButterworthFilterConf, ThirdOrderButterworthFilterParam}, real_time_fir_iir_filters};

// TODO: Do it in SOS
crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [Peak](crate::conf::Peak),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak)<1>, [Peak](crate::conf::Peak)<2>, [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                  ω^3
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 1) PEAK 1:
        /// 
        ///                  ω^2s
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 2) PEAK 2:
        /// 
        ///                  ωs^2
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// 
        /// 3) HIGH-PASS:
        /// 
        ///                  s^3
        /// H(s) = -----------------------
        ///        (s + ω)(s^2 + ωs + ω^2)
        /// ```
    }
    ThirdOrderButterworthFilter
    {
        type Conf: ThirdOrderButterworthFilterConf as ButterworthFilterConf<3>;
        type Param: ThirdOrderButterworthFilterParam = OmegaThirdOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak1(),
                    calc.b_peak2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<Peak<1>>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak1()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<Peak<2>>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak1()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
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
        fn make_coeffs<Peak>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak1(),
                    calc.b_peak2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(Peak<1>, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(Peak<2>, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak1(),
                    calc.b_peak2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<1>, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak<2>, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        {
            let calc = ThirdOrderButterworthCalc::new(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak1(),
                    calc.b_peak2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <C as ButterworthFilterConf<3>>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::Omega};

    use super::ThirdOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderButterworthFilter::new::<All>(Omega {omega: 10000.0*TAU});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}