use num::Float;

moddef::moddef!(
    flat(pub) mod {
        third_order_butterworth for cfg(feature = "third_order_butterworth"),
        third_order_chebyshev1 for cfg(feature = "third_order_chebyshev1"),
        third_order_sallen_key for cfg(feature = "third_order_sallen_key"),
        third_order for cfg(feature = "third_order")
    }
);

pub fn billinear4_0<F>(p0: F, three_p0: F) -> [F; 4]
where
    F: Float
{
    [
        p0,
        three_p0,
        three_p0,
        p0,
    ]
}
pub fn billinear4_1<F>(two_p1_rate: F) -> [F; 4]
where
    F: Float
{
    [
        two_p1_rate,
        two_p1_rate,
        -two_p1_rate,
        -two_p1_rate,
    ]
}
pub fn billinear4_2<F>(four_p2_rate2: F) -> [F; 4]
where
    F: Float
{
    [
        four_p2_rate2,
        -four_p2_rate2,
        -four_p2_rate2,
        four_p2_rate2,
    ]
}
pub fn billinear4_3<F>(eight_p3_rate3: F, twenty_four_p3_rate3: F) -> [F; 4]
where
    F: Float
{
    [
        eight_p3_rate3,
        -twenty_four_p3_rate3,
        twenty_four_p3_rate3,
        -eight_p3_rate3,
    ]
}
pub fn billinear4_0_1_2_3<F>(
    p0: F,
    three_p0: F,
    two_p1_rate: F,
    four_p2_rate2: F,
    eight_p3_rate3: F,
    twenty_four_p3_rate3: F
) -> [F; 4]
where
    F: Float
{
    let p0_p_four_p2_rate2 = p0 + four_p2_rate2;
    let three_p0_m_four_p2_rate2 = three_p0 - four_p2_rate2;

    let two_p1_rate_p_eight_p3_rate3 = two_p1_rate + eight_p3_rate3;
    let two_p1_rate_m_twenty_four_p3_rate3 = two_p1_rate - twenty_four_p3_rate3;
    [
        p0_p_four_p2_rate2 + two_p1_rate_p_eight_p3_rate3,
        three_p0_m_four_p2_rate2 + two_p1_rate_m_twenty_four_p3_rate3,
        three_p0_m_four_p2_rate2 - two_p1_rate_m_twenty_four_p3_rate3,
        p0_p_four_p2_rate2 - two_p1_rate_p_eight_p3_rate3,
    ]
}