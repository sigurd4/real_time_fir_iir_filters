use crate::{conf::Conf, param::FilterParam};

pub trait FirstOrderFilterParamBase<C>: FilterParam
where
    C: Conf
{
    /// If in doubt, set this to [Self]
    type ImplBase;
}