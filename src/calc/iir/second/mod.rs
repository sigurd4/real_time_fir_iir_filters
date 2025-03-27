moddef::moddef!(
    flat(pub) mod {
        pid for cfg(feature = "pid"),
        second_order_chebyshev1 for cfg(feature = "second_order_chebyshev1"),
        second_order_chebyshev2 for cfg(feature = "second_order_chebyshev2"),
        second_order_elliptic for cfg(feature = "second_order_elliptic"),
        second_order for cfg(feature = "second_order")
    }
);