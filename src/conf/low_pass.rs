use super::{wildcard_if_zero, Conf, ConfType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LowPass<const N: usize = 0> {}

impl<const N: usize> Conf for LowPass<N>
{
    const CONF_TYPE: ConfType = wildcard_if_zero(N);

    type Wildcard = LowPass;
}