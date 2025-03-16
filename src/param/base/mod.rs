moddef::moddef!(
    flat(pub) mod {
        chebyshev for cfg(feature = "param_omega_epsilon"),
        elliptic for cfg(feature = "param_omega_epsilon_xi"),
        first_order_all_pass for cfg(feature = "first_order_all_pass"),
        first_order for cfg(feature = "param_omega"),
        second_order_rc for cfg(feature = "param_rc2"),
        second_order_rlc for cfg(feature = "param_rlc"),
        second_order for cfg(feature = "param_omega_zeta"),
        third_order_sallen_key for cfg(feature = "param_rc3_sallen_key"),
        third_order for cfg(feature = "param_omega2_zeta")
    }
);