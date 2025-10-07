use super::ButterworthFilterParam;

use crate::{conf::Conf, param::{ChebyshevFilterParamBase, EllipticFilterConf, EllipticFilterParamBase, FilterFloat, FilterParam, Omega, OmegaDyn, OmegaEpsilonCheb1Dyn}, util::same::Same};

pub trait BesselFilterParam<
    C
>: ChebyshevFilterParamBase<C, ImplBase = OmegaDyn<<Self as FilterParam>::F>/*, TYPE = false*/>
    + EllipticFilterParamBase<C, ImplBase = OmegaEpsilonCheb1Dyn<<Self as FilterParam>::F>>
where
    C: Conf
{
    type Conf: EllipticFilterConf;
    type Omega//: Same<Omega<Self::F, {<Self as ChebyshevFilterParamBase<C>>::TYPE}, {Self::ORDER}>> // I don't understand why this doesn't work :(
    where
        [(); Self::ORDER]:;

    fn omega(&self) -> Self::Omega
    where
        [(); Self::ORDER]:;
}

macro_rules! special {
    ($trait:ident = $order:expr) => {
        pub trait $trait<C>: FilterParam
        where
            C: Conf
        {
            type Conf: EllipticFilterConf;

            fn omega(&self) -> Omega<<Self as FilterParam>::F, $order>;
        }
        impl<P, C> $trait<C> for P
        where
            P: BesselFilterParam<C, ORDER = {$order}, Omega = Omega<<Self as FilterParam>::F, $order>>,
            C: Conf,
            [(); Self::ORDER]:
        {
            type Conf = <Self as BesselFilterParam<C>>::Conf;

            fn omega(&self) -> Omega<<Self as FilterParam>::F, $order>
            {
                BesselFilterParam::omega(self)
            }
        }
    };
}

special!(DynOrderBesselFilterParam = 0);
special!(FirstOrderBesselFilterParam = 1);
special!(SecondOrderBesselFilterParam = 2);
special!(ThirdOrderBesselFilterParam = 3);

impl<F, P, C, const ORDER: usize> BesselFilterParam<C> for P
where
    P: ButterworthFilterParam<C, F = F, ORDER = {ORDER}, Omega = Omega<F, ORDER>, Conf: EllipticFilterConf>,
    C: Conf,
    F: FilterFloat,
    Omega<F, ORDER>: Same<Omega<F, {Self::ORDER}>>,
    [(); Self::ORDER]:
{
    type Conf = P::Conf;

    type Omega = Omega<F, ORDER>;

    fn omega(&self) -> Self::Omega
    {
        ButterworthFilterParam::omega(self)
    }
}