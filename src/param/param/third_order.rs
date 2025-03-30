use num::NumCast;

use crate::{conf::Conf, param::{FilterParam, Omega, Omega2Zeta, OmegaThirdOrder, Param, ThirdOrderFilterConf, ThirdOrderFilterParamBase}, util::same::Same};

use super::ButterworthFilterParam;

pub trait ThirdOrderFilterParam<
    C,
    ImplBase = <Self as ThirdOrderFilterParamBase<C>>::ImplBase
>: ThirdOrderFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: ThirdOrderFilterConf;

    fn omega2_zeta(&self) -> Omega2Zeta<Self::F>;
}

impl<P, C> ThirdOrderFilterParam<C, Param<OmegaThirdOrder<P::F>>> for P
where
    P: ButterworthFilterParam<C, Conf: ThirdOrderFilterConf, Omega = OmegaThirdOrder<<P as FilterParam>::F>> + ThirdOrderFilterParamBase<C, ImplBase = Param<OmegaThirdOrder<<P as FilterParam>::F>>>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = P::Conf;

    fn omega2_zeta(&self) -> Omega2Zeta<Self::F>
    {
        let Omega {omega} = self.omega();
        Omega2Zeta {
            omega1: omega,
            omega2: omega,
            zeta: NumCast::from(0.5).unwrap()
        }
    }
}