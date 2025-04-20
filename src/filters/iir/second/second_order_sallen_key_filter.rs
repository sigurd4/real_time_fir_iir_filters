use crate::{calc::iir::second::SecondOrderSallenKeyCalc, conf::{All, BandPass, HighPass, LowPass}, param::{RC2GSallenKey, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`BandPass`](crate::conf::BandPass),
        /// [`LowPass`](crate::conf::LowPass), <code>[BandPass](crate::conf::BandPass)<1></code>, <code>[BandPass](crate::conf::BandPass)<2></code>, [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        ///            o------------o
        ///            |            |
        ///           [C₁]          |
        ///            |            |
        ///     X-[R₁]-o-[R₂]-o-[G>-Y
        ///                   |
        ///                  [C₂]
        ///                   |
        ///                  GND
        /// 1) BAND-PASS 1:
        ///            o------------o
        ///            |            |
        ///           [R₁]          |
        ///            |            |
        ///     X-[C₁]-o-[R₂]-o-[G>-Y
        ///                   |
        ///                  [C₂]
        ///                   |
        ///                  GND
        /// 2) BAND-PASS 2:
        ///            o------------o
        ///            |            |
        ///           [C₁]          |
        ///            |            |
        ///     X-[R₁]-o-[C₂]-o-[G>-Y
        ///                   |
        ///                  [R₂]
        ///                   |
        ///                  GND
        /// 3) HIGH-PASS:
        ///            o------------o
        ///            |            |
        ///           [R₁]          |
        ///            |            |
        ///     X-[C₁]-o-[C₂]-o-[G>-Y
        ///                   |
        ///                  [R₂]
        ///                   |
        ///                  GND
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// R₁ = 15 kΩ
        /// 
        /// C₁ = 2.7 nF
        /// 
        /// R₂ = 15 kΩ
        /// 
        /// C₂ = 2.7 nF
        /// 
        /// G = 2
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Second order low-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_sallen_key_filter0.png" height="500">
        /// </div>
        /// 
        /// ## Band-pass 1
        /// 
        /// <div>
        /// <img alt="Second order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_sallen_key_filter1.png" height="500">
        /// </div>
        /// 
        /// ## Band-pass 2
        /// 
        /// <div>
        /// <img alt="Second order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_sallen_key_filter2.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order high-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_sallen_key_filter3.png" height="500">
        /// </div>
    }
    SecondOrderSallenKeyFilter
    {
        type Conf: SecondOrderSallenKeyFilterConf;
        type Param: SecondOrderSallenKeyFilterParam = RC2GSallenKey;

        const O_BUFFERS: usize = <C as SecondOrderSallenKeyFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band1(),
                    calc.a_band2(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, F, (), ()>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a_low()
                ])]
            )
        }
        fn make_coeffs<BandPass<1>>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, F, F, ()>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band1()
                ]),
                [([], [
                    calc.a_band1()
                ])]
            )
        }
        fn make_coeffs<BandPass<2>>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, (), F, F>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band2()
                ]),
                [([], [
                    calc.a_band2()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, (), (), F>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, F, F, ()>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band1()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band2()
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, F, (), F>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a_band1(),
                    calc.a_band2()
                ])]
            )
        }
        fn make_coeffs<(BandPass<1>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_band1(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<(BandPass<2>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::<F, (), F, F>::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_band2(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_band2()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band1(),
                    calc.a_band2()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<1>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band1(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band1(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass<2>, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_low(),
                    calc.a_band2(),
                    calc.a_high()
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderSallenKeyCalc::new(param.rc2g(), rate);
            (
                ([], [], [
                    calc.b_band1(),
                    calc.b_band2(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a_band1(),
                    calc.a_band2(),
                    calc.a_high()
                ])]
            )
        }
    }
    where
        [(); <C as SecondOrderSallenKeyFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderSallenKeyFilter, RC2GSallenKey};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderSallenKeyFilter::<All>::new(RC2GSallenKey {r1: 15.0e3, c1: 2.7e-9, r2: 15.0e3, c2: 2.7e-9, g: 2.0});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}