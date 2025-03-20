use num::Float;

use crate::param::{FilterFloat, FilterParam, Param, SecondOrderFilterConf, SecondOrderFilterParam, SecondOrderFilterParamBase};

#[derive(Clone, Copy, Debug)]
pub struct OmegaZeta<F>
where
    F: Float
{
    pub omega: F,
    pub zeta: F
}
impl<F> FilterParam for Param<OmegaZeta<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 2;

    type F = F;
}
impl<F, C> SecondOrderFilterParamBase<C> for Param<OmegaZeta<F>>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> SecondOrderFilterParam<C, Self> for Param<OmegaZeta<F>>
where
    F: FilterFloat,
    C: SecondOrderFilterConf
{
    type Conf = C;

    fn omega_zeta(&self) -> OmegaZeta<Self::F>
    {**self
    }
}