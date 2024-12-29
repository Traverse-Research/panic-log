#![doc = include_str!("../README.md")]

use std::{backtrace, panic, thread};

pub struct Configuration {
    /// Always force capture a backtrace.
    ///
    /// If false, the presence of a backtrace will depend on the value of `RUST_BACKTRACE`.
    /// See [`std::backtrace::Backtrace`] for more info
    pub force_capture: bool,

    /// Keep the originally set panic hook, continuing any normal panic behaviour
    /// and custom panic behaviour set.
    pub keep_original_hook: bool,

    /// Run this callback after logging the panic.
    pub cleanup: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            force_capture: false,
            keep_original_hook: true,
            cleanup: None,
        }
    }
}

pub fn initialize_hook(config: Configuration) {
    let original_hook = if config.keep_original_hook {
        Some(panic::take_hook())
    } else {
        None
    };
    panic::set_hook(Box::new(move |info| {
        let thread_name = thread::current()
            .name()
            .unwrap_or("<unnamed thread>")
            .to_owned();

        let location = if let Some(panic_location) = info.location() {
            format!(
                "{}:{}:{}",
                panic_location.file(),
                panic_location.line(),
                panic_location.column()
            )
        } else {
            "<unknown location>".to_owned()
        };
        let message = info.payload().downcast_ref::<&str>().unwrap_or(&"");

        let backtrace = if config.force_capture {
            backtrace::Backtrace::force_capture()
        } else {
            backtrace::Backtrace::capture()
        };

        log::error!("thread '{thread_name}' panicked at {location}:\n{message}\nstack bactrace:\n{backtrace}");

        if let Some(cleanup) = config.cleanup.as_ref() {
            cleanup();
        }

        if let Some(original_hook) = &original_hook {
            original_hook(info);
        }
    }));
}
