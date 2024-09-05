use crate::{param::{FilterFloat, FilterParam, PIFilterParam}, util::same::NotSame, real_time_fir_iir_filters};

crate::def_param!(
    PI<F> {
        p: F,
        i: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for PI<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F> PIFilterParam for PI<F>
where
    F: FilterFloat
{
    fn p(&self) -> Self::F
    {
        *self.p
    }
    fn i(&self) -> Self::F
    {
        *self.i
    }
}
impl<P> From<P> for PI<P::F>
where
    P: PIFilterParam + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        PI::new(value.p(), value.i())
    }
}
