use num::One;

use super::ButterworthFilterParam;

use crate::{conf::{All, Conf}, param::{ChebyshevFilterParamBase, EllipticFilterConf, EllipticFilterParamBase, FilterFloat, FilterParam, Omega, OmegaDyn, OmegaEpsilon, OmegaEpsilonCheb1, Param}, util::same::Same};

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
    type OmegaEpsilon//: Same<OmegaEpsilon<Self::F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>> // I don't understand why this doesn't work :(
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;

    fn omega_epsilon(&self) -> Self::OmegaEpsilon
    where
        [(); Self::ORDER]:,
        [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:;
}

macro_rules! special {
    ($trait:ident = $type:expr, $order:expr) => {
        pub trait $trait<C>: FilterParam
        where
            C: Conf
        {
            type Conf: EllipticFilterConf;

            fn omega_epsilon(&self) -> OmegaEpsilon<<Self as FilterParam>::F, $type, $order>;
        }
        impl<P, C> $trait<C> for P
        where
            P: ChebyshevFilterParam<C, TYPE = $type, ORDER = {$order}, OmegaEpsilon = OmegaEpsilon<<Self as FilterParam>::F, $type, $order>>,
            C: Conf,
            [(); Self::ORDER]:,
            [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:
        {
            type Conf = <Self as ChebyshevFilterParam<C>>::Conf;

            fn omega_epsilon(&self) -> OmegaEpsilon<<Self as FilterParam>::F, $type, $order>
            {
                ChebyshevFilterParam::omega_epsilon(self)
            }
        }
    };
}

special!(DynOrderChebyshev1FilterParam = false, 0);
special!(DynOrderChebyshev2FilterParam = true, 0);
special!(FirstOrderChebyshev1FilterParam = false, 1);
special!(FirstOrderChebyshev2FilterParam = true, 1);
special!(SecondOrderChebyshev1FilterParam = false, 2);
special!(SecondOrderChebyshev2FilterParam = true, 2);
special!(ThirdOrderChebyshev1FilterParam = false, 3);
special!(ThirdOrderChebyshev2FilterParam = true, 3);

impl<F, P, C, const ORDER: usize> ChebyshevFilterParam<C, Param<OmegaDyn<F>>> for P
where
    P: ButterworthFilterParam<C, F = F, ORDER = {ORDER}, Omega = Omega<F, ORDER>, Conf: EllipticFilterConf>, // TODO generalize for different orders
    C: Conf,
    F: FilterFloat,
    OmegaEpsilonCheb1<F, ORDER>: Same<OmegaEpsilon<F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>>,
    [(); Self::ORDER]:,
    [(); {<Self as ChebyshevFilterParamBase<C>>::TYPE} as usize]:
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
            epsilon
        }
    }
}