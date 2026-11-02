//! People and creatures, posed: each part of an actor (see
//! `world::actor`) skinned to its skeleton in the first frame of its idle
//! animation, and gathered into one model placed where the actor stands.
//! Each piece also keeps its bind pose and bones ([`Rig`]), so a renderer
//! can animate it on the graphics card.
//!
//! Skinning follows the files: a vertex goes to the weighted sum of each
//! of its bones' posed transform times that bone's skin-to-bone transform
//! (`nif::Skin::deform`), which with the skeleton in its own pose gives
//! back the body exactly as the model file has it (checked on
//! `upperbody.nif` against `skeleton.nif`: within 0.1 units). Gore caps
//! (the dismemberment partitions in the cap ranges) are left out, as on an
//! intact body. Rigid pieces hang from the bone their model names (see
//! [`attachment`], `rigid_transform`); pieces without a shader property
//! aren't drawn (`drawn`). The race's skin textures replace the default
//! skin on pieces whose shader has the face-and-skin flag (0x400), with the
//! NPC's face or body tint laid on; hair and eyes take their records'
//! textures, hair its layer map over it. FaceGen parts take the face's
//! shape (race + NPC values, their own `.egm`, or each piece's for hair)
//! and carry the morphs of their `.tri` ([`Rig::face`], `world::face`), for
//! talking and blinking.

use std::collections::HashMap;
use std::sync::Arc;

use nif::math::Transform;
use world::{ActorLook, ActorPart, Face};

use crate::cell::{ActorSkeleton, Loader, Model, ModelMesh, Rig};

/// The shader flag on skin pieces (bare arms, hands, the head): the race's
/// skin texture goes on these.
pub const SKIN_FLAG: u32 = 0x400;

/// A head's texture with the NPC's face tint laid on, as the texture
/// reference its mesh carries: `facetint:<tint path>|<base path>`. The
/// real-time viewer builds it (`cellview::TextureData::plus_face_tint`);
/// the CPU renderer uses the base.
pub const FACE_TINT: &str = "facetint:";

/// A face-tinted texture reference's tint and base paths.
pub fn face_tint_parts(reference: &str) -> Option<(&str, &str)> {
    reference.strip_prefix(FACE_TINT)?.split_once('|')
}

/// A hair texture with its layer map laid over it, as the texture
/// reference its mesh carries: `hairlayer:<layer path>|<base path>`. The
/// game's hair shader blends the base toward the layer map (the model's
/// texture slot 2, `HairBun_hl.dds` for `HairBun.nif`, bound as
/// `LayerMap` in the recording) by the layer's alpha; the real-time viewer
/// builds it (`cellview::TextureData::with_layer`), the CPU renderer uses
/// the base.
pub const HAIR_LAYER: &str = "hairlayer:";

/// A layered hair texture reference's layer and base paths.
pub fn hair_layer_parts(reference: &str) -> Option<(&str, &str)> {
    reference.strip_prefix(HAIR_LAYER)?.split_once('|')
}

/// Either kind of combined texture reference: (what's laid on, the base).
pub fn composite_parts(reference: &str) -> Option<(&str, &str)> {
    face_tint_parts(reference).or_else(|| hair_layer_parts(reference))
}

/// Bone names (lower case) to transforms in the skeleton's space.
pub(crate) type Bones = HashMap<String, Transform>;

/// A skeleton posed by its idle, and in its own (bind) pose.
pub(crate) struct Posed {
    pub posed: Bones,
    pub bind: Bones,
    /// Bone names (lower case) to their index in `skeleton.bones`.
    pub index: HashMap<String, usize>,
    pub skeleton: Arc<ActorSkeleton>,
}

