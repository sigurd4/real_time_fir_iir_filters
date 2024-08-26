use bytemuck::Pod;
use num::{Float, One, Zero};

use crate::{f, real_time_fir_iir_filters, iir::second::SecondOrderSallenKeyFilterParam, param::FilterParam, private::NotSame};

pub trait ThirdOrderSallenKeyFilterParam: FilterParam
{
    fn r1(&self) -> Self::F;
    fn c1(&self) -> Self::F;
    fn r2(&self) -> Self::F;
    fn c2(&self) -> Self::F;
    fn r3(&self) -> Self::F;
    fn c3(&self) -> Self::F;
    fn g(&self) -> Self::F;
}

crate::def_param!(
    RC3GSallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F,
        r3: F,
        c3: F,
        g: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC3GSallenKey<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> ThirdOrderSallenKeyFilterParam for RC3GSallenKey<F>
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
    fn r3(&self) -> Self::F
    {
        *self.r3
    }
    fn c3(&self) -> Self::F
    {
        *self.c3
    }
    fn g(&self) -> Self::F
    {
        *self.g
    }
}
/*impl<P> ThirdOrderSallenKeyFilterParam for P
where
    P: FirstOrderRCFilterParam
{
    fn r1(&self) -> Self::F
    {
        self.r()
    }
    fn c1(&self) -> Self::F
    {
        self.c()
    }
    fn r2(&self) -> Self::F
    {
        Zero::zero()
    }
    fn c2(&self) -> Self::F
    {
        Zero::zero()
    }
    fn r3(&self) -> Self::F
    {
        Zero::zero()
    }
    fn c3(&self) -> Self::F
    {
        Zero::zero()
    }
    fn g(&self) -> Self::F
    {
        One::one()
    }
}*/
impl<P> ThirdOrderSallenKeyFilterParam for P
where
    P: SecondOrderSallenKeyFilterParam
{
    fn r1(&self) -> Self::F
    {
        Zero::zero()
    }
    fn c1(&self) -> Self::F
    {
        Zero::zero()
    }
    fn r2(&self) -> Self::F
    {
        SecondOrderSallenKeyFilterParam::r1(self)
    }
    fn c2(&self) -> Self::F
    {
        SecondOrderSallenKeyFilterParam::c1(self)
    }
    fn r3(&self) -> Self::F
    {
        SecondOrderSallenKeyFilterParam::r2(self)
    }
    fn c3(&self) -> Self::F
    {
        SecondOrderSallenKeyFilterParam::c2(self)
    }
    fn g(&self) -> Self::F
    {
        SecondOrderSallenKeyFilterParam::g(self)
    }
}

crate::def_param!(
    RC3SallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F,
        r3: F,
        c3: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC3SallenKey<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> ThirdOrderSallenKeyFilterParam for RC3SallenKey<F>
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
    fn r3(&self) -> Self::F
    {
        *self.r3
    }
    fn c3(&self) -> Self::F
    {
        *self.c3
    }
    fn g(&self) -> Self::F
    {
        One::one()
    }
}

