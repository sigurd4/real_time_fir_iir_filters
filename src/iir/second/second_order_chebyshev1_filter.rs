use std::f64::consts::SQRT_2;

use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderChebyshev1Filter<F, Omega = F, Epsilon = F>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>
{
    pub omega: Omega,
    pub epsilon: Epsilon,
    pub w: [[F; 2]; 2]
}

impl<F, Omega, Epsilon> SecondOrderChebyshev1Filter<F, Omega, Epsilon>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>
{
    pub fn new(omega: Omega, epsilon: Epsilon) -> Self
    {
        Self {
            omega,
            epsilon,
            w: [[F::zero(); 2]; 2]
        }
    }
    
    fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }

    fn epsilon(&self) -> F
    {
        *(&self.epsilon).deref()
    }

    fn alpha(&self) -> F
    {
        let epsilon = self.epsilon();
        f!(0.5; F)*epsilon.recip().asinh()
    }
}

iir2_impl!(
    <Omega, Epsilon> SecondOrderChebyshev1Filter<F, Omega, Epsilon>: 2: true =>
    SecondOrderChebyshev1Filter<f32>;
    SecondOrderChebyshev1Filter<f64>
    where
        Omega: Param<F>,
        Epsilon: Param<F>
);

impl<F, Omega, Epsilon> FilterStaticCoefficients<F> for SecondOrderChebyshev1Filter<F, Omega, Epsilon>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>
{
    fn b(&self, rate: F) -> ([[[F; 3]; 2]; 0], [[F; 3]; 2])
    {
        let omega = self.omega();
        let epsilon = self.epsilon();

        let rate2 = rate*rate;
        let omega2 = omega*omega;
        let epsilon_inv = epsilon.recip();

        ([], [
            [
                omega2,
                omega2*f!(2.0),
                omega2
            ].mul_all(epsilon_inv),
            [
                rate2*f!(4.0),
                -rate2*f!(8.0),
                rate2*f!(4.0)
            ].mul_all(epsilon_inv)
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 2]; 0], [[F; 3]; 2])>
    {
        let omega = self.omega();
        let alpha = self.alpha();

        let rate2 = rate*rate;
        let omega2 = omega*omega;
        let cosh_2alpha = (alpha*f!(2.0)).cosh();
        let sinh_alpha = alpha.sinh();

        Some(([],[
            [
                f!(8.0; F)*rate2 + f!(4.0*SQRT_2)*rate*omega*sinh_alpha + omega2*cosh_2alpha,
                -f!(16.0; F)*rate2 + f!(2.0)*omega2*cosh_2alpha,
                f!(8.0; F)*rate2 - f!(4.0*SQRT_2)*rate*omega*sinh_alpha + omega2*cosh_2alpha
            ],
            [
                f!(4.0; F)*rate2*cosh_2alpha + f!(4.0*SQRT_2)*rate*omega*sinh_alpha + f!(2.0)*omega2,
                -f!(8.0; F)*rate2*cosh_2alpha + f!(4.0)*omega2,
                f!(4.0; F)*rate2*cosh_2alpha - f!(4.0*SQRT_2)*rate*omega*sinh_alpha + f!(2.0)*omega2
            ]
        ]))
    }
}

impl<F, Omega, Epsilon> FilterStaticInternals<F> for SecondOrderChebyshev1Filter<F, Omega, Epsilon>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 2]; 0], &mut [[F; 2]; 2])
    {
        ([], &mut self.w)
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::SecondOrderChebyshev1Filter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev1Filter::new(10000.0*TAU, 1.0);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}