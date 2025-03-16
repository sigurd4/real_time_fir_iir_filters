use crate::{conf::{All, Conf}, param::FilterParam};

pub trait SecondOrderRLCFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: SecondOrderRLCFilterParamBase<All, ImplBase = Self::ImplBase>;
}