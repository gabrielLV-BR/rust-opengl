use super::object::GLObject;
use gl::types::*;
use std::mem::size_of;

#[derive(Clone, Copy)]
pub struct VertexAttribute {
    count: i32,
    pub order: i32,
}

impl VertexAttribute {
    pub const POSITION: Self = VertexAttribute { count: 3, order: 0 };
    pub const NORMAL: Self = VertexAttribute { count: 3, order: 1 };
    pub const COLOR: Self = VertexAttribute { count: 3, order: 2 };
    pub const UV: Self = VertexAttribute { count: 2, order: 3 };
}

#[derive(Debug)]
pub struct VertexArray {
    handle: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut handle = 0u32;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        VertexArray { handle }
    }

    pub fn bound_with(self, objects: Vec<&dyn GLObject>) -> Self {
        self.bind();
        for obj in objects.iter() {
            obj.bind();
        }
        self.unbind();
        self
    }

    pub fn with_vertex_attributes(self, attribs: Vec<VertexAttribute>) -> Self {
        let stride = attribs.iter().map(|a| a.count).sum::<i32>() * size_of::<f32>() as i32;
        let mut offset = 0;

        println!("stride: {}", stride);

        self.bind();
        unsafe {
            for (index, attribute) in attribs.iter().enumerate() {
                gl::VertexAttribPointer(
                    index as u32,
                    attribute.count,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    offset as *const GLvoid,
                );
                gl::EnableVertexAttribArray(index as u32);

                offset += attribute.count * size_of::<f32>() as i32;
            }
        }
        self.unbind();
        self
    }
}

impl GLObject for VertexArray {
    fn handle(&self) -> u32 {
        self.handle
    }

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
