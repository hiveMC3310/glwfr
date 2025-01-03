//! # Texture Module
//!
//! This module provides a wrapper for creating, managing, and manipulating OpenGL textures.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::graphics::texture::Texture;
//! use glwfr::custom_errors::Errors;
//!
//! fn main() -> Result<(), Errors> {
//!     // Create a new texture
//!     let texture = Texture::new();
//!
//!     // Load texture from a file
//!     texture.load_from_file("path/to/texture.png")?;
//!
//!     // Bind the texture to texture unit 0
//!     texture.bind(gl::TEXTURE0);
//!
//!     // Set texture parameters
//!     texture.set_parameteri(gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//!     texture.set_parameteri(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//!
//!     Ok(())
//! }
//! ```

use crate::custom_errors::Errors;
use gl::types::*;
use image::ImageError;
use std::path::Path;

/// Represents an OpenGL texture.
///
/// This struct encapsulates an OpenGL texture object, providing methods to load,
/// bind, and configure textures.
pub struct Texture {
    id: GLuint,
}

impl Texture {
    /// Generate a new OpenGL texture handle and create a `Texture` instance wrapping it.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glGenTextures(1, &mut id)`.
    ///
    /// # Returns
    ///
    /// A new `Texture` instance with a valid OpenGL texture ID.
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self { id }
    }
    /// Bind the texture to the given active texture unit.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glActiveTexture(unit)` and `glBindTexture(GL_TEXTURE_2D, id)`.
    /// It binds the texture to the given active texture unit.
    ///
    /// # Arguments
    ///
    /// * `unit` - The active texture unit to bind the texture to.
    pub fn bind(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Set a parameter of the texture.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glTexParameteri(GL_TEXTURE_2D, param, value)`.
    /// It sets a parameter of the texture.
    ///
    /// # Arguments
    ///
    /// * `param` - The parameter to set. For example, `GL_TEXTURE_MIN_FILTER` or `GL_TEXTURE_WRAP_S`.
    /// * `value` - The value to set the parameter to. For example, `GL_LINEAR` or `GL_CLAMP_TO_EDGE`.
    pub fn set_parameteri(&self, param: GLenum, value: GLint) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }

    /// Set a parameter of the texture.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glTexParameterf(GL_TEXTURE_2D, param, value)`.
    /// It sets a parameter of the texture.
    ///
    /// # Arguments
    ///
    /// * `param` - The parameter to set. For example, `GL_TEXTURE_MIN_FILTER` or `GL_TEXTURE_WRAP_S`.
    /// * `value` - The value to set the parameter to. For example, `GL_LINEAR` or `GL_CLAMP_TO_EDGE`.
    pub fn set_parameterf(&self, param: GLenum, value: GLfloat) {
        unsafe {
            gl::TexParameterf(gl::TEXTURE_2D, param, value);
        }
    }

    /// Load a texture from a file and bind it to the texture object.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the image file to be loaded.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::TextureLoadError` if the image cannot be opened or processed.
    ///
    /// # OpenGL Functions
    ///
    /// This function binds the texture and uploads its data to the GPU using
    /// `glTexImage2D(GL_TEXTURE_2D, 0, gl::RGBA, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const _)`.
    /// It also generates mipmaps for the texture using `glGenerateMipmap(GL_TEXTURE_2D)`.

    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Errors> {
        let img = image::open(path).map_err(|e: ImageError| {
            Errors::TextureLoadError(format!("Failed to load texture: {}", e))
        })?;
        let img = img.to_rgba8();

        let (width, height) = img.dimensions();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(())
    }

    /// Loads a texture from raw data and uploads it to the GPU.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the texture.
    /// * `height` - The height of the texture.
    /// * `data` - The raw data of the texture. Must be in RGBA format with 8 bits per channel.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::TextureLoadError` if the data size is invalid.
    ///
    /// # OpenGL Functions
    ///
    /// This function binds the texture and uploads its data to the GPU using
    /// `glTexImage2D(GL_TEXTURE_2D, 0, gl::RGBA, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const _)`.
    /// It also generates mipmaps for the texture using `glGenerateMipmap(GL_TEXTURE_2D)`.
    pub fn load_from_data(&self, width: u32, height: u32, data: &[u8]) -> Result<(), Errors> {
        if data.len() != (width * height * 4) as usize {
            return Err(Errors::TextureLoadError(
                "Invalid data size for texture".to_string(),
            ));
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(())
    }
}

impl Drop for Texture {
    /// Automatically deletes the OpenGL texture when the `Texture` instance is dropped.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glDeleteTextures(1, &self.id)`.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
