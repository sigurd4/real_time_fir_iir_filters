use crate::{param::{FilterFloat, Omega2Zeta}, util};

pub struct ThirdOrderCalc<F, L = (), P1 = (), P2 = ()>
where
    F: FilterFloat
{
    three: F,
    rate: F,
    two_rate: F,
    four_rate: F,
    four_rate2: F,
    eight_rate3: F,
    twenty_four_rate3: F,
    omega1: F,
    omega2: F,
    omega2_2: F,
    zeta: F,
    k3: L,
    k2: P1,
    k: P2
}
impl<F, L, P1, P2> ThirdOrderCalc<F, L, P1, P2>
where
    F: FilterFloat,
    L: Default + Copy,
    P1: Default,
    P2: Default + Copy
{
    pub fn new(omega2_zeta: Omega2Zeta<F>, rate: F) -> Self
    {
        let Omega2Zeta {omega1, omega2, zeta} = omega2_zeta;

        let omega2_2 = omega2*omega2;

        let one = F::one();
        let two = one + one;
        let three = two + one;

        let two_rate = rate + rate;
        let four_rate = two_rate + two_rate;
        let four_rate2 = two_rate*two_rate;
        let eight_rate3 = four_rate2*two_rate;
        let twenty_four_rate3 = three*eight_rate3;

        let k3_f = || omega1*omega2_2;
        let k3 = util::same::eval_if_same(k3_f, Default::default);

        let k_f = || {
            let k3 = util::same::eval_if_same(|| k3, k3_f);
            k3.cbrt()
        };
        let k = util::same::eval_if_same(k_f, Default::default);

        let k2_f = || {
            let k = util::same::eval_if_same(|| k, k_f);
            k*k
        };
        let k2 = util::same::eval_if_same(k2_f, Default::default);

        Self {
            three,
            rate,
            two_rate,
            four_rate,
            four_rate2,
            eight_rate3,
            twenty_four_rate3,
            omega1,
            omega2,
            omega2_2,
            zeta,
            k3,
            k2,
            k
        }
    }
}
impl<F, P1, P2> ThirdOrderCalc<F, F, P1, P2>
where
    F: FilterFloat
{
    pub fn b_low(&self) -> [F; 4]
    {
        let three_k3 = self.three*self.k3;
        [
            self.k3,
            three_k3,
            three_k3,
            self.k3
        ]
    }
}
impl<F, L, P2> ThirdOrderCalc<F, L, F, P2>
where
    F: FilterFloat
{
    pub fn b_peak1(&self) -> [F; 4]
    {
        let two_k2_rate = self.k2*self.two_rate;
        let m_two_k2_rate = -two_k2_rate;
        [
            two_k2_rate,
            two_k2_rate,
            m_two_k2_rate,
            m_two_k2_rate
        ]
    }
}
impl<F, L, P1> ThirdOrderCalc<F, L, P1, F>
where
    F: FilterFloat
{
    pub fn b_peak2(&self) -> [F; 4]
    {
        let four_k_rate2 = self.k*self.four_rate2;
        let m_four_k_rate2 = -four_k_rate2;
        [
            four_k_rate2,
            m_four_k_rate2,
            m_four_k_rate2,
            four_k_rate2
        ]
    }
}
impl<F, L, P1, P2> ThirdOrderCalc<F, L, P1, P2>
where
    F: FilterFloat
{
    pub fn b_high(&self) -> [F; 4]
    {
        [
            self.eight_rate3,
            -self.twenty_four_rate3,
            self.twenty_four_rate3,
            -self.eight_rate3
        ]
    }
    pub fn a(&self) -> [F; 4]
    {
        let four_rate_omega1 = self.four_rate*self.omega1;
        let omega2_zeta = self.omega2*self.zeta;
        let omega2_2_omega1 = self.omega2_2*self.omega1;
    
        let a_lhs_rhs = four_rate_omega1*self.rate + self.four_rate*self.two_rate*omega2_zeta;
        let a_rhs_lhs = self.omega2_2*self.two_rate + four_rate_omega1*omega2_zeta;
    
        let a0_lhs = omega2_2_omega1 + a_lhs_rhs;
        let a0_rhs = a_rhs_lhs + self.eight_rate3;
    
        let a1_lhs = self.three*omega2_2_omega1 - a_lhs_rhs;
        let a1_rhs = a_rhs_lhs - self.twenty_four_rate3;
        [
            a0_lhs + a0_rhs,
            a1_lhs + a1_rhs,
            a1_lhs - a1_rhs,
            a0_lhs - a0_rhs,
        ]
    }
}