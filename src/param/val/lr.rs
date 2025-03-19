use num::Float;

pub struct LRVal<F>
where
    F: Float
{
    pub l: F,
    pub r: F
}