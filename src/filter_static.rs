use super::*;

pub trait FilterStatic<F>: FilterAny<F>
where
    F: Float,
{
    const BUFFERED_OUTPUTS: bool;
    const SOS_STAGES: usize;
    const ORDER: usize;
}