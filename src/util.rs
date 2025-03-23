#![allow(unused)]

use num::Float;

use crate::f;

pub mod same
{
    mod private
    {
        pub trait _MaybeSame<T>
        where
            T: ?Sized
        {
            const IS_SAME: bool;

            fn or_else_if_same<F>(self, eval: F) -> Self
            where
                Self: Sized,
                T: Sized,
                F: FnOnce() -> T;
        }
        impl<T, U> _MaybeSame<T> for U
        where
            T: ?Sized,
            U: ?Sized
        {
            default const IS_SAME: bool = false;

            default fn or_else_if_same<F>(self, eval: F) -> Self
            where
                Self: Sized,
                T: Sized,
                F: FnOnce() -> T
            {
                self
            }
        }
        impl<T> _MaybeSame<T> for T
        where
            T: ?Sized
        {
            const IS_SAME: bool = true;

            fn or_else_if_same<F>(self, eval: F) -> Self
            where
                Self: Sized,
                T: Sized,
                F: FnOnce() -> T
            {
                eval()
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
    
        pub trait _Same<T>: Into<T> + From<T>
        {
    
        }
        impl<T> _Same<T> for T
        where
            T: _MaybeSame<T, IS_SAME = true>
        {
    
        }
    }

    pub trait Same<T>: private::_Same<T>
    {

    }
    impl<T, U> Same<T> for U
    where
        U: private::_Same<T>
    {

    }
    
    pub trait NotSame<T>: private::_NotSame<T>
    {

    }
    impl<T, U> NotSame<T> for U
    where
        U: private::_NotSame<T>
    {
        
    }

    pub fn eval_if_same<T, U, F>(eval: F, otherwise: T) -> T
    where
        F: FnOnce() -> U
    {
        use private::_MaybeSame;

        _MaybeSame::<U>::or_else_if_same(otherwise, eval)
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
        let mut mu: [F; N] = unsafe {MaybeUninit::uninit().assume_init()};
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