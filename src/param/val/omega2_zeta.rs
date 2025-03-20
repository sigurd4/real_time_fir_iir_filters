use num::Float;

use crate::param::{FilterFloat, FilterParam, Param, ThirdOrderFilterConf, ThirdOrderFilterParam, ThirdOrderFilterParamBase};

#[derive(Clone, Copy, Debug)]
pub struct Omega2Zeta<F>
where
    F: Float
{
    pub omega1: F,
    pub omega2: F,
    pub zeta: F
}
impl<F> FilterParam for Param<Omega2Zeta<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 3;

    type F = F;
}
impl<F, C> ThirdOrderFilterParamBase<C> for Param<Omega2Zeta<F>>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type ImplBase = Self;
}
impl<F, C> ThirdOrderFilterParam<C, Self> for Param<Omega2Zeta<F>>
where
    F: FilterFloat,
    C: ThirdOrderFilterConf
{
    type Conf = C;

    fn omega2_zeta(&self) -> Omega2Zeta<Self::F>
    {
        **self
    }
}