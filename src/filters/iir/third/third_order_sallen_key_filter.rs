use num::Float;

use crate::{conf::{All, BandPass, Conf, HighPass, LowPass}, internals::{ainternals, binternals, rtfinternals}, param::{FilterFloat, FirstOrderRCFilterConf, SecondOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterConf, ThirdOrderSallenKeyFilterParam}, params::RC3GSallenKey, rtf::RtfBase, static_rtf::StaticRtfBase, util::same::Same};

#[allow(type_alias_bounds)]
type BInternals<F, CC1: FirstOrderRCFilterConf, CC2: SecondOrderSallenKeyFilterConf> = binternals!(
    F,
    <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<CC1 as FirstOrderRCFilterConf>::OUTPUTS,
    <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
    1,
    0,
    3
);
#[allow(type_alias_bounds)]
type AInternals<F, CC1: FirstOrderRCFilterConf, CC2: SecondOrderSallenKeyFilterConf> = ainternals!(
    F,
    <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
    1,
    0,
    3
);
#[allow(type_alias_bounds)]
type Internals<F, CC1: FirstOrderRCFilterConf, CC2: SecondOrderSallenKeyFilterConf> = rtfinternals!(
    F,
    <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<CC1 as FirstOrderRCFilterConf>::OUTPUTS,
    <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS,
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
    F,
    P = RC3GSallenKey<F>,
    C = All,
    CC1 = <<P as ThirdOrderSallenKeyFilterParam<C>>::Conf as ThirdOrderSallenKeyFilterConf>::S1Conf,
    CC2 = <<P as ThirdOrderSallenKeyFilterParam<C>>::Conf as ThirdOrderSallenKeyFilterConf>::S2Conf,
    CC = <<P as ThirdOrderSallenKeyFilterParam<C>>::Conf as ThirdOrderSallenKeyFilterConf>::Conf
>
where
    F: FilterFloat,
    P: ThirdOrderSallenKeyFilterParam<C, F = F>,
    C: Conf,
    CC1: FirstOrderRCFilterConf,
    CC2: SecondOrderSallenKeyFilterConf,
    CC: ThirdOrderSallenKeyFilterConf,
    P::Conf: ThirdOrderSallenKeyFilterConf<Conf = CC, S1Conf = CC1, S2Conf = CC2>,
    [(); <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS]:,
    [(); <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<CC1 as FirstOrderRCFilterConf>::OUTPUTS]:
{
    pub param: P,
    pub internals: Internals<P::F, CC1, CC2>,
    phantom: core::marker::PhantomData<C>
}

impl<P, C, CC1, CC2> ThirdOrderSallenKeyFilter<P::F, P, C, CC1, CC2>
where
    P: ThirdOrderSallenKeyFilterParam<C>,
    C: Conf,
    CC1: FirstOrderRCFilterConf,
    CC2: SecondOrderSallenKeyFilterConf,
    P::Conf: ThirdOrderSallenKeyFilterConf<S1Conf = CC1, S2Conf = CC2>,
    [(); <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS]:,
    [(); <CC2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<CC1 as FirstOrderRCFilterConf>::OUTPUTS]:
{
    pub const fn new<CCC>(param: P) -> Self
    where
        CCC: Conf + Same<C>
    {
        Self {
            param,
            internals: Internals::<P::F, CC1, CC2>::new(),
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
            impl<P, C> RtfBase for ThirdOrderSallenKeyFilter<P::F, P, C, $conf1, $conf2, <P::Conf as ThirdOrderSallenKeyFilterConf>::Conf>
            where
                P: ThirdOrderSallenKeyFilterParam<C>,
                C: Conf,
                P::Conf: ThirdOrderSallenKeyFilterConf<S1Conf = $conf1, S2Conf = $conf2>,
                $($($where_c)+)?
            {
                type Conf = <P::Conf as ThirdOrderSallenKeyFilterConf>::Conf;
                type F = P::F;
            
                const IS_IIR: bool = true;
                const OUTPUTS: usize = <$conf2 as SecondOrderSallenKeyFilterConf>::OUTPUTS*<$conf1 as FirstOrderRCFilterConf>::OUTPUTS;
            }
            impl<P, C> StaticRtfBase for ThirdOrderSallenKeyFilter<P::F, P, C, $conf1, $conf2, <P::Conf as ThirdOrderSallenKeyFilterConf>::Conf>
            where
                P: ThirdOrderSallenKeyFilterParam<C>,
                C: Conf,
                P::Conf: ThirdOrderSallenKeyFilterConf<S1Conf = $conf1, S2Conf = $conf2>,
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
                        param,
                        internals: Internals::<P::F, $conf1, $conf2>::new(),
                        phantom: core::marker::PhantomData
                    }
                }
                fn get_param(&self) -> &Self::Param
                {
                    &self.param
                }
                fn get_param_mut(&mut self) -> &mut Self::Param
                {
                    &mut self.param
                }
                fn into_param(self) -> Self::Param
                {
                    self.param
                }
                
                fn get_internals(&self) -> (&Internals<P::F, $conf1, $conf2>, &Self::Param)
                {
                    (&self.internals, &self.param)
                }
                fn get_internals_mut(&mut self) -> (&mut Internals<P::F, $conf1, $conf2>, &mut Self::Param)
                {
                    (&mut self.internals, &mut self.param)
                }

                fn make_coeffs($arg_param: &Self::Param, $arg_rate: Self::F) -> (
                    BInternals<P::F, $conf1, $conf2>,
                    [AInternals<P::F, $conf1, $conf2>; true as usize]
                )
                {
                    fn make_coeffs<F, P, C>($arg_param: &P, $arg_rate: F) -> (
                        BInternals<F, $conf1, $conf2>,
                        [AInternals<F, $conf1, $conf2>; true as usize]
                    )
                    where
                        F: FilterFloat,
                        P: ThirdOrderSallenKeyFilterParam<C, F = F>,
                        C: Conf,
                        P::Conf: ThirdOrderSallenKeyFilterConf<S1Conf = $conf1, S2Conf = $conf2>,
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
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, All>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, All>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }

    fn make_coeffs<All, LowPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, LowPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, LowPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass<1>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass<1>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass<1>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass<2>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass<2>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass<2>>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, HighPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, HighPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, HighPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }

    // Essentials^

    fn make_coeffs<All, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<1>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<2>)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, BandPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, BandPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, BandPass>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<1>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (LowPass, BandPass<2>, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    
    fn make_coeffs<All, (BandPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, (BandPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, (BandPass, HighPass)>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
);

/*
    fn make_coeffs<All, All>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<LowPass, All>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_low_pass_filter_b(g, r2),
                third_order_sallen_key_band_pass_filter2_b(c2, g, r2, rate),
                third_order_sallen_key_band_pass_filter4_b(c3, g, r2, r3, rate),
                third_order_sallen_key_band_pass_filter6_b(c2, c3, g, r2, r3, rate2)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
    fn make_coeffs<HighPass, All>(param, rate) -> _
    {
        let rate2 = rate*rate;
        let rate3 = rate2*rate;

        let r1 = param.r1();
        let c1 = param.c1();
        let r2 = param.r2();
        let c2 = param.c2();
        let r3 = param.r3();
        let c3 = param.c3();
        let g = param.g();
        let one_m_g = F::one() - g;

        (
            ([], [], [
                third_order_sallen_key_band_pass_filter1_b(c1, g, r1, r2, rate),
                third_order_sallen_key_band_pass_filter3_b(c1, c2, g, r1, r2, rate2),
                third_order_sallen_key_band_pass_filter5_b(c1, c3, g, r1, r2, r3, rate2),
                third_order_sallen_key_high_pass_filter_b(c1, c2, c3, g, r1, r2, r3, rate3)
            ]),
            [([], [
                third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3),
                third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a(c1, c2, c3, one_m_g, r1, r2, r3, rate, rate2, rate3)
            ])]
        )
    }
*/

pub(crate) fn third_order_sallen_key_low_pass_filter_b<F>(g: F, r2: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_0(g*r2)
}
pub(crate) fn third_order_sallen_key_band_pass_filter1_b<F>(c1: F, g: F, r1: F, r2: F, rate: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_1(c1*g*r1*r2, rate)
}
pub(crate) fn third_order_sallen_key_band_pass_filter2_b<F>(c2: F, g: F, r2: F, rate: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_1(c2*g*r2, rate)
}
pub(crate) fn third_order_sallen_key_band_pass_filter3_b<F>(c1: F, c2: F, g: F, r1: F, r2: F, rate2: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_2(c1*c2*g*r1*r2, rate2)
}
pub(crate) fn third_order_sallen_key_band_pass_filter4_b<F>(c3: F, g: F, r2: F, r3: F, rate: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_1(c3*g*r2*r3, rate)
}
pub(crate) fn third_order_sallen_key_band_pass_filter5_b<F>(c1: F, c3: F, g: F, r1: F, r2: F, r3: F, rate2: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_2(c1*c3*g*r1*r2*r3, rate2)
}
pub(crate) fn third_order_sallen_key_band_pass_filter6_b<F>(c2: F, c3: F, g: F, r2: F, r3: F, rate2: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_2(c2*c3*g*r2*r3, rate2)
}
pub(crate) fn third_order_sallen_key_high_pass_filter_b<F>(c1: F, c2: F, c3: F, g: F, r1: F, r2: F, r3: F, rate3: F) -> [F; 4]
where
    F: Float
{
    crate::billinear4_3(c1*c2*c3*g*r1*r2*r3, rate3)
}

pub(crate) fn third_order_sallen_key_low_pass_filter_or_band_pass_filter1_a<F>(c1: F, c2: F, c3: F, one_m_g: F, r1: F, r2: F, r3: F, rate: F, rate2: F, rate3: F) -> [F; 4]
where
    F: Float
{
    let two_r1 = r1 + r1;
    crate::billinear4_0_1_2_3(
        r2 + two_r1,
        r2*(c1*r1 + c2*one_m_g*(r2 + r1) + c3*(r3 + r2 + r1)) + two_r1*c3*r3,
        r2*(c1*r1*(c2*r2*one_m_g + c3*(r3 + r2)) + c2*c3*r3*(r2 + r1)),
        c1*c2*c3*r1*r2*r2*r3,
        rate,
        rate2,
        rate3
    )
}
pub(crate) fn third_order_sallen_key_band_pass_filter2_or_band_pass_filter3_a<F>(c1: F, c2: F, c3: F, one_m_g: F, r1: F, r2: F, r3: F, rate: F, rate2: F, rate3: F) -> [F; 4]
where
    F: Float
{
    let two_c2 = c2 + c2;
    crate::billinear4_0_1_2_3(
        one_m_g,
        r1*one_m_g*(c1 + c2) + c3*(r2 + r3) + c2*r2,
        c1*r1*(c3*r2 + c3*r3 + c2*r2) + c2*(c3*(r2*r3 + r1*r2 + r1*r3) + two_c2*r1*r2),
        c2*c3*r1*r2*r3*(c1 + two_c2),
        rate,
        rate2,
        rate3
    )
}
pub(crate) fn third_order_sallen_key_band_pass_filter4_or_band_pass_filter5_a<F>(c1: F, c2: F, c3: F, one_m_g: F, r1: F, r2: F, r3: F, rate: F, rate2: F, rate3: F) -> [F; 4]
where
    F: Float
{
    let two_r1 = r1 + r1;
    crate::billinear4_0_1_2_3(
        two_r1 + r2,
        r2*(c1*r1 + c2*(r1 + r2) + c3*(r1 + r3 + r2)) + two_r1*c3*r3,
        r2*(c1*r1*(c2*r2 + c3*(r3 + r2)) + c2*c3*r3*one_m_g*(r1 + r2)),
        c1*c2*c3*r1*r2*r2*r3*one_m_g,
        rate,
        rate2,
        rate3
    )
}
pub(crate) fn third_order_sallen_key_band_pass_filter6_or_high_pass_filter_a<F>(c1: F, c2: F, c3: F, one_m_g: F, r1: F, r2: F, r3: F, rate: F, rate2: F, rate3: F) -> [F; 4]
where
    F: Float
{
    let two_c2 = c2 + c2;
    crate::billinear4_0_1_2_3(
        F::one(),
        c1*r1 + c3*(r2 + r3*one_m_g) + c2*(r2 + r1),
        c1*r1*(c3*(r2 + r3*one_m_g) + c2*r2) + c2*(c3*(r2*r3 + r1*(r2 + r3*one_m_g)) + two_c2*r1*r2),
        c2*c3*r1*r2*r3*(c1 + two_c2),
        rate,
        rate2,
        rate3
    )
}

#[cfg(test)]
mod test
{
    use crate::{conf::All, params::RC3GSallenKey};

    use super::ThirdOrderSallenKeyFilter;

    #[test]
    fn plot()
    {
        let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC3GSallenKey::new(470.0, 47.0e-9, 15.0e3, 2.7e-9, 16.0e3, 2.7e-9, 1.3846153846153846));
        //let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC2GSallenKey::new(15.0e3, 2.7e-9, 15.0e3, 2.7e-9, 2.0));
        //let mut filter = ThirdOrderSallenKeyFilter::new::<All>(RC::new(470.0, 47.0e-9));

        crate::tests::plot_freq(&mut filter, false).unwrap();
    }
}