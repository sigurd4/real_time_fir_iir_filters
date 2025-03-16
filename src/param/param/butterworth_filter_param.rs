use super::*;
use crate::{conf::Conf, param::{ChebyshevFilterParamBase, EllipticFilterParamBase, FilterParam}, params::*};

pub trait ButterworthFilterParam<C>: ChebyshevFilterParamBase<C, ImplBase = OmegaDyn<<Self as FilterParam>::F>>
    + EllipticFilterParamBase<C, ImplBase = OmegaEpsilonDyn<<Self as FilterParam>::F, {ChebyshevType::Type1}>>
where
    C: Conf
{
    type Conf: ButterworthFilterConfFor<Self, C>;

    fn omega(&self) -> Self::F;
}

pub trait SecondOrderButterworthFilterConf = ButterworthFilterConf<2>;
pub trait ThirdOrderButterworthFilterConf = ButterworthFilterConf<3>;

pub trait ButterworthFilterConf<const ORDER: usize>: Conf
{
    type Conf: private::ButterworthFilterConfFinal<ORDER, Self>;

    const OUTPUTS: usize;
}

impl<C, const OUTPUTS: usize> ButterworthFilterConf<0> for C
where
    C: ChebyshevFilterConf<OUTPUTS = {OUTPUTS}>
{
    type Conf = <Self as ChebyshevFilterConf>::Conf;

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

pub trait ButterworthFilterConfFor<P, C>: private::ButterworthFilterConfFor<P, C>
where
    C: Conf
{

}
impl<P, C, CC> ButterworthFilterConfFor<P, C> for CC
where
    CC: private::ButterworthFilterConfFor<P, C>,
    C: Conf
{

}

mod private
{
    use crate::{conf::Conf, param::FilterParam, params::Omega};

    use super::{ButterworthFilterConf, ButterworthFilterParam};

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
        const ORDER: usize,
        CC,
        C
    > ButterworthFilterConfFinal<ORDER, C> for CC
    where
        CC: ButterworthFilterConf<
            ORDER,
            Conf = CC
        >,
        C: ButterworthFilterConf<
            ORDER,
            Conf = CC
        >,
        Omega<f64, ORDER>: ButterworthFilterParam<CC, Conf = CC>,
        Omega<f32, ORDER>: ButterworthFilterParam<CC, Conf = CC>
    {

    }

    pub trait ButterworthFilterConfFor<P, C>: Conf
    where
        C: Conf
    {

    }
    impl<P, C, CC, const ORDER: usize> ButterworthFilterConfFor<P, C> for CC
    where
        P: FilterParam<ORDER = {ORDER}>,
        C: Conf,
        CC: ButterworthFilterConf<ORDER>
    {

    }
}