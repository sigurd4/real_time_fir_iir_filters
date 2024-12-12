moddef::moddef!(
    pub flat mod {
        first,
        second,
        third,
        fourth,
        //nth
    }
);

crate::impl_from!(PIFilter <=> PIDFilter: PIFilterParam);

crate::impl_from!(SecondOrderRCFilter <=> FirstOrderRCFilter: FirstOrderRCFilterParam);
crate::impl_from!(SecondOrderRLCFilter <=> FirstOrderRCFilter: FirstOrderRCFilterParam);