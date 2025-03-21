moddef::moddef!(
    flat(pub) mod {
        pid for cfg(feature = "pid"),
        second_order for cfg(feature = "second_order")
    }
);