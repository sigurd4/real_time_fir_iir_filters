use num::{Float, One};

use crate::{conf::{all, All, Conf, HighPass, LowPass, Peak}, param::{FilterParam, ThirdOrderFilterParamBase}, params::OmegaThirdOrder, util::same::Same};

use super::ButterworthFilterParam;

pub trait ThirdOrderFilterParam<
    C,
    ImplBase = <Self as ThirdOrderFilterParamBase<C>>::ImplBase
>: ThirdOrderFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: ThirdOrderFilterConf;

    fn omega1(&self) -> Self::F;
    fn omega2(&self) -> Self::F;
    fn zeta(&self) -> Self::F;
}

impl<P, C> ThirdOrderFilterParam<C, OmegaThirdOrder<P::F>> for P
where
    P: ButterworthFilterParam<C, Conf: ThirdOrderFilterConf> + ThirdOrderFilterParamBase<C, ImplBase = OmegaThirdOrder<<P as FilterParam>::F>>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega1(&self) -> Self::F
    {
        ButterworthFilterParam::omega(self)
    }
    fn omega2(&self) -> Self::F
    {
        ButterworthFilterParam::omega(self)
    }
    fn zeta(&self) -> Self::F
    {
        let one = <Self::F as One>::one();
        (one + one).recip()
    }
}

pub trait ThirdOrderFilterConf: Conf
{
    type Conf: private::ThirdOrderFilterConfFinal<Self>;

    const OUTPUTS: usize;
}
impl ThirdOrderFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}
impl ThirdOrderFilterConf for Peak<1>
{
    type Conf = Self;
    
    const OUTPUTS: usize = 1;
}
impl ThirdOrderFilterConf for Peak<2>
{
    type Conf = Self;
    
    const OUTPUTS: usize = 1;
}
impl ThirdOrderFilterConf for HighPass
{
    type Conf = Self;
    
    const OUTPUTS: usize = 1;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl ThirdOrderFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as ThirdOrderFilterConf>::OUTPUTS $(+ <$more as ThirdOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl ThirdOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as ThirdOrderFilterConf>::OUTPUTS $(+ <$more as ThirdOrderFilterConf>::OUTPUTS)*;
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

impl_composite_conf!(Peak: Peak<1>, Peak<2>);
impl_composite_conf!(LowPass, Peak);
impl_composite_conf!(Peak, HighPass);
impl_composite_conf!(LowPass, Peak, HighPass => All);

impl_composite_conf!(All: LowPass, Peak, HighPass);

impl_composite_conf!(LowPass, Peak<1>);
impl_composite_conf!(LowPass, Peak<2>);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(Peak<1>, Peak<2> => Peak);
impl_composite_conf!(Peak<1>, HighPass);
impl_composite_conf!(Peak<2>, HighPass);

impl_composite_conf!(LowPass, Peak<1>, Peak<2> => LowPass, Peak);
impl_composite_conf!(LowPass, Peak<1>, HighPass);
impl_composite_conf!(LowPass, Peak<2>, HighPass);
impl_composite_conf!(Peak<1>, Peak<2>, HighPass => Peak, HighPass);

impl_composite_conf!(LowPass, Peak<1>, Peak<2>, HighPass => All);

mod private
{
    use crate::{filters::iir::third::ThirdOrderButterworthFilter, param::{ButterworthFilterConf, ButterworthFilterParam}, params::{Omega2Zeta, OmegaThirdOrder}, rtf::Rtf};

    use super::{ThirdOrderFilterConf, ThirdOrderFilterParam};

    pub trait ThirdOrderFilterConfFinal<C>: ThirdOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: ThirdOrderFilterConf
    {

    }
    impl<
        CC,
        C,
        const OUTPUTS: usize
    > ThirdOrderFilterConfFinal<C> for CC
    where
        CC: ThirdOrderFilterConf<
            Conf = C::Conf,
            OUTPUTS = {OUTPUTS}
        > + ButterworthFilterConf<
            3,
            Conf = C::Conf,
            OUTPUTS = {OUTPUTS}
        >,
        C: ThirdOrderFilterConf<
            OUTPUTS = {OUTPUTS}
        >,
        Omega2Zeta<f64>: ThirdOrderFilterParam<CC, Conf = CC>,
        Omega2Zeta<f32>: ThirdOrderFilterParam<CC, Conf = CC>,
        //ThirdOrderFilter<f64, OmegaZeta<f64>, C>: Rtf,
        //ThirdOrderFilter<f32, OmegaZeta<f32>, C>: Rtf,
        [(); <<CC as ThirdOrderFilterConf>::Conf as ThirdOrderFilterConf>::OUTPUTS]:,

        OmegaThirdOrder<f64>: ButterworthFilterParam<CC, Conf = CC>,
        OmegaThirdOrder<f32>: ButterworthFilterParam<CC, Conf = CC>,
        ThirdOrderButterworthFilter<f64, OmegaThirdOrder<f64>, C>: Rtf,
        ThirdOrderButterworthFilter<f32, OmegaThirdOrder<f32>, C>: Rtf,
        [(); <<CC as ButterworthFilterConf<3>>::Conf as ButterworthFilterConf<3>>::OUTPUTS]:
    {

    }
}