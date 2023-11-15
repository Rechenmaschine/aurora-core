use crate::iotree::IoTree;
use crate::signaling::Signaling;
use event_gen::event_generator::{EventGenHandle, EventGenerator};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

/// Event generator for IO Events.
pub struct IoEventGen<'io, IO, A, P, B> {
    tree: &'io IO,
    accessor: A,
    predicate: P,
    multi_shot: bool,
    event_builder: B,
}

/// Handle for IO Events
pub struct IoEventGenHandle {
    stop_flag: Arc<AtomicBool>,
}

impl EventGenHandle for IoEventGenHandle {
    fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst)
    }
}

impl<'io, T, IO, A, P, B, E> IoEventGen<'io, IO, A, P, B>
where
    T: 'static + Clone + Send,
    IO: IoTree + 'static,
    A: FnOnce(&'io IO) -> &'io Signaling<T>,
    P: Fn(&T) -> bool + Send + 'static,
    B: Fn(&T) -> E + Send + 'static,
{
    /// Creates a new event generator which is only meant to send an event once.
    pub fn new_single_shot(accessor: A, predicate: P, event_builder: B) -> Self {
        Self {
            tree: IO::get_tree(),
            accessor,
            predicate,
            multi_shot: false,
            event_builder,
        }
    }
}

impl<'io, T, IO, A, P, B, E> EventGenerator<E, ()> for IoEventGen<'io, IO, A, P, B>
where
    T: 'static + Clone + Send,
    A: FnOnce(&'io IO) -> &'io Signaling<T>,
    P: Fn(&T) -> bool + Send + 'static,
    B: Fn(&T) -> E + Send + 'static,
    E: Send + 'static,
{
    type Handle = IoEventGenHandle;

    /// Starts the IO Event generator, evaluating the condition in a separate thread.
    /// Currently only a single shot event generator is implemented. In that case, the
    /// generator destroys itself once it has sent its event.
    fn start(self, send_handle: Sender<E>) -> Self::Handle {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag2 = stop_flag.clone();

        match self.multi_shot {
            true => {
                unimplemented!()
            }
            false => {
                let signal_var = Signaling::clone((self.accessor)(self.tree));
                thread::spawn(move || {
                    let val = signal_var.wait_for(self.predicate);
                    if !stop_flag2.load(Ordering::SeqCst) {
                        send_handle
                            .send((self.event_builder)(&*val))
                            .expect("Failed to send event as other side has exited.");
                    }
                });
            }
        }

        Self::Handle { stop_flag }
    }
}

#[macro_export]
macro_rules! single_shot_io_event_gen {
    ($($accessor:ident).+, $predicate:expr, $event_builder: expr) => {
        IoEventGen::new_single_shot(|t: &crate::__iotree::__IoTree| &t.$($accessor).+, $predicate, $event_builder)
    };
}
