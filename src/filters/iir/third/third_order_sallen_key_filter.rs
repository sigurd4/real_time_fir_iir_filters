use core::fmt::Debug;

use crate::{calc::iir::third::ThirdOrderSallenKeyCalc, conf::{All, BandPass, HighPass, LowPass}, param::{FilterFloat, FilterParam, FirstOrderRCFilterConf, Param, RC3GSallenKey, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam}, rtf::StaticRtf, util::ArrayMul};

crate::rtfinternals!(
    type Conf: ThirdOrderSallenKeyFilterConf;

    const SOS_BUFS: usize = 1;
    const SOS_STAGES: usize = 0;
    const ORDER: usize = 3;
    const IS_IIR: bool = true;
);

/// # Configurations
/// 
/// [`All`](crate::conf::All), [`BandPass`](crate::conf::BandPass),
/// [`LowPass`](crate::conf::LowPass), <code>[BandPass](crate::conf::BandPass)<1></code>, <code>[BandPass](crate::conf::BandPass)<2></code>, <code>[BandPass](crate::conf::BandPass)<3></code>, <code>[BandPass](crate::conf::BandPass)<4></code>, <code>[BandPass](crate::conf::BandPass)<5></code>, <code>[BandPass](crate::conf::BandPass)<6></code>, [`HighPass`](crate::conf::HighPass)
/// 
/// <pre>
/// 0) LOW-PASS:
///                   o------------o
///                   |            |
///                  [C₂]          |
///                   |            |
///     X-[R₁]-o-[R₂]-o-[R₃]-o-[G>-Y
///            |             |
///           [C₁]          [C₃]
///            |             |
///           GND           GND
/// 1) BAND-PASS 1:
///                   o------------o
///                   |            |
///                  [C₂]          |
///                   |            |
///     X-[C₁]-o-[R₂]-o-[R₃]-o-[G>-Y
///            |             |
///           [R₁]          [C₃]
///            |             |
///           GND           GND
/// 2) BAND-PASS 2:
///                   o------------o
///                   |            |
///                  [R₂]          |
///                   |            |
///     X-[R₁]-o-[C₂]-o-[R₃]-o-[G>-Y
///            |             |
///           [C₁]          [C₃]
///            |             |
///           GND           GND
/// 3) BAND-PASS 3:
///                   o------------o
///                   |            |
///                  [R₂]          |
///                   |            |
///     X-[C₁]-o-[C₂]-o-[R₃]-o-[G>-Y
///            |             |
///           [R₁]          [C₃]
///            |             |
///           GND           GND
/// 4) BAND-PASS 4:
///                   o------------o
///                   |            |
///                  [C₂]          |
///                   |            |
///     X-[R₁]-o-[R₂]-o-[C₃]-o-[G>-Y
///            |             |
///           [C₁]          [R₃]
///            |             |
///           GND           GND
/// 5) BAND-PASS 5:
///                   o------------o
///                   |            |
///                  [C₂]          |
///                   |            |
///     X-[C₁]-o-[R₂]-o-[C₃]-o-[G>-Y
///            |             |
///           [R₁]          [R₃]
///            |             |
///           GND           GND
/// 6) BAND-PASS 6:
///                   o------------o
///                   |            |
///                  [R₂]          |
///                   |            |
///     X-[R₁]-o-[C₂]-o-[C₃]-o-[G>-Y
///            |             |
///           [C₁]          [R₃]
///            |             |
///           GND           GND
/// 7) HIGH-PASS:
///                   o------------o
///                   |            |
///                  [R₂]          |
///                   |            |
///     X-[C₁]-o-[C₂]-o-[C₃]-o-[G>-Y
///            |             |
///           [R₁]          [R₃]
///            |             |
///           GND           GND
/// </pre>
/// 
/// # Frequency response
/// 
/// ## Parameters
/// 
/// R₁ = 470 Ω
/// 
/// C₁ = 47 nF
/// 
/// R₂ = 15 kΩ
/// 
/// C₂ = 2.7 nF
/// 
/// R₃ = 16 kΩ
/// 
/// C₃ = 2.7 nF
/// 
/// G = 1.38
/// 
/// ## Low-pass
/// 
/// <div>
/// <img alt="Third order low-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter0.png" height="500">
/// </div>
/// 
/// ## Band-pass 1
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter1.png" height="500">
/// </div>
/// 
/// ## Band-pass 2
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter2.png" height="500">
/// </div>
/// 
/// ## Band-pass 3
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter3.png" height="500">
/// </div>
/// 
/// ## Band-pass 4
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter4.png" height="500">
/// </div>
/// 
/// ## Band-pass 5
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter5.png" height="500">
/// </div>
/// 
/// ## Band-pass 6
/// 
/// <div>
/// <img alt="Third order band-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter6.png" height="500">
/// </div>
/// 
/// ## High-pass
/// 
/// <div>
/// <img alt="Third order high-pass sallen-key filter response" src="https://raw.githubusercontent.com/sigurd4/real_time_fir_iir_filters/refs/heads/master/plots/third_order_sallen_key_filter7.png" height="500">
/// </div>
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThirdOrderSallenKeyFilter<
    C,
    F = f64,
    P = RC3GSallenKey<F>,
    C1 = <C as ThirdOrderSallenKeyFilterConf>::S1Conf,
    C2 = <C as ThirdOrderSallenKeyFilterConf>::S2Conf
