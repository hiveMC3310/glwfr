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

/// Initializes the logger using `env_logger`.
///
/// # Errors
///
/// Returns an error if the logger has already been initialized.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init()?;
    Ok(())
}
pub use log::*;
