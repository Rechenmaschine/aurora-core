use crate::model::Model;
use crate::{Deflections, SystemState};
use nalgebra::Vector3;
use std::alloc::System;

pub struct ThreeDof {
    state: SystemState,
}

impl ThreeDof {
    pub fn new(state: SystemState) -> Self {
        Self {
            state
        }
    }
}

impl Model for ThreeDof {
    type State = SystemState;
    type Input = Deflections;

    fn get_state(&self) -> Self::State {
        self.state
    }

    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State {
        let mut acceleration:Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);




    }

    fn landed(&self) -> bool {
        self.state.pos.z > 0.0
    }
}
