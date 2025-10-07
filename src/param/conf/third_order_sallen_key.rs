use crate::{conf::{all, All, BandPass, Conf, HighPass, InputOrFeedback, InputOrGND, LowPass}, util::{ArrayChunks, ArrayMul, ObviousArray}};

use super::{FirstOrderRCFilterConf, SecondOrderSallenKeyFilterConf};

pub trait ThirdOrderSallenKeyFilterConf: Conf
{
    type Conf: private::ThirdOrderSallenKeyFilterConfFinal<Self>;

    type S1Conf: private::S1ConfForThirdOrderSallenKeyFilterConf<Self>;
    type S2Conf: private::S2ConfForThirdOrderSallenKeyFilterConf<Self>;

    type Outputs<U>: ObviousArray<Elem = U> + ArrayChunks<Self::OutputBufs<U>, Elem = U, Rem = [U; 0]>;
    type OutputBufs<U>: ObviousArray<Elem = U>;

    const R1_CONF: InputOrGND = <Self::S1Conf as FirstOrderRCFilterConf>::R_CONF;
    const C1_CONF: InputOrGND = <Self::S1Conf as FirstOrderRCFilterConf>::C_CONF;
    const R2_CONF: InputOrFeedback = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::R1_CONF;
    const C2_CONF: InputOrFeedback = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::C1_CONF;
    const R3_CONF: InputOrGND = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::R2_CONF;
    const C3_CONF: InputOrGND = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::C2_CONF;
}

impl ThirdOrderSallenKeyFilterConf for LowPass
{
    type Conf = Self;

