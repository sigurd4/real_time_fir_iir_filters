use std::marker::PhantomData;

use array_math::ArrayOps;

use super::*;

pub struct InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F>,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    pub filter: FL,
    pub w: ([[[F; 2]; FL::OUTPUTS]; FL::SOS_STAGES], [[F; FL::ORDER]; FL::OUTPUTS])
}

impl<F, FL> InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F>,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    pub fn new(filter: FL) -> Self
    {
        Self {
            filter,
            w: ([[[F::zero(); _]; _]; _], [[F::zero(); _]; _])
        }
    }
}

impl<F, FL> FilterAny<F> for InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F>,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    const OUTPUTS: usize = FL::OUTPUTS;

    const KIND: FilterKind = FilterKind::IIR;
}

impl<F, FL> FilterStatic<F> for InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F>,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    const BUFFERED_OUTPUTS: bool = true;

    const SOS_STAGES: usize = FL::SOS_STAGES;

    const ORDER: usize = FL::ORDER;
}

impl<F, FL> FilterStaticCoefficients<F> for InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F> + FilterStaticCoefficients<F>,
    [(); FL::OUTPUTS*FL::BUFFERED_OUTPUTS as usize]:,
    [(); FL::ORDER + 1]:,
    [(); FL::OUTPUTS*FL::BUFFERED_OUTPUTS as usize + !FL::BUFFERED_OUTPUTS as usize]:,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    fn b(&self, rate: F) -> ([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS])
    {
        let a = self.filter.a(rate);

        match a
        {
            Some((a_stages, a_outputs)) => {
                (
                    a_stages.map(|a_stages| ArrayOps::fill(|i| a_stages[i % a_stages.len()]))
                        .reformulate_length(),
                    ArrayOps::fill(|i| a_outputs[i % a_outputs.len()]
                        .reformulate_length()
                    )
                )
            },
            None => {
                let one = |i| if i == 0 {F::one()} else {F::zero()};
                ([[ArrayOps::fill(one); _]; _], [ArrayOps::fill(one); _])
            }
        }
    }

    fn a(&self, rate: F) -> Option<([[[F; 3]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], [[F; Self::ORDER + 1]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])>
    {
        let (b_stages, b_outputs) = self.filter.b(rate);
        Some((
            b_stages.map(|b_stages| ArrayOps::fill(|i| b_stages[i % b_stages.len()]))
                .reformulate_length(),
            ArrayOps::fill(|i| b_outputs[i % b_outputs.len()]
                .reformulate_length()
            )
        ))
    }
}

impl<F, FL> FilterStaticInternals<F> for InverseFilter<F, FL>
where
    F: Float,
    FL: Filter<F> + FilterStatic<F> + FilterStaticInternals<F>,
    [(); FL::ORDER]:,
    [(); FL::SOS_STAGES]:,
    [(); FL::OUTPUTS]:
{
    fn w(&mut self) -> ([&mut [[F; 2]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize]; Self::SOS_STAGES], &mut [[F; Self::ORDER]; Self::OUTPUTS*Self::BUFFERED_OUTPUTS as usize + !Self::BUFFERED_OUTPUTS as usize])
    {
        (
            self.w.0.each_mut(),
            &mut self.w.1
        )
    }
}