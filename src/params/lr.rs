use crate::{param::{FilterFloat, FilterParam, FirstOrderFilterParamBase, FirstOrderLRFilterConf, FirstOrderLRFilterParam, LRVal}, real_time_fir_iir_filters};

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

    fn lr(&self) -> LRVal<Self::F>
    {
        LRVal {
            l: *self.l,
            r: *self.r
        }
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