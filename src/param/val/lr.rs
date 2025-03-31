use serde::{Deserialize, Serialize};

use crate::param::{FilterFloat, FilterParam, FirstOrderFilterParamBase, FirstOrderLRFilterConf, FirstOrderLRFilterParam, Param};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct LR<F>
where
    F: FilterFloat
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