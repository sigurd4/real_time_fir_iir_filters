use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{winternals, binternals, ainternals, param::FilterFloat, rtf::StaticRtf, serde::{DeserializeOrZeroed, MaybeSerialize}, util::{ArrayMin1, ArrayMinus1, ArrayPlus1}};

pub type WInternals<F, const OUTPUT_BUFS: usize, const SOS_BUFS: usize, const SOS_STAGES: usize, const ORDER: usize> = winternals!(F, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER);
pub type BInternals<F, const OUTPUTS: usize, const OUTPUT_BUFS: usize, const SOS_BUFS: usize, const SOS_STAGES: usize, const ORDER: usize> = binternals!(F, OUTPUTS, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER);
pub type AInternals<F, const OUTPUT_BUFS: usize, const SOS_BUFS: usize, const SOS_STAGES: usize, const ORDER: usize> = ainternals!(F, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER);

pub type RtfInternalsGiven<F, const OUTPUTS: usize, const OUTPUT_BUFS: usize, const SOS_BUFS: usize, const SOS_STAGES: usize, const ORDER: usize, const IS_IIR: bool>
    = RtfInternals<F,
        WInternals<F, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER>,
        BInternals<F, OUTPUTS, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER>,
        [AInternals<F, OUTPUT_BUFS, SOS_BUFS, SOS_STAGES, ORDER>; IS_IIR as usize]
    >;

#[allow(type_alias_bounds)]
pub type WInternalsFor<Rtf: StaticRtf> = winternals!(Rtf);
#[allow(type_alias_bounds)]
pub type BInternalsFor<Rtf: StaticRtf> = binternals!(Rtf);
#[allow(type_alias_bounds)]
pub type AInternalsFor<Rtf: StaticRtf> = ainternals!(Rtf);

#[allow(type_alias_bounds)]
pub type RtfInternalsFor<Rtf: StaticRtf> = RtfInternals<Rtf::F, WInternalsFor<Rtf>, BInternalsFor<Rtf>, Rtf::IsIir<AInternalsFor<Rtf>>>;

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
    /*[(); Rtf::ORDER + 1]:,
    [(); Rtf::IS_IIR as usize]:,
    [(); Rtf::SOS_STAGES*(Rtf::SOS_STAGES >= 1) as usize - (Rtf::SOS_STAGES >= 1) as usize]:,
    [(); (Rtf::SOS_STAGES >= 1) as usize]:,
    [(); 0 - Rtf::OUTPUTS % Rtf::OUTPUT_BUFS]:,
    [(); 0 - Rtf::OUTPUT_BUFS % Rtf::SOS_BUFS]:*/
{
    let (internals, param) = rtf.get_internals_mut();
    if !param.is_unchanged_then_set() || internals.rate != Some(rate)
    {
        (internals.b, internals.a) = Rtf::make_coeffs(param, rate)
    }
    internals.rate = Some(rate)
}