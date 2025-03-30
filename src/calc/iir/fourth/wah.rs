use crate::{f, param::{FilterFloat, Param, WahFilterParam, X}};

struct WahConstCalc<F>
where
    F: FilterFloat
{
    pub a0: F,
    pub a1: F,
    pub b1: F,
    pub b2: F,
    pub b3: F,
    pub a2a: F,
    pub a2b: F,
    pub a3a: F,
    pub a3b: F,
    pub mg_q1: F
}
impl WahConstCalc<f64>
{
    const fn new64<P>() -> WahConstCalc<f64>
    where
        Param<P>: WahFilterParam
    {
        let r_s = Param::<P>::R_S;
        let beta = Param::<P>::BETA;
        let r_e1 = Param::<P>::R_E1;
        let r_g = Param::<P>::R_G;
        let r_j = Param::<P>::R_J;
        let r_c1 = Param::<P>::R_C1;
        let r_e2 = Param::<P>::R_E2;
        let v_cc = Param::<P>::V_CC;
        let v_f = Param::<P>::V_F;
        let v_t = Param::<P>::V_T;
        let c_i = Param::<P>::C_I;
        let l = Param::<P>::L;
        let r_i = Param::<P>::R_I;
        let r_p = Param::<P>::R_P;
        let c_f = Param::<P>::C_F;

        let s_g = 1.0/(r_s + beta*r_e1) + 1.0/r_g + 1.0/r_j;
        let s_c1 = 1.0/r_c1 + 2.0/r_j + beta/(r_s + beta*r_e1)/r_j/s_g - 1.0/r_j/r_j/s_g;
        let s_b2 = 1.0/r_j + 1.0/beta/r_e2 - 1.0/r_j/r_j/s_c1;

        let v_b2 = (v_cc/r_c1 + (1.0/(r_s + beta*r_e1)/r_j/s_g - beta*(1.0 - (r_s + beta*r_e1)*s_g)/(r_s + beta*r_e1)/(r_s + beta*r_e1)/s_g)*v_f)/r_j/s_b2/s_c1 + v_f/r_e2/s_b2;
        let v_c1 = v_cc/r_c1/s_c1 + v_b2/r_j/s_c1 + (1.0/(r_s + beta*r_e1)/r_j/s_g - beta*(1.0 - (r_s + beta*r_e1)*s_g)/(r_s + beta*r_e1)/(r_s + beta*r_e1)/s_g)/s_c1*v_f;
        let v_g = v_f/(r_s + beta*r_e1)/s_g + v_c1/r_j/s_g;

        let i_c1 = beta*(v_g - v_f)/(r_s + beta*r_e1);
        let i_c2 = (v_b2 - v_f)/r_e2;

        let _r_e1 = v_t/i_c1;
        let _r_e2 = v_t/i_c2;

        let g_q1 = r_c1/(_r_e1 + r_e1);

        let s_q1 = 1.0/beta/(_r_e1 + r_e1);

        let a1 = r_s*c_i/l*(1.0 + r_i*s_q1) + s_q1*(1.0 + r_s/r_p) + r_i*c_i/l + 1.0/r_p;
        let a0 = (r_s*s_q1 + 1.0)/l;

        let b3 = r_s*c_i*c_f;
        let b2 = c_i*(1.0 + r_s/r_p);
        let b1 = r_s*c_i/l;

        let a3a = c_i*c_f*(r_s*(1.0 + r_s*s_q1) + r_i);
        let a3b = c_i*c_f*r_i*g_q1;

        let a2a = (c_i*(1.0 + r_s/r_p) + c_f)*(1.0 + r_s*s_q1) + c_i*r_i/r_p;
        let a2b = c_f*g_q1;

        let mg_q1 = -g_q1;

        Self {a0, a1, b1, b2, b3, a2a, a2b, a3a, a3b, mg_q1}
    }
}
impl<F> WahConstCalc<F>
where
    F: FilterFloat
{
    pub fn new<P>() -> Self
    where
        Param<P>: WahFilterParam<F = F>
    {
        let WahConstCalc {a0, a1, b1, b2, b3, a2a, a2b, a3a, a3b, mg_q1} = WahConstCalc::new64();
        Self {
            a0: f!(a0),
            a1: f!(a1),
            b1: f!(b1),
            b2: f!(b2),
            b3: f!(b3),
            a2a: f!(a2a),
            a2b: f!(a2b),
            a3a: f!(a3a),
            a3b: f!(a3b),
            mg_q1: f!(mg_q1)
        }
    }
}