impl Loader<'_> {
    /// The model of an actor, built once per base record.
    pub(crate) fn actor_model(&mut self, look: &ActorLook) -> Option<usize> {
        // By base record, and by what it's built from (the first-person
        // view is rebuilt for each weapon).
        let parts: Vec<&str> = look.parts.iter().map(|p| p.model.as_str()).collect();
        let key = format!(
            "actor:{}|{}|{}|{}",
            look.base,
            look.skeleton,
            look.idle,
            parts.join("|")
        );
        if let Some(&known) = self.model_index.get(&key) {
            return known;
        }
        let pose = self.pose(
            &look.skeleton,
            &look.idle,
            &look.walk,
            look.fighting.as_ref(),
        );
        let index = pose.and_then(|pose| {
            let mut meshes = Vec::new();
            for (i, part) in look.parts.iter().enumerate() {
                let face = look.face.as_ref().filter(|_| part.facegen);
                let posed = self.posed_part(part, face, &pose, &look.skeleton, &look.idle);
                meshes.extend(posed.into_iter().map(|mut m| {
                    m.actor_part = u16::try_from(i).ok();
                    m
                }));
            }
            if meshes.is_empty() {
                return None;
            }
            self.models.push(Model {
                path: key.clone(),
                meshes,
                collision: Vec::new(),
                root_transform: None,
                skeleton: Some(pose.skeleton.clone()),
                sequences: Default::default(),
                particles: None,
                bsx_flags: 0,
            });
            Some(self.models.len() - 1)
        });
        self.model_index.insert(key, index);
        index
    }

    /// A skeleton in the first frame of an idle (with a weapon, put away),
    /// cached, with its walk and its fighting animations.
    fn pose(
        &mut self,
        skeleton: &str,
        idle: &str,
        walk: &str,
        fighting: Option<&world::Fighting>,
    ) -> Option<std::rc::Rc<Posed>> {
        let key = format!(
            "{skeleton}|{idle}|{walk}|{:?}",
            fighting.map(|w| (&w.holster, &w.aim, &w.run, &w.attack))
        )
        .to_ascii_lowercase();
        if let Some(known) = self.poses.get(&key) {
            return known.clone();
        }
        let posed = self.read_nif(skeleton).and_then(|nif| {
            let bones = nif.skeleton().ok()?;
            let sequence = self
                .read_nif(idle)
                .and_then(|kf| kf.sequences().ok())
                .and_then(|s| s.into_iter().next());
            // Walking and fighting animations aren't needed to draw, so a
            // missing one isn't reported.
            let optional = |path: &str| {
                self.assets
                    .read(&assets::mesh_path(path))
                    .ok()
                    .flatten()
                    .and_then(|bytes| nif::Nif::parse(bytes).ok())
                    .and_then(|kf| kf.sequences().ok())
                    .and_then(|s| s.into_iter().next())
            };
            let walk_path = walk;
            let turn = |file: &str| optional(&turn_path(walk, file));
            let (turn_left, turn_right) = (turn("mtturnleft.kf"), turn("mtturnright.kf"));
            let walk = optional(walk);
            let held = |pick: fn(&world::Fighting) -> Option<&String>| {
                fighting.and_then(pick).and_then(|p| optional(p))
            };
            let (holster, aim, run) = (
                held(|w| w.holster.as_ref()),
                held(|w| Some(&w.aim)),
                held(|w| Some(&w.run)),
            );
            // Melee weapons and fists ship their attacks as `_a` and `_b`
            // variants (`1hmattackright_a.kf`): the first stands in.
            let attack = fighting.and_then(|w| {
                optional(&w.attack).or_else(|| optional(&w.attack.replace(".kf", "_a.kf")))
            });
            if sequence.is_none() {
                self.report
                    .missing_animations
                    .insert(assets::mesh_path(idle));
            }
            let time = sequence.as_ref().map_or(0.0, |s| s.start);
            let by_name = |world: Vec<Transform>| -> Bones {
                bones
                    .iter()
                    .zip(world)
                    .map(|(b, t)| (b.name.to_ascii_lowercase(), t))
                    .collect()
            };
            // A weapon starts put away.
            let mut first = nif::posed(&bones, sequence.as_ref(), time);
            if let Some(h) = &holster {
                nif::hang_weapon(&bones, &mut first, h);
            }
            Some(std::rc::Rc::new(Posed {
                posed: by_name(first),
                bind: by_name(nif::posed(&bones, None, 0.0)),
                index: bones
                    .iter()
                    .enumerate()
                    .map(|(i, b)| (b.name.to_ascii_lowercase(), i))
                    .collect(),
                skeleton: Arc::new(ActorSkeleton {
                    bones: bones.clone(),
                    idle: sequence.map(Arc::new),
                    walk: walk.map(Arc::new),
                    holster: holster.map(Arc::new),
                    aim: aim.map(Arc::new),
                    run: run.map(Arc::new),
                    attack: attack.map(Arc::new),
                    turn_left: turn_left.map(Arc::new),
                    turn_right: turn_right.map(Arc::new),
                    ragdoll: nif
                        .ragdoll()
                        .ok()
                        .flatten()
                        .map(|r| crate::ragdoll::RagdollRig::new(&bones, r)),
                    bound: nif.bound(),
                    skeleton_path: skeleton.to_string(),
                    idle_path: idle.to_string(),
                    walk_path: walk_path.to_string(),
                }),
            }))
        });
        self.poses.insert(key, posed.clone());
        posed
    }

    /// A model's FaceGen shape morphs (its `.egm`), if it has them; with
    /// `mesh`, the ones of that piece (see [`facegen_path`]).
    fn read_egm(&mut self, model: &str, mesh: Option<&str>) -> Option<nif::Egm> {
        let bytes = self
            .assets
            .read(&facegen_path(model, mesh, "egm")?)
            .ok()??;
        nif::Egm::parse(&bytes).ok()
    }

    /// A model's FaceGen morph targets (its `.tri`), if it has them; with
    /// `mesh`, the ones of that piece (see [`facegen_path`]).
    fn read_tri(&mut self, model: &str, mesh: Option<&str>) -> Option<nif::Tri> {
        let bytes = self
            .assets
            .read(&facegen_path(model, mesh, "tri")?)
            .ok()??;
        nif::Tri::parse(&bytes).ok()
    }

    fn read_nif(&mut self, reference: &str) -> Option<nif::Nif> {
        let path = assets::mesh_path(reference);
        let bytes = match self.assets.read(&path) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => {
                *self.report.missing_models.entry(path).or_default() += 1;
                return None;
            }
            Err(e) => {
                self.report.unreadable_models.insert(path, e.to_string());
                return None;
            }
        };
        match nif::Nif::parse(bytes) {
            Ok(nif) => Some(nif),
            Err(e) => {
                self.report.unreadable_models.insert(path, e.to_string());
                None
            }
        }
    }

    /// One part's pieces in the pose, cached (parts are shared by many
    /// actors).
    fn posed_part(
        &mut self,
        part: &ActorPart,
        face: Option<&Face>,
        pose: &Posed,
        skeleton: &str,
        idle: &str,
    ) -> Vec<ModelMesh> {
        let key = format!(
            "{skeleton}|{idle}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
            part.model,
            part.skin_texture,
            part.texture,
            part.hide_mesh,
            part.hair_tint,
            part.face_tint,
            part.bone,
            face
        )
        .to_ascii_lowercase();
        if let Some(known) = self.posed_parts.get(&key) {
            return known.clone();
        }
        let mut out = Vec::new();
        if let Some(nif) = self.read_nif(&part.model) {
            // The part file's own bones, for any its skin names that the
            // skeleton lacks: they stay in the part's own pose.
            let own: Bones = nif
                .skeleton()
                .map(|bones| {
                    let world = nif::posed(&bones, None, 0.0);
                    bones
                        .iter()
                        .zip(world)
                        .map(|(b, t)| (b.name.to_ascii_lowercase(), t))
                        .collect()
                })
                .unwrap_or_default();
            // Rigid pieces hang from the bone the part names, in that
            // bone's own axes (a weapon at `Weapon`), else from the model's
            // own attachment bone, if any: FaceGen head parts in the head's
            // upright axes (see `attachment`), anything else with its top
            // node under the bone (see `rigid_transform`).
            let attach = match &part.bone {
                Some(bone) => {
                    let name = bone.to_ascii_lowercase();
                    pose.posed
                        .get(&name)
                        .copied()
                        .zip(pose.index.get(&name).copied())
                        .map(|(posed, index)| (posed, index, Transform::IDENTITY))
                }
                // Clothes and armour: the body slot's bone, which the game
                // hangs unskinned pieces from after any `Prn` one
                // (`world::actor::slot_parent_bone`, `004ac1e0`); else the
                // model's `Prn` bone.
                None => part
                    .parent_bone
                    .clone()
                    .or_else(|| nif.attach_bone())
                    .and_then(|bone| {
                        let name = bone.to_ascii_lowercase();
                        let index = *pose.index.get(&name)?;
                        if part.facegen {
                            Some((attachment(pose, &name)?, index, unturn(pose, &name)?))
                        } else {
                            Some((*pose.posed.get(&name)?, index, Transform::IDENTITY))
                        }
                    }),
            };
            // The face's shape, from the model's FaceGen morphs.
            let part_egm = face.and_then(|_| self.read_egm(&part.model, None));
            // What the part talks and blinks with (`world::face`).
            let part_tri = part
                .facegen
                .then(|| self.read_tri(&part.model, None))
                .flatten();
            // Parts hung from their own bone keep (or, for head parts,
            // may keep) the top node's transform (see `upright`,
            // `rigid_transform`).
            let with_top = (part.bone.is_none() && attach.is_some())
                .then(|| nif.scene().ok())
                .flatten();
            // As for placed models, the top node's own transform is left
            // out (the eyes' turns them away from the face otherwise).
            if let Ok(scene) = nif.placed_scene() {
                for (i, mut mesh) in scene.meshes.into_iter().enumerate() {
                    if let Some(full) = with_top.as_ref().and_then(|s| s.meshes.get(i)) {
                        mesh.transform =
                            rigid_transform(part.facegen, &full.transform, &mesh.transform);
                    }
                    if part.hide_mesh.as_deref() == Some(mesh.name.as_str()) || !drawn(&mesh) {
                        continue;
                    }
                    // A model without its own FaceGen files may have them
                    // per piece (hair: `hairbunnohat.egm`).
                    let mesh_egm = match (&part_egm, face) {
                        (None, Some(_)) => self.read_egm(&part.model, Some(&mesh.name)),
                        _ => None,
                    };
                    let egm = part_egm.as_ref().or(mesh_egm.as_ref());
                    let mesh_tri = match (&part_tri, part.facegen) {
                        (None, true) => self.read_tri(&part.model, Some(&mesh.name)),
                        _ => None,
                    };
                    let tri = part_tri.as_ref().or(mesh_tri.as_ref());
                    if let (Some(egm), Some(face)) = (egm, face) {
                        egm.apply(&mut mesh.positions, &face.symmetric, &face.asymmetric);
                    }
                    let morphs = face_morphs(tri, egm, face, &mesh.positions);
                    let mut rig = None;
                    if let Some(skin) = &mesh.skin {
                        rig = Some(skin_rig(&mesh, skin, pose, &own));
                    } else if let Some((bone, index, unturn)) = &attach {
                        rig = Some(rigid_rig(&mesh, *index, unturn.then_child(&mesh.transform)));
                        mesh.transform = bone.then_child(&mesh.transform);
                    }
                    if let Some(rig) = &mut rig {
                        rig.face = morphs;
                    }
                    if mesh.skin.is_some() && !pose_mesh(&mut mesh, &pose.posed, &own) {
                        continue;
                    }
                    retexture(&mut mesh, part);
                    if let Some(tint) = part.hair_tint {
                        tint_hair(&mut mesh, tint);
                    }
                    if let Some(mut m) = self.mesh(&mesh) {
                        m.rig = rig;
                        if m.shading == crate::cell::Shading::Hair {
                            m.hair_tint = part.hair_tint.map(|c| c.map(|v| f32::from(v) / 255.0));
                        }
                        out.push(m);
                    }
                }
            }
        }
        self.posed_parts.insert(key, out.clone());
        out
    }
}

