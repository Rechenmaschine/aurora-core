use crate::{Reference, SystemState, Deflections};

pub struct DoubleWallGuidance {
    wall_distance: f64,
    target_yaw: f64,
}

impl DoubleWallGuidance {
    pub fn new(wall_distance: f64) -> Self {
        Self {
            wall_distance,
            target_yaw: 0.0, // Initial target yaw angle
        }
    }

    pub fn get_reference(&mut self, state: &SystemState, input: &mut Deflections, delta_t: f64) -> Reference {
        let position_x = state.inertial_frame_position.x;
        let yaw = state.inertial_frame_angle.z;

        // Determine the target yaw based on the current position and target wall
        if position_x > self.wall_distance {
            // Turn towards the left wall
            self.target_yaw = -std::f64::consts::PI / 2.0;
        } else if position_x < -self.wall_distance {
            // Turn towards the right wall
            self.target_yaw = std::f64::consts::PI / 2.0;
        }

        // Gradually adjust the yaw towards the target yaw
        let yaw_error = self.target_yaw - yaw;
        input.asym += yaw_error * delta_t; // Adjust asym deflection based on error

        // Normalize the yaw_error to the range [-PI, PI]
        let normalized_yaw_error = if yaw_error > std::f64::consts::PI {
            yaw_error - 2.0 * std::f64::consts::PI
        } else if yaw_error < -std::f64::consts::PI {
            yaw_error + 2.0 * std::f64::consts::PI
        } else {
            yaw_error
        };

        Reference(normalized_yaw_error)
    }
}
