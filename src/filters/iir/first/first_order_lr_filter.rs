use crate::{conf::{All, HighPass, LowPass}, param::{FilterFloat, FirstOrderLRFilterConf, FirstOrderLRFilterParam, LR}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[L]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// 
        ///           R
        /// H(s) = ------
        ///        Ls + R
        /// 
        /// 1) HIGH-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          GND
        /// 
        ///          Ls
        /// H(s) = ------
        ///        Ls + R
        /// ```
    }
    FirstOrderLRFilter
    {
        type Conf: FirstOrderLRFilterConf;
        type Param: FirstOrderLRFilterParam = LR;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
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
        fn make_coeffs<LowPass>(param, rate) -> _
        {
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
            (
                ([], [], [
                    calc.b_low()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
        fn make_coeffs<HighPass>(param, rate) -> _
        {
            let calc = FirstOrderLRCalc::new(param.lr(), rate);
            (
                ([], [], [
                    calc.b_high()
                ]),
                [([], [
                    calc.a()
                ])]
            )
        }
    }
    where
        [(); <CC as FirstOrderLRFilterConf>::OUTPUTS]:
);

pub(crate) struct FirstOrderLRCalc<F>
where
    F: FilterFloat
{
    r: F,
    two_rate_l: F
}

impl<F> FirstOrderLRCalc<F>
where
    F: FilterFloat
{
    pub fn new(lr: LR<F>, rate: F) -> Self
    {
        let LR {l, r} = lr;
        let two_rate = rate + rate;
        let two_rate_l = two_rate*l;
        Self {
            r,
            two_rate_l
        }
    }

    pub fn b_low(&self) -> [F; 2]
    {
        [
            self.r,
            self.r
        ]
    }

    pub fn b_high(&self) -> [F; 2]
    {
        [
            self.two_rate_l,
            -self.two_rate_l
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.r + self.two_rate_l,
            self.r - self.two_rate_l
        ]
    }
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderLRFilter, LR};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderLRFilter::<_, _, All>::new(LR {l: 100e-3, r: 10e3});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}