use super::{
    UiBox, UiElement,
    pipeline::{SharedPipeline, create_pipeline},
};
use std::{f32::consts::PI, sync::Arc};
use wgpu::{RenderPipeline, util::DeviceExt};

use crate::editor::{VIEW_HEIGHT, VIEW_WIDTH};

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
            array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32,
            }],
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
        let samples = sine_wave::<{ MONITOR_WIDTH as usize }>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Position Buffer"),
            contents: bytemuck::cast_slice(&samples),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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

    fn custom_prerender(&mut self, queue: &wgpu::Queue, buffer: &[f32; MONITOR_WIDTH as usize]) {
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(buffer));
    }
}

impl SharedPipeline for SharedMonitorPipeline {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

impl UiElement for Monitor {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.shared_pipeline.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..MONITOR_WIDTH as u32, 0..1);
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
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

pub struct MonitorGroup {
    monitor_1: Monitor,
    monitor_2: Monitor,
}

impl MonitorGroup {
    pub fn new(
        device: &wgpu::Device,
        positions: [(u16, u16); 2],
        pipeline: Arc<SharedMonitorPipeline>,
    ) -> Self {
        Self {
            monitor_1: Monitor::new(device, positions[0], Arc::clone(&pipeline)),
            monitor_2: Monitor::new(device, positions[1], Arc::clone(&pipeline)),
        }
    }
}

impl UiElement for MonitorGroup {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn prerender(
        &mut self,
        queue: &wgpu::Queue,
        params: Arc<crate::BitFlipperParams>,
        buffer: &[f32],
    ) {
        let resampled = resample_into::<{ MONITOR_WIDTH as usize }>(buffer);
        let mut remapped = sine_wave::<{ MONITOR_WIDTH as usize }>();

        let mask = params.bits.to_u32();
        let mode = params.mode.value();

        for sample in remapped.iter_mut() {
            *sample = mode.transform(*sample, mask);
        }

        self.monitor_1.custom_prerender(queue, &resampled);
        self.monitor_2.custom_prerender(queue, &remapped);
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.monitor_1.render(render_pass);
        self.monitor_2.render(render_pass);
    }
}

fn sine_wave<const N: usize>() -> [f32; N] {
    let mut samples = [0.0f32; N];
    for (i, sample) in samples.iter_mut().enumerate() {
        *sample = (i as f32 / N as f32 * 2.0 * PI).sin();
    }

    samples
}

fn resample_into<const N: usize>(buffer: &[f32]) -> [f32; N] {
    let mut out = [0.0; N];

    if buffer.is_empty() {
        return out;
    }

    let src_len = buffer.len() as f32;
    let dst_len = N as f32;

    for (i, item) in out.iter_mut().enumerate().take(N) {
        let t = i as f32 * (src_len - 1.0) / (dst_len - 1.0);
        let idx = t.floor() as usize;
        let frac = t - idx as f32;

        let next_idx = if idx + 1 < buffer.len() { idx + 1 } else { idx };

        *item = buffer[idx] * (1.0 - frac) + buffer[next_idx] * frac;
    }

    out
}
