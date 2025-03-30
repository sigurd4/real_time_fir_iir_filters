use crate::{conf::Conf, param::{FirstOrderAllPassFilterConf, FirstOrderAllPassFilterParamBase, Tau}, util::same::Same};

pub trait FirstOrderAllPassFilterParam<
    C,
    ImplBase = <Self as FirstOrderAllPassFilterParamBase<C>>::ImplBase
>: FirstOrderAllPassFilterParamBase<C, ImplBase: Same<ImplBase>>
where
    C: Conf
{
    type Conf: FirstOrderAllPassFilterConf;

    fn tau(&self) -> Tau<Self::F>;
}