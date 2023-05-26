use crate::block::Block;

use std::iter::Sum;

pub struct Adder {}

impl Adder {
    fn new() -> Self {
        Self { }
    }
}

impl<V, S> Block<V, S> for Adder where V: IntoIterator, S: Sum<V::Item> {
    fn step(&mut self, x: V) -> S {
        return x.into_iter().sum();
    }
}