// TODO: Do it in SOS
crate::def_rtf!(
    ThirdOrderSallenKeyFilter
    {
        type Param: ThirdOrderSallenKeyFilterParam = RC3GSallenKey;

        const OUTPUTS: usize = 8;
        const BUFFERED_OUTPUTS: bool = true;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
            let rate3 = rate2*rate;
    
            let r1 = param.r1();
            let c1 = param.c1();
            let r2 = param.r2();
            let c2 = param.c2();
            let r3 = param.r3();
            let c3 = param.c3();
            let g = param.g();
            (
                ([], [
                    [
                        g*r2*f!(1.0),
                        g*r2*f!(3.0),
                        g*r2*f!(3.0),
                        g*r2*f!(1.0),
                    ],
                    [
                        c1*g*r1*r2*rate*f!(2.0),
                        c1*g*r1*r2*rate*f!(2.0),
                        c1*g*r1*r2*rate*f!(-2.0),
                        c1*g*r1*r2*rate*f!(-2.0),
                    ],
                    [
                        c2*g*r2*rate*f!(2.0),
                        c2*g*r2*rate*f!(2.0),
                        c2*g*r2*rate*f!(-2.0),
                        c2*g*r2*rate*f!(-2.0),
                    ],
                    [
                        c1*c2*g*r1*r2*rate2*f!(4.0),
                        c1*c2*g*r1*r2*rate2*f!(-4.0),
                        c1*c2*g*r1*r2*rate2*f!(-4.0),
                        c1*c2*g*r1*r2*rate2*f!(4.0),
                    ],
                    [
                        c3*g*r2*r3*rate*f!(2.0),
                        c3*g*r2*r3*rate*f!(2.0),
                        c3*g*r2*r3*rate*f!(-2.0),
                        c3*g*r2*r3*rate*f!(-2.0),
                    ],
                    [
                        c1*c3*g*r1*r2*r3*rate2*f!(4.0),
                        c1*c3*g*r1*r2*r3*rate2*f!(-4.0),
                        c1*c3*g*r1*r2*r3*rate2*f!(-4.0),
                        c1*c3*g*r1*r2*r3*rate2*f!(4.0),
                    ],
                    [
                        c2*c3*g*r2*r3*rate2*f!(4.0),
                        c2*c3*g*r2*r3*rate2*f!(-4.0),
                        c2*c3*g*r2*r3*rate2*f!(-4.0),
                        c2*c3*g*r2*r3*rate2*f!(4.0),
                    ],
                    [
                        c1*c2*c3*g*r1*r2*r3*rate3*f!(8.0),
                        c1*c2*c3*g*r1*r2*r3*rate3*f!(-24.0),
                        c1*c2*c3*g*r1*r2*r3*rate3*f!(24.0),
                        c1*c2*c3*g*r1*r2*r3*rate3*f!(-8.0),
                    ],
                ]),
                [([], [
                    [
                        f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + r2 + f!(2.0)*r1,
                        -f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(3.0)*r2 + f!(6.0)*r1,
                        f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate + f!(3.0)*r2 + f!(6.0)*r1,
                        -f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate + r2 + f!(2.0)*r1,
                    ],
                    [
                        f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(2.0)*r1 + r2,
                        -f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(6.0)*r1 + f!(3.0)*r2,
                        f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate + f!(6.0)*r1 + f!(3.0)*r2,
                        -f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate + f!(2.0)*r1 + r2,
                    ],
                    [
                        f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(2.0)*c2*g*r1*rate + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                        -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(2.0)*c2*g*r1*rate - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                        -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(2.0)*c2*g*r1*rate + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                        f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(2.0)*c2*g*r1*rate - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                    ],
                    [
                        f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c2*g*r1*rate + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                        -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c2*g*r1*rate - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                        -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c2*g*r1*rate + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                        f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c2*g*r1*rate - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                    ],
                    [
                        f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + r2 + f!(2.0)*r1 + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate,
                        -f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(3.0)*r2 + f!(6.0)*r1 + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate,
                        -f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(3.0)*r2 + f!(6.0)*r1 - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate,
                        f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + r2 + f!(2.0)*r1 - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate,
                    ],
                    [
                        f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(2.0)*r1 + r2 + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate,
                        -f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(6.0)*r1 + f!(3.0)*r2 + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate,
                        -f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(6.0)*r1 + f!(3.0)*r2 - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate,
                        f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(2.0)*r1 + r2 - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate,
                    ],
                    [
                        f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                        f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                        -f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                        -f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                    ],
                    [
                        f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                        f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                        -f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                        -f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                    ],
                ])]
            )
        }
    }
);
impl<P> From<P> for RC3GSallenKey<P::F>
where
    P: ThirdOrderSallenKeyFilterParam + NotSame<RC3GSallenKey<P::F>>
{
    fn from(value: P) -> Self
    {
        RC3GSallenKey::new(value.r1(), value.c1(), value.r2(), value.c2(), value.r3(), value.c3(), value.g())
    }
}

#[cfg(test)]
mod test
{
    use super::{RC3GSallenKey, ThirdOrderSallenKeyFilter};

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderSallenKeyFilter::new(RC3GSallenKey::new(470.0, 15.0e3, 16.0e3, 47.0e-9, 2.7e-9, 2.7e-9, 1.3846153846153846));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}