/// A file in a walk animation's `locomotion` folder (the walk itself may
/// sit in its `male` or `female` folder): `characters\_male\locomotion\
/// male\mtforward.kf` gives `characters\_male\locomotion\mtturnleft.kf`.
/// The game loads every `*.KF` of a skeleton's folder and, for
/// characters, of its `Locomotion\` folder (`00447330`, `ModelLoader.cpp`;
/// `Locomotion\Hurt\*.KF` as idles), so the turn groups' plain files
/// (`mtturnleft.kf`, `mtturnright.kf`, 1 s loops) are among an actor's
/// animations and the group lookup (`00495740`) finds them.
pub fn turn_path(walk: &str, file: &str) -> String {
    let lower = walk.to_ascii_lowercase();
    match lower.rfind("locomotion\\") {
        Some(i) => format!("{}{file}", &walk[..i + "locomotion\\".len()]),
        None => {
            let folder = walk.rfind(['\\', '/']).map_or("", |i| &walk[..=i]);
            format!("{folder}{file}")
        }
    }
}

/// Where a FaceGen file of a model is: the model's path with the extension
/// (`egm` shape morphs, `tri` morph targets) for its own, or, for one piece,
/// the piece's name added to the stem: hair with a hat version ships
/// `hairbunnohat.egm` and `hairbunhat.egm` for `HairBun.NIF`'s `NoHat` and
/// `Hat` pieces (the game's table at `0119b734` names those two suffixes,
/// `findings\lips.md`). Without it, the hair kept its shape while the
/// reshaped head grew through it.
pub fn facegen_path(model: &str, mesh: Option<&str>, extension: &str) -> Option<String> {
    let path = assets::mesh_path(model);
    let (stem, _) = path.rsplit_once('.')?;
    Some(match mesh {
        Some(name) => format!("{stem}{}.{extension}", name.to_ascii_lowercase()),
        None => format!("{stem}.{extension}"),
    })
}

