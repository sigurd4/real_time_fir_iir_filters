#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(associated_const_equality)]
#![feature(specialization)]
#![feature(const_trait_impl)]
#![feature(array_methods)]
#![feature(const_closures)]
#![feature(const_mut_refs)]
#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(inline_const)]
#![feature(associated_type_bounds)]
#![feature(step_trait)]
#![feature(split_array)]

moddef::moddef!(
    pub mod {
        fir for cfg(feature = "filters"),
        iir for cfg(feature = "filters")
    }
);
moddef::moddef!(
    flat(pub) mod {
        filter_any,
        filter_kind,
        filter_static,
        filter_static_coefficients,
        filter_static_internals,
        filter
    },
    flat mod {
        param,
        plot for cfg(test)
    }
);

use num::Float;

#[macro_export]
macro_rules! f {
    ($x:expr; $f:tt) => {
        <$f>::from($x).unwrap()
    };
    ($x:expr) => {
        f!($x; F)
    }
}

#[macro_export]
macro_rules! static_filter_impl {
    (
        < $($generics:ident),* > $type:ty :
        $kind:tt,
        $outputs:literal: $buffered_outputs:literal,
        $order:literal,
        $extra_stages:literal
        where
            $($where:tt)*
    ) => {
        impl<F, $($generics),*> FilterAny<F> for $type
        where
            F: Float, $($where)*
        {
            const KIND: FilterKind = FilterKind::$kind;
            const OUTPUTS: usize = $outputs;
        }
        
        impl<F, $($generics),*> FilterStatic<F> for $type
        where
            F: Float, $($where)*
        {
            const BUFFERED_OUTPUTS: bool = $buffered_outputs;
            const SOS_STAGES: usize = $extra_stages;
            const ORDER: usize = $order;
        }
    };
}

#[cfg(test)]
mod tests
{
    use array_math::ArrayOps;

    use linspace::LinspaceArray;
    use num::{Float, Complex};
    use plotters::{prelude::{DynElement, BitMapBackend}, coord::ranged1d::{AsRangedCoord, ValueFormatter}, element::PointCollection};
    use crate::{plot, Filter};
    use core::ops::Range;
    use std::{fmt::{Debug, Display}, ops::{AddAssign, SubAssign}};

    const PLOT_TARGET: &str = "plots";

    pub fn plot_freq<F, T>(
        filter: &mut T,
        two_sided: bool
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Display + Debug,

        T: Filter<F>,
        [(); T::OUTPUTS]:,
        
        F: Float + AddAssign + SubAssign + 'static,
        Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
        for<'b, 'a> &'b DynElement<'static, BitMapBackend<'a>, (F, F)>:
            PointCollection<'b, (
                <Range<F> as AsRangedCoord>::Value,
                <Range<F> as AsRangedCoord>::Value
            )>
    {
        const N: usize = 256;
        let omega: [F; N] = (if two_sided {-core::f64::consts::PI} else {core::f64::EPSILON}..core::f64::consts::PI)
            .linspace_array()
            .map(|omega| f!(omega));

        let sampling_frequency = f!(44100.0);

        let freq_response = filter.frequency_response(sampling_frequency, omega.into_iter());

        let filter_name = core::any::type_name::<T>()
            .split_terminator("::")
            .last()
            .unwrap()
            .split_terminator("<")
            .next()
            .unwrap();
        let mut first = true;
        let file_name_no_extension: String = filter_name.chars()
            .flat_map(|c| if c.is_ascii_uppercase()
            {
                if first
                {
                    first = false;
                    vec![c.to_ascii_lowercase()]
                }
                else
                {
                    vec!['_', c.to_ascii_lowercase()]
                }
            }
            else
            {
                vec![c]
            }).collect();

        for (output_number, freq_response) in freq_response.into_iter()
            .map(|freq_response| freq_response.into_iter().next_chunk().unwrap())
            .enumerate()
        {
            plot::plot_bode(
                &format!("Frequency response of '{}', o = {}, fs = {}", filter_name, output_number, sampling_frequency),
                &format!("{}/{}{}.png", PLOT_TARGET, file_name_no_extension, output_number),
                omega.zip2(freq_response),
            )?
        }
        Ok(())
    }

    #[test]
    fn inv()
    {
        const N: usize = 5;
        const M: usize = (N + 1)/2;
        const K: usize = 2usize.pow(M as u32);
        /*const INV: [[bool; N]; N + 1]*/
        let inv_map: [[_; M]; K] = ArrayOps::fill(|mut i| ArrayOps::fill(|m| {
            let b = i % 2 == 1;
            i >>= 1;
            b
        }));
        let inv: [[_; N]; K] = ArrayOps::fill(|i| ArrayOps::fill(|n| {
            inv_map[i][N.abs_diff(n*2 + 1)/2]
        }));
        println!("inv = {:?}", inv);
    }
}