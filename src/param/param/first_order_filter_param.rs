use num::Float;

use crate::{conf::{all, All, Conf, HighPass, LowPass}, param::{FilterParam, FilterParamFirstOrder, FirstOrderFilterParamBase}, params::{Omega, OmegaFirstOrder, LR, RC}, util::same::Same};

use super::{ButterworthFilterParam, FirstOrderLRFilterParam, FirstOrderRCFilterParam};

pub trait FirstOrderFilterParam<
    C,
    ImplBase = <Self as FirstOrderFilterParamBase<C>>::ImplBase
>: FirstOrderFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: FirstOrderFilterConf;

    fn omega(&self) -> Self::F;
}

impl<P, C> FirstOrderFilterParam<C, OmegaFirstOrder<P::F>> for P
where
    P: ButterworthFilterParam<C, Conf: FirstOrderFilterConf> + FilterParamFirstOrder + FirstOrderFilterParamBase<C, ImplBase = Omega<<P as FilterParam>::F, 1>>,
    [(); <P as FilterParam>::ORDER]:,
    C: Conf
{
    type Conf = P::Conf;

    #[doc(hidden)]
    fn omega(&self) -> Self::F
    {
        ButterworthFilterParam::omega(self)
    }
}
impl<P, C> FirstOrderFilterParam<C, RC<P::F>> for P
where
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        let r = self.r();
        let c = self.c();
        (r*c).recip()
    }
}
impl<P, C> FirstOrderFilterParam<C, LR<P::F>> for P
where
    P: FirstOrderLRFilterParam<C>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        let l = self.l();
        let r = self.r();
        r/l
    }
}

pub trait FirstOrderFilterConf: Conf
{
    type Conf: private::FirstOrderFilterConfFinal<Self>;

    const OUTPUTS: usize;
}

impl FirstOrderFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}
impl FirstOrderFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as FirstOrderFilterConf>::OUTPUTS $(+ <$more as FirstOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as FirstOrderFilterConf>::OUTPUTS $(+ <$more as FirstOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf0:ty, $($more:ty),+ $(=> $($actual:ty),+)?) => {
        impl_composite_conf!(
            all!(
                $conf0,
                $($more),*
            ): $conf0, $($more),* $(=> $($actual),+)?
        );
    }
}

impl_composite_conf!(All: LowPass, HighPass);

impl_composite_conf!(LowPass, HighPass => All);

mod private
{
    use crate::{filters::iir::first::FirstOrderFilter, params::OmegaFirstOrder, rtf::Rtf};

    use super::{FirstOrderFilterConf, FirstOrderFilterParam};

    pub trait FirstOrderFilterConfFinal<C>: FirstOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: FirstOrderFilterConf
    {

    }
    impl<
        CC,
        C,
        const OUTPUTS: usize
    > FirstOrderFilterConfFinal<C> for CC
    where
        CC: FirstOrderFilterConf<
            Conf = CC,
            OUTPUTS = {OUTPUTS}
        >,
        C: FirstOrderFilterConf<
            Conf = CC::Conf,
            OUTPUTS = {OUTPUTS}
        >,
        OmegaFirstOrder<f64>: FirstOrderFilterParam<CC, Conf = CC>,
        OmegaFirstOrder<f32>: FirstOrderFilterParam<CC, Conf = CC>,
        FirstOrderFilter<f64, OmegaFirstOrder<f64>, CC>: Rtf,
        FirstOrderFilter<f32, OmegaFirstOrder<f32>, CC>: Rtf,
        [(); <<CC as FirstOrderFilterConf>::Conf as FirstOrderFilterConf>::OUTPUTS]:
    {

    }
}