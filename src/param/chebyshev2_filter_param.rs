use crate::{conf::Conf, params::OmegaEpsilonCheb2Dyn, util::same::Same};

use super::{ChebyshevFilterConf, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterParam};


pub trait Chebyshev2FilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase = OmegaEpsilonCheb2Dyn<<Self as FilterParam>::F>>
where
    C: Conf
{
    type Conf: ChebyshevFilterConf;

    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
}