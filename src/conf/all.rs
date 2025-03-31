use super::{BandPass, BandStop, Conf, ConfType, HighPass, LowPass, Peak};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum All {}

impl Conf for All
{
    const CONF_TYPE: ConfType = ConfType::Wildcard;

    type Wildcard = All;
}

mod private
{
    use crate::conf::{Conf, ConfType};

    use super::All;

    pub trait IntoConf
    {
        type IntoConf: Conf;
    }
    impl<C> IntoConf for C
    where
        C: Conf
    {
        type IntoConf = C;
    }

    pub trait AllSome
    {
        type AllSome;
    }
    impl<C> AllSome for C
    where
        C: IntoConf
    {
        type AllSome = <C as IntoConf>::IntoConf;
    }
    
    pub trait OneWildcardOrAll: Conf
    {
        type OneWildcardOrAll: Conf<CONF_TYPE = {ConfType::Wildcard}, Wildcard = Self::OneWildcardOrAll>;
    }
    impl<T> OneWildcardOrAll for T
    where
        T: Conf
    {
        default type OneWildcardOrAll = All;
    }
    impl<T> OneWildcardOrAll for T
    where
        T: Conf<CONF_TYPE = {ConfType::Wildcard}, Wildcard = T>
    {
        type OneWildcardOrAll = T;
    }
}

macro c {
    ($($c:ty),+) => {
        impl Conf for ($($c,)*)
        {
            const CONF_TYPE: ConfType = ConfType::Composite;

            type Wildcard = <all!($(<<$c as private::IntoConf>::IntoConf as Conf>::Wildcard),*) as private::OneWildcardOrAll>::OneWildcardOrAll;
        }
    },
    ($($i:ty),+ => $c:ty) => {
        impl private::IntoConf for ($($i,)*)
        {
            type IntoConf = $c;
        }
    }
}

