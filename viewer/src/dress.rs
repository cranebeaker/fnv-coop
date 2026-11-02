//! People redrawn when what they wear or hold changes: a script's
//! `EquipItem` / `UnequipItem`, clothes or a weapon taken away
//! (`RemoveItem`, `RemoveAllItems`, trading, stealing, looting), a weapon
//! swapped or put away in a fight.
//!
//! The game rebuilds an actor's model part by part: equipping puts the
//! item's models into the biped's slots (`Actor::AddWornItem`, Xbox PDB,
//! `0088db20` → `TESNPC::InitWornObject` `006061b0` → `TESBipedModelForm::
//! AddToBiped` `00480bd0` → `BipedAnim::SetBipedPart` `004abad0`), the
//! parts taken off are detached at once (`BipedAnim::RemovePart`), and the
//! process's model update (`HighProcess::Update3dModel`, flag 1) loads the
//! new ones (`BipedAnim::LoadBipedParts`, Xbox `822fd188`): a slot whose
//! model didn't change keeps its loaded 3D. A new weapon model re-hangs
//! the weapon where its drawn state has it (`004ab750`,
//! `ForceWeaponDrawnSheathed`).
//!
//! Here: what's worn (`world::outfit::worn_armour`) and held
//! (`world::combat::weapon_in_hand`, kept on the rig by `actors`) are
//! compared each frame with what the person was built from; when they
//! differ, the look they make (`world::actor::npc_look_wearing`) is
//! compared part by part with the one drawn: the pieces of parts that went
//! are despawned, the parts that came are built (one lone-actor scene for
//! all of them) and skinned to the person's own joints, and the rest stay
//! as they are.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use esm::FormId;
use world::ActorLook;

use crate::actors::ActorRig;
use crate::lighting::{GameLighting, GameLitMaterial};
use crate::{GameFiles, Spawner};

/// An actor piece: which part of its actor's look ([`Dressed::look`]) it
/// was built from.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece(pub u16);

/// A person's root: the look their pieces were built from, and the
/// clothes and weapon that look was made for.
#[derive(Component)]
pub struct Dressed {
    pub look: Arc<ActorLook>,
    /// The clothes and armour worn and the weapon held, once known (at
    /// first: what the record gives, which the look was built from).
    worn: Option<(Vec<FormId>, Option<FormId>)>,
}

impl Dressed {
    pub fn new(look: Arc<ActorLook>) -> Dressed {
        Dressed { look, worn: None }
    }
}

/// What people wore from the start, by base record (read once each).
#[derive(Resource, Default)]
pub struct StartWorn(HashMap<FormId, Arc<world::outfit::StartWorn>>);

/// How the parts of the look drawn map onto the parts of the look wanted:
/// for each drawn part, the wanted part it stays as (`None`: it goes), and
/// the wanted parts not drawn yet. Equal parts pair up in order.
pub fn match_parts(
    drawn: &[world::ActorPart],
    wanted: &[world::ActorPart],
) -> (Vec<Option<u16>>, Vec<u16>) {
    let mut taken = vec![false; wanted.len()];
    let stays = drawn
        .iter()
        .map(|part| {
            let j = (0..wanted.len()).find(|&j| !taken[j] && wanted[j] == *part)?;
            taken[j] = true;
            u16::try_from(j).ok()
        })
        .collect();
    let new = (0..wanted.len())
        .filter(|&j| !taken[j])
        .filter_map(|j| u16::try_from(j).ok())
        .collect();
    (stays, new)
}

/// A redraw being built on its own thread, as the game queues an actor's
/// 3D work for its task threads (`QueueCharacterReset3D`, Xbox PDB, when
/// `ShouldQueue3DTask`): the old pieces stay until it's put on.
#[derive(Component)]
pub struct Redrawing {
    task: Option<std::thread::JoinHandle<Redraw>>,
    /// The clothes and weapon it's for (kept as drawn if it fails).
    worn: (Vec<FormId>, Option<FormId>),
}

/// What a redraw thread made: the look wanted, how the drawn parts map onto
/// it ([`match_parts`]), the new parts' scene and the weapon kind's holster
/// pose.
pub struct Redraw {
    worn: (Vec<FormId>, Option<FormId>),
    wanted: ActorLook,
    stays: Vec<Option<u16>>,
    new: Vec<u16>,
    scene: Option<cellview::ViewerScene>,
    holster: Option<Option<Arc<nif::Sequence>>>,
    took: f64,
}

