use crate::{conf::Conf, util::ObviousArray};

use super::{EllipticFilterConf, FirstOrderFilterConf, SecondOrderFilterConf, ThirdOrderFilterConf};

pub trait FirstOrderButterworthFilterConf = ButterworthFilterConf<1>;
pub trait SecondOrderButterworthFilterConf = ButterworthFilterConf<2>;
pub trait ThirdOrderButterworthFilterConf = ButterworthFilterConf<3>;

pub trait ButterworthFilterConf<const ORDER: usize>: Conf
{
    type Conf: private::ButterworthFilterConfFinal<ORDER, Self>;

    type Outputs<U>: ObviousArray<Elem = U>;
}

impl<C> ButterworthFilterConf<0> for C
where
    C: EllipticFilterConf
{
    type Conf = <Self as EllipticFilterConf>::Conf;

    type Outputs<U> = <C as EllipticFilterConf>::Outputs<U>;
}
impl<C> ButterworthFilterConf<1> for C
where
    C: FirstOrderFilterConf
{
    type Conf = <Self as FirstOrderFilterConf>::Conf;

    type Outputs<U> = <C as FirstOrderFilterConf>::Outputs<U>;
}
impl<C> ButterworthFilterConf<2> for C
where
    C: SecondOrderFilterConf
{
    type Conf = <Self as SecondOrderFilterConf>::Conf;
    
    type Outputs<U> = <C as SecondOrderFilterConf>::Outputs<U>;
}
impl<C> ButterworthFilterConf<3> for C
where
    C: ThirdOrderFilterConf
{
    type Conf = <Self as ThirdOrderFilterConf>::Conf;
    
    type Outputs<U> = <C as ThirdOrderFilterConf>::Outputs<U>;
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