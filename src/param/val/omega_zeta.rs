use crate::{change::Change, param::{FilterFloat, FilterParam, SecondOrderFilterConf, SecondOrderFilterParam, SecondOrderFilterParamBase}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct OmegaZeta<F>
where
    F: FilterFloat
{
    pub omega: F,
    pub zeta: F
}
impl<F> Change for OmegaZeta<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.omega.change(to.omega, change);
        self.zeta.change(to.zeta, change);
    }
}
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

    fn omega_zeta(&self) -> OmegaZeta<Self::F>
    {
        *self
    }
}