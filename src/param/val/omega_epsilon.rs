use crate::{change::Change, param::{ChebyshevFilterParam, ChebyshevFilterParamBase, EllipticFilterConf, EllipticFilterParamBase, FilterFloat, FilterParam, Param}};

pub type OmegaEpsilonDyn<F, const TYPE: bool> = OmegaEpsilon<F, TYPE>;
pub type OmegaEpsilonFirstOrder<F, const TYPE: bool> = OmegaEpsilon<F, TYPE, 1>;
pub type OmegaEpsilonSecondOrder<F, const TYPE: bool> = OmegaEpsilon<F, TYPE, 2>;
pub type OmegaEpsilonThirdOrder<F, const TYPE: bool> = OmegaEpsilon<F, TYPE, 3>;

pub type OmegaEpsilonCheb1<F, const ORDER: usize = 0> = OmegaEpsilon<F, false, ORDER>;
pub type OmegaEpsilonCheb1Dyn<F> = OmegaEpsilonCheb1<F>;
pub type OmegaEpsilonCheb1FirstOrder<F> = OmegaEpsilonCheb1<F, 1>;
pub type OmegaEpsilonCheb1SecondOrder<F> = OmegaEpsilonCheb1<F, 2>;
pub type OmegaEpsilonCheb1ThirdOrder<F> = OmegaEpsilonCheb1<F, 3>;

pub type OmegaEpsilonCheb2<F, const ORDER: usize = 0> = OmegaEpsilon<F, true, ORDER>;
pub type OmegaEpsilonCheb2Dyn<F> = OmegaEpsilonCheb2<F>;
pub type OmegaEpsilonCheb2FirstOrder<F> = OmegaEpsilonCheb2<F, 1>;
pub type OmegaEpsilonCheb2SecondOrder<F> = OmegaEpsilonCheb2<F, 2>;
pub type OmegaEpsilonCheb2ThirdOrder<F> = OmegaEpsilonCheb2<F, 3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct OmegaEpsilon<F, const TYPE: bool, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: F,
    pub epsilon: F
}
impl<F, const TYPE: bool, const ORDER: usize> Change for OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.omega.change(to.omega, change);
        self.epsilon.change(to.epsilon, change);
    }
}
impl<F, const TYPE: bool, const ORDER: usize> FilterParam for Param<OmegaEpsilon<F, TYPE, ORDER>>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}
impl<F, const TYPE: bool, const ORDER: usize, C> EllipticFilterParamBase<C> for Param<OmegaEpsilon<F, TYPE, ORDER>>
where
    F: FilterFloat,
    C: EllipticFilterConf
{
    type ImplBase = Param<OmegaEpsilonDyn<F, TYPE>>;
}
impl<F, const TYPE: bool, const ORDER: usize, C> ChebyshevFilterParamBase<C> for Param<OmegaEpsilon<F, TYPE, ORDER>>
where
    F: FilterFloat,
    C: EllipticFilterConf
{
    const TYPE: bool = TYPE;

    type ImplBase = Self;
}
impl<F, const TYPE: bool, const ORDER: usize, C> ChebyshevFilterParam<C, Self> for Param<OmegaEpsilon<F, TYPE, ORDER>>
where
    F: FilterFloat,
    C: EllipticFilterConf,
    //OmegaEpsilon<F, TYPE, ORDER>: Same<OmegaEpsilon<F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>>
{
    type Conf = C;

    type OmegaEpsilon = OmegaEpsilon<F, TYPE, ORDER>
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:
    {
        **self
    }
}