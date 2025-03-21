moddef::moddef!(
    flat(pub) mod {
        first_order_all_pass for cfg(feature = "first_order_all_pass"),
        first_order_lr for cfg(feature = "first_order_lr"),
        first_order_rc for cfg(feature = "first_order_rc"),
        first_order for cfg(feature = "first_order"),
        pi for cfg(feature = "pi")
,    }
);