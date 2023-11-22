use array_math::ArrayOps;

use super::*;

const R_I: f64 = 78000.0;
const R_S: f64 = 1500.0;
const R_P: f64 = 33000.0;
const R_C1: f64 = 22000.0;
const R_E1: f64 = 390.0;
#[allow(unused)]
const R_C2: f64 = 1000.0;
const R_E2: f64 = 10000.0;
const R_J: f64 = 470000.0;
const R_G: f64 = 82000.0;
const R_POT: f64 = 100000.0;

const V_CC: f64 = 9.0;
const V_F: f64 = 0.6;
const V_T: f64 = 0.026;

const BETA: f64 = 300.0; 

const S_G: f64 = 1.0/(R_S + BETA*R_E1) + 1.0/R_G + 1.0/R_J;
const S_C1: f64 = 1.0/R_C1 + 2.0/R_J + BETA/(R_S + BETA*R_E1)/R_J/S_G - 1.0/R_J/R_J/S_G;
const S_B2: f64 = 1.0/R_J + 1.0/BETA/R_E2 - 1.0/R_J/R_J/S_C1;

const V_B2: f64 = (V_CC/R_C1 + (1.0/(R_S + BETA*R_E1)/R_J/S_G - BETA*(1.0 - (R_S + BETA*R_E1)*S_G)/(R_S + BETA*R_E1)/(R_S + BETA*R_E1)/S_G)*V_F)/R_J/S_B2/S_C1 + V_F/R_E2/S_B2;
const V_C1: f64 = V_CC/R_C1/S_C1 + V_B2/R_J/S_C1 + (1.0/(R_S + BETA*R_E1)/R_J/S_G - BETA*(1.0 - (R_S + BETA*R_E1)*S_G)/(R_S + BETA*R_E1)/(R_S + BETA*R_E1)/S_G)/S_C1*V_F;
const V_G: f64 = V_F/(R_S + BETA*R_E1)/S_G + V_C1/R_J/S_G;

const I_C1: f64 = BETA*(V_G - V_F)/(R_S + BETA*R_E1);
#[allow(unused)]
const I_C2: f64 = (V_B2 - V_F)/R_E2;

const _R_E1: f64 = V_T/I_C1;
const _R_E2: f64 = V_T/I_C2;

const G_Q1: f64 = R_C1/(_R_E1 + R_E1);

#[allow(unused)]
const C_G: f64 = 0.0000047;
const C_I: f64 = 0.00000001;
const C_F: f64 = 0.00000001;

const L: f64 = 0.5;

#[derive(Copy, Clone)]
pub struct WahFilter<F, K = F>
where
    F: Float,
    K: Param<F>
{
    pub k: K,
    pub w: [F; 4]
}

impl<F, K> WahFilter<F, K>
where
    F: Float,
    K: Param<F>
{
    pub fn new(k: K) -> Self
    {
        Self {
            k,
            w: [F::zero(); 4]
        }
    }

    fn k(&self) -> F
    {
        *(&self.k).deref()
    }
}

const S_Q1: f64 = 1.0/BETA/(_R_E1 + R_E1);

const A1: f64 = R_S*C_I/L*(1.0 + R_I*S_Q1) + S_Q1*(1.0 + R_S/R_P) + R_I*C_I/L + 1.0/R_P;
const A0: f64 = (R_S*S_Q1 + 1.0)/L;

const B3: f64 = R_S*C_I*C_F;
const B2: f64 = C_I*(1.0 + R_S/R_P);
const B1: f64 = R_S*C_I/L;

iir4_impl!(
    <K> WahFilter<F, K>: 1: false =>
    WahFilter<f32>;
    WahFilter<f64>
    where
        K: Param<F>
);

impl<F, K> FilterStaticCoefficients<F> for WahFilter<F, K>
where
    F: Float,
    K: Param<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 5]; 1])
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;
        ([], [[
            f!(8.0*B3)*rate3 + f!(4.0*B2)*rate2 + f!(2.0*B1)*rate,
            f!(-16.0*B3)*rate3 + f!(4.0*B1)*rate,
            f!(-8.0*B2)*rate2,
            f!(16.0*B3)*rate3 - f!(4.0*B1)*rate,
            f!(-8.0*B3)*rate3 + f!(4.0*B2)*rate2 - f!(2.0*B1)*rate
        ].map(|b| b*f!(-G_Q1))])
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 5]; 1])>
    {
        let k = self.k();

        let one_m_k_recip_plus_r_pot_per_r_j = (f!(1.0) - k).recip() + f!(R_POT/R_J);

        let g_q2 = one_m_k_recip_plus_r_pot_per_r_j/(k.recip() + one_m_k_recip_plus_r_pot_per_r_j);

        let a3 = f!(C_I*C_F*(R_S*(1.0 + R_S*S_Q1) + R_I)) + f!(C_I*C_F*R_I*G_Q1)*g_q2;
        let a2 = f!((C_I*(1.0 + R_S/R_P) + C_F)*(1.0 + R_S*S_Q1) + C_I*R_I/R_P) + f!(C_F*G_Q1)*g_q2;

        let rate2 = rate*rate;
        let rate3 = rate2*rate;
        Some(([], [[
            f!(8.0)*rate3*a3 + f!(4.0)*rate2*a2 + f!(2.0*A1)*rate + f!(A0),
            f!(-16.0)*rate3*a3 + f!(4.0*A1)*rate + f!(4.0*A0),
            f!(-8.0)*rate2*a2 + f!(6.0*A0),
            f!(16.0)*rate3*a3 - f!(4.0*A1)*rate + f!(4.0*A0),
            f!(-8.0)*rate3*a3 + f!(4.0)*rate2*a2 - f!(2.0*A1)*rate + f!(A0)
        ]]))
    }
}

impl<F, K> FilterStaticInternals<F> for WahFilter<F, K>
where
    F: Float,
    K: Param<F>,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; 0]; 0], &mut [[F; 4]; 1])
    {
        ([], core::array::from_mut(&mut self.w))
    }
}

#[cfg(test)]
mod test
{
    use std::f64::consts::TAU;

    use super::WahFilter;

    #[test]
    fn plot()
    {
        let mut filter = WahFilter::new(0.1);
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}