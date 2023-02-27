use aurora_hal;
use std::sync::atomic::{AtomicU32, AtomicI16, Ordering};
use std::sync::RwLock;
use aurora_hal::{ArrayGetter, GetterSetter, RingBuffer, Value};


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


#[test]
fn callback_test() {
    static x: aurora_hal::Value<AtomicU32> = aurora_hal::Value::new(AtomicU32::new(0));
    let cb = aurora_hal::Condition{
        eval: Box::new(||{
            x.get() == 2
        })
    };
    x.register_callback(Box::new(||{
        x.set(100);
    }), cb);

    x.set(1);
    assert_eq!(x.get(), 1);
    x.set(2);
    assert_eq!(x.get(), 100);
}


#[test]
fn callback_in_thread() {
    static x: aurora_hal::Value<AtomicU32> = aurora_hal::Value::new(AtomicU32::new(0));
    let cb = aurora_hal::Condition{
        eval: Box::new(||{
            x.get() == 2
        })
    };
    x.register_callback(Box::new(||{
        x.set(100);
    }), cb);

    let thread1 = std::thread::spawn(||{
        x.set(10);
    });

    let thread2 = std::thread::spawn(||{
        while !(x.get() == 10) {}
        x.set(2);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    assert_eq!(x.get(), 100);
}


#[test]
fn getter_setter_with_history() {
    static x: Value<RwLock<RingBuffer<i32, 3>>> = aurora_hal::Value::new(RwLock::new(RingBuffer::from([0; 3])));
    x.set(1);
    assert_eq!(x.get(), 1);
    x.set(2);
    x.set(3);

    let y = x.get_array();
    assert_eq!(vec![1,2,3], y);

    x.set(4);
    let z = x.get_array();
    assert_eq!(vec![2,3,4], z);

}