//! Logging helper.

use std::env;

use colored::Colorize;
use log::info;

/// Setup global logger.
///
/// If the user wants to enable `debug_mode`, then we set the environment variable `LOG` to `trace`,
/// the most verbose logging level.
///
/// # Panics
///
/// Logging setup panics if `pretty_env_logger` fails to initiate a global instance.
pub fn setup(debug_mode: bool) {
    if debug_mode {
        env::set_var("LOG", "trace");
    }

    if let Err(_) = env::var("LOG") {
        // Default to `info` level if the user did not specify.
        env::set_var("LOG", "INFO");
    }

    // Instead of `RUST_LOG`, use `LOG` environment variable name instead.
    pretty_env_logger::init_custom_env("LOG");

    info!(
        "logging level is {}",
        env::var("LOG").unwrap().to_uppercase().blue()
    );
}
