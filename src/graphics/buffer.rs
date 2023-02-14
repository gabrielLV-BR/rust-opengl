use super::object::GLObject;
use gl::{self, types::*};
use std::mem::size_of;

pub struct Buffer {
    handle: u32,
    target: u32,
    data: Vec<f32> 
}

impl Buffer {
    pub fn new(target: u32) -> Self {

        let mut handle = 0u32;

        unsafe {
            gl::GenBuffers(1, &mut handle);
            gl::BindBuffer(target, handle);
        }

        Buffer {
            handle,
            target,
            data: Vec::new()
        }
    }

    pub fn set_vertices(&mut self, usage: u32, vertices: Vec<f32>) {
        self.data = vertices;
        self.bind();

        unsafe {
            gl::BufferData(
                self.target,
                self.data.len() as isize * size_of::<f32>() as isize,
                self.data.as_ptr().cast(),
                usage  
            );
        }

        self.unbind();
    }
}

impl GLObject for Buffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.handle);
        }    
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }    
    }
}