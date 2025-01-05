//! # Graphics Module
//!
//! This module provides functionality for working with OpenGL, including window management,
//! texture loading, and shader management.
//!
//! ## Submodules
//! - **window**: Window creation and management.
//! - **texture**: Utilities for loading and managing textures.
//! - **gl_wrapper**: A wrapper for OpenGL functions.
//!
//! ## Example
//! ```rust
//! use glwfr::graphics::{window::Window, gl_wrapper::*};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors> {
//!     let mut window = Window::new(800, 600, "My Window")?;
//!     window.init_gl()?;
//!     window.enable_depth_test();
//!
//!     while !window.should_close() {
//!         window.clear(0.0, 0.0, 0.0, 1.0);
//!         window.update();
//!     }
//!     Ok(())
//! }
//! ```

pub mod gl_wrapper;
pub mod texture;
pub mod window;
