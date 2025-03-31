use serde::{Serialize, Deserialize};

use crate::param::{ButterworthFilterConf, ButterworthFilterParam, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, FirstOrderFilterParamBase, Param, SecondOrderFilterParamBase, ThirdOrderFilterParamBase};

use super::OmegaEpsilonCheb1Dyn;

pub type OmegaDyn<F> = Omega<F>;
pub type OmegaFirstOrder<F> = Omega<F, 1>;
pub type OmegaSecondOrder<F> = Omega<F, 2>;
pub type OmegaThirdOrder<F> = Omega<F, 3>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Omega<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: F
}

impl<F, const ORDER: usize> FilterParam for Param<Omega<F, ORDER>>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}

impl<F, C> FirstOrderFilterParamBase<C> for Param<OmegaFirstOrder<F>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<1>
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderFilterParamBase<C> for Param<OmegaSecondOrder<F>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<2>
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderFilterParamBase<C> for Param<OmegaThirdOrder<F>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<3>
{
    type ImplBase = Self;
}
impl<F, C, const ORDER: usize> EllipticFilterParamBase<C> for Param<Omega<F, ORDER>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<ORDER>
{
    type ImplBase = Param<OmegaEpsilonCheb1Dyn<F>>;
}
impl<F, C, const ORDER: usize> ChebyshevFilterParamBase<C> for Param<Omega<F, ORDER>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<ORDER>
{
    const TYPE: bool = false;

    type ImplBase = Param<OmegaDyn<F>>;
}
impl<F, C, const ORDER: usize> ButterworthFilterParam<C> for Param<Omega<F, ORDER>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<{Self::ORDER}> + ButterworthFilterConf<ORDER>
{
    type Conf = C
    where
        [(); Self::ORDER]:;
    type Omega = Omega<F, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega(&self) -> Self::Omega
    where
        [(); Self::ORDER]:
    {
        **self
    }
}