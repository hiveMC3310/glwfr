//! # VAO Module

use crate::custom_errors::Errors;
pub struct Vao {
    id: gl::types::GLuint,
    index_count: Option<usize>,
}

impl Vao {
    /// Create a new vertex array object (VAO) and return a `Vao` instance wrapping it.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vao` instance if successful, or an error of type
    /// `Errors::OpenGlError` if there is an error generating the VAO.
    ///
    /// # Errors
    ///
    /// Returns an `Errors::OpenGlError` if the VAO cannot be generated.

    pub fn new() -> Result<Self, Errors> {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        if id == 0 {
            return Err(Errors::OpenGlError(
                "VAO creation failed".to_string(),
                gl::INVALID_OPERATION,
            ));
        }
        Ok(Self {
            id,
            index_count: None,
        })
    }

    /// Set the index count for the vertex array object (VAO).
    ///
    /// # Parameters
    ///
    /// * `count` - The number of indices to use for drawing the VAO.
    ///
    /// # Panics
    ///
    /// Panics if the index count has already been set for this VAO.
    pub fn set_index_count(&mut self, count: usize) {
        self.index_count = Some(count);
    }

    /// Returns the index count for the vertex array object (VAO).
    ///
    /// # Panics
    ///
    /// Panics if the index count has not been set for this VAO using `set_index_count`.
    pub fn index_count(&self) -> usize {
        self.index_count.expect("Index count not set for VAO")
    }

    /// Bind the Vertex Array Object (VAO).
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindVertexArray`.
    /// It binds the VAO to the current OpenGL context, making it the active VAO.

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbind the Vertex Array Object (VAO).
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindVertexArray(0)`.
    /// It unbinds the VAO from the current OpenGL context.
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Bind the Vertex Array Object (VAO) at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to bind the VAO to.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindVertexArray`.
    pub fn bind_at_index(&self, index: u32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindVertexBuffer(index, self.id, 0, 0);
        }
    }

    /// Unbind all Vertex Array Objects (VAOs).
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindVertexArray(0)`.
    pub fn unbind_all() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
