use crate::{conf::Conf, param::{ChebyshevFilterParamBase, ChebyshevType, EllipticFilterParamBase, FilterParam, Omega, OmegaDyn, OmegaEpsilonCheb1Dyn, Param}, util::same::Same};

use super::{EllipticFilterConf, FirstOrderFilterConf, FirstOrderFilterParam, SecondOrderFilterConf, SecondOrderFilterParam, ThirdOrderFilterConf, ThirdOrderFilterParam};

pub trait ButterworthFilterParam<
    C
>: ChebyshevFilterParamBase<C, ImplBase = Param<OmegaDyn<<Self as FilterParam>::F>>, TYPE = {ChebyshevType::Type1}>
    + EllipticFilterParamBase<C, ImplBase = Param<OmegaEpsilonCheb1Dyn<<Self as FilterParam>::F>>>
where
    C: Conf
{
    type Conf: ButterworthFilterConf<{Self::ORDER}>
    where
        [(); Self::ORDER]:;
    type Omega: Same<Omega<Self::F, {Self::ORDER}>>
    where
        [(); Self::ORDER]:;

    fn omega(&self) -> Self::Omega
    where
        [(); Self::ORDER]:;
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

pub trait FirstOrderButterworthFilterConf = ButterworthFilterConf<1>;
pub trait SecondOrderButterworthFilterConf = ButterworthFilterConf<2>;
pub trait ThirdOrderButterworthFilterConf = ButterworthFilterConf<3>;

pub trait ButterworthFilterConf<const ORDER: usize>: Conf
{
    type Conf: private::ButterworthFilterConfFinal<ORDER, Self>;

    const OUTPUTS: usize;
}

impl<C, const OUTPUTS: usize> ButterworthFilterConf<0> for C
where
    C: EllipticFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = <Self as EllipticFilterConf>::Conf;

    const OUTPUTS: usize = OUTPUTS;
}
impl<C, const OUTPUTS: usize> ButterworthFilterConf<1> for C
where
    C: FirstOrderFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = <Self as FirstOrderFilterConf>::Conf;

    const OUTPUTS: usize = OUTPUTS;
}
impl<C, const OUTPUTS: usize> ButterworthFilterConf<2> for C
where
    C: SecondOrderFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = <Self as SecondOrderFilterConf>::Conf;
    
    const OUTPUTS: usize = OUTPUTS;
}
impl<C, const OUTPUTS: usize> ButterworthFilterConf<3> for C
where
    C: ThirdOrderFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = <Self as ThirdOrderFilterConf>::Conf;
    
    const OUTPUTS: usize = OUTPUTS;
}

mod private
{
    use crate::param::{EllipticFilterConf, FirstOrderFilterConf, SecondOrderFilterConf, ThirdOrderFilterConf};

    use super::ButterworthFilterConf;

    pub trait ButterworthFilterConfFinal<const ORDER: usize, C>: ButterworthFilterConf<
        ORDER,
        Conf = Self
    >
    where
        C: ButterworthFilterConf<
            ORDER,
            Conf = Self
        >
    {

    }

    impl<
        CC,
        C
    > ButterworthFilterConfFinal<0, C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC
        >,
        C: EllipticFilterConf<
            Conf = CC
        >,
    {

    }

    impl<
        CC,
        C
    > ButterworthFilterConfFinal<1, C> for CC
    where
        CC: FirstOrderFilterConf<
            Conf = CC
        >,
        C: FirstOrderFilterConf<
            Conf = CC
        >,
    {

    }

    impl<
        CC,
        C
    > ButterworthFilterConfFinal<2, C> for CC
    where
        CC: SecondOrderFilterConf<
            Conf = CC
        >,
        C: SecondOrderFilterConf<
            Conf = CC
        >,
    {

    }

    impl<
        CC,
        C
    > ButterworthFilterConfFinal<3, C> for CC
    where
        CC: ThirdOrderFilterConf<
            Conf = CC
        >,
        C: ThirdOrderFilterConf<
            Conf = CC
        >,
    {

    }
}