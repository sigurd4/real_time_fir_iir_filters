use crate::conf::{All, Conf};

use super::EllipticFilterParamBase;

pub trait ChebyshevFilterParamBase<C>: EllipticFilterParamBase<C>
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: ChebyshevFilterParamBase<All, ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase>;
}