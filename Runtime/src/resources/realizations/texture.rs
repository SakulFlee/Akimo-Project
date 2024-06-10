use cgmath::Vector2;
use image::{DynamicImage, GenericImageView};
use log::warn;
use wgpu::Color;
use wgpu::Device;
use wgpu::Queue;
use wgpu::Texture as WTexture;
use wgpu::TextureDescriptor as WTextureDescriptor;
use wgpu::{
    AddressMode, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Sampler,
    SamplerDescriptor, TextureAspect, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use crate::resources::descriptors::TextureDescriptor;

pub struct Texture {
    texture: WTexture,
    view: TextureView,
    sampler: Sampler,
}

impl Texture {
    pub fn from_descriptor(descriptor: &TextureDescriptor, device: &Device, queue: &Queue) -> Self {
        match descriptor {
            TextureDescriptor::StandardSRGBu8Image(image) => {
                Self::standard_srgb8_image(image, device, queue)
            }
            TextureDescriptor::StandardSRGBu8Data(data, size) => {
                Self::standard_srgb8_data(data, size, device, queue)
            }
            TextureDescriptor::UniformColor(color) => Self::uniform_color(*color, device, queue),
            TextureDescriptor::Depth(size) => Self::depth_texture(size, device, queue),
            TextureDescriptor::Custom(
                texture_descriptor,
                texture_view_descriptor,
                sampler_descriptor,
            ) => Self::from_descriptors(
                texture_descriptor,
                texture_view_descriptor,
                sampler_descriptor,
                device,
                queue,
            ),
        }
    }

    /// In case you want a uniform, one color, image.
    /// This results in an 1-by-1 px, i.e. 4 bytes image.
    ///
    /// ⚠️ This can be used as an empty texture as there is as minimal
    /// ⚠️ as possible data usage and this resource may not even arrive
    /// ⚠️ in the shader _if_ it is not used.
    pub fn uniform_color(color: Color, device: &Device, queue: &Queue) -> Self {
        let r = ((color.r * 256.0) as u8).min(255).max(0);
        let g = ((color.g * 256.0) as u8).min(255).max(0);
        let b = ((color.b * 256.0) as u8).min(255).max(0);
        let a = ((color.a * 256.0) as u8).min(255).max(0);

        Self::standard_srgb8_data(&[r, g, b, a], &(1, 1).into(), device, queue)
    }

    pub fn standard_srgb8_image(image: &DynamicImage, device: &Device, queue: &Queue) -> Self {
        Self::standard_srgb8_data(
            &image.to_rgba8(),
            &(image.dimensions().0, image.dimensions().1).into(),
            device,
            queue,
        )
    }

    pub fn standard_srgb8_data(
        data: &[u8],
        size: &Vector2<u32>,
        device: &Device,
        queue: &Queue,
    ) -> Self {
        Self::from_data_srgb8(
            data,
            &WTextureDescriptor {
                label: Some("Standard SRGB u8 Data Texture"),
                size: Extent3d {
                    width: size.x,
                    height: size.y,
                    ..Default::default()
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[],
            },
            &TextureViewDescriptor::default(),
            &SamplerDescriptor {
                label: Some("Standard SRGB u8 Data Texture Sampler"),
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Nearest,
                ..Default::default()
            },
            device,
            queue,
        )
    }

    pub fn from_image_srgb8(
        image: DynamicImage,
        texture_desc: &WTextureDescriptor,
        view_desc: &TextureViewDescriptor,
        sampler_desc: &SamplerDescriptor,
        device: &Device,
        queue: &Queue,
    ) -> Self {
        if texture_desc.size.width != image.dimensions().0
            || texture_desc.size.height != image.dimensions().1
        {
            warn!("Image supplied has different dimensions from what the texture description expects! This may lead to undefined behaviour.");
        }

        Self::from_data_srgb8(
            &image.to_rgba8(),
            texture_desc,
            view_desc,
            sampler_desc,
            device,
            queue,
        )
    }

    pub fn from_data_srgb8(
        data: &[u8],
        texture_desc: &WTextureDescriptor,
        view_desc: &TextureViewDescriptor,
        sampler_desc: &SamplerDescriptor,
        device: &Device,
        queue: &Queue,
    ) -> Self {
        let texture = Self::from_descriptors(texture_desc, view_desc, sampler_desc, device, queue);

        queue.write_texture(
            ImageCopyTexture {
                texture: texture.texture(),
                aspect: TextureAspect::All,
                origin: Origin3d::ZERO,
                mip_level: 0,
            },
            data,
            ImageDataLayout {
                offset: 0,
                // 4 bytes (RGBA), times the width
                bytes_per_row: Some(4 * texture_desc.size.width),
                // ... times height
                rows_per_image: Some(texture_desc.size.height),
            },
            texture_desc.size,
        );

        texture
    }

    fn depth_texture(size: &Vector2<u32>, device: &Device, queue: &Queue) -> Texture {
        Self::from_descriptors(
            &WTextureDescriptor {
                label: Some("Depth Texture"),
                size: Extent3d {
                    width: size.x,
                    height: size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            &TextureViewDescriptor::default(),
            &SamplerDescriptor {
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            },
            device,
            queue,
        )
    }

    pub fn from_descriptors(
        texture_desc: &WTextureDescriptor,
        view_desc: &TextureViewDescriptor,
        sampler_desc: &SamplerDescriptor,
        device: &Device,
        _queue: &Queue,
    ) -> Self {
        let texture = device.create_texture(texture_desc);
        let view = texture.create_view(view_desc);
        let sampler = device.create_sampler(sampler_desc);

        Self::from_existing(texture, view, sampler)
    }

    pub fn from_existing(texture: WTexture, view: TextureView, sampler: Sampler) -> Self {
        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn texture(&self) -> &WTexture {
        &self.texture
    }

    pub fn view(&self) -> &TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}
