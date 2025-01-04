//! # GL Wrapper Module
//!
//! This module provides wrappers for OpenGL objects such as VAO, VBO, EBO, and shader programs.
//!
//! ## Usage
//!
//! ```rust
//! use glwfr::graphics::gl_wrapper::{Vao, BufferObject, ShaderProgram};
//!
//! fn main() -> Result<(), glwfr::custom_errors::Errors> {
//!     let vao = Vao::new()?;
//!     vao.bind();
//!
//!     let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW)?;
//!     vbo.bind();
//!     vbo.store_f32_data(&[0.0, 0.0, 1.0, 1.0]);
//!
//!     let shader_program = ShaderProgram::new("vertex.glsl", "fragment.glsl")?;
//!     shader_program.bind();
//!
//!     Ok(())
//! }
//! ```

use crate::custom_errors::Errors;
use cgmath::*;
use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::os::raw::*;

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

pub struct ShaderProgram {
    program_handle: u32,
    uniform_ids: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram {
    /// Compile two shaders and link them into a shader program.
    ///
    /// # Errors
    ///
    /// This function will return an error if the shaders cannot be compiled or linked.
    ///
    /// # Arguments
    ///
    /// * `vertex_shader_path` - The path to the vertex shader source file.
    /// * `fragment_shader_path` - The path to the fragment shader source file.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ShaderProgram` instance if successful, or an error of type
    /// `Errors::ShaderCompilationError` or `Errors::ShaderLinkError` otherwise.
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, Errors> {
        let vertex_shader = Self::compile_shader(vertex_path, gl::VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(fragment_path, gl::FRAGMENT_SHADER)?;

        let program_handle = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_len = 0;
            unsafe {
                gl::GetProgramiv(program_handle, gl::INFO_LOG_LENGTH, &mut log_len);
            }
            let mut log = vec![0; log_len as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    program_handle,
                    log_len,
                    std::ptr::null_mut(),
                    log.as_mut_ptr() as *mut i8,
                );
            }
            return Err(Errors::ShaderLinkError(
                String::from_utf8_lossy(&log).to_string(),
            ));
        }

        Ok(Self {
            program_handle,
            uniform_ids: HashMap::new(),
        })
    }

    /// Compile a shader from a file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the shader source file cannot be read or if the shader
    /// cannot be compiled.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the shader source file.
    /// * `shader_type` - The type of shader to compile (e.g. `gl::VERTEX_SHADER`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the OpenGL shader handle if successful, or an error of type
    /// `Errors::ShaderCompilationError` otherwise.
    fn compile_shader(path: &str, shader_type: GLenum) -> Result<GLuint, Errors> {
        let mut shader_file = File::open(path).map_err(|e| Errors::FileLoadError(e.to_string()))?;
        let mut shader_source = String::new();
        shader_file
            .read_to_string(&mut shader_source)
            .map_err(|e| Errors::FileLoadError(e.to_string()))?;

        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(shader_source.as_bytes()).map_err(|e| {
            Errors::ShaderCompilationError("Failed to create CString".to_string(), e.to_string())
        })?;

        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }

