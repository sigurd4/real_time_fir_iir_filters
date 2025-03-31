use num::Float;
use serde::{Serialize, Deserialize};

use crate::{change::Change, param::{FilterFloat, FilterParam, Param, ThirdOrderFilterConf, ThirdOrderFilterParam, ThirdOrderFilterParamBase}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Omega2Zeta<F>
where
    F: Float
{
    pub omega1: F,
    pub omega2: F,
    pub zeta: F
}
impl<F> Change for Omega2Zeta<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.omega1.change(to.omega1, change);
        self.omega2.change(to.omega2, change);
        self.zeta.change(to.zeta, change);
    }
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