use array_math::ArrayOps;

use super::*;

pub struct PIDFilter<F, P = F, I = F, D = F>
where
    F: Float,
    P: Param<F>,
    I: Param<F>,
    D: Param<F>
{
    pub p: P,
    pub i: I,
    pub d: D,
    pub w: [F; 2]
}

impl<F, P, I, D> PIDFilter<F, P, I, D>
where
    F: Float,
    P: Param<F>,
    I: Param<F>,
    D: Param<F>
{
    pub fn new(p: P, i: I, d: D) -> Self
    {
        Self {
            p,
            i,
            d,
            w: [F::zero(); 2]
        }
    }

    fn p(&self) -> F
    {
        *(&self.p).deref()
    }
    fn i(&self) -> F
    {
        *(&self.i).deref()
    }
    fn d(&self) -> F
    {
        *(&self.d).deref()
    }
}

iir2_impl!(
    <P, I, D> PIDFilter<F, P, I, D>: 1: false =>
    PIDFilter<f32>;
    PIDFilter<f64>
    where
        P: Param<F>,
        I: Param<F>,
        D: Param<F>
);

impl<F, P, I, D> FilterStaticCoefficients<F> for PIDFilter<F, P, I, D>
where
    F: Float,
    P: Param<F>,
    I: Param<F>,
    D: Param<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 3]; 1])
    {
        let p = self.p();
        let i = self.i();
        let d = self.d();

        let rate2 = rate*rate;
        ([], [[
            f!(4.0)*rate2*d + f!(2.0)*rate*p + i,
            f!(-8.0)*rate2*d + f!(2.0)*i,
            f!(4.0)*rate2*d - f!(2.0)*rate*p + i,
        ]])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 3]; 1])>
    {
        Some(([], [[
            f!(2.0)*rate,
            f!(0.0),
            f!(-2.0)*rate
        ]]))
    }
}

impl<F, P, I, D> FilterStaticInternals<F> for PIDFilter<F, P, I, D>
where
    F: Float,
    P: Param<F>,
    I: Param<F>,
    D: Param<F>,
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

    use super::PIDFilter;

    #[test]
    fn plot()
    {
        let mut filter = PIDFilter::new(1.0, 0.001, 0.00001);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}