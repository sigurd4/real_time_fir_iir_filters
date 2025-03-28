use crate::{calc::iir::third::ThirdOrderSallenKeyCalc, conf::{self, All, BandPass, HighPass, LowPass}, internals::{ainternals, binternals, rtfinternals}, param::{FilterFloat, FilterParam, FirstOrderRCFilterConf, Param, RC3GSallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam}, rtf::RtfBase, static_rtf::StaticRtfBase};

#[allow(type_alias_bounds)]
type BInternals<F, C1: FirstOrderRCFilterConf, C2: SecondOrderSallenKeyFilterConf> = binternals!(
    F,
    <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<C1 as FirstOrderRCFilterConf>::OUTPUTS,
    <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
    1,
    0,
    3
);
#[allow(type_alias_bounds)]
type AInternals<F, C1: FirstOrderRCFilterConf, C2: SecondOrderSallenKeyFilterConf> = ainternals!(
    F,
    <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
    1,
    0,
    3
);
#[allow(type_alias_bounds)]
type Internals<F, C1: FirstOrderRCFilterConf, C2: SecondOrderSallenKeyFilterConf> = rtfinternals!(
    F,
    <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<C1 as FirstOrderRCFilterConf>::OUTPUTS,
    <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
    1,
    0,
    3,
    true
);

/// # Configurations
/// [All](crate::conf::All), [BandPass](crate::conf::BandPass),
/// [LowPass](crate::conf::LowPass), [BandPass](crate::conf::BandPass)<1>, [BandPass](crate::conf::BandPass)<2>, [BandPass](crate::conf::BandPass)<3>, [BandPass](crate::conf::BandPass)<4>, [BandPass](crate::conf::BandPass)<5>, [BandPass](crate::conf::BandPass)<6>, [HighPass](crate::conf::HighPass)
/// ```#md
/// 0) LOW-PASS:
///                   o------------o
///                   |            |
///                  [C2]          |
///                   |            |
///     X-[R1]-o-[R2]-o-[R3]-o-[G>-Y
///            |             |
///           [C1]          [C3]
///            |             |
///           GND           GND
/// 1) BAND-PASS 1:
///                   o------------o
///                   |            |
///                  [C2]          |
///                   |            |
///     X-[C1]-o-[R2]-o-[R3]-o-[G>-Y
///            |             |
///           [R1]          [C3]
///            |             |
///           GND           GND
/// 2) BAND-PASS 2:
///                   o------------o
///                   |            |
///                  [R2]          |
///                   |            |
///     X-[R1]-o-[C2]-o-[R3]-o-[G>-Y
///            |             |
///           [C1]          [C3]
///            |             |
///           GND           GND
/// 3) BAND-PASS 3:
///                   o------------o
///                   |            |
///                  [R2]          |
///                   |            |
///     X-[C1]-o-[C2]-o-[R3]-o-[G>-Y
///            |             |
///           [R1]          [C3]
///            |             |
///           GND           GND
/// 4) BAND-PASS 4:
///                   o------------o
///                   |            |
///                  [C2]          |
///                   |            |
///     X-[R1]-o-[R2]-o-[C3]-o-[G>-Y
///            |             |
///           [C1]          [R3]
///            |             |
///           GND           GND
/// 5) BAND-PASS 5:
///                   o------------o
///                   |            |
///                  [C2]          |
///                   |            |
///     X-[C1]-o-[R2]-o-[C3]-o-[G>-Y
///            |             |
///           [R1]          [R3]
///            |             |
///           GND           GND
/// 6) BAND-PASS 6:
///                   o------------o
///                   |            |
///                  [R2]          |
///                   |            |
///     X-[R1]-o-[C2]-o-[C3]-o-[G>-Y
///            |             |
///           [C1]          [R3]
///            |             |
///           GND           GND
/// 7) HIGH-PASS:
///                   o------------o
///                   |            |
///                  [R2]          |
///                   |            |
///     X-[C1]-o-[C2]-o-[C3]-o-[G>-Y
///            |             |
///           [R1]          [R3]
///            |             |
///           GND           GND
/// ```
pub struct ThirdOrderSallenKeyFilter<
    C,
    F,
    P = RC3GSallenKey<F>,
    C1 = <C as ThirdOrderSallenKeyFilterConf>::S1Conf,
    C2 = <C as ThirdOrderSallenKeyFilterConf>::S2Conf
