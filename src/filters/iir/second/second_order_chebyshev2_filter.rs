use crate::{calc::iir::second::SecondOrderChebyshev2Calc, conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, OmegaEpsilonCheb2SecondOrder, Param, SecondOrderChebyshev2FilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///            √(ε^2T_2^2(ω/s))
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(ω/s))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///            √(ε^2T_2^2(s/ω))
        /// |H(s)| = --------------------
        ///          √(1 + ε^2T_2^2(s/ω))
        /// ```
    }
    SecondOrderChebyshev2Filter
    {
        type Conf: EllipticFilterConf;
        type Param: SecondOrderChebyshev2FilterParam = OmegaEpsilonCheb2SecondOrder;

        const O_BUFFERS: usize = <CC as EllipticFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderChebyshev2Calc::new(param.omega_epsilon(), rate);
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
            let calc = SecondOrderChebyshev2Calc::<_, _, ()>::new(param.omega_epsilon(), rate);
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
            let calc = SecondOrderChebyshev2Calc::<_, (), _>::new(param.omega_epsilon(), rate);
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
        [(); <<<Param<P> as SecondOrderChebyshev2FilterParam<C>>::Conf as EllipticFilterConf>::Conf as EllipticFilterConf>::OUTPUTS]:
);


mod private
{
    use crate::param::EllipticFilterConf;
    
    pub trait EllipticFilterConfFinal<C>: EllipticFilterConf<
        Conf = Self
    >
    where
        C: EllipticFilterConf<
            Conf = Self
        >
    {

    }

    impl<
        C,
        CC
    > EllipticFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC
        >,
        C: EllipticFilterConf<
            Conf = CC
        >
    {

    }
}
#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::param::OmegaEpsilon;

    use super::SecondOrderChebyshev2Filter;

    #[test]
    fn plot()
    {
        /*let par = OmegaEpsilon {omega: 10000.0*TAU, epsilon: 1.0};
        let mut filter = SecondOrderChebyshev2Filter::new(par);
        crate::tests::plot_freq(&mut filter, false).unwrap();*/
    }
}