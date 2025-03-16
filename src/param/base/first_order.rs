use crate::{conf::{All, Conf}, param::FilterParamFirstOrder};

pub trait FirstOrderFilterParamBase<C>: FilterParamFirstOrder
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: FirstOrderFilterParamBase<All, ImplBase = Self::ImplBase>;
}