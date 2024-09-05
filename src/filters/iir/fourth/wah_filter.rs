use crate::{f, param::WahFilterParam, params::CrybabyGCB95, real_time_fir_iir_filters};

// TODO: make it SOS
crate::def_rtf!(
    WahFilter
    {
        type Param: WahFilterParam = CrybabyGCB95;

        const OUTPUTS: usize = 1;
        const O_BUFFERS: usize = 1;
        const SOS_BUFFERS: usize = 1;
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
            
            let two_rate = rate + rate;
            let four_rate = two_rate + two_rate;
            let four_rate2 = two_rate*two_rate;
            let eight_rate3 = four_rate2*two_rate;
            
            let x = param.position();

            let one_m_k_recip_plus_r_pot_per_r_j = (P::F::one() - x).recip() + f!(P::R_POT/P::R_J);

            let g_q2 = one_m_k_recip_plus_r_pot_per_r_j/(x.recip() + one_m_k_recip_plus_r_pot_per_r_j);

            let a3 = f!(a3a) + f!(a3b)*g_q2;
            let a2 = f!(a2a) + f!(a2b)*g_q2;
            let mg_q1 = f!(-g_q1);

            let b2 = f!(b2);
            let b3 = f!(b3);
            let b1 = f!(b1);

            let eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1 = (b3*four_rate2 + b1)*two_rate*mg_q1;
            let four_b2_rate2_mg_q1 = b2*four_rate2*mg_q1;
            let sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1 = (b3*four_rate2 - b2)*four_rate*mg_q1;

            let eight_a3_rate3 = eight_rate3*a3;
            let sixteen_a3_rate3 = eight_a3_rate3 + eight_a3_rate3;
            let two_a1_rate = f!(a1)*two_rate;
            let four_a1_rate = two_a1_rate + two_a1_rate;
            let four_a2_rate2 = four_rate2*a2;
            let eight_a2_rate2 = four_a2_rate2 + four_a2_rate2;
            let a0 = f!(a0);
            let two_a0 = a0 + a0;
            let four_a0 = two_a0 + two_a0;
            let six_a0 = four_a0 + two_a0;

            let four_a2_rate2_p_a0 = four_a2_rate2 + a0;
            let two_a1_rate_p_eight_a3_rate3 = two_a1_rate + eight_a3_rate3;
            let sixteen_a3_rate3_m_four_a1_rate = sixteen_a3_rate3 - four_a1_rate;
            (
                ([], [], [[
                    four_b2_rate2_mg_q1 + eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1,
                    -sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1,
                    -four_b2_rate2_mg_q1 - four_b2_rate2_mg_q1,
                    sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1,
                    four_b2_rate2_mg_q1 - eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1
                ]]),
                [([], [[
                    four_a2_rate2_p_a0 + two_a1_rate_p_eight_a3_rate3,
                    four_a0 - sixteen_a3_rate3_m_four_a1_rate,
                    six_a0 - eight_a2_rate2,
                    four_a0 + sixteen_a3_rate3_m_four_a1_rate,
                    four_a2_rate2_p_a0 - two_a1_rate_p_eight_a3_rate3
                ]])]
            )
        }
    }
);

#[cfg(test)]
mod test
{
    use super::{WahFilter, CrybabyGCB95};

    #[test]
    fn plot()
    {
        let mut filter = WahFilter::new(CrybabyGCB95::new(0.1));
        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}