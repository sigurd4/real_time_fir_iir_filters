use crate::{calc::iir::first::FirstOrderAllPassCalc, conf::All, param::{FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, Tau}, real_time_fir_iir_filters};

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