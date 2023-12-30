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

impl<F, Omega> FilterStaticCoefficients<F> for SecondOrderButterworthFilter<F, Omega>
where
    F: Float,
    Omega: Param<F>
{
    fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 3]; 3])
    {
        let omega = self.omega();
        let omega2 = omega*omega;

        let rate2 = rate*rate;
        ([], [
            [
                omega2,
                omega2*f!(2.0),
                omega2
            ],
            [
                rate*omega*f!(2.0),
                f!(0.0; F),
                rate*omega*f!(-2.0),
            ],
            [
                rate2*f!(4.0),
                rate2*f!(-8.0),
                rate2*f!(4.0)
            ]
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 3]; 1])>
    {
        let omega = self.omega();
        let omega2 = omega*omega;

        let zeta = self.zeta();

        let rate2 = rate*rate;
        Some(([], [
            [
                rate2*f!(4.0) + rate*zeta*omega*f!(4.0) + omega2,
                omega2*f!(2.0) - rate2*f!(8.0),
                rate2*f!(4.0) - rate*zeta*omega*f!(4.0) + omega2
            ]
        ]))
    }
}

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