>
where
    F: FilterFloat,
    Param<P>: ThirdOrderSallenKeyFilterParam<C, Conf = C, F = F>,
    C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = C1, S2Conf = C2>,
    C1: FirstOrderRCFilterConf<Conf = C1>,
    C2: SecondOrderSallenKeyFilterConf<Conf = C2>,
    [(); <C1 as FirstOrderRCFilterConf>::OUTPUTS]:,
    [(); <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<C1 as FirstOrderRCFilterConf>::OUTPUTS]:
{
    pub param: Param<P>,
    pub internals: Internals<F, C1, C2>,
    phantom: core::marker::PhantomData<C>
}

impl<P, C, C1, C2> ThirdOrderSallenKeyFilter<C, <Param<P> as FilterParam>::F, P, C1, C2>
where
    Param<P>: ThirdOrderSallenKeyFilterParam<C, Conf = C>,
    C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = C1, S2Conf = C2>,
    C1: FirstOrderRCFilterConf<Conf = C1>,
    C2: SecondOrderSallenKeyFilterConf<Conf = C2>,
    [(); <C1 as FirstOrderRCFilterConf>::OUTPUTS]:,
    [(); <C2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<C1 as FirstOrderRCFilterConf>::OUTPUTS]:
{
    pub const fn new<Conf>(param: P) -> Self
    where
        Param<P>: ThirdOrderSallenKeyFilterParam<Conf, Conf: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = C1, S2Conf = C2>>,
        Conf: conf::Conf
    {
        Self {
            param: Param::new(param),
            internals: Internals::new(),
            phantom: core::marker::PhantomData
        }
    }
}

macro_rules! c {
    (
        $(
            fn make_coeffs<$conf1:ty, $conf2:ty>($arg_param:ident, $arg_rate:ident) -> _
            $(where
                {$($where_c:tt)+})?
            $make_coeffs:block
        )*
    ) => {
        $(
            impl<P, C> RtfBase for ThirdOrderSallenKeyFilter<C, <Param<P> as FilterParam>::F, P, $conf1, $conf2>
            where
                Param<P>: ThirdOrderSallenKeyFilterParam<C, Conf = C>,
                C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = $conf1, S2Conf = $conf2>,
                $($($where_c)+)?
            {
                type Conf = C;
                type F = <Param<P> as FilterParam>::F;
            
                const IS_IIR: bool = true;
                const OUTPUTS: usize = <$conf2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<$conf1 as FirstOrderRCFilterConf>::OUTPUTS;
            }
            impl<P, C> StaticRtfBase for ThirdOrderSallenKeyFilter<C, <Param<P> as FilterParam>::F, P, $conf1, $conf2>
            where
                Param<P>: ThirdOrderSallenKeyFilterParam<C, Conf = C>,
                C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = $conf1, S2Conf = $conf2>,
                $($($where_c)+)?
            {
                type Param = P;

                const O_BUFFERS: usize = <$conf2 as SecondOrderSallenKeyFilterConf>::OUTPUTS;
                const SOS_BUFFERS: usize = 1;
                const SOS_STAGES: usize = 0;
                const ORDER: usize = 3;
                
                fn from_param(param: Self::Param) -> Self
                {
                    Self {
                        param: Param::new(param),
                        internals: Internals::new(),
                        phantom: core::marker::PhantomData
                    }
                }
                fn get_param(&self) -> &Self::Param
                {
                    &*self.param
                }
                fn get_param_mut(&mut self) -> &mut Self::Param
                {
                    &mut *self.param
                }
                fn into_param(self) -> Self::Param
                {
                    self.param.into_value()
                }
                
                fn get_internals(&self) -> (&Internals<<Param<P> as FilterParam>::F, $conf1, $conf2>, &Param<Self::Param>)
                {
                    (&self.internals, &self.param)
                }
                fn get_internals_mut(&mut self) -> (&mut Internals<<Param<P> as FilterParam>::F, $conf1, $conf2>, &mut Param<Self::Param>)
                {
                    (&mut self.internals, &mut self.param)
                }

                fn make_coeffs($arg_param: &Param<Self::Param>, $arg_rate: Self::F) -> (
                    BInternals<<Param<P> as FilterParam>::F, $conf1, $conf2>,
                    [AInternals<<Param<P> as FilterParam>::F, $conf1, $conf2>; true as usize]
                )
                {
                    fn make_coeffs<F, P, C>($arg_param: &Param<P>, $arg_rate: F) -> (
                        BInternals<F, $conf1, $conf2>,
                        [AInternals<F, $conf1, $conf2>; true as usize]
                    )
                    where
                        F: FilterFloat,
                        Param<P>: ThirdOrderSallenKeyFilterParam<C, Conf = C, F = F>,
                        C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = $conf1, S2Conf = $conf2>,
                        $($($where_c)+)?
                    $make_coeffs

                    make_coeffs($arg_param, $arg_rate)
                }
            }
        )*
    };
}

