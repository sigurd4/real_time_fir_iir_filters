use crate::{calc::iir::second::SecondOrderCalc, conf::{All, HighPass, LowPass, Peak}, param::{ButterworthFilterConf, ButterworthFilterParam, FilterParam, OmegaSecondOrder, Param, SecondOrderButterworthFilterConf, SecondOrderButterworthFilterParam}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// [All](crate::conf::All),
        /// [LowPass](crate::conf::LowPass), [Peak](crate::conf::Peak), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        /// 
        ///                ω^2
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// 
        /// 1) PEAK:
        /// 
        ///                ωs
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// 
        /// 2) HIGH-PASS:
        /// 
        ///                s^2
        /// H(s) = ------------------
        ///        s^2 + √(2)ωs + ω^2 
        /// ```
    }
    SecondOrderButterworthFilter
    {
        type Conf: SecondOrderButterworthFilterConf as ButterworthFilterConf<2>;
        type Param<C>: SecondOrderButterworthFilterParam as ButterworthFilterParam = OmegaSecondOrder;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        where {
            [(); <Param<P> as FilterParam>::ORDER]:
        }
        {
            let calc = SecondOrderCalc::new_butterworth(param.omega(), rate);
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
        [(); <Param<P> as FilterParam>::ORDER]:,
        [(); <CC as ButterworthFilterConf<2>>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::{conf::All, param::Omega};

    use super::SecondOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderButterworthFilter::<_, _, All>::new(Omega {omega: 10000.0*TAU});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}