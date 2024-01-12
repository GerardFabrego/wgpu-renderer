// use std::mem::size_of;

// use crate::{components::Vertex, utils::load_texture};

// fn create_cube_data() -> ([Vertex; 24], [u32; 36]) {
//     #[rustfmt::skip]
//     let vertices = [

//         Vertex {position: [-1.0, -1.0,  1.0], uv: [0.0, 1.0] },
//         Vertex {position: [ 1.0, -1.0,  1.0], uv: [1.0, 1.0] },
//         Vertex {position: [ 1.0,  1.0,  1.0], uv: [1.0, 0.0] },
//         Vertex {position: [-1.0,  1.0,  1.0], uv: [0.0, 0.0] },

//         Vertex {position: [-1.0,  1.0, -1.0], uv: [0.0, 1.0] },
//         Vertex {position: [ 1.0,  1.0, -1.0], uv: [1.0, 1.0] },
//         Vertex {position: [ 1.0, -1.0, -1.0], uv: [1.0, 0.0] },
//         Vertex {position: [-1.0, -1.0, -1.0], uv: [0.0, 0.0] },

//         Vertex {position: [ 1.0, -1.0, -1.0], uv: [0.0, 1.0] },
//         Vertex {position: [ 1.0,  1.0, -1.0], uv: [1.0, 1.0] },
//         Vertex {position: [ 1.0,  1.0,  1.0], uv: [1.0, 0.0] },
//         Vertex {position: [ 1.0, -1.0,  1.0], uv: [0.0, 0.0] },

//         Vertex {position: [-1.0, -1.0,  1.0], uv: [0.0, 1.0] },
//         Vertex {position: [-1.0,  1.0,  1.0], uv: [1.0, 1.0] },
//         Vertex {position: [-1.0,  1.0, -1.0], uv: [1.0, 0.0] },
//         Vertex {position: [-1.0, -1.0, -1.0], uv: [0.0, 0.0] },

//         Vertex {position: [ 1.0,  1.0, -1.0], uv: [0.0, 1.0] },
//         Vertex {position: [-1.0,  1.0, -1.0], uv: [1.0, 1.0] },
//         Vertex {position: [-1.0,  1.0,  1.0], uv: [1.0, 0.0] },
//         Vertex {position: [ 1.0,  1.0,  1.0], uv: [0.0, 0.0] },

//         Vertex {position: [ 1.0, -1.0,  1.0], uv: [0.0, 1.0] },
//         Vertex {position: [-1.0, -1.0,  1.0], uv: [1.0, 1.0] },
//         Vertex {position: [-1.0, -1.0, -1.0], uv: [1.0, 0.0] },
//         Vertex {position: [ 1.0, -1.0, -1.0], uv: [0.0, 0.0] },
//     ];

//     let indices = [
//         0, 1, 2, 2, 3, 0, // top
//         4, 5, 6, 6, 7, 4, // bottom
//         8, 9, 10, 10, 11, 8, // right
//         12, 13, 14, 14, 15, 12, // left
//         16, 17, 18, 18, 19, 16, // front
//         20, 21, 22, 22, 23, 20, // back
//     ];

//     (vertices, indices)
// }

// impl super::Mesh {
//     pub async fn create_cube(
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         texture_path: &str,
//     ) -> anyhow::Result<Self> {
//         let texture = load_texture(texture_path, device, queue).await?;

//         let (vertices, indices) = create_cube_data();

//         let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("Vertex buffer"),
//             size: (size_of::<Vertex>() * vertices.len()) as u64,
//             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

//         let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("Index buffer"),
//             size: (size_of::<u32>() * indices.len()) as u64,
//             usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));

//         Ok(Self {
//             texture,
//             vertex_buffer,
//             index_buffer,
//             index_count: indices.len(),
//         })
//     }
// }
