use crate::{calc::iir::second::SecondOrderRCCalc, conf::{All, BandPass, HighPass, LowPass}, param::{SecondOrderRCFilterConf, SecondOrderRCFilterParam, RC2}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`BandPass`](crate::conf::BandPass),
        /// [`LowPass`](crate::conf::LowPass), <code>[BandPass](crate::conf::BandPass)<1></code>, <code>[BandPass](crate::conf::BandPass)<2></code>, [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        ///     X-[R₁]-o-[R₂]-Y
        ///            |      |
        ///           [C₁]   [C₂]
        ///            |      |
        ///           GND    GND
        /// 
        /// 1) BAND-PASS 1:
        ///     X-[C₁]-o-[R₂]-Y
        ///            |      |
        ///           [R₁]   [C₂]
        ///            |      |
        ///           GND    GND
        /// 
        /// 2) BAND-PASS 2
        ///     X-[R₁]-o-[C₂]-Y
        ///            |      |
        ///           [C₁]   [R₂]
        ///            |      |
        ///           GND    GND
        /// 
        /// 3) HIGH-PASS
        ///     X-[C₁]-o-[C₂]-Y
        ///            |      |
        ///           [R₁]   [R₂]
        ///            |      |
        ///           GND    GND
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// R₁ = 22 kΩ
        /// 
        /// C₁ = 4.7 nF
        /// 
        /// R₂ = 5.6 kΩ
        /// 
        /// C₂ = 4.7 nF
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Second order low-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_c_filter0.png" height="500">
        /// </div>
        /// 
        /// ## Band-pass 1
        /// 
        /// <div>
        /// <img alt="Second order band-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_c_filter1.png" height="500">
        /// </div>
        /// 
        /// ## Band-pass 2
        /// 
        /// <div>
        /// <img alt="Second order band-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_c_filter2.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order high-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_c_filter3.png" height="500">
        /// </div>
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
        let mut filter = SecondOrderRCFilter::<All>::new(RC2 {r1: 22e3, c1: 4.7e-9, r2: 5.6e3, c2: 4.7e-9});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}