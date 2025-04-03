use num::Float;

use crate::{change::Change, param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParamBase, FirstOrderFilterParamBase, FirstOrderRCFilterConf, FirstOrderRCFilterParam, SecondOrderRCFilterParamBase, SecondOrderRLCFilterParamBase, ThirdOrderSallenKeyFilterParamBase}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC<F>
where
    F: Float
{
    pub r: F,
    pub c: F
}
impl<F> Change for RC<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.r.change(to.r, change);
        self.c.change(to.c, change);
    }
}
impl<F> FilterParam for RC<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRLCFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRCFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderRCFilterParam<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type Conf = C;

    fn rc(&self) -> RC<Self::F>
    {
        *self
    }
}