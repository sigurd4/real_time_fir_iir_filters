use crate::{conf::All, param::{FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, TauVal}, params::Tau, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [AllPass](crate::conf::AllPass)
        /// ```#md
        /// 0) ALL-PASS:
        /// 
        ///        τs - 1
        /// H(s) = ------
        ///        τs + 1
        /// 
        /// ```
    }
    FirstOrderAllPassFilter
    {
        type Conf: FirstOrderAllPassFilterConf;
        type Param: FirstOrderAllPassFilterParam = Tau;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let TauVal {tau} = param.tau();
            let tau_rate = tau*rate;
            let two_tau_rate = tau_rate + tau_rate;
            let one = F::one();
            let two_tau_rate_m_one = two_tau_rate - one;
            let two_tau_rate_p_one = two_tau_rate + one;
            (
                ([], [], [[
                    two_tau_rate_m_one,
                    -two_tau_rate_p_one,
                ]]),
                [([], [[
                    two_tau_rate_p_one,
                    -two_tau_rate_m_one,
                ]])]
            )
        }
    }
    where
        [(); <CC as FirstOrderAllPassFilterConf>::OUTPUTS]:
);

#[cfg(test)]
mod test
{
    use crate::{conf::All, params::Tau};

    use super::FirstOrderAllPassFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderAllPassFilter::<_, _, All>::new(Tau::new(0.001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}