use num::Float;

pub struct RLCVal<F>
where
    F: Float
{
    pub r: F,
    pub l: F,
    pub c: F
}