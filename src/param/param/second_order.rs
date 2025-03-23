use num::traits::FloatConst;

use crate::{conf::{all, All, Conf, HighPass, LowPass, Peak}, param::{FilterParam, Omega, OmegaSecondOrder, OmegaZeta, Param, SecondOrderFilterParamBase}, util::same::Same};

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

pub trait SecondOrderFilterConf: Conf
{
    type Conf: private::SecondOrderFilterConfFinal<Self>;

    const OUTPUTS: usize;
}
impl SecondOrderFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}
impl SecondOrderFilterConf for Peak
{
    type Conf = Self;
    
    const OUTPUTS: usize = 1;
}
impl SecondOrderFilterConf for HighPass
{
    type Conf = Self;
    
    const OUTPUTS: usize = 1;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl SecondOrderFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as SecondOrderFilterConf>::OUTPUTS $(+ <$more as SecondOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl SecondOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as SecondOrderFilterConf>::OUTPUTS $(+ <$more as SecondOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf0:ty, $($more:ty),+ $(=> $($actual:ty),+)?) => {
        impl_composite_conf!(
            all!(
                $conf0,
                $($more),*
            ): $conf0, $($more),* $(=> $($actual),*)?
        );
    }
}

impl_composite_conf!(All: LowPass, Peak, HighPass);

impl_composite_conf!(LowPass, Peak);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(Peak, HighPass);
impl_composite_conf!(LowPass, Peak, HighPass => All);

mod private
{
    use crate::param::{ButterworthFilterConf, OmegaZeta, Param};

    use super::{SecondOrderFilterConf, SecondOrderFilterParam};

    pub trait SecondOrderFilterConfFinal<C>: SecondOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: SecondOrderFilterConf
    {

    }
    impl<
        CC,
        C,
        const OUTPUTS: usize
    > SecondOrderFilterConfFinal<C> for CC
    where
        CC: SecondOrderFilterConf<
            Conf = C::Conf,
            OUTPUTS = {OUTPUTS}
        > + ButterworthFilterConf<
            2,
            Conf = C::Conf,
            OUTPUTS = {OUTPUTS}
        >,
        C: SecondOrderFilterConf<
            OUTPUTS = {OUTPUTS}
        >,
        Param<OmegaZeta<f64>>: SecondOrderFilterParam<CC, Conf = CC>,
        Param<OmegaZeta<f32>>: SecondOrderFilterParam<CC, Conf = CC>
    {

    }
}