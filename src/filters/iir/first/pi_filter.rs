use crate::{param::{PIFilterParam, PIVal}, params::PI, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configuration
        /// ```#md
        ///            I
        /// H(s) = P + -
        ///            s
        /// ```
    }
    PIFilter
    {
        type Param: PIFilterParam = PI;

        const OUTPUTS: usize = 1;
        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let PIVal {p, i} = param.pi();

            let two_rate = rate + rate;
            let two_rate_p = two_rate*p;
            (
                ([], [], [[
                    two_rate_p + i,
                    -two_rate_p + i
                ]]),
                [([], [[
                    two_rate,
                    -two_rate
                ]])]
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
        let mut filter = PIFilter::new(PI::new(1.0, 0.001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}