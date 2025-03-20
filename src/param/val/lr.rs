use num::Float;

use crate::param::{FilterFloat, FilterParam, FirstOrderFilterParamBase, FirstOrderLRFilterConf, FirstOrderLRFilterParam, Param};

#[derive(Copy, Clone, Debug)]
pub struct LR<F>
where
    F: Float
{
    pub l: F,
    pub r: F
}

impl<F> FilterParam for Param<LR<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderFilterParamBase<C> for Param<LR<F>>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderLRFilterParam<C> for Param<LR<F>>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type Conf = C;

    fn lr(&self) -> LR<Self::F>
    {
        **self
    }
}