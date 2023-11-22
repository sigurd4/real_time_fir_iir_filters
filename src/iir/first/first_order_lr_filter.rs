use array_math::ArrayOps;

use super::*;

#[derive(Clone, Copy)]
pub struct FirstOrderLRFilter<F, L = F, R = F>
where
    F: Float,
    L: Param<F>,
    R: Param<F>
{
    pub l: L,
    pub r: R,
    pub w: [F; 1]
}

impl<F, L, R> FirstOrderLRFilter<F, L, R>
where
    F: Float,
    L: Param<F>,
    R: Param<F>
{
    pub fn new(l: L, r: R) -> Self
    {
        Self {
            l,
            r,
            w: [F::zero(); 1]
        }
    }

    pub fn r(&self) -> F
    {
        *(&self.r).deref()
    }
    pub fn l(&self) -> F
    {
        *(&self.l).deref()
    }

    pub fn omega(&self) -> F
    {
        self.r()/self.l()
    }
}

iir1_impl!(
    <L, R> FirstOrderLRFilter<F, L, R>: 2: false =>
    FirstOrderLRFilter<f32>;
    FirstOrderLRFilter<f64>
    where
        L: Param<F>,
        R: Param<F>
);

impl<F, L, R> FilterStaticInternals<F> for FirstOrderLRFilter<F, L, R>
where
    F: Float,
    L: Param<F>,
    R: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 1]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

first_order_parameterization!(
    <L, R> FirstOrderLRFilter<F, L, R>
    where
        L: Param<F>,
        R: Param<F>
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::FirstOrderLRFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderLRFilter::new(0.1, 10000.0);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}