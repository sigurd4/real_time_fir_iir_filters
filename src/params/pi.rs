use crate::{param::{FilterFloat, FilterParam, PIFilterParam, PIVal}, real_time_fir_iir_filters};

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
    fn pi(&self) -> PIVal<Self::F>
    {
        PIVal {
            p: *self.p,
            i: *self.i
        }
    }
}