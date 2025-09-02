use super::{
    UiElement,
    pipeline::{SharedPipeline, create_pipeline},
    texture::TextureAtlas,
};
use crate::editor::texture::UVSegment::*;
use std::{sync::Arc, time::Instant};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BackgroundUniforms {
    pub uv_region: [f32; 4],
    pub time: f32,
    pub _padding: [f32; 3], // align to 16 bytes
}

pub struct BackgroundPipeline {
    pipeline: wgpu::RenderPipeline,
    atlas: Arc<TextureAtlas>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    position_buffer: wgpu::Buffer,
    uv_buffer: wgpu::Buffer,
    start_time: Instant,
}

pub struct Background {
    shared_pipeline: Arc<BackgroundPipeline>,
}

impl BackgroundPipeline {
    pub fn new(
        device: &wgpu::Device,
        tex_format: wgpu::TextureFormat,
        texture_atlas: Arc<TextureAtlas>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Background Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("background.wgsl").into()),
        });

        let positions: &[f32; 12] = &[
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, //
            -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let position_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Position Buffer"),
            contents: bytemuck::cast_slice(positions),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let uvs = texture_atlas.get_uvs(&UV_background).unwrap();
        let uv_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("UV Buffer"),
            contents: bytemuck::cast_slice(&uvs),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_buffer_layouts = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            },
        ];

        let uniforms = BackgroundUniforms {
            uv_region: texture_atlas.get_bounds(&UV_background).unwrap(),
            time: 0.0,
            _padding: [0.0; 3],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Background Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<BackgroundUniforms>() as _,
                    ),
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Background Uniform Bind Group"),
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline = create_pipeline(
            device,
            tex_format,
            &[&texture_atlas.layout, &uniform_layout],
            vertex_buffer_layouts,
            wgpu::PrimitiveState::default(),
            &shader,
        );

        Self {
            pipeline,
            atlas: texture_atlas,
            uniform_bind_group,
            uniform_buffer,
            position_buffer,
            uv_buffer,
            start_time: Instant::now(),
        }
    }
}

impl Background {
    pub fn new(pipeline: Arc<BackgroundPipeline>) -> Self {
        Self {
            shared_pipeline: pipeline,
        }
    }
}

impl SharedPipeline for BackgroundPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl UiElement for Background {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        _params: Arc<crate::BitFlipperParams>,
        _buffer: &crate::Bus,
    ) {
        let time = self.shared_pipeline.start_time.elapsed().as_secs_f32();
        let updated = BackgroundUniforms {
            uv_region: self
                .shared_pipeline
                .atlas
                .get_bounds(&UV_background)
                .unwrap(),
            time,
            _padding: [0.0; 3],
        };

        queue.write_buffer(
            &self.shared_pipeline.uniform_buffer,
            0,
            bytemuck::bytes_of(&updated),
        );
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.shared_pipeline.pipeline());
        render_pass.set_bind_group(0, &self.shared_pipeline.atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &self.shared_pipeline.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.shared_pipeline.position_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.shared_pipeline.uv_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}
