moddef::moddef!(
    flat(pub) mod {
        lr for cfg(feature = "param_lr"),
        omega_epsilon_xi for cfg(feature = "param_omega_epsilon_xi"),
        omega_epsilon for cfg(feature = "param_omega_epsilon"),
        omega_zeta for cfg(feature = "param_omega_zeta"),
        omega for cfg(feature = "param_omega"),
        omega2_zeta for cfg(feature = "param_omega2_zeta"),
        pi for cfg(feature = "param_pi"),
        pid for cfg(feature = "param_pid"),
        rc for cfg(feature = "param_rc"),
        rc2_sallen_key for cfg(feature = "param_rc2_sallen_key"),
        rc2 for cfg(feature = "param_rc2_sallen_key"),
        rc2g_sallen_key for cfg(feature = "param_rc2_sallen_key"),
        rc3_sallen_key for cfg(feature = "param_rc3_sallen_key"),
        rc3g_sallen_key for cfg(feature = "param_rc3_sallen_key"),
        rlc for cfg(feature = "param_rlc"),
        tau for cfg(feature = "param_tau"),
        wah for cfg(feature = "param_wah")
    }
);