pub struct WahCalc<F>
where
    F: FilterFloat
{
    four_b2_rate2_mg_q1: F,
    eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1: F,
    sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1: F,
    four_a2_rate2_p_a0: F,
    two_a1_rate_p_eight_a3_rate3: F,
    four_a0: F,
    sixteen_a3_rate3_m_four_a1_rate: F,
    six_a0: F,
    eight_a2_rate2: F
}
impl<F> WahCalc<F>
where
    F: FilterFloat
{
    pub fn new<P>(param: &Param<P>, rate: F) -> Self
    where
        Param<P>: WahFilterParam<F = F>
    {
        let WahConstCalc {a0, a1, b1, b2, b3, a2a, a2b, a3a, a3b, mg_q1} = WahConstCalc::new::<P>();
        let X {x} = param.x();

        let two_rate = rate + rate;
        let four_rate = two_rate + two_rate;
        let four_rate2 = two_rate*two_rate;
        let eight_rate3 = four_rate2*two_rate;
        
        let one_m_k_recip_plus_r_pot_per_r_j = (F::one() - x).recip() + f!(Param::<P>::R_POT/Param::<P>::R_J);

        let g_q2 = one_m_k_recip_plus_r_pot_per_r_j/(x.recip() + one_m_k_recip_plus_r_pot_per_r_j);

        let a3 = a3a + a3b*g_q2;
        let a2 = a2a + a2b*g_q2;

        let eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1 = (b3*four_rate2 + b1)*two_rate*mg_q1;
        let four_b2_rate2_mg_q1 = b2*four_rate2*mg_q1;
        let sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1 = (b3*four_rate2 - b2)*four_rate*mg_q1;

        let eight_a3_rate3 = eight_rate3*a3;
        let sixteen_a3_rate3 = eight_a3_rate3 + eight_a3_rate3;
        let two_a1_rate = a1*two_rate;
        let four_a1_rate = two_a1_rate + two_a1_rate;
        let four_a2_rate2 = four_rate2*a2;
        let eight_a2_rate2 = four_a2_rate2 + four_a2_rate2;
        let two_a0 = a0 + a0;
        let four_a0 = two_a0 + two_a0;
        let six_a0 = four_a0 + two_a0;

        let four_a2_rate2_p_a0 = four_a2_rate2 + a0;
        let two_a1_rate_p_eight_a3_rate3 = two_a1_rate + eight_a3_rate3;
        let sixteen_a3_rate3_m_four_a1_rate = sixteen_a3_rate3 - four_a1_rate;

        Self {
            four_b2_rate2_mg_q1,
            eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1,
            sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1,
            four_a2_rate2_p_a0,
            two_a1_rate_p_eight_a3_rate3,
            four_a0,
            sixteen_a3_rate3_m_four_a1_rate,
            six_a0,
            eight_a2_rate2
        }
    }

    pub fn b(&self) -> [F; 5]
    {
        [
            self.four_b2_rate2_mg_q1 + self.eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1,
            -self.sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1,
            -self.four_b2_rate2_mg_q1 - self.four_b2_rate2_mg_q1,
            self.sixteen_b3_rate3_mg_q1_m_four_b2_rate_mg_q1,
            self.four_b2_rate2_mg_q1 - self.eight_b3_rate3_mg_q1_p_two_b1_rate_mg_q1
        ]
    }

    pub fn a(&self) -> [F; 5]
    {
        [
            self.four_a2_rate2_p_a0 + self.two_a1_rate_p_eight_a3_rate3,
            self.four_a0 - self.sixteen_a3_rate3_m_four_a1_rate,
            self.six_a0 - self.eight_a2_rate2,
            self.four_a0 + self.sixteen_a3_rate3_m_four_a1_rate,
            self.four_a2_rate2_p_a0 - self.two_a1_rate_p_eight_a3_rate3
        ]
    }
}