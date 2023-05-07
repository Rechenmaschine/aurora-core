use aurora_hal;
use aurora_hal::{set, ArrayGetter, GetterSetter, RingBuffer, CALLBACKS};
use std::ops::Deref;
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::{AtomicI16, AtomicU32};
use std::sync::RwLock;

#[test]
fn getter_setter_atomic_test() {
    let x: AtomicU32 = AtomicU32::new(0);
    assert_eq!(x.get(), 0);
    x.set(5);
    assert_eq!(x.get(), 5);
    x.set(10);
    assert_eq!(x.get(), 10);

    let y: AtomicI16 = AtomicI16::new(0);
    assert_eq!(y.get(), 0);
    y.set(5);
    assert_eq!(y.get(), 5);
    y.set(10);
    assert_eq!(y.get(), 10);
}

#[test]
fn getter_setter_string_test() {
    let x: RwLock<String> = RwLock::new("test1".to_string());
    assert_eq!(x.get(), "test1");
    x.set("test2".to_string());
    assert_eq!(x.get(), "test2");
    x.set("test3".to_string());
    assert_eq!(x.get(), "test3");
}

#[test]
fn set_macro_test() {
    static X: AtomicU32 = AtomicU32::new(0);

    set!(X, 1);
    assert_eq!(X.get(), 1);
}

#[test]
fn callback_test() {
    static Y: AtomicU32 = AtomicU32::new(0);
    let mut v: Vec<(
        Box<dyn Fn() -> bool + Send + Sync>,
        Box<dyn Fn() + Send + Sync>,
    )> = Vec::new();
    v.push((
        Box::new(|| true),
        Box::new(|| {
            Y.store(5, Release);
        }),
    ));
    CALLBACKS.lock().unwrap().insert("Y".to_string(), v);

    set!(Y, 1);
    assert_eq!(Y.get(), 5);
}

#[test]
fn callback_in_thread() {
    static Z: AtomicU32 = AtomicU32::new(0);
    {
        let mut v: Vec<(
            Box<dyn Fn() -> bool + Send + Sync>,
            Box<dyn Fn() + Send + Sync>,
        )> = Vec::new();
        v.push((
            Box::new(|| true),
            Box::new(|| {
                Z.store(5, Release);
            }),
        ));
        CALLBACKS.lock().unwrap().insert("Z".to_string(), v);
    }

    let thread1 = std::thread::spawn(|| {
        set!(Z, 1);
    });
    thread1.join().unwrap();
    assert_eq!(Z.get(), 5);
}

#[test]
fn getter_setter_with_history() {
    let ring = RingBuffer::new([2; 3]);
    assert_eq!(ring.get_front(), 2);
    static X: RwLock<RingBuffer<i32, 3>> = RwLock::new(RingBuffer::new([0; 3]));
    assert_eq!(vec![0, 0, 0], X.get_array());
    X.set(1);
    assert_eq!(X.get(), 1);
    X.set(2);
    X.set(3);
    assert_eq!(X.get(), 3);

    let y = X.get_array();
    assert_eq!(vec![1, 2, 3], y);

    X.set(4);
    let z = X.get_array();
    assert_eq!(vec![2, 3, 4], z);
}

#[test]
fn callback_static() {
    let mut v: Vec<(
        Box<dyn Fn() -> bool + Send + Sync>,
        Box<dyn Fn() + Send + Sync>,
    )> = Vec::new();
    v.push((Box::new(|| true), Box::new(|| println!("Test succeeded!"))));
    CALLBACKS.lock().unwrap().insert("test".to_string(), v);
}
