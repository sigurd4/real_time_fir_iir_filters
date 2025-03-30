#![allow(incomplete_features)]
#![allow(internal_features)]
#![feature(generic_arg_infer)]
#![feature(trait_alias)]
#![feature(associated_const_equality)]
#![feature(split_array)]
#![feature(decl_macro)]
#![feature(negative_impls)]
#![feature(tuple_trait)]
#![feature(iter_array_chunks)]
#![feature(derive_const)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(const_type_id)]
#![feature(const_swap_nonoverlapping)]
#![feature(adt_const_params)]
#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

//! Ever needed a low pass filter for your VST? This crate has a wide selection of filters for real-time usage. It's designed to have as little runtime overhead as
//! possible.
//!
//! # How does it work?
//!
//! Everything that can be computed at compile-time, will be, and the filter coefficients will be cached as well.
//!
//! I then use the following algorithm to process the signal with as few steps as possible given the filter's coefficients:
//!
//! ![Block diagram represnetation of linear constant-coefficient difference equations (Figure 6.5, Alan V. Oppenheimer & Ronald W. Schafer - Discrete-Time Signal Processing)](https://github.com/user-attachments/assets/bd22e03f-b69c-4506-bbbd-baccf7a6c81d)
//!
//! (The figure is from: Alan V. Oppenheimer & Ronald W. Schafer - Discrete-Time Signal Processing)
//! # Example
//!
//! ```rust
//! #![feature(generic_const_exprs)]
//!
//! use core::f64::consts::TAU;
//!
//! use real_time_fir_iir_filters::{
//!     conf::LowPass,
//!     param::OmegaEpsilonXi,
//!     rtf::Rtf,
//!     filters::iir::second::SecondOrderEllipticFilter
//! };
//!
//! // Initialize a 2. order elliptic low-pass filter at 440Hz
//! let mut filter = SecondOrderEllipticFilter::new::<LowPass>(
//!     OmegaEpsilonXi {
//!         omega: 440.0*TAU,
//!         epsilon: 0.5,
//!         xi: 1.5
//!     }
//! );
//!
//! const N: usize = 10;
//! const RATE: f64 = 8000.0;
//!
//! {
//!     // Unit impulse
//!     let mut imp_resp = [0.0; N];
//!     imp_resp[0] = 1.0;
//!
//!     // Apply filter to imp_resp
//!     for x in &mut imp_resp
//!     {
//!         [*x] = filter.filter(RATE, *x);
//!     }
//!
//!     // Prints the impulse response of the filter.
//!     println!("h[n] = {:?}", imp_resp);
//! }
//!
//! // Resets internal state of filter to zero
//! filter.reset();
//!
//! {
//!     // Generate frequency response for ω ∈ [0, 2π)
//!     let freq_resp = core::array::from_fn::<_, N, _>(|i| {
//!         let omega = i as f64/N as f64*TAU;
//!
//!         filter.frequency_response(RATE, omega)
//!     });
//!
//!     println!("H = {:?}", freq_resp);
//! }
//! ```
//!
//! # Available filters
//!
//! | Order | Filter                                                                                      | Parameterization                                                                                                                                                                                                 | Configuration                                                                                                                                                                                                                                                                                                                                                                     |
//! |-------|---------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | 1     | [`FirstOrderAllPassFilter`](crate::filters::iir::first::FirstOrderAllPassFilter)            | [`Tau`](crate::param::Tau)                                                                                                                                                                                       | [`AllPass`](crate::conf::AllPass)                                                                                                                                                                                                                                                                                                                                                 |
//! | 1     | [`FirstOrderFilter`](crate::filters::iir::first::FirstOrderFilter)                          | [`Omega`](crate::param::Omega) [`RC`](crate::param::RC) [`LR`](crate::param::LR)                                                                                                                                 | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 1     | [`FirstOrderLRFilter`](crate::filters::iir::first::FirstOrderLRFilter)                      | [`LR`](crate::param::LR)                                                                                                                                                                                         | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 1     | [`FirstOrderRCFilter`](crate::filters::iir::first::FirstOrderRCFilter)                      | [`RC`](crate::param::RC)                                                                                                                                                                                         | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 1     | [`PIFilter`](crate::filters::iir::first::PIFilter)                                          | [`PI`](crate::param::PI)                                                                                                                                                                                         | -                                                                                                                                                                                                                                                                                                                                                                                 |
//! | 2     | [`PIDFilter`](crate::filters::iir::second::PIDFilter)                                       | [`PI`](crate::param::PI) [`PID`](crate::param::PID)                                                                                                                                                              | -                                                                                                                                                                                                                                                                                                                                                                                 |
//! | 2     | [`SecondOrderButterworthFilter`](crate::filters::iir::second::SecondOrderButterworthFilter) | [`Omega`](crate::param::Omega)                                                                                                                                                                                   | [`LowPass`](crate::conf::LowPass) [`Peak`](crate::conf::Peak) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                 |
//! | 2     | [`SecondOrderChebyshev1Filter`](crate::filters::iir::second::SecondOrderChebyshev1Filter)   | [`Omega`](crate::param::Omega) [`OmegaEpsilon`](crate::param::OmegaEpsilon)                                                                                                                                      | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 2     | [`SecondOrderChebyshev2Filter`](crate::filters::iir::second::SecondOrderChebyshev2Filter)   | [`Omega`](crate::param::Omega) [`OmegaEpsilon`](crate::param::OmegaEpsilon)                                                                                                                                      | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 2     | [`SecondOrderEllipticFilter`](crate::filters::iir::second::SecondOrderEllipticFilter)       | [`Omega`](crate::param::Omega) [`OmegaEpsilon`](crate::param::OmegaEpsilon) [`OmegaEpsilonXi`](crate::param::OmegaEpsilonXi)                                                                                     | [`LowPass`](crate::conf::LowPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                                             |
//! | 2     | [`SecondOrderFilter`](crate::filters::iir::second::SecondOrderFilter)                       | [`Omega`](crate::param::Omega) [`OmegaZeta`](crate::param::OmegaZeta)                                                                                                                                            | [`LowPass`](crate::conf::LowPass) [`Peak`](crate::conf::Peak) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                                                                 |
//! | 2     | [`SecondOrderRCFilter`](crate::filters::iir::second::SecondOrderRCFilter)                   | [`RC`](crate::param::RC) [`RC2`](crate::param::RC2)                                                                                                                                                              | [`LowPass`](crate::conf::LowPass) <code>[BandPass](crate::conf::BandPass)<1><\code> <code>[BandPass](crate::conf::BandPass)<2><\code> [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                         |
//! | 2     | [`SecondOrderRLCFilter`](crate::filters::iir::second::SecondOrderRLCFilter)                 | [`RC`](crate::param::RC) [`LR`](crate::param::LR) [`RLC`](crate::param::RLC)                                                                                                            | [`LowPass`](crate::conf::LowPass) [`BandStop`](crate::conf::BandStop) [`BandPass`](crate::conf::BandPass) [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                                     |
//! | 2     | [`SecondOrderSallenKeyFilter`](crate::filters::iir::second::SecondOrderSallenKeyFilter)     | [`RC2SallenKey`](crate::param::RC2SallenKey) [`RC2GSallenKey`](crate::param::RC2GSallenKey)                                                                                                                      | [`LowPass`](crate::conf::LowPass) <code>[BandPass](crate::conf::BandPass)<1><\code> <code>[BandPass](crate::conf::BandPass)<2><\code> [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                         |
//! | 3     | [`ThirdOrderButterworthFilter`](crate::filters::iir::third::ThirdOrderButterworthFilter)    | [`Omega`](crate::param::Omega)                                                                                                                                                                                   | [`LowPass`](crate::conf::LowPass) <code>[Peak](crate::conf::Peak)<1><\code> <code>[Peak](crate::conf::Peak)<2><\code> [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                         |
//! | 3     | [`ThirdOrderFilter`](crate::filters::iir::third::ThirdOrderFilter)                          | [`Omega`](crate::param::Omega) [`OmegaZeta`](crate::param::OmegaZeta) [`Omega2Zeta`](crate::param::Omega2Zeta)                                                                                                   | [`LowPass`](crate::conf::LowPass) <code>[Peak](crate::conf::Peak)<1><\code> <code>[Peak](crate::conf::Peak)<2><\code> [`HighPass`](crate::conf::HighPass)                                                                                                                                                                                                                         |
//! | 3     | [`ThirdOrderSallenKeyFilter`](crate::filters::iir::third::ThirdOrderSallenKeyFilter)        | [`RC`](crate::param::RC) [`RC2SallenKey`](crate::param::RC2SallenKey) [`RC2GSallenKey`](crate::param::RC2GSallenKey) [`RC3SallenKey`](crate::param::RC3SallenKey) [`RC3GSallenKey`](crate::param::RC3GSallenKey) | [`LowPass`](crate::conf::LowPass) <code>[BandPass](crate::conf::BandPass)<1><\code> <code>[BandPass](crate::conf::BandPass)<2><\code> <code>[BandPass](crate::conf::BandPass)<3><\code> <code>[BandPass](crate::conf::BandPass)<4><\code> <code>[BandPass](crate::conf::BandPass)<5><\code> <code>[BandPass](crate::conf::BandPass)<6><\code> [`HighPass`](crate::conf::HighPass) |
//! | 4     | [`ẀahFilter`](crate::filters::iir::fourth::WahFilter)                                       | [`CrybabyGCB95`](crate::param::CrybabyGCB95) [`VoxV847`](crate::param::VoxV847) [`ColorsoundWow`](crate::param::ColorsoundWow)                                                                                   | -                                                                                                                                                                                                                                                                                                                                                                                 |
//!
//! ...and more to come!
//!
//! # Adding your own filter
//!
//! You can also implement your own filter, by using the macro `def_rtf!`. See how i did it with the other filters for an example on how to use the macro.

