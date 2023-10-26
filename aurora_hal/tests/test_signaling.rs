use aurora_hal::signaling::Signaling;
use std::thread;

#[test]
fn test_get_function() {
    let sig = Signaling::new(false);
    assert_eq!(sig.get(), false);
}

#[test]
fn test_set_function() {
    let sig = Signaling::new(false);
    sig.set(true);
    assert_eq!(sig.get(), true);
}

#[test]
fn test_wait_function() {
    let sig = Signaling::new(0);
    let sig2 = sig.clone();
    let t = thread::spawn(move || {
        sig2.set(1);
    });

    // Wait for the value to be set
    let _val = sig.wait_for(|i| *i == 1);
    t.join().unwrap();
}
