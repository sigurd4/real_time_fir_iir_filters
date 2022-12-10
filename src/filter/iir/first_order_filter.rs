use super::IIRFilter;

pub mod rc;
pub mod lr;
pub mod all_pass;

pub use rc::*;
pub use lr::*;
pub use all_pass::*;

#[derive(Copy, Clone)]
pub struct FirstOrderFilter
{
    pub omega: f32,
    pub w: [f32; 1]
}

impl FirstOrderFilter
{
    pub fn new(omega: f32) -> Self
    {
        Self {
            omega,
            w: [0.0; 1]
        }
    }
}

impl IIRFilter<1, 2> for FirstOrderFilter
{
    fn a(&self, rate: f32) -> [f32; 2]
    {
        [
            self.omega + 2.0*rate,
            self.omega - 2.0*rate,
        ]
    }
    fn b(&self, rate: f32) -> [[f32; 2]; 2]
    {
        [
            [
                self.omega,
                self.omega
            ],
            [
                2.0*rate,
                -2.0*rate
            ]
        ]
    }
    fn w(&mut self) -> &mut [f32; 1]
    {
        &mut self.w
    }
}