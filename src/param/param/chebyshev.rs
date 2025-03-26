use core::marker::PhantomData;

use num::One;

use super::{ButterworthFilterParam, EllipticFilterConf};

use crate::{conf::{All, Conf}, param::{Chebyshev1, Chebyshev2, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, Omega, OmegaDyn, OmegaEpsilon, OmegaEpsilonCheb1, OmegaEpsilonCheb1FirstOrder, Param}, util::same::Same};

pub trait ChebyshevFilterParam<
    C,
    ImplBase = <Self as ChebyshevFilterParamBase<C>>::ImplBase,
    ImplBase2 = <Self as EllipticFilterParamBase<C>>::ImplBase
>: ChebyshevFilterParamBase<C, ImplBase: Same<ImplBase>>
    + EllipticFilterParamBase<C, ImplBase: Same<ImplBase2> + ChebyshevFilterParamBase<All, ImplBase: Same<ImplBase2>>>
where
    C: Conf
{
    type Conf: EllipticFilterConf;
    type OmegaEpsilon//: Same<OmegaEpsilon<Self::F, Self::Type, {Self::ORDER}>>
    where
        [(); Self::ORDER]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); Self::ORDER]:;
}

macro_rules! special {
    ($trait:ident = $type:ident, $order:expr) => {
        pub trait $trait<C>: FilterParam
        where
            C: Conf
        {
            type Conf: EllipticFilterConf;

            fn omega_epsilon(&self) -> OmegaEpsilon<<Self as FilterParam>::F, $type, $order>;
        }
        impl<P, C> $trait<C> for P
        where
            P: ChebyshevFilterParam<C, Type = $type, OmegaEpsilon = OmegaEpsilon<<Self as FilterParam>::F, $type, $order>>,
            C: Conf,
            [(); Self::ORDER]:
        {
            type Conf = <Self as ChebyshevFilterParam<C>>::Conf;

            fn omega_epsilon(&self) -> OmegaEpsilon<<Self as FilterParam>::F, $type, $order>
            {
                ChebyshevFilterParam::omega_epsilon(self)
            }
        }
    };
}

special!(DynOrderChebyshev1FilterParam = Chebyshev1, 0);
special!(DynOrderChebyshev2FilterParam = Chebyshev2, 0);
special!(FirstOrderChebyshev1FilterParam = Chebyshev1, 1);
special!(FirstOrderChebyshev2FilterParam = Chebyshev2, 1);
special!(SecondOrderChebyshev1FilterParam = Chebyshev1, 2);
special!(SecondOrderChebyshev2FilterParam = Chebyshev2, 2);
special!(ThirdOrderChebyshev1FilterParam = Chebyshev1, 3);
special!(ThirdOrderChebyshev2FilterParam = Chebyshev2, 3);

impl<F, P, C, const ORDER: usize> ChebyshevFilterParam<C, Param<OmegaDyn<F>>> for P
where
    P: ButterworthFilterParam<C, F = F, ORDER = {ORDER}, Omega = Omega<F, ORDER>, Conf: EllipticFilterConf>, // TODO generalize for different orders
    C: Conf,
    F: FilterFloat,
    OmegaEpsilonCheb1<F, ORDER>: Same<OmegaEpsilon<F, Chebyshev1, {Self::ORDER}>>,
    [(); Self::ORDER]:
{
    type Conf = P::Conf;

    type OmegaEpsilon = OmegaEpsilonCheb1<F, ORDER>;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
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
            epsilon,
            _m: PhantomData
        }
    }
}