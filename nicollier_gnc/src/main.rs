mod model;
mod controller;
mod guidance;

use std::error::Error;
use csv::Writer;
use std::fs::File;
use std::io::prelude::*;
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
    inertial_frame_position: Vector3<f64>,
    inertial_frame_velocity: Vector3<f64>,
    inertial_frame_acceleration: Vector3<f64>,

    inertial_frame_angle: Vector3<f64>,
    inertial_frame_angle_velocity: Vector3<f64>,
    inertial_frame_angle_acceleration: Vector3<f64>,

    total_time: f64,

    //body frame pos is always 0,0,0
    body_frame_velocity: Vector3<f64>,
    body_frame_angle_velocity: Vector3<f64>,
    body_frame_angle_acceleration: Vector3<f64>,
    body_frame_acceleration: Vector3<f64>,
}

#[derive(Copy, Clone, Debug)]
pub struct Deflections {
    sym: f64,
    asym: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Reference(f64);

impl Reference {


}
fn main() -> Result<()> {
    let mut wtr = Writer::from_path("foo.csv")?;

    let delta_t = 0.01;

    let initial_state = SystemState {

        inertial_frame_position: Vector3::new(0.0,0.0, -300.0),
        inertial_frame_velocity: Vector3::zeros(),
        inertial_frame_acceleration: Vector3::zeros(),
        inertial_frame_angle: Vector3::zeros(),
        inertial_frame_angle_velocity : Vector3::zeros(),
        inertial_frame_angle_acceleration: Vector3::zeros(),

        body_frame_velocity: Vector3::zeros(),
        body_frame_angle_velocity: Vector3::zeros(),
        body_frame_angle_acceleration: Vector3::zeros(),
        body_frame_acceleration: Vector3::zeros(),

        total_time: 0.0,

    };

    let mut guidance = ConstantGuidance::new(Reference(0.0));
    let mut controller = PController::new();
    let mut model = ThreeDof::new(initial_state);

    while !model.landed() && model.get_state().total_time<1000.0 {
        let reference = guidance.get_reference(model.get_state());
        let control_inputs = controller.step(model.get_state(), reference, delta_t);
        let updated_state = model.step(control_inputs, delta_t);
        wtr.write_record(&[updated_state.inertial_frame_position.x.to_string(),updated_state.inertial_frame_position.y.to_string(), (-updated_state.inertial_frame_position.z).to_string()])?;
        //println!("{},{},{}\n", updated_state.inertial_frame_position.x,updated_state.inertial_frame_position.y, -updated_state.inertial_frame_position.z);
    }
    wtr.flush()?;

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
