use std::collections::HashMap;

pub struct TextureAtlas {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub bounds_map: HashMap<&'static str, [u16; 4]>,
    tex_size: (u32, u32),
}

const UV_MAP: &[(&str, [u16; 4])] = &[
    ("background", [100, 50, 200, 150]),
    ("gui_main", [0, 0, 90, 151]),
    ("gui_monitors", [18, 154, 182, 199]),
];

impl TextureAtlas {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let img = image::load_from_memory(include_bytes!(
            "../../../assets/textures/__texture_atlas__.png"
        ))
        .unwrap()
        .to_rgba8();

        let (width, height) = img.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("TextureAtlas Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = create_sampler(device);

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("TextureAtlas BindGroup"),
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("TextureAtlas BindGroup"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let mut uv_map = HashMap::new();
        UV_MAP.iter().for_each(|(k, v)| {
            uv_map.insert(*k, *v);
        });

        Self {
            bind_group,
            layout,
            tex_size: (width, height),
            bounds_map: uv_map,
        }
    }

    pub fn get_uvs(&self, name: &str) -> Option<[f32; 12]> {
        if let Some([x1, y1, x2, y2]) = self.bounds_map.get(name) {
            let (width, height) = self.tex_size;

            let u0 = *x1 as f32 / width as f32;
            let v0 = *y1 as f32 / height as f32;
            let u1 = *x2 as f32 / width as f32;
            let v1 = *y2 as f32 / height as f32;

            let uvs = [
                u0, v0, u1, v0, u0, v1, // triangle 1
                u0, v1, u1, v0, u1, v1, // triangle 2
            ];

            return Some(uvs);
        }

        None
    }

    pub fn get_bounds(&self, name: &str) -> Option<[f32; 4]> {
        if let Some([x1, y1, x2, y2]) = self.bounds_map.get(name) {
            let (width, height) = self.tex_size;

            let bounds = [
                *x1 as f32 / width as f32,
                *y1 as f32 / height as f32,
                (*x2 as f32 - *x1 as f32) / width as f32,
                (*y2 as f32 - *y1 as f32) / height as f32,
            ];

            return Some(bounds);
        }

        None
    }

    pub fn get_size(&self, name: &str) -> Option<(u16, u16)> {
        if let Some([x1, y1, x2, y2]) = self.bounds_map.get(name) {
            let width = x2 - x1;
            let height = y2 - y1;

            return Some((width, height));
        }

        None
    }
}

pub fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}
