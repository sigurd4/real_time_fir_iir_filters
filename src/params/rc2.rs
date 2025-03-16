use crate::{conf::All, param::{FilterFloat, FilterParam, SecondOrderRCFilterConf, SecondOrderRCFilterParam, SecondOrderRCFilterParamBase}, util::same::NotSame, real_time_fir_iir_filters};

crate::def_param!(
    RC2<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for RC2<F>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> SecondOrderRCFilterParamBase<C> for RC2<F>
where
    F: FilterFloat,
    C: SecondOrderRCFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderRCFilterParam<C, RC2<F>> for RC2<F>
where
    F: FilterFloat,
    C: SecondOrderRCFilterConf
{
    type Conf = C;

    fn r1(&self) -> Self::F
    {
        *self.r1
    }

    fn c1(&self) -> Self::F
    {
        *self.c1
    }

    fn r2(&self) -> Self::F
    {
        *self.r2
    }

    fn c2(&self) -> Self::F
    {
        *self.c2
    }
}
impl<P> From<P> for RC2<P::F>
where
    P: SecondOrderRCFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        let r1 = value.r1();
        let c1 = value.c1();
        let r2 = value.r2();
        let c2 = value.c2();
        Self::new(r1, c1, r2, c2)
    }
}