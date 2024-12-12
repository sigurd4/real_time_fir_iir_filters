use crate::{internals::{ainternals, binternals, rtfinternals, RtfInternals}, private::NotSame, rtf::{Rtf, RtfBase}, static_rtf::{StaticRtf, StaticRtfBase}};

pub struct Inverse<T>
where
    T: Rtf + StaticRtf + NotSame<Self>,
    [(); T::OUTPUTS*true as usize]:,
    [(); T::OUTPUTS*true as usize + !true as usize]:,
    [(); T::SOS_STAGES]:,
    [(); T::ORDER]:,
    [(); T::ORDER + 1]:
{
    pub param: T::Param,
    pub internals: rtfinternals!(T::F, T::OUTPUTS, true, T::SOS_STAGES, T::ORDER, true)
}

impl<T> Inverse<T>
where
    T: Rtf + StaticRtf + NotSame<Self>,
    [(); T::OUTPUTS*true as usize]:,
    [(); T::OUTPUTS*true as usize + !true as usize]:,
    [(); T::SOS_STAGES]:,
    [(); T::ORDER]:,
    [(); T::ORDER + 1]:
{
    pub fn new(param: T::Param) -> Self
    {
        Self {
            param,
            internals: RtfInternals::new()
        }
    }
}

impl<T> RtfBase for Inverse<T>
where
    T: Rtf + StaticRtf + NotSame<Self>,
    [(); T::OUTPUTS*true as usize]:,
    [(); T::OUTPUTS*true as usize + !true as usize]:,
    [(); T::SOS_STAGES]:,
    [(); T::ORDER]:,
    [(); T::ORDER + 1]:
{
    type F = T::F;

    const OUTPUTS: usize = T::OUTPUTS;
    const IS_IIR: bool = true;
}

impl<T> StaticRtfBase for Inverse<T>
where
    T: Rtf + StaticRtf + NotSame<Self>,
    [(); T::OUTPUTS*true as usize]:,
    [(); T::OUTPUTS*true as usize + !true as usize]:,
    [(); T::SOS_STAGES]:,
    [(); T::ORDER]:,
    [(); T::ORDER + 1]:
{
    type Param = T::Param;

    const O_BUFFERS: usize = T::OUTPUTS;
    const SOS_BUFFERS: usize = T::OUTPUTS;
    const ORDER: usize = T::ORDER;
    const SOS_STAGES: usize = T::SOS_STAGES;
    
    fn from_param(param: Self::Param) -> Self
    {
        Self::new(param)
    }
    fn get_param(&self) -> &Self::Param
    {
        &self.param
    }
    fn get_param_mut(&mut self) -> &mut Self::Param
    {
        &mut self.param
    }
    fn into_param(self) -> Self::Param
    {
        self.param
    }
    
    fn get_internals(&self) -> (&rtfinternals!(T::F, T::OUTPUTS, true, T::SOS_STAGES, T::ORDER, true), &Self::Param)
    {
        &self.internals
    }
    fn get_internals_mut(&mut self) -> (&mut rtfinternals!(T::F, T::OUTPUTS, true, T::SOS_STAGES, T::ORDER, true), &mut Self::Param)
    {
        &mut self.internals
    }
    
    fn make_coeffs(param: &Self::Param, rate: Self::F) -> (
        binternals!(Self::F, Self::OUTPUTS, Self::BUFFEREDOUTPUTS, Self::SOS_STAGES, Self::ORDER),
        [ainternals!(Self::F, Self::OUTPUTS, Self::BUFFERED_OUTPUTS, Self::SOS_STAGES, Self::ORDER); Self::IS_IIR as usize]
    )
    {
        let (a, b) = T::make_coeffs(param, rate);

        let b = b.into_iter()
            .next()
            .unwrap_or_else(|| ([[one_followed_by_zeros(); _]; _], [one_followed_by_zeros(); _]));

        (
            b,
            [a]
        )
    }
}

const fn one_followed_by_zeros<F, const N: usize>() -> [F; N]
{
    assert!(N > 0);
    let mut a = [F::zero(); N];
    a[0] = F::one();
    a
}