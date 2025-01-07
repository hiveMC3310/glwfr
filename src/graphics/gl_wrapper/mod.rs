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
//!     vao.set_index_count(6);
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

pub mod ebo;
pub mod shader;
pub mod vao;
pub mod vbo;
pub mod vertex_attribute;

pub use ebo::*;
pub use shader::*;
pub use vao::*;
pub use vbo::*;
pub use vertex_attribute::*;
