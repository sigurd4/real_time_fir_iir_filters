use num::Float;

use crate::param::Param;

pub trait Change<T = Self>
{
    type F: Float;

    fn change(&mut self, to: T, change: Self::F);
}
impl<F> Change for F
where
    F: Float
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        *self = (F::one() - change)**self + change*to
    }
}
impl<T, P> Change<T> for Param<P>
where
    P: Change<T>
{
    type F = P::F;

    fn change(&mut self, to: T, change: Self::F)
    {
        (**self).change(to, change);
    }
}