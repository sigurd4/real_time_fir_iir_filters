use num::Float;
use serde::{Serialize, Deserialize};

use crate::param::{FilterFloat, FilterParam, PIFilterParam, Param};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct PI<F>
where
    F: Float
{
    pub p: F,
    pub i: F
}
impl<F> FilterParam for Param<PI<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F> PIFilterParam for Param<PI<F>>
where
    F: FilterFloat
{
    fn pi(&self) -> PI<Self::F>
    {
        **self
    }
}