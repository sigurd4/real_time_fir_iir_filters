use core::f32::consts::FRAC_1_SQRT_2;

use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam};

use super::{ButterworthFilterParam, SecondOrderButterworthFilter};

pub trait SecondOrderFilterParam: FilterParam + Into<OmegaZeta<Self::F>>
{
    fn omega(&self) -> Self::F;
    fn zeta(&self) -> Self::F;
}
crate::def_param!(
    OmegaZeta<F> {
        omega: F,
        zeta: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for OmegaZeta<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> SecondOrderFilterParam for OmegaZeta<F>
where
    F: Float + Pod
{
    fn omega(&self) -> Self::F
    {
        *self.omega
    }
    fn zeta(&self) -> Self::F
    {
        *self.zeta
    }
}

crate::def_rtf!(
    SecondOrderFilter
    {
        type Param: SecondOrderFilterParam = OmegaZeta;

        const OUTPUTS: usize = 3;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let omega = param.omega();
            let omega2 = omega*omega;
            let zeta = param.zeta();
            let rate2 = rate*rate;
            (
                ([], [
                    [
                        omega2,
                        omega2*f!(2.0),
                        omega2
                    ],
                    [
                        rate*omega*f!(2.0),
                        f!(0.0; F),
                        rate*omega*f!(-2.0)
                    ],
                    [
                        rate2*f!(4.0),
                        rate2*f!(-8.0),
                        rate2*f!(4.0)
                    ]
                ]),
                [([], [[
                    rate2*f!(4.0) + rate*zeta*omega*f!(4.0) + omega2,
                    omega2*f!(2.0) - rate2*f!(8.0),
                    rate2*f!(4.0) - rate*zeta*omega*f!(4.0) + omega2
                ]])]
            )
        }
    }
);

impl<P> From<SecondOrderButterworthFilter<P::F, P>> for SecondOrderFilter<P::F>
where
    P: ButterworthFilterParam
{
    fn from(value: SecondOrderButterworthFilter<P::F, P>) -> Self
    {
        SecondOrderFilter::new(OmegaZeta::new(value.param.omega(), f!(FRAC_1_SQRT_2; P::F)))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::{OmegaZeta, SecondOrderFilter};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderFilter::new(OmegaZeta::new(10000.0*TAU, 0.05));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}