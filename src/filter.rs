use std::{ops::{MulAssign, AddAssign}, fmt::Debug};

use array_math::{ArrayMath, ArrayOps, SliceOps};
use num::Complex;
use num_identities_const::ZeroConst;

use super::*;

pub trait Filter<F>: FilterAny<F>
where
    F: Float
{
    fn filter(&mut self, rate: F, x: F) -> [F; Self::OUTPUTS];

    fn frequency_response<I: IntoIterator<Item = F>>(&mut self, rate: F, omega: I) -> [Vec<Complex<F>>; Self::OUTPUTS]
    {
        self.z_response(rate, omega.into_iter().map(|omega| Complex::cis(omega)))
    }

    fn s_response<I: IntoIterator<Item = Complex<F>>>(&mut self, rate: F, s: I) -> [Vec<Complex<F>>; Self::OUTPUTS]
    {
        self.z_response(rate, s.into_iter().map(|s| (s/rate).exp()))
    }

    fn z_response<I: IntoIterator<Item = Complex<F>>>(&mut self, rate: F, z: I) -> [Vec<Complex<F>>; Self::OUTPUTS];
}

impl<F, T> Filter<F> for T
where
    F: Float + AddAssign + ZeroConst,
    Complex<F>: MulAssign + AddAssign,
    T: FilterStaticCoefficients<F>
    + FilterStaticInternals<F>
    + FilterAny<F>,
    [(); Self::ORDER + 1]:,
    [(); Self::SOS_STAGES]:,
    [(); Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize]:,
    [F; Self::ORDER + 1 - 1]: Into<[F; Self::ORDER]>
{
    fn filter(&mut self, rate: F, x: F) -> [F; Self::OUTPUTS]
    {
        fn filter_once_iir<F, const ORDER: usize, const OUTPUTS: usize, const A: bool>(
            x: [F; OUTPUTS*A as usize + !A as usize],
            w: &mut [[F; ORDER]; OUTPUTS*A as usize + !A as usize],
            b: [[F; ORDER + 1]; OUTPUTS],
            a: [[F; ORDER + 1]; OUTPUTS*A as usize + !A as usize]
        ) -> [F; OUTPUTS]
        where
            F: Float + AddAssign + ZeroConst,
            [(); ORDER + 1]:,
            [(); OUTPUTS*A as usize + !A as usize]:,
            [F; ORDER + 1 - 1]: Into<[F; ORDER]>
        {
            let aw0: [_; OUTPUTS*A as usize + !A as usize] = x.zip(a.zip(w.each_ref())).map(|(x, (a, &w))| {
                let ([a0], a_cont) = a.split_array::<1>();
                
                (a0, x - w.mul_dot(a_cont.into())/a0)
            });

            w.each_mut()
                .zip(aw0)
                .try_reformulate_length()
                .map(|waw0| {
                    b.zip(waw0)
                        .map(|(b, (w, (a0, w0)))| {
                            let y = (*w)
                                .rchain([w0])
                                .mul_dot(b)/a0;
                            
                            w.shift_right(w0);

                            y
                        })
                })
                .unwrap_or_else(|waw0| unsafe {
                    waw0.try_reformulate_length()
                        .map(|[(w, (a0, w0))]| {
                            let y = b.map(|b| (*w)
                                .rchain([w0])
                                .mul_dot(b)/a0
                            );

                            w.shift_right(w0);

                            y
                        }).unwrap_unchecked()
                })
        }
        
        fn filter_once_fir<F, const ORDER: usize, const OUTPUTS: usize, const A: bool>(
            x: [F; OUTPUTS*A as usize + !A as usize],
            w: &mut [[F; ORDER]; OUTPUTS*A as usize + !A as usize],
            b: [[F; ORDER + 1]; OUTPUTS]
        ) -> [F; OUTPUTS]
        where
            F: Float + AddAssign + ZeroConst,
            [(); ORDER + 1]:
        {
            let y = {
                let x = x.try_reformulate_length()
                    .unwrap_or_else(|x| [unsafe {x.try_into_single_item().unwrap_unchecked()}; _]);
                let w = (*w).try_reformulate_length()
                    .unwrap_or_else(|w| [unsafe {w.try_into_single_item().unwrap_unchecked()}; _]);
                b.zip(w.zip(x))
                    .map(|(b, (w, w0))| w.rchain([w0])
                        .mul_dot(b)
                    )
            };
            for (w, w0) in w.each_mut()
                .zip(x)
            {
                w.shift_right(w0);
            }
            y
        }
        
        self.on_filter_pre(rate);

        let (b_stages, b_output) = self.b(rate);
        let a = self.a(rate);
        
        let (w_stages, w_output) = self.w();
        
        let mut y = [x; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize];

        match a
        {
            Some((a_stages, a_output)) => {
                if let Some(y) = y.try_reformulate_length_mut()
                {
                    for (w_stages, (b_stages, a_stages)) in w_stages.zip(b_stages.zip(a_stages))
                    {
                        for (y, (w_stage, (b_stage, a_stage))) in y.each_mut()
                            .zip(w_stages.each_mut().zip(b_stages.zip(a_stages)))
                        {
                            *y = filter_once_iir::<F, 2, 1, false>([*y], core::array::from_mut(w_stage), [b_stage], [a_stage])
                                .into_single_item()
                        }
                    }
                }

                filter_once_iir(y, w_output, b_output, a_output)
            },
            None => {
                if let Some(y) = y.try_reformulate_length_mut()
                {     
                    for (w_stage, b_stage) in w_stages.zip(b_stages)
                    {
                        for (y, (w_stage, b_stage)) in y.each_mut()
                            .zip(w_stage.each_mut().zip(b_stage))
                        {
                            *y = filter_once_fir::<F, 2, 1, false>([*y], core::array::from_mut(w_stage), [b_stage])
                                .into_single_item()
                        }
                    }
                }
        
                filter_once_fir(y, w_output, b_output)
            }
        }
    }
    
    fn z_response<I: IntoIterator<Item = Complex<F>>>(&mut self, rate: F, z: I) -> [Vec<Complex<F>>; Self::OUTPUTS]
    {
        fn z_response_once_iir<F, const ORDER: usize>(
            z_inv_n: &[[Complex<F>; ORDER + 1]],
            b: [F; ORDER + 1],
            a: [F; ORDER + 1]
        ) -> Vec<Complex<F>>
        where
            F: Float,
            Complex<F>: AddAssign + ZeroConst,
            [(); ORDER + 1]:
        {
            z_inv_n.into_iter()
                .map(|&z_inv_n| z_inv_n.mul_dot(b)/z_inv_n.mul_dot(a))
                .collect()
        }
        
        fn z_response_once_fir<F, const ORDER: usize>(
            z_inv_n: &[[Complex<F>; ORDER + 1]],
            b: [F; ORDER + 1]
        ) -> Vec<Complex<F>>
        where
            F: Float,
            Complex<F>: AddAssign + ZeroConst,
            [(); ORDER + 1]:
        {
            z_inv_n.into_iter()
                .map(|&z_inv_n| z_inv_n.mul_dot(b))
                .collect()
        }

        fn z_inv_n<F, const ORDER_P1: usize, I: IntoIterator<Item = Complex<F>>>(z: I) -> Vec<[Complex<F>; ORDER_P1]>
        where
            F: Float,
            Complex<F>: MulAssign
        {
            z.into_iter()
                .map(|z| {
                    let z_inv_1 = z.inv();
                    let mut z_inv = Complex::from(F::one());
                    ArrayOps::fill(|_| {
                        let z_inv_n = z_inv;
                        z_inv *= z_inv_1;
                        z_inv_n
                    })
                }).collect::<Vec<[Complex<F>; ORDER_P1]>>()
        }

        self.on_filter_pre(rate);

        let z: Vec<Complex<F>> = z.into_iter().collect();
        
        let z_inv_n_3: Vec<[Complex<F>; 3]>
            = z_inv_n(z.clone());
        let z_inv_n: Vec<[Complex<F>; Self::ORDER + 1]>
            = z_inv_n(z);

        let (b_stages, b_output) = self.b(rate);

        match self.a(rate)
        {
            Some((a_stages, a_output)) => {
                let a_output = ArrayOps::fill(|i| a_output[i % a_output.len()]);

                let h_stages: Option<[Vec<Complex<F>>; Self::OUTPUTS]> = None;/*b_stages
                    .zip(a_stages)
                    .into_iter()
                    .filter_map(|(b_stage, a_stage)| b_stage.zip(a_stage)
                            .try_reformulate_length_ref()
                            .map(|&ba_stage| ba_stage
                                .map(|(b_stage, a_stage)| z_response_once_iir::<F, 2, _>(&z_inv_n_3, b_stage, a_stage))
                            )
                        )
                    .reduce(|a, b| a.zip(b)
                        .map(|(a, b)| a.into_iter() // This might be slow
                            .zip(b.into_iter())
                            .map(|(a, b)| a*b)
                            .collect()
                        )
                    );*/
                    
                if let Some(h_stages) = &h_stages
                {
                    b_output.zip(a_output)
                        .zip(h_stages.each_ref())
                        .map(|((b_output, a_output), h_stages)| {
                            let h_output = z_response_once_iir(&z_inv_n, b_output, a_output);
                            h_stages.iter()
                                .map(|&h| h)
                                .zip(h_output.into_iter())
                                .map(|(a, b)| a*b)
                                .collect()
                        })
                }
                else
                {
                    b_output.zip(a_output)
                        .map(|(b_output, a_output)| {
                            z_response_once_iir(&z_inv_n, b_output, a_output)
                        })
                }
            },
            None => {
                let h_stages: Option<[Vec<Complex<F>>; Self::OUTPUTS]> = None;/*b_stages.into_iter()
                    .filter_map(|b_stage| b_stage.try_reformulate_length_ref()
                            .map(|&b_stage| b_stage
                                .map(|b_stage| z_response_once_fir::<F, 2, _>(&z_inv_n_3, b_stage))
                            )
                        )
                    .reduce(|a, b| a.zip(b)
                        .map(|(a, b)| a.into_iter() // This might be slow
                            .zip(b.into_iter())
                            .map(|(a, b)| a*b)
                            .collect()
                        )
                    );*/
                    
                if let Some(h_stages) = h_stages
                {
                    b_output.zip(h_stages)
                        .map(|(b_output, h_stages)| {
                            let h_output = z_response_once_fir(&z_inv_n, b_output);
                            h_stages.into_iter()
                                .zip(h_output.into_iter())
                                .map(|(a, b)| a*b)
                                .collect()
                        })
                }
                else
                {
                    b_output.map(|b_output| {
                            z_response_once_fir(&z_inv_n, b_output)
                        })
                }
            }
        }
    }
}