#[allow(unused)]
pub(crate) use crate as real_time_fir_iir_filters;

moddef::moddef!(
    pub mod {
        filters,
        internals,
        param,
        conf,
        rtf,
        static_rtf
    },
    mod {
        plot for cfg(test),
        util,
        calc
    }
);

/*pub fn if_cg<const B: bool, T>(then: impl FnOnce() -> T) -> [T; B as usize]
{
    unsafe {
        if B
        {
            core::intrinsics::transmute_unchecked([then()])
        }
        else
        {
            core::intrinsics::transmute_unchecked::<[T; 0], _>([])
        }
    }
}*/

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
pub const fn min_len(a: usize, b: usize) -> usize
{
    if b < a
    {
        b
    }
    else
    {
        a
    }
}
pub const fn sort_len<const N: usize>(mut a: [usize; N]) -> [usize; N]
{
    let mut i = 0;
    while i < N
    {
        let mut j = i + 1;
        while j < N
        {
            if a[i] > a[j]
            {
                unsafe {
                    core::ptr::swap_nonoverlapping(&mut a[i] as *mut usize, &mut a[j] as *mut usize, 1);
                }
            }
            j += 1;
        }
        i += 1
    }
    a
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

macro_rules! impl_from {
    ($a:ident <=> $b:ident: $p:path) => {
        /*impl<P> From<$a<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>> for $b<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>
        where
            P: $p
        {
            fn from(value: $a<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>) -> Self
            {
                Self::new(value.param)
            }
        }
        impl<P> From<$b<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>> for $a<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>
        where
            P: $p
        {
            fn from(value: $b<<P as real_time_fir_iir_filters::param::FilterParam>::F, P>) -> Self
            {
                Self::new(value.param)
            }
        }*/
    };
}

pub(crate) use impl_from;

#[doc(hidden)]
pub macro first_expr($first:expr $(,$more:expr)*) {
    $first
}

#[doc(hidden)]
#[macro_export]
macro_rules! maybe_alias_trait {
    ($trait:ident) => {
        $trait
    };
    ($alias:ident as $trait:ident) => {
        $trait
    };
}

#[doc(hidden)]
pub macro rtf_conf_const {
    (
        $(type Conf: $conf_trait_alias:ident $(as $conf_trait:path)?$( = $cc:ty)?;)?

        const $const:ident: $ty:ty = $outputs:expr;
    ) => {
        $outputs
    },
    (
        type Conf: $conf_trait_alias:ident as $conf_trait:path = $cc:ty;

        const $const:ident: $ty:ty;
    ) => {
        <$cc as $conf_trait>::$const
    },
    (
        type Conf: $conf_trait:ident = $cc:ty;

        const $const:ident: $ty:ty;
    ) => {
        <$cc as $conf_trait>::$const
    },
}

#[doc(hidden)]
#[macro_export]
macro_rules! def_rtf_internals {
    (
        $(type Conf: $conf_trait_alias:ident $(as $conf_trait:path)?;)?

        const OUTPUTS: usize = $outputs:expr;
        const O_BUFFERS: usize = $o_buffers:expr;
        const SOS_BUFFERS: usize = $sos_buffers:expr;
        const SOS_STAGES: usize = $sos_stages:expr;
        const ORDER: usize = $order:expr;
        const IS_IIR: bool = $is_iir:expr;
    ) => {
        #[allow(unused)]
        #[allow(type_alias_bounds)]
        type BInternals<F, C$(: $conf_trait_alias $(+ $conf_trait)?)? = real_time_fir_iir_filters::conf::All> = real_time_fir_iir_filters::internals::binternals!(
            F,
            $outputs,
            $o_buffers,
            $sos_buffers,
            $sos_stages,
            $order
        );
        #[allow(unused)]
        #[allow(type_alias_bounds)]
        type AInternals<F, C$(: $conf_trait_alias $(+ $conf_trait)?)? = real_time_fir_iir_filters::conf::All> = real_time_fir_iir_filters::internals::ainternals!(
            F,
            $o_buffers,
            $sos_buffers,
            $sos_stages,
            $order
        );
        #[allow(unused)]
        #[allow(type_alias_bounds)]
        type Internals<F, C$(: $conf_trait_alias $(+ $conf_trait)?)? = real_time_fir_iir_filters::conf::All> = real_time_fir_iir_filters::internals::rtfinternals!(
            F,
            $outputs,
            $o_buffers,
            $sos_buffers,
            $sos_stages,
            $order,
            $is_iir
        );
    };
    (
        type Conf: $conf_trait_alias:ident $(as $conf_trait:path)?;

        $(const OUTPUTS: usize = $outputs:expr;)?
        $(const O_BUFFERS: usize = $o_buffers:expr;)?
        $(const SOS_BUFFERS: usize = $sos_buffers:expr;)?
        $(const SOS_STAGES: usize = $sos_stages:expr;)?
        $(const ORDER: usize = $order:expr;)?
        $(const IS_IIR: bool = $is_iir:expr;)?
    ) => {
        #[allow(type_alias_bounds)]
        type BInternals<F, C: $conf_trait_alias $(+ $conf_trait)?> = real_time_fir_iir_filters::internals::binternals!(
            F,
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const OUTPUTS: usize $(= $outputs)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const O_BUFFERS: usize $(= $o_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_BUFFERS: usize $(= $sos_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_STAGES: usize $(= $sos_stages)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const ORDER: usize $(= $order)?;
            )
        );
        #[allow(type_alias_bounds)]
        type AInternals<F, C: $conf_trait_alias $(+ $conf_trait)?> = real_time_fir_iir_filters::internals::ainternals!(
            F,
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const O_BUFFERS: usize $(= $o_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_BUFFERS: usize $(= $sos_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_STAGES: usize $(= $sos_stages)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const ORDER: usize $(= $order)?;
            )
        );
        #[allow(type_alias_bounds)]
        type Internals<F, C: $conf_trait_alias $(+ $conf_trait)?> = real_time_fir_iir_filters::internals::rtfinternals!(
            F,
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const OUTPUTS: usize $(= $outputs)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const O_BUFFERS: usize $(= $o_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_BUFFERS: usize $(= $sos_buffers)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const SOS_STAGES: usize $(= $sos_stages)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const ORDER: usize $(= $order)?;
            ),
            real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias $(as $conf_trait)? = C;

                const IS_IIR: bool $(= $is_iir)?;
            )
        );
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
            $(type Conf: $conf_trait_alias:ident $(as $conf_trait:path)?)?;
            type Param: $param_trait:ident = $param_default:ident;

            $(const O_BUFFERS: usize = $o_buffers:expr;)?
            $(const SOS_BUFFERS: usize = $sos_buffers:expr;)?
            $(const SOS_STAGES: usize = $sos_stages:expr;)?
            $(const ORDER: usize = $order:expr;)?
            $(const IS_IIR: bool = $is_iir:expr;)?

            $(
                fn make_coeffs<$conf:ty>($arg_param:ident, $arg_rate:ident) -> _
                $(where
                    {$($where_c:tt)+})?
                $make_coeffs:block
            )*
        }
        $(where
            $($where:tt)+)?
    ) => {
        real_time_fir_iir_filters::def_rtf!(
            $({
                $($docs)+
            })?
            $name
            {
                $(type Conf: $conf_trait_alias $(as $conf_trait)?)?;
                type Param<C>: $param_trait as $param_trait = $param_default;

                $(const O_BUFFERS: usize = $o_buffers;)?
                $(const SOS_BUFFERS: usize = $sos_buffers;)?
                $(const SOS_STAGES: usize = $sos_stages;)?
                $(const ORDER: usize = $order;)?
                $(const IS_IIR: bool = $is_iir;)?

                $(
                    fn make_coeffs<$conf>($arg_param, $arg_rate) -> _
                    $(where
                        {$($where_c)+})?
                    $make_coeffs
                )*
            }
            $(where
                $($where)+)?
        );
    };
    (
        $({
            $($docs:tt)+
        })?
        $name:ident
        {
            type Conf: $conf_trait:ident;
            type Param$(<$cc:ident>)?: $param_alias:ident $(as $param_trait:ident)? = $param_default:ident;

            $(const O_BUFFERS: usize = $o_buffers:expr;)?
            $(const SOS_BUFFERS: usize = $sos_buffers:expr;)?
            $(const SOS_STAGES: usize = $sos_stages:expr;)?
            $(const ORDER: usize = $order:expr;)?
            $(const IS_IIR: bool = $is_iir:expr;)?

            $(
                fn make_coeffs<$conf:ty>($arg_param:ident, $arg_rate:ident) -> _
                $(where
                    {$($where_c:tt)+})?
                $make_coeffs:block
            )*
        }
        $(where
            $($where:tt)+)?
    ) => {
        real_time_fir_iir_filters::def_rtf!(
            $({
                $($docs)+
            })?
            $name
            {
                type Conf: $conf_trait as $conf_trait;
                type Param$(<$cc>)?: $param_alias $(as $param_trait)? = $param_default;

                $(const O_BUFFERS: usize = $o_buffers;)?
                $(const SOS_BUFFERS: usize = $sos_buffers;)?
                $(const SOS_STAGES: usize = $sos_stages;)?
                $(const ORDER: usize = $order;)?
                $(const IS_IIR: bool = $is_iir;)?

                $(
                    fn make_coeffs<$conf>($arg_param, $arg_rate) -> _
                    $(where
                        {$($where_c)+})?
                    $make_coeffs
                )*
            }
            $(where
                $($where)+)?
        );
    };
    (
        $({
            $($docs:tt)+
        })?
        $name:ident
        {
            type Conf: $conf_trait_alias:ident as $conf_trait:path;
            type Param<C>: $param_trait_alias:ident as $param_trait:ident = $param_default:ident;

            $(const OUTPUTS: usize = $outputs:expr;)?
            $(const O_BUFFERS: usize = $o_buffers:expr;)?
            $(const SOS_BUFFERS: usize = $sos_buffers:expr;)?
            $(const SOS_STAGES: usize = $sos_stages:expr;)?
            $(const ORDER: usize = $order:expr;)?
            const IS_IIR: bool = $is_iir:expr;

            $(
                fn make_coeffs<$conf:ty>($arg_param:ident, $arg_rate:ident) -> _
                $(where
                    {$($where_c:tt)+})?
                $make_coeffs:block
            )*
        }
        $(where
            $($where:tt)+)?
    ) => {
        real_time_fir_iir_filters::def_rtf_internals!(
            type Conf: $conf_trait_alias as $conf_trait;

            $(const OUTPUTS: usize = $outputs;)?
            $(const O_BUFFERS: usize = $o_buffers;)?
            $(const SOS_BUFFERS: usize = $sos_buffers;)?
            $(const SOS_STAGES: usize = $sos_stages;)?
            $(const ORDER: usize = $order;)?
            const IS_IIR: bool = $is_iir;
        );

        struct __Helper<F, C>
        {
            phantom: core::marker::PhantomData<(F, C)>
        }

        impl<F, C> __Helper<F, C>
        where
            F: real_time_fir_iir_filters::param::FilterFloat,
            C: $conf_trait_alias<Conf = C> + $conf_trait
        {
            const OUTPUTS: usize = real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias as $conf_trait = C;

                const OUTPUTS: usize $(= $outputs)?;
            );
            const O_BUFFERS: usize = real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias as $conf_trait = C;

                const O_BUFFERS: usize $(= $o_buffers)?;
            );
            const SOS_BUFFERS: usize = real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias as $conf_trait = C;

                const SOS_BUFFERS: usize $(= $sos_buffers)?;
            );
            const SOS_STAGES: usize = real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias as $conf_trait = C;

                const SOS_STAGES: usize $(= $sos_stages)?;
            );
            const ORDER: usize = real_time_fir_iir_filters::rtf_conf_const!(
                type Conf: $conf_trait_alias as $conf_trait = C;

                const ORDER: usize $(= $order)?;
            );
        }

        $($($docs)*)?
        #[derive(Clone, Copy, Debug)]
        pub struct $name<C = real_time_fir_iir_filters::conf::All, F = f64, P = $param_default<F>>
        where
            F: real_time_fir_iir_filters::param::FilterFloat,
            C: $conf_trait_alias<Conf = C>,
            real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<C, Conf = C> + real_time_fir_iir_filters::param::FilterParam<F = F>,
            $($($where)+)?
        {
            pub param: real_time_fir_iir_filters::param::Param<P>,
            pub internals: Internals<F, C>,
            phantom: core::marker::PhantomData<C>
        }

        impl<C, F, P> $name<C, F, P>
        where
            F: real_time_fir_iir_filters::param::FilterFloat,
            C: $conf_trait_alias<Conf = C>,
            real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<C, Conf = C> + real_time_fir_iir_filters::param::FilterParam<F = F>,
            $($($where)+)?
        {
            pub const fn new<Conf>(param: P) -> Self
            where
                real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<Conf, Conf: $conf_trait_alias<Conf = C>>,
                Conf: real_time_fir_iir_filters::conf::Conf
            {
                Self {
                    param: real_time_fir_iir_filters::param::Param::new(param),
                    internals: Internals::new(),
                    phantom: core::marker::PhantomData
                }
            }
        }

        $(
            #[allow(unused_braces)]
            impl<P> real_time_fir_iir_filters::rtf::RtfBase for $name<$conf, <real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, P>
            where
                $conf: $conf_trait_alias<Conf = $conf>,
                real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<$conf, Conf = $conf> + real_time_fir_iir_filters::param::FilterParam,
                $($($where_c)+)?
            {
                type Conf = $conf;
                type F = <real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F;

                const IS_IIR: bool = $is_iir;
                const OUTPUTS: usize = __Helper::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>::OUTPUTS;
            }
            #[allow(unused_braces)]
            impl<P> real_time_fir_iir_filters::static_rtf::StaticRtfBase for $name<$conf, <real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, P>
            where
                $conf: $conf_trait_alias<Conf = $conf>,
                real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<$conf, Conf = $conf> + real_time_fir_iir_filters::param::FilterParam,
                $($($where_c)+)?
            {
                type Param = P;

                const O_BUFFERS: usize = __Helper::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>::O_BUFFERS;
                const SOS_BUFFERS: usize = __Helper::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>::SOS_BUFFERS;
                const SOS_STAGES: usize = __Helper::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>::SOS_STAGES;
                const ORDER: usize = __Helper::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>::ORDER;

                fn from_param(param: Self::Param) -> Self
                {
                    Self {
                        param: real_time_fir_iir_filters::param::Param::new(param),
                        internals: Internals::new(),
                        phantom: core::marker::PhantomData
                    }
                }
                fn get_param(&self) -> &Self::Param
                {
                    &*self.param
                }
                fn get_param_mut(&mut self) -> &mut Self::Param
                {
                    &mut *self.param
                }
                fn into_param(self) -> Self::Param
                {
                    self.param.into_value()
                }

                #[allow(clippy::type_complexity)]
                fn get_internals(&self) -> (&Internals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>, &real_time_fir_iir_filters::param::Param<P>)
                {
                    (&self.internals, &self.param)
                }
                #[allow(clippy::type_complexity)]
                fn get_internals_mut(&mut self) -> (&mut Internals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>, &mut real_time_fir_iir_filters::param::Param<P>)
                {
                    (&mut self.internals, &mut self.param)
                }

                #[allow(clippy::type_complexity)]
                fn make_coeffs($arg_param: &real_time_fir_iir_filters::param::Param<P>, $arg_rate: Self::F) -> (
                    BInternals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>,
                    [AInternals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, $conf>; $is_iir as usize]
                )
                {
                    fn make_coeffs<F, P>($arg_param: &real_time_fir_iir_filters::param::Param<P>, $arg_rate: F) -> (
                        BInternals<F, $conf>,
                        [AInternals<F, $conf>; $is_iir as usize]
                    )
                    where
                        F: real_time_fir_iir_filters::param::FilterFloat,
                        $conf: $conf_trait_alias<Conf = $conf>,
                        real_time_fir_iir_filters::param::Param<P>: $param_trait_alias<$conf, Conf = $conf> + real_time_fir_iir_filters::param::FilterParam<F = F>,
                        $($($where_c)+)?
                    $make_coeffs

                    make_coeffs($arg_param, $arg_rate)
                }
            }
        )*
    };
    (
        $({
            $($docs:tt)+
        })?
        $name:ident
        {
            type Param: $param_trait:ident = $param_default:ident;
            const OUTPUTS: usize = $outputs:expr;
            const O_BUFFERS: usize = $o_buffers:expr;
            const SOS_BUFFERS: usize = $sos_buffers:expr;
            const SOS_STAGES: usize = $sos_stages:expr;
            const ORDER: usize = $order:expr;
            const IS_IIR: bool = $is_iir:expr;

            fn make_coeffs($arg_param:ident, $arg_rate:ident) -> _
            $make_coeffs:block
        }
        $(where
            $($where:tt)+)?
    ) => {
        #[allow(type_alias_bounds)]
        type BInternals<F> = real_time_fir_iir_filters::internals::binternals!(F, $outputs, $o_buffers, $sos_buffers, $sos_stages, $order);
        #[allow(type_alias_bounds)]
        type AInternals<F> = real_time_fir_iir_filters::internals::ainternals!(F, $o_buffers, $sos_buffers, $sos_stages, $order);
        #[allow(type_alias_bounds)]
        type Internals<F> = real_time_fir_iir_filters::internals::rtfinternals!(F, $outputs, $o_buffers, $sos_buffers, $sos_stages, $order, $is_iir);

        $($($docs)*)?
        #[derive(Clone, Copy, Debug)]
        pub struct $name<F = f64, P = $param_default<F>>
        where
            F: real_time_fir_iir_filters::param::FilterFloat,
            real_time_fir_iir_filters::param::Param<P>: $param_trait + real_time_fir_iir_filters::param::FilterParam<F = F>,
            $($($where)+)?
        {
            pub param: real_time_fir_iir_filters::param::Param<P>,
            pub internals: Internals<F>,
            phantom: core::marker::PhantomData<()>
        }

        impl<P> $name<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, P>
        where
            real_time_fir_iir_filters::param::Param<P>: $param_trait,
            $($($where)+)?
        {
            pub const fn new(param: P) -> Self
            {
                Self {
                    param: real_time_fir_iir_filters::param::Param::new(param),
                    internals: Internals::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>::new(),
                    phantom: core::marker::PhantomData
                }
            }
        }

        #[allow(unused_braces)]
        impl<P> real_time_fir_iir_filters::rtf::RtfBase for $name<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, P>
        where
            real_time_fir_iir_filters::param::Param<P>: $param_trait,
            $($($where)+)?
        {
            type Conf = real_time_fir_iir_filters::conf::All;
            type F = <real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F;

            const IS_IIR: bool = $is_iir;
            const OUTPUTS: usize = $outputs;
        }
        #[allow(unused_braces)]
        impl<P> real_time_fir_iir_filters::static_rtf::StaticRtfBase for $name<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F, P>
        where
            real_time_fir_iir_filters::param::Param<P>: $param_trait,
            $($($where)+)?
        {
            type Param = P;

            const O_BUFFERS: usize = $o_buffers;
            const SOS_BUFFERS: usize = $sos_buffers;
            const SOS_STAGES: usize = $sos_stages;
            const ORDER: usize = $order;

            fn from_param(param: Self::Param) -> Self
            {
                Self {
                    param: real_time_fir_iir_filters::param::Param::new(param),
                    internals: Internals::<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>::new(),
                    phantom: core::marker::PhantomData
                }
            }
            fn get_param(&self) -> &Self::Param
            {
                &*self.param
            }
            fn get_param_mut(&mut self) -> &mut Self::Param
            {
                &mut *self.param
            }
            fn into_param(self) -> Self::Param
            {
                self.param.into_value()
            }

            #[allow(clippy::type_complexity)]
            fn get_internals(&self) -> (&Internals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>, &real_time_fir_iir_filters::param::Param<P>)
            {
                (&self.internals, &self.param)
            }
            #[allow(clippy::type_complexity)]
            fn get_internals_mut(&mut self) -> (&mut Internals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>, &mut real_time_fir_iir_filters::param::Param<P>)
            {
                (&mut self.internals, &mut self.param)
            }

            #[allow(clippy::type_complexity)]
            fn make_coeffs($arg_param: &real_time_fir_iir_filters::param::Param<P>, $arg_rate: Self::F) -> (
                BInternals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>,
                [AInternals<<real_time_fir_iir_filters::param::Param<P> as real_time_fir_iir_filters::param::FilterParam>::F>; $is_iir as usize]
            )
            {
                fn make_coeffs<F, P>($arg_param: &real_time_fir_iir_filters::param::Param<P>, $arg_rate: F) -> (
                    BInternals<F>,
                    [AInternals<F>; $is_iir as usize]
                )
                where
                    F: real_time_fir_iir_filters::param::FilterFloat,
                    real_time_fir_iir_filters::param::Param<P>: $param_trait + real_time_fir_iir_filters::param::FilterParam<F = F>,
                    $($($where)+)?
                $make_coeffs

                make_coeffs($arg_param, $arg_rate)
            }
        }
    };
}

