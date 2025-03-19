use num::Float;

pub struct RC2GVal<F>
where
    F: Float
{
    pub r1: F,
    pub c1: F,
    pub r2: F,
    pub c2: F,
    pub g: F
}