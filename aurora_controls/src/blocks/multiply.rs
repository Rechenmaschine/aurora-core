use crate::block::Block;

use std::iter::Product;

pub struct Multiplier {}

impl Multiplier {
    pub fn new() -> Self {
        Self { }
    }
}

impl<V, S> Block<V, S> for Multiplier where V: IntoIterator, S: Product<V::Item> {
    fn step(&mut self, x: V) -> S {
        return x.into_iter().product();
    }
}