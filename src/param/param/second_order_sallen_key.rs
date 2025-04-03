use crate::{conf::Conf, param::{FilterParam, RC2GSallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParamBase}};

pub trait SecondOrderSallenKeyFilterParam<C>: ThirdOrderSallenKeyFilterParamBase<C, ImplBase = RC2GSallenKey<<Self as FilterParam>::F>>
where
    C: Conf
{
    type Conf: SecondOrderSallenKeyFilterConf;

    fn rc2g(&self) -> RC2GSallenKey<Self::F>;
}