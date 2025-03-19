use num::Float;

pub struct OmegaEpsilonXiVal<F>
where
    F: Float
{
    pub omega: F,
    pub epsilon: F,
    pub xi: F
}