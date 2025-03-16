use crate::{param::FilterParam, params::PIVal};

pub trait PIFilterParam: FilterParam
{
    fn pi(&self) -> PIVal<Self::F>;
}