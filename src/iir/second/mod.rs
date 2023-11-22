moddef::moddef!(
    flat(pub) mod {
        pid_filter,
        second_order_filter,
        second_order_butterworth_filter,
        second_order_chebyshev1_filter,
        second_order_chebyshev2_filter,
        second_order_elliptic_filter,
        second_order_rlc_filter,
        second_order_sallen_key_filter
    }
);

use super::*;

#[macro_export]
macro_rules! iir2_impl {
    (
        < $($generics:ident),* > $type:ty :
        $outputs:literal : $buffered_outputs:literal
        => $($variant32:ty),*; $($variant64:ty),*
        where
            $($where:tt)*
    ) => {
        iir_impl!(<$($generics),*> $type: $outputs: $buffered_outputs, 2, 0 => $($variant32),*; $($variant64),* where $($where)*);
    };
}

#[macro_export]
macro_rules! second_order_parameterization {
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
            fn b(&self, rate: F) -> ([[[F; 3]; 0]; 0], [[F; 3]; 3])
            {
                let omega = self.omega();
                let omega2 = omega*omega;
        
                let rate2 = rate*rate;
                ([], [
                    [
                        omega2,
                        omega2*f!(2.0),
                        omega2
                    ],
                    [
                        rate*omega*f!(2.0),
                        f!(0.0; F),
                        rate*omega*f!(-2.0),
                    ],
                    [
                        rate2*f!(4.0),
                        rate2*f!(-8.0),
                        rate2*f!(4.0)
                    ]
                ])
            }

            fn a(&self, rate: F) -> Option<([[[F; 3]; 0]; 0], [[F; 3]; 1])>
            {
                let omega = self.omega();
                let omega2 = omega*omega;
        
                let zeta = self.zeta();
        
                let rate2 = rate*rate;
                Some(([], [
                    [
                        rate2*f!(4.0) + rate*zeta*omega*f!(4.0) + omega2,
                        omega2*f!(2.0) - rate2*f!(8.0),
                        rate2*f!(4.0) - rate*zeta*omega*f!(4.0) + omega2
                    ]
                ]))
            }
        }
    };
}