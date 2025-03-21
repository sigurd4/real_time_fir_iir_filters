use crate::{calc::iir::second::PIDCalc, param::{PIDFilterParam, PID}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configuration
        /// ```#md
        ///            I
        /// H(s) = P + - + Dspub
        ///            s
        /// ```
    }
    PIDFilter
    {
        type Param: PIDFilterParam = PID;

        const OUTPUTS: usize = 1;
        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let calc = PIDCalc::new(param.pid(), rate);
            (
                ([], [], [
                    calc.b()
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
    use super::{PIDFilter, PID};

    #[test]
    fn plot()
    {
        let mut filter = PIDFilter::new(PID {p: 1.0, i: 0.001, d: 0.00001});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}