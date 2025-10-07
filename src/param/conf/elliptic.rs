use crate::{conf::{All, Conf, HighPass, LowPass, all}, util::{self, ObviousArray}};

pub trait EllipticFilterConf: Conf
{
    type Conf: private::EllipticFilterConfFinal<Self>;

    type Outputs<U>: ObviousArray<Elem = U>;
}

impl EllipticFilterConf for LowPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}
impl EllipticFilterConf for HighPass
{
    type Conf = Self;

    type Outputs<U> = [U; 1];
}


macro impl_composite_conf {
    ($conf:ty: $($more:ty),+) => {
        impl EllipticFilterConf for $conf
        {
            type Conf = $conf;

            type Outputs<U> = util::array_sum!($(<$more as EllipticFilterConf>::Outputs::<U>),+);
        }
    },
    ($conf:ty: $($more:ty),+ => $($actual:ty),+) => {
        impl EllipticFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            type Outputs<U> = util::array_sum!($(<$more as EllipticFilterConf>::Outputs::<U>),+);
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
    use crate::param::{ButterworthFilterConf, EllipticFilterConf};

    pub trait EllipticFilterConfFinal<C>: EllipticFilterConf<
        Conf = Self
    >
    where
        C: EllipticFilterConf<
            Conf = Self
        >
    {

    }
    impl<
        CC,
        C
    > EllipticFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC,
            Outputs<()> = C::Outputs<()>
        > + ButterworthFilterConf<
            0,
            Conf = CC,
            Outputs<()> = C::Outputs<()>
        >,
        C: EllipticFilterConf<
            Conf = CC
        >,
        /*OmegaDyn<f32>: DynOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaDyn<f64>: DynOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f32>: DynOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f64>: DynOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f32>: DynOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f64>: DynOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f32>: DynOrderEllipticFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f64>: DynOrderEllipticFilterParam<CC, Conf = CC>,

        OmegaFirstOrder<f32>: FirstOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaFirstOrder<f64>: FirstOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1FirstOrder<f32>: FirstOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1FirstOrder<f64>: FirstOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2FirstOrder<f32>: FirstOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2FirstOrder<f64>: FirstOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonXiFirstOrder<f32>: FirstOrderEllipticFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiFirstOrder<f64>: FirstOrderEllipticFilterParam<CC, Conf = CC>,
        
        OmegaSecondOrder<f32>: SecondOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaSecondOrder<f64>: SecondOrderButterworthFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1SecondOrder<f32>: SecondOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1SecondOrder<f64>: SecondOrderChebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2SecondOrder<f32>: SecondOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2SecondOrder<f64>: SecondOrderChebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonXiSecondOrder<f32>: SecondOrderEllipticFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiSecondOrder<f64>: SecondOrderEllipticFilterParam<CC, Conf = CC>*/
    {

    }
}