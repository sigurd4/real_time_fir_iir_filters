use crate::param::{FilterParam, PI};

pub trait PIFilterParam: FilterParam
{
    fn pi(&self) -> PI<Self::F>;
}