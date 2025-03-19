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
        tau for cfg(feature = "param_tau")
    }
);