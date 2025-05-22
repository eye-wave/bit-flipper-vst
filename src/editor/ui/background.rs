use super::{
    UiElement,
    pipeline::{SharedPipeline, create_pipeline},
    texture::create_texture,
};
use std::{any::Any, sync::Arc};

pub struct BackgroundPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

pub struct Background {
    shared_pipeline: Arc<BackgroundPipeline>,
}

impl BackgroundPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Background Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("background.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Background BindGroupLayout"),
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
            ],
        });

        let pipeline = create_pipeline(device, config.format, &[&bind_group_layout], &[], &shader);

        let img = image::load_from_memory(include_bytes!("../../../assets/textures/__base__.png"))
            .unwrap()
            .to_rgba8();

        let (view, sampler) = create_texture(device, img, queue);

        // Bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Background BindGroup"),
        });

        Self {
            pipeline,
            bind_group,
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
    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, _queue: &wgpu::Queue) {
        render_pass.set_pipeline(self.shared_pipeline.pipeline());
        render_pass.set_bind_group(0, &self.shared_pipeline.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
