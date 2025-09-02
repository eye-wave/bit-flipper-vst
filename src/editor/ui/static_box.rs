use crate::editor::texture::UVSegment;

use super::pipeline::{SharedPipeline, create_pipeline};
use super::texture::{TextureAtlas, TextureError};
use super::{UiBox, UiElement};
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct StaticBoxPipeline {
    pub(super) pipeline: wgpu::RenderPipeline,
    pub(super) tex_atlas: Arc<TextureAtlas>,
    pub(super) uniform_layout: wgpu::BindGroupLayout,
}

pub struct StaticBox {
    shared_pipeline: Arc<StaticBoxPipeline>,
    position: (u16, u16),
    width: u16,
    height: u16,
    uv_buffer: wgpu::Buffer,
    position_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl SharedPipeline for StaticBoxPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl StaticBoxPipeline {
    pub fn new(
        device: &wgpu::Device,
        tex_format: wgpu::TextureFormat,
        tex_atlas: Arc<TextureAtlas>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Static box Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("static_box.wgsl").into()),
        });

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<f32>() as _),
                },
                count: None,
            }],
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

        let pipeline = create_pipeline(
            device,
            tex_format,
            &[&tex_atlas.layout, &uniform_layout],
            vertex_buffer_layouts,
            wgpu::PrimitiveState::default(),
            &shader,
        );

        Self {
            pipeline,
            tex_atlas,
            uniform_layout,
        }
    }
}

impl StaticBox {
    pub fn new(
        device: &wgpu::Device,
        uv_segment: &UVSegment,
        position: (u16, u16),
        mask_color: Option<f32>,
        pipeline: Arc<StaticBoxPipeline>,
    ) -> Result<Self, TextureError> {
        let (width, height) = pipeline.tex_atlas.get_size(uv_segment)?;

        let (x, y) = position;
        let pos_data = [x, y, x + width, y + height].get_vertices();
        let uv_data = pipeline.tex_atlas.get_uvs(uv_segment).unwrap();

        let position_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Position Buffer"),
            contents: bytemuck::cast_slice(&pos_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let uv_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("UV Buffer"),
            contents: bytemuck::cast_slice(&uv_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Background Uniform Buffer"),
            contents: bytemuck::bytes_of(&mask_color.unwrap_or(-1.0)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Background Uniform Bind Group"),
            layout: &pipeline.uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            width,
            height,
            position,

            position_buffer,
            uv_buffer,
            shared_pipeline: pipeline,
            uniform_bind_group,
            uniform_buffer,
        })
    }

    pub fn change_color_mask(&mut self, queue: &wgpu::Queue, new_alpha: f32) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&new_alpha));
    }

    pub fn swap_uv(
        &mut self,
        queue: &wgpu::Queue,
        new_segment: &UVSegment,
    ) -> Result<(), TextureError> {
        let new_uvs = self.shared_pipeline.tex_atlas.get_uvs(new_segment)?;

        let (new_width, new_height) = self.shared_pipeline.tex_atlas.get_size(new_segment)?;
        self.width = new_width;
        self.height = new_height;

        queue.write_buffer(&self.uv_buffer, 0, bytemuck::cast_slice(&new_uvs));

        Ok(())
    }
}

impl UiElement for StaticBox {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(self.shared_pipeline.pipeline());

        render_pass.set_bind_group(0, &self.shared_pipeline.tex_atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.position_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.uv_buffer.slice(..));

        render_pass.draw(0..6, 0..1);
    }
}

impl UiBox for StaticBox {
    fn height(&self) -> u16 {
        self.height
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn position(&self) -> (u16, u16) {
        self.position
    }
}
