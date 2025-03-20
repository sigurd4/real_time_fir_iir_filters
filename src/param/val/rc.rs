use num::Float;

use crate::param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParamBase, FirstOrderFilterParamBase, FirstOrderRCFilterConf, FirstOrderRCFilterParam, Param, SecondOrderRCFilterParamBase, SecondOrderRLCFilterParamBase, ThirdOrderSallenKeyFilterParamBase};

#[derive(Clone, Copy, Debug)]
pub struct RC<F>
where
    F: Float
{
    pub r: F,
    pub c: F
}
impl<F> FilterParam for Param<RC<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderFilterParamBase<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRLCFilterParamBase<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRCFilterParamBase<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderRCFilterParam<C> for Param<RC<F>>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type Conf = C;

    fn rc(&self) -> RC<Self::F>
    {
        **self
    }
}