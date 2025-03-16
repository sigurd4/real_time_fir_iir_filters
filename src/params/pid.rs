use crate::{real_time_fir_iir_filters, param::{FilterFloat, FilterParam, PIDFilterParam}, util::same::NotSame};

crate::def_param!(
    PID<F> {
        p: F,
        i: F,
        d: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for PID<F>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F> PIDFilterParam for PID<F>
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
    fn d(&self) -> Self::F
    {
        *self.d
    }
}
impl<P> From<P> for PID<P::F>
where
    P: PIDFilterParam + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        PID::new(value.p(), value.i(), value.d())
    }
}