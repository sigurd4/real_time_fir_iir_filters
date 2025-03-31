use serde::{Serialize, Deserialize};

use crate::param::{FilterFloat, FilterParam, Param, WahFilterParam};

use super::X;

pub(crate) const GUITAR_R_I: f64 = 10e3;
pub(crate) const V_T: f64 = 26e-3;

pub(crate) const MPSA18_V_F: f64 = 0.6;
pub(crate) const MPSA18_BETA: f64 = 580.0;

pub(crate) const BC109B_V_F: f64 = 0.7;
pub(crate) const BC109B_BETA: f64 = 500.0;

pub(crate) const BC184_V_F: f64 = 0.7;
pub(crate) const BC184_BETA: f64 = 240.0;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct CrybabyGCB95<F>
where
    F: FilterFloat
{
    pub x: F
}
impl<F> FilterParam for Param<CrybabyGCB95<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 4;

    type F = F;
}
impl<F> WahFilterParam for Param<CrybabyGCB95<F>>
where
    F: FilterFloat
{
    const R_I: f64 = 68e3 + GUITAR_R_I; // R1
    const R_S: f64 = 1.5e3; // R2
    const R_P: f64 = 33e3; // R7
    const R_C1: f64 = 22e3; // R3
    const R_E1: f64 = 470.0; // R4
    const R_C2: f64 = 1e3; // R9
    const R_E2: f64 = 10e3; // R10
    const R_J: f64 = 470e3; // R6 and R5
    const R_G: f64 = 82e3; // R8
    const R_POT: f64 = 100e3; // VR1
    
    const V_CC: f64 = 9.0; // Supply voltage
    const V_F: f64 = MPSA18_V_F; // BJT forward voltage
    const V_T: f64 = V_T; // BJT thermal voltage
    
    const BETA: f64 = MPSA18_BETA; // BJT forward gain / hFE
    
    const C_G: f64 = 4.7e-6; // C3
    const C_I: f64 = 10e-9; // C1
    const C_F: f64 = 10e-9; // C2
    
    const L: f64 = 540e-3; // L1

    fn x(&self) -> X<Self::F>
    {
        let CrybabyGCB95 {x} = **self;
        X {
            x
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct VoxV847<F>
where
    F: FilterFloat
{
    pub x: F
}
impl<F> FilterParam for Param<VoxV847<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 4;

    type F = F;
}
impl<F> WahFilterParam for Param<VoxV847<F>>
where
    F: FilterFloat
{
    const R_I: f64 = 68e3 + GUITAR_R_I; // R1 + input impedence
    const R_S: f64 = 1.5e3; // R2
    const R_P: f64 = 33e3; // R7
    const R_C1: f64 = 22e3; // R3
    const R_E1: f64 = 510.0; // R4
    const R_C2: f64 = 1e3; // R9
    const R_E2: f64 = 10e3; // R10
    const R_J: f64 = 470e3; // R6 and R5
    const R_G: f64 = 100e3; // R8
    const R_POT: f64 = 100e3; // VR1
    
    const V_CC: f64 = 9.0; // Supply voltage
    const V_F: f64 = BC109B_V_F; // BJT forward voltage
    const V_T: f64 = V_T; // BJT thermal voltage
    
    const BETA: f64 = BC109B_BETA; // BJT forward gain / hFE
    
    const C_G: f64 = 4e-6; // C3
    const C_I: f64 = 10e-9; // C1
    const C_F: f64 = 10e-9; // C2
    
    const L: f64 = 500e-3; // L1

    fn x(&self) -> X<Self::F>
    {
        let VoxV847 {x} = **self;
        X {
            x
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct ColorsoundWow<F>
where
    F: FilterFloat
{
    pub x: F
}
impl<F> FilterParam for Param<ColorsoundWow<F>>
where
    F: FilterFloat
{
    const ORDER: usize = 4;

    type F = F;
}
impl<F> WahFilterParam for Param<ColorsoundWow<F>>
where
    F: FilterFloat
{
    const R_I: f64 = 100e3 + GUITAR_R_I; // R1
    const R_S: f64 = 1.5e3; // R2
    const R_P: f64 = 33e3; // R7
    const R_C1: f64 = 22e3; // R3
    const R_E1: f64 = 0.0; // R4
    const R_C2: f64 = 0.0; // R9
    const R_E2: f64 = 10e3; // R10
    const R_J: f64 = 470e3; // R6 and R5
    const R_G: f64 = 100e3; // R8
    const R_POT: f64 = 100e3; // VR1
    
    const V_CC: f64 = 9.0; // Supply voltage
    const V_F: f64 = BC184_V_F; // BJT forward voltage
    const V_T: f64 = V_T; // BJT thermal voltage
    
    const BETA: f64 = BC184_BETA; // BJT forward gain / hFE
    
    const C_G: f64 = 4.7e-6; // C3
    const C_I: f64 = 10e-9; // C1
    const C_F: f64 = 10e-9; // C2
    
    const L: f64 = 500e-3; // L1

    fn x(&self) -> X<Self::F>
    {
        let ColorsoundWow {x} = **self;
        X {
            x
        }
    }
}