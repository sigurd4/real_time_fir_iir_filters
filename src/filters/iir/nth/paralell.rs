use core::mem::MaybeUninit;

use crate::rtf::{Rtf, RtfBase, StaticRtf};

pub struct Paralell<A, B>(pub A, pub B)
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf;

impl<A, B> RtfBase for Paralell<A, B>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf
{
    type F = A::F;

    const OUTPUTS: usize = A::OUTPUTS + B::OUTPUTS;
    const IS_IIR: bool = A::IS_IIR || B::IS_IIR;

    fn on_filter_pre(&mut self, rate: Self::F)
    {
        self.0.on_filter_pre(rate);
        self.1.on_filter_pre(rate);
    }
}
impl<A, B> !StaticRtf for Paralell<A, B>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf
{

}
impl<A, B> Rtf for Paralell<A, B>
where
    A: Rtf<F = B::F>,
    B: Rtf + StaticRtf
{
    fn filter(&mut self, rate: Self::F, mut x: Self::F) -> [Self::F; Self::OUTPUTS]
    {
        let mut out = unsafe {MaybeUninit::uninit().assume_init()};
        let y = self.0.filter(rate, x);
        unsafe {
            core::ptr::copy_nonoverlapping(y.as_ptr(), out.as_mut_ptr(), A::OUTPUTS)
        }
        let y = self.1.filter(rate, x);
        unsafe {
            core::ptr::copy_nonoverlapping(y.as_ptr(), out.as_mut_ptr().add(A::OUTPUTS), B::OUTPUTS)
        }
        out
    }
    fn z_response(&mut self, rate: Self::F, z: num::Complex<Self::F>) -> [num::Complex<Self::F>; Self::OUTPUTS]
    {
        let mut out = unsafe {MaybeUninit::uninit().assume_init()};
        let y = self.0.z_response(rate, z);
        unsafe {
            core::ptr::copy_nonoverlapping(y.as_ptr(), out.as_mut_ptr(), A::OUTPUTS)
        }
        let y = self.1.z_response(rate, z);
        unsafe {
            core::ptr::copy_nonoverlapping(y.as_ptr(), out.as_mut_ptr().add(A::OUTPUTS), B::OUTPUTS)
        }
        out
    }
    fn reset(&mut self)
    {
        self.0.reset();
        self.1.reset();
    }
}