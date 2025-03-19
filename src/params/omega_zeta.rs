use crate::{param::{FilterFloat, FilterParam, OmegaZetaVal, SecondOrderFilterConf, SecondOrderFilterParam, SecondOrderFilterParamBase}, real_time_fir_iir_filters};

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
impl<F, C> SecondOrderFilterParamBase<C> for OmegaZeta<F>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderFilterParam<C, Self> for OmegaZeta<F>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type Conf = C;

    fn omega_zeta(&self) -> OmegaZetaVal<Self::F>
    {
        OmegaZetaVal {
            omega: *self.omega,
            zeta: *self.zeta
        }
    }
}