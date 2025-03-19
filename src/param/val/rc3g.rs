use num::Float;

pub struct RC3GVal<F>
where
    F: Float
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F,
    pub r3: F,
    pub c3: F,
    pub g: F
}