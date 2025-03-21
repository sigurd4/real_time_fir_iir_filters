use crate::{param::{FilterFloat, PIDFilterParam, PID}, real_time_fir_iir_filters};

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
            let calc = PIDCalc::new(param.pid(), rate);
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

pub(crate) struct PIDCalc<F>
where
    F: FilterFloat
{
    four_rate2_d_p_i: F,
    two_i_m_eight_rate2_d: F,
    two_rate_p: F,
    two_rate: F
}

impl<F> PIDCalc<F>
where
    F: FilterFloat
{
    pub fn new(pid: PID<F>, rate: F) -> Self
    {
        let PID {p, i, d} = pid;
        let two_rate = rate + rate;
        let two_rate_p = two_rate*p;
        let four_rate2_d = two_rate*two_rate*d;
        let four_rate2_d_p_i = four_rate2_d + i;
        let i_m_four_rate2_d = i - four_rate2_d;
        let two_i_m_eight_rate2_d = i_m_four_rate2_d + i_m_four_rate2_d;
        Self {
            four_rate2_d_p_i,
            two_i_m_eight_rate2_d,
            two_rate_p,
            two_rate
        }
    }

    pub fn b(&self) -> [F; 3]
    {
        [
            self.four_rate2_d_p_i + self.two_rate_p,
            self.two_i_m_eight_rate2_d,
            self.four_rate2_d_p_i - self.two_rate_p,
        ]
    }

    pub fn a(&self) -> [F; 3]
    {
        [
            self.two_rate,
            F::zero(),
            -self.two_rate
        ]
    }
}

#[cfg(test)]
mod test
{
    use super::{PIDFilter, PID};

    #[test]
    fn plot()
    {
        let mut filter = PIDFilter::new(PID {p: 1.0, i: 0.001, d: 0.00001});
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}