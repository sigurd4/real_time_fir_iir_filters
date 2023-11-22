use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderButterworthFilter<F, Omega = F>
where
    F: Float,
    Omega: Param<F>
{
    pub omega: Omega,
    pub w: [F; 2]
}

impl<F, Omega> SecondOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>
{
    pub fn new(omega: Omega) -> Self
    {
        Self {
            omega,
            w: [F::zero(); 2]
        }
    }
    
    pub fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }

    pub fn zeta(&self) -> F
    {
        f!(0.5).sqrt()
    }
}

iir2_impl!(
    <Omega> SecondOrderButterworthFilter<F, Omega>: 3: false =>
    SecondOrderButterworthFilter<f32>;
    SecondOrderButterworthFilter<f64>
    where
        Omega: Param<F>
);

second_order_parameterization!(
    <Omega> SecondOrderButterworthFilter<F, Omega>
    where
        Omega: Param<F>
);

impl<F, Omega> FilterStaticInternals<F> for SecondOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 2]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::SecondOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderButterworthFilter::new(10000.0*TAU);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}