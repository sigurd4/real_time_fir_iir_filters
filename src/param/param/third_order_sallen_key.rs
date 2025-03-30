use num::{One, Zero};

use crate::{conf::Conf, f, param::{FirstOrderRCFilterConf, Param, RC2GSallenKey, RC3GSallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParamBase, RC}, util::same::Same};

use super::{FirstOrderRCFilterParam, SecondOrderSallenKeyFilterParam};

pub trait ThirdOrderSallenKeyFilterParam<
    C,
    ImplBase = <Self as ThirdOrderSallenKeyFilterParamBase<C>>::ImplBase
>: ThirdOrderSallenKeyFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: ThirdOrderSallenKeyFilterConf;

    fn rc3g(&self) -> RC3GSallenKey<Self::F>;
}

impl<P, C> ThirdOrderSallenKeyFilterParam<C, Param<RC2GSallenKey<P::F>>> for P
where
    P: SecondOrderSallenKeyFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as SecondOrderSallenKeyFilterConf>::AsThirdOrderSallenKeyFilterConf;

    fn rc3g(&self) -> RC3GSallenKey<Self::F>
    {
        let RC2GSallenKey {r1, c1, r2, c2, g} = self.rc2g();
        RC3GSallenKey {
            r1: Zero::zero(),
            c1: Zero::zero(),
            r2: r1,
            c2: c1,
            r3: r2,
            c3: c2,
            g
        }
    }
}

impl<P, C> ThirdOrderSallenKeyFilterParam<C, Param<RC<P::F>>> for P
where
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as FirstOrderRCFilterConf>::AsThirdOrderSallenKeyFilterConf;

    fn rc3g(&self) -> RC3GSallenKey<Self::F>
    {
        let RC {r, c} = self.rc();
        RC3GSallenKey {
            r1: r,
            c1: c,
            r2: f!(1e3; Self::F),
            c2: Zero::zero(),
            r3: Zero::zero(),
            c3: Zero::zero(),
            g: One::one()
        }
    }
}