/// A head part's mesh's morphs (its part's `.tri`), for its vertices
/// `positions` already shaped by the NPC's face; the face reshapes the
/// statistical targets too (`world::face::FaceMorphs::build`). `None` when
/// nothing in it moves.
fn face_morphs(
    tri: Option<&nif::Tri>,
    egm: Option<&nif::Egm>,
    face: Option<&Face>,
    positions: &[nif::math::Vec3],
) -> Option<Arc<world::face::FaceMorphs>> {
    let reshape = egm.zip(face).map(|(egm, face)| world::face::Reshape {
        egm,
        symmetric: &face.symmetric,
        asymmetric: &face.asymmetric,
    });
    let morphs = world::face::FaceMorphs::build(tri?, positions, reshape);
    (!morphs.is_empty()).then(|| Arc::new(morphs))
}

/// A skinned piece's rig: its raw vertices, and its skin's bones found in
/// the skeleton (bones the skeleton lacks stay put, in the part file's own
/// pose, hung from the skeleton's root).
fn skin_rig(mesh: &nif::Mesh, skin: &nif::Skin, pose: &Posed, own: &Bones) -> Rig {
    let joints = skin
        .bones
        .iter()
        .zip(&skin.bone_transforms)
        .map(|(name, skin_to_bone)| {
            let name = name.to_ascii_lowercase();
            match pose.index.get(&name) {
                Some(&i) => (i, *skin_to_bone),
                None => (
                    0,
                    own.get(&name)
                        .copied()
                        .unwrap_or(Transform::IDENTITY)
                        .then_child(skin_to_bone),
                ),
            }
        })
        .collect();
    let weights = (0..mesh.positions.len())
        .map(|v| strongest_four(skin.weights.get(v).map_or(&[][..], |w| w)))
        .collect();
    Rig {
        positions: mesh.positions.clone(),
        normals: mesh.normals.clone(),
        along_u: mesh.bitangents.clone(),
        along_v: mesh.tangents.clone(),
        joints,
        weights,
        face: None,
    }
}

/// A rigid piece's rig: one joint, the bone it hangs from.
fn rigid_rig(mesh: &nif::Mesh, bone: usize, to_bone: Transform) -> Rig {
    Rig {
        positions: mesh.positions.clone(),
        normals: mesh.normals.clone(),
        along_u: mesh.bitangents.clone(),
        along_v: mesh.tangents.clone(),
        joints: vec![(bone, to_bone)],
        weights: vec![[(0, 1.0), (0, 0.0), (0, 0.0), (0, 0.0)]; mesh.positions.len()],
        face: None,
    }
}

