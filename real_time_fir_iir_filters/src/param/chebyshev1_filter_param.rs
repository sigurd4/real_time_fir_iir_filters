use num::{Float, One};

use crate::{conf::Conf, params::{OmegaDyn, OmegaEpsilonCheb1Dyn}, util::same::Same};

use super::{ButterworthFilterParam, ChebyshevFilterConf, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterParam};

pub trait Chebyshev1FilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase = OmegaEpsilonCheb1Dyn<<Self as FilterParam>::F>>
where
    C: Conf
{
    type Conf: ChebyshevFilterConf;

    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
}

impl<P, C> Chebyshev1FilterParam<C, OmegaDyn<P::F>> for P
where
    P: ButterworthFilterParam<C, Conf: ChebyshevFilterConf> + FilterParam<ORDER = 2>, // TODO generalize for different orders
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        ButterworthFilterParam::omega(self)
    }
    fn epsilon(&self) -> Self::F
    {
        let omega = ButterworthFilterParam::omega(self);
        let x = omega.recip();
        let x2 = x*x;
        let one = <P::F as One>::one();
        let two = one + one;
        let rn = two.mul_add(x2, -one);
        let epsilon = rn.recip();
        epsilon
    }
}