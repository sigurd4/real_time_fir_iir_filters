use crate::{conf::{All, Conf}, param::ChebyshevType};

use super::EllipticFilterParamBase;

pub trait ChebyshevFilterParamBase<C>: EllipticFilterParamBase<C>
where
    C: Conf
{
    const TYPE: ChebyshevType;

    /// If in doubt, set this to [Self]
    type ImplBase: ChebyshevFilterParamBase<All, ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase>;
}