/// The four bones with the most weight on a vertex, their weights scaled
/// to add up to 1 (graphics cards blend four). A vertex with no weights
/// follows the first joint.
fn strongest_four(list: &[(u16, f32)]) -> [(u16, f32); 4] {
    let mut sorted: Vec<(u16, f32)> = list.iter().copied().filter(|w| w.1 > 0.0).collect();
    sorted.sort_by(|a, b| b.1.total_cmp(&a.1));
    sorted.truncate(4);
    let total: f32 = sorted.iter().map(|w| w.1).sum();
    let mut out = [(0u16, 0.0f32); 4];
    if total <= 0.0 {
        out[0] = (0, 1.0);
        return out;
    }
    for (slot, (joint, weight)) in out.iter_mut().zip(sorted) {
        *slot = (joint, weight / total);
    }
    out
}

/// Puts a skinned mesh into the pose (positions and directions in the
/// skeleton's space) and drops its gore caps. `false` when nothing of it
/// is left to draw.
fn pose_mesh(mesh: &mut nif::Mesh, pose: &Bones, own: &Bones) -> bool {
    let Some(skin) = mesh.skin.take() else {
        return true;
    };
    if !skin.partitions.is_empty() {
        let kept: Vec<[u16; 3]> = skin
            .partitions
            .iter()
            .filter(|p| !p.is_cap())
            .flat_map(|p| p.triangles.iter().copied())
            .collect();
        if kept.is_empty() {
            return false;
        }
        mesh.triangles = kept;
    }
    let bones: Vec<Transform> = skin
        .bones
        .iter()
        .map(|name| {
            let name = name.to_ascii_lowercase();
            pose.get(&name)
                .or_else(|| own.get(&name))
                .copied()
                .unwrap_or(Transform::IDENTITY)
        })
        .collect();
    let deform_all = |list: &[nif::math::Vec3]| -> Vec<nif::math::Vec3> {
        list.iter()
            .enumerate()
            .map(|(i, &d)| skin.deform_direction(i, d, &bones))
            .collect()
    };
    mesh.positions = mesh
        .positions
        .iter()
        .enumerate()
        .map(|(i, &p)| skin.deform(i, p, &bones))
        .collect();
    mesh.normals = deform_all(&mesh.normals);
    mesh.tangents = deform_all(&mesh.tangents);
    mesh.bitangents = deform_all(&mesh.bitangents);
    // Already in the skeleton's space.
    mesh.transform = Transform::IDENTITY;
    true
}

/// A head part's mesh transform, with or without its model's top node:
/// whichever is upright (closer to no turn). Head parts are modelled
/// upright around their bone (see [`attachment`]), and the game's files
/// get there two ways: 36 hair models (`hairbun.nif`, `hairfemalea.nif`,
/// the children's) turn their mesh node and turn it back on the top node,
/// so they're upright only with it; 16 head models (eyes, teeth, mouth,
/// tongue) turn only their top node, and are upright only without it; the
/// other 73 have neither turned. Read from every hair and head model; the
/// game's own rule isn't traced.
fn upright(with_top: &Transform, without_top: &Transform) -> Transform {
    let off = |t: &Transform| -> f32 {
        (0..3)
            .flat_map(|i| (0..3).map(move |j| (i, j)))
            .map(|(i, j)| (t.rotation[i][j] - if i == j { 1.0 } else { 0.0 }).abs())
            .sum()
    };
    if off(with_top) < off(without_top) {
        *with_top
    } else {
        *without_top
    }
}

/// Whether the game draws a piece of a person's model at all: only pieces
/// with one of its shader properties. Weapons carry a bare `Weapon:0`
/// strip with nothing but a material (the machete's: a thin bar along the
/// blade's spine); in the recording the first-person machete is drawn as
/// its one shaded piece and the bar never is (it showed here as a bright
/// untextured streak down the blade).
fn drawn(mesh: &nif::Mesh) -> bool {
    mesh.shader.is_some()
        || mesh
            .property_types
            .iter()
            .any(|t| t.contains("Shader") || t == "NiTexturingProperty")
}

/// A rigid mesh's transform relative to what it hangs from (its model's
/// `Prn` bone; for FaceGen head parts that bone in upright axes, see
/// [`attachment`]), given its transform with and without the model's top
/// node. FaceGen head parts (eyes, hair, beards, teeth) take whichever is
/// upright ([`upright`]); anything else hangs with its top node as a child
/// of the bone, the top node's own transform included. Read from the
/// actors recording: the dog Cheyenne's eyes (`eyessetblue.nif`, `Prn`
/// `Bip01 Head`, its top node turned) were drawn exactly at her posed head
/// bone × the top node's transform (to 0.0001); with the head parts' rule
/// they stood one above the other at her collar. Sunny Smiles' eyes, mouth,
/// teeth and hair were all drawn with her head mesh's skinning transform,
/// which is what the upright rule gives them.
fn rigid_transform(facegen: bool, with_top: &Transform, without_top: &Transform) -> Transform {
    if facegen {
        upright(with_top, without_top)
    } else {
        *with_top
    }
}

/// The inverse of a bone's bind rotation (see [`attachment`]).
fn unturn(pose: &Posed, name: &str) -> Option<Transform> {
    let r = pose.bind.get(name)?.rotation;
    Some(Transform {
        rotation: [
            [r[0][0], r[1][0], r[2][0]],
            [r[0][1], r[1][1], r[2][1]],
            [r[0][2], r[1][2], r[2][2]],
        ],
        translation: [0.0; 3],
        scale: 1.0,
    })
}

