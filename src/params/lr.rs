use crate::{conf::All, param::{FilterFloat, FilterParam, FilterParamFirstOrder, FirstOrderFilterParamBase, FirstOrderLRFilterConf, FirstOrderLRFilterParam}, real_time_fir_iir_filters, util::same::NotSame};

crate::def_param!(
    LR<F> {
        l: F,
        r: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for LR<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F> FilterParamFirstOrder for LR<F>
where
    F: FilterFloat
{
    
}
impl<F, C> FirstOrderFilterParamBase<C> for LR<F>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderLRFilterParam<C> for LR<F>
where
    F: FilterFloat,
    C: FirstOrderLRFilterConf
{
    type Conf = C;

    fn l(&self) -> Self::F
    {
        *self.l
    }
    fn r(&self) -> Self::F
    {
        *self.r
    }
}
/*impl<P> From<P> for LR<P::F>
where
    P: NotSame<LR<P::F>> + FirstOrderLRFilterParam<All, Conf = All>
{
    fn from(value: P) -> Self
    {
        let l = value.l();
        let r = value.r();
        LR::new(l, r)
    }
}*/