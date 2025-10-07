use crate::{util::{self, ObviousArray}, conf::{all, All, BandPass, BandStop, Conf, HighPass, InputOrGND, LowPass}};

pub trait SecondOrderRLCFilterConf: Conf
{
    type Conf: private::SecondOrderRLCFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;

    const R_CONF: InputOrGND;
    const L_CONF: InputOrGND;
    const C_CONF: InputOrGND;
}

impl SecondOrderRLCFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::Input;
    const L_CONF: InputOrGND = InputOrGND::Input;
    const C_CONF: InputOrGND = InputOrGND::GND;
}
impl SecondOrderRLCFilterConf for BandStop
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::Input;
    const L_CONF: InputOrGND = InputOrGND::GND;
    const C_CONF: InputOrGND = InputOrGND::GND;
}
impl SecondOrderRLCFilterConf for BandPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::GND;
    const L_CONF: InputOrGND = InputOrGND::Input;
    const C_CONF: InputOrGND = InputOrGND::Input;
}
impl SecondOrderRLCFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    const R_CONF: InputOrGND = InputOrGND::GND;
    const L_CONF: InputOrGND = InputOrGND::GND;
    const C_CONF: InputOrGND = InputOrGND::Input;
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl SecondOrderRLCFilterConf for $conf
        {
            type Conf = all!($($actual),*);

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderRLCFilterConf>::Outputs::<U>),+);

            const R_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderRLCFilterConf>::R_CONF),*
            ]);
            const L_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderRLCFilterConf>::L_CONF),*
            ]);
            const C_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderRLCFilterConf>::C_CONF),*
            ]);
        }
    },
    ($conf:ty: $($more:ty),+) => {
        impl SecondOrderRLCFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderRLCFilterConf>::Outputs::<U>),+);

            const R_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderRLCFilterConf>::R_CONF),*
            ]);
            const L_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderRLCFilterConf>::L_CONF),*
            ]);
            const C_CONF: InputOrGND = InputOrGND::all([
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
    use crate::{conf::InputOrGND, param::{SecondOrderRLCFilterParam, RLC}};

    use super::SecondOrderRLCFilterConf;

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
        const R_CONF: InputOrGND,
        const L_CONF: InputOrGND,
        const C_CONF: InputOrGND,
    > SecondOrderRLCFilterConfFinal<C> for CC
    where
        CC: SecondOrderRLCFilterConf<
            Conf = C::Conf,
            Outputs<()> = C::Outputs<()>,
            R_CONF = {R_CONF},
            L_CONF = {L_CONF},
            C_CONF = {C_CONF}
        >,
        C: SecondOrderRLCFilterConf<
            R_CONF = {R_CONF},
            L_CONF = {L_CONF},
            C_CONF = {C_CONF}
        >,
        RLC<f32>: SecondOrderRLCFilterParam<CC, Conf = CC>,
        RLC<f64>: SecondOrderRLCFilterParam<CC, Conf = CC>
    {

    }
}