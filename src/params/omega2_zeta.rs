use crate::{conf::All, param::{FilterFloat, FilterParam, FilterParamThirdOrder, ThirdOrderFilterConf, ThirdOrderFilterParam, ThirdOrderFilterParamBase}, real_time_fir_iir_filters, util::same::NotSame};

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
impl<F> FilterParamThirdOrder for Omega2Zeta<F>
where
    F: FilterFloat
{
    
}
impl<F, C> ThirdOrderFilterParamBase<C> for Omega2Zeta<F>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderFilterParam<C, Omega2Zeta<F>> for Omega2Zeta<F>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type Conf = C;

    fn omega1(&self) -> Self::F
    {
        *self.omega1
    }
    fn omega2(&self) -> Self::F
    {
        *self.omega2
    }
    fn zeta(&self) -> Self::F
    {
        *self.zeta
    }
}
impl<P> From<P> for Omega2Zeta<P::F>
where
    P: ThirdOrderFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        Self::new(value.omega1(), value.omega2(), value.zeta())
    }
}