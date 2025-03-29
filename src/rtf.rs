use core::{iter::Sum, mem::MaybeUninit, ops::Add};
use std::ops::MulAssign;

use num::{Complex, Float};
use crate::{conf, internals::{ainternals, binternals, rtfinternals, winternals}, max_len, param::{FilterFloat, Param}, static_rtf::StaticRtf};

pub trait RtfBase: Sized
{
    type Conf: conf::Conf;
    type F: FilterFloat;

    const OUTPUTS: usize;
    const IS_IIR: bool;
}

pub trait Rtf: RtfBase
{
    /// Feeds a single sample through the filter, and returns the results from each output in an array.
    /// 
    /// # Example
    /// 
    /// In this example, we compute the impulse response of a 1. order filter.
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use core::f64::consts::TAU;
    /// 
    /// use real_time_fir_iir_filters::{
    ///     conf::LowPass,
    ///     param::Omega,
    ///     rtf::Rtf,
    ///     filters::iir::first::FirstOrderFilter
    /// };
    /// 
    /// // Initialize a 1. order low-pass filter at 440Hz
    /// let mut filter = FirstOrderFilter::new::<LowPass>(
    ///     Omega {
    ///         omega: 440.0*TAU
    ///     }
    /// );
    /// 
    /// const N: usize = 10;
    /// const RATE: f64 = 8000.0;
    /// 
    /// // Unit impulse
    /// let mut imp_resp = [0.0; N];
    /// imp_resp[0] = 1.0;
    /// 
    /// // Apply filter to imp_resp
    /// for x in &mut imp_resp
    /// {
    ///     [*x] = filter.filter(RATE, *x);
    /// }
    /// 
    /// // Prints the impulse response of the filter.
    /// println!("h[n] = {:?}", imp_resp);
    /// ```
    fn filter(&mut self, rate: Self::F, x: Self::F) -> [Self::F; Self::OUTPUTS];

    /// Returns the response of the filter for a single frequency point, in radians.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use core::f64::consts::{TAU, FRAC_1_SQRT_2};
    /// 
    /// use real_time_fir_iir_filters::{
    ///     conf::LowPass,
    ///     param::Omega,
    ///     rtf::Rtf,
    ///     filters::iir::first::FirstOrderFilter
    /// };
    /// 
    /// let omega = 440.0*TAU;
    /// 
    /// // Initialize a 1. order low-pass filter at 440Hz
    /// let mut filter = FirstOrderFilter::new::<LowPass>(
    ///     Omega {
    ///         omega
    ///     }
    /// );
    /// 
    /// const RATE: f64 = 8000.0;
    /// 
    /// let [h] = filter.frequency_response(RATE, omega/RATE);
    /// 
    /// assert!((h.norm() - FRAC_1_SQRT_2).abs() < 1e-2);
    /// ```
    fn frequency_response(&mut self, rate: Self::F, omega: Self::F) -> [Complex<Self::F>; Self::OUTPUTS]
    {
        self.z_response(rate, Complex::cis(omega))
    }

    /// Returns the response of the filter for a single s-plane point.
    fn s_response(&mut self, rate: Self::F, s: Complex<Self::F>) -> [Complex<Self::F>; Self::OUTPUTS]
    {
        self.z_response(rate, (s/rate).exp())
    }

    /// Returns the response of the filter for a single z-plane point.
    fn z_response(&mut self, rate: Self::F, z: Complex<Self::F>) -> [Complex<Self::F>; Self::OUTPUTS];

    /// Resets all internal state of the filter back to zero, but keeps the filter coefficient cache intact.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// #![feature(generic_const_exprs)]
    /// 
    /// use core::f64::consts::TAU;
    /// 
    /// use real_time_fir_iir_filters::{
    ///     conf::LowPass,
    ///     param::Omega,
    ///     rtf::Rtf,
    ///     filters::iir::first::FirstOrderFilter
    /// };
    /// 
    /// // Initialize a 1. order low-pass filter at 440Hz
    /// let mut filter = FirstOrderFilter::new::<LowPass>(
    ///     Omega {
    ///         omega: 440.0*TAU
    ///     }
    /// );
    /// 
    /// const N: usize = 10;
    /// const RATE: f64 = 8000.0;
    /// 
    /// let mut imp1 = [0.0; N];
    /// imp1[0] = 1.0;
    /// 
    /// let mut imp2 = imp1;
    /// 
    /// // Apply filter to imp1
    /// for x in &mut imp1
    /// {
    ///     [*x] = filter.filter(RATE, *x);
    /// }
    /// 
    /// // Reset the filter's internal state
    /// filter.reset();
    /// 
    /// // Apply filter to imp2
    /// for x in &mut imp2
    /// {
    ///     [*x] = filter.filter(RATE, *x);
    /// }
    /// 
    /// assert_eq!(imp1, imp2);
    /// ```
    fn reset(&mut self);
}

