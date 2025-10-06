use crate::{conf, internals::{ainternals, binternals, rtfinternals}, param::{FilterFloat, Param}, util::{ArrayChunks, ArrayMin1, ArrayMinus1, ArrayPlus1, BoolArray}};

pub trait StaticRtf: Sized
{
    type Param;

    type Conf: conf::Conf;
    type F: FilterFloat;
    
    type Outputs<U>: ArrayChunks<Self::OutputBufs<U>, Elem = U, Rem = [U; 0]>;
    type IsIir<U>: BoolArray<Elem = U>;
    type Order<U>: ArrayPlus1<Elem = U>;
    type OutputBufs<U>: ArrayChunks<Self::SosBufs<U>, Elem = U, Rem = [U; 0]>;
    type SosBufs<U>: ArrayChunks<Self::SosBufs<U>, Elem = U, Rem = [U; 0], Chunks = [Self::SosBufs<U>; 1]>;
    type SosStages<U>: ArrayMin1<Elem = U> + ArrayMinus1<Elem = U>;
    
    fn from_param(param: Self::Param) -> Self;
    fn get_param(&self) -> &Self::Param;
    fn get_param_mut(&mut self) -> &mut Self::Param;
    fn into_param(self) -> Self::Param;
    
    #[allow(clippy::type_complexity)]
    fn get_internals(&self) -> (&rtfinternals!(Self), &Param<Self::Param>);
    #[allow(clippy::type_complexity)]
    fn get_internals_mut(&mut self) -> (&mut rtfinternals!(Self), &mut Param<Self::Param>);
    
    #[allow(clippy::type_complexity)]
    fn make_coeffs(param: &Self::Param, rate: Self::F) -> (
        binternals!(Self),
        Self::IsIir<ainternals!(Self)>
    );

    fn update_internals(&mut self, rate: Self::F)
    {
        crate::internals::update(self, rate)
    }
}