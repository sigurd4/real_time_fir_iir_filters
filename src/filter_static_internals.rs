use super::*;

pub trait FilterStaticInternals<F>: FilterStaticStages<F> + FilterStaticOrder<F>
where
    F: Float
{
    fn w(&mut self) -> ([&mut [[F; 2]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], &mut [[F; Self::ORDER]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]);
}