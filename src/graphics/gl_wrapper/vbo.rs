//! # VBO Module

use crate::custom_errors::Errors;
use gl::types::*;
use std::mem;
use std::os::raw::*;

pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject {
    /// Generate a new buffer object (VBO, IBO, etc.) of the given type with the given usage.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The type of buffer object to generate. For example, `gl::ARRAY_BUFFER` or `gl::ELEMENT_ARRAY_BUFFER`.
    /// * `usage` - The usage of the buffer object. For example, `gl::STATIC_DRAW` or `gl::DYNAMIC_DRAW`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `BufferObject` instance if successful, or an error of type
    /// `Errors::OpenGlError` otherwise.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glGenBuffers(1, &mut id)` and `glBindBuffer(r#type, id)`.
    /// It generates a new buffer object of the given type with the given usage.
    pub fn new(r#type: GLenum, usage: GLenum) -> Result<Self, Errors> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        if id == 0 {
            return Err(Errors::OpenGlError(
                "Failed to generate buffer".to_string(),
                gl::INVALID_OPERATION,
            ));
        }
        Ok(Self { id, r#type, usage })
    }

    /// Bind the buffer object to the given OpenGL buffer binding point.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBuffer(r#type, id)`.
    /// It binds the buffer object to the current OpenGL context for the given buffer binding point.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    /// Unbind the buffer object from the current OpenGL context for the given buffer binding point.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBuffer(r#type, 0)`.
    /// It unbinds the buffer object from the current OpenGL context for the given buffer binding point.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    /// Store the given i32 slice in the buffer object.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferData(r#type, size, data, usage)`.
    /// It stores the given i32 slice in the buffer object.
    ///
    /// # Arguments
    ///
    /// * `data` - The i32 slice to store in the buffer object.
    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLint>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            )
        }
    }

    /// Store the given f32 slice in the buffer object.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferData(r#type, size, data, usage)`.
    /// It stores the given f32 slice in the buffer object.
    ///
    /// # Arguments
    ///
    /// * `data` - The f32 slice to store in the buffer object.
    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            )
        }
    }

    /// Store the given u32 slice in the buffer object.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferData(r#type, size, data, usage)`.
    /// It stores the given u32 slice in the buffer object.
    ///
    /// # Arguments
    ///
    /// * `data` - The u32 slice to store in the buffer object.
    pub fn store_u32_data(&self, data: &[u32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
                &data[0] as *const u32 as *const c_void,
                self.usage,
            )
        }
    }

    /// Update the data in the buffer object.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in bytes from the start of the buffer.
    /// * `data` - The data to store in the buffer.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferSubData`.
    pub fn update_data<T>(&self, offset: usize, data: &[T]) {
        unsafe {
            gl::BufferSubData(
                self.r#type,
                offset as isize,
                (data.len() * mem::size_of::<T>()) as isize,
                data.as_ptr() as *const c_void,
            );
        }
    }
}
