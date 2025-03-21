use crate::{conf::Conf, param::{Omega, OmegaThirdOrder}};

use super::{ButterworthFilterConf, ButterworthFilterParam, ThirdOrderFilterParam};

pub trait ThirdOrderButterworthFilterParam<C>: ThirdOrderFilterParam<C>
where
    C: Conf
{
    type Conf: ThirdOrderButterworthFilterConf;

    fn omega(&self) -> OmegaThirdOrder<Self::F>;
}
impl<P, C> ThirdOrderButterworthFilterParam<C> for P
where
    P: ButterworthFilterParam<C, Conf: ThirdOrderButterworthFilterConf, ORDER = 3> + ThirdOrderFilterParam<C>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = <Self as ButterworthFilterParam<C>>::Conf;

    fn omega(&self) -> OmegaThirdOrder<Self::F>
    {
        let Omega {omega} = ButterworthFilterParam::omega(self);
        Omega {omega}
    }
}

pub trait ThirdOrderButterworthFilterConf = ButterworthFilterConf<3>;