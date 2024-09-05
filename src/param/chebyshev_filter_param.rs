use core::marker::ConstParamTy;

use super::{Chebyshev1FilterParam, Chebyshev2FilterParam, EllipticFilterConf, EllipticFilterParamBase};

use crate::{conf::{All, Conf}, params::{OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb2Dyn}, util::same::Same};

pub trait ChebyshevFilterParamBase<C>: EllipticFilterParamBase<C>
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: ChebyshevFilterParamBase<All, ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase>;
}

pub trait ChebyshevFilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase,
    ImplBase2 = <Self as EllipticFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase: Same<ImplBase2>>
    + private::EitherChebyshev1OrShebyshev2FilterParam<C>
where
    C: Conf,
    ImplBase2: ChebyshevFilterParamBase<All, ImplBase = ImplBase2>
{
    const TYPE: ChebyshevType;

    type Conf: ChebyshevFilterConf;

    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
}

impl<P, C> ChebyshevFilterParam<C, <P as ChebyshevFilterParamBase<C>>::ImplBase, OmegaEpsilonCheb1Dyn<P::F>> for P
where
    P: Chebyshev1FilterParam<C> + private::EitherChebyshev1OrShebyshev2FilterParam<C>,
    C: Conf
{
    const TYPE: ChebyshevType = ChebyshevType::Type1;

    type Conf = <P as Chebyshev1FilterParam<C>>::Conf;

    fn omega(&self) -> Self::F
    {
        Chebyshev1FilterParam::omega(self)
    }
    fn epsilon(&self) -> Self::F
    {
        Chebyshev1FilterParam::epsilon(self)
    }
}
impl<P, C> ChebyshevFilterParam<C, <P as ChebyshevFilterParamBase<C>>::ImplBase, OmegaEpsilonCheb2Dyn<P::F>> for P
where
    P: Chebyshev2FilterParam<C> + private::EitherChebyshev1OrShebyshev2FilterParam<C>,
    C: Conf
{
    const TYPE: ChebyshevType = ChebyshevType::Type2;

    type Conf = <P as Chebyshev2FilterParam<C>>::Conf;

    fn omega(&self) -> Self::F
    {
        Chebyshev2FilterParam::omega(self)
    }
    fn epsilon(&self) -> Self::F
    {
        Chebyshev2FilterParam::epsilon(self)
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
    use crate::{conf::Conf, param::{ButterworthFilterConf, Chebyshev1FilterParam, Chebyshev2FilterParam, EllipticFilterConf, FilterParamSecondOrder}, params::{OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb2Dyn}};

    use super::{ChebyshevFilterConf, ChebyshevFilterParamBase};

    pub trait MaybeChebyshev1FilterParam<C>: ChebyshevFilterParamBase<C>
    where
        C: Conf
    {
        const IS_TYPE1: bool;
    }
    impl<CC, C> MaybeChebyshev1FilterParam<C> for CC
    where
        CC: ChebyshevFilterParamBase<C>,
        C: Conf
    {
        default const IS_TYPE1: bool = false;
    }
    impl<CC, C> MaybeChebyshev1FilterParam<C> for CC
    where
        CC: Chebyshev1FilterParam<C>,
        C: Conf
    {
        const IS_TYPE1: bool = true;
    }
    
    pub trait MaybeChebyshev2FilterParam<C>: ChebyshevFilterParamBase<C>
    where
        C: Conf
    {
        const IS_TYPE2: bool;
    }
    impl<CC, C> MaybeChebyshev2FilterParam<C> for CC
    where
        CC: ChebyshevFilterParamBase<C>,
        C: Conf
    {
        default const IS_TYPE2: bool = false;
    }
    impl<CC, C> MaybeChebyshev2FilterParam<C> for CC
    where
        CC: Chebyshev2FilterParam<C>,
        C: Conf
    {
        const IS_TYPE2: bool = true;
    }
    
    trait MaybeEitherChebyshev1OrShebyshev2FilterParam<C>: MaybeChebyshev1FilterParam<C> + MaybeChebyshev2FilterParam<C>
    where
        C: Conf
    {
        const IS_EITHER: bool;
    }
    impl<CC, C, const IS_TYPE1: bool, const IS_TYPE2: bool> MaybeEitherChebyshev1OrShebyshev2FilterParam<C> for CC
    where
        CC: MaybeChebyshev1FilterParam<C, IS_TYPE1 = {IS_TYPE1}> + MaybeChebyshev2FilterParam<C, IS_TYPE2 = {IS_TYPE2}>,
        C: Conf
    {
        const IS_EITHER: bool = IS_TYPE1 ^ IS_TYPE2;
    }

    pub trait EitherChebyshev1OrShebyshev2FilterParam<C>: MaybeChebyshev1FilterParam<C> + MaybeChebyshev2FilterParam<C>
    where
        C: Conf
    {

    }
    impl<CC, C> EitherChebyshev1OrShebyshev2FilterParam<C> for CC
    where
        CC: MaybeEitherChebyshev1OrShebyshev2FilterParam<C, IS_EITHER = true>,
        C: Conf
    {

    }

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
        CC,
        const OUTPUTS: usize
    > ChebyshevFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        > + ChebyshevFilterConf<
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        > + ButterworthFilterConf<
            0,
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        >,
        C: ChebyshevFilterConf<
            Conf = CC,
            OUTPUTS = {OUTPUTS}
        >,
        OmegaEpsilonCheb1Dyn<f32>: Chebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f64>: Chebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f32>: Chebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f64>: Chebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1SecondOrder<f32>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonCheb1SecondOrder<f64>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamSecondOrder
    {

    }
}