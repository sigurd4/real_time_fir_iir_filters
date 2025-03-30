use crate::{calc::iir::second::SecondOrderRLCCalc, conf::{All, BandPass, BandStop, HighPass, LowPass}, param::{SecondOrderRLCFilterConf, SecondOrderRLCFilterParam, RLC}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All),
        /// [`LowPass`](crate::conf::LowPass), [`BandStop`](crate::conf::BandStop), [`BandPass`](crate::conf::BandPass), [`HighPass`](crate::conf::HighPass)
        /// 
        /// ```md
        /// 0) LOW-PASS:
        ///     X-[R]-[L]-Y
        ///               |
        ///              [C]
        ///               |
        ///              GND
        /// 
        /// 1) BAND-STOP:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 
        /// 2) BAND-PASS:
        ///     X-[C]-[L]-Y
        ///               |
        ///              [R]
        ///               |
        ///              GND
        /// 
        /// 3) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// ```
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// R = 1 kΩ
        /// 
        /// L = 10 mH
        /// 
        /// C = 33 nF
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Second order low-pass RLC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_l_c_filter0.png" height="500">
        /// </div>
        /// 
        /// ## Band-stop
        /// 
        /// <div>
        /// <img alt="Second order band-stop RLC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_l_c_filter1.png" height="500">
        /// </div>
        /// 
        /// ## Band-pass
        /// 
        /// <div>
        /// <img alt="Second order band-pass RLC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_l_c_filter2.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order high-pass RLC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_r_l_c_filter3.png" height="500">
        /// </div>
    }
    SecondOrderRLCFilter
    {
        type Conf: SecondOrderRLCFilterConf;
        type Param: SecondOrderRLCFilterParam = RLC;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_stop(),
                    calc.b_band_pass(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<BandStop>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_stop()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<BandPass>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_pass()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_stop()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_pass()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
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
        fn make_coeffs<(BandStop, BandPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_stop(),
                    calc.b_band_pass()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandStop, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_stop(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_pass(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop, BandPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_stop(),
                    calc.b_band_pass()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandStop, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_stop(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, BandPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_band_pass(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(BandStop, BandPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderRLCCalc::new(param.rlc(), rate);
            (
                ([], [], [
                    calc.b_band_stop(),
                    calc.b_band_pass(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <C as SecondOrderRLCFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{SecondOrderRLCFilter, RLC};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRLCFilter::new::<All>(RLC {r: 1e3, l: 10e-3, c: 33e-9});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}