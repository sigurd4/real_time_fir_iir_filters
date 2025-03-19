use crate::real_time_fir_iir_filters;
use crate::param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam, FirstOrderAllPassFilterParamBase};

crate::def_param!(
    Tau(TauVal)<F> {
        tau: F
    }
    where
        F: FilterFloat
);
impl<F> FilterParam for Tau<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for Tau<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderAllPassFilterParam<C> for Tau<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type Conf = C;

    fn tau(&self) -> TauVal<F>
    {
        TauVal {tau: *self.tau}
    }
}