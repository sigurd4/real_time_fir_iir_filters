use num::Float;

pub struct RCVal<F>
where
    F: Float
{
    pub r: F,
    pub c: F
}