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
    UV_warning,
    UV_gui_main,
    UV_gui_monitors,
    UV_btn_xor,
    UV_btn_or,
    UV_btn_and,
    UV_btn_not,
    UV_btn_open,
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
    UV_num_0,
    UV_num_1,
    UV_num_2,
    UV_num_3,
    UV_num_4,
    UV_num_5,
    UV_num_6,
    UV_num_7,
    UV_num_8,
    UV_num_9,
    UV_num_dot,
    UV_num_minus,
    UV_num_d,
    UV_num_B,
}

impl UVSegment {
    fn into_not_found(self) -> TextureError {
        TextureError::NotFound(self)
    }
}

const UV_MAP: &[(UVSegment, [u16; 4])] = &[
    (UVSegment::UV_background, [107, 89, 100, 100]),
    (UVSegment::UV_warning, [90, 0, 117, 89]),
    (UVSegment::UV_gui_main, [0, 0, 88, 145]),
    (UVSegment::UV_gui_monitors, [0, 189, 163, 45]),
    (UVSegment::UV_btn_xor, [28, 173, 16, 16]),
    (UVSegment::UV_btn_or, [44, 173, 16, 16]),
    (UVSegment::UV_btn_and, [60, 173, 16, 16]),
    (UVSegment::UV_btn_not, [76, 173, 16, 16]),
    (UVSegment::UV_btn_open, [28, 157, 16, 16]),
    (UVSegment::UV_slider_handle, [73, 165, 19, 8]),
    //
    (UVSegment::UV_digi_1_0, [0, 153, 9, 6]),
    (UVSegment::UV_digi_1_1, [0, 159, 9, 6]),
    (UVSegment::UV_digi_1_2, [0, 165, 9, 6]),
    (UVSegment::UV_digi_1_3, [9, 153, 9, 6]),
    (UVSegment::UV_digi_1_4, [9, 159, 9, 6]),
    (UVSegment::UV_digi_1_5, [9, 165, 9, 6]),
    (UVSegment::UV_digi_1_6, [18, 153, 9, 6]),
    (UVSegment::UV_digi_1_7, [18, 159, 9, 6]),
    (UVSegment::UV_digi_1_8, [18, 165, 9, 6]),
    //
    (UVSegment::UV_digi_0_0, [0, 171, 9, 6]),
    (UVSegment::UV_digi_0_1, [0, 177, 9, 6]),
    (UVSegment::UV_digi_0_2, [0, 183, 9, 6]),
    (UVSegment::UV_digi_0_3, [9, 171, 9, 6]),
    (UVSegment::UV_digi_0_4, [9, 177, 9, 6]),
    (UVSegment::UV_digi_0_5, [9, 183, 9, 6]),
    (UVSegment::UV_digi_0_6, [18, 171, 9, 6]),
    (UVSegment::UV_digi_0_7, [18, 177, 9, 6]),
    (UVSegment::UV_digi_0_8, [18, 183, 9, 6]),
    //
    (UVSegment::UV_num_0, [50, 145, 6, 8]),
    (UVSegment::UV_num_1, [56, 145, 6, 8]),
    (UVSegment::UV_num_2, [62, 145, 6, 8]),
    (UVSegment::UV_num_3, [68, 145, 6, 8]),
    (UVSegment::UV_num_4, [74, 145, 6, 8]),
    (UVSegment::UV_num_dot, [81, 145, 6, 8]),
    (UVSegment::UV_num_minus, [86, 145, 6, 8]),
    (UVSegment::UV_num_5, [50, 153, 6, 8]),
    (UVSegment::UV_num_6, [56, 153, 6, 8]),
    (UVSegment::UV_num_7, [62, 153, 6, 8]),
    (UVSegment::UV_num_8, [68, 153, 6, 8]),
    (UVSegment::UV_num_9, [74, 153, 6, 8]),
    (UVSegment::UV_num_d, [80, 153, 6, 8]),
    (UVSegment::UV_num_B, [86, 153, 6, 8]),
];

impl TextureAtlas {
    pub fn new(device: &wgpu::Device, texture: &[u8], queue: &wgpu::Queue) -> Self {
        let img = image::load_from_memory(texture).unwrap().to_rgba8();

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
        if let Some([x, y, w, h]) = self.bounds_map.get(name) {
            let (tex_width, tex_height) = self.tex_size;

            let u0 = *x as f32 / tex_width as f32;
            let v0 = *y as f32 / tex_height as f32;
            let u1 = (*x + *w) as f32 / tex_width as f32;
            let v1 = (*y + *h) as f32 / tex_height as f32;

            let uvs = [u0, v0, u1, v0, u0, v1, u0, v1, u1, v0, u1, v1];

            return Ok(uvs);
        }

        Err(name.into_not_found())
    }

    pub fn get_bounds(&self, name: &UVSegment) -> Result<[f32; 4], TextureError> {
        if let Some([x, y, w, h]) = self.bounds_map.get(name) {
            let (tex_width, tex_height) = self.tex_size;

            let bounds = [
                *x as f32 / tex_width as f32,
                *y as f32 / tex_height as f32,
                *w as f32 / tex_width as f32,
                *h as f32 / tex_height as f32,
            ];

            return Ok(bounds);
        }

        Err(name.into_not_found())
    }

    pub fn get_size(&self, name: &UVSegment) -> Result<(u16, u16), TextureError> {
        if let Some([_, _, w, h]) = self.bounds_map.get(name) {
            return Ok((*w, *h));
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
