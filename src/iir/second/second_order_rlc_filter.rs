use array_math::ArrayOps;

use super::*;

/// # Configurations
/// ```
/// 0:
///     X-[R]-[L]-Y
///               |
///              [C]
///               |
///              GND
/// 1:
///     X-[R]-Y
///           |
///          [L]
///           |
///          [C]
///           |
///          GND
/// 2:
///     X-[C]-[L]-Y
///               |
///              [R]
///               |
///              GND
/// 3:
///     X-[C]-Y
///           |
///          [L]
///           |
///          [R]
///           |
///          GND
/// ```
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
}

iir2_impl!(
    <R, L, C> SecondOrderRLCFilter<F, R, L, C>: 4: false =>
    SecondOrderRLCFilter<f32>;
    SecondOrderRLCFilter<f64>
    where
        R: Param<F>,
        L: Param<F>,
        C: Param<F>
);

impl<F, R, L, C> FilterStaticCoefficients<F> for SecondOrderRLCFilter<F, R, L, C>
where
    F: Float,
    R: Param<F>,
    L: Param<F>,
    C: Param<F>,
{
    fn b(&self, rate: F) -> ([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS])
    {
        let rate2 = rate*rate;

        let r = self.r();
        let l = self.l();
        let c = self.c();

        ([], [
            [
                f!(1.0),
                f!(2.0),
                f!(1.0),
            ],
            [
                f!(1.0) + f!(4.0)*c*l*rate2,
                f!(2.0) - f!(8.0)*c*l*rate2,
                f!(1.0) + f!(4.0)*c*l*rate2,
            ],
            [
                c*r*rate*f!(2.0),
                f!(0.0),
                c*r*rate*f!(-2.0),
            ],
            [
                c*rate*(f!(4.0)*l*rate + f!(2.0)*r),
                c*l*rate2*f!(-8.0),
                c*rate*(f!(4.0)*l*rate - f!(2.0)*r),
            ],
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])>
    {
        let rate2 = rate*rate;

        let r = self.r();
        let l = self.l();
        let c = self.c();

        Some(([], [[
            f!(1.0) + f!(4.0)*c*l*rate2 + f!(2.0)*c*r*rate,
            f!(2.0) - f!(8.0)*c*l*rate2,
            f!(1.0) + f!(4.0)*c*l*rate2 - f!(2.0)*c*r*rate,
        ]]))
    }
}

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