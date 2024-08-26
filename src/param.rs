use core::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut, Receiver}};

use bytemuck::Pod;
use num::Float;

pub trait Parameterization: Sized + 'static
{
    fn is_unchanged(&self) -> bool;
    fn set_unchanged(&mut self);
    fn is_unchanged_then_set(&mut self) -> bool
    {
        let b = self.is_unchanged();
        self.set_unchanged();
        b
    }
}
impl<T> Parameterization for Param<T>
where
    T: 'static
{
    fn is_unchanged(&self) -> bool
    {
        !self.has_maybe_changed
    }
    fn set_unchanged(&mut self)
    {
        self.has_maybe_changed = false
    }
}
pub trait FilterParam: Parameterization
{
    type F: Float + Pod;
}

pub struct Param<T>
{
    value: T,
    has_maybe_changed: bool
}
impl<T> Param<T>
{
    pub const fn new(value: T) -> Self
    {
        Self {
            value,
            has_maybe_changed: true
        }
    }
    pub fn get(&self) -> &T
    {
        self.value.borrow()
    }
    pub fn get_mut(&mut self) -> &mut T
    {
        self.has_maybe_changed = true;
        self.value.borrow_mut()
    }
    pub const fn into_value(self) -> T
    {
        let value = unsafe {(&self.value as *const T).read()};
        core::mem::forget(self);
        value
    }
}
impl<T> From<T> for Param<T>
{
    fn from(value: T) -> Self
    {
        Self::new(value)
    }
}
impl<T> Deref for Param<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        self.get()
    }
}
impl<T> DerefMut for Param<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.get_mut()
    }
}
impl<T> Borrow<T> for Param<T>
{
    fn borrow(&self) -> &T
    {
        self.get()
    }
}
impl<T> BorrowMut<T> for Param<T>
{
    fn borrow_mut(&mut self) -> &mut T
    {
        self.get_mut()
    }
}
impl<T> Receiver for Param<T>
{

}