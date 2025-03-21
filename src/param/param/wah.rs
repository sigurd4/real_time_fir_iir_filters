use crate::param::{FilterParam, GUITAR_R_I, MPSA18_BETA, MPSA18_V_F, V_T, X};

pub trait WahFilterParam: FilterParam
{
    /// Input resistance to base of Q1
    const R_I: f64 = 68e3 + GUITAR_R_I; // R1
    /// Feedback resistor to base of Q1
    const R_S: f64 = 1.5e3; // R2
    /// Resistor paralell with inductor
    const R_P: f64 = 33e3; // R7
    /// Collector resistor of Q1
    const R_C1: f64 = 22e3; // R3
    /// Emitter resistor of Q1
    const R_E1: f64 = 470.0; // R4
    /// Collector resistor of Q2
    const R_C2: f64 = 1e3; // R9
    /// Emitter resistor of Q2
    const R_E2: f64 = 10e3; // R10
    /// Junction mixing resistors
    const R_J: f64 = 470e3; // R6 and R5
    /// Resistor to ground from inductor junction
    const R_G: f64 = 82e3; // R8
    /// Potentiometer resistance
    const R_POT: f64 = 100e3; // VR1
    
    /// Supply voltage
    const V_CC: f64 = 9.0; // Supply voltage
    /// BJT forward voltage
    const V_F: f64 = MPSA18_V_F; // BJT forward voltage
    /// BJT thermal voltage
    const V_T: f64 = V_T; // BJT thermal voltage
    
    /// BJT forward gain / hFE
    const BETA: f64 = MPSA18_BETA; // BJT forward gain / hFE
    
    /// Capacitor to ground from inductor junction
    const C_G: f64 = 4.7e-6; // C3
    /// Input capacitor to base of Q1
    const C_I: f64 = 10e-9; // C1
    /// Feeedback capacitor from emitter of Q2
    const C_F: f64 = 10e-9; // C2
    
    /// Inductor
    const L: f64 = 540e-3; // L1

    /// Position of wah potentiometer from 0.0 to 1.0
    /// Values out of range may give strange results
    fn x(&self) -> X<Self::F>;
}