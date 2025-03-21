use crate::{conf::All, param::{FilterFloat, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, Tau}, real_time_fir_iir_filters};

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
            let calc = FirstOrderAllPassCalc::new(param.tau(), rate);
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
    where
        [(); <CC as FirstOrderAllPassFilterConf>::OUTPUTS]:
);

pub(crate) struct FirstOrderAllPassCalc<F>
where
    F: FilterFloat
{
    two_tau_rate_p_one: F,
    two_tau_rate_m_one: F
}

impl<F> FirstOrderAllPassCalc<F>
where
    F: FilterFloat
{
    pub fn new(tau: Tau<F>, rate: F) -> Self
    {
        let Tau {tau} = tau;
        let tau_rate = tau*rate;
        let two_tau_rate = tau_rate + tau_rate;
        let one = F::one();
        let two_tau_rate_m_one = two_tau_rate - one;
        let two_tau_rate_p_one = two_tau_rate + one;
        Self {
            two_tau_rate_p_one,
            two_tau_rate_m_one
        }
    }

    pub fn b(&self) -> [F; 2]
    {
        [
            self.two_tau_rate_m_one,
            -self.two_tau_rate_p_one
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.two_tau_rate_p_one,
            -self.two_tau_rate_m_one
        ]
    }
}

#[cfg(test)]
mod test
{
    use crate::{conf::All, param::Tau};

    use super::FirstOrderAllPassFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderAllPassFilter::<_, _, All>::new(Tau {tau: 0.001});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}