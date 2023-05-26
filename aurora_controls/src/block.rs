pub trait Block<I, O> {
    fn step(&mut self, input: I) -> O;
}