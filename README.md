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
let mut filter = SecondOrderEllipticFilter::new::<LowPass>(
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

You can also implement your own filter, by using the macro `def_rtf!`. See how i did it with the other filters for an example on how to use the macro.
