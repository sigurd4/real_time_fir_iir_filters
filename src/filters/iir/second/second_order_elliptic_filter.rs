use crate::{calc::iir::second::SecondOrderEllipticCalc, conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, OmegaEpsilonXiSecondOrder, SecondOrderEllipticFilterParam}, real_time_fir_iir_filters};

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
        ///                   1
        /// |H(s)| = --------------------
        ///          √(1 + ε²R₂²(ξ, s/ω))
        /// 
        /// 1) HIGH-PASS:
        /// 
        ///                   1
        /// |H(s)| = --------------------
        ///          √(1 + ε²R₂²(ξ, ω/s))
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
        /// ξ = 1.5
        /// 
        /// ## Low-pass
        /// 
        /// <div>
        /// <img alt="Second order elliptic low-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_elliptic_filter0.png" height="500">
        /// </div>
        /// 
        /// ## High-pass
        /// 
        /// <div>
        /// <img alt="Second order elliptic high-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_elliptic_filter1.png" height="500">
        /// </div>
    }
    SecondOrderEllipticFilter
    {
        type Conf: EllipticFilterConf;
        type Param: SecondOrderEllipticFilterParam = OmegaEpsilonXiSecondOrder;

        const O_BUFFERS: usize = <C as EllipticFilterConf>::OUTPUTS;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderEllipticCalc::new(param.omega_epsilon_xi(), rate);
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
            let calc = SecondOrderEllipticCalc::new(param.omega_epsilon_xi(), rate);
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
            let calc = SecondOrderEllipticCalc::new(param.omega_epsilon_xi(), rate);
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

    use crate::{conf::All, param::OmegaEpsilonXi};

    use super::SecondOrderEllipticFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderEllipticFilter::new::<All>(OmegaEpsilonXi {omega: 10e3*TAU, epsilon: 0.5, xi: 1.5});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}