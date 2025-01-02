use crate::custom_errors::Errors;
use gl::types::*;
use image::ImageError;
use std::path::Path;

pub struct Texture {
    id: GLuint,
}

impl Texture {
    /// Generate a new OpenGL texture handle and create a `Texture` instance wrapping it.
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
    /// * `param` - The parameter to set. See the OpenGL documentation for the list of possible parameters.
    /// * `value` - The value to set the parameter to.
    pub fn set_parameters(&self, param: GLenum, value: GLint) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }

    /// Load the texture from the given file path.
    ///
    /// # Errors
    ///
    /// This function will return an error if the image cannot be loaded from the file.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glTexImage2D` and `glGenerateMipmap`.
    /// It loads the texture from the given file path and applies the texture to the current OpenGL texture unit.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to load the texture from.
    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Errors> {
        let img =
            image::open(path).map_err(|e: ImageError| Errors::TextureLoadError(e.to_string()))?;
        let img = img.to_rgba8();

        let (width, height) = img.dimensions();

        unsafe {
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

    /// Load a texture from raw data into the texture object.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glTexImage2D(GL_TEXTURE_2D, 0, gl::RGBA, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const _)`.
    /// It loads a texture from raw data into the texture object.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the texture in pixels.
    /// * `height` - The height of the texture in pixels.
    /// * `data` - The raw pixel data to load into the texture.

    pub fn load_from_data(&self, width: u32, height: u32, data: &[u8]) {
        unsafe {
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
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
