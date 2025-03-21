use crate::{conf::Conf, param::{Omega, OmegaSecondOrder}};

use super::{ButterworthFilterConf, ButterworthFilterParam, SecondOrderFilterParam};

pub trait SecondOrderButterworthFilterParam<C>: SecondOrderFilterParam<C>
where
    C: Conf
{
    type Conf: SecondOrderButterworthFilterConf;

    fn omega(&self) -> OmegaSecondOrder<Self::F>;
}
impl<P, C> SecondOrderButterworthFilterParam<C> for P
where
    P: ButterworthFilterParam<C, Conf: SecondOrderButterworthFilterConf, ORDER = 2> + SecondOrderFilterParam<C>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = <Self as ButterworthFilterParam<C>>::Conf;

    fn omega(&self) -> OmegaSecondOrder<Self::F>
    {
        let Omega {omega} = ButterworthFilterParam::omega(self);
        Omega {omega}
    }
}

pub trait SecondOrderButterworthFilterConf = ButterworthFilterConf<2>;