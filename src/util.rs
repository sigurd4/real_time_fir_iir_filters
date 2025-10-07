#![allow(unused)]

use array_trait::{Array, AsArray, AsSlice};
use num::Float;

use crate::{f, max_len};

pub(crate) trait MaybeNeq
{
    fn maybe_neq(&self, rhs: &Self) -> bool;
}
impl<T> MaybeNeq for T
{
    default fn maybe_neq(&self, rhs: &Self) -> bool
    {
        true
    }
}
impl<T> MaybeNeq for T
where
    T: PartialEq
{
    fn maybe_neq(&self, rhs: &Self) -> bool
    {
        self != rhs
    }
}

mod private
{
    pub trait SizedAt<const SIZE: usize>: Sized
    {
        
    }
    impl<T> SizedAt<{core::mem::size_of::<T>()}> for T
    {
    
    }
}
pub trait SizedAt<const SIZE: usize>: private::SizedAt<SIZE>
{
    
}
impl<T> SizedAt<{core::mem::size_of::<T>()}> for T
{

}

pub macro array_sum {
    ($a:ty$(,)?) => {
        $a
    },
    ($a0:ty, $a1:ty$(,)?) => {
        <$a0 as ArrayPlus<$a1>>::Plus
    },
    ($a0:ty, $a1:ty, $($a:ty),+$(,)?) => {
        array_sum!(array_sum!($a0, $a1), $($a),*)
    }
}

pub trait ArrayPlus<Rhs>: Array
where
    Rhs: Array<Elem = <Self as AsSlice>::Elem>
{
    type Plus: Array<Elem = Self::Elem>;
}
impl<T, const N: usize, const M: usize> ArrayPlus<[T; M]> for [T; N]
{
    default type Plus = [T; N];
}
impl<T, const N: usize, const M: usize> ArrayPlus<[T; M]> for [T; N]
where
    [(); N + M]:
{
    type Plus = [T; N + M];
}

pub trait ArrayPlus1: Array
{
    type Plus1: Array<Elem = Self::Elem>;
}
impl<T, const N: usize> ArrayPlus1 for [T; N]
{
    default type Plus1 = Self;
}
impl<T, const N: usize> ArrayPlus1 for [T; N]
where
    [(); N.saturating_add(1)]:
{
    type Plus1 = [T; N.saturating_add(1)];
}

pub trait ArrayMinus1: Array
{
    type Minus1: Array<Elem = Self::Elem>;
}
impl<T, const N: usize> ArrayMinus1 for [T; N]
{
    default type Minus1 = Self;
}
impl<T, const N: usize> ArrayMinus1 for [T; N]
where
    [(); N.saturating_sub(1)]:
{
    type Minus1 = [T; N.saturating_sub(1)];
}

pub trait ArrayMin1: Array
{
    type Min1: BoolArray<Elem = Self::Elem>;
}
impl<T, const N: usize> ArrayMin1 for [T; N]
{
    default type Min1 = [T; 1];
}
impl<T> ArrayMin1 for [T; 0]
{
    type Min1 = [T; 0];
}

pub trait ArrayMax<Rhs>: Array
where
    Rhs: Array<Elem = <Self as AsSlice>::Elem>
{
    type Max: Array<Elem = Self::Elem>;
}
impl<T, const N: usize, const M: usize> ArrayMax<[T; M]> for [T; N]
{
    default type Max = [T; N];
}
impl<T, const N: usize, const M: usize> ArrayMax<[T; M]> for [T; N]
where
    [(); max_len(N, M)]:
{
    type Max = [T; max_len(N, M)];
}

pub trait EmptyArray: Array
{

}
impl<T> EmptyArray for [T; 0] {}

pub trait BoolArray: Array
{

}
impl<T> BoolArray for [T; 0] {}
impl<T> BoolArray for [T; 1] {}

pub trait ArrayChunks<Chunk>: Array
where
    Chunk: Array<Elem = <Self as AsSlice>::Elem>
{
    type Chunks: Array<Elem = Chunk>;
    type Rem: Array<Elem = Self::Elem>;
}
impl<T, const N: usize, const M: usize> ArrayChunks<[T; N]> for [T; M]
{
    default type Chunks = [[T; N]; 0];
    default type Rem = [T; M];
}
impl<T, const N: usize, const M: usize> ArrayChunks<[T; N]> for [T; M]
where
    [(); M / N]:,
    [(); M % N]:
{
    type Chunks = [[T; N]; M / N];
    type Rem = [T; M % N];
}

pub trait ObviousArray = ArrayChunks<[<Self as AsSlice>::Elem; 1], Rem = [<Self as AsSlice>::Elem; 0]>
    + ArrayChunks<Self, Chunks = [Self; 1], Rem = [<Self as AsSlice>::Elem; 0]>;

pub trait ZeroSized = SizedAt<0>;

pub mod same
{
    mod private
    {
        pub trait _MaybeSame<T>
        where
            T: ?Sized
        {
            const IS_SAME: bool;

            fn eval_if_same<F1, F2>(if_same: F1, otherwise: F2) -> Self
            where
                Self: Sized,
                T: Sized,
                F1: FnOnce() -> T,
                F2: FnOnce() -> Self;
        }
        impl<T, U> _MaybeSame<T> for U
        where
            T: ?Sized,
            U: ?Sized
        {
            default const IS_SAME: bool = false;

            default fn eval_if_same<F1, F2>(if_same: F1, otherwise: F2) -> Self
            where
                Self: Sized,
                T: Sized,
                F1: FnOnce() -> T,
                F2: FnOnce() -> Self
            {
                otherwise()
            }
        }
        impl<T> _MaybeSame<T> for T
        where
            T: ?Sized
        {
            const IS_SAME: bool = true;

            fn eval_if_same<F1, F2>(if_same: F1, otherwise: F2) -> Self
            where
                Self: Sized,
                T: Sized,
                F1: FnOnce() -> T,
                F2: FnOnce() -> Self
            {
                if_same()
            }
        }
    
