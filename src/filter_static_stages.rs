use super::*;

pub trait FilterStaticStages<F>: FilterAny<F>
where
    F: Float,
{
    const BUFFERED_OUTPUTS: bool;
    const SOS_STAGES: usize;
}