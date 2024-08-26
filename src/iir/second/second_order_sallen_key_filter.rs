use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam, private::NotSame};

pub trait SecondOrderSallenKeyFilterParam: FilterParam
{
    fn r1(&self) -> Self::F;
    fn c1(&self) -> Self::F;
    fn r2(&self) -> Self::F;
    fn c2(&self) -> Self::F;
    fn g(&self) -> Self::F;
}

crate::def_param!(
    RC2GSallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F,
        g: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC2GSallenKey<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> SecondOrderSallenKeyFilterParam for RC2GSallenKey<F>
where
    F: Float + Pod
{
    fn r1(&self) -> Self::F
    {
        *self.r1
    }
    fn c1(&self) -> Self::F
    {
        *self.c1
    }
    fn r2(&self) -> Self::F
    {
        *self.r2
    }
    fn c2(&self) -> Self::F
    {
        *self.c2
    }
    fn g(&self) -> Self::F
    {
        *self.g
    }
}

crate::def_param!(
    RC2SallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC2SallenKey<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> SecondOrderSallenKeyFilterParam for RC2SallenKey<F>
where
    F: Float + Pod
{
    fn r1(&self) -> Self::F
    {
        *self.r1
    }
    fn c1(&self) -> Self::F
    {
        *self.c1
    }
    fn r2(&self) -> Self::F
    {
        *self.r2
    }
    fn c2(&self) -> Self::F
    {
        *self.c2
    }
    fn g(&self) -> Self::F
    {
        F::one()
    }
}

crate::def_rtf!(
    SecondOrderSallenKeyFilter
    {
        type Param: SecondOrderSallenKeyFilterParam = RC2GSallenKey;

        const OUTPUTS: usize = 4;
        const BUFFERED_OUTPUTS: bool = true;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
    
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            let g = param.g();
            (
                ([], [
                    [
                        g*g,
                        g*f!(2.0),
                        g*g,
                    ],
                    [
                        c1*g*r1*rate*f!(2.0),
                        f!(0.0),
                        c1*g*r1*rate*f!(-2.0),
                    ],
                    [
                        c2*g*r2*rate*f!(2.0),
                        f!(0.0),
                        c2*g*r2*rate*f!(-2.0),
                    ],
                    [
                        c1*c2*g*r1*r2*rate2*f!(4.0),
                        c1*c2*g*r1*r2*rate2*f!(-8.0),
                        c1*c2*g*r1*r2*rate2*f!(4.0),
                    ]
                ]),
                [([], [
                    [
                        f!(1.0) + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c1*g*r1*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                        f!(2.0) - f!(8.0)*c1*c2*r1*r2*rate2,
                        f!(1.0) - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c1*g*r1*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                    ],
                    [
                        f!(1.0) - g + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                        f!(2.0) - f!(2.0)*g - f!(8.0)*c1*c2*r1*r2*rate2,
                        f!(1.0) - g - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                    ],
                    [
                        f!(1.0) + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate - f!(4.0)*c1*c2*g*r1*r2*rate2 + f!(4.0)*c1*c2*r1*r2*rate2,
                        f!(2.0) + f!(8.0)*c1*c2*g*r1*r2*rate2 - f!(8.0)*c1*c2*r1*r2*rate2,
                        f!(1.0) - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate - f!(4.0)*c1*c2*g*r1*r2*rate2 + f!(4.0)*c1*c2*r1*r2*rate2,
                    ],
                    [
                        f!(1.0) - f!(2.0)*c2*g*r2*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                        f!(2.0) - f!(8.0)*c1*c2*r1*r2*rate2,
                        f!(1.0) + f!(2.0)*c2*g*r2*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                    ]
                ])]
            )
        }
    }
);
impl<P> From<P> for RC2GSallenKey<P::F>
where
    P: SecondOrderSallenKeyFilterParam + NotSame<RC2GSallenKey<P::F>>
{
    fn from(value: P) -> Self
    {
        RC2GSallenKey::new(value.r1(), value.c1(), value.r2(), value.c2(), value.g())
    }
}

#[cfg(test)]
mod test
{
    use super::{SecondOrderSallenKeyFilter, RC2GSallenKey};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderSallenKeyFilter::new(RC2GSallenKey::new(15.0e3, 15.0e3, 2.7e-9, 2.7e-9, 2.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}