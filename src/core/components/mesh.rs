use gl;

use crate::core::renderer::api::buffer::{VertexBuffer, ElementBuffer};

pub struct Mesh {
    vertex_buffer: VertexBuffer,
    element_buffer: ElementBuffer
}