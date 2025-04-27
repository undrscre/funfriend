use std::{ffi::CString, mem, ptr, error::Error};
use crate::config::ConfigType;
use gl::types::*;

use super::texture::TextureBasket;

pub struct BuddyRenderer {
    shader: Shader,
    vao: GLuint,
    // vbo: GLuint,
    // ebo: GLuint,
    textures: TextureBasket
}

impl BuddyRenderer {
    pub fn new(textures: TextureBasket, vertex_src: &str, fragment_src: &str) -> Result<Self, Box<dyn Error>> {
        let shader = Shader::new(vertex_src, fragment_src);
        let (vao, _vbo, _ebo) = Self::init_buffers()?;
        Ok(Self {
            shader,
            vao,
            // vbo,
            // ebo,
            textures
        })
    }

    fn init_buffers() -> Result<(GLuint, GLuint, GLuint), Box<dyn Error>> {
        const VERTICES: [GLfloat; 20] = [
             1.0,  1.0, 0.0,  1.0, 1.0,
             1.0, -1.0, 0.0,  1.0, 0.0,
            -1.0, -1.0, 0.0,  0.0, 0.0,
            -1.0,  1.0, 0.0,  0.0, 1.0,
        ];
        const INDICES: [GLuint; 6] = [
            0, 1, 3,
            1, 2, 3, 
        ];

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                VERTICES.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (INDICES.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                INDICES.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (5 * mem::size_of::<GLfloat>()) as GLsizei;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Ok((vao, vbo, ebo))
    }

    pub fn render(&mut self, dt: f64, config: ConfigType, window_size: i32, time: f64) {
        self.textures.update(dt);
        let current_frame = self.textures.texture();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            if let Some(frame_info) = current_frame {
                gl::UseProgram(self.shader.program);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, frame_info.tex);

                Self::set_uniform_int(self.shader.program, "texture1", 0);
                let (fs_w, fs_h) = (config.friend_size,config.friend_size);
                Self::set_uniform_vec2(self.shader.program, "funfriendSize", fs_w as f32, fs_h as f32);
                Self::set_uniform_vec2(self.shader.program, "resolution", window_size as f32, window_size as f32);
                Self::set_uniform_float(self.shader.program, "time", time as f32);

                gl::BindVertexArray(self.vao);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                gl::BindVertexArray(0); 
                gl::BindTexture(gl::TEXTURE_2D, 0);
                gl::UseProgram(0);
            }
            gl::Disable(gl::BLEND);
        }
    }

    fn set_uniform_int(program: GLuint, name: &str, value: GLint) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_name.as_ptr());
            if location != -1 {
                gl::Uniform1i(location, value);
            }
        }
    }

    fn set_uniform_float(program: GLuint, name: &str, value: GLfloat) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_name.as_ptr());
            if location != -1 {
                gl::Uniform1f(location, value);
            }
        }
    }

    fn set_uniform_vec2(program: GLuint, name: &str, v1: GLfloat, v2: GLfloat) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_name.as_ptr());
            if location != -1 {
                gl::Uniform2f(location, v1, v2);
            }
        }
    }

}

pub struct Shader {
    pub vert: GLuint,
    pub frag: GLuint,
    pub program: GLuint,
}

impl Shader {
    //ts pmo
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let vert = Self::compile_shader(vertex_src, gl::VERTEX_SHADER);
        let frag = Self::compile_shader(fragment_src, gl::FRAGMENT_SHADER);
        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);
        }
        unsafe {
            let mut success: GLint = 0;
            gl::GetShaderiv(vert, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len: GLint = 0;
                gl::GetShaderiv(vert, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
                gl::GetShaderInfoLog(vert, log_len, &mut log_len, log.as_mut_ptr() as *mut i8);
                log.set_len(log_len as usize);
                panic!("Vertex shader compilation failed: {}", String::from_utf8_lossy(&log));
            }
            
            gl::GetShaderiv(frag, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len: GLint = 0;
                gl::GetShaderiv(frag, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
                gl::GetShaderInfoLog(frag, log_len, &mut log_len, log.as_mut_ptr() as *mut i8);
                log.set_len(log_len as usize);
                panic!("Fragment shader compilation failed: {}", String::from_utf8_lossy(&log));
            }
        
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut log_len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
                gl::GetProgramInfoLog(program, log_len, &mut log_len, log.as_mut_ptr() as *mut i8);
                log.set_len(log_len as usize);
                panic!("Program linking failed: {}", String::from_utf8_lossy(&log));
            }
        }
        
        Self { vert, frag, program }
    }

    pub fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        // it's very balls to the wall here i don't have any form of logging here
        // future ivy note -- no it's right there buddy
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source).expect("failed to convert");

        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
        }

        shader
    }
}
