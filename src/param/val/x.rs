use serde::{Serialize, Deserialize};

use crate::param::FilterFloat;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct X<F>
where
    F: FilterFloat
{
    pub x: F
}