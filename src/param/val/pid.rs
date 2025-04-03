use num::Float;

use crate::{change::Change, param::{FilterFloat, FilterParam, PIDFilterParam, Param}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct PID<F>
where
    F: Float
{
    pub p: F,
    pub i: F,
    pub d: F
}
impl<F> Change for PID<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.p.change(to.p, change);
        self.i.change(to.i, change);
        self.d.change(to.d, change);
    }
}
impl<F> FilterParam for Param<PID<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F> PIDFilterParam for Param<PID<F>>
where
    F: FilterFloat
{
    fn pid(&self) -> PID<Self::F>
    {
        **self
    }
}