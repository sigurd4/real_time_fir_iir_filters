use crate::param::{FilterParam, PIVal};

pub trait PIFilterParam: FilterParam
{
    fn pi(&self) -> PIVal<Self::F>;
}