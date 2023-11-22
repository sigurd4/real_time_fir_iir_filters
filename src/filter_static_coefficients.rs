use super::*;

pub trait FilterStaticCoefficients<F>: FilterStaticStages<F> + FilterStaticOrder<F>
where
    F: Float
{
    fn b(&self, rate: F) -> ([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS]);

    fn a(&self, _rate: F) -> Option<([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])>
    {
        None
    }
}