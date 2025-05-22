use super::{
    UiBox, UiElement, create_vertices,
    pipeline::{SharedPipeline, create_pipeline},
    texture::create_texture,
};
use image::RgbaImage;
use std::{any::Any, sync::Arc};
use wgpu::util::DeviceExt;

pub struct ButtonPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Button {
    position: (u16, u16),
    is_active: bool,
    bind_group: wgpu::BindGroup,
    shared_pipeline: Arc<ButtonPipeline>,
    vertex_buffer: wgpu::Buffer,
    is_active_buffer: wgpu::Buffer,
}

impl ButtonPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Button Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("button.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Button BindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(4),
                    },
                    count: None,
                },
            ],
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        };

        let pipeline = create_pipeline(
            device,
            config.format,
            &[&bind_group_layout],
            &[vertex_layout],
            &shader,
        );

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl SharedPipeline for ButtonPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl Button {
    pub fn new(
        pipeline: Arc<ButtonPipeline>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: RgbaImage,
        position: (u16, u16),
    ) -> Self {
        let (view, sampler) = create_texture(device, img, queue);
        let layout = &pipeline.bind_group_layout;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Button Vertex Buffer"),
            contents: bytemuck::cast_slice(&create_vertices(position, (16, 16), (200, 200))),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let is_active_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Boolean Uniform Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Button bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: is_active_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            is_active: false,
            shared_pipeline: pipeline,
            position,
            bind_group,
            vertex_buffer,
            is_active_buffer,
        }
    }
}

impl UiBox for Button {
    fn width(&self) -> u16 {
        16
    }

    fn height(&self) -> u16 {
        16
    }

    fn position(&self) -> (u16, u16) {
        self.position
    }

    fn on_click(&mut self) {
        self.is_active = !self.is_active
    }
}

impl UiElement for Button {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, queue: &wgpu::Queue) {
        let flag = if self.is_active { 1 } else { 0 };
        queue.write_buffer(&self.is_active_buffer, 0, bytemuck::bytes_of(&flag));

        render_pass.set_pipeline(self.shared_pipeline.pipeline());
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
