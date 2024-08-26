use std::f64::consts::SQRT_2;

use bytemuck::Pod;
use num::Float;

use crate::{f, param::FilterParam, private::NotSame};

pub trait ChebyshevFilterParam: FilterParam
{
    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
}
crate::def_param!(
    OmegaEpsilon<F> {
        omega: F,
        epsilon: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for OmegaEpsilon<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> ChebyshevFilterParam for OmegaEpsilon<F>
where
    F: Float + Pod
{
    fn omega(&self) -> Self::F
    {
        *self.omega
    }
    fn epsilon(&self) -> Self::F
    {
        *self.epsilon
    }
}

crate::def_rtf!(
    SecondOrderChebyshev1Filter
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
                        omega2*epsilon_inv,
                        omega2*epsilon_inv*f!(2.0),
                        omega2*epsilon_inv
                    ],
                    [
                        rate2*epsilon_inv*f!(4.0),
                        -rate2*epsilon_inv*f!(8.0),
                        rate2*epsilon_inv*f!(4.0)
                    ]
                ]),
                [([], [
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
                ])]
            )
        }
    }
);
impl<P> From<P> for OmegaEpsilon<P::F>
where
    P: ChebyshevFilterParam + NotSame<OmegaEpsilon<P::F>>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilon::new(value.omega(), value.epsilon())
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::{OmegaEpsilon, SecondOrderChebyshev1Filter};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderChebyshev1Filter::new(OmegaEpsilon::new(10000.0*TAU, 1.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}