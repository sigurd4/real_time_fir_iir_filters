use num::Zero;

use crate::{conf::{all, All, BandPass, BandStop, Conf, HighPass, InputOrGND, LowPass}, param::SecondOrderRLCFilterParamBase, params::RC, util::same::Same};

use super::{FirstOrderRCFilterConf, FirstOrderRCFilterParam};

pub trait SecondOrderRLCFilterParam<
    C,
    ImplBase = <Self as SecondOrderRLCFilterParamBase<C>>::ImplBase
>: SecondOrderRLCFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: SecondOrderRLCFilterConf;

    fn rlc(&self) -> RLCVal<Self::F>;
}

impl<P, C> SecondOrderRLCFilterParam<C, RC<P::F>> for P
where 
    P: FirstOrderRCFilterParam<C>,
    C: Conf
{
    type Conf = <P::Conf as FirstOrderRCFilterConf>::AsSecondOrderRLCFilterConf;

    fn r(&self) -> Self::F
    {
        FirstOrderRCFilterParam::r(self)
    }
    fn l(&self) -> Self::F
    {
        Zero::zero()
    }
    fn c(&self) -> Self::F
    {
        FirstOrderRCFilterParam::c(self)
    }
}

pub trait SecondOrderRLCFilterConf: Conf
{
    type Conf: private::SecondOrderRLCFilterConfFinal<Self>;

    const OUTPUTS: usize;

    const R_CONF: InputOrGND;
    const L_CONF: InputOrGND;
    const C_CONF: InputOrGND;
}

impl SecondOrderRLCFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::Input;
    const L_CONF: InputOrGND = InputOrGND::Input;
    const C_CONF: InputOrGND = InputOrGND::GND;
}
impl SecondOrderRLCFilterConf for BandStop
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::Input;
    const L_CONF: InputOrGND = InputOrGND::GND;
    const C_CONF: InputOrGND = InputOrGND::GND;
}
impl SecondOrderRLCFilterConf for BandPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::GND;
    const L_CONF: InputOrGND = InputOrGND::Input;
    const C_CONF: InputOrGND = InputOrGND::Input;
}
impl SecondOrderRLCFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::GND;
    const L_CONF: InputOrGND = InputOrGND::GND;
    const C_CONF: InputOrGND = InputOrGND::Input;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl SecondOrderRLCFilterConf for $conf
        {
            type Conf = all!($($actual),*);

            const OUTPUTS: usize = <$conf0 as SecondOrderRLCFilterConf>::OUTPUTS $(+ <$more as SecondOrderRLCFilterConf>::OUTPUTS)*;

            const R_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::R_CONF,
                $(<$more as SecondOrderRLCFilterConf>::R_CONF),*
            ]);
            const L_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::L_CONF,
                $(<$more as SecondOrderRLCFilterConf>::L_CONF),*
            ]);
            const C_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::C_CONF,
                $(<$more as SecondOrderRLCFilterConf>::C_CONF),*
            ]);
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl SecondOrderRLCFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as SecondOrderRLCFilterConf>::OUTPUTS $(+ <$more as SecondOrderRLCFilterConf>::OUTPUTS)*;

            const R_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::R_CONF,
                $(<$more as SecondOrderRLCFilterConf>::R_CONF),*
            ]);
            const L_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::L_CONF,
                $(<$more as SecondOrderRLCFilterConf>::L_CONF),*
            ]);
            const C_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as SecondOrderRLCFilterConf>::C_CONF,
                $(<$more as SecondOrderRLCFilterConf>::C_CONF),*
            ]);
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

impl_composite_conf!(All: LowPass, BandStop, BandPass, HighPass);

impl_composite_conf!(LowPass, BandStop);
impl_composite_conf!(LowPass, BandPass);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(BandStop, BandPass);
impl_composite_conf!(BandStop, HighPass);
impl_composite_conf!(BandPass, HighPass);

impl_composite_conf!(LowPass, BandStop, BandPass);
impl_composite_conf!(LowPass, BandStop, HighPass);
impl_composite_conf!(LowPass, BandPass, HighPass);
impl_composite_conf!(BandStop, BandPass, HighPass);

impl_composite_conf!(LowPass, BandStop, BandPass, HighPass => All);

mod private
{
    use crate::{conf::InputOrGND, filters::iir::second::SecondOrderRLCFilter, params::RLC, rtf::Rtf};

    use super::{SecondOrderRLCFilterConf, SecondOrderRLCFilterParam};

    pub trait SecondOrderRLCFilterConfFinal<C>: SecondOrderRLCFilterConf<
        Conf = C::Conf
    >
    where
        C: SecondOrderRLCFilterConf
    {

    }
    impl<
        CC,
        C,
        const OUTPUTS: usize,
        const R_CONF: InputOrGND,
        const L_CONF: InputOrGND,
        const C_CONF: InputOrGND,
    > SecondOrderRLCFilterConfFinal<C> for CC
    where
        CC: SecondOrderRLCFilterConf<
            Conf = C::Conf,
            OUTPUTS = {OUTPUTS},
            R_CONF = {R_CONF},
            L_CONF = {L_CONF},
            C_CONF = {C_CONF}
        >,
        C: SecondOrderRLCFilterConf<
            OUTPUTS = {OUTPUTS},
            R_CONF = {R_CONF},
            L_CONF = {L_CONF},
            C_CONF = {C_CONF}
        >,
        RLC<f32>: SecondOrderRLCFilterParam<CC, Conf = CC>,
        RLC<f64>: SecondOrderRLCFilterParam<CC, Conf = CC>,
        SecondOrderRLCFilter<f32, RLC<f32>, C>: Rtf,
        SecondOrderRLCFilter<f64, RLC<f64>, C>: Rtf,
        [(); <CC::Conf as SecondOrderRLCFilterConf>::OUTPUTS]:
    {

    }
}