use num::{Float, One};

use crate::{change::Change, param::{FilterFloat, FilterParam, SecondOrderSallenKeyFilterConf, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterParamBase}};

use super::RC2GSallenKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct RC2SallenKey<F>
where
    F: Float
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F
}
impl<F> Change for RC2SallenKey<F>
where
    F: FilterFloat
{
    type F = F;

    fn change(&mut self, to: Self, change: Self::F)
    {
        self.r1.change(to.r1, change);
        self.c1.change(to.c1, change);
        self.r2.change(to.r2, change);
        self.c2.change(to.c2, change);
    }
}
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

    fn rc2g(&self) -> RC2GSallenKey<Self::F>
    {
        let RC2SallenKey {r1, c1, r2, c2} = *self;
        RC2GSallenKey {
            r1,
            c1,
            r2,
            c2,
            g: One::one()
        }
    }
}