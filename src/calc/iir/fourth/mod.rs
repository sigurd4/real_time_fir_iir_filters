use num::Float;

moddef::moddef!(
    flat(pub) mod {
        wah for cfg(feature = "wah")
    }
);

pub fn bilinear4_0_1_2_3_4<F>(rate: F, c0: F, c1: F, c2: F, c3: F, c4: F) -> [F; 5]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = four_rate2*two_rate;
    let sixteen_rate4 = four_rate2*four_rate2;

    let v0 = c0 + four_rate2*c2 + sixteen_rate4*c4;
    let v0s = two_rate*c1 + eight_rate3*c3;

    let v1_quarter = c0 - sixteen_rate4*c4;
    let v1_half = v1_quarter + v1_quarter;
    let v1 = v1_half + v1_half;

    let v1s_half = two_rate*c1 - eight_rate3*c3;
    let v1s = v1s_half + v1s_half;

    let v3p_sixth = c0 + sixteen_rate4*c4;
    let v3_half = v3p_sixth + v3p_sixth + v3p_sixth - four_rate2*c2;
    let v3 = v3_half + v3_half;
    [
        v0 + v0s,
        v1 + v1s,
        v3,
        v1 - v1s,
        v0 - v0s
    ]
}