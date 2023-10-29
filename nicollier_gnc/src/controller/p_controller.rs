use crate::controller::Controller;
use crate::{Deflections, Reference, SystemState};

pub struct PController ();

impl PController {
    pub fn new() -> Self {
        Self()
    }
}

impl Controller for PController {
    type State = SystemState;
    type Reference = Reference;
    type Output = Deflections;

    fn step(
        &mut self,
        state: Self::State,
        reference: Self::Reference,
        delta_t: f64,
    ) -> Self::Output {
        todo!()
    }
}