#[cfg(test)]
mod tests
{
    use crate::{plot, rtf::Rtf};
    use core::ops::Range;
    use linspace::LinspaceArray;
    use num::{Complex, Float};
    use plotters::{
        coord::ranged1d::{AsRangedCoord, ValueFormatter},
        element::PointCollection,
        prelude::{BitMapBackend, DynElement}
    };
    use std::{
        fmt::{Debug, Display},
        ops::{AddAssign, SubAssign}
    };

    const PLOT_TARGET: &str = "plots";

    #[cfg(feature = "second_order_elliptic")]
    #[test]
    fn it_works()
    {
        use core::f64::consts::TAU;

        use crate::{conf::LowPass, filters::iir::second::SecondOrderEllipticFilter, param::OmegaEpsilonXi, rtf::Rtf};

        let omega = 440.0 * TAU;

        // Initialize a 2. order elliptic low-pass filter at 440Hz
        let mut filter = SecondOrderEllipticFilter::new::<LowPass>(OmegaEpsilonXi { omega, epsilon: 0.5, xi: 1.5 });

        const N: usize = 10;
        const RATE: f64 = 8000.0;

        {
            // Unit impulse
            let mut imp_resp = [0.0; N];
            imp_resp[0] = 1.0;

            // Apply filter to imp_resp
            for x in &mut imp_resp
            {
                [*x] = filter.filter(RATE, *x);
            }

            // Prints the impulse response of the filter.
            println!("h[n] = {:?}", imp_resp);
        }

        // Resets internal state of filter to zero
        filter.reset();

        {
            // Generate frequency response for ω ∈ [0, 2π)
            let freq_resp = core::array::from_fn::<_, N, _>(|i| {
                let omega = i as f64 / N as f64 * TAU;

                filter.frequency_response(RATE, omega)
            });

            println!("H = {:?}", freq_resp);
        }
    }

