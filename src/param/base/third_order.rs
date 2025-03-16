use crate::{conf::{All, Conf}, param::FilterParamThirdOrder};

pub trait ThirdOrderFilterParamBase<C>: FilterParamThirdOrder
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: ThirdOrderFilterParamBase<All, ImplBase = Self::ImplBase>;
}