use crate::{calc::iir::second::SecondOrderBesselCalc, conf::{All, HighPass, LowPass}, param::{EllipticFilterConf, OmegaSecondOrder, SecondOrderBesselFilterParam}};

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
        ///         θ₂(0)         3ω²
        /// H(s) = ------- = --------------
        ///        θ₂(s/ω)   s² + 3ωs + 3ω²
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///         θ₂(0)         3s²
        /// H(s) = ------- = --------------
        ///        θ₂(ω/s)   3s² + 3ωs + ω²
        /// </pre>
        /// 
        /// ## Where
        /// 
        /// <pre>
        /// θ₂(s) = s² + 3s + 3
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ω = 10 kHz 2π
        /// 
        /// <div>
        /// <img alt="Second order bessel filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/second_order_bessel_filter.png" height="500">
        /// </div>
    }
    SecondOrderBesselFilter
    {
        type Conf: EllipticFilterConf;
        type Param: SecondOrderBesselFilterParam = OmegaSecondOrder;

        type OutputBufs<U> = <C as EllipticFilterConf>::Outputs<U>;
        const SOS_BUFS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = SecondOrderBesselCalc::new(param.omega(), rate);
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
            let calc = SecondOrderBesselCalc::<_, _, ()>::new(param.omega(), rate);
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
            let calc = SecondOrderBesselCalc::<_, (), _>::new(param.omega(), rate);
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
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::Omega};

    use super::SecondOrderBesselFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderBesselFilter::<All>::new(Omega {omega: 10e3*TAU});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}