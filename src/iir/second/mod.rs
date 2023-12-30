moddef::moddef!(
    flat(pub) mod {
        pid_filter,
        second_order_filter,
        second_order_butterworth_filter,
        second_order_chebyshev1_filter,
        second_order_chebyshev2_filter,
        second_order_elliptic_filter,
        second_order_rlc_filter,
        second_order_rc_filter,
        second_order_sallen_key_filter
    }
);

use super::*;

#[macro_export]
macro_rules! iir2_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal : $buffered_outputs:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        iir_impl!(<$($generics),*> $type: $outputs: $buffered_outputs, 2, 0 => $($variant32),*; $($variant64),* where $($where)*);
    };
}