c!(
    fn make_coeffs<All, All>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_band2(),
                calc.b_high_band2(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, All>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band1(),
                calc.b_low_band2(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, All>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band1(),
                calc.b_high_band2(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }

    fn make_coeffs<All, LowPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low()
            ]),
            [([], [
                calc.a_low()
            ])]
        )
    }
    fn make_coeffs<LowPass, LowPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low()
            ]),
            [([], [
                calc.a_low()
            ])]
        )
    }
    fn make_coeffs<HighPass, LowPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low()
            ]),
            [([], [
                calc.a_low()
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass<1>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_high_band1()
            ]),
            [([], [
                calc.a_band1()
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass<1>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1()
            ]),
            [([], [
                calc.a_band1()
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass<1>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band1()
            ]),
            [([], [
                calc.a_band1()
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass<2>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band2(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass<2>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band2()
            ]),
            [([], [
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass<2>>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_band2()
            ])]
        )
    }
    
    fn make_coeffs<All, HighPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, HighPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_high()
            ]),
            [([], [
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, HighPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_high()
            ]),
            [([], [
                calc.a_high()
            ])]
        )
    }

    // Essentials^

    fn make_coeffs<All, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band1(),
                calc.b_high_band1()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band1()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band1()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1()
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band2(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2()
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_high()
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_band2(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_low_band2()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band1(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band1(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band2(),
                calc.b_high_band2(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band2(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band2(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_band2(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band1(),
                calc.b_low_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band1(),
                calc.b_high_band2()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_band2()
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band1(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band1(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band1(),
                calc.a_high()
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_high_low(),
                calc.b_low_band2(),
                calc.b_high_band2(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_low(),
                calc.b_low_band2(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_low(),
                calc.b_high_band2(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_low(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_high_band1(),
                calc.b_low_band2(),
                calc.b_high_band2(),
                calc.b_low_high(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_low_band1(),
                calc.b_low_band2(),
                calc.b_low_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass, HighPass)>(param, rate) -> _
    {
        let calc = ThirdOrderSallenKeyCalc::new(param.rc3g(), rate);
        (
            ([], [], [
                calc.b_high_band1(),
                calc.b_high_band2(),
                calc.b_high_high()
            ]),
            [([], [
                calc.a_band1(),
                calc.a_band2(),
                calc.a_high()
            ])]
        )
    }
);

#[cfg(test)]
mod test
{
    use crate::{conf::All, param::RC3GSallenKey};

    use super::ThirdOrderSallenKeyFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC3GSallenKey {r1: 470.0, c1: 47.0e-9, r2: 15.0e3, c2: 2.7e-9, r3: 16.0e3, c3: 2.7e-9, g: 1.3846153846153846});
        //let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC2GSallenKey {r1: 15.0e3, c1: 2.7e-9, r2: 15.0e3, c2: 2.7e-9, g: 2.0});
        //let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC {r: 470.0, c: 47.0e-9});

        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}