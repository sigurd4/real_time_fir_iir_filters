use crate::{util::{self, ObviousArray}, conf::{all, All, HighPass, InputOrGND, LowPass}};

use super::FirstOrderFilterConf;

pub trait FirstOrderLRFilterConf: FirstOrderFilterConf
{
    type Conf: private::FirstOrderLRFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;

    const R_CONF: InputOrGND;
    const L_CONF: InputOrGND = Self::R_CONF.opposite();
}
impl FirstOrderLRFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::GND;
}
impl FirstOrderLRFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::Input;
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl FirstOrderLRFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderLRFilterConf>::Outputs::<U>),+);
            
            const R_CONF: InputOrGND = InputOrGND::all([
                $(<$more as FirstOrderLRFilterConf>::R_CONF),*
            ]);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderLRFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderLRFilterConf>::Outputs::<U>),+);
            
            const R_CONF: InputOrGND = InputOrGND::all([
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
    use crate::{conf::InputOrGND, param::{FirstOrderLRFilterParam, LR}};

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
        const R_CONF: InputOrGND,
        const L_CONF: InputOrGND
    > FirstOrderLRFilterConfFinal<C> for CC
    where
        CC: FirstOrderLRFilterConf<
            Conf = CC,
            Outputs<()> = <C as FirstOrderLRFilterConf>::Outputs<()>,
            R_CONF = {R_CONF},
            L_CONF = {L_CONF}
        >,
        C: FirstOrderLRFilterConf<
            Conf = CC,
            R_CONF = {R_CONF},
            L_CONF = {L_CONF}
        >,
        LR<f64>: FirstOrderLRFilterParam<CC, Conf = CC>,
        LR<f32>: FirstOrderLRFilterParam<CC, Conf = CC>
    {

    }
}