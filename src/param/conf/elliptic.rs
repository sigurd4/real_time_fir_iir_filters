use crate::conf::{all, All, Conf, HighPass, LowPass};


pub trait EllipticFilterConf: Conf
{
    type Conf: private::EllipticFilterConfFinal<Self>;

    const OUTPUTS: usize;
}

impl EllipticFilterConf for LowPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}
impl EllipticFilterConf for HighPass
{
    type Conf = Self;

    const OUTPUTS: usize = 1;
}


macro impl_composite_conf {
    ($conf:ty: $conf0:ty, $($more:ty),+) => {
        impl EllipticFilterConf for $conf
        {
            type Conf = $conf;

            const OUTPUTS: usize = <$conf0 as EllipticFilterConf>::OUTPUTS $(+ <$more as EllipticFilterConf>::OUTPUTS)*;
        }
    },
    ($conf:ty: $conf0:ty, $($more:ty),+ => $($actual:ty),+) => {
        impl EllipticFilterConf for $conf
        {
            type Conf = all!($($actual),+);

            const OUTPUTS: usize = <$conf0 as EllipticFilterConf>::OUTPUTS $(+ <$more as EllipticFilterConf>::OUTPUTS)*;
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
    use crate::param::{ButterworthFilterConf, DynOrderButterworthFilterParam, DynOrderChebyshev1FilterParam, DynOrderChebyshev2FilterParam, DynOrderEllipticFilterParam, EllipticFilterConf, FirstOrderButterworthFilterParam, FirstOrderChebyshev1FilterParam, FirstOrderChebyshev2FilterParam, FirstOrderEllipticFilterParam, OmegaDyn, OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb1FirstOrder, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb2Dyn, OmegaEpsilonCheb2FirstOrder, OmegaEpsilonCheb2SecondOrder, OmegaEpsilonXiDyn, OmegaEpsilonXiFirstOrder, OmegaEpsilonXiSecondOrder, OmegaFirstOrder, OmegaSecondOrder, Param, SecondOrderButterworthFilterParam, SecondOrderChebyshev1FilterParam, SecondOrderChebyshev2FilterParam, SecondOrderEllipticFilterParam};

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
        C,
        //const OUTPUTS: usize
    > EllipticFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        > + ButterworthFilterConf<
            0,
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        >,
        C: EllipticFilterConf<
            Conf = CC,
            //OUTPUTS = {OUTPUTS}
        >,
        Param<OmegaDyn<f32>>: DynOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaDyn<f64>>: DynOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1Dyn<f32>>: DynOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1Dyn<f64>>: DynOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2Dyn<f32>>: DynOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2Dyn<f64>>: DynOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiDyn<f32>>: DynOrderEllipticFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiDyn<f64>>: DynOrderEllipticFilterParam<CC, Conf = CC>,

        Param<OmegaFirstOrder<f32>>: FirstOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaFirstOrder<f64>>: FirstOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1FirstOrder<f32>>: FirstOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1FirstOrder<f64>>: FirstOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2FirstOrder<f32>>: FirstOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2FirstOrder<f64>>: FirstOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiFirstOrder<f32>>: FirstOrderEllipticFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiFirstOrder<f64>>: FirstOrderEllipticFilterParam<CC, Conf = CC>,
        
        Param<OmegaSecondOrder<f32>>: SecondOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaSecondOrder<f64>>: SecondOrderButterworthFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1SecondOrder<f32>>: SecondOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb1SecondOrder<f64>>: SecondOrderChebyshev1FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2SecondOrder<f32>>: SecondOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonCheb2SecondOrder<f64>>: SecondOrderChebyshev2FilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiSecondOrder<f32>>: SecondOrderEllipticFilterParam<CC, Conf = CC>,
        Param<OmegaEpsilonXiSecondOrder<f64>>: SecondOrderEllipticFilterParam<CC, Conf = CC>
    {

    }
}