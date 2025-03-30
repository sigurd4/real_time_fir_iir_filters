use num::traits::FloatConst;

use crate::{conf::Conf, param::{FilterParam, Omega, OmegaSecondOrder, OmegaZeta, Param, SecondOrderFilterConf, SecondOrderFilterParamBase}, util::same::Same};

use super::ButterworthFilterParam;

pub trait SecondOrderFilterParam<
    C,
    ImplBase = <Self as SecondOrderFilterParamBase<C>>::ImplBase
>: SecondOrderFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: SecondOrderFilterConf;

    fn omega_zeta(&self) -> OmegaZeta<Self::F>;
}

impl<P, C> SecondOrderFilterParam<C, Param<OmegaSecondOrder<P::F>>> for P
where
    P: ButterworthFilterParam<C, Conf: SecondOrderFilterConf, Omega = OmegaSecondOrder<<P as FilterParam>::F>> + SecondOrderFilterParamBase<C, ImplBase = Param<OmegaSecondOrder<<P as FilterParam>::F>>>,
    C: Conf,
    [(); P::ORDER]:
{
    type Conf = P::Conf;

    fn omega_zeta(&self) -> OmegaZeta<Self::F>
    {
        let Omega {omega} = self.omega();
        OmegaZeta {
            omega,
            zeta: FloatConst::FRAC_1_SQRT_2()
        }
    }
}