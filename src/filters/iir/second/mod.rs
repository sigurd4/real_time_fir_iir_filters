moddef::moddef!(
    flat(pub) mod {
        pid_filter for cfg(feature = "pid"),
        second_order_butterworth_filter for cfg(feature = "second_order_butterworth"),
        second_order_chebyshev1_filter for cfg(feature = "second_order_chebyshev1"),
        second_order_chebyshev2_filter for cfg(feature = "second_order_chebyshev2"),
        second_order_elliptic_filter for cfg(feature = "second_order_elliptic"),
        second_order_filter for cfg(feature = "second_order"),
        second_order_rc_filter for cfg(feature = "second_order_rc"),
        second_order_rlc_filter for cfg(feature = "second_order_rlc"),
        second_order_sallen_key_filter for cfg(feature = "second_order_sallen_key")
    }
);