use num::Zero;

use crate::{conf::Conf, param::{FirstOrderRCFilterConf, SecondOrderRCFilterConf, SecondOrderRCFilterParamBase, RC, RC2}, util::same::Same};

use super::FirstOrderRCFilterParam;

pub trait SecondOrderRCFilterParam<
    C,
    ImplBase = <Self as SecondOrderRCFilterParamBase<C>>::ImplBase
>: SecondOrderRCFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: SecondOrderRCFilterConf;

    fn rc2(&self) -> RC2<Self::F>;
}

impl<P, C> SecondOrderRCFilterParam<C, RC<P::F>> for P
where
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as FirstOrderRCFilterConf>::AsSecondOrderRCFilterConf;

    fn rc2(&self) -> RC2<Self::F>
    {
        let RC {r, c} = self.rc();
        RC2 {
            r1: r,
            c1: c,
            r2: Zero::zero(),
            c2: Zero::zero()
        }
    }
}