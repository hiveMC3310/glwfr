# glwfr

[![Crates.io](https://img.shields.io/crates/v/glwfr)](https://crates.io/crates/glwfr)
[![Docs.rs](https://docs.rs/glwfr/badge.svg)](https://docs.rs/glwfr)

`glwfr` (GL Wrapper For Rust) is a library for working with OpenGL in Rust.

## Features

- Remade scene system
- Add uniform-block to ShaderProgram

## Usage

Add `glwfr` to your `Cargo.toml`:

```toml
[dependencies]
glwfr = "0.3.3"
```

### Example

```rust
use glwfr::{
    cgmath::*,
    gl,
    graphics::{gl_wrapper::*, window::Window},
    input::{self, Key},
    scene::*,
};
use std::time::Instant;

fn main() -> Result<(), glwfr::custom_errors::Errors> {
    const CUBE_VERTICES: [f32; 144] = [
        // Front face
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // 0
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // 1
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, // 2
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, // 3
        // Back face
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, // 4
        0.5, -0.5, -0.5, 0.0, 0.0, -1.0, // 5
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, // 6
        -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, // 7
        // Top face
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // 8
        0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // 9
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, // 10
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, // 11
        // Bottom face
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, // 12
        0.5, -0.5, -0.5, 0.0, -1.0, 0.0, // 13
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, // 14
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, // 15
        // Left face
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, // 16
        -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, // 17
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, // 18
        -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, // 19
        // Right face
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, // 20
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, // 21
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, // 22
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, // 23
    ];

    const CUBE_INDICES: [u32; 36] = [
        //Front face
        0, 1, 2, 2, 3, 0,
        // Back face
        4, 5, 6, 6, 7, 4,
        // Top face
        8, 9, 10, 10, 11, 8,
        // Bottom face
        12, 13, 14, 14, 15, 12,
        // Left face
        16, 17, 18, 18, 19, 16,
        // Right face
        20, 21, 22, 22, 23, 20,
    ];

    // Create window
    let mut window = Window::new(800, 600, "Rotating Cube")?;
    window.init_gl()?;
    window.enable_depth_test();

    // Create shader program
    let shader_program = ShaderProgram::new("shaders/vertex.glsl", "shaders/fragment.glsl")?;

    // Create VAO, VBO, EBO

    let mut vao = Vao::new()?;
    vao.bind();
    vao.set_index_count(CUBE_INDICES.len());

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW)?;
    vbo.bind();
    vbo.store_f32_data(&CUBE_VERTICES);

    let ebo = Ebo::new()?;
    ebo.bind();
    ebo.store_indices(&CUBE_INDICES);

    // Vertex attributes
    VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * std::mem::size_of::<f32>() as i32,
        0 as *const _,
    )
    .enable();
    VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * std::mem::size_of::<f32>() as i32,
        (3 * std::mem::size_of::<f32>()) as *const _,
    )
    .enable();

    // Create cube object
    let mut cube = Object::new(vao, shader_program);
    cube.set_transform(Matrix4::identity());

    // Create camera
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        CameraType::Perspective {
            fov: Deg(45.0),
            aspect: 800.0 / 600.0,
            near: 0.1,
            far: 100.0,
        },
    );

    // Create scene
    let mut scene = Scene::new(camera);
    scene.add_object(cube);

    // Create light
    let light = Light::new(
        LightType::Point {
            position: Point3::new(2.0, 2.0, 2.0),
            intensity: 1.0,
        },
        Vector3::new(1.0, 1.0, 1.0),
    );
    scene.add_light(light);

    // Main loop
    let mut last_time = Instant::now();
    while !window.should_close() {
        // Delta time
        let delta_time = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();

        // Camera control
        let camera = scene.get_mut_camera();
        if input::is_key_pressed(Key::Up) {
            camera.position.z -= 1.0 * delta_time;
        }
        if input::is_key_pressed(Key::Down) {
            camera.position.z += 1.0 * delta_time;
        }

        // Light control
        let lights = scene.get_mut_lights();
        let light_position = lights.get_mut(0).unwrap().get_light_data().0;
        if let Some(light) = lights.get_mut(0) {
            if input::is_key_pressed(Key::W) {
                if let LightType::Point { position, .. } = &mut light.light_type {
                    position.y += 1.0 * delta_time;
                }
            }
            if input::is_key_pressed(Key::S) {
                if let LightType::Point { position, .. } = &mut light.light_type {
                    position.y -= 1.0 * delta_time;
                }
            }
        }

        // Cube control
        if let Some(cube) = scene.get_mut_object(0) {
            if input::is_key_pressed(Key::Right) {
                cube.transform = cube.transform * Matrix4::from_angle_y(Deg(50.0 * delta_time));
            }
            if input::is_key_pressed(Key::Left) {
                cube.transform = cube.transform * Matrix4::from_angle_y(Deg(-50.0 * delta_time));
            }

            // Set uniforms

            cube.shader_program.bind();
            cube.shader_program.set_uniform_3f(
                "lightPos",
                light_position.x,
                light_position.y,
                light_position.z,
            )?;
            cube.shader_program
                .set_uniform_3f("lightColor", 1.0, 1.0, 1.0)?;
            cube.shader_program
                .set_uniform_3f("objectColor", 1.0, 0.0, 0.0)?;
        }

        // Clear
        window.clear(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0, 1.0);

        // Render
        scene.render();

        // Update (swap)
        window.update();
    }

    Ok(())
}

```
