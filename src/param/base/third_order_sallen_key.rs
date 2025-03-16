use crate::{conf::{All, Conf}, param::FilterParam};

pub trait ThirdOrderSallenKeyFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: ThirdOrderSallenKeyFilterParamBase<All, ImplBase = Self::ImplBase>;
}