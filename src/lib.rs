#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(associated_const_equality)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(associated_type_bounds)]
#![feature(split_array)]
#![feature(receiver_trait)]
#![feature(decl_macro)]
#![feature(const_refs_to_cell)]
#![feature(const_mut_refs)]

#![feature(generic_const_exprs)]
#![feature(specialization)]

pub(crate) use crate as real_time_fir_iir_filters;

moddef::moddef!(
    pub mod {
        fir for cfg(feature = "filters"),
        iir for cfg(feature = "filters"),

        internals,
        param,
        rtf,
        static_rtf
    },
    mod {
        plot for cfg(test),
        util
    }
);

mod private
{
    use crate::iir::first::{FirstOrderLRFilterParam, FirstOrderRCFilterParam};

    trait MaybeSame<T>
    where
        T: ?Sized
    {
        const IS_SAME: bool;
    }
    impl<T, U> MaybeSame<T> for U
    where
        T: ?Sized,
        U: ?Sized
    {
        default const IS_SAME: bool = false;
    }
    impl<T> MaybeSame<T> for T
    where
        T: ?Sized
    {
        const IS_SAME: bool = true;
    }

    pub(crate) trait NotSame<T>
    where
        T: ?Sized
    {

    }
    impl<T, U> NotSame<T> for U
    where
        T: ?Sized,
        U: MaybeSame<T, IS_SAME = false> + ?Sized
    {

    }

    macro_rules! not_trait_trait {
        ($trait:ident => $maybe:ident => $not:ident) => {
            trait $maybe
            {
                const IS_IMPL: bool;
            }
            impl<T> $maybe for T
            where
                T: ?Sized
            {
                default const IS_IMPL: bool = false;
            }
            impl<T> $maybe for T
            where
                T: ?Sized + $trait
            {
                const IS_IMPL: bool = true;
            }
    
            pub trait $not
            {
    
            }
            impl<T> $not for T
            where
                T: $maybe<IS_IMPL = false> + ?Sized
            {
    
            }
        }
    }

    not_trait_trait!(
        FirstOrderLRFilterParam
        => MaybeFirstOrderLRFilterParam
        => NotFirstOrderLRFilterParam
    );
    not_trait_trait!(
        FirstOrderRCFilterParam
        => MaybeFirstOrderRCFilterParam
        => NotFirstOrderRCFilterParam
    );
}

pub const fn max_len(a: usize, b: usize) -> usize
{
    if b > a
    {
        b
    }
    else
    {
        a
    }
}

#[allow(unused)]
#[macro_export]
macro_rules! f {
    ($x:expr; $($f:tt)*) => {
        <$($f)* as num::NumCast>::from($x).unwrap()
    };
    ($x:expr) => {
        f!($x; F)
    };
}

// Should be a derive macro
#[macro_export]
macro_rules! def_param {
    (
        $({
            $($docs:tt)+
        })?
        $type:ident$(<$($gg:ident),+$(,)?>)? $({
            $($var:ident: $ty:ty),+$(,)?
        })?
        $(where
            $($where:tt)+)?
    ) => {
        $($($docs)*)?
        #[derive(Clone, Copy, Debug)]
        pub struct $type$(<$($gg),*>)?
        $(where
            $($where)+)?
        $({
            $(pub $var: crate::param::Param<$ty>),*
        })?
        impl$(<$($gg),*>)? $type$(<$($gg),*>)?
        $(where
            $($where)+)?
        {
            pub const fn new($($($var: $ty),*)?) -> Self
            {
                Self $({
                    $($var: real_time_fir_iir_filters::param::Param::new($var)),*
                })?
            }
        }
        impl$(<$($gg),*>)? real_time_fir_iir_filters::param::Parameterization for $type$(<$($gg),*>)?
        $(where
            $($where)+)?
        {
            fn is_unchanged(&self) -> bool
            {
                true $($(&& self.$var.is_unchanged())*)?
            }
            fn set_unchanged(&mut self)
            {
                $($(self.$var.set_unchanged();)*)?
            }
        }
    };
}

