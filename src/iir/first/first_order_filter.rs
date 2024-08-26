use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam, private::NotSame};

use super::{FirstOrderLRFilter, FirstOrderLRFilterParam};

pub trait FirstOrderFilterParam: FilterParam
{
    fn omega(&self) -> Self::F;
}

crate::def_param!(
    Omega<F> {
        omega: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for Omega<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> FirstOrderFilterParam for Omega<F>
where
    F: Float + Pod
{
    fn omega(&self) -> Self::F
    {
        *self.omega
    }
}

impl<P> From<P> for Omega<P::F>
where
    P: FirstOrderFilterParam + NotSame<Omega<P::F>>
{
    fn from(value: P) -> Self
    {
        Omega::new(value.omega())
    }
}
impl<F, P> From<FirstOrderLRFilter<F, P>> for FirstOrderFilter<F, P>
where
    F: Float + Pod,
    P: FirstOrderLRFilterParam<F = F>
{
    fn from(value: FirstOrderLRFilter<F, P>) -> Self
    {
        FirstOrderFilter::new(value.param)
    }
}

crate::def_rtf!(
    FirstOrderFilter
    {
        type Param: FirstOrderFilterParam = Omega;

        const OUTPUTS: usize = 2;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 1;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let omega = param.omega();
            let two_rate = f!(2.0)*rate;
            (
                ([], [
                    [
                        omega,
                        omega
                    ],
                    [
                        two_rate,
                        -two_rate
                    ]
                ]),
                [([], [[
                    omega + two_rate,
                    omega - two_rate,
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::{FirstOrderFilter, Omega};

    #[test]
    fn plot()
    {
        let mut filter = FirstOrderFilter::new(Omega::new(10000.0*TAU));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}