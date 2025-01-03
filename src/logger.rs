//! # Logger
//!
//! This module provides a simple logger initialization using `env_logger`.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::logger;
//!
//! logger::init().expect("Failed to initialize logger");
//! log::info!("Logger initialized successfully!");
//! ```
//!
//! ## Setting Log Level
//!
//! You can set the log level using the `RUST_LOG` environment variable:
//!
//! ```bash
//! RUST_LOG=info cargo run
//! ```
//!
//! Supported log levels: `error`, `warn`, `info`, `debug`, `trace`.

/// Initializes the logger using `env_logger`.
///
/// # Errors
///
/// Returns an error if the logger has already been initialized.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp(None) // Optional: Customize the log format
        .try_init()
        .map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize logger: {}", e),
            )) as Box<dyn std::error::Error>
        })
}

pub use log::*;
