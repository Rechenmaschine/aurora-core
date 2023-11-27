use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct DoubleWallGuidance {
    nod_rate: f64,
    //total_time: f64,
    yaw_reference: f64,
    yaw_previous: f64,
    sym_deflection: f64,
    sym_deflection_previous: f64,
    target_wall: TargetWall,
    ramp_time: f64,
    stabilization_time: f64,
    braking_height: f64,
    steady_sym_def: f64,
}

impl DoubleWallGuidance {
    pub fn new(
        nod_rate: f64,
        ramp_time: f64,
        stabilization_time: f64,
        braking_height: f64,
        steady_sym_def: f64,
    ) -> Self {
        Self {
            nod_rate,
            yaw_reference: 0.0,
            yaw_previous: 0.0,
            sym_deflection: 0.0,
            sym_deflection_previous: 0.0,
            target_wall: TargetWall::North,
            ramp_time,
            stabilization_time,
            braking_height,
            steady_sym_def,
        }
    }

    fn generate_yaw_ramp(&mut self) {
        let yaw_tolerance = std::f64::consts::PI / (self.nod_rate * self.ramp_time);
        if (self.yaw_reference - self.yaw_previous) > yaw_tolerance {
            self.yaw_reference = self.yaw_previous + yaw_tolerance;
        } else if (self.yaw_reference - self.yaw_previous) < -yaw_tolerance {
            self.yaw_reference = self.yaw_previous - yaw_tolerance;
        }
        self.yaw_previous = self.yaw_reference;
    }

    fn generate_sym_ramp(&mut self) {
        let sym_tolerance = 1.0 / (self.nod_rate * self.ramp_time);
        if (self.sym_deflection - self.sym_deflection_previous) > sym_tolerance {
            self.sym_deflection = self.sym_deflection_previous + sym_tolerance;
        } else if (self.sym_deflection - self.sym_deflection_previous) < -sym_tolerance {
            self.sym_deflection = self.sym_deflection_previous - sym_tolerance;
        }
        self.sym_deflection_previous = self.sym_deflection;
    }

    fn target_wall_north(&mut self) {
        self.yaw_reference = 0.0 - std::f64::consts::PI / 16.0;
    }

    fn target_wall_south(&mut self) {
        self.yaw_reference = std::f64::consts::PI - std::f64::consts::PI / 16.0;
    }

    fn target_wall_east(&mut self) {
        self.yaw_reference = std::f64::consts::FRAC_PI_2;
    }

    fn target_wall_west(&mut self) {
        self.yaw_reference = -std::f64::consts::FRAC_PI_2;
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
        /*if total_time <= self.stabilization_time {
            self.sym_deflection = 0.0;
            println!("Stabilization phase ongoing");
        }
        // Braking phase
        else if state.inertial_frame_position.z >= self.braking_height {
            self.sym_deflection = 0.8; // Example braking value
            println!("Braking maneuver in progress");
        }*/
        // Normal guidance phase
        self.sym_deflection = self.steady_sym_def;

        // Wall targeting logic
        match self.target_wall {
            TargetWall::North => {
                if state.inertial_frame_position.y <= 0.0 {
                    self.target_wall = TargetWall::South;
                    self.target_wall_south();
                    println!("Switching to South wall");
                }
            }
            TargetWall::South => {
                if state.inertial_frame_position.y >= 0.0 {
                    self.target_wall = TargetWall::North;
                    self.target_wall_north();
                    println!("Switching to North wall");
                }
            }
            TargetWall::East => {
                if state.inertial_frame_position.x <= 0.0 {
                    self.target_wall = TargetWall::West;
                    self.target_wall_west();
                    println!("Switching to West wall");
                }
            }
            TargetWall::West => {
                if state.inertial_frame_position.x >= 0.0 {
                    self.target_wall = TargetWall::East;
                    self.target_wall_east();
                    println!("Switching to East wall");
                }
            }
        }

        Reference(self.yaw_reference)
    }
}
