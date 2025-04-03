use crate::{change::Change, param::{FilterFloat, FilterParam}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct X<F>
where
    F: FilterFloat
{
    pub x: F
}
impl<F> Change for X<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.x.change(to.x, change);
    }
}
impl<F> FilterParam for X<F>
where
    F: FilterFloat
{
    const ORDER: usize = 0;

    type F = F;
}