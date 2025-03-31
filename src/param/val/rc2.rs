use serde::{Serialize, Deserialize};

use crate::{change::Change, param::{FilterFloat, FilterParam, Param, SecondOrderRCFilterConf, SecondOrderRCFilterParam, SecondOrderRCFilterParamBase}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC2<F>
where
    F: FilterFloat
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F
}
impl<F> Change for RC2<F>
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
    }
}
impl<F> FilterParam for Param<RC2<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> SecondOrderRCFilterParamBase<C> for Param<RC2<F>>
where
    F: FilterFloat,
    C: SecondOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRCFilterParam<C, Param<RC2<F>>> for Param<RC2<F>>
where
    F: FilterFloat,
    C: SecondOrderRCFilterConf
{
    type Conf = C;

    fn rc2(&self) -> RC2<Self::F>
    {
        **self
    }
}