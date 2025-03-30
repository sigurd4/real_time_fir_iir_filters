use crate::conf::{all, All, HighPass, InputOrGND, LowPass};

use super::FirstOrderFilterConf;

pub trait FirstOrderLRFilterConf: FirstOrderFilterConf
{
    type Conf: private::FirstOrderLRFilterConfFinal<Self>;

    const OUTPUTS: usize;

    const R_CONF: InputOrGND;
    const L_CONF: InputOrGND = Self::R_CONF.opposite();
}
impl FirstOrderLRFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::GND;
}
impl FirstOrderLRFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;

    const R_CONF: InputOrGND = InputOrGND::Input;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl FirstOrderLRFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as FirstOrderLRFilterConf>::OUTPUTS $(+ <$more as FirstOrderLRFilterConf>::OUTPUTS)*;
            
            const R_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as FirstOrderLRFilterConf>::R_CONF,
                $(<$more as FirstOrderLRFilterConf>::R_CONF),*
            ]);
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderLRFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as FirstOrderLRFilterConf>::OUTPUTS $(+ <$more as FirstOrderLRFilterConf>::OUTPUTS)*;
            
            const R_CONF: InputOrGND = InputOrGND::all([
                <$conf0 as FirstOrderLRFilterConf>::R_CONF,
                $(<$more as FirstOrderLRFilterConf>::R_CONF),*
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

impl_composite_conf!(All: LowPass, HighPass);

impl_composite_conf!(LowPass, HighPass => All);

mod private
{
    use crate::{conf::InputOrGND, param::{FirstOrderLRFilterParam, Param, LR}};

    use super::FirstOrderLRFilterConf;

    pub trait FirstOrderLRFilterConfFinal<C>: FirstOrderLRFilterConf<
        Conf = <C as FirstOrderLRFilterConf>::Conf
    >
    where
        C: FirstOrderLRFilterConf
    {

    }
    impl<
        CC,
        C,
        const OUTPUTS: usize,
        const R_CONF: InputOrGND,
        const L_CONF: InputOrGND
    > FirstOrderLRFilterConfFinal<C> for CC
    where
        CC: FirstOrderLRFilterConf<
            Conf = <C as FirstOrderLRFilterConf>::Conf,
            OUTPUTS = {OUTPUTS},
            R_CONF = {R_CONF},
            L_CONF = {L_CONF}
        >,
        C: FirstOrderLRFilterConf<
            OUTPUTS = {OUTPUTS},
            R_CONF = {R_CONF},
            L_CONF = {L_CONF}
        >,
        Param<LR<f64>>: FirstOrderLRFilterParam<CC, Conf = CC>,
        Param<LR<f32>>: FirstOrderLRFilterParam<CC, Conf = CC>
    {

    }
}