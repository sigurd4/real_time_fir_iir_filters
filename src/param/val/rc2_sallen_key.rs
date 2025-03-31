use num::{Float, One};
use serde::{Serialize, Deserialize};

use crate::param::{FilterFloat, FilterParam, Param, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase};

use super::RC2GSallenKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC2SallenKey<F>
where
    F: Float
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F
}
impl<F> FilterParam for Param<RC2SallenKey<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for Param<RC2SallenKey<F>>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
{
    type ImplBase = Param<RC2GSallenKey<F>>;
}
impl<F, C> SecondOrderSallenKeyFilterParam<C> for Param<RC2SallenKey<F>>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
{
    type Conf = C;

    fn rc2g(&self) -> RC2GSallenKey<Self::F>
    {
        let RC2SallenKey {r1, c1, r2, c2} = **self;
        RC2GSallenKey {
            r1,
            c1,
            r2,
            c2,
            g: One::one()
        }
    }
}