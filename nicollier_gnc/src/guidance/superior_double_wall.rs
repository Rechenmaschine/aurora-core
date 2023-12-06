use nalgebra::Vector3;
use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct SuperiorDoubleWallGuidance{
    pub axis: Axis,
    pub first_wall: f64,
    pub second_wall: f64
}

pub enum Axis {
    X,
    Y
}


type Angle = f32;

type GuidanceOutput = f32;
impl SuperiorDoubleWallGuidance {
    pub fn new(axis: Axis, first_wall: f64, second_wall: f64) -> Self {
        Self { axis, first_wall, second_wall }
    }
    fn compute_zone(&mut self, state_inertial_frame_pos: Vector3<f64>) -> DoubleWallZone {
        match self.axis {
            Axis::X => {
                if state_inertial_frame_pos.x < self.first_wall {
                    DoubleWallZone::PastFirstWall
                } else if state_inertial_frame_pos.x > self.second_wall {
                    DoubleWallZone::PastSecondWall
                } else {
                    DoubleWallZone::BetweenWalls
                }
            }
            Axis::Y => {
                if state_inertial_frame_pos.y < self.first_wall {
                    DoubleWallZone::PastFirstWall
                } else if state_inertial_frame_pos.y > self.second_wall {
                    DoubleWallZone::PastSecondWall
                } else {
                    DoubleWallZone::BetweenWalls
                }
            }
        }
    }


    fn compute_target_heading(&mut self,pos: Vector3<f64>, target_coordinate: f64) -> Reference {
        // Step 1: Compute closest point on target wall
        let target_point = match self.axis{
            Axis::X => Vector3::new(target_coordinate, pos.y, pos.z),
            Axis::Y => Vector3::new(pos.x, target_coordinate, pos.z),
        };
        // Step 2: Compute vector between current pos and target point
        let direction_vector = target_point - pos;
        // Step 3: Compute Angle
        let angle = direction_vector.y.atan2(direction_vector.x);

        Reference(angle)
    }

}



    enum DoubleWallZone {
        BetweenWalls,
        PastFirstWall,
        PastSecondWall
    }

impl Guidance for SuperiorDoubleWallGuidance{
    type State = SystemState;
    type Reference = Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference {
        let current_zone = self.compute_zone( state.inertial_frame_position);

        match current_zone {
            DoubleWallZone::BetweenWalls => {
                return Reference(state.inertial_frame_angle.z);
            }
            DoubleWallZone::PastFirstWall => {
                return self.compute_target_heading(state.inertial_frame_position, self.second_wall);
            }
            DoubleWallZone::PastSecondWall => {
                return self.compute_target_heading(state.inertial_frame_position, self.first_wall);
            }
        }
    }
}
