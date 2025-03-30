use crate::{conf::Conf, param::{FirstOrderFilterConf, FirstOrderFilterParamBase, OmegaFirstOrder}, util::same::Same};

pub trait FirstOrderFilterParam<
    C,
    ImplBase = <Self as FirstOrderFilterParamBase<C>>::ImplBase
>: FirstOrderFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: FirstOrderFilterConf;

    fn omega(&self) -> OmegaFirstOrder<Self::F>;
}