use bytemuck::Pod;
use num::{Float, Zero};

use crate::{f, iir::second::SecondOrderRLCFilterParam, param::FilterParam, private::NotSame};

use super::FirstOrderFilterParam;

pub trait FirstOrderLRFilterParam: FirstOrderFilterParam
    //+ NotFirstOrderRCFilterParam
{
    fn l(&self) -> Self::F;
    fn r(&self) -> Self::F;
}

crate::def_param!(
    LR<F> {
        l: F,
        r: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for LR<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> FirstOrderLRFilterParam for LR<F>
where
    F: Float + Pod
{
    fn l(&self) -> Self::F
    {
        *self.l
    }
    fn r(&self) -> Self::F
    {
        *self.r
    }
}
impl<P> FirstOrderFilterParam for P
where
    P: FirstOrderLRFilterParam
{
    fn omega(&self) -> Self::F
    {
        self.r()/self.l()
    }
}
impl<P> SecondOrderRLCFilterParam for P
where
    P: FirstOrderLRFilterParam
{
    #[doc(hidden)]
    fn r(&self) -> Self::F
    {
        self.r()
    }
    #[doc(hidden)]
    fn l(&self) -> Self::F
    {
        self.l()
    }
    fn c(&self) -> Self::F
    {
        Zero::zero()
    }
}
impl<P> From<P> for LR<P::F>
where
    P: NotSame<LR<P::F>> + FirstOrderLRFilterParam
{
    fn from(value: P) -> Self
    {
        LR::new(value.l(), value.r())
    }
}

crate::def_rtf!(
    {
        /// # Configurations
        /// ```
        /// 0) LOW-PASS:
        ///     X-[L]-Y
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// 
        /// 1) HIGH-PASS:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          GND
        /// ```
    }
    FirstOrderLRFilter
    {
        type Param: FirstOrderLRFilterParam = LR;

        const OUTPUTS: usize = 2;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let r = param.r();
            let l = param.l();
            let two_rate_mul_l = f!(2.0)*rate*l;
            (
                ([], [
                    [
                        r,
                        r
                    ],
                    [
                        two_rate_mul_l,
                        -two_rate_mul_l
                    ]
                ]),
                [([], [[
                    r + two_rate_mul_l,
                    r - two_rate_mul_l,
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{FirstOrderLRFilter, LR};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderLRFilter::new(LR::new(0.1, 10000.0));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}