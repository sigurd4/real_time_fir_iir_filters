use crate::rtf::{Rtf, RtfBase, StaticRtf};

pub struct Cascade<A, B, const O: usize>(pub A, pub B)
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf,
    [(); A::OUTPUTS - O - 1]:;

impl<A, B, const O: usize> RtfBase for Cascade<A, B, O>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf,
    [(); A::OUTPUTS - O - 1]:
{
    type F = A::F;

    const OUTPUTS: usize = B::OUTPUTS;
    const IS_IIR: bool = A::IS_IIR || B::IS_IIR;

    fn on_filter_pre(&mut self, rate: Self::F)
    {
        self.0.on_filter_pre(rate);
        self.1.on_filter_pre(rate);
    }
}
impl<A, B, const O: usize> !StaticRtf for Cascade<A, B, O>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf,
    [(); A::OUTPUTS - O - 1]:
{

}
impl<A, B, const O: usize> Rtf for Cascade<A, B, O>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf,
    [(); A::OUTPUTS - O - 1]:
{
    fn filter(&mut self, rate: Self::F, mut x: Self::F) -> [Self::F; Self::OUTPUTS]
    {
        x = self.0.filter(rate, x)[O];
        self.1.filter(rate, x)
    }
    fn z_response(&mut self, rate: Self::F, z: num::Complex<Self::F>) -> [num::Complex<Self::F>; Self::OUTPUTS]
    {
        let h0 = self.0.z_response(rate, z)[O];
        self.1.z_response(rate, z)
            .map(|h1| h1*h0)
    }
    fn reset(&mut self)
    {
        self.0.reset();
        self.1.reset();
    }
}