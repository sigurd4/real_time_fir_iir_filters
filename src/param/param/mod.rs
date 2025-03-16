
moddef::moddef!(
    flat(pub) mod {
        butterworth_filter_param for cfg(feature = "param_omega"),
        chebyshev_filter_param for cfg(feature = "param_omega_epsilon"),
        chebyshev1_filter_param for cfg(feature = "param_omega_epsilon"),
        chebyshev2_filter_param for cfg(feature = "param_omega_epsilon"),
        elliptic_filter_param for cfg(feature = "param_omega_epsilon_xi"),
        first_order_all_pass_filter_param for cfg(feature = "param_tau"),
        first_order_filter_param for cfg(feature = "param_omega"),
        first_order_lr_filter_param for cfg(feature = "param_lr"),
        first_order_rc_filter_param for cfg(feature = "param_rc"),
        pi_filter_param for cfg(feature = "param_pi"),
        pid_filter_param for cfg(feature = "param_pid"),
        second_order_filter_param for cfg(feature = "param_omega_zeta"),
        second_order_rc_filter_param for cfg(feature = "param_rc2"),
        second_order_rlc_filter_param for cfg(feature = "param_rlc"),
        second_order_sallen_key_filter_param for cfg(feature = "param_rc2_sallen_key"),
        third_order_filter_param for cfg(feature = "param_omega2_zeta"),
        third_order_sallen_key_filter_param for cfg(feature = "param_rc3_sallen_key"),
        wah_filter_param for cfg(feature = "param_wah")
    }
);