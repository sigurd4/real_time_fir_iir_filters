use crate::{param::{FilterFloat, PIFilterParam, PI}, real_time_fir_iir_filters};

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

pub(crate) struct PICalc<F>
where
    F: FilterFloat
{
    two_rate: F,
    two_rate_p: F,
    i: F
}

impl<F> PICalc<F>
where
    F: FilterFloat
{
    pub fn new(pi: PI<F>, rate: F) -> Self
    {
        let PI {p, i} = pi;
        let two_rate = rate + rate;
        let two_rate_p = two_rate*p;
        Self {
            two_rate,
            two_rate_p,
            i
        }
    }

    pub fn b(&self) -> [F; 2]
    {
        [
            self.i + self.two_rate_p,
            self.i - self.two_rate_p
        ]
    }

    pub fn a(&self) -> [F; 2]
    {
        [
            self.two_rate,
            -self.two_rate
        ]
    }
}

#[cfg(test)]
mod test
{
    use super::{PIFilter, PI};

    #[test]
    fn plot()
    {
        let mut filter = PIFilter::new(PI {p: 1.0, i: 0.001});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}