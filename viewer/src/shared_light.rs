//! The hour's light outdoors (the ambient, the sun's colour and direction,
//! the fog), kept once for every surface instead of in each material.
//!
//! Outdoors the light changes with the clock (`crate::daylight`, a game
//! minute at a time, two real seconds at `TimeScale` 30). Written into
//! each lit, terrain and distant-land material, every change made Bevy
//! upload every material's uniform and build its bind group again: 20 to
//! 30 ms in Goodsprings, a hitch every two seconds. Here the light is a
//! storage buffer every material binds ([`BUFFER`]); its asset never
//! changes after it's made, so the materials' bind groups stay as they
//! are, and a render-world system writes the light into the buffer
//! itself when it changes.
//!
//! The lit surfaces' materials are bindless where the device allows, and
//! bindless materials can't bind a buffer of their own: they read the same
//! light from [`TEXTURE`] instead, five texels (one per vector, 32-bit
//! floats, so the values are exactly the buffer's), written alongside it.
//!
//! The terrain and the distant land are outdoors only and always read it;
//! a lit surface reads it when its `GameLighting::scale.z` is 1, which
//! `crate::daylight` sets on the surfaces it lights (not the menus' 3D
//! pieces, marked [`MenuLit`], which keep their own light).

// The shader-layout derive generates checking functions the compiler
// reports as unused.
#![allow(dead_code)]

use bevy::asset::weak_handle;
use bevy::image::Image;
use bevy::prelude::*;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_asset::{RenderAssetUsages, RenderAssets};
use bevy::render::render_resource::{
    encase, BufferUsages, Extent3d, Origin3d, ShaderType, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::renderer::RenderQueue;
use bevy::render::storage::{GpuShaderStorageBuffer, ShaderStorageBuffer};
use bevy::render::texture::GpuImage;
use bevy::render::{Render, RenderApp, RenderSet};

/// The buffer every outdoor surface's material binds.
pub const BUFFER: Handle<ShaderStorageBuffer> =
    weak_handle!("8d2c1f6a-4e7b-4b9a-a3d5-61c0e9f2b7d4");

/// The same, as five texels (`GameLit` reads it texel by texel).
pub const TEXTURE: Handle<Image> = weak_handle!("2f7d0b8e-91c4-4f3a-b6e2-7a5c3d9e4f10");

/// The hour's light, in the materials' units (`crate::LightFields`).
#[derive(Clone, Copy, Debug, Default, PartialEq, ShaderType)]
pub struct SharedLight {
    pub ambient: Vec4,
    pub directional_color: Vec4,
    /// Toward the light.
    pub directional_direction: Vec4,
    pub fog_color: Vec4,
    pub fog_range: Vec4,
}

/// The light now (set by `crate::daylight`).
#[derive(Resource, Clone, Copy, Default, ExtractResource)]
pub struct SharedLightNow(pub SharedLight);

/// A 3D menu's piece (lockpicking, the Caravan table), lit by its menu's
/// own lights rather than the hour's.
#[derive(Component)]
pub struct MenuLit;

pub struct SharedLightPlugin;

impl Plugin for SharedLightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SharedLightNow>()
            .add_plugins(ExtractResourcePlugin::<SharedLightNow>::default())
            .add_systems(Startup, (make_buffer, make_texture));
        if let Some(render) = app.get_sub_app_mut(RenderApp) {
            render.add_systems(
                Render,
                (write_buffer, write_texture).in_set(RenderSet::PrepareResources),
            );
        }
    }
}

fn bytes(light: &SharedLight) -> Vec<u8> {
    let mut out = encase::StorageBuffer::new(Vec::new());
    out.write(light).expect("the shared light fits its buffer");
    out.into_inner()
}

/// The buffer, made once (writable in place).
fn make_buffer(mut buffers: ResMut<Assets<ShaderStorageBuffer>>, now: Res<SharedLightNow>) {
    let mut buffer = ShaderStorageBuffer::new(&bytes(&now.0), RenderAssetUsages::default());
    buffer.buffer_description.usage = BufferUsages::STORAGE | BufferUsages::COPY_DST;
    buffers.insert(BUFFER.id(), buffer);
}

/// The five vectors as texels (RGBA, 32-bit floats).
fn texels(light: &SharedLight) -> Vec<u8> {
    [
        light.ambient,
        light.directional_color,
        light.directional_direction,
        light.fog_color,
        light.fog_range,
    ]
    .iter()
    .flat_map(|v| v.to_array())
    .flat_map(f32::to_le_bytes)
    .collect()
}

/// The texture, made once (writable in place).
fn make_texture(mut images: ResMut<Assets<Image>>, now: Res<SharedLightNow>) {
    let mut image = Image::new(
        Extent3d {
            width: 5,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texels(&now.0),
        TextureFormat::Rgba32Float,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    images.insert(TEXTURE.id(), image);
}

/// The light into the texture when it changed, or the texture is new on
/// the GPU.
fn write_texture(
    now: Res<SharedLightNow>,
    images: Res<RenderAssets<GpuImage>>,
    queue: Res<RenderQueue>,
    mut written: Local<Option<(SharedLight, bevy::render::render_resource::TextureId)>>,
) {
    let Some(gpu) = images.get(TEXTURE.id()) else {
        return;
    };
    let id = gpu.texture.id();
    if *written == Some((now.0, id)) {
        return;
    }
    queue.write_texture(
        TexelCopyTextureInfo {
            texture: &gpu.texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        &texels(&now.0),
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(5 * 16),
            rows_per_image: Some(1),
        },
        Extent3d {
            width: 5,
            height: 1,
            depth_or_array_layers: 1,
        },
    );
    *written = Some((now.0, id));
}

/// The light into the buffer when it changed (once the buffer's on the
/// GPU).
fn write_buffer(
    now: Res<SharedLightNow>,
    buffers: Res<RenderAssets<GpuShaderStorageBuffer>>,
    queue: Res<RenderQueue>,
    mut written: Local<Option<SharedLight>>,
) {
    if *written == Some(now.0) {
        return;
    }
    let Some(gpu) = buffers.get(BUFFER.id()) else {
        return;
    };
    queue.write_buffer(&gpu.buffer, 0, &bytes(&now.0));
    *written = Some(now.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Five vectors, as the shaders' `SharedLight` reads them.
    #[test]
    fn the_buffer_is_five_vectors() {
        let light = SharedLight {
            ambient: Vec4::new(1.0, 2.0, 3.0, 4.0),
            fog_range: Vec4::splat(9.0),
            ..default()
        };
        let b = bytes(&light);
        assert_eq!(b.len(), 5 * 16);
        assert_eq!(&b[..4], &1.0f32.to_le_bytes());
        assert_eq!(&b[64..68], &9.0f32.to_le_bytes());
    }

    /// The texture holds the buffer's numbers: the same five vectors in the
    /// same order, a texel each.
    #[test]
    fn the_texture_is_the_buffer() {
        let light = SharedLight {
            ambient: Vec4::new(1.0, 2.0, 3.0, 4.0),
            directional_color: Vec4::splat(5.0),
            directional_direction: Vec4::new(0.0, 0.6, 0.8, 0.0),
            fog_color: Vec4::splat(0.25),
            fog_range: Vec4::new(1000.0, 250_000.0, 0.1, 1.0),
        };
        assert_eq!(texels(&light), bytes(&light));
    }
}
