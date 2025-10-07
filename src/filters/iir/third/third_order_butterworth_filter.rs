use crate::{calc::iir::third::ThirdOrderButterworthCalc, conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, OmegaThirdOrder, ThirdOrderButterworthFilterConf, ThirdOrderButterworthFilterParam}};

// TODO: Do it in SOS
crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`Peak`](crate::conf::Peak),
        /// [`LowPass`](crate::conf::LowPass), <code>[Peak](crate::conf::Peak)<1></code>, <code>[Peak](crate::conf::Peak)<2></code>, [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        /// 
        ///                  ω³
        /// H(s) = ---------------------
        ///        (s + ω)(s² + ωs + ω²)
        /// 
        /// 1) PEAK 1:
        /// 
        ///                 ω²s
        /// H(s) = ---------------------
        ///        (s + ω)(s² + ωs + ω²)
        /// 
        /// 2) PEAK 2:
        /// 
        ///                 ωs²
        /// H(s) = ---------------------
        ///        (s + ω)(s² + ωs + ω²)
        /// 
        /// 3) HIGH-PASS:
        /// 
        ///                  s³
        /// H(s) = ---------------------
        ///        (s + ω)(s² + ωs + ω²)
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// ω = 10 kHz 2π
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Third order butterworth low-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_butterworth_filter0.png" height="500">
        /// </div>
        /// 
        /// ## Peak 1
        /// 
        /// <div>
        /// <img alt="Third order butterworth peak filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_butterworth_filter1.png" height="500">
        /// </div>
        /// 
        /// ## Peak 2
        /// 
        /// <div>
        /// <img alt="Third order butterworth peak filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_butterworth_filter2.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Third order butterworth high-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_butterworth_filter3.png" height="500">
        /// </div>
    }
    ThirdOrderButterworthFilter
    {
        type Conf: ThirdOrderButterworthFilterConf as ButterworthFilterConf<3>;
        type Param: ThirdOrderButterworthFilterParam = OmegaThirdOrder;

        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
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
        let mut filter = ThirdOrderButterworthFilter::<All>::new(Omega {omega: 10e3*TAU});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}