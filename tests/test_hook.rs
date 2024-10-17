#![cfg(test)]
use std::{
    panic,
    sync::{Arc, Mutex},
};

use rust_template::{initialize_hook, Configuration};

#[test]
#[should_panic]
fn test() {
    initialize_hook(Configuration::default());
    panic!("Test");
}

#[test]
#[should_panic]
fn test_forced_trace() {
    initialize_hook(Configuration {
        force_capture: true,
        ..Default::default()
    });
    panic!("Test");
}

#[test]
fn test_original_hook() {
    let original_hook = panic::take_hook();
    let ran_hook = Arc::new(Mutex::new(false));
    let ran_hook_copy = Arc::clone(&ran_hook);
    panic::set_hook(Box::new(move |info| {
        *ran_hook_copy.lock().unwrap() = true;
        original_hook(info);
    }));

    initialize_hook(Configuration {
        force_capture: true,
        keep_original_hook: true,
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert_eq!(*ran_hook.lock().unwrap(), true);
}

#[test]
fn test_no_original_hook() {
    let original_hook = panic::take_hook();
    let ran_hook = Arc::new(Mutex::new(false));
    let ran_hook_copy = Arc::clone(&ran_hook);
    panic::set_hook(Box::new(move |info| {
        *ran_hook_copy.lock().unwrap() = true;
        original_hook(info);
    }));

    initialize_hook(Configuration {
        force_capture: true,
        keep_original_hook: false,
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert_eq!(*ran_hook.lock().unwrap(), false);
}
