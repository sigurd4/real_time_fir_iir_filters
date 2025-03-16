use crate::{conf::{All, AllPass, Conf}, param::FirstOrderAllPassFilterParamBase, util::same::Same};

pub trait FirstOrderAllPassFilterParam<
    C,
    ImplBase = <Self as FirstOrderAllPassFilterParamBase<C>>::ImplBase
>: FirstOrderAllPassFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: FirstOrderAllPassFilterConf;

    fn tau(&self) -> Self::F;
}

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
    use crate::{filters::iir::first::FirstOrderAllPassFilter, params::Tau, rtf::Rtf};

    use super::{FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParam};

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
        Tau<f64>: FirstOrderAllPassFilterParam<CC, Conf = CC>,
        Tau<f32>: FirstOrderAllPassFilterParam<CC, Conf = CC>,
        FirstOrderAllPassFilter<f64, Tau<f64>, C>: Rtf,
        FirstOrderAllPassFilter<f32, Tau<f32>, C>: Rtf,
        [(); <<CC as FirstOrderAllPassFilterConf>::Conf as FirstOrderAllPassFilterConf>::OUTPUTS]:
    {

    }
}