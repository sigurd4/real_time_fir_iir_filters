use num::{Float, One};

use cond::{Cond, True};

use crate::{conf::{all, All, Conf, HighPass, LowPass}, params::{OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb2Dyn}, util::same::Same};

use super::{Chebyshev1FilterParam, Chebyshev2FilterParam, FilterParam};

pub trait EllipticFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase: EllipticFilterParamBase<All, ImplBase = Self::ImplBase>;
}

pub trait EllipticFilterParam<
    C,
    ImplBase = <Self as EllipticFilterParamBase<C>>::ImplBase
>: EllipticFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: EllipticFilterConf;

    fn omega(&self) -> Self::F;
    fn epsilon(&self) -> Self::F;
    fn xi(&self) -> Self::F;
}

impl<P, C> EllipticFilterParam<C, OmegaEpsilonCheb1Dyn<P::F>> for P
where
    P: Chebyshev1FilterParam<C, Conf: EllipticFilterConf>,
    C: Conf
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        Chebyshev1FilterParam::omega(self)
    }
    fn epsilon(&self) -> Self::F
    {
        Chebyshev1FilterParam::epsilon(self)
    }
    fn xi(&self) -> Self::F
    {
        Float::infinity()
    }
}

impl<P, C> EllipticFilterParam<C, OmegaEpsilonCheb2Dyn<P::F>> for P
where
    P: Chebyshev2FilterParam<C, Conf: EllipticFilterConf>,
    C: Conf,
    Cond<{can_ln_be_calculated_through_recursion::<P, C>()}>: True // For now. It is possible to do it otherwise, but not implemented yet
{
    type Conf = P::Conf;

    fn omega(&self) -> Self::F
    {
        Chebyshev2FilterParam::omega(self)
    }
    fn epsilon(&self) -> Self::F
    {
        let omega = Chebyshev2FilterParam::omega(self);
        let mut xi = omega.recip();

        // https://en.wikipedia.org/wiki/Elliptic_rational_functions

        let ln = match Self::ORDER
        {
            0 => panic!(),
            1 => xi,
            2 => {
                let one = One::one();
                let xi2 = xi*xi;
                
                let ln_sqrt = xi + (xi2 - one).sqrt();
                ln_sqrt*ln_sqrt
            },
            3 => {
                let one = One::one();
                let xi2 = xi*xi;
                let xi3 = xi2*xi;
                let two_xi2 = xi2 + xi2;
                let four_xi2 = two_xi2 + two_xi2;
                let eight_xi2 = four_xi2 + four_xi2;
                let twelve_xi2 = eight_xi2 + four_xi2;

                let mut g = four_xi2*(xi2 - one);
                let g2 = four_xi2 + (g*g).cbrt();
                g = g2.sqrt();
                let g3 = g2*g;

                let xp2 = two_xi2*g.sqrt()/((eight_xi2*(xi2 + one) + twelve_xi2*g - g3).sqrt() - g3.sqrt());

                let ln_rhs_sqrt = (one - xp2)/(xi2 - xp2);
                xi3*(ln_rhs_sqrt*ln_rhs_sqrt)
            },
            4 => {
                let one = <P::F as One>::one();
                let xi2 = xi*xi;
                let sqrt_one_m_xi2 = (one - xi2).sqrt();

                let ln_lhs_4thrt = xi.sqrt() + sqrt_one_m_xi2.sqrt();
                let ln_lhs_sqrt = ln_lhs_4thrt*ln_lhs_4thrt;

                let ln_rhs_sqrt = xi + sqrt_one_m_xi2;

                (ln_lhs_sqrt*ln_lhs_sqrt)*(ln_rhs_sqrt*ln_rhs_sqrt)
            },
            _ => {
                if can_ln_be_calculated_through_recursion::<P, C>()
                {
                    let one = <P::F as One>::one();

                    let mut o = Self::ORDER;
                    loop
                    {
                        match o
                        {
                            0 => panic!(),
                            1 => break xi,
                            _ => {
                                let xi2 = xi*xi;

                                if o % 4 == 0
                                {
                                    let sqrt_one_m_xi2 = (one - xi2).sqrt();
                
                                    let ln_lhs_4thrt = xi.sqrt() + sqrt_one_m_xi2.sqrt();
                                    let ln_lhs_sqrt = ln_lhs_4thrt*ln_lhs_4thrt;
                
                                    let ln_rhs_sqrt = xi + sqrt_one_m_xi2;
                
                                    xi = (ln_lhs_sqrt*ln_lhs_sqrt)*(ln_rhs_sqrt*ln_rhs_sqrt);
                                    o /= 4
                                }
                                else if o % 3 == 0
                                {
                                    let xi3 = xi2*xi;
                                    let two_xi2 = xi2 + xi2;
                                    let four_xi2 = two_xi2 + two_xi2;
                                    let eight_xi2 = four_xi2 + four_xi2;
                                    let twelve_xi2 = eight_xi2 + four_xi2;
                
                                    let mut g = four_xi2*(xi2 - one);
                                    let g2 = four_xi2 + (g*g).cbrt();
                                    g = g2.sqrt();
                                    let g3 = g2*g;
                
                                    let xp2 = two_xi2*g.sqrt()/((eight_xi2*(xi2 + one) + twelve_xi2*g - g3).sqrt() - g3.sqrt());
                
                                    let ln_rhs_sqrt = (one - xp2)/(xi2 - xp2);
                                    xi = xi3*(ln_rhs_sqrt*ln_rhs_sqrt);
                                    o /= 3
                                }
                                else if o % 2 == 0
                                {                                        
                                    let ln_sqrt = xi + (xi2 - one).sqrt();
                                    xi = ln_sqrt*ln_sqrt;
                                    o /= 2
                                }
                            }
                        }
                    }
                }
                else
                {
                    todo!()
                }
            }
        };

        let alpha = Chebyshev2FilterParam::epsilon(self)*ln;
        alpha
    }
    fn xi(&self) -> Self::F
    {
        Chebyshev2FilterParam::omega(self).recip()
    }
}

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

