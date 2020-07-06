use gl::types::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::str::from_utf8;
use std::time::{Duration, Instant};
use std::{mem, ptr};

use crate::render_camera::Camera;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 transform;

    void main() {
       gl_Position = transform * vec4(aPos, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub struct Renderer {
    pub program: u32,
    pub vao: u32,
    pub transform_uniform: i32,
    pub camera: Camera,
    pub total_frames: i32,
    pub frame_count: i32,
    pub last_return: Instant,
}

impl Renderer {
    pub fn init(window: &mut glfw::Window) -> Renderer {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        let (shader_program, vao, transform_uniform) = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    from_utf8(&info_log).unwrap()
                );
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    from_utf8(&info_log).unwrap()
                );
            }

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    from_utf8(&info_log).unwrap()
                );
            }

            // Grab the uniforms from our shader program
            let c_str_transform_uniform = CString::new("transform").unwrap();
            let transform_uniform =
                gl::GetUniformLocation(shader_program, c_str_transform_uniform.as_ptr());

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let vertices: [f32; 9] = [
                -0.5, -0.5, 0.0, // left
                0.5, -0.5, 0.0, // right
                0.0, 0.5, 0.0, // top
            ];
            let (mut vbo, mut vao) = (0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            (shader_program, vao, transform_uniform)
        };

        let camera = Camera::new(1280.0 / 720.0, 70.0);

        return Renderer {
            program: shader_program,
            vao,
            transform_uniform,
            camera,
            total_frames: 0,
            last_return: Instant::now(),
            frame_count: 0,
        };
    }

    pub fn update(&mut self) {
        unsafe {
            //let frame_time = (self.total_frames as f32) / 60.0;
            // Update the camera, and then construct the world space
            // matrix.
            //self.camera.borrow_spatial_mut().set_rotation(
            //    &Vector3::new(
            //        (frame_time * 1.265).sin() * 10.0,
            //        (frame_time * 1.567).sin() * 10.0,
            //        0.0));
            let mut world_space_matrix = self.camera.get_projection().clone_owned();

            let camera_model_space_matrix = self
                .camera
                .borrow_spatial_mut()
                .get_model_space_matrix()
                .try_inverse();

            if let Some(x) = camera_model_space_matrix {
                world_space_matrix *= x;
            }

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.program);

            let world_space_matrix_data = world_space_matrix.as_slice();

            gl::UniformMatrix4fv(
                self.transform_uniform,
                1,
                gl::FALSE,
                world_space_matrix_data.as_ptr(),
            );

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            if self.last_return.elapsed() >= Duration::from_secs(1) {
                println!("FPS: {}", self.frame_count);
                self.frame_count = 0;
                self.last_return = Instant::now();
            }

            self.frame_count += 1;
            self.total_frames += 1;
        }
    }
}
