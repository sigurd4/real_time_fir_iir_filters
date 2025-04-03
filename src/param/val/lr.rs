use crate::{change::Change, param::{FilterFloat, FilterParam, FirstOrderFilterParamBase, FirstOrderLRFilterConf, FirstOrderLRFilterParam}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct LR<F>
where
    F: FilterFloat
{
    pub l: F,
    pub r: F
}
impl<F> Change for LR<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.l.change(to.l, change);
        self.r.change(to.r, change);
    }
}
impl<F> FilterParam for LR<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderFilterParamBase<C> for LR<F>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderLRFilterParam<C> for LR<F>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type Conf = C;

    fn lr(&self) -> LR<Self::F>
    {
        *self
    }
}