c!(LowPass, Peak, Peak, HighPass => (LowPass, Peak, HighPass));
c!(Peak, Peak, HighPass => (Peak, HighPass));
c!(Peak, Peak => Peak);
c!(LowPass, Peak, Peak => (LowPass, Peak));
c!(LowPass, BandPass, BandPass, BandPass, BandPass, BandPass, BandPass => (LowPass, BandPass));
c!(LowPass, BandPass, BandPass, BandPass, BandPass, BandPass, HighPass => (LowPass, BandPass, HighPass));
c!(LowPass, BandPass, BandPass, BandPass, BandPass, BandPass, BandPass, HighPass => (LowPass, BandPass, HighPass));
c!(BandPass, BandPass, BandPass, BandPass, BandPass, BandPass, HighPass => (BandPass, HighPass));
c!(BandPass, BandPass, BandPass, BandPass, BandPass, BandPass => BandPass);
c!(BandPass, BandPass, BandPass, BandPass, BandPass, HighPass => (BandPass, HighPass));
c!(LowPass, BandPass, BandPass, BandPass, BandPass, BandPass => (LowPass, BandPass));
c!(LowPass, BandPass, BandPass, BandPass, BandPass, HighPass => (LowPass, BandPass, HighPass));
c!(BandPass, BandPass, BandPass, BandPass, HighPass => (BandPass, HighPass));
c!(BandPass, BandPass, BandPass, BandPass, BandPass => BandPass);
c!(LowPass, BandPass, BandPass, BandPass, HighPass => (LowPass, BandPass, HighPass));
c!(LowPass, BandPass, BandPass, BandPass, BandPass => (LowPass, BandPass));
c!(BandPass, BandPass, BandPass, HighPass => (BandPass, HighPass));
c!(BandPass, BandPass, BandPass, BandPass => BandPass);
c!(LowPass, BandPass, BandPass, HighPass => (LowPass, BandPass, HighPass));
c!(LowPass, BandPass, BandPass, BandPass => (LowPass, BandPass));
c!(BandPass, BandPass, BandPass => BandPass);
c!(BandPass, BandPass, HighPass => (BandPass, HighPass));
c!(LowPass, HighPass);
c!(LowPass, Peak);
c!(LowPass, Peak, HighPass);
c!(LowPass, BandPass);
c!(BandPass, HighPass);
c!(LowPass, BandPass, HighPass);
c!(LowPass, BandPass<1>);
c!(LowPass, BandPass<2>);
c!(BandPass<1>, BandPass<2>);
c!(BandPass<1>, HighPass);
c!(BandPass<2>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>);
c!(LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, All => All);
c!(HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, BandPass<1>, HighPass);
c!(LowPass, BandPass<2>, HighPass);
c!(BandPass<1>, BandPass<2>, HighPass);
c!(LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(All, HighPass => All);
c!(LowPass, BandStop);
c!(BandStop, BandPass);
c!(BandStop, HighPass);
c!(LowPass, BandStop, BandPass);
c!(LowPass, BandStop, HighPass);
c!(BandStop, BandPass, HighPass);
c!(LowPass, BandStop, BandPass, HighPass);
c!(LowPass, BandPass<2>, BandPass<4>);
c!(BandPass<2>, BandPass<4>);
c!(LowPass, (BandPass<2>, BandPass<4>) => (LowPass, BandPass<2>, BandPass<4>));
c!(BandPass<2>, BandPass<4>, BandPass<6>);
c!(BandPass<2>, (BandPass<4>, BandPass<6>) => (BandPass<2>, BandPass<4>, BandPass<6>));
c!((BandPass<2>, BandPass<4>), BandPass<6> => (BandPass<2>, BandPass<4>, BandPass<6>));
c!(LowPass, BandPass<3>);
c!(LowPass, BandPass<4>);
c!(LowPass, BandPass<5>);
c!(LowPass, BandPass<6>);
c!(BandPass<1>, BandPass<3>);
c!(BandPass<1>, BandPass<4>);
c!(BandPass<1>, BandPass<5>);
c!(BandPass<1>, BandPass<6>);
c!(BandPass<2>, BandPass<3>);
c!(BandPass<2>, BandPass<5>);
c!(BandPass<2>, BandPass<6>);
c!(BandPass<3>, BandPass<4>);
c!(BandPass<3>, BandPass<5>);
c!(BandPass<3>, BandPass<6>);
c!(BandPass<3>, HighPass);
c!(BandPass<4>, BandPass<5>);
c!(BandPass<4>, BandPass<6>);
c!(BandPass<4>, HighPass);
c!(BandPass<5>, BandPass<6>);
c!(BandPass<5>, HighPass);
c!(BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>);
c!(LowPass, BandPass<1>, BandPass<4>);
c!(LowPass, BandPass<1>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<3>);
c!(LowPass, BandPass<2>, BandPass<5>);
c!(LowPass, BandPass<2>, BandPass<6>);
c!(LowPass, BandPass<3>, BandPass<4>);
c!(LowPass, BandPass<3>, BandPass<5>);
c!(LowPass, BandPass<3>, BandPass<6>);
c!(LowPass, BandPass<3>, HighPass);
c!(LowPass, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<4>, HighPass);
c!(LowPass, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<5>, HighPass);
c!(LowPass, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>);
c!(BandPass<1>, BandPass<2>, BandPass<4>);
c!(BandPass<1>, BandPass<2>, BandPass<5>);
c!(BandPass<1>, BandPass<2>, BandPass<6>);
c!(BandPass<1>, BandPass<3>, BandPass<4>);
c!(BandPass<1>, BandPass<3>, BandPass<5>);
c!(BandPass<1>, BandPass<3>, BandPass<6>);
c!(BandPass<1>, BandPass<3>, HighPass);
c!(BandPass<1>, BandPass<4>, BandPass<5>);
c!(BandPass<1>, BandPass<4>, BandPass<6>);
c!(BandPass<1>, BandPass<4>, HighPass);
c!(BandPass<1>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<4>);
c!(BandPass<2>, BandPass<3>, BandPass<5>);
c!(BandPass<2>, BandPass<3>, BandPass<6>);
c!(BandPass<2>, BandPass<3>, HighPass);
c!(BandPass<2>, BandPass<4>, BandPass<5>);
c!(BandPass<2>, BandPass<4>, HighPass);
c!(BandPass<2>, BandPass<5>, BandPass<6>);
c!(BandPass<2>, BandPass<5>, HighPass);
c!(BandPass<2>, BandPass<6>, HighPass);
c!(BandPass<3>, BandPass<4>, BandPass<5>);
c!(BandPass<3>, BandPass<4>, BandPass<6>);
c!(BandPass<3>, BandPass<4>, HighPass);
c!(BandPass<3>, BandPass<5>, BandPass<6>);
c!(BandPass<3>, BandPass<5>, HighPass);
c!(BandPass<3>, BandPass<6>, HighPass);
c!(BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<3>, HighPass);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<4>, HighPass);
c!(LowPass, BandPass<1>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<3>, HighPass);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<4>, HighPass);
c!(LowPass, BandPass<2>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<5>, HighPass);
c!(LowPass, BandPass<2>, BandPass<6>, HighPass);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<3>, BandPass<4>, HighPass);
c!(LowPass, BandPass<3>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<3>, BandPass<5>, HighPass);
c!(LowPass, BandPass<3>, BandPass<6>, HighPass);
c!(LowPass, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<4>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(BandPass<1>, BandPass<3>, BandPass<4>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<3>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<3>, BandPass<4>, BandPass<5>, BandPass<6>, HighPass);
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(BandPass, BandPass => BandPass);
c!(LowPass, BandPass, BandPass => (LowPass, BandPass));
c!(LowPass, All, HighPass => All);
c!(LowPass, LowPass => LowPass);
c!(HighPass, HighPass => HighPass);
c!(LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass => LowPass);
c!(LowPass, (BandPass<2>, BandPass<4>), BandPass<6> => (LowPass, BandPass<2>, BandPass<4>, BandPass<6>));
c!(LowPass, LowPass, LowPass, LowPass => LowPass);
c!(LowPass, HighPass, LowPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, HighPass => HighPass);
c!(LowPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(BandPass<1>, BandPass<1> => BandPass<1>);
c!(BandPass<2>, BandPass<2> => BandPass<2>);
c!(LowPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, BandPass<1> => (LowPass, BandPass<1>));
c!(HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, BandPass<2> => (LowPass, BandPass<2>));
c!(LowPass, BandPass<1>, BandPass<1> => (LowPass, BandPass<1>));
c!(LowPass, BandPass<2>, BandPass<2> => (LowPass, BandPass<2>));
c!(BandPass<1>, BandPass<1>, BandPass<2> => (BandPass<1>, BandPass<2>));
c!(HighPass, LowPass, LowPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, HighPass, HighPass => HighPass);
c!(LowPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(BandPass<1>, BandPass<1>, HighPass => (BandPass<1>, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(HighPass, LowPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, LowPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, HighPass, LowPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, LowPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(HighPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, LowPass, HighPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, LowPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, HighPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, LowPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, HighPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(LowPass, LowPass, HighPass, LowPass, HighPass, LowPass, HighPass => (LowPass, HighPass));
c!(BandPass<1>, BandPass<2>, BandPass<2> => (BandPass<1>, BandPass<2>));
c!(BandPass<1>, HighPass, HighPass => (BandPass<1>, HighPass));
c!(BandPass<2>, BandPass<2>, HighPass => (BandPass<2>, HighPass));
c!(BandPass<2>, HighPass, HighPass => (BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1> => (LowPass, BandPass<1>));
c!(LowPass, LowPass, BandPass<1>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, LowPass, BandPass<1>, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, LowPass, BandPass<2>, BandPass<2> => (LowPass, BandPass<2>));
c!(LowPass, LowPass, BandPass<2>, HighPass => (LowPass, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, BandPass<1>, BandPass<1>, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, BandPass<1>, HighPass, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<2>, HighPass));
c!(LowPass, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2> => (BandPass<1>, BandPass<2>));
c!(BandPass<1>, BandPass<1>, BandPass<2>, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<1>, HighPass, HighPass => (BandPass<1>, HighPass));
c!(BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<2>, HighPass, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<2>, BandPass<2>, HighPass, HighPass => (BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<2>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, LowPass, BandPass<1>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, HighPass, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, LowPass, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, HighPass, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<1>, BandPass<2>, HighPass, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2> => (LowPass, BandPass<1>, BandPass<2>));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, HighPass, HighPass => (LowPass, BandPass<1>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<2>, BandPass<2>, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<1>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(BandPass<1>, BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<1>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(LowPass, LowPass, BandPass<1>, BandPass<2>, BandPass<2>, HighPass, HighPass => (LowPass, BandPass<1>, BandPass<2>, HighPass));
c!(Peak, HighPass);
c!(LowPass, Peak<1>);
c!(LowPass, Peak<2>);
c!(Peak<1>, Peak<2>);
c!(Peak<1>, HighPass);
c!(Peak<2>, HighPass);
c!(LowPass, Peak<1>, Peak<2>);
c!(LowPass, Peak<1>, HighPass);
c!(LowPass, Peak<2>, HighPass);
c!(Peak<1>, Peak<2>, HighPass);
c!(LowPass, Peak<1>, Peak<2>, HighPass);

//(?:band_pass::|high_pass::|low_pass::)

pub macro all {
    ($c:ty$(,)?) => {
        $c
    },
    ($c0:ty, $($c:ty),+$(,)?) => {
        <($c0, $($c),*) as private::IntoConf>::IntoConf
    }
}

pub macro all_if_all_some {
    ($($c:ty),+$(,)?) => {
        <($c0, $($c),*) as private::AllSome>::AllSome
    }
}