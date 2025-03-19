use crate::{conf::{All, Conf}, param::FilterParam};

pub trait FirstOrderAllPassFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: FirstOrderAllPassFilterParamBase<All, ImplBase = Self::ImplBase>;
}