use crate::{
    BitFlipperParams,
    editor::{VIEW_HEIGHT, VIEW_WIDTH},
};

use boxi::prelude::*;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct ColorBoxPipeline {
    pub(super) pipeline: wgpu::RenderPipeline,
    color_bind_group_layout: wgpu::BindGroupLayout,
}

pub struct ColorBox {
    shared_pipeline: Arc<ColorBoxPipeline>,
    position: (u16, u16),
    width: u16,
    height: u16,
    vertex_buffer: wgpu::Buffer,
    color_bind_group: wgpu::BindGroup,
}

impl ColorBoxPipeline {
    pub fn new(device: &wgpu::Device, tex_format: wgpu::TextureFormat) -> Self {
        let color_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ColorBox Uniform Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<[f32; 4]>() as _
                        ),
                    },
                    count: None,
                }],
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ColorBox Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("color_box.wgsl").into()),
        });

        let vertex_buffer_layouts = &[wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }];

        let pipeline = create_pipeline(
            device,
            tex_format,
            &[&color_bind_group_layout],
            vertex_buffer_layouts,
            wgpu::PrimitiveState::default(),
            &shader,
        );

        Self {
            pipeline,
            color_bind_group_layout,
        }
    }
}

impl ColorBox {
    pub fn new(
        device: &wgpu::Device,
        position: (u16, u16),
        width: u16,
        height: u16,
        color: [f32; 4],
        pipeline: Arc<ColorBoxPipeline>,
    ) -> Self {
        let (x, y) = position;
        let x = (x as f32 / VIEW_WIDTH as f32) * 2.0 - 1.0;
        let y = (y as f32 / VIEW_WIDTH as f32) * 2.0 - 1.0;
        let w = (width as f32 / VIEW_WIDTH as f32) * 2.0;
        let h = (height as f32 / VIEW_HEIGHT as f32) * 2.0;

        #[rustfmt::skip]
        let vertex_data: [f32; 12] = [
            x, y,
            (x + w), y,
            x,(y + h),
            x,(y + h),
            (x + w) , y,
            (x + w) ,(y + h),
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ColorBox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create uniform buffer for color
        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ColorBox Color Buffer"),
            contents: bytemuck::cast_slice(&color),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ColorBox BindGroup"),
            layout: &pipeline.color_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
        });

        Self {
            shared_pipeline: pipeline,
            position,
            width,
            height,
            vertex_buffer,
            color_bind_group,
        }
    }
}

impl UiElement<BitFlipperParams> for ColorBox {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.shared_pipeline.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.color_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

impl UiBox for ColorBox {
    fn height(&self) -> u16 {
        self.height
    }
    fn width(&self) -> u16 {
        self.width
    }
    fn position(&self) -> (u16, u16) {
        (self.position.0, self.position.1)
    }
}
