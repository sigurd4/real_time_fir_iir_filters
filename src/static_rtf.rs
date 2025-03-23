use crate::{internals::{ainternals, binternals, rtfinternals}, param::Param, rtf::RtfBase};

pub trait StaticRtfBase: RtfBase + Sized + 'static
{
    type Param;

    const ORDER: usize;
    const SOS_STAGES: usize;
    const O_BUFFERS: usize;
    const SOS_BUFFERS: usize;
    
    fn from_param(param: Self::Param) -> Self;
    fn get_param(&self) -> &Self::Param;
    fn get_param_mut(&mut self) -> &mut Self::Param;
    fn into_param(self) -> Self::Param;
    
    fn get_internals(&self) -> (&rtfinternals!(Self::F, Self::OUTPUTS, Self::O_BUFFERS, Self::SOS_BUFFERS, Self::SOS_STAGES, Self::ORDER, Self::IS_IIR), &Param<Self::Param>)
    where
        [(); Self::OUTPUTS]:;
    fn get_internals_mut(&mut self) -> (&mut rtfinternals!(Self::F, Self::OUTPUTS, Self::O_BUFFERS, Self::SOS_BUFFERS, Self::SOS_STAGES, Self::ORDER, Self::IS_IIR), &mut Param<Self::Param>);
    
    fn make_coeffs(param: &Param<Self::Param>, rate: Self::F) -> (
        binternals!(Self::F, Self::OUTPUTS, Self::O_BUFFERS, Self::SOS_BUFFERS, Self::SOS_STAGES, Self::ORDER),
        [ainternals!(Self::F, Self::O_BUFFERS, Self::SOS_BUFFERS, Self::SOS_STAGES, Self::ORDER); Self::IS_IIR as usize]
    );
}

mod private
{
    use super::StaticRtfBase;

    pub trait _StaticRtf: StaticRtfBase
    {
        fn _update_internals(&mut self, rate: Self::F);
    }

    impl<T> _StaticRtf for T
    where
        T: StaticRtfBase,
        [(); Self::ORDER + 1]:,
        [(); Self::IS_IIR as usize]:,
        [(); Self::SOS_STAGES*(Self::SOS_STAGES >= 1) as usize - (Self::SOS_STAGES >= 1) as usize]:,
        [(); (Self::SOS_STAGES >= 1) as usize]:,
        [(); 0 - Self::OUTPUTS % Self::O_BUFFERS]:,
        [(); 0 - Self::O_BUFFERS % Self::SOS_BUFFERS]:
    {
        fn _update_internals(&mut self, rate: Self::F)
        {
            crate::internals::update(self, rate)
        }
    }
}

pub trait StaticRtf: private::_StaticRtf
{
    const O_BUF_CHUNK: usize;
    const SOS_BUF_CHUNK: usize;

    const REM_SOS_LEN: usize;
    const LAST_SOS_LEN: usize;

    fn update_internals(&mut self, rate: Self::F);
}

impl<T> StaticRtf for T
where
    T: private::_StaticRtf
{
    const O_BUF_CHUNK: usize = Self::OUTPUTS/Self::O_BUFFERS;
    const SOS_BUF_CHUNK: usize = Self::O_BUFFERS/Self::SOS_BUFFERS;

    const REM_SOS_LEN: usize = Self::SOS_STAGES*(Self::SOS_STAGES >= 1) as usize - (Self::SOS_STAGES >= 1) as usize;
    const LAST_SOS_LEN: usize = (Self::SOS_STAGES >= 1) as usize;

    fn update_internals(&mut self, rate: Self::F)
    {
        self._update_internals(rate)
    }
}