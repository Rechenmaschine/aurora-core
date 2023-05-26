use crate::block::Block;

use std::ops::Neg;

pub struct Negator {}

impl Negator {
    fn new() -> Self {
        Self { }
    }
}

impl<V, S> Block<V, S> for Negator where V: IntoIterator, S: FromIterator<<V::Item as Neg>::Output>, V::Item: Neg {
    fn step(&mut self, x: V) -> S {
        return S::from_iter(x.into_iter().map(|x| {-x}));
    }
}