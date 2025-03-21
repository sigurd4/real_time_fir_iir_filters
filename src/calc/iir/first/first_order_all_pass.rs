use crate::param::{FilterFloat, Tau};

pub struct FirstOrderAllPassCalc<F>
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