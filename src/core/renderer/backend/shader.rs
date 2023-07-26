use gl::{self, types::GLchar};
use std::{collections::HashMap, io, path::PathBuf};

use super::object::GLObject;

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    handle: u32,
}
impl Shader {
    pub fn new(source: &str, shader_type: ShaderType) -> Shader {
        let handle = unsafe {
            let handle = gl::CreateShader(match shader_type {
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
                ShaderType::Vertex => gl::VERTEX_SHADER,
            });

            let source_ptr = source.as_ptr() as *const std::os::raw::c_char;
            gl::ShaderSource(handle, 1, &source_ptr, &(source.len() as i32));
            gl::CompileShader(handle);

            Shader::check_for_errors(handle, shader_type);

            handle
        };

        Shader { handle }
    }

    pub fn from_file(path: &PathBuf, shader_type: ShaderType) -> Result<Shader, io::Error> {
        let source = std::fs::read_to_string(path)?;
        Ok(Shader::new(&source, shader_type))
    }

    unsafe fn check_for_errors(handle: u32, shader_type: ShaderType) {
        let mut success = 0i32;
        gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);

            gl::GetShaderInfoLog(
                handle,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );

            panic!(
                "[AT {} SHADER]\n{}",
                match shader_type {
                    ShaderType::Fragment => "Fragment",
                    ShaderType::Vertex => "Vertex",
                },
                std::str::from_utf8(&info_log).ok().unwrap()
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.handle);
        }
    }
}

//

#[derive(Debug)]
pub struct Program {
    handle: u32,
    // material_type: MaterialShaderType,
    uniform_cache: HashMap<String, i32>,
}

pub enum UniformType<'a> {
    Int(i32),
    Float(f32),
    Uint(u32),
    Matrix4(&'a ultraviolet::Mat4),
    Vec3(&'a ultraviolet::Vec3),
}

impl Program {
    // Shader uniform contants
    pub const COLOR_UNIFORM: &str = "uColor";
    pub const TEXTURE_MAP_UNIFORM: &str = "uTexturemap";

    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Self {
        let handle = unsafe {
            let handle = gl::CreateProgram();

            gl::AttachShader(handle, vertex_shader.handle);
            gl::AttachShader(handle, fragment_shader.handle);
            gl::LinkProgram(handle);

            handle
        };

        drop(vertex_shader);
        drop(fragment_shader);

        // let uniform_cache = Program::map_uniforms(handle);

        Self {
            handle,
            uniform_cache: HashMap::new(),
        }
    }

    fn map_uniforms(handle: u32) -> HashMap<String, i32> {
        let mut uniform_map: HashMap<String, i32> = HashMap::new();

        let mut uniform_count = 0i32;
        let capacity = 16;
        let mut name: Vec<GLchar> = Vec::with_capacity(capacity);

        let mut length = 0i32;
        let mut size = 0i32;
        let mut _type = 0u32;

        unsafe {
            gl::GetProgramiv(handle, gl::ACTIVE_UNIFORMS, &mut uniform_count);

            println!("Uniform count: {}", uniform_count);

            for i in 0..uniform_count {
                gl::GetActiveUniform(
                    handle,
                    i as u32,
                    capacity as i32,
                    &mut length,
                    &mut size,
                    &mut _type,
                    name.as_mut_ptr(),
                );

                let location = gl::GetUniformLocation(handle, name.as_ptr().cast());
                assert!(location != -1);

                let porra: Vec<u8> = name.iter().map(|i| *i as u8).collect();

                let name = std::str::from_utf8(&porra).unwrap().to_owned();
                println!("Cached uniform [{}]!", name);

                uniform_map.insert(name, location);
            }
        }
        uniform_map
    }

    pub fn get_uniform_location(&self, name: &String) -> Option<i32> {
        return if self.uniform_cache.contains_key(name) {
            Some(self.uniform_cache[name])
        } else {
            Some(unsafe { gl::GetUniformLocation(self.handle, name.as_bytes().as_ptr().cast()) })
        };
    }

    pub fn set_uniform(&self, name: &str, value: UniformType) {
        let location = self.get_uniform_location(&String::from(name)).unwrap();

        match value {
            UniformType::Float(v) => unsafe {
                gl::Uniform1f(location, v);
            },
            UniformType::Int(v) => unsafe {
                gl::Uniform1i(location, v);
            },
            UniformType::Uint(v) => unsafe {
                gl::Uniform1ui(location, v);
            },
            UniformType::Matrix4(v) => unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, v.as_ptr().cast());
            },
            UniformType::Vec3(v) => unsafe {
                gl::Uniform3fv(location, 1, v.as_ptr());
            },
        }
    }
}

impl GLObject for Program {
    fn handle(&self) -> u32 {
        self.handle
    }

    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}