        let mut success = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_len = 0;
            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
            }
            let mut log = vec![0; log_len as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    log_len,
                    std::ptr::null_mut(),
                    log.as_mut_ptr() as *mut i8,
                );
            }
            return Err(Errors::ShaderCompilationError(
                "Shader compilation failed".to_string(),
                String::from_utf8_lossy(&log).to_string(),
            ));
        }

        Ok(shader)
    }

    /// Bind the shader program to the current OpenGL context.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUseProgram(program_handle)`.
    /// It binds the shader program to the current OpenGL context.
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    /// Unbind any shader program from the current OpenGL context, making no shader program active.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUseProgram(0)`.
    /// It unbinds any shader program from the current OpenGL context, making no shader program active.
    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Retrieve the location of a uniform variable within the shader program.
    ///
    /// This function first checks if the uniform location is cached in `uniform_ids`.
    /// If the location is not cached, it queries OpenGL for the location of the uniform
    /// variable with the given `name` and caches the result.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable whose location is to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing the location of the uniform variable as a `GLint` if successful,
    /// or an error of type `Errors::OpenGlError` if the uniform variable is not found or if
    /// there is an error converting the name to a `CString`.

    pub fn get_uniform_location(&mut self, name: &str) -> Result<GLint, Errors> {
        if let Some(&location) = self.uniform_ids.get(name) {
            Ok(location)
        } else {
            let c_name = CString::new(name)
                .map_err(|e| Errors::OpenGlError(e.to_string(), gl::INVALID_VALUE))?;
            let location = unsafe { gl::GetUniformLocation(self.program_handle, c_name.as_ptr()) };
            if location < 0 {
                Err(Errors::OpenGlError(
                    format!("Uniform '{}' not found", name,),
                    gl::UNIFORM,
                ))
            } else {
                self.uniform_ids.insert(name.to_string(), location);
                Ok(location)
            }
        }
    }

    /// Create a uniform block and bind it to the specified binding point.
    ///
    /// # Arguments
    ///
    /// * `block_name` - The name of the uniform block in the shader.
    /// * `binding_point` - The binding point to bind the uniform block to.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if successful, or an error of type `Errors::OpenGlError` otherwise.
    pub fn create_uniform_block(&self, block_name: &str, binding_point: u32) -> Result<(), Errors> {
        let c_name = CString::new(block_name)
            .map_err(|e| Errors::OpenGlError(e.to_string(), gl::INVALID_VALUE))?;

        let block_index = unsafe { gl::GetUniformBlockIndex(self.program_handle, c_name.as_ptr()) };
        if block_index == gl::INVALID_INDEX {
            return Err(Errors::OpenGlError(
                format!("Uniform block '{}' not found", block_name),
                gl::INVALID_VALUE,
            ));
        }

        unsafe {
            gl::UniformBlockBinding(self.program_handle, block_index, binding_point);
        }

        Ok(())
    }

    /// Set the value of a uniform variable of type `f32`.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniform1f(location, value)`.
    /// It sets the value of a uniform variable of type `f32`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable to set.
    /// * `value` - The value to set the uniform variable to.
    ///
    /// # Returns
    ///
    /// A `Result` containing a value of type `()` if successful, or an error of type
    /// `Errors::OpenGlError` if there is an error setting the uniform variable.
    pub fn set_uniform_1f(&mut self, name: &str, value: f32) -> Result<(), Errors> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::Uniform1f(location, value);
        }
        Ok(())
    }

    /// Set the value of a uniform variable of type `i32`.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniform1f(location, value)`.
    /// It sets the value of a uniform variable of type `i32`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable to set.
    /// * `value` - The value to set the uniform variable to.
    ///
    /// # Returns
    ///
    /// A `Result` containing a value of type `()` if successful, or an error of type
    /// `Errors::OpenGlError` if there is an error setting the uniform variable.
    pub fn set_uniform_1i(&mut self, name: &str, value: i32) -> Result<(), Errors> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::Uniform1i(location, value);
        }
        Ok(())
    }

    /// Set the value of a uniform variable of type `vec3` (three f32 components).
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniform3f(location, x, y, z)`.
    /// It sets the value of a uniform variable of type `vec3`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable to set.
    /// * `x` - The x component of the vector.
    /// * `y` - The y component of the vector.
    /// * `z` - The z component of the vector.
    ///
    /// # Returns
    ///
    /// A `Result` containing a value of type `()` if successful, or an error of type
    /// `Errors::OpenGlError` if there is an error setting the uniform variable.

    pub fn set_uniform_3f(&mut self, name: &str, x: f32, y: f32, z: f32) -> Result<(), Errors> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::Uniform3f(location, x, y, z);
        }
        Ok(())
    }

    /// Set the value of a uniform variable of type `cgmath::Matrix4<f32>`.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniformMatrix4fv(location, 1, transpose, matrix.as_ptr())`.
    /// It sets the value of a uniform variable of type `cgmath::Matrix4<f32>`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable to set.
    /// * `matrix` - The value to set the uniform variable to.
    ///
    /// # Returns
    ///
    /// A `Result` containing a value of type `()` if successful, or an error of type
    /// `Errors::OpenGlError` if there is an error setting the uniform variable.
    pub fn set_uniform_matrix4fv(
        &mut self,
        name: &str,
        matrix: &cgmath::Matrix4<f32>,
    ) -> Result<(), Errors> {
        let location = self.get_uniform_location(name)?;
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
        Ok(())
    }
}

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

pub struct UniformBuffer {
    id: GLuint,
    binding_point: u32,
}

impl UniformBuffer {
    /// Create a new uniform buffer object (UBO).
    ///
    /// # Arguments
    ///
    /// * `binding_point` - The binding point to bind the UBO to.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `UniformBuffer` instance if successful, or an error of type `Errors::OpenGlError` otherwise.
    pub fn new(binding_point: u32) -> Result<Self, Errors> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        if id == 0 {
            return Err(Errors::OpenGlError(
                "Failed to generate uniform buffer".to_string(),
                gl::INVALID_OPERATION,
            ));
        }

        Ok(Self { id, binding_point })
    }

    /// Bind the uniform buffer to its binding point.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBufferBase`.
    pub fn bind(&self) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, self.binding_point, self.id);
        }
    }

    /// Unbind the uniform buffer.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBindBuffer(gl::UNIFORM_BUFFER, 0)`.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    /// Store data in the uniform buffer.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the buffer.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glBufferData`.
    pub fn store_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (data.len() * mem::size_of::<T>()) as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }

    /// Update data in the uniform buffer.
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
                gl::UNIFORM_BUFFER,
                offset as isize,
                (data.len() * mem::size_of::<T>()) as isize,
                data.as_ptr() as *const c_void,
            );
        }
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
