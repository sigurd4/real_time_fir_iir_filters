use crate::{calc::iir::second::SecondOrderCalc, conf::{All, HighPass, LowPass, Peak}, param::{OmegaZeta, Param, SecondOrderFilterConf, SecondOrderFilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///              ω^2
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// 
        /// 1) PEAK:
        /// 
        ///               ωs
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///              s^2
        /// H(s) = ----------------
        ///        s^2 + 2ζωs + ω^2 
        /// ```
    }
    SecondOrderFilter
    {
        type Conf: SecondOrderFilterConf;
        type Param: SecondOrderFilterParam = OmegaZeta;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
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
    where
        [(); <<<Param<P> as SecondOrderFilterParam<C>>::Conf as SecondOrderFilterConf>::Conf as SecondOrderFilterConf>::OUTPUTS]:
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
        let mut filter = SecondOrderFilter::new::<All>(OmegaZeta {omega: 10000.0*TAU, zeta: 0.05});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}