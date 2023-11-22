use super::*;

pub trait FilterAny<F>
where
    F: Float,
{
    const OUTPUTS: usize;
    const KIND: FilterKind;

    fn on_filter_pre(&mut self, _rate: F)
    {
        
    }
}