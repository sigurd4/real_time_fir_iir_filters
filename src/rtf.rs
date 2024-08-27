use core::{iter::Sum, mem::MaybeUninit, ops::{Add, Sub}};
use std::ops::MulAssign;

use slice_ops::SliceOps;
use bytemuck::Pod;
use num::{Complex, Float};
use crate::{internals::{ainternals, binternals, winternals}, max_len, static_rtf::StaticRtf};

pub trait RtfBase: Sized
{
    type F: Float + Pod;

    const OUTPUTS: usize;
    const IS_IIR: bool;
}

pub trait Rtf: RtfBase
{
    fn filter(&mut self, rate: Self::F, x: Self::F) -> [Self::F; Self::OUTPUTS];

    fn frequency_response(&mut self, rate: Self::F, omega: Self::F) -> [Complex<Self::F>; Self::OUTPUTS]
    {
        self.z_response(rate, Complex::cis(omega))
    }

    fn s_response(&mut self, rate: Self::F, s: Complex<Self::F>) -> [Complex<Self::F>; Self::OUTPUTS]
    {
        self.z_response(rate, (s/rate).exp())
    }

    fn z_response(&mut self, rate: Self::F, z: Complex<Self::F>) -> [Complex<Self::F>; Self::OUTPUTS];

    fn reset(&mut self);
}

