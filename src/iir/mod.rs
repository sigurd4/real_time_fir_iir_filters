moddef::moddef!(
    pub mod {
        first,
        second,
        third,
        fourth,
        nth
    }
);

use crate::iir::first::FirstOrderFilter;

use super::*;

#[macro_export]
macro_rules! iir_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal: $buffered_outputs:literal,
        $order:literal,
        $extra_stages:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        static_filter_impl!(<$($generics),*> $type: IIR, $outputs: $buffered_outputs, $order, $extra_stages where $($where)*);
        $(
            static_assertions::assert_impl_all!($variant32: Filter<f32>);
        )*
        $(
            static_assertions::assert_impl_all!($variant64: Filter<f64>);
        )*
    };
}