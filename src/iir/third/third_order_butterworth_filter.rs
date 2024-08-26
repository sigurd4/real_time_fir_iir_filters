use crate::{f, real_time_fir_iir_filters, iir::{first::Omega, second::ButterworthFilterParam}};

// TODO: Do it in SOS
crate::def_rtf!(
    ThirdOrderButterworthFilter
    {
        type Param: ButterworthFilterParam = Omega;

        const OUTPUTS: usize = 4;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 3;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
            let rate3 = rate2*rate;
            
            let omega = param.omega();
            let omega2 = omega*omega;
            let omega3 = omega2*omega;
            (
                ([], [
                    [
                        omega3,
                        f!(3.0)*omega3,
                        f!(3.0)*omega3,
                        omega3
                    ],
                    [
                        f!(2.0)*rate*omega2,
                        f!(2.0)*rate*omega2,
                        f!(-2.0)*rate*omega2,
                        f!(-2.0)*rate*omega2
                    ],
                    [
                        f!(4.0)*rate2*omega,
                        f!(-4.0)*rate2*omega,
                        f!(-4.0)*rate2*omega,
                        f!(4.0)*rate2*omega
                    ],
                    [
                        f!(8.0)*rate3,
                        f!(-24.0)*rate3,
                        f!(24.0)*rate3,
                        f!(-8.0)*rate3
                    ]
                ]),
                [([], [[
                    f!(8.0)*rate3 + f!(8.0)*rate2*omega + f!(4.0)*rate*omega2 + omega3,
                    f!(-24.0)*rate3 - f!(8.0)*rate2*omega + f!(4.0)*rate*omega2 + f!(3.0)*omega3,
                    f!(24.0)*rate3 - f!(8.0)*rate2*omega - f!(4.0)*rate*omega2 + f!(3.0)*omega3,
                    f!(-8.0)*rate3 + f!(8.0)*rate2*omega - f!(4.0)*rate*omega2 + omega3
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

    use super::ThirdOrderButterworthFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderButterworthFilter::new(Omega::new(10000.0*TAU));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}