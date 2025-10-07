use crate::{conf::{All, Conf, HighPass, LowPass, Peak, all}, util::{self, ObviousArray}};

pub trait ThirdOrderFilterConf: Conf
{
    type Conf: private::ThirdOrderFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;
}
impl ThirdOrderFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}
impl ThirdOrderFilterConf for Peak<1>
{
    type Conf = Self;
    
    type Outputs<U> = [U; 1];
}
impl ThirdOrderFilterConf for Peak<2>
{
    type Conf = Self;
    
    type Outputs<U> = [U; 1];
}
impl ThirdOrderFilterConf for HighPass
{
    type Conf = Self;
    
    type Outputs<U> = [U; 1];
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl ThirdOrderFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as ThirdOrderFilterConf>::Outputs::<U>),+);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl ThirdOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as ThirdOrderFilterConf>::Outputs::<U>),+);
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
    use crate::param::{ButterworthFilterConf, Omega2Zeta, ThirdOrderFilterParam};

    use super::ThirdOrderFilterConf;

    pub trait ThirdOrderFilterConfFinal<C>: ThirdOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: ThirdOrderFilterConf
    {

    }
    impl<
        CC,
        C
    > ThirdOrderFilterConfFinal<C> for CC
    where
        CC: ThirdOrderFilterConf<
            Conf = C::Conf,
            Outputs<()> = C::Outputs<()>
        > + ButterworthFilterConf<
            3,
            Conf = C::Conf,
            Outputs<()> = C::Outputs<()>
        >,
        C: ThirdOrderFilterConf,
        Omega2Zeta<f64>: ThirdOrderFilterParam<CC, Conf = CC>,
        Omega2Zeta<f32>: ThirdOrderFilterParam<CC, Conf = CC>,
    {

    }
}