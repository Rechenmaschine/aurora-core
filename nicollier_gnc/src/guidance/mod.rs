use nalgebra::{Vector2, Vector3};

pub mod constant_guidance;





pub trait Guidance {
    type State;
    type Reference;

    fn get_reference(&mut self, state: Self::State) -> Self::Reference;
}
