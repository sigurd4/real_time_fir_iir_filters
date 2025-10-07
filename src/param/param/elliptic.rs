use num::{Float, One};

use crate::{conf::Conf, param::{EllipticFilterParamBase, FilterParam, OmegaEpsilon, OmegaEpsilonXi, EllipticFilterConf}, util::same::Same};

use super::ChebyshevFilterParam;

pub trait EllipticFilterParam<
    C,
    ImplBase = <Self as EllipticFilterParamBase<C>>::ImplBase
>: EllipticFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: EllipticFilterConf;
    type OmegaEpsilonXi//: Same<OmegaEpsilonXi<Self::F, {Self::ORDER}>> // I don't know why this doesn't work :(
    where
        [(); Self::ORDER]:;

    fn omega_epsilon_xi(&self) -> Self::OmegaEpsilonXi
    where
        [(); Self::ORDER]:;
}

macro_rules! special {
    ($trait:ident = $order:expr) => {
        pub trait $trait<C>: FilterParam
        where
            C: Conf
        {
            type Conf: EllipticFilterConf;

            fn omega_epsilon_xi(&self) -> OmegaEpsilonXi<<Self as FilterParam>::F, $order>;
        }
        impl<P, C> $trait<C> for P
        where
            P: EllipticFilterParam<C, ORDER = $order, OmegaEpsilonXi = OmegaEpsilonXi<<Self as FilterParam>::F, $order>>,
            C: Conf,
            [(); Self::ORDER]:
        {
            type Conf = <Self as EllipticFilterParam<C>>::Conf;

            fn omega_epsilon_xi(&self) -> OmegaEpsilonXi<<Self as FilterParam>::F, $order>
            {
                EllipticFilterParam::omega_epsilon_xi(self)
            }
        }
    };
}

special!(DynOrderEllipticFilterParam = 0);
special!(FirstOrderEllipticFilterParam = 1);
special!(SecondOrderEllipticFilterParam = 2);
special!(ThirdOrderEllipticFilterParam = 3);

const fn can_ln_be_calculated_through_recursion(cheb_type: bool, mut order: usize) -> bool
{
    if cheb_type
    {
        loop
        {
            match order
            {
                0 => return false,
                1..=4 => return true,
                _ => {
                    if order.is_multiple_of(4)
                    {
                        order /= 4
                    }
                    else if order.is_multiple_of(3)
                    {
                        order /= 3
                    }
                    else if order.is_multiple_of(2)
                    {
                        order /= 2
                    }
                    else
                    {
                        return false
                    }
                }
            }
        }
    }
    else
    {
        true
    }
}

impl<P, C, const ORDER: usize> EllipticFilterParam<C, OmegaEpsilon<<P as FilterParam>::F, false, ORDER>> for P
where
    P: ChebyshevFilterParam<C, TYPE = false, ORDER = {ORDER}, OmegaEpsilon = OmegaEpsilon<<P as FilterParam>::F, false, ORDER>, Conf: EllipticFilterConf> + EllipticFilterParamBase<C, ImplBase: Same<OmegaEpsilon<<P as FilterParam>::F, false, ORDER>>>,
    C: Conf,
    OmegaEpsilonXi<P::F, ORDER>: Same<OmegaEpsilonXi<P::F, {Self::ORDER}>>,
    [(); {Self::TYPE} as usize]:
{
    type Conf = P::Conf;
    type OmegaEpsilonXi = OmegaEpsilonXi<P::F, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega_epsilon_xi(&self) -> Self::OmegaEpsilonXi
    where
        [(); Self::ORDER]:
    {
        let OmegaEpsilon {omega, epsilon} = self.omega_epsilon();
        OmegaEpsilonXi {
            omega,
            epsilon,
            xi: Float::infinity()
        }
    }
}

impl<P, C, const TYPE: bool, const ORDER: usize> EllipticFilterParam<C, OmegaEpsilon<<P as FilterParam>::F, TYPE, ORDER>> for P
where
    P: ChebyshevFilterParam<C, TYPE = {TYPE}, ORDER = {ORDER}, OmegaEpsilon = OmegaEpsilon<<P as FilterParam>::F, TYPE, ORDER>, Conf: EllipticFilterConf> + EllipticFilterParamBase<C, ImplBase: Same<OmegaEpsilon<<P as FilterParam>::F, TYPE, ORDER>>>,
    C: Conf,
    OmegaEpsilonXi<P::F, ORDER>: Same<OmegaEpsilonXi<P::F, {Self::ORDER}>>,
    [(); {Self::TYPE} as usize]:,
    [(); can_ln_be_calculated_through_recursion(Self::TYPE, Self::ORDER) as usize - 1]: // For now. It is possible to do it otherwise, but not implemented yet
{
    type Conf = P::Conf;
    type OmegaEpsilonXi = OmegaEpsilonXi<P::F, ORDER>
    where
        [(); Self::ORDER]:;

    fn omega_epsilon_xi(&self) -> Self::OmegaEpsilonXi
    where
        [(); Self::ORDER]:
    {
        let OmegaEpsilon {omega, epsilon} = self.omega_epsilon();

        if TYPE
        {
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
                    if can_ln_be_calculated_through_recursion(Self::TYPE, Self::ORDER)
                    {
                        let mut o = Self::ORDER;
                        
                        let one = <P::F as One>::one();

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
                        todo!("Radix not supported.")
                    }
                }
            };

            OmegaEpsilonXi {
                omega,
                epsilon: epsilon*ln,
                xi: omega.recip()
            }
        }
        else
        {
            OmegaEpsilonXi {
                omega,
                epsilon,
                xi: Float::infinity()
            }
        }
    }
}