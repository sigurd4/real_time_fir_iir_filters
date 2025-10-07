use crate::{calc::iir::second::SecondOrderCalc, conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, OmegaSecondOrder, SecondOrderButterworthFilterConf, SecondOrderButterworthFilterParam}};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All),
        /// [`LowPass`](crate::conf::LowPass), [`Peak`](crate::conf::Peak), [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        /// 
        ///               ω²
        /// H(s) = ----------------
        ///        s² + √(2)ωs + ω² 
        /// 
        /// 1) PEAK:
        /// 
        ///               ωs
        /// H(s) = ----------------
        ///        s² + √(2)ωs + ω² 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///               s²
        /// H(s) = ----------------
        ///        s² + √(2)ωs + ω²
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
        /// <img alt="Second order butterworth low-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_butterworth_filter0.png" height="500">
        /// </div>
        /// 
        /// ## Peak
        /// 
        /// <div>
        /// <img alt="Second order butterworth peak filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_butterworth_filter1.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order butterworth high-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_butterworth_filter2.png" height="500">
        /// </div>
    }
    SecondOrderButterworthFilter
    {
        type Conf: SecondOrderButterworthFilterConf as ButterworthFilterConf<2>;
        type Param: SecondOrderButterworthFilterParam = OmegaSecondOrder;

        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<Peak>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, Peak)>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_low(),
                    calc.b_peak()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<(LowPass, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        fn make_coeffs<(Peak, HighPass)>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
            (
                ([], [], [
                    calc.b_peak(),
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::Omega};

    use super::SecondOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderButterworthFilter::<All>::new(Omega {omega: 10e3*TAU});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}