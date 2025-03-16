use crate::{conf::{All, Conf}, param::FilterParamSecondOrder};

pub trait SecondOrderFilterParamBase<C>: FilterParamSecondOrder
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: SecondOrderFilterParamBase<All, ImplBase = Self::ImplBase>;
}