use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderRLCFilter<F, R = F, L = F, C = F>
where
    F: Float,
    R: Param<F>,
    L: Param<F>,
    C: Param<F>
{
    pub r: R,
    pub l: L,
    pub c: C,
    pub w: [F; 2]
}

impl<F, R, L, C> SecondOrderRLCFilter<F, R, L, C>
where
    F: Float,
    R: Param<F>,
    L: Param<F>,
    C: Param<F>
{
    pub fn new(r: R, l: L, c: C) -> Self
    {
        Self {
            r,
            l,
            c,
            w: [F::zero(); 2]
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
    pub fn c(&self) -> F
    {
        *(&self.c).deref()
    }
    
    pub fn omega(&self) -> F
    {
        (self.l()*self.c()).sqrt().recip()
    }

    pub fn zeta(&self) -> F
    {
        f!(0.5)*self.r()*(self.c()/self.l()).sqrt()
    }
}

iir2_impl!(
    <R, L, C> SecondOrderRLCFilter<F, R, L, C>: 3: false =>
    SecondOrderRLCFilter<f32>;
    SecondOrderRLCFilter<f64>
    where
        R: Param<F>,
        L: Param<F>,
        C: Param<F>
);

second_order_parameterization!(
    <R, L, C> SecondOrderRLCFilter<F, R, L, C>
    where
        R: Param<F>,
        L: Param<F>,
        C: Param<F>
);

impl<F, R, L, C> FilterStaticInternals<F> for SecondOrderRLCFilter<F, R, L, C>
where
    F: Float,
    R: Param<F>,
    L: Param<F>,
    C: Param<F>,
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

    use super::SecondOrderRLCFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRLCFilter::new(1000.0, 0.01, 0.000000033);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}