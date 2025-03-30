moddef::moddef!(
    flat(pub) mod {
        lr for cfg(feature = "first_order_lr"),
        omega_epsilon_xi for cfg(any(
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
        omega_epsilon for cfg(any(
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
        omega_zeta for cfg(any(
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
        omega for cfg(any(
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
        omega2_zeta for cfg(any(
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
        pi for cfg(any(
            feature = "pi",
            feature = "pid"
        )),
        pid for cfg(feature = "pid"),
        rc for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rc2_sallen_key for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rc2 for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rc2g_sallen_key for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rc3_sallen_key for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rc3g_sallen_key for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        rlc for cfg(any(
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        tau for cfg(any(
            feature = "first_order_all_pass",
            feature = "first_order_rc",
            feature = "second_order_rc",
            feature = "second_order_rlc",
            feature = "second_order_sallen_key",
            feature = "third_order_sallen_key"
        )),
        wah for cfg(feature = "wah"),
        x for cfg(feature = "wah")
    }
);