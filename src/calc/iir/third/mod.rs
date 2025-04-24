use num::Float;

moddef::moddef!(
    flat(pub) mod {
        third_order_butterworth for cfg(feature = "third_order_butterworth"),
        third_order_sallen_key for cfg(feature = "third_order_sallen_key"),
        third_order for cfg(feature = "third_order")
    }
);

pub fn bilinear3_0<F>(c0: F) -> [F; 4]
where
    F: Float
{
    let three_c0 = c0 + c0 + c0;
    [
        c0,
        three_c0,
        three_c0,
        c0,
    ]
}
pub fn bilinear3_1<F>(rate: F, c1: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let two_rate_c1 = two_rate*c1;
    [
        two_rate_c1,
        two_rate_c1,
        -two_rate_c1,
        -two_rate_c1,
    ]
}
pub fn bilinear3_2<F>(rate: F, c2: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let four_rate2_c2 = four_rate2*c2;
    [
        four_rate2_c2,
        -four_rate2_c2,
        -four_rate2_c2,
        four_rate2_c2,
    ]
}
pub fn bilinear3_3<F>(rate: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = four_rate2*two_rate;
    let eight_rate3_c3 = eight_rate3*c3;
    let twenty_four_rate3_c3 = eight_rate3_c3 + eight_rate3_c3 + eight_rate3_c3;
    [
        eight_rate3_c3,
        -twenty_four_rate3_c3,
        twenty_four_rate3_c3,
        -eight_rate3_c3,
    ]
}

pub fn bilinear3_0_1<F>(rate: F, c0: F, c1: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;

    let s = two_rate*c1;

    let v1 = c0 + c0 + c0;
    [
        c0 + s,
        v1 + s,
        v1 - s,
        c0 - s
    ]
}
pub fn bilinear3_0_2<F>(rate: F, c0: F, c2: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let v = four_rate2*c2;

    let v0 = c0 + v;

    let v1 = (c0 + c0 + c0) - v;
    [
        v0,
        v1,
        v1,
        v0
    ]
}
pub fn bilinear3_0_3<F>(rate: F, c0: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let b = eight_rate3*c3;

    let v0 = c0;
    let v0s = b;

    let v1 = c0 + c0 + c0;
    let v1s = -(b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}
pub fn bilinear3_1_3<F>(rate: F, c1: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let s = two_rate*c1;
    let b = eight_rate3*c3;

    let v0s = s + b;
    let v1s = s - (b + b + b);
    [
        v0s,
        v1s,
        -v1s,
        -v0s
    ]
}
pub fn bilinear3_1_2<F>(rate: F, c1: F, c2: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let v = four_rate2*c2;
    let s = two_rate*c1;

    let v0 = v;
    let v1 = -v;
    [
        v0 + s,
        v1 + s,
        v1 - s,
        v0 - s
    ]
}
pub fn bilinear3_2_3<F>(rate: F, c2: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let v = four_rate2*c2;
    let b = eight_rate3*c3;

    let v0 = v;
    let v0s = b;

    let v1 = -v;
    let v1s = -(b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}

pub fn bilinear3_0_1_2<F>(rate: F, c0: F, c1: F, c2: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;

    let v = four_rate2*c2;
    let s = two_rate*c1;

    let v0 = c0 + v;
    let v0s = s;

    let v1 = (c0 + c0 + c0) - v;
    let v1s = s;
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}
pub fn bilinear3_0_1_3<F>(rate: F, c0: F, c1: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let s = two_rate*c1;
    let b = eight_rate3*c3;

    let v0 = c0;
    let v0s = s + b;

    let v1 = c0 + c0 + c0;
    let v1s = s - (b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}
pub fn bilinear3_0_2_3<F>(rate: F, c0: F, c2: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let v = four_rate2*c2;
    let b = eight_rate3*c3;

    let v0 = c0 + v;
    let v0s = b;

    let v1 = (c0 + c0 + c0) - v;
    let v1s = -(b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}
pub fn bilinear3_1_2_3<F>(rate: F, c1: F, c2: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = four_rate2*two_rate;

    let v = four_rate2*c2;
    let s = two_rate*c1;
    let b = eight_rate3*c3;

    let v0 = v;
    let v0s = s + b;

    let v1 = -v;
    let v1s = s - (b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}

pub fn bilinear3_0_1_2_3<F>(rate: F, c0: F, c1: F, c2: F, c3: F) -> [F; 4]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = two_rate*four_rate2;

    let v = four_rate2*c2;
    let s = two_rate*c1;
    let b = eight_rate3*c3;

    let v0 = c0 + v;
    let v0s = s + b;

    let v1 = (c0 + c0 + c0) - v;
    let v1s = s - (b + b + b);
    [
        v0 + v0s,
        v1 + v1s,
        v1 - v1s,
        v0 - v0s
    ]
}