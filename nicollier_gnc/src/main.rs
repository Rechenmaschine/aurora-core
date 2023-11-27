use anyhow::Result;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;
use std::ops::{Deref, DerefMut};

use nicollier_gnc::controller::Controller;
use nicollier_gnc::guidance::Guidance;
use nicollier_gnc::model::Model;
use nicollier_gnc::Simulation;

fn main() -> Result<()> {
    let mut writer = Writer::from_path("all.csv")?;

    let mut sim = Simulation::new();

    while !sim.done()
    /*&& model.get_state().total_time<1000.0*/
    {
        let (state, reference, control_inputs, updated_state) = sim.step();

        writer.write_record(&[
            state.total_time.to_string(),
            control_inputs.asym.to_string(),
            reference.0.to_string(),
            updated_state.inertial_frame_angle.z.to_string(),
            updated_state.inertial_frame_position.x.to_string(),
            updated_state.inertial_frame_position.y.to_string(),
            (-updated_state.inertial_frame_position.z).to_string(),
        ])?;
    }

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
