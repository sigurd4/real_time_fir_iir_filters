moddef::moddef!(
    flat(pub) mod {
        first_order_all_pass_filter,
        first_order_filter,
        first_order_lr_filter,
        first_order_rc_filter,
        pi_filter
    }
);

crate::impl_from!(FirstOrderFilter <=> FirstOrderHighPassFilter: FirstOrderFilterParam);
crate::impl_from!(FirstOrderFilter <=> FirstOrderLowPassFilter: FirstOrderFilterParam);
crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderHighPassFilter: FirstOrderFilterParam);

crate::impl_from!(FirstOrderFilter <=> FirstOrderLRFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderFilter <=> FirstOrderLRHighPassFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderFilter <=> FirstOrderLRLowPassFilter: FirstOrderLRFilterParam);

crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderLRFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderLRHighPassFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderLRLowPassFilter: FirstOrderLRFilterParam);

crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderLRFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderLRHighPassFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderLRLowPassFilter: FirstOrderLRFilterParam);

crate::impl_from!(FirstOrderFilter <=> FirstOrderRCFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderFilter <=> FirstOrderRCHighPassFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderFilter <=> FirstOrderRCLowPassFilter: FirstOrderRCFilterParam);

crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderRCFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderRCHighPassFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderLowPassFilter <=> FirstOrderRCLowPassFilter: FirstOrderRCFilterParam);

crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderRCFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderRCHighPassFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderHighPassFilter <=> FirstOrderRCLowPassFilter: FirstOrderRCFilterParam);

// LR
crate::impl_from!(FirstOrderLRFilter <=> FirstOrderLRHighPassFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderLRFilter <=> FirstOrderLRLowPassFilter: FirstOrderLRFilterParam);
crate::impl_from!(FirstOrderLRLowPassFilter <=> FirstOrderLRHighPassFilter: FirstOrderLRFilterParam);

// RC
crate::impl_from!(FirstOrderRCFilter <=> FirstOrderRCHighPassFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderRCFilter <=> FirstOrderRCLowPassFilter: FirstOrderRCFilterParam);
crate::impl_from!(FirstOrderRCLowPassFilter <=> FirstOrderRCHighPassFilter: FirstOrderRCFilterParam);