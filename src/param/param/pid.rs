use num::Zero;

use crate::param::FilterParam;

use super::PIFilterParam;

pub trait PIDFilterParam: FilterParam
{
    fn pid(&self) -> PIDVal<Self::F>;
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