use core::marker::ConstParamTy;

use num::One;

use super::{ButterworthFilterParam, EllipticFilterConf};

use crate::{conf::{All, Conf}, param::{ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, Omega, OmegaDyn, OmegaEpsilon, OmegaEpsilonCheb1, OmegaEpsilonCheb1FirstOrder, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb1ThirdOrder, OmegaEpsilonCheb2, OmegaEpsilonCheb2FirstOrder, OmegaEpsilonCheb2SecondOrder, OmegaEpsilonCheb2ThirdOrder, Param}, util::same::Same};

pub trait ChebyshevFilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase,
    ImplBase2 = <Self as EllipticFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase: Same<ImplBase2> + ChebyshevFilterParamBase<All, ImplBase: Same<ImplBase2>>>
where
    C: Conf
{
    type Conf: ChebyshevFilterConf;
    type OmegaEpsilon: Same<OmegaEpsilon<Self::F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>>
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
}

pub trait Chebyshev1FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type1}, OmegaEpsilon = OmegaEpsilonCheb1<<Self as FilterParam>::F, {<Self as FilterParam>::ORDER}>>
where
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait Chebyshev2FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type2}, OmegaEpsilon = OmegaEpsilonCheb2<<Self as FilterParam>::F, {<Self as FilterParam>::ORDER}>>
where
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait FirstOrderChebyshev1FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type1}, ORDER = 1, OmegaEpsilon = OmegaEpsilonCheb1FirstOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait FirstOrderChebyshev2FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type2}, ORDER = 1, OmegaEpsilon = OmegaEpsilonCheb2FirstOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait SecondOrderChebyshev1FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type1}, ORDER = 2, OmegaEpsilon = OmegaEpsilonCheb1SecondOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait SecondOrderChebyshev2FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type2}, ORDER = 2, OmegaEpsilon = OmegaEpsilonCheb2SecondOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait ThirdOrderChebyshev1FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type1}, ORDER = 3, OmegaEpsilon = OmegaEpsilonCheb1ThirdOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
pub trait ThirdOrderChebyshev2FilterParam<C: Conf> = ChebyshevFilterParam<C, TYPE = {ChebyshevType::Type2}, ORDER = 3, OmegaEpsilon = OmegaEpsilonCheb2ThirdOrder<<Self as FilterParam>::F>>
where
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;

impl<F, P, C, const ORDER: usize> ChebyshevFilterParam<C, Param<OmegaDyn<F>>> for P
where
    P: ButterworthFilterParam<C, F = F, ORDER = {ORDER}, Omega = Omega<F, ORDER>, Conf: ChebyshevFilterConf>, // TODO generalize for different orders
    C: Conf,
    F: FilterFloat,
    OmegaEpsilonCheb1<F, ORDER>: Same<OmegaEpsilon<F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>>,
    [(); Self::ORDER]:
{
    type Conf = P::Conf;

    type OmegaEpsilon = OmegaEpsilonCheb1<F, ORDER>
    where
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:
    {
        let Omega {omega} = self.omega();
        let x = omega.recip();
        let x2 = x*x;
        let one = <F as One>::one();
        let two = one + one;
        let rn = two.mul_add(x2, -one);
        let epsilon = rn.recip();

        OmegaEpsilon {
            omega,
            epsilon
        }
    }
}

#[derive(Clone, Copy, Debug, ConstParamTy, PartialEq, Eq)]
pub enum ChebyshevType
{
    Type1,
    Type2
}

pub trait ChebyshevFilterConf: Conf
{
    type Conf: private::ChebyshevFilterConfFinal<Self>;

    const OUTPUTS: usize;
}
impl<C, const OUTPUTS: usize> ChebyshevFilterConf for C
where
    C: EllipticFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = C::Conf;

    const OUTPUTS: usize = OUTPUTS;
}

mod private
{
    use crate::param::EllipticFilterConf;

    use super::ChebyshevFilterConf;

    pub trait ChebyshevFilterConfFinal<C>: ChebyshevFilterConf<
        Conf = Self
    >
    where
        C: ChebyshevFilterConf<
            Conf = Self
        >
    {

    }

    impl<
        C,
        CC
    > ChebyshevFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC
        >,
        C: EllipticFilterConf<
            Conf = CC
        >,
        /*Param<OmegaEpsilonCheb1Dyn<f32>>: ChebyshevFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1Dyn<f64>>: ChebyshevFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2Dyn<f32>>: ChebyshevFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2Dyn<f64>>: ChebyshevFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1SecondOrder<f32>>: ChebyshevFilterParam<CC, Conf = CC>, //+ SecondOrderFilterParam<CC>,
        Param<OmegaEpsilonCheb1SecondOrder<f64>>: ChebyshevFilterParam<CC, Conf = CC>, //+ SecondOrderFilterParam<CC>,
        Param<OmegaEpsilonCheb2SecondOrder<f32>>: ChebyshevFilterParam<CC, Conf = CC>, //+ SecondOrderFilterParam<CC>,
        Param<OmegaEpsilonCheb2SecondOrder<f64>>: ChebyshevFilterParam<CC, Conf = CC> //+ SecondOrderFilterParam<CC>*/
    {

    }
}