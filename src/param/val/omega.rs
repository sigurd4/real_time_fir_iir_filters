use crate::{change::Change, param::{ButterworthFilterConf, ButterworthFilterParam, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, FirstOrderFilterParamBase, SecondOrderFilterParamBase, ThirdOrderFilterParamBase}};

use super::OmegaEpsilonCheb1Dyn;

pub type OmegaDyn<F> = Omega<F>;
pub type OmegaFirstOrder<F> = Omega<F, 1>;
pub type OmegaSecondOrder<F> = Omega<F, 2>;
pub type OmegaThirdOrder<F> = Omega<F, 3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Omega<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: F
}
impl<F, const ORDER: usize> Change for Omega<F, ORDER>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.omega.change(to.omega, change);
    }
}
impl<F, const ORDER: usize> FilterParam for Omega<F, ORDER>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}

impl<F, C> FirstOrderFilterParamBase<C> for OmegaFirstOrder<F>
where
    F: FilterFloat,
    C: ButterworthFilterConf<1>
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderFilterParamBase<C> for OmegaSecondOrder<F>
where
    F: FilterFloat,
    C: ButterworthFilterConf<2>
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderFilterParamBase<C> for OmegaThirdOrder<F>
where
    F: FilterFloat,
    C: ButterworthFilterConf<3>
{
    type ImplBase = Self;
}
impl<F, C, const ORDER: usize> EllipticFilterParamBase<C> for Omega<F, ORDER>
where
    F: FilterFloat,
    C: ButterworthFilterConf<ORDER>
{
    type ImplBase = OmegaEpsilonCheb1Dyn<F>;
}
impl<F, C, const ORDER: usize> ChebyshevFilterParamBase<C> for Omega<F, ORDER>
where
    F: FilterFloat,
    C: ButterworthFilterConf<ORDER>
{
    const TYPE: bool = false;

    type ImplBase = OmegaDyn<F>;
}
impl<F, C, const ORDER: usize> ButterworthFilterParam<C> for Omega<F, ORDER>
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
        *self
    }
}