use num::Float;

pub struct OmegaEpsilonVal<F>
where
    F: Float
{
    pub omega: F,
    pub epsilon: F
}