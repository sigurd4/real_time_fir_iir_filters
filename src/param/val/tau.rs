use num::Float;
use serde::{Serialize, Deserialize};

use crate::{change::Change, param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, FirstOrderAllPassFilterParamBase, Param}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Tau<F>
where
    F: Float
{
    pub tau: F
}
impl<F> Change for Tau<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.tau.change(to.tau, change);
    }
}
impl<F> FilterParam for Param<Tau<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for Param<Tau<F>>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderAllPassFilterParam<C> for Param<Tau<F>>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type Conf = C;

    fn tau(&self) -> Tau<F>
    {
        **self
    }
}