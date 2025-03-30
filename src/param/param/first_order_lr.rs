use crate::{conf::Conf, param::{FilterParam, FirstOrderFilterParamBase, FirstOrderLRFilterConf, Omega, OmegaFirstOrder, Param, LR}};

use super::FirstOrderFilterParam;

pub trait FirstOrderLRFilterParam<C>: FirstOrderFilterParamBase<C, ImplBase = Param<LR<<Self as FilterParam>::F>>>
    + FilterParam<ORDER = 1>
where
    C: Conf
{
    type Conf: FirstOrderLRFilterConf;

    fn lr(&self) -> LR<Self::F>;
}

impl<P, C> FirstOrderFilterParam<C, Param<LR<P::F>>> for P
where
    P: FirstOrderLRFilterParam<C>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> OmegaFirstOrder<Self::F>
    {
        let LR {l, r} = self.lr();
        Omega {
            omega: r/l
        }
    }
}