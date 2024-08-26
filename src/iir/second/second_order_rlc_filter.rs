use bytemuck::Pod;
use num::{Float, Zero};

use crate::{f, real_time_fir_iir_filters, iir::first::{FirstOrderLRFilter, FirstOrderLRFilterParam, FirstOrderRCFilterParam}, param::FilterParam};

pub trait SecondOrderRLCFilterParam: FilterParam
{
    fn r(&self) -> Self::F;
    fn l(&self) -> Self::F;
    fn c(&self) -> Self::F;
}

crate::def_param!(
    RLC<F> {
        r: F,
        l: F,
        c: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RLC<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> SecondOrderRLCFilterParam for RLC<F>
where
    F: Float + Pod
{
    fn r(&self) -> Self::F
    {
        *self.r
    }
    fn l(&self) -> Self::F
    {
        *self.l
    }
    fn c(&self) -> Self::F
    {
        *self.c
    }
}

impl<P> From<P> for RLC<P::F>
where
    P: FirstOrderRCFilterParam
{
    fn from(value: P) -> Self
    {
        RLC::new(value.r(), Zero::zero(), value.c())
    }
}
impl<F, P> From<FirstOrderLRFilter<F, P>> for SecondOrderRLCFilter<F, P>
where
    F: Float + Pod,
    P: FirstOrderLRFilterParam<F = F>
{
    fn from(value: FirstOrderLRFilter<F, P>) -> Self
    {
        SecondOrderRLCFilter::new(value.param)
    }
}

crate::def_rtf!(
    {
        /// # Configurations
        /// ```
        /// 0) LOW-PASS:
        ///     X-[R]-[L]-Y
        ///               |
        ///              [C]
        ///               |
        ///              GND
        /// 1) BAND-STOP:
        ///     X-[R]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [C]
        ///           |
        ///          GND
        /// 2) BAND-PASS:
        ///     X-[C]-[L]-Y
        ///               |
        ///              [R]
        ///               |
        ///              GND
        /// 3) HIGH-PASS:
        ///     X-[C]-Y
        ///           |
        ///          [L]
        ///           |
        ///          [R]
        ///           |
        ///          GND
        /// ```
    }
    SecondOrderRLCFilter
    {
        type Param: SecondOrderRLCFilterParam = RLC;

        const OUTPUTS: usize = 4;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
    
            let r = param.r();
            let l = param.l();
            let c = param.c();
            (
                ([], [
                    [
                        f!(1.0),
                        f!(2.0),
                        f!(1.0),
                    ],
                    [
                        f!(1.0) + f!(4.0)*c*l*rate2,
                        f!(2.0) - f!(8.0)*c*l*rate2,
                        f!(1.0) + f!(4.0)*c*l*rate2,
                    ],
                    [
                        c*r*rate*f!(2.0),
                        f!(0.0),
                        c*r*rate*f!(-2.0),
                    ],
                    [
                        c*rate*(f!(4.0)*l*rate + f!(2.0)*r),
                        c*l*rate2*f!(-8.0),
                        c*rate*(f!(4.0)*l*rate - f!(2.0)*r),
                    ],
                ]),
                [([], [[
                    f!(1.0) + f!(4.0)*c*l*rate2 + f!(2.0)*c*r*rate,
                    f!(2.0) - f!(8.0)*c*l*rate2,
                    f!(1.0) + f!(4.0)*c*l*rate2 - f!(2.0)*c*r*rate,
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{SecondOrderRLCFilter, RLC};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRLCFilter::new(RLC::new(1000.0, 0.01, 0.000000033));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}