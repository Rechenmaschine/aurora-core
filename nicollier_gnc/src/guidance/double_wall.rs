use crate::guidance::Guidance;
use crate::{Reference, SystemState};


pub struct DoubleWallGuidance {
    yaw_reference: f64,
    sym_deflection: f64,
    target_wall: TargetWall,
    stabilization_time: f64,
    braking_height: f64,
    north_wall_x: f64,
    //100.0
    south_wall_x: f64,
    //-100.0
    east_wall_y: f64,
    //100.0
    west_wall_y: f64,
    //-100.0
}

impl DoubleWallGuidance {
    pub fn new(
        stabilization_time: f64,
        braking_height: f64,
        north_wall_x: f64,
        south_wall_x: f64,
        east_wall_y: f64,
        west_wall_y: f64,
    ) -> Self {
        Self {
            yaw_reference: std::f64::consts::FRAC_PI_2 - std::f64::consts::PI / 16.0,
            sym_deflection: 0.0,
            target_wall: TargetWall::East,
            stabilization_time,
            braking_height,
            north_wall_x,
            south_wall_x,
            east_wall_y,
            west_wall_y,
        }
    }



    fn target_wall_north(&mut self) {
        self.yaw_reference =  std::f64::consts::PI / 16.0;
    }

    fn target_wall_south(&mut self) {
        self.yaw_reference = std::f64::consts::PI - std::f64::consts::PI / 16.0;
    }

    fn target_wall_east(&mut self) {
        self.yaw_reference = std::f64::consts::FRAC_PI_2 - std::f64::consts::PI / 16.0;
    }

    fn target_wall_west(&mut self) {
        self.yaw_reference = std::f64::consts::PI+std::f64::consts::FRAC_PI_2 + std::f64::consts::PI / 16.0;
    }
}
pub enum TargetWall {
    North,
    South,
    East,
    West,
}

impl Guidance for DoubleWallGuidance {
    type State = SystemState;
    type Reference = Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference {

        // Stabilization phase
        if state.total_time <= self.stabilization_time {
            self.sym_deflection = 0.0;
            println!("Stabilization phase ongoing");
        }
        // Braking phase
        else if state.inertial_frame_position.z >= self.braking_height {
            self.sym_deflection = 0.8; // Example braking value
            println!("Braking maneuver in progress");
        }
        // Normal guidance phase

        // Wall targeting logic
        match self.target_wall {
            TargetWall::North => {
                if state.inertial_frame_position.x >= self.north_wall_x {
                    self.target_wall = TargetWall::South;
                    self.target_wall_south();
                    println!("Switching to South wall");
                }
            }
            TargetWall::South => {
                if state.inertial_frame_position.x <= self.south_wall_x {
                    self.target_wall = TargetWall::North;
                    self.target_wall_north();
                    println!("Switching to North wall");
                }
            }
            TargetWall::East => {
                if state.inertial_frame_position.y >= self.east_wall_y {
                    self.target_wall = TargetWall::West;
                    self.target_wall_west();
                    println!("Switching to West wall");
                }
            }
            TargetWall::West => {
                if state.inertial_frame_position.y <= self.west_wall_y {
                    self.target_wall = TargetWall::East;
                    self.target_wall_east();
                    println!("Switching to East wall");
                }
            }
        }

        Reference(self.yaw_reference)
    }
}
