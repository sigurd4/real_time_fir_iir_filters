moddef::moddef!(
    flat(pub) mod {
        third_order_butterworth_filter for cfg(feature = "third_order_butterworth"),
        third_order_filter for cfg(feature = "third_order"),
        third_order_sallen_key_filter for cfg(feature = "third_order_sallen_key")
    }
);