use core::{borrow::{Borrow, BorrowMut}, fmt::Debug, ops::{Deref, DerefMut}};

use bytemuck::Pod;
use num::{traits::FloatConst, Float};
use serde::{Serialize, Deserialize};

moddef::moddef!(
    flat(pub) mod {
        base,
        conf,
        param,
        val
    }
);

pub trait FilterFloat: Float + FloatConst + Pod + Default
{

}
impl<F> FilterFloat for F
where
    F: Float + FloatConst + Pod + Default
{

}

pub trait FilterParam
{
    const ORDER: usize = 0;

    type F: FilterFloat;
}

fn tru() -> bool
{
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Param<T>
{
    value: T,
    #[serde(default = "tru")]
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
    pub fn assign(&mut self, value: T)
    where
        T: PartialEq
    {
        if self.value != value
        {
            self.has_maybe_changed = true;
            self.value = value
        }
    }
    pub const fn get(&self) -> &T
    {
        &self.value
    }
    pub const fn get_mut(&mut self) -> &mut T
    {
        self.has_maybe_changed = true;
        &mut self.value
    }
    pub const fn into_value(self) -> T
    {
        let value = unsafe {(&self.value as *const T).read()};
        core::mem::forget(self);
        value
    }
    pub fn is_unchanged(&self) -> bool
    {
        !self.has_maybe_changed
    }
    pub fn set_unchanged(&mut self)
    {
        self.has_maybe_changed = false
    }
    pub fn is_unchanged_then_set(&mut self) -> bool
    {
        let b = self.is_unchanged();
        self.set_unchanged();
        b
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
impl<T> Default for Param<T>
where
    T: Default
{
    fn default() -> Self
    {
        T::default().into()
    }
}