use super::{wildcard_if_zero, Conf, ConfType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Peak<const N: usize = 0> {}

impl<const N: usize> Conf for Peak<N>
{
    const CONF_TYPE: ConfType = wildcard_if_zero(N);

    type Wildcard = Peak;
}