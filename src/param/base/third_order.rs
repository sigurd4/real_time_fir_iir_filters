use crate::{conf::{All, Conf}, param::FilterParam};

pub trait ThirdOrderFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: ThirdOrderFilterParamBase<All, ImplBase = Self::ImplBase>;
}