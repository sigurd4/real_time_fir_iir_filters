use crate::conf::{all, All, Conf, HighPass, LowPass};

pub trait FirstOrderFilterConf: Conf
{
    type Conf: private::FirstOrderFilterConfFinal<Self>;

    const OUTPUTS: usize;
}

impl FirstOrderFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}
impl FirstOrderFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as FirstOrderFilterConf>::OUTPUTS $(+ <$more as FirstOrderFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as FirstOrderFilterConf>::OUTPUTS $(+ <$more as FirstOrderFilterConf>::OUTPUTS)*;
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
        C,
        const OUTPUTS: usize
    > FirstOrderFilterConfFinal<C> for CC
    where
        CC: FirstOrderFilterConf<
            Conf = CC,
            OUTPUTS = {OUTPUTS}
        >,
        C: FirstOrderFilterConf<
            Conf = CC::Conf,
            OUTPUTS = {OUTPUTS}
        >,
        OmegaFirstOrder<f64>: FirstOrderFilterParam<CC, Conf = CC>,
        OmegaFirstOrder<f32>: FirstOrderFilterParam<CC, Conf = CC>
    {

    }
}