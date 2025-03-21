use num::One;

use crate::{conf::{All, HighPass, LowPass}, param::{FilterFloat, FirstOrderRCFilterConf, FirstOrderRCFilterParam, RC}, real_time_fir_iir_filters};

crate::def_rtf!(
    {
        /// # Configurations
        /// 
        /// [All](crate::conf::All), [LowPass](crate::conf::LowPass), [HighPass](crate::conf::HighPass)
        /// ```#md
        /// 0) LOW-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 
        ///           1
        /// H(s) = -------
        ///        RCs + 1
        /// 
        /// 1) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// 
        ///          RCs
        /// H(s) = -------
        ///        RCs + 1
        /// ```
    }
    FirstOrderRCFilter
    {
        type Conf: FirstOrderRCFilterConf;
        type Param: FirstOrderRCFilterParam = RC;

        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs<All>(param, rate) -> _
        {
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
            let calc = FirstOrderRCCalc::new(param.rc(), rate);
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
        [(); <CC as FirstOrderRCFilterConf>::OUTPUTS]:
);

pub(crate) struct FirstOrderRCCalc<F>
where
    F: FilterFloat
{
    one: F,
    two_rate_d_omega: F
}

impl<F> FirstOrderRCCalc<F>
where
    F: FilterFloat
{
    pub fn new(rc: RC<F>, rate: F) -> Self
    {
        let RC {r, c} = rc;
        let rate_d_omega = rate*r*c;
        let two_rate_d_omega = rate_d_omega + rate_d_omega;
        let one = One::one();
        Self {
            one,
            two_rate_d_omega
        }
    }

    pub fn b_low(&self) -> [F; 2]
    {
        [
            self.one,
            self.one
        ]
    }

    pub fn b_high(&self) -> [F; 2]
    {
        [
            self.two_rate_d_omega,
            -self.two_rate_d_omega
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.one + self.two_rate_d_omega,
            self.one - self.two_rate_d_omega
        ]
    }
}

#[cfg(test)]
mod test
{
    use crate::conf::All;

    use super::{FirstOrderRCFilter, RC};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::<_, _, All>::new(RC {r: 10000.0, c: 0.000000033});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}