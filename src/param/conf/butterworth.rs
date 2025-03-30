use crate::conf::Conf;

use super::{EllipticFilterConf, FirstOrderFilterConf, SecondOrderFilterConf, ThirdOrderFilterConf};

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