        pub trait _NotSame<T>
        where
            T: ?Sized
        {
    
        }
        impl<T, U> _NotSame<T> for U
        where
            T: ?Sized,
            U: _MaybeSame<T, IS_SAME = false> + ?Sized
        {
    
        }
    
        pub trait _Same<T>
        where
            T: ?Sized
        {
    
        }
        impl<T> _Same<T> for T
        where
            T: ?Sized
        {
    
        }
    }

    pub trait Same<T>: private::_Same<T>
    where
        T: ?Sized
    {

    }
    impl<T> Same<T> for T
    where
        T: ?Sized
    {

    }
    
    pub trait NotSame<T>: private::_NotSame<T>
    where
        T: ?Sized
    {

    }
    impl<T, U> NotSame<T> for U
    where
        T: ?Sized,
        U: private::_NotSame<T> + ?Sized
    {
        
    }

    pub fn eval_if_same<T, U, F1, F2>(if_same: F1, otherwise: F2) -> T
    where
        F1: FnOnce() -> U,
        F2: FnOnce() -> T
    {
        use private::_MaybeSame;

        <T as _MaybeSame<U>>::eval_if_same(if_same, otherwise)
    }
}

pub(crate) mod jacobi_elliptic_functions
{
    use core::mem::MaybeUninit;

    use super::*;

    // sn cn dn
    pub fn elljac_e<F>(u: F, m: F) -> (F, F, F)
    where
        F: Float
    {
        let m_abs = m.abs();
        let one = F::one();
        let two = one + one;
        let half = two.recip();

        assert!(m_abs <= one);

        let eps = F::epsilon();
        let two_eps = eps + eps;

        if m_abs < two_eps
        {
            return (u.sin(), u.cos(), one)
        }
        if (m - one).abs() < two_eps
        {
            let cn = u.cosh().recip();
            return (u.tanh(), cn, cn)
        }

        let zero = F::zero();

        const N: usize = 16;
        #[allow(clippy::uninit_assumed_init)]
        let mut mu: [F; N] = unsafe {MaybeUninit::uninit().assume_init()};
        #[allow(clippy::uninit_assumed_init)]
        let mut nu: [F; N] = unsafe {MaybeUninit::uninit().assume_init()};
        let mut n = 0;
    
        mu[0] = one;
        nu[0] = (one - m).sqrt();

        let four_eps = two_eps + two_eps;
    
        while (mu[n] - nu[n]).abs() > four_eps*(mu[n] + nu[n]).abs()
        {
            mu[n + 1] = (mu[n] + nu[n])*half;
            nu[n + 1] = (mu[n] * nu[n]).sqrt();
            n += 1;
            
            if n >= N - 1
            {
                break
            }
        }
    
        let (sin_umu, cos_umu) = (u*mu[n]).sin_cos();
        
        let mut c = [zero; 16];
        let mut d = [zero; 16];
        let t;
        let mut r;

        let dn;
        let sn;
        let cn;
    
        if sin_umu.abs() < cos_umu.abs()
        {
            t = sin_umu/cos_umu;
            
            c[n] = mu[n]*t;
            d[n] = one;
            
            while n > 0
            {
                n -= 1;
                c[n] = d[n + 1]*c[n + 1];
                r = (c[n + 1]*c[n + 1])/mu[n + 1];
                d[n] = (r + nu[n])/(r + mu[n]);
            }
            
            dn = (one - m).sqrt()/d[n];
            cn = dn.copysign(cos_umu)/one.hypot(c[n]);
            sn = cn*c[n]/(one - m).sqrt();
        }
        else
        {
            t = cos_umu/sin_umu;
            
            c[n] = mu[n]*t;
            d[n] = one;
            
            while n > 0
            {
                n -= 1;
                c[n] = d[n + 1]*c[n + 1];
                r = (c[n + 1]*c[n + 1])/mu[n + 1];
                d[n] = (r + nu[n])/(r + mu[n]);
            }
            
            dn = d[n];
            sn = sin_umu.signum()/one.hypot(c[n]);
            cn = c[n]*sn;
        }
        
        (sn, cn, dn)
    }

    pub fn sn<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, _, _) = elljac_e(u, m);
        sn
    }

    pub fn cn<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, cn, _) = elljac_e(u, m);
        cn
    }

    pub fn dn<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, _, dn) = elljac_e(u, m);
        dn
    }

    pub fn cd<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, cn, dn) = elljac_e(u, m);
        cn/dn
    }

    pub fn sd<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, _, dn) = elljac_e(u, m);
        sn/dn
    }

    pub fn nd<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, _, dn) = elljac_e(u, m);
        dn.recip()
    }

    pub fn dc<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, cn, dn) = elljac_e(u, m);
        dn/cn
    }

    pub fn nc<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (_, cn, _) = elljac_e(u, m);
        cn.recip()
    }

    pub fn sc<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, cn, _) = elljac_e(u, m);
        sn/cn
    }

    pub fn ns<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, _, _) = elljac_e(u, m);
        sn.recip()
    }

    pub fn ds<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, _, dn) = elljac_e(u, m);
        dn/sn
    }

    pub fn cs<F>(u: F, m: F) -> F
    where
        F: Float
    {
        let (sn, cn, _) = elljac_e(u, m);
        cn/sn
    }
}