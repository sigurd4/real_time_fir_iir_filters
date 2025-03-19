use num::{Float, One};

use crate::{conf::{all, All, Conf, HighPass, LowPass}, param::{EllipticFilterParamBase, FilterParam, OmegaEpsilonVal, OmegaEpsilonXiVal}, params::OmegaEpsilon, util::same::Same};

use super::{ChebyshevFilterParam, ChebyshevType};

pub trait EllipticFilterParam<
    C,
    ImplBase = <Self as EllipticFilterParamBase<C>>::ImplBase
>: EllipticFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: EllipticFilterConf;

    fn omega_epsilon_xi(&self) -> OmegaEpsilonXiVal<Self::F>;
}

impl<P, C> EllipticFilterParam<C, OmegaEpsilon<<P as FilterParam>::F, {P::TYPE}>> for P
where
    P: ChebyshevFilterParam<C, Conf: EllipticFilterConf> + EllipticFilterParamBase<C, ImplBase: Same<OmegaEpsilon<<P as FilterParam>::F, {P::TYPE}>>>,
    C: Conf,
    [(); can_ln_be_calculated_through_recursion::<P, C>() as usize - 1]: // For now. It is possible to do it otherwise, but not implemented yet
{
    type Conf = P::Conf;

    fn omega_epsilon_xi(&self) -> OmegaEpsilonXiVal<Self::F>
    {
        let OmegaEpsilonVal {omega, epsilon} = self.omega_epsilon();

        match P::TYPE
        {
            ChebyshevType::Type1 => OmegaEpsilonXiVal {
                omega,
                epsilon,
                xi: Float::infinity()
            },
            ChebyshevType::Type2 => {
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
        
                OmegaEpsilonXiVal {
                    omega,
                    epsilon: epsilon*ln,
                    xi: omega.recip()
                }
            }
        }
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
    P: ChebyshevFilterParam<C>,
    C: Conf
{
    match P::TYPE
    {
        ChebyshevType::Type1 => true,
        ChebyshevType::Type2 => {
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
    }
}

mod private
{
    use crate::{param::{ButterworthFilterConf, ButterworthFilterParam, ChebyshevFilterConf, ChebyshevFilterParam, FirstOrderFilterParam}, params::{OmegaDyn, OmegaEpsilonCheb1Dyn, OmegaEpsilonCheb1FirstOrder, OmegaEpsilonCheb1SecondOrder, OmegaEpsilonCheb2Dyn, OmegaEpsilonCheb2FirstOrder, OmegaEpsilonCheb2SecondOrder, OmegaEpsilonXiDyn, OmegaEpsilonXiFirstOrder, OmegaEpsilonXiSecondOrder, OmegaFirstOrder, OmegaSecondOrder}};

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
        OmegaEpsilonCheb1Dyn<f32>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1Dyn<f64>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f32>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb2Dyn<f64>: ChebyshevFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f32>: EllipticFilterParam<CC, Conf = CC>,
        OmegaEpsilonXiDyn<f64>: EllipticFilterParam<CC, Conf = CC>,

        OmegaFirstOrder<f32>: ButterworthFilterParam<CC, Conf = CC> + FirstOrderFilterParam<CC, Conf = CC>,
        OmegaFirstOrder<f64>: ButterworthFilterParam<CC, Conf = CC> + FirstOrderFilterParam<CC, Conf = CC>,
        OmegaEpsilonCheb1FirstOrder<f32>: ChebyshevFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb1FirstOrder<f64>: ChebyshevFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb2FirstOrder<f32>: ChebyshevFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb2FirstOrder<f64>: ChebyshevFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonXiFirstOrder<f32>: EllipticFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonXiFirstOrder<f64>: EllipticFilterParam<CC, Conf = CC> /*+ FirstOrderFilterParam<CC, Conf = CC>*/,
        
        OmegaSecondOrder<f32>: ButterworthFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaSecondOrder<f64>: ButterworthFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb1SecondOrder<f32>: ChebyshevFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb1SecondOrder<f64>: ChebyshevFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb2SecondOrder<f32>: ChebyshevFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonCheb2SecondOrder<f64>: ChebyshevFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonXiSecondOrder<f32>: EllipticFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
        OmegaEpsilonXiSecondOrder<f64>: EllipticFilterParam<CC, Conf = CC> /*+ SecondOrderFilterParam<CC, Conf = CC>*/,
    {

    }
}