use crate::{conf::{All, Conf}, param::FilterParam};

pub trait SecondOrderFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: SecondOrderFilterParamBase<All, ImplBase = Self::ImplBase>;
}