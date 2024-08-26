use bytemuck::Pod;
use num::Float;

use crate::{f, param::FilterParam};

pub trait WahFilterParam: FilterParam
{
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
    
    #[allow(unused)]
    const C_G: f64 = 0.0000047;
    const C_I: f64 = 0.00000001;
    const C_F: f64 = 0.00000001;
    
    const L: f64 = 0.5;

    fn x(&self) -> Self::F;
}
crate::def_param!(
    X<F> {
        x: F
    } where
        F: Float + Pod
);
impl<F> FilterParam for X<F>
where
    F: Float + Pod
{
    type F = F;
}
impl<F> WahFilterParam for X<F>
where
    F: Float + Pod
{
    fn x(&self) -> Self::F
    {
        *self.x
    }
}

// TODO: make it SOS
crate::def_rtf!(
    WahFilter
    {
        type Param: WahFilterParam = X;

        const OUTPUTS: usize = 1;
        const BUFFERED_OUTPUTS: bool = false;
        const SOS_STAGES: usize = 0;
        const ORDER: usize = 4;
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            const fn consts<P>() -> [f64; 10]
            where
                P: WahFilterParam
            {
                let s_g = 1.0/(P::R_S + P::BETA*P::R_E1) + 1.0/P::R_G + 1.0/P::R_J;
                let s_c1 = 1.0/P::R_C1 + 2.0/P::R_J + P::BETA/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - 1.0/P::R_J/P::R_J/s_g;
                let s_b2 = 1.0/P::R_J + 1.0/P::BETA/P::R_E2 - 1.0/P::R_J/P::R_J/s_c1;

                let v_b2 = (P::V_CC/P::R_C1 + (1.0/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - P::BETA*(1.0 - (P::R_S + P::BETA*P::R_E1)*s_g)/(P::R_S + P::BETA*P::R_E1)/(P::R_S + P::BETA*P::R_E1)/s_g)*P::V_F)/P::R_J/s_b2/s_c1 + P::V_F/P::R_E2/s_b2;
                let v_c1 = P::V_CC/P::R_C1/s_c1 + v_b2/P::R_J/s_c1 + (1.0/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - P::BETA*(1.0 - (P::R_S + P::BETA*P::R_E1)*s_g)/(P::R_S + P::BETA*P::R_E1)/(P::R_S + P::BETA*P::R_E1)/s_g)/s_c1*P::V_F;
                let v_g = P::V_F/(P::R_S + P::BETA*P::R_E1)/s_g + v_c1/P::R_J/s_g;

                let i_c1 = P::BETA*(v_g - P::V_F)/(P::R_S + P::BETA*P::R_E1);
                let i_c2 = (v_b2 - P::V_F)/P::R_E2;

                let _r_e1 = P::V_T/i_c1;
                let _r_e2 = P::V_T/i_c2;

                //let g_q1 = P::R_C1/(_r_e1 + P::R_E1);
                
                let s_g = 1.0/(P::R_S + P::BETA*P::R_E1) + 1.0/P::R_G + 1.0/P::R_J;
                let s_c1 = 1.0/P::R_C1 + 2.0/P::R_J + P::BETA/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - 1.0/P::R_J/P::R_J/s_g;
                let s_b2 = 1.0/P::R_J + 1.0/P::BETA/P::R_E2 - 1.0/P::R_J/P::R_J/s_c1;

                let v_b2 = (P::V_CC/P::R_C1 + (1.0/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - P::BETA*(1.0 - (P::R_S + P::BETA*P::R_E1)*s_g)/(P::R_S + P::BETA*P::R_E1)/(P::R_S + P::BETA*P::R_E1)/s_g)*P::V_F)/P::R_J/s_b2/s_c1 + P::V_F/P::R_E2/s_b2;
                let v_c1 = P::V_CC/P::R_C1/s_c1 + v_b2/P::R_J/s_c1 + (1.0/(P::R_S + P::BETA*P::R_E1)/P::R_J/s_g - P::BETA*(1.0 - (P::R_S + P::BETA*P::R_E1)*s_g)/(P::R_S + P::BETA*P::R_E1)/(P::R_S + P::BETA*P::R_E1)/s_g)/s_c1*P::V_F;
                let v_g = P::V_F/(P::R_S + P::BETA*P::R_E1)/s_g + v_c1/P::R_J/s_g;

                let i_c1 = P::BETA*(v_g - P::V_F)/(P::R_S + P::BETA*P::R_E1);
                let i_c2 = (v_b2 - P::V_F)/P::R_E2;

                let _r_e1 = P::V_T/i_c1;
                let _r_e2 = P::V_T/i_c2;

                let g_q1 = P::R_C1/(_r_e1 + P::R_E1);

                let s_q1 = 1.0/P::BETA/(_r_e1 + P::R_E1);

                let a1 = P::R_S*P::C_I/P::L*(1.0 + P::R_I*s_q1) + s_q1*(1.0 + P::R_S/P::R_P) + P::R_I*P::C_I/P::L + 1.0/P::R_P;
                let a0 = (P::R_S*s_q1 + 1.0)/P::L;

                let b3 = P::R_S*P::C_I*P::C_F;
                let b2 = P::C_I*(1.0 + P::R_S/P::R_P);
                let b1 = P::R_S*P::C_I/P::L;

                let a3a = P::C_I*P::C_F*(P::R_S*(1.0 + P::R_S*s_q1) + P::R_I);
                let a3b = P::C_I*P::C_F*P::R_I*g_q1;

                let a2a = (P::C_I*(1.0 + P::R_S/P::R_P) + P::C_F)*(1.0 + P::R_S*s_q1) + P::C_I*P::R_I/P::R_P;
                let a2b = P::C_F*g_q1;

                [a0, a1, b1, b2, b3, a2a, a2b, a3a, a3b, g_q1]
            }

            let [a0, a1, b1, b2, b3, a2a, a2b, a3a, a3b, g_q1] = consts::<P>();
            
            let rate2 = rate*rate;
            let rate3 = rate2*rate;
            
            let x = param.x();

            let one_m_k_recip_plus_r_pot_per_r_j = (f!(1.0) - x).recip() + f!(P::R_POT/P::R_J);

            let g_q2 = one_m_k_recip_plus_r_pot_per_r_j/(x.recip() + one_m_k_recip_plus_r_pot_per_r_j);

            let a3 = f!(a3a) + f!(a3b)*g_q2;
            let a2 = f!(a2a) + f!(a2b)*g_q2;
            let mg_q1 = f!(-g_q1);
            (
                ([], [[
                    (f!(8.0*b3)*rate3 + f!(4.0*b2)*rate2 + f!(2.0*b1)*rate)*mg_q1,
                    (f!(-16.0*b3)*rate3 + f!(4.0*b1)*rate)*mg_q1,
                    f!(-8.0*b2)*rate2*mg_q1,
                    (f!(16.0*b3)*rate3 - f!(4.0*b1)*rate)*mg_q1,
                    (f!(-8.0*b3)*rate3 + f!(4.0*b2)*rate2 - f!(2.0*b1)*rate)*mg_q1
                ]]),
                [([], [[
                    f!(8.0)*rate3*a3 + f!(4.0)*rate2*a2 + f!(2.0*a1)*rate + f!(a0),
                    f!(-16.0)*rate3*a3 + f!(4.0*a1)*rate + f!(4.0*a0),
                    f!(-8.0)*rate2*a2 + f!(6.0*a0),
                    f!(16.0)*rate3*a3 - f!(4.0*a1)*rate + f!(4.0*a0),
                    f!(-8.0)*rate3*a3 + f!(4.0)*rate2*a2 - f!(2.0*a1)*rate + f!(a0)
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{WahFilter, X};

    #[test]
    fn plot()
    {
        let mut filter = WahFilter::new(X::new(0.1));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}