
moddef::moddef!(
    flat(pub) mod {
        butterworth for cfg(feature = "param_omega"),
        chebyshev for cfg(feature = "param_omega_epsilon"),
        elliptic for cfg(feature = "param_omega_epsilon_xi"),
        first_order_all_pass for cfg(feature = "param_tau"),
        first_order for cfg(feature = "param_omega"),
        first_order_lr for cfg(feature = "param_lr"),
        first_order_rc for cfg(feature = "param_rc"),
        pi for cfg(feature = "param_pi"),
        pid for cfg(feature = "param_pid"),
        second_order for cfg(feature = "param_omega_zeta"),
        second_order_rc for cfg(feature = "param_rc2"),
        second_order_rlc for cfg(feature = "param_rlc"),
        second_order_sallen_key for cfg(feature = "param_rc2_sallen_key"),
        third_order for cfg(feature = "param_omega2_zeta"),
        third_order_sallen_key for cfg(feature = "param_rc3_sallen_key"),
        wah_filter_param for cfg(feature = "param_wah")
    }
);