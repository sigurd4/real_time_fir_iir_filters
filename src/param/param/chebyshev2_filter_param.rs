use crate::{conf::Conf, param::{ChebyshevFilterParamBase, EllipticFilterParamBase, FilterParam}, params::OmegaEpsilonCheb2Dyn, util::same::Same};

use super::ChebyshevFilterConf;

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