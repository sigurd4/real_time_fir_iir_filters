use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam, private::NotSame};

pub trait FirstOrderAllPassFilterParam: FilterParam
{
    fn tau(&self) -> Self::F;
}

crate::def_param!(
    Tau<F> {
        tau: F
    }
    where
        F: Float + Pod
);
impl<F> FilterParam for Tau<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> FirstOrderAllPassFilterParam for Tau<F>
where
    F: Float + Pod
{
    fn tau(&self) -> Self::F
    {
        *self.tau
    }
}
impl<P> From<P> for Tau<P::F>
where
    P: FirstOrderAllPassFilterParam + NotSame<Tau<P::F>>
{
    fn from(value: P) -> Self
    {
        Tau::new(value.tau())
    }
}

crate::def_rtf!(
    FirstOrderAllPassFilter
    {
        type Param: FirstOrderAllPassFilterParam = Tau;

        const OUTPUTS: usize = 1;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let tau = param.tau();
            (
                ([], [[
                    f!(2.0)*tau*rate - f!(1.0),
                    f!(1.0) - f!(2.0)*tau*rate
                ]]),
                [([], [[
                    f!(1.0) + f!(2.0)*tau*rate,
                    f!(1.0) - f!(2.0)*tau*rate
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{FirstOrderAllPassFilter, Tau};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderAllPassFilter::new(Tau::new(0.001));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}