use core::marker::PhantomData;

use crate::{param::{EllipticFilterConf, ChebyshevFilterParam, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, Param}, util::same::Same};

#[derive(Copy, Clone, Debug)]
pub enum Chebyshev1 {}
#[derive(Copy, Clone, Debug)]
pub enum Chebyshev2 {}

mod private
{
    use core::fmt::Debug;

    use super::{Chebyshev1, Chebyshev2};

    pub trait ChebyshevType: Sized + Copy + Clone + Debug + 'static
    {
        
    }
    impl ChebyshevType for Chebyshev1
    {
        
    }
    impl ChebyshevType for Chebyshev2
    {
        
    }
}

pub trait ChebyshevType: private::ChebyshevType {}
impl<T> ChebyshevType for T where T: private::ChebyshevType {}

pub type OmegaEpsilonDyn<F, T> = OmegaEpsilon<F, T>;
pub type OmegaEpsilonFirstOrder<F, T> = OmegaEpsilon<F, T, 1>;
pub type OmegaEpsilonSecondOrder<F, T> = OmegaEpsilon<F, T, 2>;
pub type OmegaEpsilonThirdOrder<F, T> = OmegaEpsilon<F, T, 3>;

pub type OmegaEpsilonCheb1<F, const ORDER: usize = 0> = OmegaEpsilon<F, Chebyshev1, ORDER>;
pub type OmegaEpsilonCheb1Dyn<F> = OmegaEpsilonCheb1<F>;
pub type OmegaEpsilonCheb1FirstOrder<F> = OmegaEpsilonCheb1<F, 1>;
pub type OmegaEpsilonCheb1SecondOrder<F> = OmegaEpsilonCheb1<F, 2>;
pub type OmegaEpsilonCheb1ThirdOrder<F> = OmegaEpsilonCheb1<F, 3>;

pub type OmegaEpsilonCheb2<F, const ORDER: usize = 0> = OmegaEpsilon<F, Chebyshev2, ORDER>;
pub type OmegaEpsilonCheb2Dyn<F> = OmegaEpsilonCheb2<F>;
pub type OmegaEpsilonCheb2FirstOrder<F> = OmegaEpsilonCheb2<F, 1>;
pub type OmegaEpsilonCheb2SecondOrder<F> = OmegaEpsilonCheb2<F, 2>;
pub type OmegaEpsilonCheb2ThirdOrder<F> = OmegaEpsilonCheb2<F, 3>;

#[derive(Clone, Copy, Debug)]
pub struct OmegaEpsilon<F, T, const ORDER: usize = 0>
where
    F: FilterFloat,
    T: ChebyshevType
{
    pub omega: F,
    pub epsilon: F,
    pub _m: PhantomData<T>
}
impl<F, T, const ORDER: usize> FilterParam for Param<OmegaEpsilon<F, T, ORDER>>
where
    T: ChebyshevType,
    F: FilterFloat
{
    const ORDER: usize = ORDER;

    type F = F;
}
impl<F, T, const ORDER: usize, C> EllipticFilterParamBase<C> for Param<OmegaEpsilon<F, T, ORDER>>
where
    T: ChebyshevType,
    F: FilterFloat,
    C: EllipticFilterConf
{
    type ImplBase = Param<OmegaEpsilonDyn<F, T>>;
}
impl<F, T, const ORDER: usize, C> ChebyshevFilterParamBase<C> for Param<OmegaEpsilon<F, T, ORDER>>
where
    T: ChebyshevType,
    F: FilterFloat,
    C: EllipticFilterConf
{
    type Type = T;

    type ImplBase = Self;
}
impl<F, T, const ORDER: usize, C> ChebyshevFilterParam<C, Self> for Param<OmegaEpsilon<F, T, ORDER>>
where
    T: ChebyshevType,
    F: FilterFloat,
    C: EllipticFilterConf,
    //OmegaEpsilon<F, T, ORDER>: Same<OmegaEpsilon<F, T, {Self::ORDER}>>
{
    type Conf = C;

    type OmegaEpsilon = OmegaEpsilon<F, T, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); Self::ORDER]:
    {
        **self
    }
}