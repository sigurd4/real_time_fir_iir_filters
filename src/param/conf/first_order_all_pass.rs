use crate::{conf::{All, AllPass, Conf}, util::{self, ObviousArray}};

pub trait FirstOrderAllPassFilterConf: Conf
{
    type Conf: private::FirstOrderAllPassFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;
}

impl FirstOrderAllPassFilterConf for AllPass
{
    type Conf = All;

    type Outputs<U> = [U; 1];
}

macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl FirstOrderAllPassFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderAllPassFilterConf>::Outputs::<U>),+);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl FirstOrderAllPassFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as FirstOrderAllPassFilterConf>::Outputs::<U>),+);
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
    use crate::param::{FirstOrderAllPassFilterParam, Tau};

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
            Conf = CC,
            Outputs<()> = C::Outputs<()>
        >,
        C: FirstOrderAllPassFilterConf<
            Conf = CC
        >,
        Tau<f64>: FirstOrderAllPassFilterParam<CC, Conf = CC>,
        Tau<f32>: FirstOrderAllPassFilterParam<CC, Conf = CC>
    {

    }
}