/// Where a rigid piece hanging from `bone` goes: the head parts (hair,
/// eyes, beards) are modelled around the bone's position in the
/// skeleton's upright axes, not the bone's own (the head bone's axes are
/// turned a quarter: with them the hair lands beside the head). So the
/// piece takes the bone's pose times the inverse of its bind rotation.
/// Checked against the skinned head: the hair then spans z 112–128 over
/// the head's 105–128, the eyes and mustache sit at the face's front.
fn attachment(pose: &Posed, bone: &str) -> Option<Transform> {
    let name = bone.to_ascii_lowercase();
    let posed = pose.posed.get(&name)?;
    Some(posed.then_child(&unturn(pose, &name)?))
}

/// The shader flag on hair pieces (hair, beards, eyebrows).
pub const HAIR_FLAG: u32 = 0x40000;

/// Hair colour, as the game's hair shader gives it (`SM3002.pso` and its
/// kin): the texture times `lerp(1, 2 × HairTint, mask)` on stored values,
/// where the mask is the vertex colour's green (the hair meshes' vertex
/// colours are masks, not colours) and `HairTint` the NPC's hair colour.
/// Baked into the vertex colours, which the renderers multiply the texture
/// by. Confirmed by the actors recording: Sunny Smiles' hair and eyebrows
/// get `HairTint` (0.2588, 0.1098, 0.0588), her `HCLR` (66, 28, 15) / 255,
/// and their vertex shader (`SM3003.vso`) hands the vertex colour on.
fn tint_hair(mesh: &mut nif::Mesh, tint: [u8; 3]) {
    let hair = mesh
        .shader
        .as_ref()
        .is_some_and(|s| s.shader_flags & HAIR_FLAG != 0);
    if !hair {
        return;
    }
    let tint = tint.map(|c| 2.0 * f32::from(c) / 255.0);
    let factor = |mask: f32| tint.map(|t| 1.0 + mask * (t - 1.0));
    if mesh.colors.len() == mesh.positions.len() {
        for c in &mut mesh.colors {
            let [r, g, b] = factor(c[1]);
            *c = [r, g, b, c[3]];
        }
    } else {
        let [r, g, b] = factor(1.0);
        mesh.colors = vec![[r, g, b, 1.0]; mesh.positions.len()];
    }
}