const fn can_ln_be_calculated_through_recursion<P, C>() -> bool
where
    P: Chebyshev2FilterParam<C>,
    C: Conf
{
    let mut o = P::ORDER;

    loop
    {
        match o
        {
            0 => return false,
            1..=4 => return true,
            _ => {
                if o % 4 == 0
                {
                    o /= 4
                }
                else if o % 3 == 0
                {
                    o /= 3
                }
                else if o % 2 == 0
                {
                    o /= 2
                }
                else
                {
                    return false
                }
            }
        }
    }
}

mod private
{
    use crate::{filters::iir::second::{SecondOrderChebyshev1Filter, SecondOrderChebyshev2Filter, SecondOrderEllipticFilter}, param::{ButterworthFilterConf, ButterworthFilterParam, Chebyshev1FilterParam, Chebyshev2FilterParam, ChebyshevFilterConf, FilterParamFirstOrder, FilterParamSecondOrder}, params::{OmegaDyn, OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb1FirstOrder, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb2Dyn, OmegaEpsilonCheb2FirstOrder, OmegaEpsilonCheb2SecondOrder, OmegaEpsilonXiDyn, OmegaEpsilonXiFirstOrder, OmegaEpsilonXiSecondOrder, OmegaFirstOrder, OmegaSecondOrder}, rtf::Rtf};

    use super::{EllipticFilterConf, EllipticFilterParam};

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
        const OUTPUTS: usize
    > EllipticFilterConfFinal<C> for CC
    where
        CC: EllipticFilterConf<
            Conf = Self,
            OUTPUTS = {OUTPUTS}
        > + ChebyshevFilterConf<
            Conf = Self,
            OUTPUTS = {OUTPUTS}
        > + ButterworthFilterConf<
            0,
            Conf = Self,
            OUTPUTS = {OUTPUTS}
        >,
        C: EllipticFilterConf<
            Conf = CC,
            OUTPUTS = {OUTPUTS}
        >,
        OmegaDyn<f32>: ButterworthFilterParam<CC, Conf = CC>,
        OmegaDyn<f64>: ButterworthFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f32>: Chebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f64>: Chebyshev1FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f32>: Chebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f64>: Chebyshev2FilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f32>: EllipticFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f64>: EllipticFilterParam<CC, Conf = CC>,

        OmegaFirstOrder<f32>: ButterworthFilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaFirstOrder<f64>: ButterworthFilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonCheb1FirstOrder<f32>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonCheb1FirstOrder<f64>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonCheb2FirstOrder<f32>: Chebyshev2FilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonCheb2FirstOrder<f64>: Chebyshev2FilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonXiFirstOrder<f32>: EllipticFilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        OmegaEpsilonXiFirstOrder<f64>: EllipticFilterParam<CC, Conf = CC> + FilterParamFirstOrder,
        
        OmegaSecondOrder<f32>: ButterworthFilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaSecondOrder<f64>: ButterworthFilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonCheb1SecondOrder<f32>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonCheb1SecondOrder<f64>: Chebyshev1FilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonCheb2SecondOrder<f32>: Chebyshev2FilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonCheb2SecondOrder<f64>: Chebyshev2FilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonXiSecondOrder<f32>: EllipticFilterParam<CC, Conf = CC> + FilterParamSecondOrder,
        OmegaEpsilonXiSecondOrder<f64>: EllipticFilterParam<CC, Conf = CC> + FilterParamSecondOrder,

        SecondOrderChebyshev1Filter<f32, OmegaEpsilonCheb1SecondOrder<f32>, CC>: Rtf,
        SecondOrderChebyshev1Filter<f64, OmegaEpsilonCheb1SecondOrder<f64>, CC>: Rtf,
        SecondOrderChebyshev2Filter<f32, OmegaEpsilonCheb2SecondOrder<f32>, CC>: Rtf,
        SecondOrderChebyshev2Filter<f64, OmegaEpsilonCheb2SecondOrder<f64>, CC>: Rtf,
        [(); <CC as ChebyshevFilterConf>::OUTPUTS]:,

        SecondOrderEllipticFilter<f32, OmegaEpsilonXiSecondOrder<f32>, CC>: Rtf,
        SecondOrderEllipticFilter<f64, OmegaEpsilonXiSecondOrder<f64>, CC>: Rtf,
        [(); <CC as EllipticFilterConf>::OUTPUTS]:
    {

    }
}