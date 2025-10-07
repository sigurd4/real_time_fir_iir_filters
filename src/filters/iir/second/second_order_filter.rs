use crate::{calc::iir::second::SecondOrderCalc, conf::{All, HighPass, LowPass, Peak}, param::{OmegaZeta, SecondOrderFilterConf, SecondOrderFilterParam}};

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
        ///              ω²
        /// H(s) = --------------
        ///        s² + 2ζωs + ω² 
        /// 
        /// 1) PEAK:
        /// 
        ///              ωs
        /// H(s) = --------------
        ///        s² + 2ζωs + ω² 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///              s²
        /// H(s) = --------------
        ///        s² + 2ζωs + ω²
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ω = 10 kHz 2π
        /// 
        /// ζ = 0.2
        /// 
        /// <div>
        /// <img alt="Second order filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_filter.png" height="500">
        /// </div>
    }
    SecondOrderFilter
    {
        type Conf: SecondOrderFilterConf;
        type Param: SecondOrderFilterParam = OmegaZeta;

        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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
            let calc = SecondOrderCalc::new(param.omega_zeta(), rate);
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

    use crate::conf::All;

    use super::{OmegaZeta, SecondOrderFilter};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderFilter::<All>::new(OmegaZeta {omega: 10e3*TAU, zeta: 0.2});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}