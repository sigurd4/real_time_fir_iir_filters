use num::Zero;

use crate::param::{FilterParam, PID, PI};

use super::PIFilterParam;

pub trait PIDFilterParam: FilterParam
{
    fn pid(&self) -> PID<Self::F>;
}
impl<P> PIDFilterParam for P
where
    P: PIFilterParam
{
    fn pid(&self) -> PID<Self::F>
    {
        let PI {p, i} = self.pi();
        PID {
            p,
            i,
            d: Zero::zero()
        }
    }
}