use crate::conf::{All, AllPass, Conf};

pub trait FirstOrderAllPassFilterConf: Conf
{
    type Conf: private::FirstOrderAllPassFilterConfFinal<Self>;

    const OUTPUTS: usize;
}

impl FirstOrderAllPassFilterConf for AllPass
{
    type Conf = All;

    const OUTPUTS: usize = 1;
}

macro impl_composite_conf {
    ($conf:ty: $conf0:ty $(,$more:ty)*) => {
        impl FirstOrderAllPassFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as FirstOrderAllPassFilterConf>::OUTPUTS $(+ <$more as FirstOrderAllPassFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty $(,$more:ty)* => $($actual:ty),+) => {
        impl FirstOrderAllPassFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as FirstOrderAllPassFilterConf>::OUTPUTS $(+ <$more as FirstOrderAllPassFilterConf>::OUTPUTS)*;
        }
    },
    ($conf0:ty $(,$more:ty)* $(=> $($actual:ty),+)?) => {
        impl_composite_conf!(
            all!(
                $conf0,
                $($more),*
            ): $conf0, $($more),* $(=> $($actual),+)?
        );
    }
}

impl_composite_conf!(All: AllPass);

mod private
{
    use crate::param::{FirstOrderAllPassFilterParam, Param, Tau};

    use super::FirstOrderAllPassFilterConf;

    pub trait FirstOrderAllPassFilterConfFinal<C>: FirstOrderAllPassFilterConf<
        Conf = C::Conf
    >
    where
        C: FirstOrderAllPassFilterConf
    {

    }
    impl<
        CC,
        C
    > FirstOrderAllPassFilterConfFinal<C> for CC
    where
        CC: FirstOrderAllPassFilterConf<
            Conf = CC
        >,
        C: FirstOrderAllPassFilterConf<
            Conf = CC::Conf
        >,
        Param<Tau<f64>>: FirstOrderAllPassFilterParam<CC, Conf = CC>,
        Param<Tau<f32>>: FirstOrderAllPassFilterParam<CC, Conf = CC>
    {

    }
}