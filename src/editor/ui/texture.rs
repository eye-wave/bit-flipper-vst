use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum TextureError {
    NotFound(UVSegment),
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(str) => write!(f, "UV map \"{str:?}\" does not exist."),
        }
    }
}

impl std::error::Error for TextureError {}

pub struct TextureAtlas {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    pub bounds_map: HashMap<UVSegment, [u16; 4]>,
    tex_size: (u32, u32),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UVSegment {
    UV_background,
    UV_gui_main,
    UV_gui_monitors,
    UV_btn_xor,
    UV_btn_or,
    UV_btn_and,
    UV_btn_not,
    UV_slider_handle,
    UV_digi_1_0,
    UV_digi_1_1,
    UV_digi_1_2,
    UV_digi_1_3,
    UV_digi_1_4,
    UV_digi_1_5,
    UV_digi_1_6,
    UV_digi_1_7,
    UV_digi_1_8,
    UV_digi_0_0,
    UV_digi_0_1,
    UV_digi_0_2,
    UV_digi_0_3,
    UV_digi_0_4,
    UV_digi_0_5,
    UV_digi_0_6,
    UV_digi_0_7,
    UV_digi_0_8,
}

impl UVSegment {
    fn into_not_found(self) -> TextureError {
        TextureError::NotFound(self)
    }
}

const UV_MAP: &[(UVSegment, [u16; 4])] = &[
    (UVSegment::UV_background, [100, 50, 200, 150]),
    (UVSegment::UV_gui_main, [0, 0, 90, 151]),
    (UVSegment::UV_gui_monitors, [18, 154, 182, 199]),
    (UVSegment::UV_btn_xor, [100, 33, 116, 49]),
    (UVSegment::UV_btn_or, [116, 33, 132, 49]),
    (UVSegment::UV_btn_and, [132, 33, 148, 49]),
    (UVSegment::UV_btn_not, [148, 33, 164, 49]),
    (UVSegment::UV_slider_handle, [172, 39, 191, 47]),
    //
    (UVSegment::UV_digi_1_0, [173, 0, 182, 6]),
    (UVSegment::UV_digi_1_1, [182, 0, 191, 6]),
    (UVSegment::UV_digi_1_2, [191, 0, 200, 6]),
    (UVSegment::UV_digi_1_3, [173, 6, 182, 12]),
    (UVSegment::UV_digi_1_4, [182, 6, 191, 12]),
    (UVSegment::UV_digi_1_5, [191, 6, 200, 12]),
    (UVSegment::UV_digi_1_6, [173, 12, 182, 18]),
    (UVSegment::UV_digi_1_7, [182, 12, 191, 18]),
    (UVSegment::UV_digi_1_8, [191, 12, 200, 18]),
    //
    (UVSegment::UV_digi_0_0, [173, 18, 182, 24]),
    (UVSegment::UV_digi_0_1, [182, 18, 191, 24]),
    (UVSegment::UV_digi_0_2, [191, 18, 200, 24]),
    (UVSegment::UV_digi_0_3, [173, 24, 182, 30]),
    (UVSegment::UV_digi_0_4, [182, 24, 191, 30]),
    (UVSegment::UV_digi_0_5, [191, 24, 200, 30]),
    (UVSegment::UV_digi_0_6, [173, 30, 182, 36]),
    (UVSegment::UV_digi_0_7, [182, 30, 191, 36]),
    (UVSegment::UV_digi_0_8, [191, 30, 200, 36]),
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

    pub fn get_uvs(&self, name: &UVSegment) -> Result<[f32; 12], TextureError> {
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

            return Ok(uvs);
        }

        Err(name.into_not_found())
    }

    pub fn get_bounds(&self, name: &UVSegment) -> Result<[f32; 4], TextureError> {
        if let Some([x1, y1, x2, y2]) = self.bounds_map.get(name) {
            let (width, height) = self.tex_size;

            let bounds = [
                *x1 as f32 / width as f32,
                *y1 as f32 / height as f32,
                (*x2 as f32 - *x1 as f32) / width as f32,
                (*y2 as f32 - *y1 as f32) / height as f32,
            ];

            return Ok(bounds);
        }

        Err(name.into_not_found())
    }

    pub fn get_size(&self, name: &UVSegment) -> Result<(u16, u16), TextureError> {
        if let Some([x1, y1, x2, y2]) = self.bounds_map.get(name) {
            let width = x2 - x1;
            let height = y2 - y1;

            return Ok((width, height));
        }

        Err(name.into_not_found())
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
