#![doc = include_str!("../README.md")]

use std::{backtrace, panic, thread};

pub struct Configuration {
    pub force_capture: bool,
    pub keep_original_hook: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            force_capture: false,
            keep_original_hook: true,
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

        if let Some(original_hook) = &original_hook {
            original_hook(info);
        }
    }));
}
