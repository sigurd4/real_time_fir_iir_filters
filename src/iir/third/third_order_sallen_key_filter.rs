use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct ThirdOrderSallenKeyFilter<F, R1 = F, R2 = F, R3 = F, C1 = F, C2 = F, C3 = F, G = F>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    R3: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    C3: Param<F>,
    G: Param<F>
{
    pub r1: R1,
    pub r2: R2,
    pub r3: R3,
    pub c1: C1,
    pub c2: C2,
    pub c3: C3,
    pub g: G,
    pub w: [[F; 3]; 8]
}

impl<F, R1, R2, R3, C1, C2, C3, G> ThirdOrderSallenKeyFilter<F, R1, R2, R3, C1, C2, C3, G>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    R3: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    C3: Param<F>,
    G: Param<F>
{
    pub fn new(r1: R1, r2: R2, r3: R3, c1: C1, c2: C2, c3: C3, g: G) -> Self
    {
        Self
        {
            r1,
            r2,
            r3,
            c1,
            c2,
            c3,
            g,
            w: [[F::zero(); 3]; 8]
        }
    }
    
    
    pub fn r1(&self) -> F
    {
        *(&self.r1).deref()
    }
    pub fn r2(&self) -> F
    {
        *(&self.r2).deref()
    }
    pub fn r3(&self) -> F
    {
        *(&self.r3).deref()
    }
    pub fn c1(&self) -> F
    {
        *(&self.c1).deref()
    }
    pub fn c2(&self) -> F
    {
        *(&self.c2).deref()
    }
    pub fn c3(&self) -> F
    {
        *(&self.c3).deref()
    }
    pub fn g(&self) -> F
    {
        *(&self.g).deref()
    }
}

iir3_impl!(
    <R1, R2, R3, C1, C2, C3, G> ThirdOrderSallenKeyFilter<F, R1, R2, R3, C1, C2, C3, G>: 8: true =>
    ThirdOrderSallenKeyFilter<f32>;
    ThirdOrderSallenKeyFilter<f64>
    where
        R1: Param<F>,
        R2: Param<F>,
        R3: Param<F>,
        C1: Param<F>,
        C2: Param<F>,
        C3: Param<F>,
        G: Param<F>
);

