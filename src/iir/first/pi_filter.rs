use bytemuck::Pod;
use num::{Float, Zero};

use crate::{f, real_time_fir_iir_filters, iir::second::{PIDFilter, PIDFilterParam}, param::FilterParam, private::NotSame};

pub trait PIFilterParam: FilterParam
{
    fn p(&self) -> Self::F;
    fn i(&self) -> Self::F;
}
crate::def_param!(
    PI<F> {
        p: F,
        i: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for PI<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> PIFilterParam for PI<F>
where
    F: Float + Pod
{
    fn p(&self) -> Self::F
    {
        *self.p
    }
    fn i(&self) -> Self::F
    {
        *self.i
    }
}
impl<P> PIDFilterParam for P
where
    P: PIFilterParam
{
    #[doc(hidden)]
    fn p(&self) -> Self::F
    {
        self.p()
    }
    #[doc(hidden)]
    fn i(&self) -> Self::F
    {
        self.i()
    }
    fn d(&self) -> Self::F
    {
        Zero::zero()
    }
}
impl<P> From<P> for PI<P::F>
where
    P: PIFilterParam + NotSame<PI<P::F>>
{
    fn from(value: P) -> Self
    {
        PI::new(value.p(), value.i())
    }
}

crate::def_rtf!(
    PIFilter
    {
        type Param: PIFilterParam = PI;

        const OUTPUTS: usize = 1;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let p = param.p();
            let i = param.i();
            (
                ([], [[
                    f!(2.0)*rate*p + i,
                    f!(-2.0)*rate*p + i
                ]]),
                [([], [[
                    f!(2.0)*rate,
                    f!(-2.0)*rate
                ]])]
            )
        }
    }
);

impl<F, P> From<PIFilter<F, P>> for PIDFilter<F, P>
where
    F: Float + Pod,
    P: PIFilterParam<F = F>
{
    fn from(value: PIFilter<F, P>) -> Self
    {
        PIDFilter::new(value.param)
    }
}

#[cfg(test)]
mod test
{
    use super::{PIFilter, PI};

    #[test]
    fn plot()
    {
        let mut filter = PIFilter::new(PI::new(1.0, 0.001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}