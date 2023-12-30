use array_math::ArrayOps;

use super::*;

/// # Configurations
/// ```
/// 0: X-[R1]-o-[R2]-Y
///           |      |
///          [C1]   [C2]
///           |      |
///          GND    GND
/// 1: X-[C1]-o-[R2]-Y
///           |      |
///          [R1]   [C2]
///           |      |
///          GND    GND
/// 2: X-[R1]-o-[C2]-Y
///           |      |
///          [C1]   [R2]
///           |      |
///          GND    GND
/// 3: X-[C1]-o-[C2]-Y
///           |      |
///          [R1]   [R2]
///           |      |
///          GND    GND
/// ```
#[derive(Copy, Clone)]
pub struct SecondOrderRCFilter<F, R1 = F, R2 = F, C1 = F, C2 = F>
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

impl<F, R1, R2, C1, C2> SecondOrderRCFilter<F, R1, R2, C1, C2>
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
}

iir2_impl!(
    <R1, R2, C1, C2> SecondOrderRCFilter<F, R1, R2, C1, C2>: 4: false =>
    SecondOrderRCFilter<f32>;
    SecondOrderRCFilter<f64>
    where
        R1: Param<F>,
        R2: Param<F>,
        C1: Param<F>,
        C2: Param<F>
);

impl<F, R1, R2, C1, C2> FilterStaticCoefficients<F> for SecondOrderRCFilter<F, R1, R2, C1, C2>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
{
    fn b(&self, rate: F) -> ([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS])
    {
        let rate2 = rate*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let c1 = self.c1();
        let c2 = self.c2();

        ([], [
            [
                f!(1.0),
                f!(2.0),
                f!(1.0),
            ],
            [
                c1*r1*rate*f!(2.0),
                f!(0.0),
                c1*r1*rate*f!(-2.0),
            ],
            [
                c2*r2*rate*f!(2.0),
                f!(0.0),
                c2*r2*rate*f!(-2.0),
            ],
            [
                c1*c2*r1*r2*rate2*f!(4.0),
                c1*c2*r1*r2*rate2*f!(-8.0),
                c1*c2*r1*r2*rate2*f!(4.0),
            ],
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])>
    {
        let rate2 = rate*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let c1 = self.c1();
        let c2 = self.c2();

        Some(([], [[
            c1*r1*rate*(f!(2.0) + f!(4.0)*c2*r2*rate) + f!(1.0) + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate,
            f!(-8.0)*c1*c2*r1*r2*rate2 + f!(2.0),
            c1*r1*rate*(f!(-2.0) + f!(4.0)*c2*r2*rate) + f!(1.0) - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate,
        ]]))
    }
}



impl<F, R1, R2, C1, C2> FilterStaticInternals<F> for SecondOrderRCFilter<F, R1, R2, C1, C2>
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

    use super::SecondOrderRCFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRCFilter::new(390e3, 4.7e3, 100e-9, 47e-12);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}