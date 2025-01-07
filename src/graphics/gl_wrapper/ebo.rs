//! # EBO Module

use crate::custom_errors::Errors;
use std::mem;
use std::os::raw::*;

pub struct Ebo {
    id: gl::types::GLuint,
}

impl Ebo {
    /// Generate a new Element Buffer Object (EBO) and create an `Ebo` instance wrapping it.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Ebo` instance if successful, or an error of type
    /// `Errors::OpenGlError` if the EBO cannot be generated.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::OpenGlError` if the EBO cannot be generated.

    pub fn new() -> Result<Self, Errors> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        if id == 0 {
            return Err(Errors::OpenGlError(
                "Failed to generate EBO".to_string(),
                gl::INVALID_OPERATION,
            ));
        }
        Ok(Self { id })
    }

    /// Bind the Element Buffer Object (EBO) to the current OpenGL context, making it the active EBO.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBuffer(gl::ELEMENT_ARRAY_BUFFER, id)`.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    /// Unbind any Element Buffer Object (EBO) from the current OpenGL context, making no EBO active.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0)`.
    /// It unbinds any EBO from the current OpenGL context, making no EBO active.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    /// Store the given u32 slice in the Element Buffer Object (EBO).
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, usage)`.
    /// It stores the given u32 slice in the EBO.
    ///
    /// # Arguments
    ///
    /// * `indices` - The u32 slice to store in the EBO.
    pub fn store_indices(&self, indices: &[u32]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }

    /// Update the indices in the Element Buffer Object (EBO).
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in bytes from the start of the buffer.
    /// * `indices` - The indices to store in the buffer.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferSubData`.
    pub fn update_indices(&self, offset: usize, indices: &[u32]) {
        unsafe {
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                offset as isize,
                (indices.len() * mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const c_void,
            );
        }
    }
}
