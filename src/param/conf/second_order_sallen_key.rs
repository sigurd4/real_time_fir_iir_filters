use crate::{conf::{all, All, BandPass, Conf, HighPass, InputOrFeedback, InputOrGND, LowPass}, util::{self, ArrayChunks, ArrayMul, ObviousArray}};

use super::ThirdOrderSallenKeyFilterConf;

pub trait SecondOrderSallenKeyFilterConf: Conf
{
    type Conf: private::SecondOrderSallenKeyFilterConfFinal<Self>;

    type Outputs<U>: private::SecondOrderSallenKeyFilterOutputs<U>;

    type AsThirdOrderSallenKeyFilterConf: private::ThirdOrderSallenKeyFilterConfForSecondOrderSallenKeyFilterConf<Self>;

    const R1_CONF: InputOrFeedback;
    const C1_CONF: InputOrFeedback = Self::R1_CONF.opposite();
    const R2_CONF: InputOrGND;
    const C2_CONF: InputOrGND = Self::R2_CONF.opposite();
}

impl SecondOrderSallenKeyFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsThirdOrderSallenKeyFilterConf = LowPass;

    const R1_CONF: InputOrFeedback = InputOrFeedback::Input;
    const R2_CONF: InputOrGND = InputOrGND::Input;
}
impl SecondOrderSallenKeyFilterConf for BandPass<1>
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsThirdOrderSallenKeyFilterConf = BandPass<2>;

    const R1_CONF: InputOrFeedback = InputOrFeedback::Feedback;
    const R2_CONF: InputOrGND = InputOrGND::Input;
}
impl SecondOrderSallenKeyFilterConf for BandPass<2>
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsThirdOrderSallenKeyFilterConf = BandPass<4>;

    const R1_CONF: InputOrFeedback = InputOrFeedback::Input;
    const R2_CONF: InputOrGND = InputOrGND::GND;
}
impl SecondOrderSallenKeyFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];

    type AsThirdOrderSallenKeyFilterConf = BandPass<6>;

    const R1_CONF: InputOrFeedback = InputOrFeedback::Feedback;
    const R2_CONF: InputOrGND = InputOrGND::GND;
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl SecondOrderSallenKeyFilterConf for $conf
        {
            type Conf = all!($($actual),*);

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderSallenKeyFilterConf>::Outputs::<U>),+);

            type AsThirdOrderSallenKeyFilterConf = <all!(
                $(<$more as SecondOrderSallenKeyFilterConf>::AsThirdOrderSallenKeyFilterConf),*
            ) as ThirdOrderSallenKeyFilterConf>::Conf;
        
            const R1_CONF: InputOrFeedback = InputOrFeedback::all([
                $(<$more as SecondOrderSallenKeyFilterConf>::R1_CONF),*
            ]);
            const R2_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderSallenKeyFilterConf>::R2_CONF),*
            ]);
        }
    },
    ($conf:ty: $($more:ty),+) => {
        impl SecondOrderSallenKeyFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderSallenKeyFilterConf>::Outputs::<U>),+);

            type AsThirdOrderSallenKeyFilterConf = <all!(
                $(<$more as SecondOrderSallenKeyFilterConf>::AsThirdOrderSallenKeyFilterConf),*
            ) as ThirdOrderSallenKeyFilterConf>::Conf;
        
            const R1_CONF: InputOrFeedback = InputOrFeedback::all([
                $(<$more as SecondOrderSallenKeyFilterConf>::R1_CONF),*
            ]);
            const R2_CONF: InputOrGND = InputOrGND::all([
                $(<$more as SecondOrderSallenKeyFilterConf>::R2_CONF),*
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
    use crate::{conf::{InputOrFeedback, InputOrGND, LowPass}, param::{FirstOrderRCFilterConf, RC2GSallenKey, RC2SallenKey, SecondOrderSallenKeyFilterParam, ThirdOrderSallenKeyFilterConf}};

    use super::SecondOrderSallenKeyFilterConf;

    pub trait SecondOrderSallenKeyFilterOutputs<U> = ObviousArray<Elem = U>
        + ArrayMul<
            [U; 1],
            Elem = U,
            Product: ObviousArray<Elem = U> + ArrayChunks<Self, Elem = U, Rem = [U; 0]>
        > + ArrayMul<
            [U; 2],
            Elem = U,
            Product: ObviousArray<Elem = U> + ArrayChunks<Self, Elem = U, Rem = [U; 0]>
        >;
        
    pub trait SecondOrderSallenKeyFilterConfFinal<C>: SecondOrderSallenKeyFilterConf<
        Conf = C::Conf,
        AsThirdOrderSallenKeyFilterConf = C::AsThirdOrderSallenKeyFilterConf
    >
    where
        C: SecondOrderSallenKeyFilterConf
    {

    }
    impl<
        CC,
        C,
        const R1_CONF: InputOrFeedback,
        const C1_CONF: InputOrFeedback,
        const R2_CONF: InputOrGND,
        const C2_CONF: InputOrGND
    > SecondOrderSallenKeyFilterConfFinal<C> for CC
    where
        CC: SecondOrderSallenKeyFilterConf<
            Conf = C::Conf,
            AsThirdOrderSallenKeyFilterConf = C::AsThirdOrderSallenKeyFilterConf,
            Outputs<()> = C::Outputs<()>,
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >,
        C: SecondOrderSallenKeyFilterConf<
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >,
        RC2SallenKey<f32>: SecondOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC2SallenKey<f64>: SecondOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC2GSallenKey<f32>: SecondOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC2GSallenKey<f64>: SecondOrderSallenKeyFilterParam<CC, Conf = CC>
    {

    }

    pub trait ThirdOrderSallenKeyFilterConfForSecondOrderSallenKeyFilterConf<C>: ThirdOrderSallenKeyFilterConf<
        S1Conf = LowPass,
        S2Conf = C::Conf,
        R1_CONF = {<LowPass as FirstOrderRCFilterConf>::R_CONF},
        C1_CONF = {<LowPass as FirstOrderRCFilterConf>::C_CONF}
    >
    where
        C: SecondOrderSallenKeyFilterConf<
            AsThirdOrderSallenKeyFilterConf = Self
        >
    {
        
    }
    impl<
        CC,
        C,
        const R1_CONF: InputOrFeedback,
        const C1_CONF: InputOrFeedback,
        const R2_CONF: InputOrGND,
        const C2_CONF: InputOrGND
    > ThirdOrderSallenKeyFilterConfForSecondOrderSallenKeyFilterConf<C> for CC
    where
        CC: ThirdOrderSallenKeyFilterConf<
            S1Conf = LowPass,
            S2Conf = C::Conf,
            R1_CONF = {<LowPass as FirstOrderRCFilterConf>::R_CONF},
            C1_CONF = {<LowPass as FirstOrderRCFilterConf>::C_CONF},
            R2_CONF = {R1_CONF},
            C2_CONF = {C1_CONF},
            R3_CONF = {R2_CONF},
            C3_CONF = {C2_CONF}
        >,
        C: SecondOrderSallenKeyFilterConf<
            AsThirdOrderSallenKeyFilterConf = CC,
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >
    {

    }
}