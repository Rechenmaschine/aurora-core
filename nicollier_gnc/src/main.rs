use anyhow::Result;
use csv::Writer;
use nalgebra::Vector3;
use nicollier_gnc::controller::p_controller::PController;
use nicollier_gnc::controller::Controller;
use nicollier_gnc::guidance::double_wall::DoubleWallGuidance;
use nicollier_gnc::guidance::Guidance;
use nicollier_gnc::model::three_dof::ThreeDof;
use nicollier_gnc::model::Model;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};

use nicollier_gnc::{Deflections, Reference, SystemState};

fn main() -> Result<()> {
    /*
    let mut position_writer = Writer::from_path("pos.csv")?;
    let mut deflection_writer = Writer::from_path("def.csv")?;
    let mut reference_writer = Writer::from_path("ref.csv")?;
    let mut yaw_writer = Writer::from_path("yaw.csv")?;

     */
    let mut writer = Writer::from_path("all.csv")?;

    //let constant_yaw_angle = Reference(2.0); // constant guidance given

    let delta_t = 0.01;

    let initial_state = SystemState::initial_state();

    //let mut guidance = ConstantGuidance::new(Reference(0.0));
    //let mut guidance = ConstantYawGuidance::new(constant_yaw_angle);//my const guidance
    let mut guidance = DoubleWallGuidance::new(60.0, 10.0, 5.0, 100.0, 0.5);
    let mut controller = PController::new();
    let mut model = ThreeDof::new(initial_state);

    while !model.landed()
    /*&& model.get_state().total_time<1000.0*/
    {
        let state = model.get_state();
        let reference = guidance.get_reference(state);
        let control_inputs = controller.step(model.get_state(), reference, delta_t);
        let updated_state = model.step(control_inputs, delta_t);

        writer.write_record(&[
            model.get_state().total_time.to_string(),
            control_inputs.asym.to_string(),
            reference.0.to_string(),
            updated_state.inertial_frame_angle.z.to_string(),
            updated_state.inertial_frame_position.x.to_string(),
            updated_state.inertial_frame_position.y.to_string(),
            (-updated_state.inertial_frame_position.z).to_string(),
        ])?;
    }
    /*
    position_writer.flush()?;
    deflection_writer.flush()?;
    reference_writer.flush()?;
    yaw_writer.flush()?;
     */
    writer.flush()?;

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
