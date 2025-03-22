use num::Float;

use crate::{calc::iir::second::SecondOrderChebyshev1Calc, conf::{All, HighPass, LowPass}, param::{ChebyshevFilterConf, ChebyshevFilterParam, OmegaEpsilonCheb1SecondOrder, SecondOrderChebyshev1FilterParam}, real_time_fir_iir_filters};

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
        type Conf: ChebyshevFilterConf as ChebyshevFilterConf;
        type Param<C>: SecondOrderChebyshev1FilterParam as ChebyshevFilterParam = OmegaEpsilonCheb1SecondOrder;

        const O_BUFFERS: usize = <CC as ChebyshevFilterConf>::OUTPUTS;
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
        [(); <CC as ChebyshevFilterConf>::OUTPUTS]:
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
        let mut filter = SecondOrderChebyshev1Filter::<_, _, All>::new(OmegaEpsilon {omega: 10000.0*TAU, epsilon: 1.0});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}