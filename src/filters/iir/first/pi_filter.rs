use crate::{calc::iir::first::PICalc, param::{PIFilterParam, PI}};

crate::def_rtf!(
    {
        /// # Configuration
        /// 
        /// <pre>
        ///            I
        /// H(s) = P + -
        ///            s
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// P = 1
        /// 
        /// I = 1 mHz
        /// 
        /// <div>
        /// <img alt="PI-filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/p_i_filter.png" height="500">
        /// </div>
    }
    PIFilter
    {
        type Param: PIFilterParam = PI;

        const OUTPUTS: usize = 1;
        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let calc = PICalc::new(param.pi(), rate);
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
    use super::{PIFilter, PI};

    #[test]
    fn plot()
    {
        let mut filter = PIFilter::new(PI {p: 1.0, i: 0.001});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}