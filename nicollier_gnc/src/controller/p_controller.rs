use crate::controller::Controller;
use crate::{Deflections, Reference, SystemState};

pub struct PController ();

impl PController {
    pub fn new() -> Self {
        Self()
    }
}
pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-z))
}

const K:f64 = 1.0;
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
        let error:f64 = state.inertial_frame_angle.z - reference.0;
        let output = Deflections {
            sym: 0.0,
            asym: (sigmoid(K*error)-0.5)*2.0,
        };
        output
    }
}
