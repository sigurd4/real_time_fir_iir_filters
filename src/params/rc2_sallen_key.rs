use num::One;

use crate::{param::{FilterFloat, FilterParam, RC2GVal, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}, real_time_fir_iir_filters};

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

    fn rc2g(&self) -> RC2GVal<Self::F>
    {
        RC2GVal {
            r1: *self.r1,
            c1: *self.c1,
            r2: *self.r2,
            c2: *self.c2,
            g: One::one()
        }
    }
}