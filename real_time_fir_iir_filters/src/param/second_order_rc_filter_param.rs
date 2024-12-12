use num::Zero;

use crate::{conf::{all, All, BandPass, Conf, HighPass, InputOrGND, LowPass}, params::RC, util::same::Same};

use super::{FilterParam, FirstOrderRCFilterConf, FirstOrderRCFilterParam};

pub trait SecondOrderRCFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: SecondOrderRCFilterParamBase<All, ImplBase = Self::ImplBase>;
}

pub trait SecondOrderRCFilterParam<
    C,
    ImplBase = <Self as SecondOrderRCFilterParamBase<C>>::ImplBase
>: SecondOrderRCFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: SecondOrderRCFilterConf;

    fn r1(&self) -> Self::F;
    fn c1(&self) -> Self::F;
    fn r2(&self) -> Self::F;
    fn c2(&self) -> Self::F;
}

impl<P, C> SecondOrderRCFilterParam<C, RC<P::F>> for P
where
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as FirstOrderRCFilterConf>::AsSecondOrderRCFilterConf;

    fn r1(&self) -> Self::F
    {
        FirstOrderRCFilterParam::r(self)
    }
    fn c1(&self) -> Self::F
    {
        FirstOrderRCFilterParam::c(self)
    }
    fn r2(&self) -> Self::F
    {
        Zero::zero()
    }
    fn c2(&self) -> Self::F
    {
        Zero::zero()
    }
}

pub trait SecondOrderRCFilterConf: Conf
{
    type Conf: private::SecondOrderRCFilterConfFinal<Self>;

    const OUTPUTS: usize;

    type S1Conf: private::S1ConfForSecondOrderRCFilterConf<Self>;
    type S2Conf: private::S2ConfForSecondOrderRCFilterConf<Self>;

    const R1_CONF: InputOrGND = <Self::S1Conf as FirstOrderRCFilterConf>::R_CONF;
    const C1_CONF: InputOrGND = <Self::S1Conf as FirstOrderRCFilterConf>::C_CONF;
    const R2_CONF: InputOrGND = <Self::S2Conf as FirstOrderRCFilterConf>::R_CONF;
    const C2_CONF: InputOrGND = <Self::S2Conf as FirstOrderRCFilterConf>::C_CONF;
}

impl SecondOrderRCFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    type S1Conf = LowPass;
    type S2Conf = LowPass;
}
impl SecondOrderRCFilterConf for BandPass<1>
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    type S1Conf = HighPass;
    type S2Conf = LowPass;
}
impl SecondOrderRCFilterConf for BandPass<2>
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    type S1Conf = LowPass;
    type S2Conf = HighPass;
}
impl SecondOrderRCFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    type S1Conf = HighPass;
    type S2Conf = HighPass;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),*) => {
        impl SecondOrderRCFilterConf for $conf
        {
            type Conf = all!($($actual),*);
            
            const OUTPUTS: usize = <$conf0 as SecondOrderRCFilterConf>::OUTPUTS $(+ <$more as SecondOrderRCFilterConf>::OUTPUTS)*;

            type S1Conf = <all!(
                <$conf0 as SecondOrderRCFilterConf>::S1Conf,
                $(<$more as SecondOrderRCFilterConf>::S1Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
            type S2Conf = <all!(
                <$conf0 as SecondOrderRCFilterConf>::S2Conf,
                $(<$more as SecondOrderRCFilterConf>::S2Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl SecondOrderRCFilterConf for $conf
        {
            type Conf = $conf;
            
            const OUTPUTS: usize = <$conf0 as SecondOrderRCFilterConf>::OUTPUTS $(+ <$more as SecondOrderRCFilterConf>::OUTPUTS)*;

            type S1Conf = <all!(
                <$conf0 as SecondOrderRCFilterConf>::S1Conf,
                $(<$more as SecondOrderRCFilterConf>::S1Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
            type S2Conf = <all!(
                <$conf0 as SecondOrderRCFilterConf>::S2Conf,
                $(<$more as SecondOrderRCFilterConf>::S2Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
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

impl_composite_conf!(BandPass: BandPass<1>, BandPass<2>);
impl_composite_conf!(LowPass, BandPass);
impl_composite_conf!(BandPass, HighPass);
impl_composite_conf!(LowPass, BandPass, HighPass => All);

impl_composite_conf!(All: LowPass, BandPass, HighPass);

impl_composite_conf!(LowPass, BandPass<1>);
impl_composite_conf!(LowPass, BandPass<2>);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2> => BandPass);
impl_composite_conf!(BandPass<1>, HighPass);
impl_composite_conf!(BandPass<2>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2> => LowPass, BandPass);
impl_composite_conf!(LowPass, BandPass<1>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, HighPass => BandPass, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, HighPass => All);

mod private
{
    use crate::{conf::InputOrGND, filters::iir::second::SecondOrderRCFilter, param::FirstOrderRCFilterConf, params::RC2, rtf::Rtf};

    use super::{SecondOrderRCFilterConf, SecondOrderRCFilterParam};

    pub trait SecondOrderRCFilterConfFinal<C>: SecondOrderRCFilterConf<
        Conf = C::Conf,
        S1Conf = C::S1Conf,
        S2Conf = C::S2Conf
    >
    where
        C: SecondOrderRCFilterConf
    {

    }
    impl<
        CC,
        C,
        const R1_CONF: InputOrGND,
        const C1_CONF: InputOrGND,
        const R2_CONF: InputOrGND,
        const C2_CONF: InputOrGND
    > SecondOrderRCFilterConfFinal<C> for CC
    where
        CC: SecondOrderRCFilterConf<
            Conf = C::Conf,
            S1Conf = C::S1Conf,
            S2Conf = C::S2Conf,
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >,
        C: SecondOrderRCFilterConf<
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >,
        RC2<f64>: SecondOrderRCFilterParam<CC, Conf = CC>,
        RC2<f32>: SecondOrderRCFilterParam<CC, Conf = CC>,
        SecondOrderRCFilter<f64, RC2<f64>, C>: Rtf,
        SecondOrderRCFilter<f32, RC2<f32>, C>: Rtf,
        [(); <<CC as SecondOrderRCFilterConf>::Conf as SecondOrderRCFilterConf>::OUTPUTS]:,
    {

    }

    pub trait S1ConfForSecondOrderRCFilterConf<C>: FirstOrderRCFilterConf
    where
        C: SecondOrderRCFilterConf<
            S1Conf = Self
        >
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > S1ConfForSecondOrderRCFilterConf<C> for CC
    where
        CC: FirstOrderRCFilterConf<
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >,
        C: SecondOrderRCFilterConf<
            S1Conf = CC,
            R1_CONF = {R_CONF},
            C1_CONF = {C_CONF}
        >
    {

    }
    
    pub trait S2ConfForSecondOrderRCFilterConf<C>: FirstOrderRCFilterConf
    where
        C: SecondOrderRCFilterConf<
            S2Conf = Self
        >
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > S2ConfForSecondOrderRCFilterConf<C> for CC
    where
        CC: FirstOrderRCFilterConf<
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >,
        C: SecondOrderRCFilterConf<
            S2Conf = CC,
            R2_CONF = {R_CONF},
            C2_CONF = {C_CONF}
        >
    {

    }
}