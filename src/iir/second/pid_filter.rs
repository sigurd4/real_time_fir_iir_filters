use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam, private::NotSame};

pub trait PIDFilterParam: FilterParam
{
    fn p(&self) -> Self::F;
    fn i(&self) -> Self::F;
    fn d(&self) -> Self::F;
}
crate::def_param!(
    PID<F> {
        p: F,
        i: F,
        d: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for PID<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> PIDFilterParam for PID<F>
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
    fn d(&self) -> Self::F
    {
        *self.d
    }
}
impl<P> From<P> for PID<P::F>
where
    P: PIDFilterParam + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        PID::new(value.p(), value.i(), value.d())
    }
}

crate::def_rtf!(
    PIDFilter
    {
        type Param: PIDFilterParam = PID;

        const OUTPUTS: usize = 1;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
            let p = param.p();
            let i = param.i();
            let d = param.d();
            (
                ([], [[
                    f!(4.0)*rate2*d + f!(2.0)*rate*p + i,
                    f!(-8.0)*rate2*d + f!(2.0)*i,
                    f!(4.0)*rate2*d - f!(2.0)*rate*p + i,
                ]]),
                [([], [[
                    f!(2.0)*rate,
                    f!(0.0),
                    f!(-2.0)*rate
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{PIDFilter, PID};

    #[test]
    fn plot()
    {
        let mut filter = PIDFilter::new(PID::new(1.0, 0.001, 0.00001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}