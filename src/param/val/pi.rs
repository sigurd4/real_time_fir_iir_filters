use num::Float;

pub struct PIVal<F>
where
    F: Float
{
    pub p: F,
    pub i: F
}