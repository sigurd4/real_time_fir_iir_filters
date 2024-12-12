use crate::{param::{FilterFloat, FilterParam, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}, real_time_fir_iir_filters};

use super::RC2GSallenKey;

crate::def_param!(
    RC2SallenKey<F> {
        r1: F,
        c1: F,
        r2: F,
        c2: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for RC2SallenKey<F>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> ThirdOrderSallenKeyFilterParamBase<C> for RC2SallenKey<F>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
{
    type ImplBase = RC2GSallenKey<F>;
}
impl<F, C> SecondOrderSallenKeyFilterParam<C> for RC2SallenKey<F>
where
    F: FilterFloat,
    C: SecondOrderSallenKeyFilterConf
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