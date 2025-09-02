use wgpu::PrimitiveState;

use super::{UiElement, pipeline::create_pipeline, texture::create_sampler};

pub struct Postprocess {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl Postprocess {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        tex_format: wgpu::TextureFormat,
        grayscale_view: &wgpu::TextureView,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Postprocess Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("postprocess.wgsl").into()),
        });

        let img = image::load_from_memory(include_bytes!(
            "../../../assets/textures/__palette__.png"
        ))
        .unwrap()
        .to_rgba8();

        let palette_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Palette Texture"),
            size: wgpu::Extent3d {
                width: 7,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let palette_view = palette_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = create_sampler(device);

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &palette_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 8),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 7,
                height: 1,
                depth_or_array_layers: 1,
            },
        );

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Postprocess bind group layout"),
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
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Postprocess bind group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(grayscale_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&palette_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline = create_pipeline(
            device,
            tex_format,
            &[&layout],
            &[],
            PrimitiveState::default(),
            &shader,
        );

        Self {
            pipeline,
            bind_group,
        }
    }
}

impl UiElement for Postprocess {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.draw(0..6, 0..1);
    }
}
