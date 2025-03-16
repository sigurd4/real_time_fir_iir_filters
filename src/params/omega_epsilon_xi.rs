use crate::{conf::All, param::{EllipticFilterConf, EllipticFilterParam, EllipticFilterParamBase, FilterFloat, FilterParam, FilterParamFirstOrder, FilterParamSecondOrder, Param, Parameterization}, util::same::NotSame};

pub type OmegaEpsilonXiDyn<F> = OmegaEpsilonXi<F>;
pub type OmegaEpsilonXiFirstOrder<F> = OmegaEpsilonXi<F, 1>;
pub type OmegaEpsilonXiSecondOrder<F> = OmegaEpsilonXi<F, 2>;
pub type OmegaEpsilonXiThirdOrder<F> = OmegaEpsilonXi<F, 3>;

pub struct OmegaEpsilonXi<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: Param<F>,
    pub epsilon: Param<F>,
    pub xi: Param<F>
}
impl<F, const ORDER: usize> OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat
{
    pub const fn new(omega: F, epsilon: F, xi: F) -> Self
    {
        Self {
            omega: Param::new(omega),
            epsilon: Param::new(epsilon),
            xi: Param::new(xi)
        }
    }
}
impl<F, const ORDER: usize> Parameterization for OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat
{
    fn is_unchanged(&self) -> bool
    {
        self.omega.is_unchanged()
            && self.epsilon.is_unchanged()
            && self.xi.is_unchanged()
    }
    fn set_unchanged(&mut self)
    {
        self.omega.set_unchanged();
        self.epsilon.set_unchanged();
        self.xi.set_unchanged();
    }
}
impl<F, const ORDER: usize> FilterParam for OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}
impl<F> FilterParamFirstOrder for OmegaEpsilonXiFirstOrder<F>
where
    F: FilterFloat
{
    
}
impl<F> FilterParamSecondOrder for OmegaEpsilonXiSecondOrder<F>
where
    F: FilterFloat
{
    
}
impl<F, const ORDER: usize, C> EllipticFilterParamBase<C> for OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat,
    C: EllipticFilterConf
{
    type ImplBase = Self;
}
impl<F, const ORDER: usize, C> EllipticFilterParam<C, Self> for OmegaEpsilonXi<F, ORDER>
where
    F: FilterFloat,
    C: EllipticFilterConf
{
    type Conf = C;

    fn omega(&self) -> Self::F
    {
        *self.omega
    }
    fn epsilon(&self) -> Self::F
    {
        *self.epsilon
    }
    fn xi(&self) -> Self::F
    {
        *self.xi
    }
}
impl<P, const ORDER: usize> From<P> for OmegaEpsilonXi<P::F, ORDER>
where
    P: EllipticFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilonXi::new(value.omega(), value.epsilon(), value.xi())
    }
}