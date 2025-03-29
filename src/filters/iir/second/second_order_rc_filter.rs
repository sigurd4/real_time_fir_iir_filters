use crate::{calc::iir::second::SecondOrderRCCalc, conf::{All, BandPass, HighPass, LowPass}, param::{SecondOrderRCFilterConf, SecondOrderRCFilterParam, RC2}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [BandPass](crate::conf::BandPass),
        /// [LowPass](crate::conf::LowPass), [BandPass](crate::conf::BandPass)<1>, [BandPass](crate::conf::BandPass)<2>, [HighPass](crate::conf::HighPass)
        /// ```md
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
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::<F, ()>::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<BandPass<1>>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band1()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<BandPass<2>>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::<F, ()>::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::<F, ()>::new(param.rc2(), rate);
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
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandPass<1>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandPass<2>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRCCalc::new(param.rc2(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <C as SecondOrderRCFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderRCFilter, RC2};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRCFilter::new::<All>(RC2 {r1: 390e3, c1: 100e-9, r2: 4.7e3, c2: 47e-12});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}