>
where
    F: FilterFloat,
    P: ThirdOrderSallenKeyFilterParam<C, Conf = C, F = F>,
    C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = C1, S2Conf = C2>,
    C1: FirstOrderRCFilterConf<Conf = C1>,
    C2: SecondOrderSallenKeyFilterConf<Conf = C2>,
    Internals<F, C>: Copy + Debug + PartialEq
{
    pub param: Param<P>,
    pub internals: Internals<F, C>,
    #[serde(skip)]
    phantom: core::marker::PhantomData<C>
}

impl<P, C, C1, C2> ThirdOrderSallenKeyFilter<C, <P as FilterParam>::F, P, C1, C2>
where
    P: ThirdOrderSallenKeyFilterParam<C, Conf = C>,
    C: ThirdOrderSallenKeyFilterConf<Conf = C, S1Conf = C1, S2Conf = C2>,
    C1: FirstOrderRCFilterConf<Conf = C1>,
    C2: SecondOrderSallenKeyFilterConf<Conf = C2>,
    Internals<P::F, C>: Copy + Debug + PartialEq
{
    pub const fn new(param: P) -> Self
    {
        Self {
            param: Param::new(param),
            internals: Internals::<P::F, C>::new(),
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
            impl<P, C> StaticRtf for ThirdOrderSallenKeyFilter<C, <P as FilterParam>::F, P, $conf1, $conf2>
            where
                P: ThirdOrderSallenKeyFilterParam<C, Conf = C>,
                C: ThirdOrderSallenKeyFilterConf<
                    Conf = C,
                    S1Conf = $conf1,
                    S2Conf = $conf2,
                    OutputBufs<[P::F; 3]> = <$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 3]>,
                    OutputBufs<[P::F; 4]> = <$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 4]>,
                    Outputs<[P::F; 4]> = <<$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 4]> as ArrayMul<<$conf1 as FirstOrderRCFilterConf>::Outputs<[P::F; 4]>>>::Product
                >,
                //
                $conf1: FirstOrderRCFilterConf<Conf = $conf1>,
                $conf2: SecondOrderSallenKeyFilterConf<Conf = $conf2>,
                Internals<P::F, C>: Copy + Debug + PartialEq,
                $($($where_c)+)?
            {
                type Param = P;
                type Conf = C;
                type F = <P as FilterParam>::F;

                type IsIir<U> = <C as private::_Helper>::IsIir<U>;
                type Outputs<U> = <C as private::_Helper>::Outputs<U>;
                type Order<U> = <C as private::_Helper>::Order<U>;
                type OutputBufs<U> = <C as private::_Helper>::OutputBufs<U>;
                type SosBufs<U> = <C as private::_Helper>::SosBufs<U>;
                type SosStages<U> = <C as private::_Helper>::SosStages<U>;
                
                fn from_param(param: Self::Param) -> Self
                {
                    Self::new(param)
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
                
                fn get_internals(&self) -> (&$crate::internals::RtfInternalsFor<Self>, &Param<Self::Param>)
                {
                    (&self.internals, &self.param)
                }
                fn get_internals_mut(&mut self) -> (&mut $crate::internals::RtfInternalsFor<Self>, &mut Param<Self::Param>)
                {
                    (&mut self.internals, &mut self.param)
                }

                fn make_coeffs($arg_param: &Self::Param, $arg_rate: Self::F) -> (
                    $crate::internals::BInternalsFor<Self>,
                    Self::IsIir<$crate::internals::AInternalsFor<Self>>
                )
                {
                    fn make_coeffs<F, P, C>($arg_param: &P, $arg_rate: F) -> (
                        BInternals<F, C>,
                        [AInternals<F, C>; 1]
                    )
                    where
                        F: FilterFloat,
                        P: ThirdOrderSallenKeyFilterParam<C, Conf = C, F = F>,
                        C: ThirdOrderSallenKeyFilterConf<
                            Conf = C, S1Conf = $conf1, S2Conf = $conf2,
                            OutputBufs<[P::F; 3]> = <$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 3]>,
                            OutputBufs<[P::F; 4]> = <$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 4]>,
                            Outputs<[P::F; 4]> = <<$conf2 as SecondOrderSallenKeyFilterConf>::Outputs<[P::F; 4]> as ArrayMul<<$conf1 as FirstOrderRCFilterConf>::Outputs<[P::F; 4]>>>::Product
                        >,
                        $conf1: FirstOrderRCFilterConf<Conf = $conf1>,
                        $conf2: SecondOrderSallenKeyFilterConf<Conf = $conf2>,
                        $($($where_c)+)?
                    $make_coeffs

                    make_coeffs($arg_param, $arg_rate)
                }
            }
        )*
    };
}

// Now... how could i make this shorter?
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
        let mut filter = ThirdOrderSallenKeyFilter::<All>::new(RC3GSallenKey {r1: 470.0, c1: 47.0e-9, r2: 15.0e3, c2: 2.7e-9, r3: 16.0e3, c3: 2.7e-9, g: 1.3846153846153846});
        //let mut filter = ThirdOrderSallenKeyFilter::<All>::new(RC2GSallenKey {r1: 15.0e3, c1: 2.7e-9, r2: 15.0e3, c2: 2.7e-9, g: 2.0});
        //let mut filter = ThirdOrderSallenKeyFilter::<All>::new(RC {r: 470.0, c: 47.0e-9});

        crate::tests::plot_freq(&mut filter).unwrap();
    }
}