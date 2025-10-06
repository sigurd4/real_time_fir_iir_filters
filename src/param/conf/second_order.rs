use crate::{util::{self, ArrayChunks}, conf::{all, All, Conf, HighPass, LowPass, Peak}};

pub trait SecondOrderFilterConf: Conf
{
    type Conf: private::SecondOrderFilterConfFinal<Self>;

    type Outputs<U>: ArrayChunks<[U; 1], Elem = U, Rem = [U; 0]>;
}
impl SecondOrderFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}
impl SecondOrderFilterConf for Peak
{
    type Conf = Self;
    
    type Outputs<U> = [U; 1];
}
impl SecondOrderFilterConf for HighPass
{
    type Conf = Self;
    
    type Outputs<U> = [U; 1];
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl SecondOrderFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderFilterConf>::Outputs::<U>),+);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl SecondOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as SecondOrderFilterConf>::Outputs::<U>),+);
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

impl_composite_conf!(All: LowPass, Peak, HighPass);

impl_composite_conf!(LowPass, Peak);
impl_composite_conf!(LowPass, HighPass);
impl_composite_conf!(Peak, HighPass);
impl_composite_conf!(LowPass, Peak, HighPass => All);

mod private
{
    use crate::param::{ButterworthFilterConf, OmegaZeta, SecondOrderFilterParam};

    use super::SecondOrderFilterConf;

    pub trait SecondOrderFilterConfFinal<C>: SecondOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: SecondOrderFilterConf
    {

    }
    impl<
        CC,
        C
    > SecondOrderFilterConfFinal<C> for CC
    where
        CC: SecondOrderFilterConf<
            Conf = C::Conf,
            Outputs<()> = C::Outputs<()>
        > + ButterworthFilterConf<
            2,
            Conf = C::Conf
        >,
        C: SecondOrderFilterConf,
        OmegaZeta<f64>: SecondOrderFilterParam<CC, Conf = CC>,
        OmegaZeta<f32>: SecondOrderFilterParam<CC, Conf = CC>
    {

    }
}