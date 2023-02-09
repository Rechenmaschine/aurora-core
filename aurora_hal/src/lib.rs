#![allow(dead_code)]

#[macro_use]
extern crate cfg_if;
mod atomic_traits;

use std::ops::Deref;
use std::sync::{atomic, RwLock};
use aurora_hal_macros::{add_fields, init_callbacks};
use atomic::{AtomicI64, AtomicU64, AtomicI32, AtomicU32, AtomicI16, AtomicU16, AtomicBool};
use atomic_float::{AtomicF32, AtomicF64};
use atomic_traits::{Atomic};


struct Condition {
    eval: Box<dyn Fn() -> bool>,
}


struct Value<T> {
    val: T,
    callbacks: RwLock<Vec<(Condition, Box<dyn Fn()>)>>,
}

impl<T: Sync + Atomic> Value<T> {
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

    pub fn get(&self) -> <T as Atomic>::Type {
        self.val.load(atomic::Ordering::Acquire)
    }
}

impl Value<RwLock<String>> {
    pub fn register_callback(&mut self, callback: Box<dyn Fn()>, condition: Condition) {
        self.callbacks.get_mut()
            .unwrap()
            .push((condition, callback));
    }

    pub fn set(&mut self, val: String) {
        let mut s = self.val.write().unwrap();
        *s = val;

        for (ref condition, callback) in self.callbacks.read().unwrap().deref() {
            if condition.eval.deref()() == true {
                callback.deref()();
            }
        }
    }

    pub fn get(&self) -> String {
        self.val.read().unwrap().deref().clone() //TODO: Maybe return a reference here? -> Problem with standardized access, we don't get a reference to the atomic variables
    }
}

#[add_fields]
#[derive(init_callbacks)]
struct CentralDataStruct {}
