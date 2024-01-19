use std::{
    io::{BufReader, Cursor},
    mem::size_of,
};

use crate::{
    components::{Material, Mesh, Model, Vertex},
    texture::Texture,
};

pub fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

pub fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}

pub fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<Texture> {
    let data = load_binary(file_name)?;
    Ok(Texture::from_bytes(device, queue, &data, file_name))
}

pub fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<Model> {
    let obj_text = load_string(file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| {
            let mat_text = load_string(p.file_name().unwrap().to_str().unwrap()).unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let texture = load_texture(&m.diffuse_texture.unwrap(), device, queue)?;

        materials.push(Material {
            name: m.name,
            texture,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| Vertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    uv: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    // no
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                size: (size_of::<Vertex>() * vertices.len()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

            let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                size: (size_of::<u32>() * m.mesh.indices.len()) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&m.mesh.indices));

            (
                Mesh {
                    name: file_name.to_string(),
                    vertex_buffer,
                    index_buffer,
                    index_count: m.mesh.indices.len(),
                },
                m.mesh.material_id.unwrap_or(0),
            )
        })
        .collect::<Vec<_>>();

    Ok(Model { meshes, materials })
}
