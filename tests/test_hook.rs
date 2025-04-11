#![cfg(test)]
use std::{
    panic,
    sync::{Arc, LazyLock, Mutex},
};

use panic_log::{initialize_hook, Configuration};

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
        ..Default::default()
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
        ..Default::default()
    });
    let _ = panic::catch_unwind(|| panic!("Test"));

    assert_eq!(*ran_hook.lock().unwrap(), false);
}

#[test]
fn test_flush_logger() {
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
