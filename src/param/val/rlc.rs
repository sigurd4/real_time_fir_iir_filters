use num::Float;

use crate::{change::Change, param::{FilterFloat, FilterParam, Param, SecondOrderRLCFilterConf, SecondOrderRLCFilterParam, SecondOrderRLCFilterParamBase}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RLC<F>
where
    F: Float
{
    pub r: F,
    pub l: F,
    pub c: F
}
impl<F> Change for RLC<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.r.change(to.r, change);
        self.l.change(to.l, change);
        self.c.change(to.c, change);
    }
}
impl<F> FilterParam for Param<RLC<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> SecondOrderRLCFilterParamBase<C> for Param<RLC<F>>
where
    F: FilterFloat,
    C: SecondOrderRLCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRLCFilterParam<C, Param<RLC<F>>> for Param<RLC<F>>
where
    F: FilterFloat,
    C: SecondOrderRLCFilterConf
{
    type Conf = C;

    fn rlc(&self) -> RLC<Self::F>
    {
        **self
    }
}