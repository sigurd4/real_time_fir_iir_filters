use crate::{conf::Conf, param::{Omega, OmegaFirstOrder}};

use super::{ButterworthFilterConf, ButterworthFilterParam, FirstOrderFilterParam};

pub trait FirstOrderButterworthFilterParam<C>: FirstOrderFilterParam<C>
where
    C: Conf
{
    type Conf: FirstOrderButterworthFilterConf;

    fn omega(&self) -> OmegaFirstOrder<Self::F>;
}
impl<P, C> FirstOrderButterworthFilterParam<C> for P
where
    P: ButterworthFilterParam<C, Conf: FirstOrderButterworthFilterConf, ORDER = 1> + FirstOrderFilterParam<C>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = <Self as ButterworthFilterParam<C>>::Conf;

    fn omega(&self) -> OmegaFirstOrder<Self::F>
    {
        let Omega {omega} = ButterworthFilterParam::omega(self);
        Omega {omega}
    }
}

pub trait FirstOrderButterworthFilterConf = ButterworthFilterConf<1>;