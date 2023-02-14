use super::object::GLObject;
use gl::types::*;
use std::mem::size_of;


pub struct VertexAttribute {
    count: i32
}
impl VertexAttribute {
    pub fn with_count(count: i32) -> Self {
        VertexAttribute { count }
    }
}


pub struct VertexArray {
    handle: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut handle = 0u32;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        VertexArray { 
            handle
        }
    }

    pub fn vertex_attributes(&mut self, attribs: Vec<VertexAttribute>) {

        let stride = attribs.iter().map(|a| a.count).sum::<i32>() * size_of::<f32>() as i32;
        let mut offset = 0;

        self.bind();
        unsafe {
            for (index, attribute) in attribs.iter().enumerate() {
                gl::VertexAttribPointer(
                    index as u32,
                    attribute.count, 
                    gl::FLOAT, 
                    gl::FALSE, 
                    stride, 
                    offset as *const GLvoid
                );
                gl::EnableVertexAttribArray(index as u32);

                offset += attribute.count * size_of::<f32>() as i32;
            }
        }
        self.bind();
    }
}

impl GLObject for VertexArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
        }
    }
}