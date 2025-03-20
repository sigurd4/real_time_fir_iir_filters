use num::Float;

use crate::param::{FilterFloat, FilterParam, PIDFilterParam, Param};

#[derive(Clone, Copy, Debug)]
pub struct PID<F>
where
    F: Float
{
    pub p: F,
    pub i: F,
    pub d: F
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