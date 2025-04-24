use num::Float;

moddef::moddef!(
    flat(pub) mod {
        pid for cfg(feature = "pid"),
        second_order_chebyshev1 for cfg(feature = "second_order_chebyshev1"),
        second_order_chebyshev2 for cfg(feature = "second_order_chebyshev2"),
        second_order_elliptic for cfg(feature = "second_order_elliptic"),
        second_order_rc for cfg(feature = "second_order_rc"),
        second_order_rlc for cfg(feature = "second_order_rlc"),
        second_order_sallen_key for cfg(feature = "second_order_sallen_key"),
        second_order for cfg(any(
            feature = "second_order",
            feature = "second_order_butterworth"
        ))
    }
);

pub fn bilinear2_0<F>(c0: F) -> [F; 3]
where
    F: Float
{
    let v0s = c0;
    let v1s = v0s + v0s;
    [
        v0s,
        v1s,
        v0s
    ]
}
pub fn bilinear2_1<F>(rate: F, c1: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;

    let b = two_rate*c1;
    [
        b,
        F::zero(),
        -b
    ]
}
pub fn bilinear2_2<F>(rate: F, c2: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let s = four_rate2*c2;
    [
        s,
        -(s + s),
        s
    ]
}

pub fn bilinear2_0_1<F>(rate: F, c0: F, c1: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;

    let v0 = c0;
    let v1 = v0 + v0;

    let b = two_rate*c1;
    [
        v0 + b,
        v1,
        v0 - b
    ]
}
pub fn bilinear2_0_2<F>(rate: F, c0: F, c2: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let s = four_rate2*c2;

    let v0s = c0 + s;
    let v1s_half = c0 - s;
    let v1s = v1s_half + v1s_half;
    [
        v0s,
        v1s,
        v0s
    ]
}
pub fn bilinear2_1_2<F>(rate: F, c1: F, c2: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let s = four_rate2*c2;

    let v0s = s;
    let v1s_half = -s;
    let v1s = v1s_half + v1s_half;

    let b = two_rate*c1;
    [
        v0s + b,
        v1s,
        v0s - b
    ]
}

pub fn bilinear2_0_1_2<F>(rate: F, c0: F, c1: F, c2: F) -> [F; 3]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let s = four_rate2*c2;

    let v0s = c0 + s;
    let v1s_half = c0 - s;
    let v1s = v1s_half + v1s_half;

    let b = two_rate*c1;
    [
        v0s + b,
        v1s,
        v0s - b
    ]
}