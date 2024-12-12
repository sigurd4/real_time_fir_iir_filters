#![allow(incomplete_features)]
#![allow(internal_features)]

#![feature(decl_macro)]
#![feature(const_type_id)]
#![feature(associated_const_equality)]
#![feature(inherent_associated_types)]

#![feature(core_intrinsics)]
#![feature(specialization)]
#![feature(generic_const_exprs)]

pub macro type_id_cmp {
    ($c1:ty, $c2:ty => $lt:ty, $eq:ty, $gt:ty) => {
        <$c1 as private::TypeIdCmp<$c2>>::TypeIdCmp<$lt, $eq, $gt>
    }
}

mod private
{
    use core::cmp::Ordering;

    trait TypeIdCmpBase<T>: 'static
    where
        T: 'static
    {
        const TYPE_ID_CMP: Ordering;
        const TYPE_ID_LT: bool;
        const TYPE_ID_EQ: bool;
        const TYPE_ID_GT: bool;
    }
    impl<T, U> TypeIdCmpBase<T> for U
    where
        T: 'static,
        U: 'static
    {
        const TYPE_ID_CMP: Ordering = _type_id_cmp::<U, T>();
        const TYPE_ID_LT: bool = <Self as TypeIdCmpBase<T>>::TYPE_ID_CMP.is_lt();
        const TYPE_ID_EQ: bool = <Self as TypeIdCmpBase<T>>::TYPE_ID_CMP.is_eq();
        const TYPE_ID_GT: bool = <Self as TypeIdCmpBase<T>>::TYPE_ID_CMP.is_gt();
    }
    pub trait TypeIdEq<T>
    {
        type Select<True, False>;
    }
    impl<T, U> TypeIdEq<T> for U
    {
        default type Select<True, False> = False;
    }
    impl<T, U> TypeIdEq<T> for U
    where
        T: 'static,
        U: TypeIdCmpBase<T, TYPE_ID_EQ = true>
    {
        type Select<True, False> = True;
    }
    pub trait TypeIdLt<T>
    {
        type Select<True, False>;
    }
    impl<T, U> TypeIdLt<T> for U
    {
        default type Select<True, False> = False;
    }
    impl<T, U> TypeIdLt<T> for U
    where
        T: 'static,
        U: TypeIdCmpBase<T, TYPE_ID_LT = true>
    {
        type Select<True, False> = True;
    }
    pub trait TypeIdCmp<T>
    {
        type TypeIdCmp<Lt, Eq, Gt>;
    }
    impl<T, U> TypeIdCmp<T> for U
    where
        T: 'static,
        U: TypeIdEq<T> + TypeIdLt<T>
    {
        type TypeIdCmp<Lt, Eq, Gt> = <U as TypeIdEq<T>>::Select<Eq, <U as TypeIdLt<T>>::Select<Lt, Gt>>;
    }

    const fn _type_id_cmp<CC, C>() -> Ordering
    where
        CC: 'static,
        C: 'static
    {
        let n = core::intrinsics::type_id::<CC>();
        let m = core::intrinsics::type_id::<C>();
    
        if n > m
        {
            Ordering::Greater
        }
        else if n < m
        {
            Ordering::Less
        }
        else
        {
            Ordering::Equal
        }
    }
    
    mod test
    {
        use crate::type_id_cmp;
    
        trait Test
        {
            type O;
        }
    
        impl<T> Test for T
        {
            type O = type_id_cmp!(T, u8 => (), u8, ());
        }
    
        #[test]
        fn test()
        {
            fn test<T>(_: <T as Test>::O)
            {
    
            }
    
            test::<u8>(1)
        }
    }
}

pub enum Cond<const COND: bool> {}

pub macro select {
    ($ty:ty) => {
        $ty
    },
    (
        if $cond:expr =>
        {
            $($if:tt)*
        }
        else
        {
            $($else:tt)*
        }
    ) => {
        <Cond<$cond> as Select>::Select<select!($($if)*), select!($($else)*)>
    },
    (
        if $cond:expr =>
        {
            $($if:tt)*
        }
        else if $cond_elif:expr =>
        {
            $($elif:tt)*
        }
        $(else if $cond_elif_more:expr =>
        {
            $($elif_more:tt)*
        })*
        else
        {
            $($else:tt)*
        }
    ) => {
        cond::select!(
            if $cond =>
            {
                $($if)*
            }
            else
            {
                if $cond_elif =>
                {
                    $($elif)*
                }
                $(else if $cond_elif_more =>
                {
                    $($elif_more)*
                })*
                else
                {
                    $($else)*
                }
            }
        )
    },
}

pub trait Select
{
    type Select<True, False>;
}
impl<const COND: bool> Select for Cond<COND>
{
    default type Select<True, False> = False;
}
impl Select for Cond<true>
{
    type Select<True, False> = True;
}
impl Select for Cond<false>
{
    type Select<True, False> = False;
}

pub enum EitherCond<const COND1: bool, const COND2: bool> {}

pub trait True
{

}
impl True for Cond<true>
{

}
impl<const COND2: bool> True for EitherCond<true, COND2>
{

}
impl True for EitherCond<false, true>
{

}
pub trait False
{

}
impl False for Cond<false>
{
    
}
impl False for EitherCond<false, false>
{
    
}