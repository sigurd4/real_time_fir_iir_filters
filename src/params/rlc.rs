use crate::{conf::All, param::{FilterFloat, FilterParam, SecondOrderRLCFilterConf, SecondOrderRLCFilterParam, SecondOrderRLCFilterParamBase}, util::same::NotSame, real_time_fir_iir_filters};

crate::def_param!(
    RLC<F> {
        r: F,
        l: F,
        c: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for RLC<F>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> SecondOrderRLCFilterParamBase<C> for RLC<F>
where
    F: FilterFloat,
    C: SecondOrderRLCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRLCFilterParam<C, RLC<F>> for RLC<F>
where
    F: FilterFloat,
    C: SecondOrderRLCFilterConf
{
    type Conf = C;

    fn r(&self) -> Self::F
    {
        *self.r
    }

    fn l(&self) -> Self::F
    {
        *self.l
    }

    fn c(&self) -> Self::F
    {
        *self.c
    }
}

impl<P> From<P> for RLC<P::F>
where
    P: SecondOrderRLCFilterParam<All, Conf = All> + NotSame<RLC<P::F>>
{
    fn from(value: P) -> Self
    {
        let r = value.r();
        let l = value.l();
        let c = value.c();
        RLC::new(r, l, c)
    }
}