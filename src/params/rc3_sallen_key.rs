use crate::{param::{FilterFloat, FilterParam, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}, real_time_fir_iir_filters};

crate::def_param!(
    RC3SallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F,
        r3: F,
        c3: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for RC3SallenKey<F>
where
    F: FilterFloat
{
    const ORDER: usize = 3;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for RC3SallenKey<F>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderSallenKeyFilterParam<C, RC3SallenKey<F>> for RC3SallenKey<F>
where
    F: FilterFloat,
    C: ThirdOrderSallenKeyFilterConf
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
    fn r3(&self) -> Self::F
    {
        *self.r3
    }
    fn c3(&self) -> Self::F
    {
        *self.c3
    }
}