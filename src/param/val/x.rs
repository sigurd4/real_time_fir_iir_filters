use crate::{change::Change, param::FilterFloat};

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