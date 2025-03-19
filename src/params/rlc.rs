use crate::{param::{FilterFloat, FilterParam, RLCVal, SecondOrderRLCFilterConf, SecondOrderRLCFilterParam, SecondOrderRLCFilterParamBase}, real_time_fir_iir_filters};

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

    fn rlc(&self) -> RLCVal<Self::F>
    {
        RLCVal {
            r: *self.r,
            l: *self.l,
            c: *self.c
        }
    }
}