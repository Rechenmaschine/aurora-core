#![allow(dead_code)]

use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::{atomic, RwLock};
use aurora_hal_macros::{add_fields, init_callbacks};
// use std::sync::atomic;
use atomic_traits::{Atomic};



struct Condition {
    eval: Box<dyn Fn() -> bool>,
}


struct Value<T> {
    val: T,
    callbacks: RwLock<Vec<(Condition, Box<dyn Fn()>)>>,
}

impl<T: Copy + Sync + Atomic> Value<T> {
    pub fn register_callback(&mut self, callback: Box<dyn Fn()>, condition: Condition) {
        self.callbacks.get_mut()
            .unwrap()
            .push((condition, callback));
    }

    pub fn set(&mut self, val: T::Type) {
        self.val.store(val, atomic::Ordering::Release);

        for (ref condition, callback) in self.callbacks.read().unwrap().deref() {
            if condition.eval.deref()() == true {
                callback.deref()();
            }
        }
    }

    pub fn get(&self) -> T {
        self.val.clone()
    }
}

#[derive(init_callbacks)]
#[add_fields] //TODO: Add the init_callbacks struct to this, or refactor so all is in one macro
struct CentralDataStruct {}
