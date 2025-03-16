use super::{wildcard_if_zero, Conf, ConfType};

#[derive(Clone, Copy, Debug)]
pub enum HighPass<const N: usize = 0> {}

impl<const N: usize> Conf for HighPass<N>
{
    const CONF_TYPE: ConfType = wildcard_if_zero(N);

    type Wildcard = HighPass;
}