    pub fn plot_freq<F, T>(filter: &mut T, two_sided: bool) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Display + Debug,
        T: Rtf<F = F>,
        [(); T::OUTPUTS - 1]:,

        F: Float + AddAssign + SubAssign + 'static,
        Range<F>: AsRangedCoord<CoordDescType: ValueFormatter<<Range<F> as AsRangedCoord>::Value>, Value: Debug + Clone>,
        for<'b, 'a> &'b DynElement<'static, BitMapBackend<'a>, (F, F)>: PointCollection<'b, (<Range<F> as AsRangedCoord>::Value, <Range<F> as AsRangedCoord>::Value)>
    {
        const N: usize = 256;
        let omega: [F; N] = (if two_sided { -core::f64::consts::PI } else { core::f64::EPSILON }..core::f64::consts::PI)
            .linspace_array()
            .map(|omega| f!(omega));

        let sampling_frequency = f!(44100.0);

        let freq_response = omega.into_iter().map(|omega| filter.frequency_response(sampling_frequency, omega));

        let type_name = core::any::type_name::<T>();
        let filter_name = {
            let mut k = 0;
            let mut i = 0;
            loop
            {
                if i >= type_name.len()
                {
                    break &type_name[k..];
                }
                else if type_name[i..].starts_with("::")
                {
                    i += "::".len();
                    k = i;
                }
                else if type_name[i..].starts_with("<")
                {
                    break &type_name[k..i];
                }
                else
                {
                    i += 1;
                }
            }
        };
        let mut first = true;
        let file_name_no_extension: String = filter_name
            .chars()
            .flat_map(|c| {
                if c.is_ascii_uppercase()
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
                }
            })
            .collect();

