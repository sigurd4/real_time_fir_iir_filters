use crate::{calc::iir::second::SecondOrderChebyshev1Calc, conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, OmegaEpsilonCheb1SecondOrder, Param, SecondOrderChebyshev1FilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                   1
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(s/ω))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///                   1
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(ω/s))
        /// ```
    }
    SecondOrderChebyshev1Filter
    {
        type Conf: EllipticFilterConf;
        type Param: SecondOrderChebyshev1FilterParam = OmegaEpsilonCheb1SecondOrder;

        const O_BUFFERS: usize = <C as EllipticFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderChebyshev1Calc::new(param.omega_epsilon(), rate);
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
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = SecondOrderChebyshev1Calc::new(param.omega_epsilon(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a_low()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = SecondOrderChebyshev1Calc::new(param.omega_epsilon(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a_high()
                ])]
            )
        }
    }
    where
        [(); <<<Param<P> as SecondOrderChebyshev1FilterParam<C>>::Conf as EllipticFilterConf>::Conf as EllipticFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::OmegaEpsilon};

    use super::SecondOrderChebyshev1Filter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev1Filter::new::<All>(OmegaEpsilon {omega: 10000.0*TAU, epsilon: 1.0});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}