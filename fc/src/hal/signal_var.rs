use std::sync::{Arc, Condvar, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Signaling<T> {
    pair: Arc<(Condvar, Mutex<T>)>,
}

impl<T> Signaling<T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Self {
            pair: Arc::new((Condvar::new(), Mutex::new(data))),
        }
    }

    pub fn set(&self, new_val: T) {
        let (cvar, lock) = &*self.pair;
        let mut val = lock.lock().unwrap();
        *val = new_val;

        cvar.notify_all();
    }

    pub fn get(&self) -> T {
        let (_, lock) = &*self.pair;
        let val = lock.lock().unwrap();

        val.clone()
    }

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
