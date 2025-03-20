use crate::param::FilterFloat;

#[derive(Clone, Copy, Debug)]
pub struct X<F>
where
    F: FilterFloat
{
    pub x: F
}