use num::Float;

use crate::param::{FilterFloat, FilterParam, Param, SecondOrderRLCFilterConf, SecondOrderRLCFilterParam, SecondOrderRLCFilterParamBase};

#[derive(Clone, Copy, Debug)]
pub struct RLC<F>
where
    F: Float
{
    pub r: F,
    pub l: F,
    pub c: F
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