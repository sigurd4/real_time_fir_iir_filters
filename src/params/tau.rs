use crate::*;
use crate::param::*;
use crate::conf::*;
use crate::util::same::*;

crate::def_param!(
    Tau<F> {
        tau: F
    }
    where
        F: FilterFloat
);
impl<F> FilterParam for Tau<F>
where
    F: FilterFloat
{
    const ORDER: usize = 1;

    type F = F;
}
impl<F, C> FirstOrderAllPassFilterParamBase<C> for Tau<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type ImplBase = Self;
}
impl<F, C> FirstOrderAllPassFilterParam<C> for Tau<F>
where
    F: FilterFloat,
    C: FirstOrderAllPassFilterConf
{
    type Conf = C;

    fn tau(&self) -> Self::F
    {
        *self.tau
    }
}
impl<P> From<P> for Tau<P::F>
where
    P: FirstOrderAllPassFilterParam<All> + NotSame<Tau<P::F>>
{
    fn from(value: P) -> Self
    {
        Tau::new(value.tau())
    }
}