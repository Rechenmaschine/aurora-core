use aurora_hal;
use std::sync::atomic::{AtomicU32, AtomicI16, Ordering};
use std::sync::RwLock;
use aurora_hal::GetterSetter;


#[test]
fn getter_setter_atomic_test() {
    let x: aurora_hal::Value<AtomicU32> = aurora_hal::Value::new(AtomicU32::new(0));
    assert_eq!(x.get(), 0);
    x.set(5);
    assert_eq!(x.get(), 5);
    x.set(10);
    assert_eq!(x.get(), 10);

    let y: aurora_hal::Value<AtomicI16> = aurora_hal::Value::new(AtomicI16::new(0));
    assert_eq!(y.get(), 0);
    y.set(5);
    assert_eq!(y.get(), 5);
    y.set(10);
    assert_eq!(y.get(), 10);
}


#[test]
fn getter_setter_string_test() {
    let x: aurora_hal::Value<RwLock<String>> = aurora_hal::Value::new(RwLock::new("test1".to_string()));
    assert_eq!(x.get(), "test1");
    x.set("test2".to_string());
    assert_eq!(x.get(), "test2");
    x.set("test3".to_string());
    assert_eq!(x.get(), "test3");
}
