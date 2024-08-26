use bytemuck::Pod;
use num::{Complex, Float};

use crate::{f, param::FilterParam, private::NotSame};

use super::{ButterworthFilterParam, ChebyshevFilterParam, SecondOrderButterworthFilter, SecondOrderChebyshev1Filter};

pub trait EllipticFilterParam: FilterParam
{
    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
    fn xi(&self) -> Self::F;
}
crate::def_param!(
    OmegaEpsilonXi<F> {
        omega: F,
        epsilon: F,
        xi: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for OmegaEpsilonXi<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> EllipticFilterParam for OmegaEpsilonXi<F>
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
    fn xi(&self) -> Self::F
    {
        *self.xi
    }
}

crate::def_rtf!(
    SecondOrderEllipticFilter
    {
        type Param: EllipticFilterParam = OmegaEpsilonXi;

        const OUTPUTS: usize = 2;
        const BUFFERED_OUTPUTS: bool = true;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let omega = param.omega();
            let epsilon = param.epsilon();
            let xi = param.xi();
    
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
            let epsilon2 = epsilon*epsilon;
            
            let g = (tm1*tm1 + epsilon2*tp1*tp1).sqrt().recip();
            (
                ([], [
                    [
                        (omega2 - f!(4.0)*rate2*tm1)*g,
                        (f!(2.0)*omega2 + f!(8.0)*rate2*tm1)*g,
                        (omega2 - f!(4.0)*rate2*tm1)*g
                    ],
                    [
                        (tm1*omega2 - f!(4.0)*rate2)*g,
                        (f!(2.0)*tm1*omega2 + f!(8.0)*rate2)*g,
                        (tm1*omega2 - f!(4.0)*rate2)*g
                    ]
                ]),
                [([], [
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
                ])]
            )
        }
    }
);
impl<P> From<P> for OmegaEpsilonXi<P::F>
where
    P: EllipticFilterParam + NotSame<OmegaEpsilonXi<P::F>>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilonXi::new(value.omega(), value.epsilon(), value.xi())
    }
}
impl<P> From<SecondOrderButterworthFilter<P::F, P>> for SecondOrderEllipticFilter<P::F>
where
    P: ButterworthFilterParam
{
    fn from(value: SecondOrderButterworthFilter<P::F, P>) -> Self
    {
        let omega = value.param.omega();
        let x = omega.recip();
        let x2 = x*x;
        let rn = f!(2.0; P::F).mul_add(x2, f!(-1.0; P::F));
        let epsilon = rn.recip();
        SecondOrderEllipticFilter::new(OmegaEpsilonXi::new(omega, epsilon, Float::infinity()))
    }
}
impl<P> From<SecondOrderChebyshev1Filter<P::F, P>> for SecondOrderEllipticFilter<P::F>
where
    P: ChebyshevFilterParam
{
    fn from(value: SecondOrderChebyshev1Filter<P::F, P>) -> Self
    {
        SecondOrderEllipticFilter::new(OmegaEpsilonXi::new(value.param.omega(), value.param.epsilon(), Float::infinity()))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::{OmegaEpsilonXi, SecondOrderEllipticFilter};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderEllipticFilter::new(OmegaEpsilonXi::new(10000.0*TAU, 0.5, 1.5));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}