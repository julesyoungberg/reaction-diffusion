use glsl_layout::float;
use glsl_layout::*;
use nannou::prelude::*;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Uniform)]
pub struct Uniforms {
    pub width: float,
    pub height: float,
    pub time: float,
    pub diffusion_rate_a: float,
    pub diffusion_rate_b: float,
    pub feed_rate: float,
    pub kill_rate: float,
    pub reaction_speed: float,
}

impl Uniforms {
    pub fn new(width: float, height: float, time: f32) -> Self {
        Uniforms {
            width,
            height,
            time,
            diffusion_rate_a: 1.0,
            diffusion_rate_b: 0.5,
            feed_rate: 0.047,
            kill_rate: 0.052,
            reaction_speed: 1.0,
        }
    }
}

pub struct UniformBuffer {
    pub data: Uniforms,
    pub buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new(device: &wgpu::Device, width: float, height: float, time: f32) -> Self {
        let data = Uniforms::new(width, height, time);

        let std140_uniforms = data.std140();
        let uniforms_bytes = std140_uniforms.as_raw();
        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let buffer = device.create_buffer_with_data(uniforms_bytes, usage);

        Self { data, buffer }
    }

    pub fn update(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, time: f32) {
        self.data.time = time;

        // An update for the uniform buffer with the current time.
        let std140_uniforms = self.data.std140();
        let uniforms_bytes = std140_uniforms.as_raw();
        let uniforms_size = uniforms_bytes.len();
        let usage = wgpu::BufferUsage::COPY_SRC;
        let new_uniform_buffer = device.create_buffer_with_data(uniforms_bytes, usage);

        encoder.copy_buffer_to_buffer(
            &new_uniform_buffer,
            0,
            &self.buffer,
            0,
            uniforms_size as u64,
        );
    }
}
