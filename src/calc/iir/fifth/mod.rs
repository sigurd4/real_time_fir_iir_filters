use num::Float;

pub fn bilinear5_1_2_3_4_5<F>(rate: F, c1: F, c2: F, c3: F, c4: F, c5: F) -> [F; 6]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = four_rate2*two_rate;
    let sixteen_rate4 = four_rate2*four_rate2;
    let thirty_two_rate5 = eight_rate3*four_rate2;

    let two_c1_rate = c1*two_rate;
    let four_c2_rate2 = c2*four_rate2;
    let eight_c3_rate3 = c3*eight_rate3;
    let sixteen_c4_rate4 = c4*sixteen_rate4;
    let thirty_two_c5_rate5 = c5*thirty_two_rate5;

    let hundred_and_sixty_c5_rate5 = thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5;

    let v0 = four_c2_rate2 + sixteen_c4_rate4;
    let v0s = two_c1_rate + eight_c3_rate3 + thirty_two_c5_rate5;

    let v1 = four_c2_rate2 - (sixteen_c4_rate4 + sixteen_c4_rate4 + sixteen_c4_rate4);
    let v1s = (two_c1_rate + two_c1_rate + two_c1_rate) - eight_c3_rate3 - hundred_and_sixty_c5_rate5;

    let v2_half = sixteen_c4_rate4 - four_c2_rate2;
    let v2 = v2_half + v2_half;

    let v2s_half = two_c1_rate - eight_c3_rate3 + hundred_and_sixty_c5_rate5;
    let v2s = v2s_half + v2s_half;
    [
        v0 + v0s,
        v1 + v1s,
        v2 + v2s,
        v2 - v2s,
        v1 - v1s,
        v0 - v0s,
    ]
}

pub fn bilinear5_0_1_2_3_4_5<F>(rate: F, c0: F, c1: F, c2: F, c3: F, c4: F, c5: F) -> [F; 6]
where
    F: Float
{
    let two_rate = rate + rate;
    let four_rate2 = two_rate*two_rate;
    let eight_rate3 = four_rate2*two_rate;
    let sixteen_rate4 = four_rate2*four_rate2;
    let thirty_two_rate5 = eight_rate3*four_rate2;

    let two_c1_rate = c1*two_rate;
    let four_c2_rate2 = c2*four_rate2;
    let eight_c3_rate3 = c3*eight_rate3;
    let sixteen_c4_rate4 = c4*sixteen_rate4;
    let thirty_two_c5_rate5 = c5*thirty_two_rate5;

    let five_c0 = c0 + c0 + c0 + c0 + c0;
    let hundred_and_sixty_c5_rate5 = thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5 + thirty_two_c5_rate5;

    let v0 = c0 + four_c2_rate2 + sixteen_c4_rate4;
    let v0s = two_c1_rate + eight_c3_rate3 + thirty_two_c5_rate5;

    let v1 = five_c0 + four_c2_rate2 - (sixteen_c4_rate4 + sixteen_c4_rate4 + sixteen_c4_rate4);
    let v1s = (two_c1_rate + two_c1_rate + two_c1_rate) - eight_c3_rate3 - hundred_and_sixty_c5_rate5;

    let v2_half = five_c0 - four_c2_rate2 + sixteen_c4_rate4;
    let v2 = v2_half + v2_half;
    let v2s_half = two_c1_rate - eight_c3_rate3 + hundred_and_sixty_c5_rate5;
    let v2s = v2s_half + v2s_half;
    [
        v0 + v0s,
        v1 + v1s,
        v2 + v2s,
        v2 - v2s,
        v1 - v1s,
        v0 - v0s,
    ]
}