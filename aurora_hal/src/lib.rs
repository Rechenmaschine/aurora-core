#![allow(dead_code)]
#![allow(unconditional_recursion)]
#![allow(unused_imports)]

#[macro_use]
extern crate cfg_if;
mod atomic_traits;

use std::ops::Deref;
use std::sync::{atomic, RwLock};
use aurora_hal_macros::{add_fields, Callbacks};
use atomic::{AtomicI64, AtomicU64, AtomicI32, AtomicU32, AtomicI16, AtomicU16, AtomicBool};
use atomic_float::{AtomicF32, AtomicF64};
use atomic_traits::{Atomic};
use std::sync::Arc;


pub struct Condition {
    eval: Box<dyn Fn() -> bool>,
}


pub struct Value<T> {
    val: T,
    callbacks: RwLock<Vec<(Condition, Box<dyn Fn()>)>>,
}


pub trait GetterSetter {
    type InnerType;
    fn set(&self, val: Self::InnerType);
    fn get(&self)-> Self::InnerType;
}


impl<T: Atomic> GetterSetter for Value<T> {
    type InnerType = <T as Atomic>::Type;

    fn set(&self, val: Self::InnerType) {
        self.val.store(val, atomic::Ordering::Release);

        for (ref condition, callback) in self.callbacks.read().unwrap().deref() {
            if condition.eval.deref()() == true {
                callback.deref()();
            }
        }
    }

    fn get(&self) -> Self::InnerType {
        self.val.load(atomic::Ordering::Acquire)
    }
}


impl GetterSetter for Value<RwLock<String>> {
    type InnerType = String;

    fn set(&self, val: Self::InnerType) {
        let mut s = self.val.write().unwrap();
        *s = val;

        for (condition, callback) in self.callbacks.read().unwrap().deref() {
            if condition.eval.deref()() == true {
                callback.deref()();
            }
        }
    }

    fn get(&self) -> Self::InnerType {
        self.val.read().unwrap().deref().clone()
    }
}


impl<T> Value<T> {
    pub fn new(val: T) -> Value<T> {
        let x: Vec<(Condition, Box<dyn Fn()>)> = Vec::new();
        Value {
            val,
            callbacks: RwLock::new(x),
        }
    }

    pub fn register_callback(&self, callback: Box<dyn Fn()>, condition: Condition) {
        self.callbacks.write()
            .unwrap()
            .push((condition, callback));
    }
}



#[add_fields]
#[derive(Callbacks)]
struct CentralDataStruct {}
