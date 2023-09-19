pub trait Block<I, O> {
    fn step(&mut self, input: I) -> O;
}

pub trait UpdateBlock {
    type InputType;
    type OutputType;
    fn update(&mut self, new_value: Self::InputType) -> Self::OutputType;
}


impl<U> Block<U::InputType, U::OutputType> for U where U: UpdateBlock {
    fn step(&mut self, input: U::InputType) -> U::OutputType {
        self.update(input)
    }
}