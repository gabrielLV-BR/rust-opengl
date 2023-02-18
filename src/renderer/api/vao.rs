use super::object::GLObject;
use bevy_ecs::prelude::Component;
use gl::types::*;
use std::{mem::size_of, ops::BitAnd};


#[derive(Clone, Copy)]
pub struct VertexAttribute {
    count: i32,
}

impl VertexAttribute {
    pub const POSITION  : Self = VertexAttribute { count: 3 };
    pub const NORMAL    : Self = VertexAttribute { count: 3 };
    pub const COLOR     : Self = VertexAttribute { count: 3 };
    pub const UV        : Self = VertexAttribute { count: 2 };

    fn new(count: i32) -> Self {
        VertexAttribute { count }
    }
}

impl BitAnd for VertexAttribute {
    type Output = VertexAttribute;
    fn bitand(self, rhs: Self) -> Self::Output {
        VertexAttribute { 
            count: self.count + rhs.count, 
        }
    }
}

#[derive(Component, Debug)]
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