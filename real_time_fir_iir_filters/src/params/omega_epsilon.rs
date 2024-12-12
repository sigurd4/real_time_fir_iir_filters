use crate::{conf::All, param::{Chebyshev1FilterParam, Chebyshev2FilterParam, ChebyshevFilterConf, ChebyshevFilterParamBase, ChebyshevType, EllipticFilterParamBase, FilterFloat, FilterParam, FilterParamFirstOrder, FilterParamSecondOrder, Param, Parameterization}, util::same::NotSame};

pub type OmegaEpsilonDyn<F, const TYPE: ChebyshevType> = OmegaEpsilon<F, TYPE>;
pub type OmegaEpsilonFirstOrder<F, const TYPE: ChebyshevType> = OmegaEpsilon<F, TYPE, 1>;
pub type OmegaEpsilonSecondOrder<F, const TYPE: ChebyshevType> = OmegaEpsilon<F, TYPE, 2>;
pub type OmegaEpsilonThirdOrder<F, const TYPE: ChebyshevType> = OmegaEpsilon<F, TYPE, 3>;

pub type OmegaEpsilonCheb1<F, const ORDER: usize = 0> = OmegaEpsilon<F, {ChebyshevType::Type1}, ORDER>;
pub type OmegaEpsilonCheb1Dyn<F> = OmegaEpsilonCheb1<F>;
pub type OmegaEpsilonCheb1FirstOrder<F> = OmegaEpsilonCheb1<F, 1>;
pub type OmegaEpsilonCheb1SecondOrder<F> = OmegaEpsilonCheb1<F, 2>;
pub type OmegaEpsilonCheb1ThirdOrder<F> = OmegaEpsilonCheb1<F, 3>;

pub type OmegaEpsilonCheb2<F, const ORDER: usize = 0> = OmegaEpsilon<F, {ChebyshevType::Type2}, ORDER>;
pub type OmegaEpsilonCheb2Dyn<F> = OmegaEpsilonCheb2<F>;
pub type OmegaEpsilonCheb2FirstOrder<F> = OmegaEpsilonCheb2<F, 1>;
pub type OmegaEpsilonCheb2SecondOrder<F> = OmegaEpsilonCheb2<F, 2>;
pub type OmegaEpsilonCheb2ThirdOrder<F> = OmegaEpsilonCheb2<F, 3>;


pub struct OmegaEpsilon<F, const TYPE: ChebyshevType, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: Param<F>,
    pub epsilon: Param<F>
}
impl<F, const TYPE: ChebyshevType, const ORDER: usize> OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat
{
    pub const fn new(omega: F, epsilon: F) -> Self
    {
        Self {
            omega: Param::new(omega),
            epsilon: Param::new(epsilon)
        }
    }
}
impl<F, const TYPE: ChebyshevType, const ORDER: usize> Parameterization for OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat
{
    fn is_unchanged(&self) -> bool
    {
        self.omega.is_unchanged()
            && self.epsilon.is_unchanged()
    }
    fn set_unchanged(&mut self)
    {
        self.omega.set_unchanged();
        self.epsilon.set_unchanged();
    }
}
impl<F, const TYPE: ChebyshevType, const ORDER: usize> FilterParam for OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}
impl<F, const TYPE: ChebyshevType> FilterParamFirstOrder for OmegaEpsilonFirstOrder<F, TYPE>
where
    F: FilterFloat
{
    
}
impl<F, const TYPE: ChebyshevType> FilterParamSecondOrder for OmegaEpsilonSecondOrder<F, TYPE>
where
    F: FilterFloat
{
    
}
impl<F, const TYPE: ChebyshevType, const ORDER: usize, C> EllipticFilterParamBase<C> for OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat,
    C: ChebyshevFilterConf
{
    type ImplBase = OmegaEpsilonDyn<F, TYPE>;
}
impl<F, const TYPE: ChebyshevType, const ORDER: usize, C> ChebyshevFilterParamBase<C> for OmegaEpsilon<F, TYPE, ORDER>
where
    F: FilterFloat,
    C: ChebyshevFilterConf
{
    type ImplBase = Self;
}
impl<F, const ORDER: usize, C> Chebyshev1FilterParam<C, Self> for OmegaEpsilonCheb1<F, ORDER>
where
    F: FilterFloat,
    C: ChebyshevFilterConf
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
}
impl<F, const ORDER: usize, C> Chebyshev2FilterParam<C, Self> for OmegaEpsilonCheb2<F, ORDER>
where
    F: FilterFloat,
    C: ChebyshevFilterConf
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
}
impl<P, const ORDER: usize> From<P> for OmegaEpsilonCheb1<P::F, ORDER>
where
    P: Chebyshev1FilterParam<All, Conf = All> + NotSame<OmegaEpsilonCheb1<P::F>>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilon::new(value.omega(), value.epsilon())
    }
}
impl<P, const ORDER: usize> From<P> for OmegaEpsilonCheb2<P::F, ORDER>
where
    P: Chebyshev2FilterParam<All, Conf = All> + NotSame<OmegaEpsilonCheb2<P::F>>
{
    fn from(value: P) -> Self
    {
        OmegaEpsilon::new(value.omega(), value.epsilon())
    }
}