impl<F, T> Rtf for T
where
    F: Float + Pod + Sum,
    Complex<F>: MulAssign,
    T: StaticRtf<F = F>,
    [(); Self::BUFFERED_OUTPUTS as usize]:,
    [(); max_len(Self::ORDER + 1, 3)]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]:,
    [(); Self::SOS_STAGES]:,
    [(); Self::ORDER]:,
    [(); Self::ORDER + 1]:,
    [(); Self::IS_IIR as usize]:
{
    fn filter(&mut self, rate: F, x: F) -> [F; Self::OUTPUTS]
    {
        if Self::OUTPUTS == 0
        {
            return unsafe {MaybeUninit::uninit().assume_init()}
        }

        fn filter_once_iir<F, const ORDER: usize, const A: bool>(
            y: &mut [F],
            w: &mut [[F; ORDER]],
            b: &[[F; ORDER + 1]],
            a: &[[F; ORDER + 1]]
        )
        where
            F: Float + Sum
        {
            if !A
            {
                let ((x, w), a) = y.iter()
                    .copied()
                    .zip(w.iter_mut())
                    .zip(a.iter())
                    .next()
                    .unwrap();
                let mut a = a.iter()
                    .copied();
                let a0 = a.next().unwrap();
                
                let mut w0 = x - w.iter()
                    .copied()
                    .zip(a)
                    .map(|(w, a)| w*a)
                    .sum::<F>()/a0;

                for (y, b) in y.iter_mut()
                    .zip(b.iter())
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
                
                w.shift_right(&mut w0);
            }
            else
            {
                for (((y, w), b), a) in y.iter_mut()
                    .zip(w.iter_mut())
                    .zip(b.iter())
                    .zip(a.iter())
                {
                    let mut a = a.iter()
                        .copied();
                    let a0 = a.next().unwrap();

                    let mut w0 = *y - w.iter()
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
                    
                    w.shift_right(&mut w0);
                }
            }
        }
        
        fn filter_once_fir<F, const ORDER: usize, const A: bool>(
            y: &mut [F],
            w: &mut [[F; ORDER]],
            b: &[[F; ORDER + 1]]
        )
        where
            F: Float + Sum,
            [(); ORDER + 1]:
        {
            if !A
            {
                let (x, w) = y.iter()
                    .copied()
                    .zip(w.iter_mut())
                    .next()
                    .unwrap();

                for (y, b) in y.iter_mut()
                    .zip(b.iter())
                {
                    *y = core::iter::once(x)
                        .chain(w.iter()
                            .copied()
                        ).zip(b.iter()
                            .copied()
                        ).map(|(w, b)| w*b)
                        .reduce(Add::add)
                        .unwrap();
                    
                    let mut w0 = x;
                    w.shift_right(&mut w0);
                }
            }
            else
            {
                for ((y, w), b) in y.iter_mut()
                    .zip(w.iter_mut())
                    .zip(b.iter())
                {
                    let mut x = *y;
                    
                    *y = core::iter::once(x)
                        .chain(w.iter()
                            .copied()
                        ).zip(b.iter()
                            .copied()
                        ).map(|(w, b)| w*b)
                        .reduce(Add::add)
                        .unwrap();
                    
                    w.shift_right(&mut x);
                }
            }
        }
        
        self.update_internals(rate);

        let (internals, _) = self.get_internals_mut();
        let (w, b, a): (&mut winternals!(Self), &binternals!(Self), &[ainternals!(Self); Self::IS_IIR as usize])
            = (&mut internals.w, &internals.b, &internals.a);
        let (w_stages, w_output) = w;
        let (b_stages, b_output) = b;
        
        let mut y = [x; Self::OUTPUTS];

        if let Some((a_stages, a_output)) = a.iter().next()
        {
            for ((w_stages, b_stages), a_stages) in w_stages.iter_mut()
                .zip(b_stages.iter())
                .zip(a_stages.iter())
            {
                for (((y, w_stage), b_stage), a_stage) in y.iter_mut()
                    .zip(w_stages.iter_mut())
                    .zip(b_stages.iter())
                    .zip(a_stages.iter())
                {
                    filter_once_iir::<F, 2, false>(
                        core::array::from_mut(y),
                        core::array::from_mut(w_stage),
                        core::array::from_ref(b_stage),
                        core::array::from_ref(a_stage)
                    )
                }
            }

            filter_once_iir::<F, {Self::ORDER}, {Self::BUFFERED_OUTPUTS}>(
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
                for ((y, w_stage), b_stage) in y.iter_mut()
                    .zip(w_stage.iter_mut())
                    .zip(b_stage.iter())
                {
                    filter_once_fir::<F, 2, false>(
                        core::array::from_mut(y),
                        core::array::from_mut(w_stage),
                        core::array::from_ref(b_stage)
                    )
                }
            }
    
            filter_once_fir::<F, {Self::ORDER}, {Self::BUFFERED_OUTPUTS}>(
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

        fn z_response_once_iir<F>(
            z_inv_n: &[Complex<F>],
            b: &[F],
            a: &[F]
        ) -> Complex<F>
        where
            F: Float
        {
            let (b, a) = z_inv_n.iter()
                .zip(b.iter())
                .zip(a.iter())
                .map(|((&z_inv_n, &b), &a)| (z_inv_n*b, z_inv_n*a))
                .reduce(|r1, r2| (r1.0 + r2.0, r1.1 + r2.1))
                .unwrap();
            b/a
        }
        
        fn z_response_once_fir<F>(
            z_inv_n: &[Complex<F>],
            b: &[F]
        ) -> Complex<F>
        where
            F: Float
        {
            z_inv_n.iter()
                .zip(b.iter())
                .map(|(&z_inv_n, &b)| z_inv_n*b)
                .reduce(Add::add)
                .unwrap()
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
        let (b_stages, b_output) = b;
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
            for (b_stage, a_stage) in b_stages.iter()
                .zip(a_stages.iter())
            {
                for ((h, b), a) in h.iter_mut()
                    .zip(b_stage.iter())
                    .zip(a_stage.iter())
                {
                    *h *= z_response_once_iir(
                        z_inv_n_3.unwrap(),
                        b,
                        a
                    )
                }
            }
            if !Self::BUFFERED_OUTPUTS
            {
                let h0 = h[0];
                h.fill(h0);

                let a = a_output.iter()
                    .next()
                    .unwrap();

                for (h, b) in h.iter_mut()
                    .zip(b_output.iter())
                {
                    *h *= z_response_once_iir(
                        z_inv_n,
                        b,
                        a
                    )
                }
            }
            else
            {
                for ((h, b), a) in h.iter_mut()
                    .zip(b_output.iter())
                    .zip(a_output.iter())
                {
                    *h *= z_response_once_iir(
                        z_inv_n,
                        b,
                        a
                    )
                }
            }
        }
        else
        {
            for b_stage in b_stages.iter()
            {
                for (h, b) in h.iter_mut()
                    .zip(b_stage.iter())
                {
                    *h *= z_response_once_fir(
                        z_inv_n_3.unwrap(),
                        b
                    )
                }
            }
            
            for (h, b) in h.iter_mut()
                .zip(b_output.iter())
            {
                *h = z_response_once_fir(
                    z_inv_n,
                    b
                )
            }
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

#[cfg(test)]
mod test
{
    use core::f64::consts::TAU;

    use crate::{iir::{first::{FirstOrderFilter, Omega}, second::{OmegaZeta, SecondOrderFilter}}, rtf::Rtf};

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
    }
}