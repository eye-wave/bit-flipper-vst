use super::{
    UiBox, UiElement,
    pipeline::{SharedPipeline, create_pipeline},
};
use crate::editor::{VIEW_HEIGHT, VIEW_WIDTH};
use std::{f32::consts::PI, sync::Arc};
use wgpu::{RenderPipeline, util::DeviceExt};

const MONITOR_WIDTH: u16 = 76;
const MONITOR_HEIGHT: u16 = 42;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MonitorUniforms {
    pub position: [f32; 4],
    pub color: f32,
    pub _padding: [f32; 3], // align to 16 bytes
}

pub struct SharedMonitorPipeline {
    pipeline: RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Monitor {
    shared_pipeline: Arc<SharedMonitorPipeline>,
    position: (u16, u16),
    vertex_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl SharedMonitorPipeline {
    pub fn new(device: &wgpu::Device, tex_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Monitor Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("monitor.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Monitor Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(
                        std::mem::size_of::<MonitorUniforms>() as u64,
                    ),
                },
                count: None,
            }],
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 4,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        };

        let pipeline = create_pipeline(
            device,
            tex_format,
            &[&bind_group_layout],
            &[vertex_layout],
            wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                ..Default::default()
            },
            &shader,
        );

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl Monitor {
    pub fn new(
        device: &wgpu::Device,
        position: (u16, u16),
        pipeline: Arc<SharedMonitorPipeline>,
    ) -> Self {
        let mut samples = [0.0; MONITOR_WIDTH as usize * 2];
        for i in 0..MONITOR_WIDTH as usize {
            samples[i * 2] = i as f32 / MONITOR_WIDTH as f32;
            samples[i * 2 + 1] = (i as f32 / MONITOR_WIDTH as f32 * 2.0 * PI).sin();
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Position Buffer"),
            contents: bytemuck::cast_slice(&samples),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let uniforms = MonitorUniforms {
            position: [
                position.0 as f32 / VIEW_WIDTH as f32 * 2.0 - 1.0,
                position.1 as f32 / VIEW_HEIGHT as f32 * 2.0 - 1.0,
                MONITOR_WIDTH as f32,
                MONITOR_HEIGHT as f32,
            ],
            color: 6.0 / 7.0,
            _padding: [0.0; 3],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Monitor Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Monitor Bind Group"),
            layout: &pipeline.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            position,
            vertex_buffer,
            bind_group,
            shared_pipeline: pipeline,
        }
    }
}

impl SharedPipeline for SharedMonitorPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl UiElement for Monitor {
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, _queue: &wgpu::Queue) {
        render_pass.set_pipeline(&self.shared_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..MONITOR_WIDTH as u32, 0..1);
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl UiBox for Monitor {
    fn width(&self) -> u16 {
        MONITOR_WIDTH
    }

    fn height(&self) -> u16 {
        MONITOR_HEIGHT
    }

    fn position(&self) -> (u16, u16) {
        self.position
    }
}
