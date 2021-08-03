use crate::renderer;
use std::num::NonZeroU32;
use crate::rendering::renderer;

pub const MATRIX_UNIFORM_LAYOUT_DESC: &'a wgpu::BindGroupLayoutDescriptor<'a> =
    &wgpu::BindGroupLayoutDescriptor {
        label: Some("MATRIX_UNIFORM_LAYOUT_DESC"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };

pub const SAMPLED_TEXTURE_AND_SAMPLER_LAYOUT_DESC: &'a wgpu::BindGroupLayoutDescriptor<'a> =
    &wgpu::BindGroupLayoutDescriptor {
        label: Some("SAMPLED_TEXTURE_AND_SAMPLER_LAYOUT_DESC"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    };

pub const SAMPLED_TEXTURE_ARRAY_AND_SAMPLER_LAYOUT_DESC: &'a wgpu::BindGroupLayoutDescriptor<'a> =
    &wgpu::BindGroupLayoutDescriptor {
        label: Some("SAMPLED_TEXTURE_ARRAY_AND_SAMPLER_LAYOUT_DESC"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(renderer::SAMPLED_TEXTURES_COUNT),
            },
        ],
    };
