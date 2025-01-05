//! # Custom Errors Module
//!
//! This module defines custom error types for handling various failures in the library.
//!
//! ## Error Types
//! - **GlfwInitializationError**: Failed to initialize GLFW.
//! - **WindowCreationError**: Failed to create a window.
//! - **TextureLoadError**: Failed to load a texture.
//! - **ShaderCompilationError**: Failed to compile a shader.
//! - **ShaderLinkError**: Failed to link a shader program.
//! - **FileLoadError**: Failed to load a file.
//! - **OpenGlError**: OpenGL-related errors.
//! - **AudioInitializationError**: Failed to initialize the audio system.
//! - **SoundLoadError**: Failed to load a sound file.
//! - **SoundPlayError**: Failed to play a sound.
//! - **SoundNotFoundError**: Sound not found in the audio system.
//! - **AudioDecodeError**: Failed to decode an audio file.
//! - **AudioVolumeError**: Failed to set audio volume.
//!
//! ## Example
//! ```rust
//! use glwfr::custom_errors::Errors;
//!
//! fn load_texture(path: &str) -> Result<(), Errors> {
//!     if path.is_empty() {
//!         return Err(Errors::TextureLoadError("Empty path provided".to_string()));
//!     }
//!     // Load texture logic...
//!     Ok(())
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

    #[error("Failed to initialize audio system: {0}")]
    AudioInitializationError(String),

    #[error("Failed to load sound file: {0}")]
    SoundLoadError(String),

    #[error("Failed to play sound: {0}")]
    SoundPlayError(String),

    #[error("Sound not found: {0}")]
    SoundNotFoundError(String),

    #[error("Failed to decode audio file: {0}")]
    AudioDecodeError(String),

    #[error("Failed to set audio volume: {0}")]
    AudioVolumeError(String),
}

impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Errors::FileLoadError(err.to_string())
    }
}

impl From<rodio::decoder::DecoderError> for Errors {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Errors::AudioDecodeError(err.to_string())
    }
}

impl From<rodio::PlayError> for Errors {
    fn from(err: rodio::PlayError) -> Self {
        Errors::SoundPlayError(err.to_string())
    }
}

impl From<rodio::StreamError> for Errors {
    fn from(err: rodio::StreamError) -> Self {
        Errors::AudioInitializationError(err.to_string())
    }
}

impl From<rodio::DevicesError> for Errors {
    fn from(err: rodio::DevicesError) -> Self {
        Errors::AudioInitializationError(err.to_string())
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
