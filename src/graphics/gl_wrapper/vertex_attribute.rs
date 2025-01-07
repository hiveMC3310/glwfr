//! # VertexAttribute Module

use gl::types::*;
use std::os::raw::*;

pub struct VertexAttribute {
    index: gl::types::GLuint,
}

impl VertexAttribute {
    /// Create a new VertexAttribute and enable it on the given index.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glVertexAttribPointer(index, size, type, normalized, stride, pointer)`.
    /// It creates a new VertexAttribute and enables it on the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the vertex attribute to enable.
    /// * `size` - The number of components of the vertex attribute.
    /// * `r#type` - The type of the vertex attribute.
    /// * `normalized` - Whether the vertex attribute is normalized.
    /// * `stride` - The stride of the vertex attribute.
    /// * `pointer` - The pointer to the vertex attribute data.
    ///
    /// # Returns
    ///
    /// A `VertexAttribute` instance with the given index.
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute { index }
    }

    /// Enable the vertex attribute at the given index.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glEnableVertexAttribArray(index)`.
    /// It enables the vertex attribute at the given index.
    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    /// Disable the vertex attribute at the given index.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glDisableVertexAttribArray(index)`.
    /// It disables the vertex attribute at the given index.
    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
