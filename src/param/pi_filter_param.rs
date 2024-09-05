use super::FilterParam;

pub trait PIFilterParam: FilterParam
{
    fn p(&self) -> Self::F;
    fn i(&self) -> Self::F;
}