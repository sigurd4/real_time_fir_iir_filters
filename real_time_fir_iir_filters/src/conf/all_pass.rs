use super::{wildcard_if_zero, Conf, ConfType};

#[derive_const(Clone, Copy, Debug)]
pub enum AllPass<const N: usize = 0> {}

impl<const N: usize> Conf for AllPass<N>
{
    const CONF_TYPE: ConfType = wildcard_if_zero(N);

    type Wildcard = AllPass;
}