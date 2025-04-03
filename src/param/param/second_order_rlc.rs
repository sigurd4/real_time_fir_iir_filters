use num::Zero;

use crate::{conf::Conf, param::{FirstOrderRCFilterConf, SecondOrderRLCFilterConf, SecondOrderRLCFilterParamBase, RC, RLC}, util::same::Same};

use super::FirstOrderRCFilterParam;

pub trait SecondOrderRLCFilterParam<
    C,
    ImplBase = <Self as SecondOrderRLCFilterParamBase<C>>::ImplBase
>: SecondOrderRLCFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: SecondOrderRLCFilterConf;

    fn rlc(&self) -> RLC<Self::F>;
}

impl<P, C> SecondOrderRLCFilterParam<C, RC<P::F>> for P
where 
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as FirstOrderRCFilterConf>::AsSecondOrderRLCFilterConf;

    fn rlc(&self) -> RLC<Self::F>
    {
        let RC {r, c} = self.rc();
        RLC {
            r,
            l: Zero::zero(),
            c
        }
    }
}