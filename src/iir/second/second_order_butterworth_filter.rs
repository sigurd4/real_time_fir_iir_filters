use core::f64::consts::FRAC_1_SQRT_2;

use bytemuck::Pod;
use num::Float;

use crate::{f, iir::first::{FirstOrderFilterParam, Omega}};

pub trait ButterworthFilterParam: FirstOrderFilterParam
{
    
}
impl<F> ButterworthFilterParam for Omega<F>
where
    F: Float + Pod
{
    
}

crate::def_rtf!(
    SecondOrderButterworthFilter
    {
        type Param: ButterworthFilterParam = Omega;

        const OUTPUTS: usize = 3;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let omega = param.omega();
            let omega2 = omega*omega;
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
                        rate*omega*f!(-2.0),
                    ],
                    [
                        rate2*f!(4.0),
                        rate2*f!(-8.0),
                        rate2*f!(4.0)
                    ]
                ]),
                [([], [[
                    rate2*f!(4.0) + rate*f!(FRAC_1_SQRT_2)*omega*f!(4.0) + omega2,
                    omega2*f!(2.0) - rate2*f!(8.0),
                    rate2*f!(4.0) - rate*f!(FRAC_1_SQRT_2)*omega*f!(4.0) + omega2
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use crate::iir::first::Omega;

    use super::SecondOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderButterworthFilter::new(Omega::new(10000.0*TAU));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}