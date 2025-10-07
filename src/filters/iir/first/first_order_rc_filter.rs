use crate::{calc::iir::first::FirstOrderRCCalc, conf::{All, HighPass, LowPass}, param::{FirstOrderRCFilterConf, FirstOrderRCFilterParam, RC}};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`LowPass`](crate::conf::LowPass), [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
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
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// R = 10 kΩ
        /// 
        /// C = 33 nF
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="First order low-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_r_c_filter0.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="First order high-pass RC-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_r_c_filter1.png" height="500">
        /// </div>
    }
    FirstOrderRCFilter
    {
        type Conf: FirstOrderRCFilterConf;
        type Param: FirstOrderRCFilterParam = RC;

        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
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
);

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderRCFilter, RC};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::<All>::new(RC {r: 10e3, c: 33e-9});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}