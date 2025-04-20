[![Build Status (nightly)](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Test/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/real_time_fir_iir_filters/workflows/Lint/badge.svg)](https://github.com/sigurd4/real_time_fir_iir_filters/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/real_time_fir_iir_filters.svg)](https://crates.io/crates/real_time_fir_iir_filters)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/real_time_fir_iir_filters)](https://docs.rs/real_time_fir_iir_filters)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/real_time_fir_iir_filters)](https://app.codecov.io/github/sigurd4/real_time_fir_iir_filters)

# real_time_fir_iir_filters

Ever needed a low pass filter for your VST? This crate has a wide selection of filters for real-time usage. It's designed to have as little runtime overhead as possible.

## How does it work?

Everything that can be computed at compile-time, will be, and the filter coefficients will be cached as well.

I then use the following algorithm to process the signal with as few steps as possible given the filter's coefficients:

![2025-03-24-032452_hyprshot](https://github.com/user-attachments/assets/bd22e03f-b69c-4506-bbbd-baccf7a6c81d)

(The figure is from: Alan V. Oppenheimer & Ronald W. Schafer - Discrete-Time Signal Processing)

## Example

```rust
use core::f64::consts::TAU;

use real_time_fir_iir_filters::{
    conf::LowPass,
    param::OmegaEpsilonXi,
    rtf::Rtf,
    filters::iir::second::SecondOrderEllipticFilter
};

// Initialize a 2. order elliptic low-pass filter at 440Hz
let mut filter = SecondOrderEllipticFilter::<LowPass>::new(
    OmegaEpsilonXi {
        omega: 440.0*TAU,
        epsilon: 0.5,
        xi: 1.5
    }
);

const N: usize = 10;
const RATE: f64 = 8000.0;

{
    // Unit impulse
    let mut imp_resp = [0.0; N];
    imp_resp[0] = 1.0;

    // Apply filter to imp_resp
    for x in &mut imp_resp
    {
        [*x] = filter.filter(RATE, *x);
    }

    // Prints the impulse response of the filter.
    println!("h[n] = {:?}", imp_resp);
}

// Resets internal state of filter to zero
filter.reset();

{
    // Generate frequency response for ω ∈ [0, 2π)
    let freq_resp = core::array::from_fn::<_, N, _>(|i| {
        let omega = i as f64/N as f64*TAU;

        filter.frequency_response(RATE, omega)
    });

    println!("H = {:?}", freq_resp);
}
```

## Available filters

| Order | Filter                         | Parameterization                                                   | Configuration                                                                                            |
|-------|--------------------------------|--------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| 1     | `FirstOrderAllPassFilter`      | `Tau`                                                              | `AllPass`                                                                                                |
| 1     | `FirstOrderFilter`             | `Omega` `RC` `LR`                                                  | `LowPass` `HighPass`                                                                                     |
| 1     | `FirstOrderLRFilter`           | `LR`                                                               | `LowPass` `HighPass`                                                                                     |
| 1     | `FirstOrderRCFilter`           | `RC`                                                               | `LowPass` `HighPass`                                                                                     |
| 1     | `PIFilter`                     | `PI`                                                               | -                                                                                                        |
| 2     | `PIDFilter`                    | `PI` `PID`                                                         | -                                                                                                        |
| 2     | `SecondOrderButterworthFilter` | `Omega`                                                            | `LowPass` `Peak` `HighPass`                                                                              |
| 2     | `SecondOrderChebyshev1Filter`  | `Omega` `OmegaEpsilon`                                             | `LowPass` `HighPass`                                                                                     |
| 2     | `SecondOrderChebyshev2Filter`  | `Omega` `OmegaEpsilon`                                             | `LowPass` `HighPass`                                                                                     |
| 2     | `SecondOrderEllipticFilter`    | `Omega` `OmegaEpsilon` `OmegaEpsilonXi`                            | `LowPass` `HighPass`                                                                                     |
| 2     | `SecondOrderFilter`            | `Omega` `OmegaZeta`                                                | `LowPass` `Peak` `HighPass`                                                                              |
| 2     | `SecondOrderRCFilter`          | `RC` `RC2`                                                         | `LowPass` `BandPass<1>` `BandPass<2>` `HighPass`                                                         |
| 2     | `SecondOrderRLCFilter`         | `RC` `LR` `RLC`                                                    | `LowPass` `BandStop` `BandPass` `HighPass`                                                               |
| 2     | `SecondOrderSallenKeyFilter`   | `RC2SallenKey` `RC2GSallenKey`                                     | `LowPass` `BandPass<1>` `BandPass<2>` `HighPass`                                                         |
| 3     | `ThirdOrderButterworthFilter`  | `Omega`                                                            | `LowPass` `Peak<1>` `Peak<2>` `HighPass`                                                                 |
| 3     | `ThirdOrderFilter`             | `Omega` `OmegaZeta` `Omega2Zeta`                                   | `LowPass` `Peak<1>` `Peak<2>` `HighPass`                                                                 |
| 3     | `ThirdOrderSallenKeyFilter`    | `RC` `RC2SallenKey` `RC2GSallenKey` `RC3SallenKey` `RC3GSallenKey` | `LowPass` `BandPass<1>` `BandPass<2>` `BandPass<3>` `BandPass<4>` `BandPass<5>` `BandPass<6>` `HighPass` |
| 4     | `WahFilter`                    | `CrybabyGCB95` `VoxV847` `ColorsoundWow`                           | -                                                                                                        |

...and more to come!

## Adding your own filter

You can also implement your own filter, by using the macro `def_rtf!`.

Here's an example on how to implement your very own first-order low-pass filter:

```rust
// This macro declares a struct for your filter, and implements all the necessary traits for it.
real_time_fir_iir_filters::def_rtf!(
    {
        /// My very own first-order low-pass filter.
        /// 
        /// You can write your own doc-string here for your filter struct, just like you would document any other struct.
    }
    FirstOrderLowPassFilter
    {
        // The parameter trait for this filter (the struct containing variables the filter depends on) and its default parameter type.
        // In this case, we use the pre-defined `FirstOrderFilterParam` and the first order `Omega` as defaults, but you are free to make your own parameter trait and types.
        // Parameter traits need to extend the the `FilterParam` trait.
        type Param: FirstOrderFilterParam = OmegaFirstOrder;

        // Amount of outputs for the filter.
        // This is also how many numerators you need in the output-stage.
        // This is typically for when you want the filter to act as a low-pass and high-pass filter simultaneously.
        // This can be useful sometimes, since various configurations can often share filter coefficient denominators, so they can then re-use the same computation.
        const OUTPUTS: usize = 1;

        // The amount of separate buffers for the output stage.
        // This is also how many denominators you need in the output-stage.
        // `OUTPUTS` must be a multiple of `O_BUFFERS`.
        // When you have less output buffers than outputs, denominators will be shared across outputs.
        const O_BUFFERS: usize = 1;

        // The amount of separate buffers for the second-order section stages.
        // `O_BUFFERS` must be a multiple of `SOS_BUFFERS`.
        // When you have less SOS-buffers than output buffers, the result of the final SOS-stage for each buffer will be shared across output-buffers.
        const SOS_BUFFERS: usize = 1;

        // The amount of additional second-order section stages.
        // Second-order sections allow computing filters with higher accuracy, by using a cascade of second order filters before the final stage (which can be any order).
        const SOS_STAGES: usize = 0;

        // The order of the output stage.
        // This means the output stage will have `ORDER + 1` coefficients.
        const ORDER: usize = 1;

        // Wether or not the filter has a denominator in its transfer function.
        const IS_IIR: bool = true;

        fn make_coeffs(param, rate) -> _
        {
            // This retrieves the parameter variable, and does the necessary calculations on fore-hand to then compute the filter-coefficients.
            // This code will only be ran after the filter parameter has changed, and the coefficients will be cached in the mean-time.
            let Omega {omega} = omega.omega();
            let two_rate = rate + rate;

            // This contains the polynomial coefficients of the filter's transfer function in the Z-domain.
            // In this case, the transfer function (in the Z-domain) is on the form:
            // ```
            //        b0 + z⁻¹b1
            // H(z) = ----------
            //        a0 + z⁻¹a1
            // ```
            // For analog circuits, i typically use Kirchhoff's voltage or current law to find `H(s)`.
            // You can then find these Z-domain coefficients by applying the bilinear transform to `H(s)`.
            // Alternatively, there are some functions available in this library that does the bilinear transform for you for specific orders,
            // given the right S-domain coefficients. Those are only available for some specific filter orders.
            (
                // Numerator
                (
                    [
                        // Preceeding second-order section-stages numerator coefficients (all except the last one).
                        // One set of `3` coefficients for each second-order section-buffer, for each second-order section except the last one, if there are more than one.
                        // If there is no more than one second-order section, this array is empty.
                        // In this case we have no second-order sections, so this array is empty.
                    ],
                    [
                        // Final second-order section-stage numerator coefficients.
                        // One set of `3` coefficients for each buffered output, for the final second-order section, if it exists.
                        // In this case we have no second-order sections, so this array is empty.
                    ],
                    [
                        // Output-stage numerator coefficients, one set of `ORDER + 1` coefficients for each output
                        [
                            omega, // b0
                            omega // b1
                        ]
                    ]
                ),
                // Denominator
                [
                    // This is an empty array for FIR filters, since it would then have no denominator in its transfer function.
                    // Otherwise, as in this case, it is an array of length `1`.
                    (
                        [
                            // Second-order section-stage denominator coefficients.
                            // One set of `3` coefficients for each second-order section-buffer, for each second-order section.
                            // In this case we have no second-order sections, so this array is empty.
                        ],
                        [
                            // Output-stage denominator coefficients.
                            // One set of `ORDER + 1` coefficients for each buffered output
                            [
                                omega + two_rate, // a0
                                omega - two_rate // a1
                            ]
                        ]
                    )
                ]
            )
        }
    }
);
```