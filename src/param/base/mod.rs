moddef::moddef!(
    flat(pub) mod {
        chebyshev for cfg(any(
            feature = "second_order_elliptic",
            feature = "second_order_chebyshev1",
            feature = "second_order_chebyshev2",
            feature = "second_order_butterworth",
            feature = "third_order_butterworth",
            feature = "first_order",
            feature = "second_order",
            feature = "third_order",
            feature = "first_order_lr",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        elliptic for cfg(any(
            feature = "second_order_elliptic",
            feature = "second_order_chebyshev1",
            feature = "second_order_chebyshev2",
            feature = "second_order_butterworth",
            feature = "third_order_butterworth",
            feature = "first_order",
            feature = "second_order",
            feature = "third_order",
            feature = "first_order_lr",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        first_order_all_pass for cfg(any(
            feature = "first_order_all_pass",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        first_order for cfg(any(
            feature = "second_order_elliptic",
            feature = "second_order_chebyshev1",
            feature = "second_order_chebyshev2",
            feature = "second_order_butterworth",
            feature = "third_order_butterworth",
            feature = "first_order",
            feature = "second_order",
            feature = "third_order",
            feature = "first_order_lr",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        second_order_rc for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        second_order_rlc for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        second_order for cfg(any(
            feature = "second_order_elliptic",
            feature = "second_order_chebyshev1",
            feature = "second_order_chebyshev2",
            feature = "second_order_butterworth",
            feature = "third_order_butterworth",
            feature = "first_order",
            feature = "second_order",
            feature = "third_order",
            feature = "first_order_lr",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        third_order_sallen_key for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        third_order for cfg(any(
            feature = "second_order_elliptic",
            feature = "second_order_chebyshev1",
            feature = "second_order_chebyshev2",
            feature = "second_order_butterworth",
            feature = "third_order_butterworth",
            feature = "first_order",
            feature = "second_order",
            feature = "third_order",
            feature = "first_order_lr",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        ))
    }
);