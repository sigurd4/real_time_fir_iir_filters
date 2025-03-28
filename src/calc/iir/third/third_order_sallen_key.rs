use crate::param::{FilterFloat, RC3GSallenKey};

pub struct ThirdOrderSallenKeyCalc<F>
where
    F: FilterFloat
{
    r1: F,
    c1: F,
    r2: F,
    c2: F,
    r3: F,
    c3: F,
    g: F,
    one_m_g: F,
    three: F,
    two_rate: F,
    four_rate2: F,
    eight_rate3: F
}
impl<F> ThirdOrderSallenKeyCalc<F>
where
    F: FilterFloat
{
    pub fn new(rc3g: RC3GSallenKey<F>, rate: F) -> Self
    {
        let RC3GSallenKey {r1, c1, r2, c2, r3, c3, g} = rc3g;

        let one = F::one();
        let one_m_g = one - g;
        let three = one + one + one;

        let two_rate = rate + rate;
        let four_rate2 = two_rate*two_rate;
        let eight_rate3 = four_rate2*two_rate;

        Self {
            r1,
            c1,
            r2,
            c2,
            r3,
            c3,
            g,
            one_m_g,
            three,
            two_rate,
            four_rate2,
            eight_rate3
        }
    }

    pub fn b_low_low(&self) -> [F; 4]
    {
        let p0 = self.g*self.r2;
        let three_p0 = p0*self.three;
        super::billinear4_0(p0, three_p0)
    }
    pub fn b_high_low(&self) -> [F; 4]
    {
        let two_p1_rate = self.g*self.r1*self.c1*self.r2*self.two_rate;
        super::billinear4_1(two_p1_rate)
    }
    pub fn b_low_band1(&self) -> [F; 4]
    {
        let two_p1_rate = self.g*self.r2*self.c2*self.two_rate;
        super::billinear4_1(two_p1_rate)
    }
    pub fn b_high_band1(&self) -> [F; 4]
    {
        let four_p2_rate2 = self.g*self.r1*self.c1*self.r2*self.c2*self.four_rate2;
        super::billinear4_2(four_p2_rate2)
    }
    pub fn b_low_band2(&self) -> [F; 4]
    {
        let two_p1_rate = self.g*self.r2*self.r3*self.c3*self.two_rate;
        super::billinear4_1(two_p1_rate)
    }
    pub fn b_high_band2(&self) -> [F; 4]
    {
        let four_p2_rate2 = self.g*self.r1*self.c1*self.r2*self.r3*self.c3*self.four_rate2;
        super::billinear4_2(four_p2_rate2)
    }
    pub fn b_low_high(&self) -> [F; 4]
    {
        let four_p2_rate2 = self.g*self.r2*self.c2*self.r3*self.c3*self.four_rate2;
        super::billinear4_2(four_p2_rate2)
    }
    pub fn b_high_high(&self) -> [F; 4]
    {
        let eight_p3_rate3 = self.g*self.r1*self.c1*self.r2*self.c2*self.r3*self.c3*self.eight_rate3;
        let twenty_four_p3_rate3 = eight_p3_rate3*self.three;
        super::billinear4_3(eight_p3_rate3, twenty_four_p3_rate3)
    }
    pub fn a_low(&self) -> [F; 4]
    {
        let two_r1 = self.r1 + self.r1;

        let p0 = self.r2 + two_r1;
        let p1 = self.r2*(self.c1*self.r1 + self.c2*self.one_m_g*(self.r2 + self.r1) + self.c3*(self.r3 + self.r2 + self.r1)) + two_r1*self.c3*self.r3;
        let p2 = self.r2*(self.c1*self.r1*(self.c2*self.r2*self.one_m_g + self.c3*(self.r3 + self.r2)) + self.c2*self.c3*self.r3*(self.r2 + self.r1));
        let p3 = self.c1*self.c2*self.c3*self.r1*self.r2*self.r2*self.r3;

        let three_p0 = p0*self.three;
        let two_p1_rate = p1*self.two_rate;
        let four_p2_rate2 = p2*self.four_rate2;
        let eight_p3_rate3 = p3*self.eight_rate3;
        let twenty_four_p3_rate3 = eight_p3_rate3*self.three;
        super::billinear4_0_1_2_3(p0, three_p0, two_p1_rate, four_p2_rate2, eight_p3_rate3, twenty_four_p3_rate3)
    }
    pub fn a_band1(&self) -> [F; 4]
    {
        let two_c2 = self.c2 + self.c2;

        let p0 = self.one_m_g;
        let p1 = self.r1*self.one_m_g*(self.c1 + self.c2) + self.c3*(self.r2 + self.r3) + self.c2*self.r2;
        let p2 = self.c1*self.r1*(self.c3*self.r2 + self.c3*self.r3 + self.c2*self.r2) + self.c2*(self.c3*(self.r2*self.r3 + self.r1*self.r2 + self.r1*self.r3) + two_c2*self.r1*self.r2);
        let p3 = self.c2*self.c3*self.r1*self.r2*self.r3*(self.c1 + two_c2);

        let three_p0 = p0*self.three;
        let two_p1_rate = p1*self.two_rate;
        let four_p2_rate2 = p2*self.four_rate2;
        let eight_p3_rate3 = p3*self.eight_rate3;
        let twenty_four_p3_rate3 = eight_p3_rate3*self.three;
        super::billinear4_0_1_2_3(p0, three_p0, two_p1_rate, four_p2_rate2, eight_p3_rate3, twenty_four_p3_rate3)
    }
    pub fn a_band2(&self) -> [F; 4]
    {
        let two_r1 = self.r1 + self.r1;
    
        let p0 = two_r1 + self.r2;
        let p1 = self.r2*(self.c1*self.r1 + self.c2*(self.r1 + self.r2) + self.c3*(self.r1 + self.r3 + self.r2)) + two_r1*self.c3*self.r3;
        let p2 = self.r2*(self.c1*self.r1*(self.c2*self.r2 + self.c3*(self.r3 + self.r2)) + self.c2*self.c3*self.r3*self.one_m_g*(self.r1 + self.r2));
        let p3 = self.c1*self.c2*self.c3*self.r1*self.r2*self.r2*self.r3*self.one_m_g;

        let three_p0 = p0*self.three;
        let two_p1_rate = p1*self.two_rate;
        let four_p2_rate2 = p2*self.four_rate2;
        let eight_p3_rate3 = p3*self.eight_rate3;
        let twenty_four_p3_rate3 = eight_p3_rate3*self.three;
        super::billinear4_0_1_2_3(p0, three_p0, two_p1_rate, four_p2_rate2, eight_p3_rate3, twenty_four_p3_rate3)
    }
    pub fn a_high(&self) -> [F; 4]
    {
        let two_c2 = self.c2 + self.c2;
        
        let p0 = F::one();
        let p1 = self.c1*self.r1 + self.c3*(self.r2 + self.r3*self.one_m_g) + self.c2*(self.r2 + self.r1);
        let p2 = self.c1*self.r1*(self.c3*(self.r2 + self.r3*self.one_m_g) + self.c2*self.r2) + self.c2*(self.c3*(self.r2*self.r3 + self.r1*(self.r2 + self.r3*self.one_m_g)) + two_c2*self.r1*self.r2);
        let p3 = self.c2*self.c3*self.r1*self.r2*self.r3*(self.c1 + two_c2);

        let three_p0 = p0*self.three;
        let two_p1_rate = p1*self.two_rate;
        let four_p2_rate2 = p2*self.four_rate2;
        let eight_p3_rate3 = p3*self.eight_rate3;
        let twenty_four_p3_rate3 = eight_p3_rate3*self.three;
        super::billinear4_0_1_2_3(p0, three_p0, two_p1_rate, four_p2_rate2, eight_p3_rate3, twenty_four_p3_rate3)
    }
}