#![feature(associated_const_equality)]
#![feature(tuple_trait)]
#![feature(decl_macro)]

use core::marker::Tuple;

use cond::type_id_cmp as cmp;

moddef::moddef!(
    mod util
);

pub macro set {
    ($($c:ty),+) => {
        <($($c,)*) as TupleSet>::Set
    }
}

macro sort {
    ({$($first:ty),+}) => {
        ($($first,)*)
    },
    ($first:ty) => {
        sort!({$first})
    },
    ({$({$($first_pre:ty),+})? $first_first:ty $(,$first_more:ty)+} $second:ty $(,$more:ty)*) => {
        cmp!($first_first, $second => 
            sort!({{$($($first_pre,)*)? $first_first} $($first_more),*} $second $(,$more)*),
            set!($($($first_pre,)*)? $first_first $(,$first_more)* $(,$more)*),
            sort!({$($($first_pre,)*)? $second, $first_first $(,$first_more)*} $($more),*)
        )
    },
    ({$({$($first_pre:ty),+})? $first_first:ty} $second:ty $(,$more:ty)*) => {
        cmp!($first_first, $second => 
            sort!({$($($first_pre,)*)? $first_first, $second} $($more),*),
            set!($($($first_pre,)*)? $first_first $(,$more)*),
            sort!({$($($first_pre,)*)? $second, $first_first} $($more),*)
        )
    },
    ($first:ty, $second:ty $(,$more:ty)*) => {
        cmp!($first, $second =>
            sort!({$first, $second} $($more),*),
            set!($first $(,$more)*),
            sort!({$second, $first} $($more),*)
        )
    },
}

trait TupleSet: Tuple
{
    type Set;
}

macro impl_set {
    ($first:ident $($(,$more:ident)+)?) => {
        impl<$first $($(,$more)*)?> TupleSet for ($first, $($($more),*)?)
        where
            $first: 'static,
            $($($more: 'static),*)?
        {
            type Set = sort!($first $($(,$more)*)?);
        }
        $(impl_set!($($more),*);)?
    }
}

impl_set!(_1, _2, _3, _4, _5);

#[cfg(test)]
mod test
{
    type Test = super::set!(u8, u16, u8, u8, u8);

    #[test]
    fn test()
    {
        fn test(_: Test)
        {

        }

        let t: Test = unsafe {core::mem::zeroed()};

        let t = t.1;
    }
}