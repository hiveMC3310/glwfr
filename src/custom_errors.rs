//! # Custom Errors Module
//!
//! This module defines a set of custom error types for handling various failures in the library.
//! These errors are designed to provide clear and informative messages for debugging and error handling.
//!
//! ## Error Types
//! - `GlfwInitializationError`: Failed to initialize GLFW.
//! - `WindowCreationError`: Failed to create a window.
//! - `TextureLoadError`: Failed to load a texture.
//! - `ShaderCompilationError`: Failed to compile a shader.
//! - `ShaderLinkError`: Failed to link a shader program.
//! - `FileLoadError`: Failed to load a file.
//! - `OpenGlError`: An OpenGL-related error occurred.
//!
//! ## Usage
//! These errors can be returned by functions in the library to indicate specific failure conditions.
//! They implement the `std::error::Error` trait, making them compatible with Rust's error handling ecosystem.
//!
//! ### Example: Handling Errors
//! ```rust
//! use glwfr::custom_errors::Errors;
//!
//! fn load_texture(path: &str) -> Result<(), Errors> {
//!     if path.is_empty() {
//!         return Err(Errors::TextureLoadError("Empty path provided".to_string()));
//!     }
//!     // Simulate texture loading logic
//!     Ok(())
//! }
//!
//! fn main() {
//!     match load_texture("") {
//!         Ok(_) => println!("Texture loaded successfully!"),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

use thiserror::Error;

/// A custom error type for handling various failures in the library.
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Failed to initialize GLFW: {0}")]
    GlfwInitializationError(String),

    #[error("Failed to create window: {0}")]
    WindowCreationError(String),

    #[error("Failed to load texture: {0}")]
    TextureLoadError(String),

    #[error("Failed to compile shader: {0}\nShader source: {1}")]
    ShaderCompilationError(String, String),

    #[error("Failed to link shader program: {0}")]
    ShaderLinkError(String),

    #[error("Failed to load file: {0}")]
    FileLoadError(String),

    #[error("OpenGL error (code: {1}): {0}")]
    OpenGlError(String, u32),
}

impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Errors::FileLoadError(err.to_string())
    }
}

/// Checks for OpenGL errors and returns a custom error if any are found.
pub fn check_opengl_error() -> Result<(), Errors> {
    let error_code = unsafe { gl::GetError() };
    if error_code != gl::NO_ERROR {
        let error_message = match error_code {
            gl::INVALID_ENUM => "Invalid enum".to_string(),
            gl::INVALID_VALUE => "Invalid value".to_string(),
            gl::INVALID_OPERATION => "Invalid operation".to_string(),
            gl::OUT_OF_MEMORY => "Out of memory".to_string(),
            _ => format!("Unknown OpenGL error (code: {})", error_code),
        };
        return Err(Errors::OpenGlError(error_message, error_code));
    }
    Ok(())
}
