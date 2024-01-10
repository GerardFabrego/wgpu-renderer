pub struct UniformPool {
    label: &'static str,
    pub buffers: Vec<wgpu::Buffer>,
    size: u64,
}

impl UniformPool {
    pub fn new(label: &'static str, size: u64) -> Self {
        Self {
            label,
            buffers: Vec::new(),
            size,
        }
    }

    pub fn alloc_buffers(&mut self, count: usize, device: &wgpu::Device) {
        self.buffers = Vec::new();

        for _ in 0..count {
            let local_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(self.label),
                size: self.size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.buffers.push(local_uniform_buffer);
        }
    }

    pub fn update_uniform<T: bytemuck::Pod>(&self, index: usize, data: T, queue: &wgpu::Queue) {
        if !self.buffers.is_empty() {
            queue.write_buffer(&self.buffers[index], 0, bytemuck::cast_slice(&[data]));
        }
    }
}
