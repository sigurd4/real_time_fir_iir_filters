use crate::{change::Change, param::{EllipticFilterConf, EllipticFilterParam, EllipticFilterParamBase, FilterFloat, FilterParam, Param}};

pub type OmegaEpsilonXiDyn<F> = OmegaEpsilonXi<F>;
pub type OmegaEpsilonXiFirstOrder<F> = OmegaEpsilonXi<F, 1>;
pub type OmegaEpsilonXiSecondOrder<F> = OmegaEpsilonXi<F, 2>;
pub type OmegaEpsilonXiThirdOrder<F> = OmegaEpsilonXi<F, 3>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct OmegaEpsilonXi<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: F,
    pub epsilon: F,
    pub xi: F
}
impl<F, const ORDER: usize> Change for OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.omega.change(to.omega, change);
        self.epsilon.change(to.epsilon, change);
        self.xi.change(to.xi, change);
    }
}
impl<F, const ORDER: usize> FilterParam for Param<OmegaEpsilonXi<F, ORDER>>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}
impl<F, const ORDER: usize, C> EllipticFilterParamBase<C> for Param<OmegaEpsilonXi<F, ORDER>>
where
    F: FilterFloat,
    C: EllipticFilterConf
{
    type ImplBase = Self;
}
impl<F, const ORDER: usize, C> EllipticFilterParam<C, Self> for Param<OmegaEpsilonXi<F, ORDER>>
where
    F: FilterFloat,
    C: EllipticFilterConf,
    //OmegaEpsilonXi<F, ORDER>: Same<OmegaEpsilonXi<F, {Self::ORDER}>>
{
    type Conf = C;
    type OmegaEpsilonXi = OmegaEpsilonXi<F, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega_epsilon_xi(&self) -> Self::OmegaEpsilonXi
    where
        [(); Self::ORDER]:
    {
        **self
    }
}
/*impl<P, const ORDER: usize> From<P> for OmegaEpsilonXi<P::F, ORDER>
where
    P: EllipticFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilonXi::new(value.omega(), value.epsilon(), value.xi())
    }
}*/