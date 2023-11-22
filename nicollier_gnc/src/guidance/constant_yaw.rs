use crate::guidance::Guidance;
use crate::{Reference, SystemState};

pub struct ConstantYawGuidance {
    desired_yaw: Reference,
}

impl ConstantYawGuidance {
    pub fn new(desired_yaw: Reference) -> Self {
        Self { desired_yaw }
    }
}

impl Guidance for ConstantYawGuidance {
    type State = SystemState;
    type Reference = Reference; // This should be Reference, not ()

    fn get_reference(&mut self, _state: Self::State) -> Self::Reference {
        // Just return the constant desired yaw angle.
        self.desired_yaw
    }
}
