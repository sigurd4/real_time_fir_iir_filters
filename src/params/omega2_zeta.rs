use crate::{param::{FilterFloat, FilterParam, Omega2ZetaVal, ThirdOrderFilterConf, ThirdOrderFilterParam, ThirdOrderFilterParamBase}, real_time_fir_iir_filters};

crate::def_param!(
    Omega2Zeta<F> {
        omega1: F,
        omega2: F,
        zeta: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for Omega2Zeta<F>
where
    F: FilterFloat
{
    const ORDER: usize = 3;

    type F = F;
}
impl<F, C> ThirdOrderFilterParamBase<C> for Omega2Zeta<F>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderFilterParam<C, Self> for Omega2Zeta<F>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type Conf = C;

    fn omega2_zeta(&self) -> Omega2ZetaVal<Self::F>
    {
        Omega2ZetaVal {
            omega1: *self.omega1,
            omega2: *self.omega2,
            zeta: *self.zeta
        }
    }
}
/*impl<P> From<P> for Omega2Zeta<P::F>
where
    P: ThirdOrderFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        Self::new(value.omega1(), value.omega2(), value.zeta())
    }
}*/