use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{param::FilterFloat, static_rtf::{StaticRtf, StaticRtfBase}, serde::{MaybeSerialize, DeserializeOrZeroed}};

pub macro winternals {
    ($f:ty, $o_buffers:expr, $sos_buffers:expr, $sos:expr, $order:expr) => {
        ([[[$f; 2]; $sos_buffers]; $sos], [[$f; $order]; $o_buffers])
    },
    ($rtf:ty) => {
        winternals!(<$rtf>::F, <$rtf>::O_BUFFERS, <$rtf>::SOS_BUFFERS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro binternals {
    ($f:ty, $outputs:expr, $o_buffers:expr, $sos_buffers:expr, $sos:expr, $order:expr) => {
        ([[[$f; 3]; $sos_buffers]; $sos*($sos >= 1) as usize - ($sos >= 1) as usize], [[[$f; 3]; $o_buffers]; ($sos >= 1) as usize], [[$f; $order + 1]; $outputs])
    },
    ($rtf:ty) => {
        binternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::O_BUFFERS, <$rtf>::SOS_BUFFERS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro ainternals {
    ($f:ty, $o_buffers:expr, $sos_buffers:expr, $sos:expr, $order:expr) => {
        ([[[$f; 3]; $sos_buffers]; $sos], [[$f; $order + 1]; $o_buffers])
    },
    ($rtf:ty) => {
        ainternals!(<$rtf>::F, <$rtf>::O_BUFFERS, <$rtf>::SOS_BUFFERS, <$rtf>::SOS_STAGES, <$rtf>::ORDER)
    }
}
pub macro rtfinternals {
    ($f:ty, $outputs:expr, $o_buffers:expr, $sos_buffers:expr, $sos:expr, $order:expr, $is_iir:expr) => {
        RtfInternals<$f,
            winternals!($f, $o_buffers, $sos_buffers, $sos, $order),
            binternals!($f, $outputs, $o_buffers, $sos_buffers, $sos, $order),
            [ainternals!($f, $o_buffers, $sos_buffers, $sos, $order); $is_iir as usize]
        >
    },
    ($rtf:ty) => {
        rtfinternals!(<$rtf>::F, <$rtf>::OUTPUTS, <$rtf>::O_BUFFERS, <$rtf>::SOS_BUFFERS, <$rtf>::SOS_STAGES, <$rtf>::ORDER, <$rtf>::IS_IIR)
    }
}

pub type WInternals<F, const O_BUFFERS: usize, const SOS_BUFFERS: usize, const SOS_STAGES: usize, const ORDER: usize> = winternals!(F, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER);
pub type BInternals<F, const OUTPUTS: usize, const O_BUFFERS: usize, const SOS_BUFFERS: usize, const SOS_STAGES: usize, const ORDER: usize> = binternals!(F, OUTPUTS, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER);
pub type AInternals<F, const O_BUFFERS: usize, const SOS_BUFFERS: usize, const SOS_STAGES: usize, const ORDER: usize> = ainternals!(F, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER);

pub type RtfInternalsGiven<F, const OUTPUTS: usize, const O_BUFFERS: usize, const SOS_BUFFERS: usize, const SOS_STAGES: usize, const ORDER: usize, const IS_IIR: bool>
    = RtfInternals<F,
        WInternals<F, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER>,
        BInternals<F, OUTPUTS, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER>,
        [AInternals<F, O_BUFFERS, SOS_BUFFERS, SOS_STAGES, ORDER>; IS_IIR as usize]
    >;

#[allow(type_alias_bounds)]
pub type WInternalsFor<Rtf: StaticRtfBase> = winternals!(Rtf);
#[allow(type_alias_bounds)]
pub type BInternalsFor<Rtf: StaticRtfBase> = binternals!(Rtf);
#[allow(type_alias_bounds)]
pub type AInternalsFor<Rtf: StaticRtfBase> = ainternals!(Rtf);

#[allow(type_alias_bounds)]
pub type RtfInternalsFor<Rtf: StaticRtfBase> = RtfInternals<Rtf::F, WInternalsFor<Rtf>, BInternalsFor<Rtf>, [AInternalsFor<Rtf>; Rtf::IS_IIR as usize]>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RtfInternals<F, W, B, A>
where
    F: FilterFloat
{
    pub w: W,
    pub b: B,
    pub a: A,
    pub(crate) rate: Option<F>
}

impl<F, W, B, A> Serialize for RtfInternals<F, W, B, A>
where
    F: FilterFloat
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.w.maybe_serialize(serializer)
    }
}
impl<'de, F, W, B, A> Deserialize<'de> for RtfInternals<F, W, B, A>
where
    F: FilterFloat
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(Self {
            w: W::deserialize_or_zeroed(deserializer)?,
            ..Default::default()
        })
    }
}

impl<F, W, B, A> RtfInternals<F, W, B, A>
where
    F: FilterFloat
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

impl<F, W, B, A> Default for RtfInternals<F, W, B, A>
where
    F: FilterFloat
{
    fn default() -> Self
    {
        Self::new()
    }
}

pub(crate) fn update<Rtf>(
    rtf: &mut Rtf,
    rate: Rtf::F,
)
where
    Rtf: StaticRtf,
    [(); Rtf::ORDER + 1]:,
    [(); Rtf::IS_IIR as usize]:,
    [(); Rtf::SOS_STAGES*(Rtf::SOS_STAGES >= 1) as usize - (Rtf::SOS_STAGES >= 1) as usize]:,
    [(); (Rtf::SOS_STAGES >= 1) as usize]:,
    [(); 0 - Rtf::OUTPUTS % Rtf::O_BUFFERS]:,
    [(); 0 - Rtf::O_BUFFERS % Rtf::SOS_BUFFERS]:
{
    let (internals, param) = rtf.get_internals_mut();
    if !param.is_unchanged_then_set() || internals.rate != Some(rate)
    {
        (internals.b, internals.a) = Rtf::make_coeffs(param, rate)
    }
    internals.rate = Some(rate)
}