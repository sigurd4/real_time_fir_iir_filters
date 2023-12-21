use array_math::ArrayOps;

use super::*;

#[derive(Copy, Clone)]
pub struct SecondOrderSallenKeyFilter<F, R1 = F, R2 = F, C1 = F, C2 = F, G = F>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    G: Param<F>
{
    pub r1: R1,
    pub r2: R2,
    pub c1: C1,
    pub c2: C2,
    pub g: G,
    pub w: [[F; 2]; 4]
}

impl<F, R1, R2, C1, C2, G> SecondOrderSallenKeyFilter<F, R1, R2, C1, C2, G>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    G: Param<F>
{
    pub fn new(r1: R1, r2: R2, c1: C1, c2: C2, g: G) -> Self
    {
        Self {
            r1,
            r2,
            c1,
            c2,
            g,
            w: [[F::zero(); 2]; 4]
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
    pub fn c1(&self) -> F
    {
        *(&self.c1).deref()
    }
    pub fn c2(&self) -> F
    {
        *(&self.c2).deref()
    }
    pub fn g(&self) -> F
    {
        *(&self.g).deref()
    }
}

iir2_impl!(
    <R1, R2, C1, C2, G> SecondOrderSallenKeyFilter<F, R1, R2, C1, C2, G>: 4: true =>
    SecondOrderSallenKeyFilter<f32>;
    SecondOrderSallenKeyFilter<f64>
    where
        R1: Param<F>,
        R2: Param<F>,
        C1: Param<F>,
        C2: Param<F>,
        G: Param<F>
);

impl<F, R1, R2, C1, C2, G> FilterStaticCoefficients<F> for SecondOrderSallenKeyFilter<F, R1, R2, C1, C2, G>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    G: Param<F>

{
    fn b(&self, rate: F) -> ([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS])
    {
        let rate2 = rate*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let c1 = self.c1();
        let c2 = self.c2();
        let g = self.g();

        ([], [
            [
                g*(g),
                g*f!(2.0),
                g*(g),
            ],
            [
                c1*g*r1*rate*f!(2.0),
                f!(0.0),
                c1*g*r1*rate*(-f!(2.0)),
            ],
            [
                c2*g*r2*rate*(f!(2.0)),
                f!(0.0),
                c2*g*r2*rate*(-f!(2.0)),
            ],
            [
                c1*c2*g*r1*r2*rate2*(f!(4.0)),
                c1*c2*g*r1*r2*rate2*(-f!(8.0)),
                c1*c2*g*r1*r2*rate2*(f!(4.0)),
            ]
        ])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])>
    {
        let rate2 = rate*rate;

        let r1 = self.r1();
        let r2 = self.r2();
        let c1 = self.c1();
        let c2 = self.c2();
        let g = self.g();

        Some(([], [
            [
                f!(1.0) + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate - f!(2.0)*c1*g*r1*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                f!(2.0) - f!(8.0)*c1*c2*r1*r2*rate2,
                f!(1.0) - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate + f!(2.0)*c1*g*r1*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
            ],
            [
                f!(1.0) - g + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                f!(2.0) - f!(2.0)*g - f!(8.0)*c1*c2*r1*r2*rate2,
                f!(1.0) - g - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
            ],
            [
                f!(1.0) + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate - f!(4.0)*c1*c2*g*r1*r2*rate2 + f!(4.0)*c1*c2*r1*r2*rate2,
                f!(2.0) + f!(8.0)*c1*c2*g*r1*r2*rate2 - f!(8.0)*c1*c2*r1*r2*rate2,
                f!(1.0) - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate - f!(4.0)*c1*c2*g*r1*r2*rate2 + f!(4.0)*c1*c2*r1*r2*rate2,
            ],
            [
                f!(1.0) - f!(2.0)*c2*g*r2*rate + f!(2.0)*c2*r1*rate + f!(2.0)*c2*r2*rate + f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
                f!(2.0) - f!(8.0)*c1*c2*r1*r2*rate2,
                f!(1.0) + f!(2.0)*c2*g*r2*rate - f!(2.0)*c2*r1*rate - f!(2.0)*c2*r2*rate - f!(2.0)*c1*r1*rate + f!(4.0)*c1*c2*r1*r2*rate2,
            ]
        ]))
    }
}

impl<F, R1, R2, C1, C2> FilterStaticInternals<F> for SecondOrderSallenKeyFilter<F, R1, R2, C1, C2>
where
    F: Float,
    R1: Param<F>,
    R2: Param<F>,
    C1: Param<F>,
    C2: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 4]; 0], &mut [[F; 2]; 4])
    {
        ([], &mut self.w)
    }
}

#[cfg(test)]
mod test
{
    use super::SecondOrderSallenKeyFilter;

    #[test]
    fn plot()
    {
        let mut filter = SecondOrderSallenKeyFilter::new(15.0e3, 15.0e3, 2.7e-9, 2.7e-9, 2.0);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}