        let freq_response = {
            let mut h: [_; T::OUTPUTS] = core::array::from_fn(|_| Box::new([Complex::from(F::zero()); N]));

            for (i, hh) in freq_response.into_iter().enumerate()
            {
                for (h, hh) in h.iter_mut().zip(hh)
                {
                    h[i] = hh;
                }
            }

            h
        };

        for (output_number, freq_response) in freq_response.into_iter().enumerate()
        {
            plot::plot_bode::<F, N>(
                &format!("Frequency response of '{}', o = {}, fs = {}", filter_name, output_number, sampling_frequency),
                &format!("{}/{}{}.png", PLOT_TARGET, file_name_no_extension, output_number),
                core::array::from_fn(|i| (omega[i], freq_response[i]))
            )?
        }
        Ok(())
    }

    #[test]
    fn inv()
    {
        const N: usize = 5;
        const M: usize = (N + 1) / 2;
        const K: usize = 2usize.pow(M as u32);
        let inv_map: [[_; M]; K] = core::array::from_fn(|mut i| {
            core::array::from_fn(|_| {
                let b = i % 2 == 1;
                i >>= 1;
                b
            })
        });
        let inv: [[_; N]; K] = core::array::from_fn(|i| core::array::from_fn(|n| inv_map[i][N.abs_diff(n * 2 + 1) / 2]));
        println!("inv = {:?}", inv);
    }
}
