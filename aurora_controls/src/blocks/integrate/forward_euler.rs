use crate::blocks::integrate::Integrator;
use std::ops::{Mul, AddAssign};
use crate::block::UpdateBlock;

pub struct ForwardEulerIntegrator<V, T> {
    current_value: V,
    delta_step: T
}

impl<V, T> ForwardEulerIntegrator<V, T> {
    fn new(initial_value: V, delta_step: T) -> Self {
        Self {
            current_value: initial_value,
            delta_step
        }
    }
}

impl<V, T> UpdateBlock for ForwardEulerIntegrator<V, T>
    where T: Copy + Mul<V, Output = V>, V: Copy + AddAssign<V> {
    type InputType = V;
    type OutputType = V;

    fn update(&mut self, new_value: Self::InputType) -> Self::OutputType {
        self.current_value += self.delta_step.clone() * new_value;
        self.current_value.clone()
    }
}

impl<V, T> Integrator for ForwardEulerIntegrator<V, T> where Self: UpdateBlock {}