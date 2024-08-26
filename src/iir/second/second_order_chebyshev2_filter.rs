use std::f64::consts::SQRT_2;

use num::{Float, One};

use crate::f;

use super::{OmegaEpsilon, OmegaEpsilonXi, ChebyshevFilterParam, SecondOrderEllipticFilter};

crate::def_rtf!(
    SecondOrderChebyshev2Filter
    {
        type Param: ChebyshevFilterParam = OmegaEpsilon;

        const OUTPUTS: usize = 2;
        const BUFFERED_OUTPUTS: bool = true;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let omega = param.omega();
            let epsilon = param.epsilon();
    
            let rate2 = rate*rate;
            let omega2 = omega*omega;
            let epsilon_inv = epsilon.recip();
            let alpha = f!(0.5; F)*epsilon_inv.asinh();
            let cosh_2alpha = (alpha*f!(2.0)).cosh();
            let sinh_alpha = alpha.sinh();
            (
                ([], [
                    [
                        (f!(4.0)*rate2 + f!(2.0)*omega2)*epsilon_inv,
                        (f!(-8.0)*rate2 + f!(4.0)*omega2)*epsilon_inv,
                        (f!(4.0)*rate2 + f!(2.0)*omega2)*epsilon_inv
                    ],
                    [
                        (f!(8.0)*rate2 + omega2)*epsilon_inv,
                        (f!(-16.0)*rate2 + f!(2.0)*omega2)*epsilon_inv,
                        (f!(8.0)*rate2 + omega2)*epsilon_inv
                    ]
                ]),
                [([], [
                    [
                        f!(4.0)*rate2*cosh_2alpha + f!(4.0*SQRT_2)*rate*omega*sinh_alpha + f!(2.0)*omega2,
                        f!(-8.0)*rate2*cosh_2alpha + f!(4.0)*omega2,
                        f!(4.0)*rate2*cosh_2alpha - f!(4.0*SQRT_2)*rate*omega*sinh_alpha + f!(2.0)*omega2
                    ],
                    [
                        f!(8.0)*rate2 + f!(4.0*SQRT_2)*rate*omega*sinh_alpha + omega2*cosh_2alpha,
                        f!(-16.0)*rate2 + f!(2.0)*omega2*cosh_2alpha,
                        f!(8.0)*rate2 - f!(4.0*SQRT_2)*rate*omega*sinh_alpha + omega2*cosh_2alpha,
                    ]
                ])]
            )
        }
    }
);
impl<P> From<SecondOrderChebyshev2Filter<P::F, P>> for SecondOrderEllipticFilter<P::F>
where
    P: ChebyshevFilterParam
{
    fn from(value: SecondOrderChebyshev2Filter<P::F, P>) -> Self
    {
        // https://en.wikipedia.org/wiki/Elliptic_rational_functions
        let omega = value.param.omega();
        let xi = omega.recip();
        let one = One::one();
        let t = omega.hypot(one);
        let xi2 = xi*xi;
        let ln = (t + one).mul_add(xi2, f!(-1.0; P::F))/(t - one).mul_add(xi2, one);
        let alpha = value.param.epsilon()*ln;
        SecondOrderEllipticFilter::new(OmegaEpsilonXi::new(omega, alpha, xi))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::iir::second::OmegaEpsilon;

    use super::SecondOrderChebyshev2Filter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev2Filter::new(OmegaEpsilon::new(10000.0*TAU, 1.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}