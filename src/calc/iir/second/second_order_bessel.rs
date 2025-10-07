use crate::{param::{FilterFloat, Omega, OmegaSecondOrder}, util};

pub struct SecondOrderBesselCalc<F, L = (), H = ()>
where
    F: FilterFloat
{
    six_rate_omega: F,
    four_rate2: L,
    eight_rate2: L,
    twelve_rate2: H,
    twenty_four_rate2: H,
    three_omega2: L,
    six_omega2: L,
    omega2: H,
    two_omega2: H
}
impl<F, L, H> SecondOrderBesselCalc<F, L, H>
where
    F: FilterFloat,
    L: Default,
    H: Default
{
    pub fn new(omega: OmegaSecondOrder<F>, rate: F) -> Self
    {
        let Omega {omega} = omega;

        let one = F::one();
        let two = one + one;
        let three = two + one;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let six_rate_omega = three*two_rate*omega;
        let omega2 = omega*omega;

        fn mul_one_two<F, U>(x: impl FnOnce() -> F) -> (U, U)
        where
            U: Default,
            F: FilterFloat
        {
            util::same::eval_if_same(|| {
                let one_x = x();
                let two_x = one_x + one_x;
                (one_x, two_x)
            }, Default::default)
        }

        let (twelve_rate2, twenty_four_rate2) = mul_one_two(|| three*four_rate2);
        let (three_omega2, six_omega2) = mul_one_two(|| three*omega2);
        let (four_rate2, eight_rate2) = mul_one_two(|| four_rate2);
        let (omega2, two_omega2) = mul_one_two(|| omega2);
        
        Self {
            six_rate_omega,
            four_rate2,
            eight_rate2,
            twelve_rate2,
            twenty_four_rate2,
            three_omega2,
            six_omega2,
            omega2,
            two_omega2
        }
    }
}

impl<F, H> SecondOrderBesselCalc<F, F, H>
where
    F: FilterFloat
{
    pub fn b_low(&self) -> [F; 3]
    {
        [
            self.three_omega2,
            self.six_omega2,
            self.three_omega2
        ]
    }
    pub fn a_low(&self) -> [F; 3]
    {
        [
            self.four_rate2 - self.six_rate_omega + self.three_omega2,
            -self.eight_rate2 + self.six_omega2,
            self.four_rate2 + self.six_rate_omega + self.three_omega2,
        ]
    }
}

impl<F, L> SecondOrderBesselCalc<F, L, F>
where
    F: FilterFloat
{
    pub fn b_high(&self) -> [F; 3]
    {
        [
            self.twelve_rate2,
            -self.twenty_four_rate2,
            self.twelve_rate2
        ]
    }
    pub fn a_high(&self) -> [F; 3]
    {
        [
            self.twelve_rate2 - self.six_rate_omega + self.omega2,
            -self.twenty_four_rate2 + self.two_omega2,
            self.twelve_rate2 + self.six_rate_omega + self.omega2
        ]
    }
}