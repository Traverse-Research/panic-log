#![cfg(test)]
use std::{
    panic,
    sync::{Arc, LazyLock, Mutex},
};

use panic_log::{initialize_hook, Configuration};

// The test binary runs all tests in parallel by default; this lets multiple tests overwrite the
// panic hook concurrently and cause spurious failure.
static SERIAL_TEST: Mutex<()> = Mutex::new(());

#[test]
#[should_panic]
fn test() {
    let _serial = SERIAL_TEST.lock().unwrap();

    initialize_hook(Configuration::default());

    // Drop the lock to not poison it
    drop(_serial);
    panic!("Test");
}

#[test]
#[should_panic]
fn test_forced_trace() {
    let _serial = SERIAL_TEST.lock().unwrap();

    initialize_hook(Configuration {
        force_capture: true,
        ..Default::default()
    });

    // Drop the lock to not poison it
    drop(_serial);
    panic!("Test");
}

#[test]
fn test_original_hook() {
    let _serial = SERIAL_TEST.lock().unwrap();

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
        ..Default::default()
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert!(*ran_hook.lock().unwrap());
}

#[test]
fn test_no_original_hook() {
    let _serial = SERIAL_TEST.lock().unwrap();

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
        ..Default::default()
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert!(!*ran_hook.lock().unwrap());
}

#[test]
fn test_flush_logger() {
    let _serial = SERIAL_TEST.lock().unwrap();

    struct Logger {
        pub flushed: Arc<Mutex<bool>>,
    }

    impl log::Log for Logger {
        fn enabled(&self, _metadata: &log::Metadata) -> bool {
            unimplemented!()
        }

        fn log(&self, _record: &log::Record) {
            unimplemented!()
        }

        fn flush(&self) {
            *self.flushed.lock().unwrap() = true;
        }
    }

    static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger {
        flushed: Arc::new(Mutex::new(false)),
    });

    initialize_hook(Configuration {
        force_capture: true,
        keep_original_hook: true,
        logger: Some(&*LOGGER),
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert!(*LOGGER.flushed.lock().unwrap());
}
