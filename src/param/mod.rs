use core::{borrow::{Borrow, BorrowMut}, fmt::Debug, ops::{Deref, DerefMut}};

use bytemuck::Pod;
use num::{traits::FloatConst, Float};
use serde::{Serialize, Deserialize};
use private::ParamChange;

use crate::util::ZeroSized;

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

mod private
{
    use serde::Serialize;

    use crate::util::ZeroSized;

    pub(super) trait ParamChange: Sized + Copy + core::fmt::Debug + Eq + Ord + Serialize
    {
        const NEW: Self;
        
        fn set_changed(&mut self);
        fn set_unchanged(&mut self);
        fn is_unchanged(&self) -> bool;
    }

    impl ParamChange for ()
    {
        const NEW: Self = ();
        
        fn set_changed(&mut self)
        {
            
        }
        fn set_unchanged(&mut self)
        {
            
        }
        fn is_unchanged(&self) -> bool
        {
            true
        }
    }
    impl ParamChange for bool
    {
        const NEW: Self = true;

        fn set_changed(&mut self)
        {
            *self = true
        }
        fn set_unchanged(&mut self)
        {
            *self = false
        }
        fn is_unchanged(&self) -> bool
        {
            !*self
        }
    }
    pub(super) trait ParamSpec
    {
        type Change: ParamChange;
    }
    impl<T> ParamSpec for T
    {
        default type Change = bool;
    }
    impl<T> ParamSpec for T
    where
        T: ZeroSized
    {
        type Change = ();
    }
}

const fn new_change<T>() -> T
where
    T: ParamChange
{
    T::NEW
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Param<T>
{
    value: T,
    #[serde(skip, default = "new_change")]
    has_maybe_changed: <T as private::ParamSpec>::Change
}
impl<T> Param<T>
{
    pub const fn new(value: T) -> Self
    {
        Self {
            value,
            has_maybe_changed: ParamChange::NEW
        }
    }
    pub fn assign(&mut self, value: T)
    where
        T: PartialEq
    {
        if self.value != value
        {
            self.has_maybe_changed.set_changed();
            self.value = value
        }
    }
    pub const fn get(&self) -> &T
    {
        &self.value
    }
    pub fn get_mut(&mut self) -> &mut T
    {
        self.has_maybe_changed.set_changed();
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
        self.has_maybe_changed.is_unchanged()
    }
    pub fn set_unchanged(&mut self)
    {
        self.has_maybe_changed.set_unchanged()
    }
    pub fn is_unchanged_then_set(&mut self) -> bool
    {
        let b = self.is_unchanged();
        self.set_unchanged();
        b
    }
}
impl<T> Param<T>
where
    T: ZeroSized
{
    pub fn null<'a>() -> &'a mut Self
    {
        static mut P: Param<()> = Param::new(());

        assert_eq!(core::mem::size_of::<Param<()>>(), 0);
        assert_eq!(core::mem::size_of::<Param<()>>(), core::mem::size_of::<T>());

        unsafe {
            &mut *(&raw mut P).cast()
        }
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
impl<T> PartialEq for Param<T>
where
    T: PartialEq
{
    fn eq(&self, other: &Self) -> bool
    {
        (**self).eq(&**other)
    }
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool
    {
        (**self).ne(&**other)
    }
}
impl<T> Eq for Param<T>
where
    T: Eq
{
    
}
impl<T> PartialOrd for Param<T>
where
    T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        (**self).partial_cmp(&**other)
    }
    fn ge(&self, other: &Self) -> bool
    {
        (**self).ge(&**other)
    }
    fn gt(&self, other: &Self) -> bool
    {
        (**self).gt(&**other)
    }
    fn le(&self, other: &Self) -> bool
    {
        (**self).le(&**other)
    }
    fn lt(&self, other: &Self) -> bool
    {
        (**self).lt(&**other)
    }
}
impl<T> Ord for Param<T>
where
    T: Ord
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        (**self).cmp(&**other)
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized
    {
        Self::new(self.into_value().clamp(min.into_value(), max.into_value()))
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized
    {
        Self::new(self.into_value().max(other.into_value()))
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized
    {
        Self::new(self.into_value().min(other.into_value()))
    }
}