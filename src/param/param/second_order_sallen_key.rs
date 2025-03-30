use crate::{conf::Conf, param::{FilterParam, Param, RC2GSallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParamBase}};

pub trait SecondOrderSallenKeyFilterParam<C>: ThirdOrderSallenKeyFilterParamBase<C, ImplBase = Param<RC2GSallenKey<<Self as FilterParam>::F>>>
where
    C: Conf
{
    type Conf: SecondOrderSallenKeyFilterConf;

    fn rc2g(&self) -> RC2GSallenKey<Self::F>;
}