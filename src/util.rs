#![allow(unused)]

pub(crate) mod jacobi_elliptic_functions
{
    use core::mem::MaybeUninit;

    use num::Float;

    use crate::f;

    // sn cn dn
    pub fn elljac_e<F>(u: F, m: F) -> (F, F, F)
    where
        F: Float
    {
        let m_abs = m.abs();
        let one = F::one();

        assert!(m_abs <= one);

        let eps = F::epsilon();
        let two_eps = f!(2.0)*eps;

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

        let four_eps = f!(4.0)*eps;
        let half = f!(0.5);
    
        while (mu[n] - nu[n]).abs() > four_eps*(mu[n] + nu[n]).abs()
        {
            mu[n + 1] = half*(mu[n] + nu[n]);
            nu[n + 1] = (mu[n] * nu[n]).sqrt();
            n += 1;
            
            if n >= N - 1
            {
                break
            }
        }
    
        let sin_umu = (u * mu[n]).sin();
        let cos_umu = (u * mu[n]).cos();
        
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