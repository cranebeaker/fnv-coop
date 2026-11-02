//! Textures scripts swap on placed objects (`SwapTextureOnRef`,
//! `world::scripting::Event::SwapTexture`): the game finds the node by its
//! exact name in the reference's 3D (vtable +0x9c) and gives it
//! `Textures\<name>.dds` in its first slot (`005cf860`). Dead Money's intro
//! slides change this way. Here the piece of the placed object whose NIF
//! shape has that name takes a material of its own with the new texture.
//! Only while the object is drawn: nothing is kept (as in the game, where
//! the swap is on the loaded 3D).

use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::settings::WgpuFeatures;
use esm::FormId;

use crate::lighting::GameLitMaterial;
use crate::scripts::PlacedRef;
use crate::GameFiles;

/// The NIF shape name of a placed object's piece (`MeshData::shape_name`).
#[derive(Component, Debug, Clone)]
pub struct PieceName(pub String);

/// Swaps scripts asked for: the reference, the node's name and the texture
/// path (`Textures\….dds`).
#[derive(Resource, Default)]
pub struct TextureSwaps(pub Vec<(FormId, String, String)>);

/// Carries out the swaps asked for.
#[allow(clippy::too_many_arguments)]
pub fn swap_textures(
    game: Res<GameFiles>,
    settings: Res<crate::Settings>,
    device: Option<Res<RenderDevice>>,
    mut swaps: ResMut<TextureSwaps>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<GameLitMaterial>>,
    mut pieces: Query<(&PlacedRef, &PieceName, &mut MeshMaterial3d<GameLitMaterial>)>,
    mut loaded: Local<HashMap<String, Option<Handle<Image>>>>,
) {
    if swaps.0.is_empty() {
        return;
    }
    let compressed = device
        .as_ref()
        .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));
    for (what, node, path) in std::mem::take(&mut swaps.0) {
        let image = loaded
            .entry(path.to_ascii_lowercase())
            .or_insert_with(|| {
                let bytes = game.0.assets.read(&path).ok().flatten()?;
                let texture = cellview::TextureData::from_dds(path.clone(), bytes)
                    .map_err(|e| println!("Can't read {path}: {e}"))
                    .ok()?;
                crate::upload_texture(&mut images, &texture, compressed, settings.anisotropy)
            })
            .clone();
        let Some(image) = image else {
            println!("A script swaps in {path}, which isn't in the game's files.");
            continue;
        };
        let mut swapped = 0;
        for (r, name, mut material) in &mut pieces {
            if r.0 != what.0 || !name.0.eq_ignore_ascii_case(&node) {
                continue;
            }
            let Some(mut own) = materials.get(&material.0).cloned() else {
                continue;
            };
            own.base.base_color_texture = Some(image.clone());
            material.0 = materials.add(own);
            swapped += 1;
        }
        if swapped == 0 {
            println!("A script swaps {node}'s texture on {what}, which isn't drawn here.");
        } else {
            println!("{what}'s {node} now shows {path}.");
        }
    }
}