/// The race's skin on skin pieces; hair and eye textures on everything;
/// on skin, the NPC's face or body tint over it (see [`FACE_TINT`]); on
/// hair, its layer map over it (see [`HAIR_LAYER`]).
fn retexture(mesh: &mut nif::Mesh, part: &ActorPart) {
    let flags = mesh
        .shader
        .as_ref()
        .filter(|s| s.lit)
        .map_or(0, |s| s.shader_flags);
    let skin = flags & SKIN_FLAG != 0;
    let hair = flags & HAIR_FLAG != 0;
    let replacement = part
        .texture
        .as_ref()
        .or(part.skin_texture.as_ref().filter(|_| skin));
    if let Some(texture) = replacement {
        let path = assets::texture_path(texture);
        match mesh.textures.first_mut() {
            Some(first) => *first = path,
            None => mesh.textures.push(path),
        }
    }
    let layer = mesh
        .textures
        .get(2)
        .filter(|t| hair && !t.is_empty())
        .map(|t| assets::texture_path(t));
    if let (Some(layer), Some(first)) = (layer, mesh.textures.first_mut()) {
        *first = format!("{HAIR_LAYER}{layer}|{first}");
    }
    if let (true, Some(tint), Some(first)) = (skin, &part.face_tint, mesh.textures.first_mut()) {
        // A tint the game makes (`world::MadeBodyTint`) goes as it is.
        let tint = if tint.starts_with(world::actor::MADE_BODY_TINT) {
            tint.clone()
        } else {
            assets::texture_path(tint)
        };
        *first = format!("{FACE_TINT}{tint}|{first}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shader(flags: u32) -> nif::ShaderProperty {
        nif::ShaderProperty {
            net: nif::ObjectNet {
                name: String::new(),
                extra_data: Vec::new(),
                controller: -1,
            },
            type_name: "BSShaderPPLightingProperty".into(),
            lit: true,
            shade_flags: 0,
            shader_type: 0,
            shader_flags: flags,
            shader_flags2: 0,
            env_map_scale: 1.0,
            texture_clamp: 3,
            texture_set: -1,
            file_name: None,
            layout_ok: true,
            falloff: None,
        }
    }

    #[test]
    fn turns_come_from_the_walks_locomotion_folder() {
        assert_eq!(
            turn_path(
                "Characters\\_Male\\locomotion\\male\\mtforward.kf",
                "mtturnleft.kf"
            ),
            "Characters\\_Male\\locomotion\\mtturnleft.kf"
        );
        assert_eq!(
            turn_path(
                "Creatures\\Molerat\\locomotion\\mtforward.kf",
                "mtturnright.kf"
            ),
            "Creatures\\Molerat\\locomotion\\mtturnright.kf"
        );
        assert_eq!(
            turn_path("odd\\walk.kf", "mtturnleft.kf"),
            "odd\\mtturnleft.kf"
        );
    }

    #[test]
    fn hair_takes_its_colour_where_the_mask_says() {
        let mut mesh = nif::Mesh {
            name: "Hair".into(),
            block: 0,
            transform: Transform::IDENTITY,
            positions: vec![[0.0; 3]; 2],
            normals: Vec::new(),
            tangents: Vec::new(),
            bitangents: Vec::new(),
            uvs: Vec::new(),
            colors: vec![[0.3, 0.0, 0.7, 0.5], [0.3, 1.0, 0.7, 1.0]],
            triangles: Vec::new(),
            bound: ([0.0; 3], 0.0),
            nodes: Vec::new(),
            billboard: None,
            textures: Vec::new(),
            shader: Some(shader(HAIR_FLAG)),
            material: None,
            alpha: None,
            zbuffer: None,
            double_sided: false,
            skinned: false,
            skin: None,
            property_types: Vec::new(),
        };
        // Grey hair: 2 × 0.5 = 1, so it changes nothing; mid red brightens
        // red where the mask (green) is full.
        tint_hair(&mut mesh, [255, 128, 0]);
        assert_eq!(
            mesh.colors[0],
            [1.0, 1.0, 1.0, 0.5],
            "no mask: the texture as it is"
        );
        let c = mesh.colors[1];
        assert!(
            (c[0] - 2.0).abs() < 1e-6 && (c[1] - 1.0039).abs() < 1e-3 && c[2] == 0.0,
            "{c:?}"
        );
    }

    #[test]
    fn the_heads_skin_carries_the_npcs_face_tint() {
        let mesh = |flags: u32| nif::Mesh {
            name: "Head".into(),
            block: 0,
            transform: Transform::IDENTITY,
            positions: Vec::new(),
            normals: Vec::new(),
            tangents: Vec::new(),
            bitangents: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            triangles: Vec::new(),
            bound: ([0.0; 3], 0.0),
            nodes: Vec::new(),
            billboard: None,
            textures: vec!["textures\\characters\\male\\headhuman.dds".into()],
            shader: Some(shader(flags)),
            material: None,
            alpha: None,
            zbuffer: None,
            double_sided: false,
            skinned: false,
            skin: None,
            property_types: Vec::new(),
        };
        let part = ActorPart {
            model: "characters\\head\\headhuman.nif".into(),
            skin_texture: Some("characters\\male\\headhuman.dds".into()),
            texture: None,
            hide_mesh: None,
            hair_tint: None,
            facegen: true,
            face_tint: Some("textures\\characters\\facemods\\falloutnv.esm\\00104c0c_0.dds".into()),
            bone: None,
            parent_bone: None,
        };
        let mut skin = mesh(SKIN_FLAG);
        retexture(&mut skin, &part);
        let (tint, base) = face_tint_parts(&skin.textures[0]).unwrap();
        assert_eq!(
            tint,
            "textures\\characters\\facemods\\falloutnv.esm\\00104c0c_0.dds"
        );
        assert_eq!(base, "textures\\characters\\male\\headhuman.dds");
        // Not on pieces that aren't skin.
        let mut other = mesh(0);
        retexture(&mut other, &part);
        assert!(face_tint_parts(&other.textures[0]).is_none());
    }

    #[test]
    fn bare_pieces_without_a_shader_are_not_drawn() {
        // The machete's `Weapon:0`: a material and nothing else.
        let mut bar = nif::Mesh {
            name: "Weapon:0".into(),
            block: 0,
            transform: Transform::IDENTITY,
            positions: Vec::new(),
            normals: Vec::new(),
            tangents: Vec::new(),
            bitangents: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            triangles: Vec::new(),
            bound: ([0.0; 3], 0.0),
            nodes: Vec::new(),
            billboard: None,
            textures: Vec::new(),
            shader: None,
            material: None,
            alpha: None,
            zbuffer: None,
            double_sided: false,
            skinned: false,
            skin: None,
            property_types: vec!["NiMaterialProperty".into()],
        };
        assert!(!drawn(&bar));
        bar.shader = Some(shader(0));
        assert!(drawn(&bar));
    }

    #[test]
    fn hair_lays_its_layer_map_over_the_hair_texture() {
        let mut hair = nif::Mesh {
            name: "NoHat".into(),
            block: 0,
            transform: Transform::IDENTITY,
            positions: Vec::new(),
            normals: Vec::new(),
            tangents: Vec::new(),
            bitangents: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            triangles: Vec::new(),
            bound: ([0.0; 3], 0.0),
            nodes: Vec::new(),
            billboard: None,
            textures: vec![
                "textures\\characters\\hair\\HairBun.dds".into(),
                "textures\\characters\\hair\\HairBun_n.dds".into(),
                "textures\\characters\\hair\\HairBun_hl.dds".into(),
            ],
            shader: Some(shader(HAIR_FLAG)),
            material: None,
            alpha: None,
            zbuffer: None,
            double_sided: false,
            skinned: false,
            skin: None,
            property_types: Vec::new(),
        };
        let part = ActorPart {
            model: "Characters\\Hair\\HairBun.NIF".into(),
            skin_texture: None,
            texture: Some("Characters\\Hair\\HairBun.dds".into()),
            hide_mesh: Some("Hat".into()),
            hair_tint: Some([66, 28, 15]),
            facegen: true,
            face_tint: None,
            bone: None,
            parent_bone: None,
        };
        retexture(&mut hair, &part);
        let (layer, base) = composite_parts(&hair.textures[0]).unwrap();
        assert_eq!(layer, "textures\\characters\\hair\\hairbun_hl.dds");
        assert_eq!(base, "textures\\characters\\hair\\hairbun.dds");
        assert!(hair_layer_parts(&hair.textures[0]).is_some());
    }

    #[test]
    fn head_parts_carry_their_morphs_shaped_by_the_face() {
        // Two vertices; a blink moving the second to a target, and a
        // morph named like none of the engine's channels.
        let tri = nif::Tri {
            base: vec![[0.0; 3], [0.0, 0.0, 1.0]],
            differential: vec![nif::tri::DifferentialMorph {
                name: "Ee".into(),
                offsets: vec![[1.0; 3]; 2],
            }],
            statistical: vec![nif::tri::StatisticalMorph {
                name: "BlinkLeft".into(),
                vertices: vec![1],
                targets: vec![[0.0, 0.0, -1.0]],
                first_target: 0,
            }],
        };
        // The face raises every point (both vertices and the target) by 2.
        let mut egm = b"FREGM002".to_vec();
        for v in [3u32, 1, 0] {
            egm.extend(v.to_le_bytes());
        }
        egm.extend([0u8; 44]);
        egm.extend(1.0f32.to_le_bytes());
        for _ in 0..3 {
            for c in [0i16, 0, 2] {
                egm.extend(c.to_le_bytes());
            }
        }
        let egm = nif::Egm::parse(&egm).unwrap();
        let face = Face {
            symmetric: vec![1.0],
            asymmetric: Vec::new(),
        };
        let shaped = [[0.0, 0.0, 2.0], [0.0, 0.0, 3.0]];
        let morphs = face_morphs(Some(&tri), Some(&egm), Some(&face), &shaped).unwrap();
        assert_eq!(morphs.morphs.len(), 1, "`Ee` never moves anything");
        // Closed, the lid lands on the target as the face moved it.
        assert_eq!(morphs.morphs[0].moves, [(1, [0.0, 0.0, -2.0])]);
        // A part without a `.tri`, or with nothing the engine uses: none.
        assert!(face_morphs(None, Some(&egm), Some(&face), &shaped).is_none());
        let unused = nif::Tri {
            statistical: Vec::new(),
            ..tri
        };
        assert!(face_morphs(Some(&unused), None, None, &shaped).is_none());
    }

    #[test]
    fn vertices_keep_their_four_strongest_bones() {
        let w = strongest_four(&[(1, 0.1), (2, 0.4), (3, 0.2), (4, 0.2), (5, 0.1)]);
        assert_eq!(w[0].0, 2);
        let total: f32 = w.iter().map(|x| x.1).sum();
        assert!((total - 1.0).abs() < 1e-6, "{w:?}");
        // One of the two weakest is dropped.
        assert!(!(w.iter().any(|x| x.0 == 1) && w.iter().any(|x| x.0 == 5)));
        // No weights: the first joint carries the vertex.
        assert_eq!(strongest_four(&[])[0], (0, 1.0));
    }

    #[test]
    fn head_parts_take_whichever_mesh_transform_is_upright() {
        let turn = |rotation: [[f32; 3]; 3]| Transform {
            rotation,
            translation: [0.0, 0.0, 0.1],
            scale: 1.0,
        };
        // `hairbun.nif`: the mesh node turned (x → -x, y ↔ z), the top
        // node turning it back.
        let mesh_node = turn([[-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]]);
        let with_top = turn([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        assert_eq!(upright(&with_top, &mesh_node), with_top);
        // The eyes: only the top node turned (a quarter about y).
        let eye_with_top = turn([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]]);
        assert_eq!(upright(&eye_with_top, &with_top), with_top);
    }

    #[test]
    fn other_rigid_pieces_keep_their_top_nodes_turn() {
        // A creature's eyes: the top node turned 180° about a tilted axis,
        // the mesh node not at all.
        let top = Transform {
            rotation: [[0.4, 0.9165, 0.0], [0.9165, -0.4, 0.0], [0.0, 0.0, -1.0]],
            translation: [0.0; 3],
            scale: 1.0,
        };
        let mesh = Transform::IDENTITY;
        assert_eq!(rigid_transform(false, &top, &mesh), top);
        // A FaceGen eye with the same files takes the upright one.
        assert_eq!(rigid_transform(true, &top, &mesh), mesh);
    }

    #[test]
    fn rigid_pieces_follow_the_bone_in_upright_axes() {
        // A head bone a quarter-turned in its bind pose, at 100 up.
        let turned = [[0.0, 0.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
        let bind = Transform {
            rotation: turned,
            translation: [0.0, 0.0, 100.0],
            scale: 1.0,
        };
        let pose = Posed {
            posed: [("bip01 head".to_string(), bind)].into(),
            bind: [("bip01 head".to_string(), bind)].into(),
            index: HashMap::new(),
            skeleton: Arc::new(ActorSkeleton::default()),
        };
        let t = attachment(&pose, "Bip01 Head").unwrap();
        // In the bind pose, a point 15 above the bone stays 15 above it.
        let p = t.apply_point([0.0, 0.0, 15.0]);
        assert!((p[2] - 115.0).abs() < 1e-5 && p[0].abs() < 1e-5, "{p:?}");
    }
}
