use std::f64::consts::SQRT_2;

use array_math::ArrayOps;
use num::Complex;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderEllipticFilter<F, Omega = F, Epsilon = F, Xi = F>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>,
    Xi: Param<F>
{
    pub omega: Omega,
    pub epsilon: Epsilon,
    pub xi: Xi,
    pub w: [[F; 2]; 2]
}

impl<F, Omega, Epsilon, Xi> SecondOrderEllipticFilter<F, Omega, Epsilon, Xi>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>,
    Xi: Param<F>
{
    pub fn new(omega: Omega, epsilon: Epsilon, xi: Xi) -> Self
    {
        Self {
            omega,
            epsilon,
            xi,
            w: [[F::zero(); 2]; 2]
        }
    }
    
    pub fn omega(&self) -> F
    {
        *(&self.omega).deref()
    }

    pub fn epsilon(&self) -> F
    {
        *(&self.epsilon).deref()
    }

    pub fn xi(&self) -> F
    {
        *(&self.xi).deref()
    }
}

iir2_impl!(
    <Omega, Epsilon, Xi> SecondOrderEllipticFilter<F, Omega, Epsilon, Xi>: 2: true =>
    SecondOrderEllipticFilter<f32>;
    SecondOrderEllipticFilter<f64>
    where
        Omega: Param<F>,
        Epsilon: Param<F>,
        Xi: Param<F>
);

impl<F, Omega, Epsilon, Xi> FilterStaticCoefficients<F> for SecondOrderEllipticFilter<F, Omega, Epsilon, Xi>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>,
    Xi: Param<F>
{
    fn b(&self, rate: F) -> ([[[F; 3]; 2]; 0], [[F; 3]; 2])
    {
        let omega = self.omega();
        let epsilon = self.epsilon();
        let xi = self.xi();

        let xi2 = xi*xi;
        let xi3 = xi2*xi;

        let t = (F::one() - xi3.recip()).sqrt();
        let tm1 = t - F::one();
        let tp1 = t + F::one();

        let rate2 = rate*rate;
        let omega2 = omega*omega;
        let epsilon2 = epsilon*epsilon;

        let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();

        ([], [
            [
                omega2 - f!(4.0)*rate2*tm1,
                f!(2.0)*omega2 + f!(8.0)*rate2*tm1,
                omega2 - f!(4.0)*rate2*tm1
            ].mul_all(g),
            [
                tm1*omega2 - f!(4.0)*rate2,
                f!(2.0)*tm1*omega2 + f!(8.0)*rate2,
                tm1*omega2 - f!(4.0)*rate2
            ].mul_all(g)
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 2]; 0], [[F; 3]; 2])>
    {
        let omega = self.omega();
        let epsilon = self.epsilon();
        let xi = self.xi();

        let xi2 = xi*xi;
        let xi3 = xi2*xi;

        let t = (F::one() - xi3.recip()).sqrt();
        let tm1 = t - F::one();
        let tp1 = t + F::one();

        let s1 = -(Complex::new(epsilon, F::one())/Complex::new(-epsilon*tp1, tm1)).sqrt();
        let s2 = s1.conj();
        let s1ms2 = (s1*s2).re;
        let s1ps2 = (s1 + s2).re;

        let rate2 = rate*rate;
        let omega2 = omega*omega;

        Some(([], [
            [
                f!(4.0)*rate2 - f!(2.0)*rate*s1ps2*omega + s1ms2*omega2,
                -f!(8.0)*rate2 + f!(2.0)*s1ms2*omega2,
                f!(4.0)*rate2 + f!(2.0)*rate*s1ps2*omega + s1ms2*omega2
            ],
            [
                omega2 - f!(2.0)*rate*s1ps2*omega + f!(4.0)*rate2*s1ms2,
                f!(2.0)*omega2 - f!(8.0)*rate2*s1ms2,
                omega2 + f!(2.0)*rate*s1ps2*omega + f!(4.0)*rate2*s1ms2
            ]
        ]))
    }
}

impl<F, Omega, Epsilon, Xi> FilterStaticInternals<F> for SecondOrderEllipticFilter<F, Omega, Epsilon, Xi>
where
    F: Float,
    Omega: Param<F>,
    Epsilon: Param<F>,
    Xi: Param<F>,
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

    use super::SecondOrderEllipticFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderEllipticFilter::new(10000.0*TAU, 0.5, 1.5);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}