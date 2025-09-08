use super::texture::TextureAtlas;

use crate::BitFlipperParams;
use crate::editor::{VIEW_HEIGHT, VIEW_WIDTH, texture::UVSegment::*};

use boxi::prelude::*;
use nih_plug::params::Param;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SliderUniforms {
    pub uv_region: [f32; 4],
    pub value: f32,
    pub _padding: [f32; 3], // align to 16 bytes
}

pub struct SliderPipeline {
    pipeline: wgpu::RenderPipeline,
    tex_atlas: Arc<TextureAtlas>,
    uniform_layout: wgpu::BindGroupLayout,
}

pub struct Slider {
    shared_pipeline: Arc<SliderPipeline>,
    position: (u16, u16),
    position_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uv_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uv_region: [f32; 4],
}

impl SliderPipeline {
    pub fn new(
        device: &wgpu::Device,
        tex_format: wgpu::TextureFormat,
        tex_atlas: Arc<TextureAtlas>,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Slider Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("slider.wgsl").into()),
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
                        std::mem::size_of::<SliderUniforms>() as _
                    ),
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

impl Slider {
    pub fn new(device: &wgpu::Device, position: (u16, u16), pipeline: Arc<SliderPipeline>) -> Self {
        let (x, y) = position;
        let pos_data = [x, y, x + 59, y + 8]
            .get_vertices::<{ VIEW_WIDTH as usize }, { VIEW_HEIGHT as usize }>();

        let position_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Position Buffer"),
            contents: bytemuck::cast_slice(&pos_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let uv_region = pipeline.tex_atlas.get_bounds(&UV_slider_handle).unwrap();
        let uniforms = SliderUniforms {
            uv_region,
            value: 0.0,
            _padding: [0.0; 3],
        };

        let uv_data = pipeline.tex_atlas.get_uvs(&UV_slider_handle).unwrap();
        let uv_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("UV Buffer"),
            contents: bytemuck::cast_slice(&uv_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Background Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniforms),
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

        Self {
            position,

            position_buffer,
            shared_pipeline: pipeline,
            uniform_bind_group,
            uniform_buffer,
            uv_region,
            uv_buffer,
        }
    }
}

impl UiInteractive<BitFlipperParams> for Slider {}
impl UiElement<BitFlipperParams> for Slider {
    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        _buffer: &[f32],
    ) {
        let value = params.pre_gain.value();
        let value = params.pre_gain.preview_normalized(value);

        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&SliderUniforms {
                uv_region: self.uv_region,
                value,
                _padding: [0.0; 3],
            }),
        );
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.shared_pipeline.pipeline);

        render_pass.set_bind_group(0, &self.shared_pipeline.tex_atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.position_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.uv_buffer.slice(..));

        render_pass.draw(0..6, 0..1);
    }
}

impl UiBox for Slider {
    fn width(&self) -> u16 {
        59
    }

    fn height(&self) -> u16 {
        8
    }

    fn position(&self) -> (u16, u16) {
        self.position
    }
}
