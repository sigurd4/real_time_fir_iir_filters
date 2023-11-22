use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct ThirdOrderButterworthFilter<F, Omega = F>
where
    F: Float,
    Omega: Param<F>
{
    pub omega: Omega,
    pub w: [F; 3]
}

impl<F, Omega> ThirdOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>
{
    pub fn new(omega: Omega) -> Self
    {
        Self
        {
            omega,
            w: [F::zero(); 3]
        }
    }

    pub fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }
    pub fn zeta() -> F
    {
        f!(0.5)
    }
}

iir3_impl!(
    <Omega> ThirdOrderButterworthFilter<F, Omega>: 4: false =>
    ThirdOrderButterworthFilter<f32>;
    ThirdOrderButterworthFilter<f64>
    where
        Omega: Param<F>
);

impl<F, Omega> FilterStaticCoefficients<F> for ThirdOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 4]; 4])
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;
        
        let omega = self.omega();
        let omega2 = omega*omega;
        let omega3 = omega2*omega;
        ([], [
            [
                omega3,
                f!(3.0)*omega3,
                f!(3.0)*omega3,
                omega3
            ],
            [
                f!(2.0)*rate*omega2,
                f!(2.0)*rate*omega2,
                f!(-2.0)*rate*omega2,
                f!(-2.0)*rate*omega2
            ],
            [
                f!(4.0)*rate2*omega,
                f!(-4.0)*rate2*omega,
                f!(-4.0)*rate2*omega,
                f!(4.0)*rate2*omega
            ],
            [
                f!(8.0)*rate3,
                f!(-24.0)*rate3,
                f!(24.0)*rate3,
                f!(-8.0)*rate3
            ]
        ])
    }
    
    fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 4]; 1])>
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let omega = *(&self.omega).deref();
        let omega2 = omega*omega;
        let omega3 = omega2*omega;
        Some(([], [[
            f!(8.0)*rate3 + f!(8.0)*rate2*omega + f!(4.0)*rate*omega2 + omega3,
            f!(-24.0)*rate3 - f!(8.0)*rate2*omega + f!(4.0)*rate*omega2 + f!(3.0)*omega3,
            f!(24.0)*rate3 - f!(8.0)*rate2*omega - f!(4.0)*rate*omega2 + f!(3.0)*omega3,
            f!(-8.0)*rate3 + f!(8.0)*rate2*omega - f!(4.0)*rate*omega2 + omega3
        ]]))
    }
}

impl<F, Omega> FilterStaticInternals<F> for ThirdOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 3]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::ThirdOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderButterworthFilter::new(10000.0*TAU);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}