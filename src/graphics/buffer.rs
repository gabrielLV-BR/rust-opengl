use super::object::GLObject;
use gl::{self, types::*};
use std::mem::size_of;

pub struct Buffer<T> {
    handle: u32,
    target: u32,
    pub data: Vec<T>
}

impl<T> Buffer<T> {
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

    pub fn size(&self) -> usize {
        self.data.len() * size_of::<T>()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn set_data(&mut self, usage: u32, data: Vec<T>) {
        self.data = data;
        self.bind();

        unsafe {
            gl::BufferData(
                self.target,
                self.data.len() as isize * size_of::<T>() as isize,
                self.data.as_ptr().cast(),
                usage
            );
        }

        self.unbind();
    }
}

impl<T> GLObject for Buffer<T> {
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