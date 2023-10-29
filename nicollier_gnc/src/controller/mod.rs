pub mod p_controller;

pub trait Controller {
    type State;
    type Reference;
    type Output;

    fn step(
        &mut self,
        state: Self::State,
        reference: Self::Reference,
        delta_t: f64,
    ) -> Self::Output;
}