/// What a redraw thread does: the look `worn` makes (the one drawn when
/// it can't be made), matched with the one drawn, and the parts that came
/// built.
fn build(
    game: Arc<cellview::Game>,
    base: FormId,
    drawn: Arc<ActorLook>,
    worn: (Vec<FormId>, Option<FormId>),
) -> Redraw {
    let started = std::time::Instant::now();
    let wanted = world::actor::npc_look_wearing(&game.order, base, &worn.0, worn.1)
        .unwrap_or_else(|| (*drawn).clone());
    let (stays, new) = match_parts(&drawn.parts, &wanted.parts);
    let scene = (!new.is_empty()).then(|| {
        game.actor_scene(&ActorLook {
            parts: new
                .iter()
                .map(|&j| wanted.parts[usize::from(j)].clone())
                .collect(),
            ..wanted.clone()
        })
    });
    let holster =
        (wanted.fighting != drawn.fighting).then(|| holster(&game, wanted.fighting.as_ref()));
    Redraw {
        worn,
        wanted,
        stays,
        new,
        scene,
        holster,
        took: started.elapsed().as_secs_f64() * 1000.0,
    }
}

/// Each frame, after the people have been posed: anyone whose clothes or
/// weapon changed gets a redraw started; a redraw that's ready is put on
/// them (the pieces of the parts that went despawned, the new ones
/// spawned, the weapon re-hung).
#[allow(clippy::too_many_arguments)]
pub fn redress(
    mut commands: Commands,
    game: Res<GameFiles>,
    state: Res<crate::dialogue::DialogueState>,
    mut start: ResMut<StartWorn>,
    mut spawner: Spawner,
    mut people: Query<(
        Entity,
        &crate::ai::Walker,
        &mut Dressed,
        &mut ActorRig,
        Option<&mut Redrawing>,
    )>,
    mut pieces: Query<(
        Entity,
        &mut Piece,
        &ChildOf,
        &MeshMaterial3d<GameLitMaterial>,
    )>,
) {
    let order = &game.0.order;
    let state = &state.0;
    for (root, walker, mut dressed, mut rig, redrawing) in &mut people {
        if let Some(mut redrawing) = redrawing {
            if redrawing.task.as_ref().is_some_and(|t| !t.is_finished()) {
                continue;
            }
            let done = redrawing.task.take().and_then(|t| t.join().ok());
            commands.entity(root).remove::<Redrawing>();
            let Some(done) = done else {
                eprintln!("{}: the redraw failed", walker.reference);
                dressed.worn = Some(std::mem::take(&mut redrawing.worn));
                continue;
            };
            let started = std::time::Instant::now();
            // The person's pieces: kept ones renumbered, the others gone.
            let mut lighting: Option<GameLighting> = None;
            let mut gone = 0;
            for (entity, mut piece, child_of, material) in &mut pieces {
                if child_of.parent() != root {
                    continue;
                }
                if lighting.is_none() {
                    lighting = spawner
                        .lit_materials
                        .get(&material.0)
                        .map(|m| m.extension.lighting);
                }
                match done.stays.get(usize::from(piece.0)).copied().flatten() {
                    Some(j) => piece.0 = j,
                    None => {
                        commands.entity(entity).despawn();
                        gone += 1;
                    }
                }
            }
            let mut came = 0;
            if let (Some(scene), Some(lighting)) = (
                &done.scene,
                lighting.or_else(|| spawner.place_lighting.get()),
            ) {
                came = spawner
                    .spawn_actor_pieces(scene, lighting, root, &rig.joints, &done.new)
                    .len();
            }
            // Another weapon kind: hung where its holster pose has it; a
            // new weapon is put where the drawn state has it (`004ab750`).
            if let Some(holster) = done.holster {
                rig.skeleton = Arc::new(preview::cell::ActorSkeleton {
                    holster,
                    ..(*rig.skeleton).clone()
                });
            }
            if dressed.worn.as_ref().map(|w| w.1) != Some(done.worn.1) && rig.ragdoll.is_none() {
                rig.weapon_attached = true;
            }
            println!(
                "{} redrawn: {gone} pieces gone, {} parts ({came} pieces) came; \
                 built in {:.1} ms on its own thread, put on in {:.1} ms",
                walker.reference,
                done.new.len(),
                done.took,
                started.elapsed().as_secs_f64() * 1000.0
            );
            dressed.worn = Some(done.worn);
            dressed.look = Arc::new(done.wanted);
            continue;
        }
        // The weapon as `actors` last saw it (not yet: wait); someone held
        // still (`ai`: frozen, or not the speaker in a conversation) isn't
        // looked at there, so here; the dead keep theirs.
        let weapon = if rig.still && rig.ragdoll.is_none() {
            world::combat::weapon_in_hand(order, state, walker.reference).map(|w| w.form_id)
        } else {
            let Some(weapon) = rig.held_weapon else {
                continue;
            };
            weapon
        };
        let base = dressed.look.base;
        let start = start
            .0
            .entry(base)
            .or_insert_with(|| Arc::new(world::outfit::start_worn(order, base)))
            .clone();
        let armour = world::outfit::worn_armour(order, state, walker.reference, &start);
        let record_weapon = dressed.look.fighting.as_ref().and_then(|f| f.weapon);
        let drawn = dressed
            .worn
            .get_or_insert_with(|| (start.pieces.iter().map(|p| p.item).collect(), record_weapon));
        if drawn.0 == armour && drawn.1 == weapon {
            continue;
        }
        let (game, look) = (game.0.clone(), dressed.look.clone());
        let worn = (armour, weapon);
        let asked = worn.clone();
        let task = std::thread::spawn(move || build(game, base, look, asked));
        commands.entity(root).insert(Redrawing {
            task: Some(task),
            worn,
        });
    }
}

