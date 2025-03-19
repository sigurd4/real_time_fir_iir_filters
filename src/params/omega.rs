use crate::param::{ButterworthFilterConf, ButterworthFilterParam, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, FirstOrderFilterParamBase, OmegaVal, Param, Parameterization, SecondOrderFilterParamBase, ThirdOrderFilterParamBase};

use super::OmegaEpsilonCheb1Dyn;

pub type OmegaDyn<F> = Omega<F>;
pub type OmegaFirstOrder<F> = Omega<F, 1>;
pub type OmegaSecondOrder<F> = Omega<F, 2>;
pub type OmegaThirdOrder<F> = Omega<F, 3>;

pub struct Omega<F, const ORDER: usize = 0>
where
    F: FilterFloat
{
    pub omega: Param<F>
}
impl<F, const ORDER: usize> Omega<F, ORDER>
where
    F: FilterFloat
{
    pub const fn new(omega: F) -> Self
    {
        Self {
            omega: Param::new(omega)
        }
    }
}
impl<F, const ORDER: usize> Parameterization for Omega<F, ORDER>
where
    F: FilterFloat
{
    fn is_unchanged(&self) -> bool
    {
        self.omega.is_unchanged()
    }
    fn set_unchanged(&mut self)
    {
        self.omega.set_unchanged()
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
    type ImplBase = OmegaDyn<F>;
}
impl<F, C, const ORDER: usize> ButterworthFilterParam<C> for Omega<F, ORDER>
where
    F: FilterFloat,
    C: ButterworthFilterConf<ORDER> + ButterworthFilterConf<{Self::ORDER}>,
    [(); Self::ORDER]:
{
    type Conf = C;

    fn omega(&self) -> OmegaVal<Self::F>
    {
        OmegaVal {
            omega: *self.omega
        }
    }
}

/*impl<P> From<P> for OmegaFirstOrder<P::F>
where
    P: FirstOrderFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        Omega::new(value.omega())
    }
}
impl<P, const ORDER: usize> From<P> for Omega<P::F, {ORDER}>
where
    P: ButterworthFilterParam<All, Conf = All> + FilterParam<ORDER = {ORDER}> + NotSame<Self>,
    Self: NotSame<OmegaFirstOrder<P::F>>
{
    fn from(value: P) -> Self
    {
        Omega::new(value.omega())
    }
}*/