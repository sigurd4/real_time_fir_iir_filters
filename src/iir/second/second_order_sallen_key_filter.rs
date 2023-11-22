use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderSallenKeyFilter<F, R1 = F, R2 = F, C1 = F, C2 = F>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>
{
    pub r1: R1,
    pub r2: R2,
    pub c1: C1,
    pub c2: C2,
    pub w: [F; 2]
}

impl<F, R1, R2, C1, C2> SecondOrderSallenKeyFilter<F, R1, R2, C1, C2>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>
{
    pub fn new(r1: R1, r2: R2, c1: C1, c2: C2) -> Self
    {
        Self {
            r1,
            r2,
            c1,
            c2,
            w: [F::zero(); 2]
        }
    }
    
    pub fn r1(&self) -> F
    {
        *(&self.r1).deref()
    }
    pub fn r2(&self) -> F
    {
        *(&self.r2).deref()
    }
    pub fn c1(&self) -> F
    {
        *(&self.c1).deref()
    }
    pub fn c2(&self) -> F
    {
        *(&self.c2).deref()
    }
    
    pub fn omega(&self) -> F
    {
        (self.r1()*self.r2()*self.c1()*self.c2()).sqrt().recip()
    }

    pub fn zeta(&self) -> F
    {
        let r1 = self.r1();
        let r2 = self.r2();
        let c1 = self.c1();
        f!(0.5)*((r2*c1).recip() + (r1*c1).recip())*(r1*r2*c1*self.c2()).sqrt()
    }
}

iir2_impl!(
    <R1, R2, C1, C2> SecondOrderSallenKeyFilter<F, R1, R2, C1, C2>: 3: false =>
    SecondOrderSallenKeyFilter<f32>;
    SecondOrderSallenKeyFilter<f64>
    where
        R1: Param<F>,
        R2: Param<F>,
        C1: Param<F>,
        C2: Param<F>
);

second_order_parameterization!(
    <R1, R2, C1, C2> SecondOrderSallenKeyFilter<F, R1, R2, C1, C2>
    where
        R1: Param<F>,
        R2: Param<F>,
        C1: Param<F>,
        C2: Param<F>
);

impl<F, R1, R2, C1, C2> FilterStaticInternals<F> for SecondOrderSallenKeyFilter<F, R1, R2, C1, C2>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
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

    use super::SecondOrderSallenKeyFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderSallenKeyFilter::new(1000.0, 1000.0, 0.00000001, 0.00000001);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}