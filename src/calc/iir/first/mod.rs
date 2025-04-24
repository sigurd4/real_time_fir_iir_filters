use num::Float;

moddef::moddef!(
    flat(pub) mod {
        first_order_all_pass for cfg(feature = "first_order_all_pass"),
        first_order_lr for cfg(feature = "first_order_lr"),
        first_order_rc for cfg(feature = "first_order_rc"),
        first_order for cfg(feature = "first_order"),
        pi for cfg(feature = "pi")
,    }
);

pub fn bilinear1_0<F>(c0: F) -> [F; 2]
where
    F: Float
{
    [
        c0,
        c0
    ]
}
pub fn bilinear1_1<F>(rate: F, c1: F) -> [F; 2]
where
    F: Float
{
    let two_rate = rate + rate;
    let two_rate_c1 = two_rate*c1;
    [
        two_rate_c1,
        -two_rate_c1
    ]
}
pub fn bilinear1_0_1<F>(rate: F, c0: F, c1: F) -> [F; 2]
where
    F: Float
{
    let two_rate = rate + rate;
    let two_rate_c1 = two_rate*c1;
    [
        c0 + two_rate_c1,
        c0 - two_rate_c1
    ]
}