#[macro_export]
macro_rules! def_rtf {
    (
        $({
            $($docs:tt)+
        })?
        $name:ident
        {
            type Param: $param_trait:ident = $param_default:ident;
            const OUTPUTS: usize = $outputs:literal;
            const BUFFERED_OUTPUTS: bool = $buffered_outputs:literal;
            const SOS_STAGES: usize = $sos_stages:literal;
            const ORDER: usize = $order:literal;
            const IS_IIR: bool = $is_iir:literal;

            fn make_coeffs($arg_param:ident, $arg_rate:ident) -> _
            $make_coeffs:block
        }
    ) => {
        $($($docs)*)?
        #[derive(Clone, Copy, Debug)]
        pub struct $name<F, P = $param_default<F>>
        where
            F: num::Float + bytemuck::Pod,
            P: $param_trait<F = F>
        {
            pub param: P,
            pub internals: Internals<F>
        }
        
        type Internals<F> = real_time_fir_iir_filters::internals::RtfInternalsGiven<F, $outputs, $buffered_outputs, $sos_stages, $order, $is_iir>;
        /*type B<F> = real_time_fir_iir_filters::internals::binternals!(F, $outputs, $buffered_outputs, $sos_stages, $order);
        type A<F> = real_time_fir_iir_filters::internals::ainternals!(F, $outputs, $buffered_outputs, $sos_stages, $order);*/
        
        
        impl<F, P> $name<F, P>
        where
            F: num::Float + bytemuck::Pod,
            P: $param_trait<F = F>
        {
            pub const fn new(param: P) -> Self
            {
                Self {
                    param,
                    internals: Internals::new()
                }
            }
        }
        
        impl<F, P> real_time_fir_iir_filters::rtf::RtfBase for $name<F, P>
        where
            F: num::Float + bytemuck::Pod,
            P: $param_trait<F = F>
        {
            type F = F;
        
            const IS_IIR: bool = $is_iir;
            const OUTPUTS: usize = $outputs;
        }

        impl<F, P> real_time_fir_iir_filters::static_rtf::StaticRtfBase for $name<F, P>
        where
            F: num::Float + bytemuck::Pod,
            P: $param_trait<F = F>
        {
            type Param = P;

            const BUFFERED_OUTPUTS: bool = $buffered_outputs;
            const SOS_STAGES: usize = $sos_stages;
            const ORDER: usize = $order;
            
            fn from_param(param: Self::Param) -> Self
            {
                Self::new(param)
            }
            fn get_param(&self) -> &Self::Param
            {
                &self.param
            }
            fn get_param_mut(&mut self) -> &mut Self::Param
            {
                &mut self.param
            }
            fn into_param(self) -> Self::Param
            {
                self.param
            }
            
            fn get_internals(&self) -> (&Internals<F>, &Self::Param)
            {
                (&self.internals, &self.param)
            }
            fn get_internals_mut(&mut self) -> (&mut Internals<F>, &mut Self::Param)
            {
                (&mut self.internals, &mut self.param)
            }

            fn make_coeffs($arg_param: &Self::Param, $arg_rate: Self::F) -> (
                real_time_fir_iir_filters::internals::binternals!(F, $outputs, $buffered_outputs, $sos_stages, $order),
                [real_time_fir_iir_filters::internals::ainternals!(F, $outputs, $buffered_outputs, $sos_stages, $order); $is_iir as usize]
            )
            $make_coeffs
        }
    };
}

#[cfg(test)]
mod tests
{
    use linspace::LinspaceArray;
    use num::{Complex, Float};
    use plotters::{prelude::{DynElement, BitMapBackend}, coord::ranged1d::{AsRangedCoord, ValueFormatter}, element::PointCollection};
    use crate::{plot, rtf::Rtf};
    use core::ops::Range;
    use std::{fmt::{Debug, Display}, ops::{AddAssign, SubAssign}};

    const PLOT_TARGET: &str = "plots";

    pub fn plot_freq<F, T>(
        filter: &mut T,
        two_sided: bool
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Display + Debug,
        T: Rtf<F = F>,
        [(); T::OUTPUTS - 1]:,
        
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

        let freq_response = omega.into_iter()
            .map(|omega| filter.frequency_response(sampling_frequency, omega));

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

        let freq_response = {
            let mut h: [_; T::OUTPUTS] = core::array::from_fn(|_| Box::new([Complex::from(F::zero()); N]));

            for (i, hh) in freq_response.into_iter()
                .enumerate()
            {
                for (h, hh) in h.iter_mut()
                    .zip(hh)
                {
                    h[i] = hh;
                }
            }

            h
        };

        for (output_number, freq_response) in freq_response.into_iter()
            .enumerate()
        {
            plot::plot_bode(
                &format!("Frequency response of '{}', o = {}, fs = {}", filter_name, output_number, sampling_frequency),
                &format!("{}/{}{}.png", PLOT_TARGET, file_name_no_extension, output_number),
                omega.zip(*freq_response),
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
        let inv_map: [[_; M]; K] = core::array::from_fn(|mut i| core::array::from_fn(|m| {
            let b = i % 2 == 1;
            i >>= 1;
            b
        }));
        let inv: [[_; N]; K] = core::array::from_fn(|i| core::array::from_fn(|n| {
            inv_map[i][N.abs_diff(n*2 + 1)/2]
        }));
        println!("inv = {:?}", inv);
    }
}