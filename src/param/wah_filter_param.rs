use super::FilterParam;

pub trait WahFilterParam: FilterParam
{
    /// Input resistance to base of Q1
    const R_I: f64 = 78e3;
    /// Feedback resistor to base of Q1
    const R_S: f64 = 1.5e3;
    /// Resistor paralell with inductor
    const R_P: f64 = 33e3;
    /// Collector resistor of Q1
    const R_C1: f64 = 22e3;
    /// Emitter resistor of Q1
    const R_E1: f64 = 390.0;
    /// Collector resistor of Q2
    const R_C2: f64 = 1e3;
    /// Emitter resistor of Q2
    const R_E2: f64 = 10e3;
    /// Junction mixing resistors
    const R_J: f64 = 470e3;
    /// Resistor to ground from inductor junction
    const R_G: f64 = 82e3;
    /// Potentiometer resistance
    const R_POT: f64 = 100e3;
    
    /// Supply voltage
    const V_CC: f64 = 9.0;
    /// BJT forward voltage
    const V_F: f64 = 0.6;
    /// BJT thermal voltage
    const V_T: f64 = 26e-3;
    
    /// BJT forward gain / hFE
    const BETA: f64 = 580.0;
    
    /// Capacitor to ground from inductor junction
    const C_G: f64 = 4.7e-6;
    /// Input capacitor to base of Q1
    const C_I: f64 = 10e-9;
    /// Feeedback capacitor from emitter of Q2
    const C_F: f64 = 10e-9;
    
    /// Inductor
    const L: f64 = 500e-3;

    /// Position of wah potentiometer from 0.0 to 1.0
    /// Values out of range may give strange results
    fn position(&self) -> Self::F;
}