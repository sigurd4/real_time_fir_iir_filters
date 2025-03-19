use num::Float;

pub struct PIDVal<F>
where
    F: Float
{
    pub p: F,
    pub i: F,
    pub d: F
}