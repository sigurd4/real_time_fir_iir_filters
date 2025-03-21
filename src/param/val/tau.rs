use num::Float;

use crate::param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, FirstOrderAllPassFilterParamBase, Param};

#[derive(Clone, Copy, Debug)]
pub struct Tau<F>
where
    F: Float
{
    pub tau: F
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