moddef::moddef!(
    flat(pub) mod {
        third_order_butterworth_filter
    }
);

use super::*;

#[macro_export]
macro_rules! iir3_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal : $buffered_outputs:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        iir_impl!(<$($generics),*> $type: $outputs: $buffered_outputs, 3, 0 => $($variant32),*; $($variant64),* where $($where)*);
    };
}