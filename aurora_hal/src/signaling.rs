use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

/// A type used for asynchronously waiting for a variable to fulfill certain conditions. Optimized
/// to require minimal CPU time, only checking the conditions if the variable has been changed.
#[derive(Clone)]
pub struct Signaling<T> {
    pair: Arc<(Condvar, Mutex<T>)>,
}

impl<T> Signaling<T>
where
    T: Clone,
{
    /// Initialize a new [Signaling] variable
    pub fn new(data: T) -> Self {
        Self {
            pair: Arc::new((Condvar::new(), Mutex::new(data))),
        }
    }

    /// Set the [Signaling] variable to a new value. This notifies all conditions that the value has
    /// changed.
    pub fn set(&self, new_val: T) {
        let (cvar, lock) = &*self.pair;
        let mut val = lock.lock().unwrap();
        *val = new_val;

        cvar.notify_all();
    }

    /// Return a copy of the internal value of the [Signaling] variable.
    pub fn get(&self) -> T {
        let (_, lock) = &*self.pair;
        let val = lock.lock().unwrap();

        val.clone()
    }

    /// Wait for a condition (predicate) to return true. This function blocks the thread it is in,
    /// but consumes no CPU time until the [Signaling::set] function has been called.
    pub fn wait_for<F>(&self, mut predicate: F) -> MutexGuard<T>
    where
        F: FnMut(&T) -> bool,
    {
        let (cvar, lock) = &*self.pair;

        // .wait_while waits while the predicate is true, while wait_for should wait
        // _until_ the predicate turns true, thus we need to invert the predicate
        cvar.wait_while(lock.lock().unwrap(), |val| !predicate(val))
            .unwrap()
    }
}

impl<T> Debug for Signaling<T>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signaling {{ {:?} }}", self.get())
    }
}

impl<T> Display for Signaling<T>
where
    T: Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signaling {{ {} }}", self.get())
    }
}

impl<T> Default for Signaling<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Signaling::new(T::default())
    }
}
