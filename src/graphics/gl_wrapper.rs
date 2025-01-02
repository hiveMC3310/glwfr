use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::mem;

use std::io::Read;

use std::os::raw::*;
use std::ptr;

use gl::types::*;

use cgmath::*;

pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    /// Create a new Vertex Array Object (VAO).
    ///
    /// # Returns
    ///
    /// A `Vao` instance with a generated OpenGL vertex array ID.

    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
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
}

pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject {
    /// Create a new BufferObject with the given type and usage.
    ///
    /// # Returns
    ///
    /// A `BufferObject` instance with a generated OpenGL buffer ID.
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        BufferObject { id, r#type, usage }
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
    /// Construct a new `ShaderProgram` from the given vertex shader and fragment shader paths.
    ///
    /// # Errors
    ///
    /// This function will panic if the shader source files cannot be read.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glCreateShader`, `glShaderSource`, `glCompileShader`, `glCreateProgram`, `glAttachShader`, `glLinkProgram`, and `glDeleteShader`.
    ///
    /// # Arguments
    ///
    /// * `vertex_shader_path` - The path to the vertex shader source file.
    /// * `fragment_shader_path` - The path to the fragment shader source file.
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");

        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram {
                program_handle,
                uniform_ids: HashMap::new(),
            }
        }
    }

    /// Bind this shader program to the current OpenGL context, making it the active program.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUseProgram(program_handle)`.
    /// It binds the shader program to the current OpenGL context, making it the active program.
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    /// Unbind any shader program from the current OpenGL context, making no program active.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUseProgram(0)`.
    /// It unbinds any shader program from the current OpenGL context, making no program active.
    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Create a new uniform in the shader program with the given name and store it in the internal map of uniforms.
    ///
    /// # Panics
    ///
    /// This function will panic if the uniform with the given name cannot be found in the shader program.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glGetUniformLocation(program_handle, uniform_name)`.
    pub fn create_uniform(&mut self, uniform_name: &str) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handle,
                CString::new(uniform_name).unwrap().as_ptr(),
            )
        };
        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_name);
        } else {
            self.uniform_ids
                .insert(uniform_name.to_string(), uniform_location);
        }
    }

    /// Set the value of a uniform variable in the shader program to the given matrix.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_ptr())`.
    ///
    /// # Arguments
    ///
    /// * `uniform_name` - The name of the uniform variable to set.
    /// * `matrix` - The matrix value to set the uniform to.
    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            )
        }
    }

    /// Set the value of a uniform variable in the shader program to the given 3 floats.
    ///
    /// # OpenGL Functions
    ///
    /// This function is a wrapper around `glUniform3f(uniform_location, x, y, z)`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the uniform variable to set.
    /// * `x` - The x component of the value to set the uniform to.
    /// * `y` - The y component of the value to set the uniform to.
    /// * `z` - The z component of the value to set the uniform to.
    pub fn set_uniform_3f(&self, name: &str, x: f32, y: f32, z: f32) {
        if let Some(&location) = self.uniform_ids.get(name) {
            unsafe {
                gl::Uniform3f(location, x, y, z);
            }
        } else {
            eprintln!("Uniform '{}' not found! Did you call create_uniform?", name);
        }
    }
}

pub struct Ebo {
    id: gl::types::GLuint,
}

impl Ebo {
    /// Create a new Element Buffer Object (EBO) with a generated OpenGL buffer ID.
    pub fn new() -> Ebo {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        Ebo { id }
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
}
