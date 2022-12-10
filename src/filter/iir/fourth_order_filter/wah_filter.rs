use crate::iir::IIRFilter;

const R_I: f32 = 78000.0;
const R_S: f32 = 1500.0;
const R_P: f32 = 33000.0;
const R_C1: f32 = 22000.0;
const R_E1: f32 = 390.0;
const R_C2: f32 = 1000.0;
const R_E2: f32 = 10000.0;
const R_J: f32 = 470000.0;
const R_G: f32 = 82000.0;
const R_POT: f32 = 100000.0;

const V_CC: f32 = 9.0;
const V_F: f32 = 0.6;
const V_T: f32 = 0.026;

const BETA: f32 = 300.0; 

const S_G: f32 = 1.0/(R_S + BETA*R_E1) + 1.0/R_G + 1.0/R_J;
const S_C1: f32 = 1.0/R_C1 + 2.0/R_J + BETA/(R_S + BETA*R_E1)/R_J/S_G - 1.0/R_J/R_J/S_G;
const S_B2: f32 = 1.0/R_J + 1.0/BETA/R_E2 - 1.0/R_J/R_J/S_C1;

const V_B2: f32 = (V_CC/R_C1 + (1.0/(R_S + BETA*R_E1)/R_J/S_G - BETA*(1.0 - (R_S + BETA*R_E1)*S_G)/(R_S + BETA*R_E1)/(R_S + BETA*R_E1)/S_G)*V_F)/R_J/S_B2/S_C1 + V_F/R_E2/S_B2;
const V_C1: f32 = V_CC/R_C1/S_C1 + V_B2/R_J/S_C1 + (1.0/(R_S + BETA*R_E1)/R_J/S_G - BETA*(1.0 - (R_S + BETA*R_E1)*S_G)/(R_S + BETA*R_E1)/(R_S + BETA*R_E1)/S_G)/S_C1*V_F;
const V_G: f32 = V_F/(R_S + BETA*R_E1)/S_G + V_C1/R_J/S_G;

const I_C1: f32 = BETA*(V_G - V_F)/(R_S + BETA*R_E1);
const I_C2: f32 = (V_B2 - V_F)/R_E2;

const _R_E1: f32 = V_T/I_C1;
const _R_E2: f32 = V_T/I_C2;

const G_Q1: f32 = R_C1/(_R_E1 + R_E1);

const C_G: f32 = 0.0000047;
const C_I: f32 = 0.00000001;
const C_F: f32 = 0.00000001;

const L: f32 = 0.5;

#[derive(Copy, Clone)]
pub struct WahFilter
{
    pub k: f32,
    pub w: [f32; 4]
}

impl WahFilter
{
    pub fn new(k: f32) -> Self
    {
        Self {
            k,
            w: [0.0; 4]
        }
    }
}

const S_Q1: f32 = 1.0/BETA/(_R_E1 + R_E1);

const A1: f32 = R_S*C_I/L*(1.0 + R_I*S_Q1) + S_Q1*(1.0 + R_S/R_P) + R_I*C_I/L + 1.0/R_P;
const A0: f32 = (R_S*S_Q1 + 1.0)/L;

const B3: f32 = R_S*C_I*C_F;
const B2: f32 = C_I*(1.0 + R_S/R_P);
const B1: f32 = R_S*C_I/L;

impl IIRFilter<4, 1> for WahFilter
{
    fn a(&self, rate: f32) -> [f32; 5]
    {
        let g_q2 = (1.0/(1.0 - self.k) + R_POT/R_J)/(1.0/self.k + 1.0/(1.0 - self.k) + R_POT/R_J);

        let a3 = C_I*C_F*(R_S*(1.0 + R_S*S_Q1) + R_I*(1.0 + G_Q1*g_q2));
        let a2 = C_I*(1.0 + R_I*S_Q1)*(1.0 + R_S/R_P) + R_I*C_I/R_P + C_F*(1.0 + R_S*S_Q1 + G_Q1*g_q2);

        let rate2 = rate.powi(2);
        let rate3 = rate.powi(3);

        [
            8.0*rate3*a3 + 4.0*rate2*a2 + 2.0*rate*A1 + A0,
            -16.0*rate3*a3 + 4.0*rate*A1 + 4.0*A0,
            -8.0*rate2*a2 + 6.0*A0,
            16.0*rate3*a3 - 4.0*rate*A1 + 4.0*A0,
            - 8.0*rate3*a3 + 4.0*rate2*a2 - 2.0*rate*A1 + A0
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 5]; 1]
    {
        let rate2 = rate.powi(2);
        let rate3 = rate.powi(3);

        [
            [
                8.0*rate3*B3 + 4.0*rate2*B2 + 2.0*rate*B1,
                -16.0*rate3*B3 + 4.0*rate*B1,
                -8.0*rate2*B2,
                16.0*rate3*B3 - 4.0*rate*B1,
                -8.0*rate3*B3 + 4.0*rate2*B2 - 2.0*rate*B1
            ].map(|b| -b*G_Q1)
        ]
    }
    fn w(&mut self) -> &mut [f32; 4]
    {
        &mut self.w
    }
}