    type S1Conf = LowPass;
    type S2Conf = LowPass;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<1>
{
    type Conf = Self;

    type S1Conf = HighPass;
    type S2Conf = LowPass;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<2>
{
    type Conf = Self;

    type S1Conf = LowPass;
    type S2Conf = BandPass<1>;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<3>
{
    type Conf = Self;

    type S1Conf = HighPass;
    type S2Conf = BandPass<1>;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<4>
{
    type Conf = Self;

    type S1Conf = LowPass;
    type S2Conf = BandPass<2>;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<5>
{
    type Conf = Self;

    type S1Conf = HighPass;
    type S2Conf = BandPass<2>;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for BandPass<6>
{
    type Conf = Self;

    type S1Conf = LowPass;
    type S2Conf = HighPass;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}
impl ThirdOrderSallenKeyFilterConf for HighPass
{
    type Conf = Self;

    type S1Conf = HighPass;
    type S2Conf = HighPass;

    type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
    type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl ThirdOrderSallenKeyFilterConf for $conf
        {
            type Conf = all!($($actual),*);

            type S1Conf = <all!(
                <$conf0 as ThirdOrderSallenKeyFilterConf>::S1Conf,
                $(<$more as ThirdOrderSallenKeyFilterConf>::S1Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
            type S2Conf = <all!(
                <$conf0 as ThirdOrderSallenKeyFilterConf>::S2Conf,
                $(<$more as ThirdOrderSallenKeyFilterConf>::S2Conf),*
            ) as SecondOrderSallenKeyFilterConf>::Conf;

            type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
            type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl ThirdOrderSallenKeyFilterConf for $conf
        {
            type Conf = $conf;

            type S1Conf = <all!(
                <$conf0 as ThirdOrderSallenKeyFilterConf>::S1Conf,
                $(<$more as ThirdOrderSallenKeyFilterConf>::S1Conf),*
            ) as FirstOrderRCFilterConf>::Conf;
            type S2Conf = <all!(
                <$conf0 as ThirdOrderSallenKeyFilterConf>::S2Conf,
                $(<$more as ThirdOrderSallenKeyFilterConf>::S2Conf),*
            ) as SecondOrderSallenKeyFilterConf>::Conf;

            type Outputs<U> = <<Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U> as ArrayMul<<<Self::S1Conf as FirstOrderRCFilterConf>::Conf as FirstOrderRCFilterConf>::Outputs<U>>>::Product;
            type OutputBufs<U> = <Self::S2Conf as SecondOrderSallenKeyFilterConf>::Outputs<U>;
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

impl_composite_conf!(BandPass: BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass);
impl_composite_conf!(BandPass, HighPass);
impl_composite_conf!(LowPass, BandPass, HighPass => All);

impl_composite_conf!(All: LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>);
impl_composite_conf!(LowPass, BandPass<2>);
impl_composite_conf!(LowPass, BandPass<3>);
impl_composite_conf!(LowPass, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<6>);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>);
impl_composite_conf!(BandPass<1>, BandPass<3>);
impl_composite_conf!(BandPass<1>, BandPass<4>);
impl_composite_conf!(BandPass<1>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<6>);
impl_composite_conf!(BandPass<1>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>);
impl_composite_conf!(BandPass<2>, BandPass<4>);
impl_composite_conf!(BandPass<2>, BandPass<5>);
impl_composite_conf!(BandPass<2>, BandPass<6>);
impl_composite_conf!(BandPass<2>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<4>);
impl_composite_conf!(BandPass<3>, BandPass<5>);
impl_composite_conf!(BandPass<3>, BandPass<6>);
impl_composite_conf!(BandPass<3>, HighPass);
impl_composite_conf!(BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<4>, HighPass);
impl_composite_conf!(BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<5>, HighPass);
impl_composite_conf!(BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<3>, HighPass);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<3>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<5>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<3>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<5>, BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<4>, BandPass<5>, BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6> => LowPass, BandPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
impl_composite_conf!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass => BandPass, HighPass);

impl_composite_conf!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass => All);

mod private
{
    use crate::{conf::{InputOrFeedback, InputOrGND}, param::{FirstOrderRCFilterConf, RC3GSallenKey, RC3SallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam}};

    use super::ThirdOrderSallenKeyFilterConf;

    pub trait ThirdOrderSallenKeyFilterConfFinal<C>: ThirdOrderSallenKeyFilterConf<
        Conf = C::Conf,
        S1Conf = C::S1Conf,
        S2Conf = C::S2Conf
    >
    where
        C: ThirdOrderSallenKeyFilterConf
    {

    }
    impl<
        CC,
        C,
        const R1_CONF: InputOrGND,
        const C1_CONF: InputOrGND,
        const R2_CONF: InputOrFeedback,
        const C2_CONF: InputOrFeedback,
        const R3_CONF: InputOrGND,
        const C3_CONF: InputOrGND
    > ThirdOrderSallenKeyFilterConfFinal<C> for CC
    where
        CC: ThirdOrderSallenKeyFilterConf<
            Conf = C::Conf,
            S1Conf = C::S1Conf,
            S2Conf = C::S2Conf,
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF},
            R3_CONF = {R3_CONF},
            C3_CONF = {C3_CONF},
        >,
        C: ThirdOrderSallenKeyFilterConf<
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF},
            R3_CONF = {R3_CONF},
            C3_CONF = {C3_CONF},
        >,
        RC3SallenKey<f32>: ThirdOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC3SallenKey<f64>: ThirdOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC3GSallenKey<f32>: ThirdOrderSallenKeyFilterParam<CC, Conf = CC>,
        RC3GSallenKey<f64>: ThirdOrderSallenKeyFilterParam<CC, Conf = CC>
    {

    }

    pub trait S1ConfForThirdOrderSallenKeyFilterConf<C>: FirstOrderRCFilterConf
    where
        C: ThirdOrderSallenKeyFilterConf<
            S1Conf = Self
        >
    {

    }
    impl<
        CC,
        C,
        const R_CONF: InputOrGND,
        const C_CONF: InputOrGND
    > S1ConfForThirdOrderSallenKeyFilterConf<C> for CC
    where
        CC: FirstOrderRCFilterConf<
            R_CONF = {R_CONF},
            C_CONF = {C_CONF}
        >,
        C: ThirdOrderSallenKeyFilterConf<
            S1Conf = CC,
            R1_CONF = {R_CONF},
            C1_CONF = {C_CONF}
        >
    {

    }
    
    pub trait S2ConfForThirdOrderSallenKeyFilterConf<C>: SecondOrderSallenKeyFilterConf
    where
        C: ThirdOrderSallenKeyFilterConf<
            S2Conf = Self
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
    > S2ConfForThirdOrderSallenKeyFilterConf<C> for CC
    where
        CC: SecondOrderSallenKeyFilterConf<
            R1_CONF = {R1_CONF},
            C1_CONF = {C1_CONF},
            R2_CONF = {R2_CONF},
            C2_CONF = {C2_CONF}
        >,
        C: ThirdOrderSallenKeyFilterConf<
            S2Conf = CC,
            R2_CONF = {R1_CONF},
            C2_CONF = {C1_CONF},
            R3_CONF = {R2_CONF},
            C3_CONF = {C2_CONF}
        >
    {

    }
}