use num::Float;

use crate::{change::Change, param::{FilterFloat, FilterParam, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC3GSallenKey<F>
where
    F: Float
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F,
    pub r3: F,
    pub c3: F,
    pub g: F
}
impl<F> Change for RC3GSallenKey<F>
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
        self.g.change(to.g, change);
    }
}
impl<F> FilterParam for RC3GSallenKey<F>
where
    F: FilterFloat
{
    const ORDER: usize = 3;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for RC3GSallenKey<F>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderSallenKeyFilterParam<C, RC3GSallenKey<F>> for RC3GSallenKey<F>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
{
    type Conf = C;

    fn rc3g(&self) -> RC3GSallenKey<Self::F>
    {
        *self
    }
}