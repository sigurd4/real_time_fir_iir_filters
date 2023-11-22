use array_math::ArrayOps;

use super::*;

pub struct FirstOrderAllPassFilter<F, Tau = F>
where
    F: Float,
    Tau: Param<F>
{
    pub tau: Tau,
    pub w: [F; 1]
}

impl<F, Tau> FirstOrderAllPassFilter<F, Tau>
where
    F: Float,
    Tau: Param<F>
{
    pub fn new(tau: Tau) -> Self
    {
        Self {
            tau,
            w: [F::zero(); 1]
        }
    }

    fn tau(&self) -> F
    {
        *(&self.tau).deref()
    }
}

iir1_impl!(
    <Tau> FirstOrderAllPassFilter<F, Tau>: 1: false =>
    FirstOrderAllPassFilter<f32>;
    FirstOrderAllPassFilter<f64>
    where
        Tau: Param<F>
);

impl<F, Tau> FilterStaticCoefficients<F> for FirstOrderAllPassFilter<F, Tau>
where
    F: Float,
    Tau: Param<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 2]; 1])
    {
        let tau = self.tau();
        ([], [[
            f!(2.0)*tau*rate - f!(1.0),
            f!(1.0) - f!(2.0)*tau*rate
        ]])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 2]; 1])>
    {
        let tau = self.tau();
        Some(([], [[
            f!(1.0) + f!(2.0)*tau*rate,
            f!(1.0) - f!(2.0)*tau*rate
        ]]))
    }
}

impl<F, Tau> FilterStaticInternals<F> for FirstOrderAllPassFilter<F, Tau>
where
    F: Float,
    Tau: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 1]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

#[cfg(test)]
mod test
{
    use super::FirstOrderAllPassFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderAllPassFilter::new(0.001);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}