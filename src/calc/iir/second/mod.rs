moddef::moddef!(
    flat(pub) mod {
        pid for cfg(feature = "pid"),
        second_order_chebyshev1 for cfg(feature = "second_order_chebyshev1"),
        second_order_chebyshev2 for cfg(feature = "second_order_chebyshev2"),
        second_order_elliptic for cfg(feature = "second_order_elliptic"),
        second_order_rc for cfg(feature = "second_order_rc"),
        second_order_rlc for cfg(feature = "second_order_rlc"),
        second_order_sallen_key for cfg(feature = "second_order_sallen_key"),
        second_order for cfg(feature = "second_order")
    }
);