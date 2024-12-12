use crate::{conf::All, param::{FilterFloat, FilterParam, FilterParamSecondOrder, SecondOrderFilterConf, SecondOrderFilterParam, SecondOrderFilterParamBase}, real_time_fir_iir_filters, util::same::NotSame};

crate::def_param!(
    OmegaZeta<F> {
        omega: F,
        zeta: F
    } where
        F: FilterFloat
);
impl<F> FilterParam for OmegaZeta<F>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F> FilterParamSecondOrder for OmegaZeta<F>
where
    F: FilterFloat
{
    
}
impl<F, C> SecondOrderFilterParamBase<C> for OmegaZeta<F>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderFilterParam<C, OmegaZeta<F>> for OmegaZeta<F>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type Conf = C;

    fn omega(&self) -> Self::F
    {
        *self.omega
    }
    fn zeta(&self) -> Self::F
    {
        *self.zeta
    }
}
impl<P> From<P> for OmegaZeta<P::F>
where
    P: SecondOrderFilterParam<All, Conf = All> + NotSame<Self>
{
    fn from(value: P) -> Self
    {
        Self::new(value.omega(), value.zeta())
    }
}