use array_math::ArrayOps;

use super::*;

#[derive(Clone, Copy)]
pub struct FirstOrderRCFilter<F, R = F, C = F>
where
    F: Float,
    R: Param<F>,
    C: Param<F>
{
    pub r: R,
    pub c: C,
    pub w: [F; 1]
}

impl<F, R, C> FirstOrderRCFilter<F, R, C>
where
    F: Float,
    R: Param<F>,
    C: Param<F>
{
    pub fn new(r: R, c: C) -> Self
    {
        Self {
            r,
            c,
            w: [F::zero(); 1]
        }
    }

    pub fn r(&self) -> F
    {
        *(&self.r).deref()
    }
    
    pub fn c(&self) -> F
    {
        *(&self.c).deref()
    }
    
    pub fn omega(&self) -> F
    {
        (self.r()*self.c()).recip()
    }
}

iir1_impl!(
    <R, C> FirstOrderRCFilter<F, R, C>: 2: false =>
    FirstOrderRCFilter<f32>;
    FirstOrderRCFilter<f64>
    where
        R: Param<F>,
        C: Param<F>
);

impl<F, R, C> FilterStaticInternals<F> for FirstOrderRCFilter<F, R, C>
where
    F: Float,
    R: Param<F>,
    C: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 1]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

first_order_parameterization!(
    <R, C> FirstOrderRCFilter<F, R, C>
    where
        R: Param<F>, C: Param<F>
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::FirstOrderRCFilter;

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::new(10000.0, 0.000000033);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}