/// A weapon kind's holster pose (`world::Fighting::holster`), read from
/// the game's files.
fn holster(
    game: &cellview::Game,
    fighting: Option<&world::Fighting>,
) -> Option<Arc<nif::Sequence>> {
    let path = fighting?.holster.as_ref()?;
    let bytes = game.assets.read(&assets::mesh_path(path)).ok()??;
    let sequence = nif::Nif::parse(bytes)
        .ok()?
        .sequences()
        .ok()?
        .into_iter()
        .next()?;
    Some(Arc::new(sequence))
}

impl Spawner<'_, '_> {
    /// The pieces of a lone actor's scene (`scene`, built from some parts
    /// of a person's look) put on that person: skinned to their joints,
    /// under their root, lit by `lighting`, each tagged with the part of
    /// the person's look it is (`parts`: the scene's part `i` is the
    /// person's `parts[i]`). As the place's own people's pieces are
    /// spawned (`spawn_parts`).
    pub fn spawn_actor_pieces(
        &mut self,
        scene: &cellview::ViewerScene,
        lighting: GameLighting,
        root: Entity,
        joints: &[Entity],
        parts: &[u16],
    ) -> Vec<Entity> {
        let compressed = self.device.as_ref().is_none_or(|d| {
            d.features()
                .contains(bevy::render::render_resource::WgpuFeatures::TEXTURE_COMPRESSION_BC)
        });
        let anisotropy = self.settings.anisotropy;
        let textures: Vec<Option<Handle<Image>>> = scene
            .textures
            .iter()
            .map(|t| crate::upload_texture(&mut self.images, t, compressed, anisotropy))
            .collect();
        let mut out = Vec::new();
        for draw in scene.draws.iter().filter(|d| d.actor == Some(0)) {
            let data = &scene.meshes[draw.mesh];
            let Some((mesh, bind)) = crate::actors::skinned_mesh(data) else {
                continue;
            };
            let part = data
                .actor_part
                .and_then(|i| parts.get(usize::from(i)).copied());
            let mesh = self.meshes.add(mesh);
            let bind = self.inverse_bindposes.add(bind);
            let material = self
                .lit_materials
                .add(crate::lit_material(data, &textures, lighting));
            let mut piece = self.commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::IDENTITY,
                crate::actors::skinned(bind, crate::actors::skin_joints(data, joints)),
                bevy::render::view::NoFrustumCulling,
                ChildOf(root),
            ));
            if let Some(part) = part {
                piece.insert(Piece(part));
            }
            if let Some(face) = crate::faces::FacePiece::of(data) {
                piece.insert(face);
            }
            if let Some(link) = data.material.emittance {
                piece.insert(crate::emittance::Glow(link));
            }
            out.push(piece.id());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(model: &str) -> world::ActorPart {
        world::ActorPart {
            model: model.into(),
            skin_texture: None,
            texture: None,
            hide_mesh: None,
            hair_tint: None,
            facegen: false,
            face_tint: None,
            bone: None,
            parent_bone: None,
        }
    }

    #[test]
    fn only_the_parts_that_changed_are_rebuilt() {
        // Upper body, hands, head, hair drawn; a dress put on over the
        // upper body and the hands, and a hat that changes the hair's
        // piece shown (`NoHat` → `Hat`).
        let mut hair = part("Hair.nif");
        hair.hide_mesh = Some("Hat".into());
        let drawn = vec![
            part("UpperBody.nif"),
            part("Hands.nif"),
            part("Head.nif"),
            hair.clone(),
        ];
        let mut hat_hair = hair;
        hat_hair.hide_mesh = Some("NoHat".into());
        let wanted = vec![
            part("Dress.nif"),
            part("Hat.nif"),
            part("Head.nif"),
            hat_hair,
        ];
        let (stays, new) = match_parts(&drawn, &wanted);
        assert_eq!(stays, [None, None, Some(2), None]);
        assert_eq!(new, [0, 1, 3]);
        // Nothing changed: everything stays where it is.
        let (stays, new) = match_parts(&drawn, &drawn);
        assert_eq!(stays, [Some(0), Some(1), Some(2), Some(3)]);
        assert!(new.is_empty());
        // The same model twice pairs up in order.
        let twice = vec![part("Ring.nif"), part("Ring.nif")];
        let (stays, new) = match_parts(&twice[..1], &twice);
        assert_eq!(stays, [Some(0)]);
        assert_eq!(new, [1]);
    }
}
