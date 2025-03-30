use crate::{conf::Conf, param::{ButterworthFilterConf, ChebyshevFilterParamBase, EllipticFilterParamBase, FilterFloat, FilterParam, FirstOrderFilterConf, FirstOrderFilterParamBase, Omega, OmegaDyn, OmegaEpsilonCheb1Dyn, OmegaFirstOrder, Param}, util::same::Same};

use super::{FirstOrderFilterParam, SecondOrderFilterParam, ThirdOrderFilterParam};

pub trait ButterworthFilterParam<
    C
>: ChebyshevFilterParamBase<C, ImplBase = Param<OmegaDyn<<Self as FilterParam>::F>>, TYPE = false>
    + EllipticFilterParamBase<C, ImplBase = Param<OmegaEpsilonCheb1Dyn<<Self as FilterParam>::F>>>
where
    C: Conf
{
    type Conf: ButterworthFilterConf<{Self::ORDER}>
    where
        [(); Self::ORDER]:;
    type Omega//: Same<Omega<Self::F, {Self::ORDER}>>
    where
        [(); Self::ORDER]:;

    fn omega(&self) -> Self::Omega
    where
        [(); Self::ORDER]:;
}

impl<F, C, const ORDER: usize> ButterworthFilterParam<C> for Param<Omega<F, ORDER>>
where
    F: FilterFloat,
    C: ButterworthFilterConf<{Self::ORDER}> + ButterworthFilterConf<ORDER>,
    Omega<F, ORDER>: Same<Omega<F, {Self::ORDER}>>
{
    type Conf = C
    where
        [(); Self::ORDER]:;
    type Omega = Omega<F, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega(&self) -> Self::Omega
    where
        [(); Self::ORDER]:
    {
        **self
    }
}

impl<P, C> FirstOrderFilterParam<C, Param<OmegaFirstOrder<P::F>>> for P
where
    P: ButterworthFilterParam<C, ORDER = 1, Conf: FirstOrderFilterConf, Omega = OmegaFirstOrder<<P as FilterParam>::F>> + FirstOrderFilterParamBase<C, ImplBase = Param<OmegaFirstOrder<<P as FilterParam>::F>>>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = P::Conf;

    #[doc(hidden)]
    fn omega(&self) -> OmegaFirstOrder<Self::F>
    {
        ButterworthFilterParam::omega(self)
    }
}

macro_rules! special {
    ($trait:ident$(: $of:ident)? = $order:expr) => {
        pub trait $trait<C>: FilterParam $(+ $of<C>)?
        where
            C: Conf
        {
            type Conf: ButterworthFilterConf<$order>;

            fn omega(&self) -> Omega<<Self as FilterParam>::F, $order>;
        }
        impl<P, C> $trait<C> for P
        where
            P: ButterworthFilterParam<C, Conf: ButterworthFilterConf<$order>, ORDER = $order, Omega = Omega<<Self as FilterParam>::F, $order>> $(+ $of<C>)?,
            C: Conf,
            [(); Self::ORDER]:
        {
            type Conf = <Self as ButterworthFilterParam<C>>::Conf;

            fn omega(&self) -> Omega<<Self as FilterParam>::F, $order>
            {
                ButterworthFilterParam::omega(self)
            }
        }
    };
}

special!(DynOrderButterworthFilterParam = 0);
special!(FirstOrderButterworthFilterParam: FirstOrderFilterParam = 1);
special!(SecondOrderButterworthFilterParam: SecondOrderFilterParam = 2);
special!(ThirdOrderButterworthFilterParam: ThirdOrderFilterParam = 3);