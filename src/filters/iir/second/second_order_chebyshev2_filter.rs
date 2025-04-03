use crate::{calc::iir::second::SecondOrderChebyshev2Calc, conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, OmegaEpsilonCheb2SecondOrder, SecondOrderChebyshev2FilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All),
        /// [`LowPass`](crate::conf::LowPass), [`HighPass`](crate::conf::HighPass)
        /// 
        /// <pre>
        /// 0) LOW-PASS:
        /// 
        ///            √(ε²T₂²(ω/s))
        /// |H(s)| = -----------------
        ///          √(1 + ε²T₂²(ω/s))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///            √(ε²T₂²(s/ω))
        /// |H(s)| = -----------------
        ///          √(1 + ε²T₂²(s/ω))
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// ω = 10 kHz 2π
        /// 
        /// ε = 0.5
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Second order chebyshev2 low-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_chebyshev2_filter0.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order chebyshev2 high-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_chebyshev2_filter1.png" height="500">
        /// </div>
    }
    SecondOrderChebyshev2Filter
    {
        type Conf: EllipticFilterConf;
        type Param: SecondOrderChebyshev2FilterParam = OmegaEpsilonCheb2SecondOrder;

        const O_BUFFERS: usize = <C as EllipticFilterConf>::OUTPUTS;
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
        [(); <C as EllipticFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::OmegaEpsilon};

    use super::SecondOrderChebyshev2Filter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev2Filter::new::<All>(OmegaEpsilon {omega: 10e3*TAU, epsilon: 0.5});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}