moddef::moddef!(
    flat(pub) mod {
        nth
    }
);

use super::*;

#[macro_export]
macro_rules! fir_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal,
        $order:literal,
        $extra_stages:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        static_filter_impl!(<$($generics),*> $type: FIR, $outputs, $order, $extra_stages where $($where)*);
    };
}