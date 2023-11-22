use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct FirstOrderFilter<F, Omega = F>
where
    F: Float,
    Omega: Param<F>
{
    pub omega: Omega,
    pub w: [F; 1]
}

impl<F, Omega> FirstOrderFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>
{
    pub fn new(omega: Omega) -> Self
    {
        Self {
            omega,
            w: [F::zero(); 1]
        }
    }
    
    pub fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }
}

iir1_impl!(
    <Omega> FirstOrderFilter<F, Omega>: 2: false =>
    FirstOrderFilter<f32>;
    FirstOrderFilter<f64>
    where
        Omega: Param<F>
);

impl<F, Omega> FilterStaticInternals<F> for FirstOrderFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 1]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

first_order_parameterization!(
    <Omega> FirstOrderFilter<F, Omega>
    where
        Omega: Param<F>
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::FirstOrderFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderFilter::new(10000.0*TAU);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}