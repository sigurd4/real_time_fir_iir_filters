use serde::{Serialize, Deserialize};

use crate::param::{FilterFloat, FilterParam, Param, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC2GSallenKey<F>
where
    F: FilterFloat
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F,
    pub g: F
}
impl<F> FilterParam for Param<RC2GSallenKey<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for Param<RC2GSallenKey<F>>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderSallenKeyFilterParam<C> for Param<RC2GSallenKey<F>>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
{
    type Conf = C;

    fn rc2g(&self) -> RC2GSallenKey<Self::F>
    {
        **self
    }
}