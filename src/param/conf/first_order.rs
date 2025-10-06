use crate::{conf::{All, Conf, HighPass, LowPass, all}, util::{self, ArrayChunks}};

pub trait FirstOrderFilterConf: Conf
{
    type Conf: private::FirstOrderFilterConfFinal<Self>;

    type Outputs<U>: ArrayChunks<[U; 1], Elem = U, Rem = [U; 0]>;
}

impl FirstOrderFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}
impl FirstOrderFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderFilterConf>::Outputs::<U>),+);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderFilterConf>::Outputs::<U>),+);
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
    use crate::param::{FirstOrderFilterParam, OmegaFirstOrder};

    use super::FirstOrderFilterConf;

    pub trait FirstOrderFilterConfFinal<C>: FirstOrderFilterConf<
        Conf = C::Conf
    >
    where
        C: FirstOrderFilterConf
    {

    }
    impl<
        CC,
        C
    > FirstOrderFilterConfFinal<C> for CC
    where
        CC: FirstOrderFilterConf<
            Conf = CC,
            Outputs<()> = C::Outputs<()>
        >,
        C: FirstOrderFilterConf<
            Conf = CC
        >,
        OmegaFirstOrder<f64>: FirstOrderFilterParam<CC, Conf = CC>,
        OmegaFirstOrder<f32>: FirstOrderFilterParam<CC, Conf = CC>
    {

    }
}