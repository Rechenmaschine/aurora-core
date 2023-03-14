use std::sync::mpsc::Receiver;

pub trait State<E, I>
where
    I: StateIdentifier<E>,
{
    fn handle_event(&mut self, event: E) -> Option<I>;

    fn construct_successor(&mut self, next_state_ident: I) -> Box<dyn State<E, I>>;

    fn create_event_sources(&mut self) -> Receiver<E>;

    fn destroy_event_sources(&mut self);

    fn identifier(&self) -> I;

    fn enter(&mut self, prev_state_ident: I, prev_state: Box<dyn State<E, I>>);

    fn exit(&mut self);
}

pub trait StateIdentifier<E>: Sized {
    fn transition_from(
        self,
        mut prev_state: Box<dyn State<E, Self>>,
    ) -> (Box<dyn State<E, Self>>, Receiver<E>) {
        let prev_state_ident = prev_state.identifier();
        let mut next_state = self.construct_state();

        prev_state.exit();
        prev_state.destroy_event_sources();
        let recv = next_state.create_event_sources();
        next_state.enter(prev_state_ident, prev_state);

        (next_state, recv)
    }

    fn construct_state(self) -> Box<dyn State<E, Self>>;
}
