use num::Float;

use crate::{change::Change, param::{FilterFloat, FilterParam, PIFilterParam}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct PI<F>
where
    F: Float
{
    pub p: F,
    pub i: F
}
impl<F> Change for PI<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.p.change(to.p, change);
        self.i.change(to.i, change);
    }
}
impl<F> FilterParam for PI<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F> PIFilterParam for PI<F>
where
    F: FilterFloat
{
    fn pi(&self) -> PI<Self::F>
    {
        *self
    }
}