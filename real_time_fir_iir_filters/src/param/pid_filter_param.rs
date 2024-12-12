use num::Zero;

use super::{FilterParam, PIFilterParam};

pub trait PIDFilterParam: FilterParam
{
    fn p(&self) -> Self::F;
    fn i(&self) -> Self::F;
    fn d(&self) -> Self::F;
}
impl<P> PIDFilterParam for P
where
    P: PIFilterParam
{
    fn p(&self) -> Self::F
    {
        PIFilterParam::p(self)
    }
    fn i(&self) -> Self::F
    {
        PIFilterParam::i(self)
    }
    fn d(&self) -> Self::F
    {
        Zero::zero()
    }
}