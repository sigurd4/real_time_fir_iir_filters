use std::ops::Deref;

pub trait Param<F> = Sized
where
    for<'a> &'a Self: Deref<Target = F>;