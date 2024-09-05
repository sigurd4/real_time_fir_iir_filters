use core::{borrow::{Borrow, BorrowMut}, fmt::Debug, ops::{Deref, DerefMut, Receiver}};

use bytemuck::Pod;
use num::{traits::FloatConst, Float};

moddef::moddef!(
    flat(pub) mod {
        butterworth_filter_param,
        chebyshev_filter_param,
        chebyshev1_filter_param,
        chebyshev2_filter_param,
        elliptic_filter_param,
        first_order_all_pass_filter_param,
        first_order_filter_param,
        first_order_lr_filter_param,
        first_order_rc_filter_param,
        pi_filter_param,
        pid_filter_param,
        second_order_filter_param,
        second_order_rc_filter_param,
        second_order_rlc_filter_param,
        second_order_sallen_key_filter_param,
        third_order_filter_param,
        third_order_sallen_key_filter_param,
        wah_filter_param
    }
);

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

pub trait FilterFloat: Float + FloatConst + Pod
{

}
impl<F> FilterFloat for F
where
    F: Float + FloatConst + Pod
{

}

pub trait FilterParam: Parameterization
{
    const ORDER: usize;
    
    type F: FilterFloat;
}

pub trait FilterParamFirstOrder: FilterParam<ORDER = 1>
{

}
pub trait FilterParamSecondOrder: FilterParam<ORDER = 2>
{
    
}
pub trait FilterParamThirdOrder: FilterParam<ORDER = 3>
{
    
}

#[derive(Clone, Copy, Debug)]
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