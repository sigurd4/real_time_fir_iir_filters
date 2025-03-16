use crate::{param::{FilterFloat, FilterParam, PIFilterParam}, real_time_fir_iir_filters};

crate::def_param!(
    PI(PIVal)<F> {
        p: F,
        i: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for PI<F>
where
    F: FilterFloat
{
    type F = F;
}
impl<F> PIFilterParam for PI<F>
where
    F: FilterFloat
{
    fn pi(&self) -> PIVal<F>
    {
        PIVal { p: *self.p.get(), i: *self.i.get() }
    }
}