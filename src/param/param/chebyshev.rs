use core::marker::ConstParamTy;

use num::One;

use super::{ButterworthFilterParam, EllipticFilterConf};

use crate::{conf::{All, Conf}, param::{ChebyshevFilterParamBase, EllipticFilterParamBase, OmegaEpsilonVal, OmegaVal}, params::OmegaDyn, util::same::Same};

pub trait ChebyshevFilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase,
    ImplBase2 = <Self as EllipticFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase: Same<ImplBase2> + ChebyshevFilterParamBase<All, ImplBase: Same<ImplBase2>>>
where
    C: Conf
{
    const TYPE: ChebyshevType;

    type Conf: ChebyshevFilterConf;

    fn omega_epsilon(&self) -> OmegaEpsilonVal<Self::F>;
}

impl<P, C> ChebyshevFilterParam<C, OmegaDyn<P::F>> for P
where
    P: ButterworthFilterParam<C, Conf: ChebyshevFilterConf>, // TODO generalize for different orders
    C: Conf,
    [(); P::ORDER]:
{
    const TYPE: ChebyshevType = ChebyshevType::Type1;

    type Conf = P::Conf;

    fn omega_epsilon(&self) -> OmegaEpsilonVal<Self::F>
    {
        let OmegaVal {omega} = self.omega();
        let x = omega.recip();
        let x2 = x*x;
        let one = <P::F as One>::one();
        let two = one + one;
        let rn = two.mul_add(x2, -one);
        let epsilon = rn.recip();

        OmegaEpsilonVal {
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
    use crate::{param::EllipticFilterConf, params::{OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb2Dyn, OmegaEpsilonCheb2SecondOrder}};

    use super::{ChebyshevFilterConf, ChebyshevFilterParam};

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
        >,
        C: EllipticFilterConf<
            Conf = CC,
            OUTPUTS = {OUTPUTS}
        >,
        OmegaEpsilonCheb1Dyn<f32>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f64>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f32>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f64>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1SecondOrder<f32>: ChebyshevFilterParam<CC, Conf = CC>/* + SecondOrderFilterParam<CC>*/,
        OmegaEpsilonCheb1SecondOrder<f64>: ChebyshevFilterParam<CC, Conf = CC>/* + SecondOrderFilterParam<CC>*/,
        OmegaEpsilonCheb2SecondOrder<f32>: ChebyshevFilterParam<CC, Conf = CC>/* + SecondOrderFilterParam<CC>*/,
        OmegaEpsilonCheb2SecondOrder<f64>: ChebyshevFilterParam<CC, Conf = CC>/* + SecondOrderFilterParam<CC>*/
    {

    }
}