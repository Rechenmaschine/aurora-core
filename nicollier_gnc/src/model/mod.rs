pub mod three_dof;

use std::todo;
use nalgebra::{Rotation3, Vector3};
use crate::{Deflections, SystemState};

pub trait Model {
    type State;
    type Input;

    fn get_state(&self) -> Self::State;

    fn step(&mut self, input: Self::Input, delta_t: f64) -> Self::State;
    fn inertial_to_body(&self) -> Rotation3<f64>;

    fn landed(&self) -> bool;
}