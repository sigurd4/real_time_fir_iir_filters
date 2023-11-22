use num::Float;

use super::*;

pub struct ImpulseFilter<F, const N: usize>
where
    F: Float,
    [(); N + 1]:
{
    pub b: [F; N + 1],
    pub w: [F; N]
}

impl<F, const N: usize> ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    pub fn new_from_slice(impulse: &[F]) -> Self
    {
        let mut array = [F::zero(); N + 1];
        array[..impulse.len()].copy_from_slice(impulse);

        Self::new(array)
    }
    pub fn new(impulse: [F; N + 1]) -> Self
    {
        Self
        {
            b: impulse,
            w: [F::zero(); N]
        }
    }
}

impl<F, const N: usize> FilterAny<F> for ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    const KIND: FilterKind = FilterKind::FIR;
    const OUTPUTS: usize = 1;
}

impl<F, const N: usize> FilterStaticOrder<F> for ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    const ORDER: usize = N;
}

impl<F, const N: usize> FilterStaticStages<F> for ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    const EXTRA_STAGES: usize = 0;
}

impl<F, const N: usize> FilterStaticCoefficients<F> for ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    type CoefficientsB = [F; N + 1];
    type CoefficientsA = [F; N + 1];

    fn b(&self, _rate: F) -> Self::CoefficientsB
    {
        self.b
    }
}

impl<F, const N: usize> FilterStaticInternals<F> for ImpulseFilter<F, N>
where
    F: Float,
    [(); N + 1]:
{
    type InternalsW = [F; N];

    fn w(&mut self) -> &mut Self::InternalsW
    {
        &mut self.w
    }
}