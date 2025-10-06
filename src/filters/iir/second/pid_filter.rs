use crate::{calc::iir::second::PIDCalc, param::{PIDFilterParam, PID}};

crate::def_rtf!(
    {
        /// # Configuration
        /// 
        /// <pre>
        ///            I
        /// H(s) = P + - + Ds
        ///            s
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// P = 1
        /// 
        /// I = 1 mHz
        /// 
        /// D = 10 μs
        /// 
        /// ## Output
        /// 
        /// <div>
        /// <img alt="PID-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/p_i_d_filter0.png" height="500">
        /// </div>
    }
    PIDFilter
    {
        type Param: PIDFilterParam = PID;

        const OUTPUTS: usize = 1;
        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
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