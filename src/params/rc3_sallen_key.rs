use num::One;

use crate::{param::{FilterFloat, FilterParam, RC3GVal, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}, real_time_fir_iir_filters};

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

    fn rc3g(&self) -> RC3GVal<Self::F>
    {
        RC3GVal {
            r1: *self.r1,
            c1: *self.c1,
            r2: *self.r2,
            c2: *self.c2,
            r3: *self.r3,
            c3: *self.c3,
            g: One::one()
        }
    }
}