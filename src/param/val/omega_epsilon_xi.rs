use crate::param::{EllipticFilterConf, EllipticFilterParam, EllipticFilterParamBase, FilterFloat, FilterParam, Param};

pub type OmegaEpsilonXiDyn<F> = OmegaEpsilonXi<F>;
pub type OmegaEpsilonXiFirstOrder<F> = OmegaEpsilonXi<F, 1>;
pub type OmegaEpsilonXiSecondOrder<F> = OmegaEpsilonXi<F, 2>;
pub type OmegaEpsilonXiThirdOrder<F> = OmegaEpsilonXi<F, 3>;

#[derive(Clone, Copy, Debug)]
pub struct OmegaEpsilonXi<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: F,
    pub epsilon: F,
    pub xi: F
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
    [(); Self::ORDER]:
{
    type Conf = C;

    fn omega_epsilon_xi(&self) -> OmegaEpsilonXi<Self::F, {Self::ORDER}>
    {
        let OmegaEpsilonXi {omega, epsilon, xi} = **self;
        OmegaEpsilonXi {
            omega,
            epsilon,
            xi
        }
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