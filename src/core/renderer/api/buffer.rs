use super::object::GLObject;
use bevy_ecs::prelude::Component;
use gl;
use std::mem::size_of;

#[derive(Component, Debug)]
pub struct Buffer<T> {
    pub handle: u32,
    target: u32,
    pub data: Vec<T>
}

pub type VertexBuffer = Buffer<f32>;
pub type ElementBuffer = Buffer<u32>;

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

        println!("{}", self.size());

        unsafe {
            gl::BufferData(
                self.target,
                self.size() as isize,
                self.data.as_ptr().cast(),
                usage
            );
        }

        self.unbind();
    }
}
 
impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle);
        }
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