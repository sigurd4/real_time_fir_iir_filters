use crate::{conf::{All, Conf}, param::FilterParam};

pub trait EllipticFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: EllipticFilterParamBase<All, ImplBase = Self::ImplBase>;
}