extern crate gl;

use ultraviolet::{Vec4,Mat4};
use crate::util::resources::{Shader, ShaderManager, AssetManager};
use crate::{error, warn, trace, c_str};

#[derive(Debug)]
pub struct ShaderProgram {
    pub id: u32,
    pub vert_shader: Shader,
    pub frag_shader: Shader,
}

impl ShaderProgram {
    pub unsafe fn new() -> Self {
        ShaderProgram {
            id: 0,
            vert_shader: Shader { id: 0, code: String::from("") },
            frag_shader: Shader { id: 0, code: String::from("") },
        }
    }

    pub unsafe fn with_vert_shader(mut self, path: String) -> Self {
        let mut vert_shader = ShaderManager::load_direct(path);

        let gl_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(gl_shader, 1, &vert_shader.code.as_ptr().cast(), &(vert_shader.code.len() as i32));
        gl::CompileShader(gl_shader);

        let mut success = 0;
        gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 { // non-zero == success
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut len = 0;

            gl::GetShaderInfoLog(gl_shader, log.capacity() as i32, &mut len, log.as_mut_ptr().cast());
            log.set_len(len as usize);
            let log = std::str::from_utf8(&log).unwrap_or("Unknown error!");
            error!("Error while compiling vertex shader: {}, {:?}", log, vert_shader);

            // Clean up since something went wrong
            gl::DeleteShader(gl_shader);
            
            return self;
        }

        vert_shader.id = gl_shader;
        self.vert_shader = vert_shader;
        self
    }

    pub unsafe fn with_frag_shader(mut self, path: String) -> Self {
        let mut frag_shader = ShaderManager::load_direct(path);

        let gl_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(gl_shader, 1, &frag_shader.code.as_ptr().cast(), &(frag_shader.code.len() as i32));
        gl::CompileShader(gl_shader);

        let mut success = 0;
        gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 { // non-zero == success
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut len = 0;

            gl::GetShaderInfoLog(gl_shader, log.capacity() as i32, &mut len, log.as_mut_ptr().cast());
            log.set_len(len as usize);
            let log = std::str::from_utf8(&log).unwrap_or("Unknown error!");
            error!("Error while compiling fragment shader: {}, {:?}", log, frag_shader);

            // Clean up since something went wrong
            gl::DeleteShader(gl_shader);
            
            return self;
        }

        frag_shader.id = gl_shader;
        self.frag_shader = frag_shader;
        self
    }

    pub unsafe fn build(mut self) -> Self {
        if self.vert_shader.id == 0 || self.frag_shader.id == 0 {
            warn!("Building shader program with unregistered shaders!\n{:?}", self);
        }

        let program = gl::CreateProgram();
        gl::AttachShader(program, self.vert_shader.id);
        gl::AttachShader(program, self.frag_shader.id);
        gl::LinkProgram(program);
        gl::ValidateProgram(program);

        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 { // non-zero == success
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut len = 0;

            gl::GetProgramInfoLog(program, log.capacity() as i32, &mut len, log.as_mut_ptr().cast());
            log.set_len(len as usize);
            let log = std::str::from_utf8(&log).unwrap_or("Unknown error!");
            error!("Error while linking shader program: {}, {:?}", log, self);

            // Clean up since something went wrong
            gl::DetachShader(program, self.vert_shader.id);
            gl::DeleteShader(self.vert_shader.id);

            gl::DetachShader(program, self.frag_shader.id);
            gl::DeleteShader(self.frag_shader.id);

            gl::DeleteProgram(program);
            return self;
        }

        let mut success = 0;
        gl::GetProgramiv(program, gl::VALIDATE_STATUS, &mut success);
        if success == 0 { // non-zero == success
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut len = 0;

            gl::GetProgramInfoLog(program, log.capacity() as i32, &mut len, log.as_mut_ptr().cast());
            log.set_len(len as usize);
            let log = std::str::from_utf8(&log).unwrap_or("Unknown error!");
            error!("Error while validating shader program: {}, {:?}", log, self);

            // Clean up since something went wrong
            gl::DetachShader(program, self.vert_shader.id);
            gl::DeleteShader(self.vert_shader.id);

            gl::DetachShader(program, self.frag_shader.id);
            gl::DeleteShader(self.frag_shader.id);

            gl::DeleteProgram(program);
            return self;
        }

        self.id = program;

        // We mark the shaders for deletion, but they won't be destroyed until
        // they are removed from the program
        gl::DeleteShader(self.vert_shader.id);
        gl::DeleteShader(self.frag_shader.id);

        trace!("Created shader program: {:?}", self);

        self
    }

    pub unsafe fn activate(&self) {
        if self.id > 0 {
            gl::UseProgram(self.id);
        } else {
            warn!("Tried to activate an unregistered shader program!\n{:?}", self);
        }
    }

    pub unsafe fn set_uniform_mat4(&self, name: &str, value: &Mat4) {
        let mat_loc = gl::GetUniformLocation(self.id, c_str!(name).as_ptr().cast());
        gl::UniformMatrix4fv(mat_loc, 1, gl::FALSE, value.as_ptr());
    }

    pub unsafe fn set_uniform_vec4(&self, name: &str, value: &Vec4) {
        let loc = gl::GetUniformLocation(self.id, c_str!(name).as_ptr().cast());
        gl::Uniform4f(loc, value.x, value.y, value.z, value.w);
    }

    pub unsafe fn set_uniform_int(&self, name: &str, value: u32) {
        let loc = gl::GetUniformLocation(self.id, c_str!(name).as_ptr().cast());
        gl::Uniform1i(loc, value as i32);
    }
    
    pub unsafe fn deactivate(&self) {
        gl::UseProgram(0);
    }

    pub unsafe fn destroy(&self) {
        gl::DeleteProgram(self.id);
    }
}
