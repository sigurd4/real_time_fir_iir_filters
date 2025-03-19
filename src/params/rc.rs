use crate::{param::{FilterFloat, FilterParam, FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParamBase, FirstOrderFilterParamBase, FirstOrderRCFilterConf, FirstOrderRCFilterParam, RCVal, SecondOrderRCFilterParamBase, SecondOrderRLCFilterParamBase, ThirdOrderSallenKeyFilterParamBase}, real_time_fir_iir_filters};

crate::def_param!(
    RC<F> {
        r: F,
        c: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for RC<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRLCFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRCFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderRCFilterParam<C> for RC<F>
where
    F: FilterFloat,
    C: FirstOrderRCFilterConf
{
    type Conf = C;

    fn rc(&self) -> RCVal<Self::F>
    {
        RCVal {
            r: *self.r,
            c: *self.c
        }
    }
}
/*impl<P> From<P> for RC<P::F>
where
    P: FirstOrderRCFilterParam<All, Conf = All> + NotSame<RC<P::F>>
{
    fn from(value: P) -> Self
    {
        let r = value.r();
        let c = value.c();
        RC::new(r, c)
    }
}*/