use crate::{calc::iir::third::ThirdOrderCalc, conf::{All, HighPass, LowPass, Peak}, param::{Omega2Zeta, ThirdOrderFilterConf, ThirdOrderFilterParam}, real_time_fir_iir_filters};

// TODO: Do it in SOS
crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All), [Peak](crate::conf::Peak),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak)<1>, [Peak](crate::conf::Peak)<2>, [HighPass](crate::conf::HighPass)
        /// ```md
        /// 0) LOW-PASS:
        /// 
        ///                   ω₁ω₂^2
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 1) PEAK 1:
        /// 
        ///              (ω₁ω₂^2)^(2/3)s
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 2) PEAK 2:
        /// 
        ///              (ω₁ω₂^2)^(1/3)s^2
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// 
        /// 3) HIGH-PASS:
        /// 
        ///                    s^3
        /// H(s) = ----------------------------
        ///        (s + ω₁)(s^2 + 2ζω₂s + ω₂^2)
        /// ```
    }
    ThirdOrderFilter
    {
        type Conf: ThirdOrderFilterConf;
        type Param: ThirdOrderFilterParam = Omega2Zeta;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = ThirdOrderCalc::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, (), ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), F, ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), (), F>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), (), ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, F, ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, (), F>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, (), ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), F, F>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), F, ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), (), F>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, F, ()>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, F, (), F>::new(param.omega2_zeta(), rate);
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
            let calc = ThirdOrderCalc::<F, (), F, F>::new(param.omega2_zeta(), rate);
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
        [(); <C as ThirdOrderFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::Omega2Zeta};

    use super::ThirdOrderFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderFilter::new::<All>(Omega2Zeta {omega1: 1000.0*TAU, omega2: 10000.0*TAU, zeta: 0.1});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}