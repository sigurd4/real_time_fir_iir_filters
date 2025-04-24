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
    rate: F
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

        Self {
            r1,
            c1,
            r2,
            c2,
            r3,
            c3,
            g,
            one_m_g,
            rate
        }
    }

    pub fn b_low_low(&self) -> [F; 4]
    {
        let p0 = self.g*self.r2;
        super::bilinear3_0(p0)
    }
    pub fn b_high_low(&self) -> [F; 4]
    {
        let p1 = self.g*self.r1*self.c1*self.r2;
        super::bilinear3_1(self.rate, p1)
    }
    pub fn b_low_band1(&self) -> [F; 4]
    {
        let p1 = self.g*self.r2*self.c2;
        super::bilinear3_1(self.rate, p1)
    }
    pub fn b_high_band1(&self) -> [F; 4]
    {
        let p2 = self.g*self.r1*self.c1*self.r2*self.c2;
        super::bilinear3_2(self.rate, p2)
    }
    pub fn b_low_band2(&self) -> [F; 4]
    {
        let p1 = self.g*self.r2*self.r3*self.c3;
        super::bilinear3_1(self.rate, p1)
    }
    pub fn b_high_band2(&self) -> [F; 4]
    {
        let p2 = self.g*self.r1*self.c1*self.r2*self.r3*self.c3;
        super::bilinear3_2(self.rate, p2)
    }
    pub fn b_low_high(&self) -> [F; 4]
    {
        let p2 = self.g*self.r2*self.c2*self.r3*self.c3;
        super::bilinear3_2(self.rate, p2)
    }
    pub fn b_high_high(&self) -> [F; 4]
    {
        let p3 = self.g*self.r1*self.c1*self.r2*self.c2*self.r3*self.c3;
        super::bilinear3_3(self.rate, p3)
    }
    pub fn a_low(&self) -> [F; 4]
    {
        let two_r1 = self.r1 + self.r1;

        let p0 = self.r2 + two_r1;
        let p1 = self.r2*(self.c1*self.r1 + self.c2*self.one_m_g*(self.r2 + self.r1) + self.c3*(self.r3 + self.r2 + self.r1)) + two_r1*self.c3*self.r3;
        let p2 = self.r2*(self.c1*self.r1*(self.c2*self.r2*self.one_m_g + self.c3*(self.r3 + self.r2)) + self.c2*self.c3*self.r3*(self.r2 + self.r1));
        let p3 = self.c1*self.c2*self.c3*self.r1*self.r2*self.r2*self.r3;
        
        super::bilinear3_0_1_2_3(self.rate, p0, p1, p2, p3)
    }
    pub fn a_band1(&self) -> [F; 4]
    {
        let two_c2 = self.c2 + self.c2;

        let p0 = self.one_m_g;
        let p1 = self.r1*self.one_m_g*(self.c1 + self.c2) + self.c3*(self.r2 + self.r3) + self.c2*self.r2;
        let p2 = self.c1*self.r1*(self.c3*self.r2 + self.c3*self.r3 + self.c2*self.r2) + self.c2*(self.c3*(self.r2*self.r3 + self.r1*self.r2 + self.r1*self.r3) + two_c2*self.r1*self.r2);
        let p3 = self.c2*self.c3*self.r1*self.r2*self.r3*(self.c1 + two_c2);

        super::bilinear3_0_1_2_3(self.rate, p0, p1, p2, p3)
    }
    pub fn a_band2(&self) -> [F; 4]
    {
        let two_r1 = self.r1 + self.r1;
    
        let p0 = two_r1 + self.r2;
        let p1 = self.r2*(self.c1*self.r1 + self.c2*(self.r1 + self.r2) + self.c3*(self.r1 + self.r3 + self.r2)) + two_r1*self.c3*self.r3;
        let p2 = self.r2*(self.c1*self.r1*(self.c2*self.r2 + self.c3*(self.r3 + self.r2)) + self.c2*self.c3*self.r3*self.one_m_g*(self.r1 + self.r2));
        let p3 = self.c1*self.c2*self.c3*self.r1*self.r2*self.r2*self.r3*self.one_m_g;

        super::bilinear3_0_1_2_3(self.rate, p0, p1, p2, p3)
    }
    pub fn a_high(&self) -> [F; 4]
    {
        let two_c2 = self.c2 + self.c2;
        
        let p0 = F::one();
        let p1 = self.c1*self.r1 + self.c3*(self.r2 + self.r3*self.one_m_g) + self.c2*(self.r2 + self.r1);
        let p2 = self.c1*self.r1*(self.c3*(self.r2 + self.r3*self.one_m_g) + self.c2*self.r2) + self.c2*(self.c3*(self.r2*self.r3 + self.r1*(self.r2 + self.r3*self.one_m_g)) + two_c2*self.r1*self.r2);
        let p3 = self.c2*self.c3*self.r1*self.r2*self.r3*(self.c1 + two_c2);

        super::bilinear3_0_1_2_3(self.rate, p0, p1, p2, p3)
    }
}