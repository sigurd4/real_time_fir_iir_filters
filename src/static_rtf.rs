use crate::{internals::{ainternals, binternals, rtfinternals}, param::FilterParam, rtf::RtfBase};

pub trait StaticRtfBase: RtfBase + Sized + 'static
{
    type Param: FilterParam;

    const BUFFERED_OUTPUTS: bool;
    const SOS_STAGES: usize;
    const ORDER: usize;
    
    fn from_param(param: Self::Param) -> Self;
    fn get_param(&self) -> &Self::Param;
    fn get_param_mut(&mut self) -> &mut Self::Param;
    fn into_param(self) -> Self::Param;
    
    fn get_internals(&self) -> (&rtfinternals!(Self::F, Self::OUTPUTS, Self::BUFFERED_OUTPUTS, Self::SOS_STAGES, Self::ORDER, Self::IS_IIR), &Self::Param);
    fn get_internals_mut(&mut self) -> (&mut rtfinternals!(Self::F, Self::OUTPUTS, Self::BUFFERED_OUTPUTS, Self::SOS_STAGES, Self::ORDER, Self::IS_IIR), &mut Self::Param);
    
    fn make_coeffs(param: &Self::Param, rate: Self::F) -> (
        binternals!(Self::F, Self::OUTPUTS, Self::BUFFERED_OUTPUTS, Self::SOS_STAGES, Self::ORDER),
        [ainternals!(Self::F, Self::OUTPUTS, Self::BUFFERED_OUTPUTS, Self::SOS_STAGES, Self::ORDER); Self::IS_IIR as usize]
    );
}

pub trait StaticRtf: StaticRtfBase
{
    fn update_internals(&mut self, rate: Self::F);
}
impl<T> StaticRtf for T
where
    T: StaticRtfBase,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]:,
    [(); Self::SOS_STAGES]:,
    [(); Self::ORDER]:,
    [(); Self::ORDER + 1]:,
    [(); Self::IS_IIR as usize]:
{
    fn update_internals(&mut self, rate: Self::F)
    {
        crate::internals::update(self, rate)
    }
}