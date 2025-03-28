moddef::moddef!(
    flat(pub) mod {
        third_order_butterworth for cfg(feature = "third_order_butterworth"),
        third_order_chebyshev1 for cfg(feature = "third_order_chebyshev1"),
        third_order for cfg(feature = "third_order")
    }
);