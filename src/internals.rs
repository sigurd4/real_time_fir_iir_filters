use core::mem::MaybeUninit;

use bytemuck::Pod;
use num::Float;

use crate::{param::Parameterization, static_rtf::{StaticRtf, StaticRtfBase}};

pub macro winternals {
    ($f:ty, $outputs:expr, $buffered_outputs:expr, $sos:expr, $order:expr) => {
        ([[[$f; 2]; {$outputs*$buffered_outputs as usize}]; $sos], [[$f; $order]; {$outputs*$buffered_outputs as usize + !$buffered_outputs as usize}])
    },
    ($rtf:ty) => {
        winternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::BUFFERED_OUTPUTS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro binternals {
    ($f:ty, $outputs:expr, $buffered_outputs:expr, $sos:expr, $order:expr) => {
        ([[[$f; 3]; {$outputs*$buffered_outputs as usize}]; $sos], [[$f; $order + 1]; $outputs])
    },
    ($rtf:ty) => {
        binternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::BUFFERED_OUTPUTS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro ainternals {
    ($f:ty, $outputs:expr, $buffered_outputs:expr, $sos:expr, $order:expr) => {
        ([[[$f; 3]; {$outputs*$buffered_outputs as usize}]; $sos], [[$f; $order + 1]; {$outputs*$buffered_outputs as usize + !$buffered_outputs as usize}])
    },
    ($rtf:ty) => {
        ainternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::BUFFERED_OUTPUTS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro rtfinternals {
    ($f:ty, $outputs:expr, $buffered_outputs:expr, $sos:expr, $order:expr, $is_iir:expr) => {
        RtfInternals<$f, winternals!($f, $outputs, $buffered_outputs, $sos, $order), binternals!($f, $outputs, $buffered_outputs, $sos, $order), [ainternals!($f, $outputs, $buffered_outputs, $sos, $order); $is_iir as usize]>
    },
    ($rtf:ty) => {
        rtfinternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::BUFFERED_OUTPUTS, <$rtf>::SOS_STAGES, <$rtf>::ORDER, <$rtf>::IS_IIR)
    }
}

pub type WInternals<F, const OUTPUTS: usize, const BUFFERED_OUTPUTS: bool, const SOS_STAGES: usize, const ORDER: usize> = winternals!(F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER);
pub type BInternals<F, const OUTPUTS: usize, const BUFFERED_OUTPUTS: bool, const SOS_STAGES: usize, const ORDER: usize> = binternals!(F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER);
pub type AInternals<F, const OUTPUTS: usize, const BUFFERED_OUTPUTS: bool, const SOS_STAGES: usize, const ORDER: usize> = ainternals!(F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER);

pub type RtfInternalsGiven<F, const OUTPUTS: usize, const BUFFERED_OUTPUTS: bool, const SOS_STAGES: usize, const ORDER: usize, const IS_IIR: bool>
    = RtfInternals<F, WInternals<F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER>, BInternals<F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER>, [AInternals<F, OUTPUTS, BUFFERED_OUTPUTS, SOS_STAGES, ORDER>; IS_IIR as usize]>;

#[allow(type_alias_bounds)]
pub type WInternalsFor<Rtf: StaticRtfBase> = winternals!(Rtf);
#[allow(type_alias_bounds)]
pub type BInternalsFor<Rtf: StaticRtfBase> = binternals!(Rtf);
#[allow(type_alias_bounds)]
pub type AInternalsFor<Rtf: StaticRtfBase> = ainternals!(Rtf);

#[allow(type_alias_bounds)]
pub type RtfInternalsFor<Rtf: StaticRtfBase> = RtfInternals<Rtf::F, WInternalsFor<Rtf>, BInternalsFor<Rtf>, [AInternalsFor<Rtf>; Rtf::IS_IIR as usize]>;

#[derive(Clone, Copy, Debug)]
pub struct RtfInternals<F, W, B, A>
where
    F: Float + Pod
{
    pub w: W,
    pub b: B,
    pub a: A,
    pub(crate) rate: Option<F>
}

impl<F, W, B, A> RtfInternals<F, W, B, A>
where
    F: Float + Pod
{
    pub const fn new() -> Self
    {
        Self {
            w: unsafe {core::mem::zeroed()},
            b: unsafe {core::mem::zeroed()},
            a: unsafe {core::mem::zeroed()},
            rate: None
        }
    }
}

pub(crate) fn update<Rtf>(
    rtf: &mut Rtf,
    rate: Rtf::F,
)
where
    Rtf: StaticRtf,
    [(); Rtf::OUTPUTS*Rtf::BUFFERED_OUTPUTS as usize + !Rtf::BUFFERED_OUTPUTS as usize]:,
    [(); Rtf::OUTPUTS*Rtf::BUFFERED_OUTPUTS as usize]:,
    [(); Rtf::SOS_STAGES]:,
    [(); Rtf::ORDER]:,
    [(); Rtf::ORDER + 1]:,
    [(); Rtf::IS_IIR as usize]:
{
    let (internals, param) = rtf.get_internals_mut();
    if !param.is_unchanged_then_set() || internals.rate != Some(rate)
    {
        (internals.b, internals.a) = Rtf::make_coeffs(param, rate)
    }
    internals.rate = Some(rate)
}