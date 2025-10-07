use crate::{conf::{All, BandPass, HighPass, InputOrGND, LowPass, all}, util::{self, ObviousArray}};

use super::{FirstOrderFilterConf, SecondOrderRLCFilterConf, SecondOrderRCFilterConf, ThirdOrderSallenKeyFilterConf};

pub trait FirstOrderRCFilterConf: FirstOrderFilterConf
{
    type Conf: private::FirstOrderRCFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;

    type AsSecondOrderRLCFilterConf: private::SecondOrderRLCFilterConfForFirstOrderRCFilterConf<Self>;
    type AsSecondOrderRCFilterConf: private::SecondOrderRCFilterConfForFirstOrderRCFilterConf<Self>;
    type AsThirdOrderSallenKeyFilterConf: private::ThirdOrderSallenKeyFilterConfForFirstOrderRCFilterConf<Self>;

    const R_CONF: InputOrGND;
    const C_CONF: InputOrGND = Self::R_CONF.opposite();
}

impl FirstOrderRCFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsSecondOrderRLCFilterConf = LowPass;
    type AsSecondOrderRCFilterConf = LowPass;
    type AsThirdOrderSallenKeyFilterConf = LowPass;
    
    const R_CONF: InputOrGND = InputOrGND::Input;
}
impl FirstOrderRCFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsSecondOrderRLCFilterConf = BandPass;
    type AsSecondOrderRCFilterConf = BandPass<1>;
    type AsThirdOrderSallenKeyFilterConf = BandPass<1>;
    
    const R_CONF: InputOrGND = InputOrGND::GND;
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl FirstOrderRCFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderRCFilterConf>::Outputs::<U>),+);
            
            type AsSecondOrderRLCFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsSecondOrderRLCFilterConf),*
            ) as SecondOrderRLCFilterConf>::Conf;
            type AsSecondOrderRCFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsSecondOrderRCFilterConf),*
            ) as SecondOrderRCFilterConf>::Conf;
            type AsThirdOrderSallenKeyFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsThirdOrderSallenKeyFilterConf),*
            ) as ThirdOrderSallenKeyFilterConf>::Conf;

            const R_CONF: InputOrGND = InputOrGND::all([
                $(<$more as FirstOrderRCFilterConf>::R_CONF),*
            ]);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderRCFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderRCFilterConf>::Outputs::<U>),+);
            
            type AsSecondOrderRLCFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsSecondOrderRLCFilterConf),*
            ) as SecondOrderRLCFilterConf>::Conf;
            type AsSecondOrderRCFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsSecondOrderRCFilterConf),*
            ) as SecondOrderRCFilterConf>::Conf;
            type AsThirdOrderSallenKeyFilterConf = <all!(
                $(<$more as FirstOrderRCFilterConf>::AsThirdOrderSallenKeyFilterConf),*
            ) as ThirdOrderSallenKeyFilterConf>::Conf;

            const R_CONF: InputOrGND = InputOrGND::all([
                $(<$more as FirstOrderRCFilterConf>::R_CONF),*
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
    use crate::{conf::{InputOrFeedback, InputOrGND, LowPass}, param::{FirstOrderRCFilterParam, SecondOrderRCFilterConf, SecondOrderRLCFilterConf, ThirdOrderSallenKeyFilterConf, RC}};

    use super::FirstOrderRCFilterConf;

    pub trait FirstOrderRCFilterConfFinal<C>: FirstOrderRCFilterConf<
        Conf = <C as FirstOrderRCFilterConf>::Conf,
        AsSecondOrderRLCFilterConf = C::AsSecondOrderRLCFilterConf,
        AsSecondOrderRCFilterConf = C::AsSecondOrderRCFilterConf,
        AsThirdOrderSallenKeyFilterConf = C::AsThirdOrderSallenKeyFilterConf
    >
    where
        C: FirstOrderRCFilterConf
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > FirstOrderRCFilterConfFinal<C> for CC
    where
        CC: FirstOrderRCFilterConf<
            Conf = <C as FirstOrderRCFilterConf>::Conf,
            AsSecondOrderRLCFilterConf = C::AsSecondOrderRLCFilterConf,
            AsSecondOrderRCFilterConf = C::AsSecondOrderRCFilterConf,
            AsThirdOrderSallenKeyFilterConf = C::AsThirdOrderSallenKeyFilterConf,
            Outputs<()> = <C as FirstOrderRCFilterConf>::Outputs<()>,
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >,
        C: FirstOrderRCFilterConf<
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >,
        RC<f64>: FirstOrderRCFilterParam<CC, Conf = CC>,
        RC<f32>: FirstOrderRCFilterParam<CC, Conf = CC>
    {

    }

    pub trait SecondOrderRLCFilterConfForFirstOrderRCFilterConf<C>: SecondOrderRLCFilterConf<
        L_CONF = {InputOrGND::Input}
    >
    where
        C: FirstOrderRCFilterConf<
            AsSecondOrderRLCFilterConf = Self
        >
    {
    
    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > SecondOrderRLCFilterConfForFirstOrderRCFilterConf<C> for CC
    where
        CC: SecondOrderRLCFilterConf<
            R_CONF = {R_CONF},
            L_CONF = {InputOrGND::Input},
            C_CONF = {C_CONF}
        >,
        C: FirstOrderRCFilterConf<
            AsSecondOrderRLCFilterConf = CC,
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >
    {
    
    }

    pub trait SecondOrderRCFilterConfForFirstOrderRCFilterConf<C>: SecondOrderRCFilterConf<
        S1Conf = <C as FirstOrderRCFilterConf>::Conf,
        S2Conf = LowPass,
        R2_CONF = {InputOrGND::Input},
        C2_CONF = {InputOrGND::GND}
    >
    where
        C: FirstOrderRCFilterConf<
            AsSecondOrderRCFilterConf = Self
        >
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > SecondOrderRCFilterConfForFirstOrderRCFilterConf<C> for CC
    where
        CC: SecondOrderRCFilterConf<
            S1Conf = <C as FirstOrderRCFilterConf>::Conf,
            S2Conf = LowPass,
            R1_CONF = {R_CONF},
            C1_CONF = {C_CONF},
            R2_CONF = {InputOrGND::Input},
            C2_CONF = {InputOrGND::GND}
        >,
        C: FirstOrderRCFilterConf<
            AsSecondOrderRCFilterConf = CC,
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >
    {

    }

    pub trait ThirdOrderSallenKeyFilterConfForFirstOrderRCFilterConf<C>: ThirdOrderSallenKeyFilterConf<
        S1Conf = <C as FirstOrderRCFilterConf>::Conf,
        S2Conf = LowPass,
        R2_CONF = {InputOrFeedback::Input},
        C2_CONF = {InputOrFeedback::Feedback},
        R3_CONF = {InputOrGND::Input},
        C3_CONF = {InputOrGND::GND}
    >
    where
        C: FirstOrderRCFilterConf<
            AsThirdOrderSallenKeyFilterConf = Self
        >
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > ThirdOrderSallenKeyFilterConfForFirstOrderRCFilterConf<C> for CC
    where
        CC: ThirdOrderSallenKeyFilterConf<
            S1Conf = <C as FirstOrderRCFilterConf>::Conf,
            S2Conf = LowPass,
            R1_CONF = {R_CONF},
            C1_CONF = {C_CONF},
            R2_CONF = {InputOrFeedback::Input},
            C2_CONF = {InputOrFeedback::Feedback},
            R3_CONF = {InputOrGND::Input},
            C3_CONF = {InputOrGND::GND}
        >,
        C: FirstOrderRCFilterConf<
            AsThirdOrderSallenKeyFilterConf = CC,
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >
    {

    }
}