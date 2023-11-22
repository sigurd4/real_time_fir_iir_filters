use super::*;

pub trait FilterStaticOrder<F>: FilterAny<F>
where
    F: Float,
{
    const ORDER: usize;
}