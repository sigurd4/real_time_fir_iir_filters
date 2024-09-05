use crate::{param::PIDFilterParam, params::PID, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configuration
        /// ```#md
        ///            I
        /// H(s) = P + - + Ds
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
            let p = param.p();
            let i = param.i();
            let d = param.d();

            let two_rate = rate + rate;
            let two_rate_p = two_rate*p;
            let four_rate2_d = two_rate*two_rate*d;
            let four_rate2_d_p_i = four_rate2_d + i;
            let i_m_four_rate2_d = i - four_rate2_d;
            let two_i_m_eight_rate2_d = i_m_four_rate2_d + i_m_four_rate2_d;
            (
                ([], [], [[
                    four_rate2_d_p_i + two_rate_p,
                    two_i_m_eight_rate2_d,
                    four_rate2_d_p_i - two_rate_p,
                ]]),
                [([], [[
                    two_rate,
                    F::zero(),
                    -two_rate
                ]])]
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
        let mut filter = PIDFilter::new(PID::new(1.0, 0.001, 0.00001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}