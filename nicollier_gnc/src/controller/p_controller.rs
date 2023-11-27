use crate::controller::Controller;
use crate::{Deflections, Reference, SystemState};
use std::f32::consts::PI;

pub struct PController();

impl PController {
    pub fn new() -> Self {
        Self()
    }
}
pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + f64::exp(2.0 * -z))
}

const K: f64 = 2.0;
impl Controller for PController {
    type State = SystemState;
    type Reference = Reference;
    type Output = Deflections;

    fn step(
        &mut self,
        state: Self::State,
        reference: Self::Reference,
        delta_t: f64,
    ) -> Self::Output {
        let error: f64 = (state.inertial_frame_angle.z % (PI * 2.0) as f64) - reference.0;

        let output = Deflections {
            sym: 0.0,
            asym: -(sigmoid(K * error) - 0.5) * 2.0,
        };
        output
    }
}