impl<F, T> Rtf for T
where
    F: FilterFloat + Sum,
    Complex<F>: MulAssign,
    T: StaticRtf<F = F>,
    [(); crate::max_len(Self::ORDER + 1, 3)]:,
    [(); Self::ORDER + 1]:,
    [(); Self::IS_IIR as usize]:,
    [(); (Self::SOS_STAGES >= 1) as usize]:,
    [(); Self::SOS_STAGES*(Self::SOS_STAGES >= 1) as usize - (Self::SOS_STAGES >= 1) as usize]:,
    [(); 0 - Self::OUTPUTS % Self::O_BUFFERS]:,
    [(); 0 - Self::O_BUFFERS % Self::SOS_BUFFERS]:,
    [(); 0 - Self::SOS_BUFFERS % Self::SOS_BUFFERS]:
{
    fn filter(&mut self, rate: F, x: F) -> [F; Self::OUTPUTS]
    {
        if Self::OUTPUTS == 0
        {
            return unsafe {MaybeUninit::uninit().assume_init()}
        }

        fn filter_once_iir<F, const ORDER: usize, const B: usize, const A: usize>(
            y: &mut [F],
            w: &mut [[F; ORDER]; A],
            b: &[[F; ORDER + 1]; B], // B = A*CHUNKS
            a: &[[F; ORDER + 1]; A]
        )
        where
            F: Float + Sum,
            [(); 0 - B % A]:
        {
            assert!(y.len() >= B);

            if A <= 1
            {
                if let Some(((w, a), x)) = w.first_mut()
                    .zip(a.first())
                    .zip(y.first()
                        .copied()
                    )
                {
                    let mut a = a.iter()
                        .copied();
                    let a0 = a.next().unwrap();

                    let w0 = x - w.iter()
                        .copied()
                        .zip(a)
                        .map(|(w, a)| w*a)
                        .sum::<F>()/a0;

                    for (b, y) in b.iter()
                        .zip(y.iter_mut())
                    {
                        *y = core::iter::once(w0)
                            .chain(w.iter()
                                .copied()
                            ).zip(b.iter()
                                .copied()
                            ).map(|(w, b)| w*b)
                            .reduce(Add::add)
                            .unwrap()/a0;
                    }
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
            else if B/A <= 1
            {
                for (((b, w), a), y) in b.iter()
                    .zip(w.iter_mut())
                    .zip(a.iter())
                    .zip(y.iter_mut())
                {
                    let mut a = a.iter()
                        .copied();
                    let a0 = a.next().unwrap();

                    let w0 = *y - w.iter()
                        .copied()
                        .zip(a)
                        .map(|(w, a)| w*a)
                        .sum::<F>()/a0;

                    *y = core::iter::once(w0)
                        .chain(w.iter()
                            .copied()
                        ).zip(b.iter()
                            .copied()
                        ).map(|(w, b)| w*b)
                        .reduce(Add::add)
                        .unwrap()/a0;
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
            else
            {
                let mut j = A;
                let mut i = B;
                for ((b, w), a) in b.chunks_exact(B/A)
                    .rev()
                    .zip(w.iter_mut()
                        .rev()
                    ).zip(a.iter()
                        .rev()
                    )
                {
                    let mut a = a.iter()
                        .copied();
                    let a0 = a.next().unwrap();

                    j -= 1;
                    let w0 = y[j] - w.iter()
                        .copied()
                        .zip(a)
                        .map(|(w, a)| w*a)
                        .sum::<F>()/a0;

                    for b in b.iter()
                        .rev()
                    {
                        i -= 1;
                        y[i] = core::iter::once(w0)
                            .chain(w.iter()
                                .copied()
                            ).zip(b.iter()
                                .copied()
                            ).map(|(w, b)| w*b)
                            .reduce(Add::add)
                            .unwrap()/a0;
                    }
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
        }
        
        fn filter_once_fir<F, const ORDER: usize, const B: usize, const A: usize>(
            y: &mut [F],
            w: &mut [[F; ORDER]; A],
            b: &[[F; ORDER + 1]; B]
        )
        where
            F: Float + Sum,
            [(); 0 - B % A]:
        {
            assert!(y.len() >= B);

            if A <= 1
            {
                if let Some((w, w0)) = w.first_mut()
                    .zip(y.first()
                        .copied()
                    )
                {
                    for (b, y) in b.iter()
                        .zip(y.iter_mut())
                    {
                        *y = core::iter::once(w0)
                            .chain(w.iter()
                                .copied()
                            ).zip(b.iter()
                                .copied()
                            ).map(|(w, b)| w*b)
                            .reduce(Add::add)
                            .unwrap();
                    }
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
            else if B/A <= 1
            {
                for ((b, w), y) in b.iter()
                    .zip(w.iter_mut())
                    .zip(y.iter_mut())
                {
                    let w0 = *y;

                    *y = core::iter::once(w0)
                        .chain(w.iter()
                            .copied()
                        ).zip(b.iter()
                            .copied()
                        ).map(|(w, b)| w*b)
                        .reduce(Add::add)
                        .unwrap();
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
            else
            {
                let mut j = A;
                let mut i = B;
                for (b, w) in b.chunks(B/A)
                    .rev()
                    .zip(w.iter_mut()
                        .rev()
                    )
                {
                    j -= 1;
                    let w0 = y[j];

                    for b in b.iter()
                        .rev()
                    {
                        i -= 1;
                        y[i] = core::iter::once(w0)
                            .chain(w.iter()
                                .copied()
                            ).zip(b.iter()
                                .copied()
                            ).map(|(w, b)| w*b)
                            .reduce(Add::add)
                            .unwrap();
                    }
                    
                    w.rotate_right(1);
                    if let Some(w_first) = w.first_mut()
                    {
                        *w_first = w0;
                    }
                }
            }
        }
        
        self.update_internals(rate);

        let (internals, _): (&mut rtfinternals!(Self), &mut Param<T::Param>) = self.get_internals_mut();
        let (w, b, a): (&mut winternals!(Self), &binternals!(Self), &[ainternals!(Self); Self::IS_IIR as usize])
            = (&mut internals.w, &internals.b, &internals.a);
        let (w_stages, w_output) = w;
        let (w_stages, w_last_stage) = w_stages.split_at_mut(Self::SOS_STAGES.saturating_sub(1));
        let (b_stages, b_last_stage, b_output) = b;
        
        let mut y = [x; Self::OUTPUTS];

        if let Some((a_stages, a_output)) = a.first()
        {
            let (a_stages, a_last_stage) = a_stages.split_at(Self::SOS_STAGES.saturating_sub(1));
            for ((w_stage, b_stage), a_stage) in w_stages.iter_mut()
                .zip(b_stages.iter())
                .zip(a_stages.iter())
            {
                filter_once_iir::<F, 2, {Self::SOS_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut y,
                    w_stage,
                    b_stage,
                    a_stage
                )
            }
            if let Some(((w_stage, b_stage), a_stage)) = w_last_stage.first_mut()
                .zip(b_last_stage.first())
                .zip(a_last_stage.first())
            {
                filter_once_iir::<F, 2, {Self::O_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut y,
                    w_stage,
                    b_stage,
                    a_stage
                )
            }
            filter_once_iir::<F, {Self::ORDER}, {Self::OUTPUTS}, {Self::O_BUFFERS}>(
                &mut y,
                w_output,
                b_output,
                a_output
            )
        }
        else
        {
            for (w_stage, b_stage) in w_stages.iter_mut()
                .zip(b_stages.iter())
            {
                filter_once_fir::<F, 2, {Self::SOS_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut y,
                    w_stage,
                    b_stage
                )
            }
            if let Some((w_stage, b_stage)) = w_last_stage.first_mut()
                .zip(b_last_stage.first())
            {
                filter_once_fir::<F, 2, {Self::O_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut y,
                    w_stage,
                    b_stage
                )
            }
            filter_once_fir::<F, {Self::ORDER}, {Self::OUTPUTS}, {Self::O_BUFFERS}>(
                &mut y,
                w_output,
                b_output
            )
        }

        y
    }
    
    fn z_response(&mut self, rate: Self::F, z: Complex<Self::F>) -> [Complex<Self::F>; Self::OUTPUTS]
    {
        if Self::OUTPUTS == 0
        {
            return unsafe {MaybeUninit::uninit().assume_init()}
        }
        
        fn z_response_once_iir<F, const ORDER: usize, const B: usize, const A: usize>(
            h: &mut [Complex<F>],
            z_inv_n: &[Complex<F>],
            b: &[[F; ORDER + 1]; B],
            a: &[[F; ORDER + 1]; A]
        )
        where
            F: Float + Sum,
            Complex<F>: MulAssign,
            [(); 0 - B % A]:
        {
            assert!(h.len() >= B);

            if A <= 1
            {
                if let Some((a, h0)) = a.first()
                    .zip(h.first()
                        .copied()
                    )
                {
                    for (b, h) in b.iter()
                        .zip(h.iter_mut())
                    {
                        let (b, a) = z_inv_n.iter()
                            .zip(b.iter())
                            .zip(a.iter())
                            .map(|((&z_inv_n, &b), &a)| (z_inv_n*b, z_inv_n*a))
                            .reduce(|r1, r2| (r1.0 + r2.0, r1.1 + r2.1))
                            .unwrap();
                        *h = h0*(b/a)
                    }
                }
            }
            else if B/A <= 1
            {
                for ((b, a), h) in b.iter()
                    .zip(a.iter())
                    .zip(h.iter_mut())
                {
                    let (b, a) = z_inv_n.iter()
                        .zip(b.iter())
                        .zip(a.iter())
                        .map(|((&z_inv_n, &b), &a)| (z_inv_n*b, z_inv_n*a))
                        .reduce(|r1, r2| (r1.0 + r2.0, r1.1 + r2.1))
                        .unwrap();
                    *h *= b/a
                }
            }
            else
            {
                let mut j = A;
                let mut i = B;
                for (b, a) in b.chunks(B/A)
                    .rev()
                    .zip(a.iter()
                        .rev()
                    )
                {
                    j -= 1;
                    for b in b.iter()
                        .rev()
                    {
                        i -= 1;
                        let (b, a) = z_inv_n.iter()
                            .zip(b.iter())
                            .zip(a.iter())
                            .map(|((&z_inv_n, &b), &a)| (z_inv_n*b, z_inv_n*a))
                            .reduce(|r1, r2| (r1.0 + r2.0, r1.1 + r2.1))
                            .unwrap();
                        h[i] = h[j]*(b/a)
                    }
                }
            }
        }
        
        fn z_response_once_fir<F, const ORDER: usize, const B: usize, const A: usize>(
            h: &mut [Complex<F>],
            z_inv_n: &[Complex<F>],
            b: &[[F; ORDER + 1]; B]
        )
        where
            F: Float + Sum,
            [(); 0 - B % A]:
        {
            assert!(h.len() >= B);

            if A <= 1
            {
                if let Some(h0) = h.first()
                    .copied()
                {
                    for (b, h) in b.iter()
                        .zip(h.iter_mut())
                    {
                        *h = h0*z_inv_n.iter()
                            .zip(b.iter())
                            .map(|(&z_inv_n, &b)| z_inv_n*b)
                            .reduce(Add::add)
                            .unwrap()
                    }
                }
            }
            else if B/A <= 1
            {
                for (b, h) in b.iter()
                    .zip(h.iter_mut())
                {
                    *h = z_inv_n.iter()
                        .zip(b.iter())
                        .map(|(&z_inv_n, &b)| z_inv_n*b)
                        .reduce(Add::add)
                        .unwrap();
                }
            }
            else
            {
                let mut j = A;
                let mut i = B;
                for b in b.chunks(B/A)
                    .rev()
                {
                    j -= 1;
                    for b in b.iter()
                        .rev()
                    {
                        i -= 1;
                        h[i] = h[j]*z_inv_n.iter()
                            .zip(b.iter())
                            .map(|(&z_inv_n, &b)| z_inv_n*b)
                            .reduce(Add::add)
                            .unwrap();
                    }
                }
            }
        }

        fn into_z_inv_n<F, const ORDER_P1: usize>(z: Complex<F>) -> [Complex<F>; ORDER_P1]
        where
            F: Float,
            Complex<F>: MulAssign
        {
            let z_inv_1 = z.inv();
            let mut z_inv = Complex::from(F::one());
            core::array::from_fn(|_| {
                let z_inv_n = z_inv;
                z_inv *= z_inv_1;
                z_inv_n
            })
        }

        self.update_internals(rate);

        let (internals, _) = self.get_internals();
        let (b, a) = (&internals.b, &internals.a);
        let (b_stages, b_last_stage, b_output) = b;
        let a = a.iter().next();

        let z_inv_n_3: Option<&[_; 3]>;
        let z_inv_n_owned: Result<[_; Self::ORDER + 1], [_; max_len(Self::ORDER + 1, 3)]>;
        let z_inv_n: &[_; Self::ORDER + 1];
        match Self::SOS_STAGES == 0
        {
            true => {
                z_inv_n_owned = Ok(into_z_inv_n(z));
                z_inv_n = unsafe {
                    z_inv_n_owned.as_ref().ok().unwrap_unchecked()
                };
                z_inv_n_3 = None;
            },
            false => {
                z_inv_n_owned = Err(into_z_inv_n(z));
                let z_inv_n_max = unsafe {
                    z_inv_n_owned.as_ref().err().unwrap_unchecked()
                };
                z_inv_n = z_inv_n_max.split_array_ref().0;
                z_inv_n_3 = Some(z_inv_n_max.split_array_ref().0);
            }
        };

        let mut h = [Complex::from(F::one()); Self::OUTPUTS];
        if let Some((a_stages, a_output)) = &a
        {
            let (a_stages, a_last_stage) = a_stages.split_at(Self::SOS_STAGES.saturating_sub(1));
            for (b_stage, a_stage) in b_stages.iter()
                .zip(a_stages.iter())
            {
                z_response_once_iir::<F, 2, {Self::SOS_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut h,
                    z_inv_n_3.unwrap(),
                    b_stage,
                    a_stage
                )
            }
            if let Some((b_stage, a_stage)) = b_last_stage.first()
                .zip(a_last_stage.first())
            {
                z_response_once_iir::<F, 2, {Self::O_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut h,
                    z_inv_n_3.unwrap(),
                    b_stage,
                    a_stage
                )
            }
            z_response_once_iir::<F, {Self::ORDER}, {Self::OUTPUTS}, {Self::O_BUFFERS}>(
                &mut h,
                z_inv_n,
                b_output,
                a_output
            )
        }
        else
        {
            for b_stage in b_stages.iter()
            {
                z_response_once_fir::<F, 2, {Self::SOS_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut h,
                    z_inv_n_3.unwrap(),
                    b_stage
                )
            }
            if let Some(b_stage) = b_last_stage.first()
            {
                z_response_once_fir::<F, 2, {Self::O_BUFFERS}, {Self::SOS_BUFFERS}>(
                    &mut h,
                    z_inv_n_3.unwrap(),
                    b_stage
                )
            }
            z_response_once_fir::<F, {Self::ORDER}, {Self::OUTPUTS}, {Self::O_BUFFERS}>(
                &mut h,
                z_inv_n,
                b_output
            )
        }
        h
    }
    
    fn reset(&mut self)
    {
        let w = &mut self.get_internals_mut().0.w;

        unsafe {
            core::ptr::write_bytes(w as *mut winternals!(Self), 0u8, 1)
        }
    }
}

/*#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use crate::{iir::second::{OmegaZeta, SecondOrderFilter}, rtf::Rtf};

    #[test]
    fn test()
    {
        const N: usize = 4;
        const RATE: f64 = 8000.0;

        let mut filt = SecondOrderFilter::new(OmegaZeta::new(220.0*TAU, 0.1));

        for _ in 0..N
        {
            println!("{:?}", filt.internals.w);
            filt.filter(RATE, 1.0);
        }

        println!("Done!");
    }
}*/