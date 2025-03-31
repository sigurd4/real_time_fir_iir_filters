use num::One;
use serde::{Serialize, Deserialize};

use crate::{change::Change, param::{FilterFloat, FilterParam, Param, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}};

use super::RC3GSallenKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC3SallenKey<F>
where
    F: FilterFloat
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F,
    pub r3: F,
    pub c3: F
}
impl<F> Change for RC3SallenKey<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.r1.change(to.r1, change);
        self.c1.change(to.c1, change);
        self.r2.change(to.r2, change);
        self.c2.change(to.c2, change);
        self.r3.change(to.r3, change);
        self.c3.change(to.c3, change);
    }
}
impl<F> FilterParam for Param<RC3SallenKey<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 3;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for Param<RC3SallenKey<F>>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
{
    type ImplBase = Param<RC3GSallenKey<F>>;
}
impl<F, C> ThirdOrderSallenKeyFilterParam<C, Param<RC3GSallenKey<F>>> for Param<RC3SallenKey<F>>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
{
    type Conf = C;

    fn rc3g(&self) -> RC3GSallenKey<Self::F>
    {
        let RC3SallenKey {r1, c1, r2, c2, r3, c3} = **self;
        RC3GSallenKey {
            r1,
            c1,
            r2,
            c2,
            r3,
            c3,
            g: One::one()
        }
    }
}