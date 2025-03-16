use num::traits::FloatConst;

use crate::{conf::{all, All, Conf, HighPass, LowPass, Peak}, param::{FilterParam, SecondOrderFilterParamBase}, params::Omega, util::same::Same};

use super::ButterworthFilterParam;

pub trait SecondOrderFilterParam<
    C,
    ImplBase = <Self as SecondOrderFilterParamBase<C>>::ImplBase
>: SecondOrderFilterParamBase<C, ImplBase: Same<ImplBase>> + FilterParam<ORDER = 2>
where
    C: Conf
{
    type Conf: SecondOrderFilterConf;

    fn omega(&self) -> Self::F;
    fn zeta(&self) -> Self::F;
}

impl<P, C> SecondOrderFilterParam<C, Omega<P::F, 2>> for P
where
    P: ButterworthFilterParam<C, Conf: SecondOrderFilterConf> + FilterParam<ORDER = 2> + SecondOrderFilterParamBase<C, ImplBase = Omega<<P as FilterParam>::F, 2>>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        ButterworthFilterParam::omega(self)
    }
    fn zeta(&self) -> Self::F
    {
        FloatConst::FRAC_1_SQRT_2()
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
    use crate::{filters::iir::second::{SecondOrderButterworthFilter, SecondOrderFilter}, param::{ButterworthFilterConf, ButterworthFilterParam}, params::{OmegaSecondOrder, OmegaZeta}, rtf::Rtf};

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
        OmegaZeta<f64>: SecondOrderFilterParam<CC, Conf = CC>,
        OmegaZeta<f32>: SecondOrderFilterParam<CC, Conf = CC>,
        SecondOrderFilter<f64, OmegaZeta<f64>, C>: Rtf,
        SecondOrderFilter<f32, OmegaZeta<f32>, C>: Rtf,
        [(); <<CC as SecondOrderFilterConf>::Conf as SecondOrderFilterConf>::OUTPUTS]:,

        OmegaSecondOrder<f64>: ButterworthFilterParam<CC, Conf = CC>,
        OmegaSecondOrder<f32>: ButterworthFilterParam<CC, Conf = CC>,
        SecondOrderButterworthFilter<f64, OmegaSecondOrder<f64>, C>: Rtf,
        SecondOrderButterworthFilter<f32, OmegaSecondOrder<f32>, C>: Rtf,
        [(); <<CC as ButterworthFilterConf<2>>::Conf as ButterworthFilterConf<2>>::OUTPUTS]:
    {

    }
}