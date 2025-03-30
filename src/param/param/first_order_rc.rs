use num::Float;

use crate::{conf::{all, All, Conf, HighPass, LowPass}, param::{FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParamBase, FirstOrderFilterParamBase, FirstOrderRCFilterConf, Omega, OmegaFirstOrder, Param, SecondOrderRCFilterParamBase, SecondOrderRLCFilterParamBase, Tau, ThirdOrderSallenKeyFilterParamBase, RC}};

use super::{FirstOrderAllPassFilterParam, FirstOrderFilterParam};

pub type AllFirstOrderRCFilterParamConf = all!(LowPass, HighPass);

pub trait FirstOrderRCFilterParam<C>: FirstOrderFilterParamBase<C, ImplBase = Param<RC<<Self as FilterParam>::F>>>
    + ThirdOrderSallenKeyFilterParamBase<C, ImplBase = Param<RC<<Self as FilterParam>::F>>>
    + SecondOrderRCFilterParamBase<C, ImplBase = Param<RC<<Self as FilterParam>::F>>>
    + SecondOrderRLCFilterParamBase<C, ImplBase = Param<RC<<Self as FilterParam>::F>>>
    + FilterParam<ORDER = 1>
where
    C: Conf
{
    type Conf: FirstOrderRCFilterConf;

    fn rc(&self) -> RC<Self::F>;
}

impl<P, C> FirstOrderFilterParam<C, Param<RC<P::F>>> for P
where
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> OmegaFirstOrder<Self::F>
    {
        let RC {r, c} = self.rc();
        Omega {
            omega: (r*c).recip()
        }
    }
}

impl<P, C> FirstOrderAllPassFilterParam<C, Param<RC<P::F>>> for P
where
    P: FirstOrderRCFilterParam<All> + FirstOrderAllPassFilterParamBase<C, ImplBase = Param<RC<<P as FilterParam>::F>>>,
    C: FirstOrderAllPassFilterConf
{
    type Conf = C;

    fn tau(&self) -> Tau<Self::F>
    {
        let RC {r, c} = self.rc();
        Tau {
            tau: r*c
        }
    }
}