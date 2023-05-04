#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate lazy_static;

mod atomic_traits;

use atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicU16, AtomicU32, AtomicU64};
use atomic_float::{AtomicF32, AtomicF64};
use atomic_traits::Atomic;
use aurora_hal_macros::{add_fields, derive_callbacks, Init};
use std::collections::HashMap;
use std::ops::Deref;
use std::string::ToString;
use std::sync::Mutex;
use std::sync::{atomic, RwLock};

type Callback = Box<dyn Fn() + Send + Sync>;
type Condition = Box<dyn Fn() -> bool + Send + Sync>;

pub struct RingBuffer<T, const N: usize> {
    buf: [T; N],
    ptr: usize,
}

impl<T: Clone, const N: usize> RingBuffer<T, N> {
    pub const fn new(val: [T; N]) -> RingBuffer<T, N> {
        RingBuffer { buf: val, ptr: 0 }
    }

    fn enqueue(&mut self, val: T) {
        self.buf[self.ptr] = val;
        self.ptr += 1;
        if self.ptr == N {
            self.ptr = 0;
        }
    }

    fn get_front(&self) -> T {
        if self.ptr > 0 {
            self.buf[self.ptr - 1].clone()
        } else {
            self.buf[N - 1].clone()
        }
    }
}

// This trait allows setting and getting values from the IoTree struct. All allowed types in the IoTree implement this trait.
// Retrieving information from the tree is done directly by using the get() function in this trait.
// To store a value in the IoTree, use the set! macro (defined at the bottom of this file). The set! macro also executes any callbacks associated with the value to be set
pub trait GetterSetter {
    type InnerType;
    fn set(&self, val: Self::InnerType);
    fn get(&self) -> Self::InnerType;
}

impl<T: Atomic> GetterSetter for T {
    type InnerType = <T as Atomic>::Type;
    fn set(&self, val: Self::InnerType) {
        self.store(val, atomic::Ordering::Release);
    }

    fn get(&self) -> Self::InnerType {
        self.load(atomic::Ordering::Acquire)
    }
}

impl GetterSetter for RwLock<String> {
    type InnerType = String;

    fn set(&self, val: Self::InnerType) {
        let mut x = self.write().unwrap();
        *x = val;
    }

    fn get(&self) -> Self::InnerType {
        self.read().unwrap().deref().clone()
    }
}

impl<T: Clone + Copy, const N: usize> GetterSetter for RwLock<RingBuffer<T, N>> {
    type InnerType = T;

    fn set(&self, val: Self::InnerType) {
        let mut x = self.write().unwrap();
        x.enqueue(val);
    }

    fn get(&self) -> Self::InnerType {
        let x = self.read().unwrap();
        x.get_front()
    }
}

// This Trait is used to return the contents of an entire RingBuffer. If required, an additional function can be implemented here to retrieve specific parts of the buffer
pub trait ArrayGetter {
    type InnerType;
    fn get_array(&self) -> Vec<Self::InnerType>;
}

impl<T: Copy, const N: usize> ArrayGetter for RwLock<RingBuffer<T, N>> {
    type InnerType = T;

    fn get_array(&self) -> Vec<Self::InnerType> {
        let x = self.read().unwrap();
        let mut ptr = x.ptr;
        let mut res: Vec<Self::InnerType> = Vec::new();
        res.reserve(N);

        let mut counter = 0;
        while counter < N {
            res.push(x.buf[ptr]);
            ptr += 1;
            if ptr == N {
                ptr = 0;
            }
            counter += 1;
        }
        res
    }
}

// These Structs are global. Callback chains are stored in the CALLBACKS struct, and the IOTREE struct is the data center for all data used during flight

// Three macros are associated with the IoTree.
// add_fields parses the IoTree.toml file and builds the nested structure of the IoTree struct.
// derive(Init) derives an initialization function for the IoTree struct
#[add_fields]
#[derive(Init)]
pub struct IoTree {}

lazy_static! {
    pub static ref CALLBACKS: Mutex<HashMap<String, Vec<(Condition, Callback)>>> = {
        let v: Vec<(Condition, Callback)> = Vec::new();
        Mutex::new(HashMap::from([("empty".to_string(), v)]))
    };
}

lazy_static! {
    pub static ref IOTREE: IoTree = IoTree::new();
}

// Generate the init_callbacks function, which adds all callbacks defined in the Callbacks.toml file to the CALLBACKS HashMap
derive_callbacks!();

// ============================ MACROS ==========================================================

#[macro_export]
macro_rules! set {
    ( $path:expr, $val:expr ) => {
        $path.set($val);
        if CALLBACKS
            .lock()
            .unwrap()
            .deref()
            .contains_key(stringify!($path))
        {
            for cb in CALLBACKS.lock().unwrap().get(stringify!($path)).unwrap() {
                if cb.0.deref()() == true {
                    cb.1.deref()();
                }
            }
        }
    };
}
