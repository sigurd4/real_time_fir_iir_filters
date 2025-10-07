use crate::{calc::iir::first::FirstOrderAllPassCalc, conf::All, param::{FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, Tau}};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [`All`](crate::conf::All), [`AllPass`](crate::conf::AllPass)
        /// 
        /// ## All-pass
        /// 
        /// <pre>
        ///        τs - 1
        /// H(s) = ------
        ///        τs + 1
        /// </pre>
        /// 
        /// # Frequency response
        /// 
        /// ## Parameters
        /// 
        /// τ = 1 ms
        /// 
        /// ## All-pass
        /// 
        /// <div>
        /// <img alt="First order all-pass filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/first_order_all_pass_filter0.png" height="500">
        /// </div>
    }
    FirstOrderAllPassFilter
    {
        type Conf: FirstOrderAllPassFilterConf;
        type Param: FirstOrderAllPassFilterParam = Tau;

        const OUTPUT_BUFS: usize = 1;
        const SOS_BUFS: usize = 1;
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
);

#[cfg(test)]
mod test
{
    use crate::{conf::All, param::Tau};

    use super::FirstOrderAllPassFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderAllPassFilter::<All>::new(Tau {tau: 0.001});
        crate::tests::plot_freq(&mut filter).unwrap();
    }
}