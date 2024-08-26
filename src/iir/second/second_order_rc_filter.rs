use bytemuck::Pod;
use num::Float;

use crate::{f, real_time_fir_iir_filters, param::FilterParam, private::NotSame};

pub trait SecondOrderRCFilterParam: FilterParam
{
    fn r1(&self) -> Self::F;
    fn c1(&self) -> Self::F;
    fn r2(&self) -> Self::F;
    fn c2(&self) -> Self::F;
}

crate::def_param!(
    RC2<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for RC2<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> SecondOrderRCFilterParam for RC2<F>
where
    F: Float + Pod
{
    fn r1(&self) -> Self::F
    {
        *self.r1
    }
    fn c1(&self) -> Self::F
    {
        *self.r1
    }
    fn r2(&self) -> Self::F
    {
        *self.r1
    }
    fn c2(&self) -> Self::F
    {
        *self.r1
    }
}

crate::def_rtf!(
    {
        /// # Configurations
        /// ```
        /// 0) LOW-PASS:
        ///     X-[R1]-o-[R2]-Y
        ///            |      |
        ///           [C1]   [C2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 1) BAND-PASS 1:
        ///     X-[C1]-o-[R2]-Y
        ///            |      |
        ///           [R1]   [C2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 2) BAND-PASS 2
        ///     X-[R1]-o-[C2]-Y
        ///            |      |
        ///           [C1]   [R2]
        ///            |      |
        ///           GND    GND
        /// 
        /// 3) HIGH-PASS
        ///     X-[C1]-o-[C2]-Y
        ///            |      |
        ///           [R1]   [R2]
        ///            |      |
        ///           GND    GND
        /// ```
    }
    SecondOrderRCFilter
    {
        type Param: SecondOrderRCFilterParam = RC2;

        const OUTPUTS: usize = 4;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 2;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            let rate2 = rate*rate;
    
            let r1 = param.r1();
            let r2 = param.r2();
            let c1 = param.c1();
            let c2 = param.c2();
            (
                ([], [
                    [
                        f!(1.0),
                        f!(2.0),
                        f!(1.0),
                    ],
                    [
                        c1*r1*rate*f!(2.0),
                        f!(0.0),
                        c1*r1*rate*f!(-2.0),
                    ],
                    [
                        c2*r2*rate*f!(2.0),
                        f!(0.0),
                        c2*r2*rate*f!(-2.0),
                    ],
                    [
                        c1*c2*r1*r2*rate2*f!(4.0),
                        c1*c2*r1*r2*rate2*f!(-8.0),
                        c1*c2*r1*r2*rate2*f!(4.0),
                    ],
                ]),
                [([], [[
                    c1*r1*rate*(f!(2.0) + f!(4.0)*c2*r2*rate) + f!(1.0) + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate,
                    f!(-8.0)*c1*c2*r1*r2*rate2 + f!(2.0),
                    c1*r1*rate*(f!(-2.0) + f!(4.0)*c2*r2*rate) + f!(1.0) - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate,
                ]])]
            )
        }
    }
);
impl<P> From<P> for RC2<P::F>
where
    P: SecondOrderRCFilterParam + NotSame<RC2<P::F>>
{
    fn from(value: P) -> Self
    {
        RC2::new(value.r1(), value.c1(), value.r2(), value.c2())
    }
}

#[cfg(test)]
mod test
{
    use super::{SecondOrderRCFilter, RC2};

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderRCFilter::new(RC2::new(390e3, 100e-9, 4.7e3, 47e-12));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}