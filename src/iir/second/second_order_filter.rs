use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderFilter<F, Omega = F, Zeta = F>
where
    F: Float,
    Omega: Param<F>,
    Zeta: Param<F>
{
    pub omega: Omega,
    pub zeta: Zeta,
    pub w: [F; 2]
}

impl<F, Omega, Zeta> SecondOrderFilter<F, Omega, Zeta>
where
    F: Float,
    Omega: Param<F>,
    Zeta: Param<F>
{
    pub fn new(omega: Omega, zeta: Zeta) -> Self
    {
        Self {
            omega,
            zeta,
            w: [F::one(); 2]
        }
    }
    
    pub fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }

    pub fn zeta(&self) -> F
    {
        *(&self.zeta).deref()
    }
}

iir2_impl!(
    <Omega, Zeta> SecondOrderFilter<F, Omega, Zeta>: 3: false =>
    SecondOrderFilter<f32>;
    SecondOrderFilter<f64>
    where
        Omega: Param<F>,
        Zeta: Param<F>
);

impl<F, Omega, Zeta> FilterStaticCoefficients<F> for SecondOrderFilter<F, Omega, Zeta>
where
    F: Float,
    Omega: Param<F>,
    Zeta: Param<F>,
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

impl<F, Omega, Zeta> FilterStaticInternals<F> for SecondOrderFilter<F, Omega, Zeta>
where
    F: Float,
    Omega: Param<F>,
    Zeta: Param<F>,
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

    use super::SecondOrderFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderFilter::new(10000.0*TAU, 1.0);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}