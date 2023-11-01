mod model;
mod controller;
mod guidance;


use crate::controller::p_controller::PController;
use crate::controller::Controller;
use crate::guidance::constant_guidance::ConstantGuidance;
use crate::guidance::Guidance;
use crate::model::three_dof::ThreeDof;
use crate::model::Model;
use serde::{Serialize, Deserialize};
use nalgebra::{Vector3};
use anyhow::Result;
use std::ops::{Deref, DerefMut};


#[derive(Copy, Clone, Debug, Serialize)]
pub struct SystemState {
    inertial_frame: FrameState,
    body_frame: FrameState,
    total_time: f64,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct FrameState {
    pos: Vector3<f64>,
    velocity: Vector3<f64>,
    acceleration: Vector3<f64>,

    angle: Vector3<f64>,
    angle_velocity: Vector3<f64>,
    angle_acceleration: Vector3<f64>,
}

#[derive(Copy, Clone, Debug)]
pub struct Deflections {
    sym: f64,
    asym: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Reference{
    yaw: f64,
    yaw_previous: f64,
    sym_deflection: f64,
    sym_deflection_previous:f64
}

impl Reference {
    pub fn new(yaw: f64, yaw_previous: f64, sym_deflection: f64, sym_deflection_previous: f64) -> Self {
        Self {
            yaw,
            yaw_previous,
            sym_deflection,
            sym_deflection_previous
        }
    }
}
fn main() -> Result<()> {

    let delta_t = 1.0 / 100.0;

    let initial_state = SystemState {
        inertial_frame: FrameState{
            pos: Vector3::zeros(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            angle: Vector3::zeros(),
            angle_velocity : Vector3::zeros(),
            angle_acceleration: Vector3::zeros()
        },
        body_frame: FrameState{
            pos: Vector3::zeros(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            angle: Vector3::zeros(),
            angle_velocity : Vector3::zeros(),
            angle_acceleration: Vector3::zeros()
        },
        total_time: 0.0
    };

    let mut guidance = ConstantGuidance::new(Reference::new(0.0,0.0,0.0,0.0));
    let mut controller = PController::new();
    let mut model = ThreeDof::new(initial_state);

    while !model.landed() && initial_state.total_time<1000.0 {
        let reference = guidance.get_reference(model.get_state());
        let control_inputs = controller.step(model.get_state(), reference, delta_t);
        let updated_state = model.step(control_inputs, delta_t);

        println!("{}", serde_json::to_string(&updated_state)?);
    }

    Ok(())
}



//future tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn always_lands() {
        assert_eq!(0, 0);
    }
}
