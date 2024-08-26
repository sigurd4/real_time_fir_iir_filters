use bytemuck::Pod;
use num::{Float, Zero};

use crate::{f, iir::second::{SecondOrderRCFilter, SecondOrderRCFilterParam}, param::FilterParam, private::NotSame};

pub trait FirstOrderRCFilterParam: FilterParam
    //+ NotFirstOrderLRFilterParam
{
    fn r(&self) -> Self::F;
    fn c(&self) -> Self::F;
}

crate::def_param!(
    RC<F> {
        r: F,
        c: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> FirstOrderRCFilterParam for RC<F>
where
    F: Float + Pod
{
    fn r(&self) -> Self::F
    {
        *self.r
    }
    fn c(&self) -> Self::F
    {
        *self.c
    }
}
/*impl<P> FirstOrderFilterParam for P
where
    P: FirstOrderRCFilterParam
{
    fn omega(&self) -> Self::F
    {
        (self.r()*self.c()).recip()
    }
}*/
impl<P> From<P> for RC<P::F>
where
    P: FirstOrderRCFilterParam + NotSame<RC<P::F>>
{
    fn from(value: P) -> Self
    {
        RC::new(value.r(), value.c())
    }
}
impl<P> SecondOrderRCFilterParam for P
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
}
/*impl<P> SecondOrderRLCFilterParam for P
where
    P: FirstOrderRCFilterParam
{
    #[doc(hidden)]
    fn r(&self) -> Self::F
    {
        self.r()
    }
    fn l(&self) -> Self::F
    {
        Zero::zero()
    }
    #[doc(hidden)]
    fn c(&self) -> Self::F
    {
        self.c()
    }
}*/

crate::def_rtf!(
    {
        /// # Configurations
        /// ```
        /// 0) LOW-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 
        /// 1) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// ```
    }
    FirstOrderRCFilter
    {
        type Param: FirstOrderRCFilterParam = RC;

        const OUTPUTS: usize = 2;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let two_rate_mul_omega_inv = f!(2.0)*rate*param.r()*param.c();
            let one = F::one();
            (
                ([], [
                    [
                        one,
                        one
                    ],
                    [
                        two_rate_mul_omega_inv,
                        -two_rate_mul_omega_inv
                    ]
                ]),
                [([], [[
                    one + two_rate_mul_omega_inv,
                    one - two_rate_mul_omega_inv,
                ]])]
            )
        }
    }
);

/*impl<F, P> From<FirstOrderRCFilter<F, P>> for FirstOrderFilter<F, P>
where
    F: Float + Pod,
    P: FirstOrderRCFilterParam<F = F>
{
    fn from(value: FirstOrderRCFilter<F, P>) -> Self
    {
        FirstOrderFilter::new(value.param)
    }
}*/
/*impl<F, P> From<FirstOrderRCFilter<F, P>> for SecondOrderRLCFilter<F, P>
where
    F: Float + Pod,
    P: FirstOrderRCFilterParam<F = F>
{
    fn from(value: FirstOrderRCFilter<F, P>) -> Self
    {
        SecondOrderRLCFilter::new(value.param)
    }
}*/
impl<F, P> From<FirstOrderRCFilter<F, P>> for SecondOrderRCFilter<F, P>
where
    F: Float + Pod,
    P: FirstOrderRCFilterParam<F = F>
{
    fn from(value: FirstOrderRCFilter<F, P>) -> Self
    {
        SecondOrderRCFilter::new(value.param)
    }
}

#[cfg(test)]
mod test
{
    use super::{FirstOrderRCFilter, RC};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderRCFilter::new(RC::new(10000.0, 0.000000033));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}