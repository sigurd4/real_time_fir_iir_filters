use crate::{change::Change, param::{FilterFloat, FilterParam, Param, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
impl<F> Change for RC2GSallenKey<F>
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
        self.g.change(to.g, change);
    }
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