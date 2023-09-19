use crate::blocks::differentiate::Differentiator;
use std::ops::{Sub, Div};
use crate::block::UpdateBlock;

pub struct BackwardFiniteDifferenceDifferentiator<V, T> {
    current_value: V,
    delta_step: T
}

impl<V, T> BackwardFiniteDifferenceDifferentiator<V, T> {
    fn new(initial_value: V, delta_step: T) -> Self {
        Self {
            current_value: initial_value,
            delta_step
        }
    }
}


impl<V, T> UpdateBlock for BackwardFiniteDifferenceDifferentiator<V, T>
    where T: Copy, V: Copy + Sub<Output = V> + Div<T, Output = V>{
    type InputType = V;
    type OutputType = V;

    fn update(&mut self, new_value: Self::InputType) -> Self::OutputType {
        let differential = (new_value - self.current_value) / self.delta_step;
        self.current_value = new_value;
        differential
    }
}

impl<V, T> Differentiator for BackwardFiniteDifferenceDifferentiator<V, T> where Self: UpdateBlock {}