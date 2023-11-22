moddef::moddef!(
    flat(pub) mod {
        first_order_filter,
        first_order_all_pass_filter,
        first_order_lr_filter,
        first_order_rc_filter
    }
);

use super::*;

#[macro_export]
macro_rules! iir1_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal : $buffered_outputs:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        iir_impl!(<$($generics),*> $type: $outputs: $buffered_outputs, 1, 0 => $($variant32),*; $($variant64),* where $($where)*);
    };
}

#[macro_export]
macro_rules! first_order_parameterization {
    (
        < $($generics:ident),* > $type:ty
        $(
            where
                $($where:tt)+
        )?
    ) => {
        impl<F $(, $generics)*> FilterStaticCoefficients<F> for $type
        where
            F: Float,
            [(); Self::ORDER + 1]:,
            [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:,
            $($($where)+)?
        {
            fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 2]; 2])
            {
                let omega = self.omega();
                ([], [
                    [
                        omega,
                        omega
                    ],
                    [
                        f!(2.0)*rate,
                        f!(-2.0)*rate
                    ]
                ])
            }

            fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 2]; 1])>
            {
                let omega = self.omega();
                Some(([], [[
                    omega + f!(2.0)*rate,
                    omega - f!(2.0)*rate,
                ]]))
            }
        }
    };
}