impl<F, R1, R2, R3, C1, C2, C3> FilterStaticCoefficients<F> for ThirdOrderSallenKeyFilter<F, R1, R2, R3, C1, C2, C3>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    R3: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    C3: Param<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; 8]; 0], [[F; 4]; 8])
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let r3 = self.r3();
        let c1 = self.c1();
        let c2 = self.c2();
        let c3 = self.c3();
        let g = self.g();
        
        ([], [
            [
                g*r2*f!(1.0),
                g*r2*f!(3.0),
                g*r2*f!(3.0),
                g*r2*f!(1.0),
            ],
            [
                c1*g*r1*r2*rate*f!(2.0),
                c1*g*r1*r2*rate*f!(2.0),
                c1*g*r1*r2*rate*f!(-2.0),
                c1*g*r1*r2*rate*f!(-2.0),
            ],
            [
                c2*g*r2*rate*f!(2.0),
                c2*g*r2*rate*f!(2.0),
                c2*g*r2*rate*f!(-2.0),
                c2*g*r2*rate*f!(-2.0),
            ],
            [
                c1*c2*g*r1*r2*rate2*f!(4.0),
                c1*c2*g*r1*r2*rate2*f!(-4.0),
                c1*c2*g*r1*r2*rate2*f!(-4.0),
                c1*c2*g*r1*r2*rate2*f!(4.0),
            ],
            [
                c3*g*r2*r3*rate*f!(2.0),
                c3*g*r2*r3*rate*f!(2.0),
                c3*g*r2*r3*rate*f!(-2.0),
                c3*g*r2*r3*rate*f!(-2.0),
            ],
            [
                c1*c3*g*r1*r2*r3*rate2*f!(4.0),
                c1*c3*g*r1*r2*r3*rate2*f!(-4.0),
                c1*c3*g*r1*r2*r3*rate2*f!(-4.0),
                c1*c3*g*r1*r2*r3*rate2*f!(4.0),
            ],
            [
                c2*c3*g*r2*r3*rate2*f!(4.0),
                c2*c3*g*r2*r3*rate2*f!(-4.0),
                c2*c3*g*r2*r3*rate2*f!(-4.0),
                c2*c3*g*r2*r3*rate2*f!(4.0),
            ],
            [
                c1*c2*c3*g*r1*r2*r3*rate3*f!(8.0),
                c1*c2*c3*g*r1*r2*r3*rate3*f!(-24.0),
                c1*c2*c3*g*r1*r2*r3*rate3*f!(24.0),
                c1*c2*c3*g*r1*r2*r3*rate3*f!(-8.0),
            ],
        ])
    }
    
    fn a(&self, rate: F) -> Option<([[[F; 3]; 8]; 0], [[F; 4]; 8])>
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let r3 = self.r3();
        let c1 = self.c1();
        let c2 = self.c2();
        let c3 = self.c3();
        let g = self.g();

        Some(([], [
            [
                f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + r2 + f!(2.0)*r1,
                -f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(3.0)*r2 + f!(6.0)*r1,
                f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate + f!(3.0)*r2 + f!(6.0)*r1,
                -f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate + r2 + f!(2.0)*r1,
            ],
            [
                f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(2.0)*r1 + r2,
                -f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*g*r1*r2*rate - f!(2.0)*c2*g*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(6.0)*r1 + f!(3.0)*r2,
                f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(4.0)*c1*c2*g*r1*r2*r2*rate2 - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate + f!(6.0)*r1 + f!(3.0)*r2,
                -f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(4.0)*c1*c2*g*r1*r2*r2*rate2 + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*g*r1*r2*rate + f!(2.0)*c2*g*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate + f!(2.0)*r1 + r2,
            ],
            [
                f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(2.0)*c2*g*r1*rate + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(2.0)*c2*g*r1*rate - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(2.0)*c2*g*r1*rate + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(2.0)*c2*g*r1*rate - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
            ],
            [
                f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c2*g*r1*rate + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
                -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(2.0)*c1*r1*rate - f!(2.0)*c1*g*r1*rate - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c2*g*r1*rate - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                -f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(3.0) - f!(3.0)*g - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c2*g*r1*rate + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3 - f!(8.0)*c2*c2*r1*r2*rate2,
                f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(2.0)*c1*r1*rate + f!(2.0)*c1*g*r1*rate - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(1.0) - g + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c2*g*r1*rate - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3 + f!(8.0)*c2*c2*r1*r2*rate2,
            ],
            [
                f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + r2 + f!(2.0)*r1 + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate,
                -f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r2*r2*rate + f!(2.0)*c2*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(3.0)*r2 + f!(6.0)*r1 + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate,
                -f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate - f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(3.0)*r2 + f!(6.0)*r1 - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate,
                f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r2*r2*rate - f!(2.0)*c2*r1*r2*rate + f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + r2 + f!(2.0)*r1 - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate,
            ],
            [
                f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(2.0)*r1 + r2 + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate,
                -f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 + f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 + f!(2.0)*c2*r1*r2*rate + f!(2.0)*c2*r2*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(6.0)*r1 + f!(3.0)*r2 + f!(4.0)*c3*r1*r3*rate + f!(2.0)*c3*r1*r2*rate + f!(2.0)*c3*r2*r3*rate + f!(2.0)*c3*r2*r2*rate,
                -f!(4.0)*c1*c2*r1*r2*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r2*r3*rate3 - f!(24.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate - f!(4.0)*c1*c3*r1*r2*r3*rate2 - f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate - f!(4.0)*c2*c3*r1*r2*r3*rate2 - f!(4.0)*c2*c3*r2*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r2*r3*rate2 + f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(6.0)*r1 + f!(3.0)*r2 - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate,
                f!(4.0)*c1*c2*r1*r2*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r2*r3*rate3 + f!(8.0)*c1*c2*c3*g*r1*r2*r2*r3*rate3 - f!(2.0)*c1*r1*r2*rate + f!(4.0)*c1*c3*r1*r2*r3*rate2 + f!(4.0)*c1*c3*r1*r2*r2*rate2 - f!(2.0)*c2*r1*r2*rate - f!(2.0)*c2*r2*r2*rate + f!(4.0)*c2*c3*r1*r2*r3*rate2 + f!(4.0)*c2*c3*r2*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r2*r3*rate2 - f!(4.0)*c2*c3*g*r2*r2*r3*rate2 + f!(2.0)*r1 + r2 - f!(4.0)*c3*r1*r3*rate - f!(2.0)*c3*r1*r2*rate - f!(2.0)*c3*r2*r3*rate - f!(2.0)*c3*r2*r2*rate,
            ],
            [
                f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c2*r1*rate - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                -f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate - f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                -f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c2*r1*rate + f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
            ],
            [
                f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 + f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 + f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
                f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 - f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) + f!(2.0)*c3*r2*rate + f!(2.0)*c3*r3*rate - f!(2.0)*c3*g*r3*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 - f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                -f!(2.0)*c1*r1*rate - f!(4.0)*c1*c3*r1*r2*rate2 - f!(4.0)*c1*c3*r1*r3*rate2 + f!(4.0)*c1*c3*g*r1*r3*rate2 - f!(4.0)*c1*c2*r1*r2*rate2 + f!(24.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(3.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(4.0)*c2*c3*r1*r2*rate2 - f!(4.0)*c2*c3*r1*r3*rate2 - f!(4.0)*c2*c3*r2*r3*rate2 + f!(4.0)*c2*c3*g*r1*r3*rate2 - f!(8.0)*c2*c2*r1*r2*rate2 + f!(48.0)*c2*c2*c3*r1*r2*r3*rate3,
                -f!(2.0)*c1*r1*rate + f!(4.0)*c1*c3*r1*r2*rate2 + f!(4.0)*c1*c3*r1*r3*rate2 - f!(4.0)*c1*c3*g*r1*r3*rate2 + f!(4.0)*c1*c2*r1*r2*rate2 - f!(8.0)*c1*c2*c3*r1*r2*r3*rate3 + f!(1.0) - f!(2.0)*c3*r2*rate - f!(2.0)*c3*r3*rate + f!(2.0)*c3*g*r3*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(4.0)*c2*c3*r1*r2*rate2 + f!(4.0)*c2*c3*r1*r3*rate2 + f!(4.0)*c2*c3*r2*r3*rate2 - f!(4.0)*c2*c3*g*r1*r3*rate2 + f!(8.0)*c2*c2*r1*r2*rate2 - f!(16.0)*c2*c2*c3*r1*r2*r3*rate3,
            ],
        ]))
    }
}

impl<F, R1, R2, R3, C1, C2, C3> FilterStaticInternals<F> for ThirdOrderSallenKeyFilter<F, R1, R2, R3, C1, C2, C3>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    R3: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    C3: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 8]; 0], &mut [[F; 3]; 8])
    {
        ([], &mut self.w)
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::ThirdOrderSallenKeyFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderSallenKeyFilter::new(470.0, 15.0e3, 16.0e3, 47.0e-9, 2.7e-9, 2.7e-9, 1.3846153846153846);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}