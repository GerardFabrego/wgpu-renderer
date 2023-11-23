use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
