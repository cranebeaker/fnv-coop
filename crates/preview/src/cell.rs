//! Turning a loaded cell into images: load each object's model and
//! textures once, place them with a rotation convention, and draw a floor
//! plan and a first-person view.

use std::collections::{BTreeMap, HashMap};

use assets::Assets;
use nif::math::Transform;
use world::{is_tilted, LoadedCell, RotationConvention};

use crate::raster::{
    self, add, normalize, plane_segment, scale, AlphaMode, BlendFactor, Camera, Falloff, Lighting,
    Material, MeshInput, Plane, PointLight, Renderer, Texture, Vec3,
};

/// Textures are shrunk to at most this size, which is plenty for a preview.
const MAX_TEXTURE_SIZE: u32 = 512;
/// How far above the floor the plan's ceiling cut sits, in game units.
pub const DEFAULT_CUT_HEIGHT: f32 = 160.0;
/// Eye height above the floor for the first-person view. Matches the game:
/// lined up with an in-game screenshot at a known `player.getpos`, the
/// wall lamps sit at the same height on screen.
pub const EYE_HEIGHT: f32 = 120.0;
/// The game's field of view setting (`fDefaultFOV` in `FalloutPrefs.ini`).
pub const GAME_FOV_DEGREES: f32 = 75.0;

/// The vertical field of view (radians) for the game's FOV setting: the
/// setting is the width of a 4:3 picture, and wider screens keep that
/// picture's height. Confirmed against an in-game 1920x1080 screenshot,
/// whose view is 91.3° wide, not 75°.
pub fn vertical_fov(setting_degrees: f32) -> f32 {
    2.0 * ((setting_degrees.to_radians() * 0.5).tan() * 0.75).atan()
}

/// The horizontal field of view (radians) for the game's FOV setting on a
/// picture of the given width / height (see [`vertical_fov`]).
pub fn horizontal_fov(setting_degrees: f32, aspect: f32) -> f32 {
    2.0 * ((vertical_fov(setting_degrees) * 0.5).tan() * aspect).atan()
}

/// Shader flag: a decal drawn over another surface.
const DECAL_FLAGS: u32 = 0x0400_0000 | 0x0800_0000;
/// Shader flag: the glow takes its color from outside the model (the placed
/// object's Emittance setting).
const EXTERNAL_EMITTANCE: u32 = 0x2000_0000;
/// Shader flag 0x40: what later games call "use falloff" (a guess for
/// these files: see the falloff in `Loader::mesh`).
const USE_FALLOFF: u32 = 0x0000_0040;
/// Shader flag: vertex alpha is used.
const VERTEX_ALPHA: u32 = 0x0000_0008;
/// Shader flag: the surface gets the specular pass.
const SPECULAR: u32 = 0x0000_0001;
/// Shader flag: the surface gets the reflection (environment map) pass.
const ENVIRONMENT_MAP: u32 = 0x0000_0080;
/// Shader flag: window reflections, drawn with the view direction turned
/// around (the game's `SLS2058` rather than `SLS2057`).
const WINDOW_ENVIRONMENT_MAP: u32 = 0x0020_0000;

/// The cube map the game reflects when a model names none: this flat
/// picture copied onto all six faces. Confirmed byte for byte from a
/// recording of the game uploading it at startup (64×64, 7 levels); the
/// microscope and IV stand in Doc Mitchell's house reflect it.
pub const DEFAULT_ENVIRONMENT_MAP: &str = "textures\\effects\\reflection.dds";

/// The reflection pass a lit surface gets (the game's `SLS2057` /
/// `SLS2058`, added on top of the finished surface): the cube map looked
/// up along the view reflected about the normal, times the mask, the
/// strength, the material's opacity and the vertex color, faded by fog.
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    /// The model's cube map, or [`DEFAULT_ENVIRONMENT_MAP`].
    pub cube_path: String,
    /// Where reflections show (red channel). Without one, the normal
    /// map's alpha is the mask.
    pub mask_path: Option<String>,
    /// The shader's environment map scale.
    pub strength: f32,
    /// Window reflections: the view direction turned around.
    pub window: bool,
}

/// Which of the game's lighting a lit surface gets, by its shader flags
/// (read from the actors recording, `FalloutNVActors.trace`):
/// - `Skin` (flag 0x400: faces, bare arms and hands): the skin passes. The
///   directional light comes in its own pass times the image space's
///   `skin_directional` (`world::Hdr`); each point light's pass (shaders
///   the game compiles at run time) adds a rim, `(1 − N·V)² × saturate(−L·V)
///   × (0.5 × colour + (0.15, 0, 0))`, and a red glow where the light wraps
///   past the edge, `(0.3, 0, 0) × saturate(S(w) − S(N·L))` with `S` the
///   smooth step `3x² − 2x³`, `w = saturate((N·L + 0.3) / 1.3)`, both times
///   the light's fade with distance. Skin always gets the specular pass,
///   whatever flag 0x1 says (Sunny Smiles' bare arms, flags 0x82000402,
///   got it).
/// - `Hair` (flag 0x40000): the hair shader (also compiled at run time):
///   texture × `lerp(1, 2 × HairTint, vertex green)`, the texture blended
///   with the model's layer map (texture slot 2) by its alpha; no
///   ordinary specular pass, but a highlight from each point light,
///   `0.7 × (1 − saturate(|B·L − B·H|))³⁰ × fade × colour ×
///   max(L·N, 0)`, with `B = normalize(0.5 × normal map normal + vertex
///   normal)`, times the vertex green and the normal map's alpha (the
///   directional light's highlight is multiplied by 0 in every recorded
///   draw).
/// - `Plain`: everything else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shading {
    #[default]
    Plain,
    Skin,
    Hair,
}

impl Shading {
    /// A lit surface's shading by its shader flags.
    pub fn from_flags(flags: u32) -> Shading {
        if flags & crate::actor::HAIR_FLAG != 0 {
            Shading::Hair
        } else if flags & crate::actor::SKIN_FLAG != 0 {
            Shading::Skin
        } else {
            Shading::Plain
        }
    }
}

/// The specular highlight a lit surface gets (the game's separate additive
/// pass): the material's specular color and glossiness, from its
/// `NiMaterialProperty`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Specular {
    pub color: Vec3,
    pub glossiness: f32,
}

/// One drawable piece of a model, in the model's space.
#[derive(Clone)]
pub struct ModelMesh {
    pub name: String,
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    /// The tangent space for the normal map, measured from the game's files
    /// (`nvinspect nif` prints it): the direction the texture's U runs and
    /// the direction its V runs at each vertex. The file stores them the
    /// other way round from their names: its "tangents" run along V and its
    /// "bitangents" along U. Empty when the mesh stores none.
    pub along_u: Vec<Vec3>,
    pub along_v: Vec<Vec3>,
    /// The normal map as the mesh names it (lit meshes). Its alpha is the
    /// specular mask.
    pub normal_path: Option<String>,
    /// Set when the shader's specular flag is on.
    pub specular: Option<Specular>,
    /// Set when the shader's environment map flag is on (lit meshes).
    pub environment: Option<Environment>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub triangles: Vec<[u16; 3]>,
    pub texture: Option<usize>,
    /// The diffuse texture as the mesh names it, if it names one.
    pub texture_path: Option<String>,
    pub alpha: AlphaMode,
    pub double_sided: bool,
    /// Tested against the depth buffer, and writing its depth (see
    /// [`depth_of`]).
    pub depth_test: bool,
    pub depth_write: bool,
    /// The centre of the bounding sphere the file stores with the mesh, in
    /// the model's space: what the game sorts blended meshes by (see
    /// `cellview::MaterialData::sort_center`).
    pub bound_center: Vec3,
    /// Big enough for the local map's pictures (`RegisterObject_LocalMap`,
    /// `00b64440`: the mesh's bound reaches 50 units). The pass also wants
    /// the property's runtime flag 0x40000 (`Show in Local Map`), which no
    /// mesh of Doc Mitchell's house has in its file: the game sets it on the
    /// place's objects at run time [guess: where isn't traced; every placed
    /// object not hidden from the local map is taken].
    pub local_map: bool,
    pub lit: bool,
    /// Self-lit color, with its multiplier applied. It is added to the light
    /// reaching the surface (masked by the glow map, if there is one), so it
    /// multiplies the diffuse texture. See [`CellScene::emissive_for`] for
    /// the color a placed copy actually uses.
    pub emissive: Vec3,
    /// The self-lit color's multiplier, on its own.
    pub emissive_mult: f32,
    /// The glow's color comes from the placed object's Emittance setting
    /// rather than from the model.
    pub external_emittance: bool,
    /// The glow map, loaded only for self-lit meshes.
    pub glow: Option<usize>,
    /// The glow map as the mesh names it, for self-lit meshes.
    pub glow_path: Option<String>,
    pub decal: bool,
    /// The material's opacity (multiplies texture and vertex alpha).
    pub opacity: f32,
    /// Fades the surface by viewing angle (no-lighting effect shaders).
    pub falloff: Option<Falloff>,
    /// A light effect rather than a solid surface: a glow, light beam or
    /// haze drawn with blending. Left out of floor plans.
    pub effect: bool,
    /// Actor pieces: the piece in its bind pose and the bones that move
    /// it, for animating (see `preview::actor`).
    pub rig: Option<Rig>,
    /// For pieces of models with animations that move them: the nodes from
    /// the model's top down to this piece, with their own transforms
    /// (`nif::Mesh::nodes`), and all the model's sequences (in file order;
    /// see [`placed_sequences`] for which play). Empty otherwise.
    pub nodes: Vec<(String, Transform)>,
    pub sequences: std::sync::Arc<Vec<nif::Sequence>>,
    /// Pieces under an `NiBillboardNode` that turns them to the camera.
    pub billboard: Option<Billboard>,
    /// Which of the game's lighting it gets (lit meshes).
    pub shading: Shading,
    /// Hair pieces: the NPC's hair colour (`HCLR` / 255), the shader's
    /// `HairTint` (already laid into the vertex colours, see
    /// `preview::actor`).
    pub hair_tint: Option<[f32; 3]>,
    /// Actor pieces: which of the look's parts (`world::ActorLook::parts`)
    /// the piece comes from, so a part can be rebuilt on its own (the
    /// game's biped slots, `BipedAnim::LoadBipedParts`, Xbox PDB).
    pub actor_part: Option<u16>,
    /// The file stores it as triangle strips (`NiTriStrips`): the only
    /// geometry the game puts world decals on (`004a1a70`, `0068b4c0` take
    /// the `NiTriStrips` type at `011f4a20`; `0068d230` walks the strip).
    pub strips: bool,
}

/// A piece that turns toward the camera with the `NiBillboardNode` above
/// it. Its vertices are loaded at rest (`above · node · below`); drawn, the
/// node's rotation is replaced each frame so that the node's x and y lie
/// in the picture (x right, y up) and its z points back at the camera,
/// keeping its position and size, as Gamebryo's billboards do. The game's
/// glow cards are modelled flat in their node's x–y plane
/// (`fxbasicglowbillboard.nif`, the shack lights' `LightGlow01`), so they
/// face the camera this way.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Billboard {
    /// The nodes above the billboard node, composed (model space).
    pub above: Transform,
    /// The billboard node's own transform.
    pub node: Transform,
    /// The nodes below it down to the piece, composed.
    pub below: Transform,
    pub kind: BillboardKind,
}

/// How a billboard turns (`NiBillboardNode`'s mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillboardKind {
    /// Modes 0 and 2 (always / rigidly face the camera): parallel to the
    /// picture. The game's camera never rolls, so the two are the same.
    FaceCamera,
    /// Modes 3 and 4 (face the centre): toward the camera's position, its
    /// y as near world up as it can be.
    FaceCentre,
}

impl BillboardKind {
    /// The kinds drawn turning; the others (1, 5, 9: rotate about the up
    /// axis) stay as modelled (not done).
    pub fn of_mode(mode: u16) -> Option<BillboardKind> {
        match mode {
            0 | 2 => Some(BillboardKind::FaceCamera),
            3 | 4 => Some(BillboardKind::FaceCentre),
            _ => None,
        }
    }
}

impl Billboard {
    /// For a piece under a billboard node that turns.
    pub fn of(mesh: &nif::Mesh) -> Option<Billboard> {
        let (at, mode) = mesh.billboard?;
        let kind = BillboardKind::of_mode(mode)?;
        let (_, node) = mesh.nodes.get(at)?;
        Some(Billboard {
            above: nif::posed_chain(&mesh.nodes[..at], &[]),
            node: *node,
            below: nif::posed_chain(&mesh.nodes[at + 1..], &[]),
            kind,
        })
    }

    /// The piece's move in the model's space (from its loaded vertices)
    /// for a camera at `eye` (game units) whose right, up and backward
    /// directions are given (game axes), with the object placed by
    /// `placement` (model to world).
    pub fn facing(&self, placement: &Transform, eye: Vec3, axes: [Vec3; 3]) -> Transform {
        self.facing_posed((&self.above, &self.node, &self.below), placement, eye, axes)
    }

    /// [`Self::facing`] with the nodes posed by an animation (`above`, the
    /// billboard node's own and `below` as it has them now): the move from
    /// the loaded vertices to the turned, posed piece. The billboard keeps
    /// its posed node's position and size, its rotation replaced.
    pub fn facing_posed(
        &self,
        (above, node, below): (&Transform, &Transform, &Transform),
        placement: &Transform,
        eye: Vec3,
        axes: [Vec3; 3],
    ) -> Transform {
        let parent = placement.then_child(above);
        let node_world = parent.then_child(node);
        let [right, up, back] = match self.kind {
            BillboardKind::FaceCamera => axes,
            BillboardKind::FaceCentre => {
                let p = node_world.translation;
                let back = normalize([eye[0] - p[0], eye[1] - p[1], eye[2] - p[2]]);
                let right = normalize(raster::cross([0.0, 0.0, 1.0], back));
                [right, raster::cross(back, right), back]
            }
        };
        // World rotation with those columns; the node's local rotation is
        // that relative to its parent's.
        let world_rotation = [
            [right[0], up[0], back[0]],
            [right[1], up[1], back[1]],
            [right[2], up[2], back[2]],
        ];
        let parent_inverse = parent.inverse();
        let local = Transform {
            rotation: nif::math::mat_mul(&parent_inverse.rotation, &world_rotation),
            translation: node.translation,
            scale: node.scale,
        };
        let rest = self.above.then_child(&self.node).then_child(&self.below);
        above
            .then_child(&local)
            .then_child(below)
            .then_child(&rest.inverse())
    }
}

/// Base object types the game opens and closes (`0047a490`: activators,
/// terminals, containers and doors). Placed, their models' `Open` and
/// `Close` sequences show their state.
pub fn opens_and_closes(base_type: esm::FourCC) -> bool {
    [b"ACTI", b"TERM", b"CONT", b"DOOR"]
        .iter()
        .any(|t| base_type == esm::FourCC::new(t))
}

/// The animations a placed object shows from the moment it appears, out
/// of its model's sequences (in file order). Read from the game's code
/// (`005659f0`, which sets up a placed object's animation, and `0047aec0`,
/// an openable object's state) and checked against the recordings:
///
/// - an object that opens and closes ([`opens_and_closes`]) whose model
///   has both an `Open` and a `Close` sequence is shown closed: `Close`
///   held at its end (the footlockers in Doc Mitchell's house were
///   recorded with their lids where `Close` ends; `0047aec0` activates
///   the sequence as if it had started at time 0, so the next frame is
///   already at its end). A door placed open is the viewer's business
///   (`world::doors`: `Open` at its end);
/// - the `Idle` sequence plays (when it loops: the game complains "Idle
///   animation must be looping" about one that doesn't and leaves it off),
///   and the `SpecialIdle` sequence (the ceiling fan, Goodsprings'
///   windmill);
/// - with neither of those, and unless the object was just set to its
///   state, the first sequence is set going and the model's animation
///   switched off again straight after (`0047aa40` turned on, then off,
///   around it): it shows that sequence's first frame and stays there.
///   Goodsprings' mailboxes (containers whose first sequence, `Forward`,
///   raises the flag) were recorded with the flag down.
///
/// Each starts when the object appears, at full weight. Sequences moving
/// the same node are laid one over another in this order (the game blends
/// them; a guess). Later, a script's `PlayGroup` replaces them with the
/// sequence it names ([`MeshMotion::layers`]); opening and closing aren't
/// played. Movements of nodes and the material's opacity and glow are
/// played (see `nif::MaterialTrack`); texture, other colour and
/// visibility controllers aren't.
pub fn placed_sequences(all: &[nif::Sequence], openable: bool) -> Vec<Playing> {
    let named = |name: &str| all.iter().find(|s| s.name.eq_ignore_ascii_case(name));
    let runs = |s: &nif::Sequence| Playing {
        sequence: s.clone(),
        runs: true,
        at_end: false,
    };
    let mut playing = Vec::new();
    let mut closed = false;
    if openable {
        if let (Some(_), Some(close)) = (named("Open"), named("Close")) {
            playing.push(Playing {
                sequence: close.clone(),
                runs: false,
                at_end: true,
            });
            closed = true;
        }
    }
    let idle = named("Idle");
    let special = named("SpecialIdle");
    if idle.is_none() && special.is_none() {
        if !closed {
            playing.extend(all.first().map(|s| Playing {
                sequence: s.clone(),
                runs: false,
                at_end: false,
            }));
        }
    } else {
        playing.extend(idle.filter(|s| s.looping).map(runs));
        playing.extend(special.map(runs));
    }
    playing
}

/// A sequence a placed object shows ([`placed_sequences`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Playing {
    pub sequence: nif::Sequence,
    /// It plays on; otherwise it holds its first frame, or its last with
    /// `at_end`.
    pub runs: bool,
    pub at_end: bool,
}

/// How a piece of a placed model moves, and how its material changes,
/// with the model's own animation: the sequences shown from the start
/// ([`placed_sequences`]), or one a script starts (`PlayGroup`).
#[derive(Debug, Clone, PartialEq)]
pub struct MeshMotion {
    /// The nodes from the model's top down to the piece, with their own
    /// transforms (`nif::Mesh::nodes`); the last is the piece itself,
    /// whose name material tracks give.
    pub nodes: Vec<(String, Transform)>,
    /// The sequences shown from the start that change it.
    pub sequences: Vec<Playing>,
    /// Every sequence of the model, for scripts' `PlayGroup`.
    pub all: std::sync::Arc<Vec<nif::Sequence>>,
}

/// A piece's material values a sequence sets at a moment (`None`: left as
/// the material has it).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MaterialNow {
    pub alpha: Option<f32>,
    pub emissive: Option<Vec3>,
    pub emissive_mult: Option<f32>,
}

impl MeshMotion {
    /// The motion of a piece with these nodes, if any sequence of the
    /// model changes it (moves one of its nodes or changes its material);
    /// `playing` are the ones shown from the start.
    pub fn of(
        nodes: &[(String, Transform)],
        playing: &[Playing],
        all: &std::sync::Arc<Vec<nif::Sequence>>,
    ) -> Option<MeshMotion> {
        let changes = |s: &nif::Sequence| touches(nodes, s);
        if !all.iter().any(changes) {
            return None;
        }
        Some(MeshMotion {
            nodes: nodes.to_vec(),
            sequences: playing
                .iter()
                .filter(|p| changes(&p.sequence))
                .cloned()
                .collect(),
            all: all.clone(),
        })
    }

    /// The sequences shown `seconds` after the object appeared, each at
    /// its own time: looping ones wrap, others play once and hold their
    /// last pose, held ones stay at their first frame. With `group` (a
    /// sequence a script started, and the seconds since), that one alone,
    /// played once from its start (`PlayGroup` takes over the model's
    /// animation; a guess at how the game mixes it with what was playing).
    /// The sequences' frequency is taken as 1 (a guess: not read).
    pub fn layers(&self, seconds: f32, group: Option<(&str, f32)>) -> Vec<(&nif::Sequence, f32)> {
        if let Some((name, since)) = group {
            if let Some(s) = self.all.iter().find(|s| s.name.eq_ignore_ascii_case(name)) {
                return vec![(s, sequence_time(s, since))];
            }
        }
        self.sequences
            .iter()
            .map(|p| {
                let s = &p.sequence;
                (
                    s,
                    if p.runs {
                        sequence_time(s, seconds)
                    } else if p.at_end {
                        s.stop
                    } else {
                        s.start
                    },
                )
            })
            .collect()
    }

    /// One of the model's sequences by name at a moment of its own clock
    /// (seconds from its start, clamped to its length): a door's `Open`
    /// or `Close` as it swings (`world::doors`). `None` when the model
    /// has no sequence of that name.
    pub fn sequence_at(&self, name: &str, seconds: f32) -> Option<(&nif::Sequence, f32)> {
        let s = self
            .all
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))?;
        let length = (s.stop - s.start).max(0.0);
        Some((s, s.start + seconds.clamp(0.0, length)))
    }

    /// Where these layers have the piece, as a move in the model's space
    /// from where the piece rests (its vertices as loaded).
    pub fn transform(&self, layers: &[(&nif::Sequence, f32)]) -> Transform {
        let rest = nif::posed_chain(&self.nodes, &[]);
        nif::posed_chain(&self.nodes, layers).then_child(&rest.inverse())
    }

    /// The material values these layers give the piece (a later layer's
    /// over an earlier one's).
    pub fn material(&self, layers: &[(&nif::Sequence, f32)]) -> MaterialNow {
        let mut now = MaterialNow::default();
        let Some((shape, _)) = self.nodes.last() else {
            return now;
        };
        for (sequence, time) in layers {
            for track in &sequence.materials {
                if !track.node.eq_ignore_ascii_case(shape) {
                    continue;
                }
                match track.target {
                    nif::MaterialTarget::Alpha => now.alpha = track.float_at(*time).or(now.alpha),
                    nif::MaterialTarget::EmissiveMult => {
                        now.emissive_mult = track.float_at(*time).or(now.emissive_mult)
                    }
                    nif::MaterialTarget::Emissive => {
                        now.emissive = track.color_at(*time).or(now.emissive)
                    }
                }
            }
        }
        now
    }

    /// Where the animation has the piece `seconds` after the object
    /// appeared (the sequences shown from the start).
    pub fn at(&self, seconds: f32) -> Transform {
        self.transform(&self.layers(seconds, None))
    }

    /// Whether the piece ever moves by itself (a held first frame doesn't).
    pub fn moves(&self) -> bool {
        self.sequences.iter().any(|p| p.runs)
    }
}

/// Whether a sequence moves one of these nodes or changes the last one's
/// (the piece's) material.
fn touches(nodes: &[(String, Transform)], s: &nif::Sequence) -> bool {
    s.tracks
        .iter()
        .any(|t| nodes.iter().any(|(n, _)| n.eq_ignore_ascii_case(&t.node)))
        || nodes.last().is_some_and(|(shape, _)| {
            s.materials
                .iter()
                .any(|m| m.node.eq_ignore_ascii_case(shape))
        })
}

/// Whether a sequence `since` seconds after it started still counts as
/// playing (`IsAnimPlaying`): a looping one always, any other until its end
/// (checked against the original game: a one-shot group reads 1 while it
/// plays and 0 once it has finished, for good; `Backward` on Dead Money's
/// Elijah projector).
pub fn sequence_playing(s: &nif::Sequence, since: f32) -> bool {
    s.looping || since < s.stop - s.start
}

/// A sequence's own clock `seconds` after it started.
fn sequence_time(s: &nif::Sequence, seconds: f32) -> f32 {
    let length = s.stop - s.start;
    if length <= 0.0 {
        return s.start;
    }
    if s.looping {
        s.start + seconds.max(0.0) % length
    } else {
        s.start + seconds.clamp(0.0, length)
    }
}

/// How an actor piece moves with its skeleton. `world = bone pose ×
/// joint transform × vertex`, blended over up to four joints per vertex.
#[derive(Debug, Clone)]
pub struct Rig {
    /// The piece's own vertices, before posing (same order as the mesh's).
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub along_u: Vec<Vec3>,
    pub along_v: Vec<Vec3>,
    /// Per joint: the bone of the actor's skeleton (an index into
    /// [`ActorSkeleton::bones`]) and the transform from the piece's space
    /// to that bone's (its skin-to-bone transform).
    pub joints: Vec<(usize, Transform)>,
    /// Per vertex: four (joint, weight), the weights adding up to 1.
    pub weights: Vec<[(u16, f32); 4]>,
    /// FaceGen head parts: the morphs they talk and blink with (from the
    /// part's `.tri`), moving `positions` (see `world::face`).
    pub face: Option<std::sync::Arc<world::face::FaceMorphs>>,
}

/// An actor's skeleton, the idle it plays, and its walk, and its fighting
/// animations (see `world::Fighting`).
#[derive(Debug, Clone, Default)]
pub struct ActorSkeleton {
    pub bones: Vec<nif::Bone>,
    pub idle: Option<std::sync::Arc<nif::Sequence>>,
    pub walk: Option<std::sync::Arc<nif::Sequence>>,
    /// A weapon put away (where it hangs, `nif::hang_weapon`), held
    /// ready, running, and the attack.
    pub holster: Option<std::sync::Arc<nif::Sequence>>,
    pub aim: Option<std::sync::Arc<nif::Sequence>>,
    pub run: Option<std::sync::Arc<nif::Sequence>>,
    pub attack: Option<std::sync::Arc<nif::Sequence>>,
    /// Turning in place, left and right: `mtturnleft.kf` and
    /// `mtturnright.kf` in the walk's `locomotion` folder (the groups
    /// `TurnLeft`/`TurnRight` the game's animation picker takes for a turn
    /// in place, `00895110`; the folder's `*.KF` are among the files the
    /// game loads for the skeleton, see [`crate::actor::turn_path`]).
    pub turn_left: Option<std::sync::Arc<nif::Sequence>>,
    pub turn_right: Option<std::sync::Arc<nif::Sequence>>,
    /// The bodies the skeleton carries for when its actor dies.
    pub ragdoll: Option<crate::ragdoll::RagdollRig>,
    /// The skeleton's box (`BSBound`), which sizes a creature's collision
    /// (`world::combat_ai::creature_radius`).
    pub bound: Option<nif::Bound>,
    /// Where the skeleton, its idle and its walk were read from (relative
    /// to `meshes\`): the folders the actor's animations are loaded from
    /// (`00447330`, `008b73f0`).
    pub skeleton_path: String,
    pub idle_path: String,
    pub walk_path: String,
}

pub struct Model {
    pub path: String,
    pub meshes: Vec<ModelMesh>,
    /// The model's solid collision (Havok shapes, game units, model
    /// space), placed the same way as its meshes.
    pub collision: Vec<nif::CollisionPart>,
    /// The transform stored on the model's top node, if any. The game
    /// ignores it for placed models, and so does this renderer unless told
    /// otherwise.
    pub root_transform: Option<Transform>,
    /// For actors: the skeleton their pieces' rigs refer to.
    pub skeleton: Option<std::sync::Arc<ActorSkeleton>>,
    /// The model's own animation sequences (`NiControllerSequence`s), in
    /// file order; what its pieces' `sequences` share.
    pub sequences: std::sync::Arc<Vec<nif::Sequence>>,
    /// Its particle systems, if it has any.
    pub particles: Option<std::sync::Arc<ParticleModel>>,
    /// Its `BSXFlags` value (0 without the block).
    pub bsx_flags: u32,
}

/// A model's particle systems (`nif::particles`, as placed: the top node's
/// transform left out), the sequences that drive their controllers, and
/// all the model's sequences, for which play ([`placed_sequences`]).
#[derive(Debug, Clone, Default)]
pub struct ParticleModel {
    pub systems: Vec<std::sync::Arc<nif::particles::ParticleSystem>>,
    pub sequences: Vec<nif::particles::ParticleSequence>,
    pub all_sequences: std::sync::Arc<Vec<nif::Sequence>>,
}

/// One model placed in the cell: an object, or one piece of a static
/// collection whose combined model is missing.
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    /// Index into [`LoadedCell::objects`].
    pub object: usize,
    /// Index into the object's static collection parts.
    pub part: Option<usize>,
    pub model: usize,
}

/// What went into a scene, and what couldn't.
#[derive(Debug, Clone, Default)]
pub struct SceneReport {
    /// Model path → number of objects using it.
    pub missing_models: BTreeMap<String, usize>,
    /// Model path → error.
    pub unreadable_models: BTreeMap<String, String>,
    /// Model path → why its collision couldn't be read.
    pub unreadable_collision: BTreeMap<String, String>,
    /// Texture path → number of meshes using it.
    pub missing_textures: BTreeMap<String, usize>,
    pub unreadable_textures: BTreeMap<String, String>,
    /// Textures found only after correcting the file extension.
    pub renamed_textures: usize,
    /// Static collections drawn piece by piece.
    pub collections_from_parts: usize,
    pub skinned_meshes: usize,
    pub skipped_meshes: BTreeMap<String, usize>,
    /// Idle animations that couldn't be found (their actors stand in the
    /// skeleton's own pose).
    pub missing_animations: std::collections::BTreeSet<String>,
    /// People and creatures drawn.
    pub actors_drawn: usize,
}

/// A cell with its models and textures loaded.
pub struct CellScene {
    pub cell: LoadedCell,
    pub models: Vec<Model>,
    pub textures: Vec<Texture>,
    pub instances: Vec<Instance>,
    pub report: SceneReport,
}

pub(crate) struct Loader<'a> {
    pub(crate) assets: &'a Assets,
    keep_root_transforms: bool,
    pub(crate) models: Vec<Model>,
    pub(crate) model_index: HashMap<String, Option<usize>>,
    textures: Vec<Texture>,
    texture_index: HashMap<String, Option<usize>>,
    pub(crate) report: SceneReport,
    /// Skeletons posed by their idles, and actor parts in those poses.
    pub(crate) poses: HashMap<String, Option<std::rc::Rc<crate::actor::Posed>>>,
    pub(crate) posed_parts: HashMap<String, Vec<ModelMesh>>,
}

impl Loader<'_> {
    fn model(&mut self, reference: &str) -> Option<usize> {
        let path = assets::mesh_path(reference);
        if let Some(&known) = self.model_index.get(&path) {
            return known;
        }
        let loaded = self.load_model(&path);
        self.model_index.insert(path, loaded);
        loaded
    }

    fn load_model(&mut self, path: &str) -> Option<usize> {
        let bytes = match self.assets.read(path) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => return None,
            Err(e) => {
                self.report
                    .unreadable_models
                    .insert(path.to_string(), e.to_string());
                return None;
            }
        };
        let keep_root = self.keep_root_transforms;
        let mut collision = Vec::new();
        let mut sequences = Vec::new();
        let mut particle_systems = Vec::new();
        let mut particle_sequences = Vec::new();
        let mut bsx_flags = 0;
        let mut strip_blocks = std::collections::HashSet::new();
        let scene = match nif::Nif::parse(bytes).and_then(|nif| {
            strip_blocks = (0..nif.blocks().len())
                .filter(|&i| nif.block_type(i) == "NiTriStrips")
                .collect();
            sequences = nif.sequences().unwrap_or_default();
            bsx_flags = nif.bsx_flags().unwrap_or(0);
            // Particle systems that can't be read leave the model without
            // them (every one in the game's files reads).
            particle_systems = nif.particle_systems(keep_root).unwrap_or_default();
            if !particle_systems.is_empty() {
                particle_sequences = nif.particle_sequences().unwrap_or_default();
            }
            // Collision that can't be read leaves the model without it,
            // rather than without its meshes.
            let solid = if keep_root {
                nif.collision()
            } else {
                nif.placed_collision()
            };
            match solid {
                Ok(c) => collision = c.parts,
                Err(e) => {
                    self.report
                        .unreadable_collision
                        .insert(path.to_string(), e.to_string());
                }
            }
            if keep_root {
                nif.scene()
            } else {
                nif.placed_scene()
            }
        }) {
            Ok(scene) => scene,
            Err(e) => {
                self.report
                    .unreadable_models
                    .insert(path.to_string(), e.to_string());
                return None;
            }
        };
        let sequences = std::sync::Arc::new(sequences);
        let mut meshes = Vec::with_capacity(scene.meshes.len());
        for source in &scene.meshes {
            if let Some(mut mesh) = self.mesh(source) {
                mesh.strips = strip_blocks.contains(&source.block);
                if sequences.iter().any(|s| touches(&source.nodes, s)) {
                    mesh.nodes = source.nodes.clone();
                    mesh.sequences = sequences.clone();
                }
                meshes.push(mesh);
            }
        }
        let particles = (!particle_systems.is_empty()).then(|| {
            std::sync::Arc::new(ParticleModel {
                systems: particle_systems
                    .into_iter()
                    .map(std::sync::Arc::new)
                    .collect(),
                sequences: particle_sequences,
                all_sequences: sequences.clone(),
            })
        });
        self.models.push(Model {
            path: path.to_string(),
            meshes,
            collision,
            root_transform: scene.root_transform,
            skeleton: None,
            sequences,
            particles,
            bsx_flags,
        });
        Some(self.models.len() - 1)
    }

    pub(crate) fn mesh(&mut self, mesh: &nif::Mesh) -> Option<ModelMesh> {
        let shader_type = mesh.shader.as_ref().map(|s| s.type_name.as_str());
        if let Some(t) = shader_type.filter(|t| t.contains("Sky") || t.contains("Water")) {
            *self.report.skipped_meshes.entry(t.to_string()).or_default() += 1;
            return None;
        }
        // Placed water's models carry a `WaterShaderProperty` (not read as
        // a shader here); the water is drawn on its own (`cellview::water`).
        if let Some(t) = mesh
            .property_types
            .iter()
            .find(|t| t.as_str() == "WaterShaderProperty")
        {
            *self.report.skipped_meshes.entry(t.clone()).or_default() += 1;
            return None;
        }
        if !is_drawn(mesh) {
            *self
                .report
                .skipped_meshes
                .entry("no shader property".into())
                .or_default() += 1;
            return None;
        }
        if mesh.skinned {
            self.report.skinned_meshes += 1;
        }
        let unlit = shader_type.is_some_and(|t| t.contains("NoLighting"));
        let (vertex_rgb, vertex_alpha, decal) = vertex_color_use(mesh.shader.as_ref());
        let colors = if (vertex_rgb || vertex_alpha) && mesh.colors.len() == mesh.positions.len() {
            mesh.colors
                .iter()
                .map(|c| {
                    let rgb = |v: f32| if vertex_rgb { v } else { 1.0 };
                    [
                        rgb(c[0]),
                        rgb(c[1]),
                        rgb(c[2]),
                        if vertex_alpha { c[3] } else { 1.0 },
                    ]
                })
                .collect()
        } else {
            Vec::new()
        };
        // A face-tinted skin or layered hair keeps its combined reference
        // for the viewer; here its base texture is drawn.
        let diffuse = mesh.diffuse_texture();
        let (texture_path, texture) = match diffuse.and_then(crate::actor::composite_parts) {
            Some((_, base)) => (diffuse.map(str::to_string), self.texture(base)),
            None => (
                diffuse.map(assets::texture_path),
                diffuse.and_then(|t| self.texture(t)),
            ),
        };
        let emissive_mult = mesh.material.as_ref().map_or(1.0, |m| {
            if m.emissive_mult > 0.0 {
                m.emissive_mult
            } else {
                1.0
            }
        });
        let emissive = mesh
            .material
            .as_ref()
            .map_or([0.0; 3], |m| scale(m.emissive, emissive_mult));
        // Lit and no-lighting shaders alike: the light beams in Doc
        // Mitchell's house (no-lighting, flag set) are tinted by their
        // Emittance in game.
        let external_emittance = mesh
            .shader
            .as_ref()
            .is_some_and(|s| s.shader_flags & EXTERNAL_EMITTANCE != 0);
        // A mesh whose glow color comes from outside can glow even with a
        // black color of its own.
        let self_lit = !unlit && (external_emittance || emissive.iter().any(|&c| c > 0.0));
        let glow_path = mesh
            .glow_texture()
            .filter(|_| self_lit)
            .map(assets::texture_path);
        let glow = mesh
            .glow_texture()
            .filter(|_| self_lit)
            .and_then(|t| self.texture(t));
        let alpha = alpha_of(mesh);
        let additive = matches!(alpha.blend, Some((_, BlendFactor::One)));
        // [G] With `NV_GUESSES=1`, only shaders with flag 0x40 fade by
        // angle (the bit later games name "use falloff"; the game's impact
        // models, flags 0x82080008, store a falloff of all zeros, which
        // would hide them). Which no-lighting technique the exe picks isn't
        // traced.
        let falloff_flag = |s: &nif::ShaderProperty| {
            !world::guesses::enabled() || s.shader_flags & USE_FALLOFF != 0
        };
        let falloff = mesh
            .shader
            .as_ref()
            .filter(|s| falloff_flag(s))
            .and_then(|s| s.falloff)
            .map(|f| {
                // Stored as cosines; tolerate files that store degrees.
                let cos = |v: f32| {
                    if v.abs() > 1.0 {
                        v.to_radians().cos()
                    } else {
                        v
                    }
                };
                Falloff {
                    start_cos: cos(f.start_angle),
                    stop_cos: cos(f.stop_angle),
                    start_opacity: f.start_opacity,
                    stop_opacity: f.stop_opacity,
                }
            });
        let directions = |v: &[Vec3]| -> Vec<Vec3> {
            v.iter()
                .map(|&d| mesh.transform.apply_direction(d))
                .collect()
        };
        let shading = mesh
            .shader
            .as_ref()
            .filter(|s| s.lit && !unlit)
            .map_or(Shading::Plain, |s| Shading::from_flags(s.shader_flags));
        // Skin always gets the specular pass; hair has its own highlight
        // instead (see [`Shading`]).
        let specular = mesh
            .shader
            .as_ref()
            .filter(|s| {
                s.lit
                    && match shading {
                        Shading::Skin => true,
                        Shading::Hair => false,
                        Shading::Plain => s.shader_flags & SPECULAR != 0,
                    }
            })
            .map(|_| {
                mesh.material.as_ref().map_or(
                    Specular {
                        color: [1.0; 3],
                        glossiness: 10.0,
                    },
                    |m| Specular {
                        color: m.specular,
                        glossiness: m.glossiness,
                    },
                )
            });
        let environment = mesh
            .shader
            .as_ref()
            .filter(|s| s.lit && !unlit && s.shader_flags & ENVIRONMENT_MAP != 0)
            .map(|s| Environment {
                cube_path: assets::texture_path(
                    mesh.environment_texture()
                        .unwrap_or(DEFAULT_ENVIRONMENT_MAP),
                ),
                mask_path: mesh.environment_mask_texture().map(assets::texture_path),
                strength: s.env_map_scale,
                window: s.shader_flags & WINDOW_ENVIRONMENT_MAP != 0,
            });
        let (depth_test, depth_write) = depth_of(mesh);
        Some(ModelMesh {
            name: mesh.name.clone(),
            positions: mesh.model_positions().collect(),
            normals: directions(&mesh.normals),
            along_u: directions(&mesh.bitangents),
            along_v: directions(&mesh.tangents),
            normal_path: mesh
                .normal_texture()
                .filter(|_| !unlit)
                .map(assets::texture_path),
            specular: specular.filter(|_| !unlit),
            environment,
            uvs: mesh.uvs.clone(),
            colors,
            triangles: mesh.triangles.clone(),
            texture,
            texture_path,
            alpha,
            double_sided: mesh.double_sided,
            depth_test,
            depth_write,
            bound_center: mesh.model_bound().0,
            local_map: mesh.model_bound().1 >= 50.0,
            lit: !unlit,
            emissive,
            emissive_mult,
            external_emittance,
            glow,
            glow_path,
            decal,
            opacity: mesh
                .material
                .as_ref()
                .map_or(1.0, |m| m.alpha.clamp(0.0, 1.0)),
            falloff,
            effect: alpha.blend.is_some() && (unlit || additive),
            rig: None,
            nodes: Vec::new(),
            sequences: std::sync::Arc::default(),
            billboard: Billboard::of(mesh),
            shading,
            hair_tint: None,
            actor_part: None,
            strips: false,
        })
    }

    fn texture(&mut self, reference: &str) -> Option<usize> {
        let path = assets::texture_path(reference);
        if let Some(&known) = self.texture_index.get(&path) {
            return known;
        }
        let mut loaded = self.load_texture(&path);
        if loaded.is_none() && !path.ends_with(".dds") {
            // A few meshes name a texture with the wrong extension.
            if let Some((stem, _)) = path.rsplit_once('.') {
                loaded = self.load_texture(&format!("{stem}.dds"));
                if loaded.is_some() {
                    self.report.renamed_textures += 1;
                }
            }
        }
        if loaded.is_none() && !self.report.unreadable_textures.contains_key(&path) {
            *self
                .report
                .missing_textures
                .entry(path.clone())
                .or_default() += 1;
        }
        self.texture_index.insert(path, loaded);
        loaded
    }

    fn load_texture(&mut self, path: &str) -> Option<usize> {
        let bytes = self.assets.read(path).ok()??;
        let decoded = dds::Dds::parse(bytes).and_then(|dds| {
            let level = (0..dds.mip_count())
                .find(|&l| {
                    let (w, h) = dds.level_dimensions(l);
                    w.max(h) <= MAX_TEXTURE_SIZE
                })
                .unwrap_or(dds.mip_count().saturating_sub(1));
            let (w, h) = dds.level_dimensions(level);
            Ok((w, h, dds.decode_rgba(0, level)?))
        });
        match decoded {
            Ok((w, h, rgba)) => {
                self.textures
                    .push(Texture::from_rgba(w as usize, h as usize, &rgba));
                Some(self.textures.len() - 1)
            }
            Err(e) => {
                self.report
                    .unreadable_textures
                    .insert(path.to_string(), e.to_string());
                None
            }
        }
    }
}

/// Which parts of a mesh's vertex colors the game uses: (color, alpha,
/// decal). The colors are used whenever the mesh has them, whatever the
/// shader's vertex-color flag says: no-lighting shaders (the soft shadow
/// behind picture frames is nothing but black vertex colors, flag unset)
/// and lit ones alike. For lit shaders this was read from the game's own
/// shader constants (an apitrace recording of Doc Mitchell's house): its
/// walls, whose flag is off, get the "use vertex colors" toggle on, while
/// the rug, which has no colors, gets it off. Vertex alpha follows its own
/// flag. Meshes without a shader use both.
pub fn vertex_color_use(shader: Option<&nif::ShaderProperty>) -> (bool, bool, bool) {
    match shader {
        Some(s) => (
            true,
            s.shader_flags & VERTEX_ALPHA != 0,
            s.shader_flags & DECAL_FLAGS != 0,
        ),
        None => (true, true, false),
    }
}

/// Whether the game draws a mesh at all: only with one of Bethesda's
/// shader properties. Shapes without one are left in the files by the
/// exporter: 3ds Max's bone boxes (the Nevada flag's eleven), helper
/// quads (a footlocker's latch, `Footlocker01:0`, which the recording of
/// Doc Mitchell's house shows isn't drawn: its box and lid are, nothing
/// else of it), shadow and collision stand-ins. Of the 597 such shapes in
/// the game's archives (leaving out the distant-land chunks' water
/// surfaces, which are drawn on their own), 578 have no texture at all;
/// 19 name one through the older `NiTexturingProperty` alone (rubble
/// piles, a door): not checked against the game, left out like the rest
/// (a guess).
pub fn is_drawn(mesh: &nif::Mesh) -> bool {
    mesh.shader.is_some()
}

/// Shader flag (first set): "dynamic alpha", for pieces whose opacity an
/// animation changes (windows that light up at night, crops that vanish
/// when picked).
const DYNAMIC_ALPHA: u32 = 0x0008_0000;

/// How a mesh blends and alpha-tests: its `NiAlphaProperty`; without one,
/// a mesh with the dynamic alpha shader flag is blended (source alpha,
/// one minus source alpha), else it's opaque. The flag's rule is read off
/// the recording at Goodsprings: the houses' window glow cards
/// (`craftsmanwindowext.nif`, no alpha property) were drawn with
/// `D3DRS_ALPHABLENDENABLE` on, `SRCALPHA` / `INVSRCALPHA`, writing depth.
pub fn alpha_of(mesh: &nif::Mesh) -> AlphaMode {
    match mesh.alpha {
        Some(a) => AlphaMode::from_nif(a.flags, a.threshold),
        None if mesh
            .shader
            .as_ref()
            .is_some_and(|s| s.shader_flags & DYNAMIC_ALPHA != 0) =>
        {
            AlphaMode {
                blend: Some((BlendFactor::SrcAlpha, BlendFactor::InvSrcAlpha)),
                test: None,
            }
        }
        None => AlphaMode::OPAQUE,
    }
}

/// Shader flag (second set): the mesh writes its depth.
const DEPTH_WRITE: u32 = 0x0000_0001;

/// How the game sets up the graphics card for a mesh: its depth test and
/// depth write, blending and alpha test, and which sides are drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha: AlphaMode,
    pub double_sided: bool,
    /// Drawn over another surface, pulled toward the camera.
    pub decal: bool,
}

impl DrawState {
    /// The state for a mesh as its file sets it up (see
    /// [`depth_of`] for the depth buffer).
    pub fn of(mesh: &nif::Mesh) -> DrawState {
        let (depth_test, depth_write) = depth_of(mesh);
        DrawState {
            depth_test,
            depth_write,
            alpha: alpha_of(mesh),
            double_sided: mesh.double_sided,
            decal: vertex_color_use(mesh.shader.as_ref()).2,
        }
    }

    /// One line, for `nvinspect meshes`.
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        match self.alpha.blend {
            Some((src, dst)) => parts.push(format!("blended {src:?}/{dst:?}")),
            None => parts.push("opaque".into()),
        }
        if let Some((func, threshold)) = self.alpha.test {
            parts.push(format!("alpha test {func:?} {threshold}"));
        }
        parts.push(
            match (self.depth_test, self.depth_write) {
                (true, true) => "depth test+write",
                (true, false) => "depth test only",
                (false, true) => "depth write only",
                (false, false) => "no depth",
            }
            .into(),
        );
        if self.double_sided {
            parts.push("two-sided".into());
        }
        if self.decal {
            parts.push("decal".into());
        }
        parts.join(", ")
    }
}

/// Whether the game tests a mesh against the depth buffer and writes its
/// depth (test, write).
///
/// Meshes with one of Bethesda's shader properties: always tested; written
/// unless they're decals (first set 0x04000000 / 0x08000000), and, when
/// alpha-blended, only with the write flag (second set 0x1); distant-object
/// blocks (second set 0x4) always write. Confirmed against the game's
/// render states in the recordings (`D3DRS_ZENABLE`, `D3DRS_ZWRITEENABLE`
/// at every draw, matched to meshes by vertex count): in Doc Mitchell's
/// house and the Mojave Outpost barracks every blended mesh with the write
/// flag (windows, the glass bulbs, the lamps' glow cards, a gourd) was
/// drawn writing depth, the light beams (flag off) without, and all 18
/// decal draws (picture-frame shadows, shelf grime) without, though their
/// flag is on; at Goodsprings the distant-object blocks, whose flags are
/// 0x2000 / 0x4 (neither the test flag, first set 0x80000000, nor the write
/// flag), were drawn tested and writing depth, and no scene draw in any
/// recording had the test off: the test flag isn't followed. Opaque meshes
/// write whatever the flag says: the BOS water crate
/// (`nv_cratebosgeneric01.nif`, second set 0x8000 with no write flag) was
/// drawn writing depth at Goodsprings (call 152845241: the crate lying on
/// its side at −68537.9, 4365.0, scale 0.65, 3725.7 units up the view, is
/// the only match); with the flag followed it was see-through in the
/// viewer. The game's code reads the file's flags as they are
/// (`BSShaderProperty::LoadBinary`, `00ba92c0`: the two words, then
/// 0x2000 in the second also sets 0x8000) and its lit shader turns
/// `ZWRITEENABLE` off for a draw whose property lacks the flag
/// (`00bc9a50`, restored by `00bc8d90`), so something sets the flag on
/// opaque pieces after loading; what, isn't traced. New Vegas's files
/// carry no `NiZBufferProperty` (none in any archive); for meshes without
/// a shader property one is used if present, else both (Gamebryo's
/// default).
pub fn depth_of(mesh: &nif::Mesh) -> (bool, bool) {
    match (&mesh.shader, mesh.zbuffer) {
        (Some(s), _) if s.shader_flags2 & LOD_BUILDING != 0 => (true, true),
        (Some(s), _) => {
            let decal = s.shader_flags & DECAL_FLAGS != 0;
            let blended = alpha_of(mesh).blend.is_some();
            (
                true,
                !decal && (!blended || s.shader_flags2 & DEPTH_WRITE != 0),
            )
        }
        (None, Some(z)) => (z.test(), z.write()),
        (None, None) => (true, true),
    }
}

/// How much closer decals are drawn: the game sets `D3DRS_DEPTHBIAS` to
/// this (the float with bits 0xB7A7C5AC, −0.0000200000) for every decal
/// draw and back to 0 after, in all three recordings. It's added to the
/// depth the projection gives (0 at the near plane, 1 at the far one), so
/// with the game's projection `z = A − B / d` (d the distance along the
/// view, B = near × far / (far − near)) a decal at d is drawn as if at d'
/// with `1 / d' = 1 / d − bias / B`.
pub fn decal_depth_bias() -> f32 {
    f32::from_bits(0xB7A7_C5AC)
}

/// The game's projection indoors, as recorded (`D3DTS_PROJECTION` in Doc
/// Mitchell's house and the barracks: z row 1.001001, −5.005005): its `B`
/// = near × far / (far − near) for near 5 (`fNearDistance`) and far 5000,
/// what [`decal_depth_bias`] is measured against.
pub const INTERIOR_DEPTH_SCALE: f32 = 5.005_005;
/// The same outdoors, as recorded at Goodsprings by day (1.0001,
/// −35.38754: near about 35.4, far about 354,000). Where the game takes
/// that near plane from outdoors isn't traced.
pub const EXTERIOR_DEPTH_SCALE: f32 = 35.387_54;

/// One actor on its own, posed, at the origin facing north (the
/// first-person view).
pub fn actor_scene(assets: &Assets, look: &world::ActorLook) -> CellScene {
    build_scene(assets, LoadedCell::lone_actor(look.clone()))
}

/// Loads the models and textures of every object in a cell, placing each
/// model the way the game does (see [`build_scene_with`]).
pub fn build_scene(assets: &Assets, cell: LoadedCell) -> CellScene {
    build_scene_with(assets, cell, false)
}

/// Loads the models and textures of every object in a cell. The game
/// replaces a placed model's top-node transform with the placement's own;
/// `keep_root_transforms` applies it anyway, as model viewers do.
pub fn build_scene_with(
    assets: &Assets,
    cell: LoadedCell,
    keep_root_transforms: bool,
) -> CellScene {
    let mut loader = Loader {
        assets,
        keep_root_transforms,
        models: Vec::new(),
        model_index: HashMap::new(),
        textures: Vec::new(),
        texture_index: HashMap::new(),
        report: SceneReport::default(),
        poses: HashMap::new(),
        posed_parts: HashMap::new(),
    };
    let mut instances = Vec::new();
    for (i, object) in cell.objects.iter().enumerate() {
        // Trees' models are SpeedTree files, grown and drawn on their own
        // (`cellview::tree`), not `.nif`s.
        if object.base_type == esm::FourCC::new(b"TREE") {
            continue;
        }
        let whole = object.model.as_deref().and_then(|m| loader.model(m));
        if let Some(model) = whole {
            instances.push(Instance {
                object: i,
                part: None,
                model,
            });
            continue;
        }
        if !object.parts.is_empty() {
            loader.report.collections_from_parts += 1;
            for (j, part) in object.parts.iter().enumerate() {
                let model = part.model.as_deref().and_then(|m| loader.model(m));
                match model {
                    Some(model) => instances.push(Instance {
                        object: i,
                        part: Some(j),
                        model,
                    }),
                    None => note_missing(&mut loader, part.model.as_deref()),
                }
            }
        } else if object.model.is_some() {
            note_missing(&mut loader, object.model.as_deref());
        }
    }
    // People and creatures, posed, are drawn like any other object: they
    // join the cell's objects after its own.
    let mut cell = cell;
    for actor in cell.actors.clone() {
        let Some(look) = &actor.actor else {
            continue;
        };
        if let Some(model) = loader.actor_model(look) {
            let mut placed = actor.clone();
            placed.scale *= look.scale;
            cell.objects.push(placed);
            instances.push(Instance {
                object: cell.objects.len() - 1,
                part: None,
                model,
            });
            loader.report.actors_drawn += 1;
        }
    }
    CellScene {
        cell,
        models: loader.models,
        textures: loader.textures,
        instances,
        report: loader.report,
    }
}

/// Second shader flag set: a distant-object (LOD building) mesh.
const LOD_BUILDING: u32 = 0x4;
/// Gamebryo's alpha test function "greater" (`NiAlphaProperty` flag bits
/// 10–12), the test the game ran on distant objects.
const LOD_BUILDING_ALPHA_GREATER: u16 = 4;

/// A distant-object block (`nif::segments`: one merged mesh per 4 × 4
/// cells) as a scene of one model per segment, each placed by a "reference"
/// numbered after its segment (form ID = segment number) so its part can be
/// hidden over a loaded cell. The block's vertices are already in the world
/// (its top node isn't moved). Textures named with a leading `Data\` are
/// found under `textures\` as usual. `None` when it can't be read.
pub fn lod_block_scene(assets: &Assets, path: &str) -> Option<CellScene> {
    let bytes = assets.read(path).ok()??;
    let nif = nif::Nif::parse(bytes).ok()?;
    let scene = nif.scene().ok()?;
    let mut loader = Loader {
        assets,
        keep_root_transforms: true,
        models: Vec::new(),
        model_index: HashMap::new(),
        textures: Vec::new(),
        texture_index: HashMap::new(),
        report: SceneReport::default(),
        poses: HashMap::new(),
        posed_parts: HashMap::new(),
    };
    let mut cell = LoadedCell::actors_only(Vec::new());
    let mut instances = Vec::new();
    for mesh in scene.meshes {
        let mut mesh = mesh;
        for t in &mut mesh.textures {
            let lower = t.replace('/', "\\").to_ascii_lowercase();
            if let Some(rest) = lower.strip_prefix("data\\") {
                *t = rest.to_string();
            }
        }
        // Shader flag 0x4 of the second set marks LOD buildings (in the
        // community's flag lists). Their shared atlas (`<world>.buildings.dds`,
        // DXT3) is 14% fully transparent: trees and fences are cut-outs, so
        // they're alpha-tested though the mesh has no alpha property: the
        // Goodsprings recording drew them (`SLS2000`, right after the sky)
        // with alpha testing on, `ALPHAFUNC` greater, `ALPHAREF` 128.
        let lod_building = mesh
            .shader
            .as_ref()
            .is_some_and(|s| s.shader_flags2 & LOD_BUILDING != 0);
        if lod_building && mesh.alpha.is_none() {
            mesh.alpha = Some(nif::AlphaProperty {
                flags: 0x0200 | (LOD_BUILDING_ALPHA_GREATER << 10),
                threshold: 128,
            });
        }
        let segments = nif.segments(mesh.block);
        let pieces: Vec<(u32, Vec<[u16; 3]>)> = if segments.is_empty() {
            vec![(u32::MAX, mesh.triangles.clone())]
        } else {
            segments
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let tris = mesh
                        .triangles
                        .iter()
                        .skip(s.first_triangle as usize)
                        .take(s.triangles as usize)
                        .copied()
                        .collect();
                    (i as u32, tris)
                })
                .collect()
        };
        for (segment, triangles) in pieces {
            if triangles.is_empty() {
                continue;
            }
            let piece = nif::Mesh {
                triangles,
                ..mesh.clone()
            };
            let Some(model_mesh) = loader.mesh(&piece) else {
                continue;
            };
            loader.models.push(Model {
                path: path.to_string(),
                meshes: vec![model_mesh],
                collision: Vec::new(),
                root_transform: None,
                skeleton: None,
                sequences: Default::default(),
                particles: None,
                bsx_flags: 0,
            });
            cell.objects.push(world::Placement {
                form_id: esm::FormId(segment),
                record_type: esm::FourCC::new(b"REFR"),
                editor_id: None,
                base: esm::FormId(0),
                base_type: esm::FourCC::new(b"STAT"),
                base_editor_id: None,
                position: [0.0; 3],
                rotation: [0.0; 3],
                scale: 1.0,
                model: Some(path.to_string()),
                parts: Vec::new(),
                light: None,
                radius: None,
                teleport: None,
                emittance: None,
                flags: 0,
                enable_parent: None,
                plugin: String::new(),
                actor: None,
                primitive: None,
                open_by_default: false,
            });
            instances.push(Instance {
                object: cell.objects.len() - 1,
                part: None,
                model: loader.models.len() - 1,
            });
        }
    }
    cell.info.interior = false;
    Some(CellScene {
        cell,
        models: loader.models,
        textures: loader.textures,
        instances,
        report: loader.report,
    })
}

/// A collision part's box in model space, placed: its centre, the placed
/// model axes and half sizes (`world::ai::doors::DoorBox`).
fn door_box(part: &nif::CollisionPart, place: &Transform) -> Option<world::ai::doors::DoorBox> {
    let points: Vec<[f32; 3]> = match &part.shape {
        nif::CollisionShape::Triangles { vertices, .. }
        | nif::CollisionShape::Convex { vertices, .. } => vertices.clone(),
        nif::CollisionShape::Sphere { center, radius } => vec![
            [center[0] - radius, center[1] - radius, center[2] - radius],
            [center[0] + radius, center[1] + radius, center[2] + radius],
        ],
        nif::CollisionShape::Capsule { a, b, radius } => {
            let lo = [0, 1, 2].map(|k| a[k].min(b[k]) - radius);
            let hi = [0, 1, 2].map(|k| a[k].max(b[k]) + radius);
            vec![lo, hi]
        }
    };
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for p in &points {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k] - part.shell);
            hi[k] = hi[k].max(p[k] + part.shell);
        }
    }
    if lo[0] > hi[0] {
        return None;
    }
    let middle = [0, 1, 2].map(|k| 0.5 * (lo[k] + hi[k]));
    let origin = place.apply_point([0.0; 3]);
    let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| {
        let p = place.apply_point(e);
        let d = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-6);
        d.map(|v| v / l)
    });
    Some(world::ai::doors::DoorBox {
        center: place.apply_point(middle),
        axes,
        half: [0, 1, 2].map(|k| 0.5 * (hi[k] - lo[k]) * place.scale),
    })
}

/// A door that opens where it stands: a `DOOR` that isn't a load door.
pub fn is_opening_door(object: &world::Placement) -> bool {
    object.base_type == esm::FourCC::new(b"DOOR") && object.teleport.is_none()
}

/// A placed door that opens where it stands (`world::doors`): its
/// model's sequences, and the node chains its leaf's collision hangs on,
/// for moving the collider's owned triangles with the swing.
#[derive(Debug, Clone)]
pub struct SwingDoor {
    pub reference: esm::FormId,
    pub base: esm::FormId,
    /// The base's editor ID, for messages.
    pub name: String,
    pub open_by_default: bool,
    /// Model space to world space.
    pub place: Transform,
    /// Every sequence of the model (`Open`, `Close`, …).
    pub sequences: std::sync::Arc<Vec<nif::Sequence>>,
    /// Each keyframed collision part's nodes, top down (`nif::CollisionPart::nodes`).
    pub leaves: Vec<Vec<(String, Transform)>>,
    /// Its collision parts' boxes as placed (closed), in its placed axes:
    /// the navmesh obstacles a closed door marks (`world::ai::doors`).
    pub boxes: Vec<world::ai::doors::DoorBox>,
}

impl SwingDoor {
    /// How long the model's `Open` and `Close` sequences play, when it
    /// has both.
    pub fn lengths(&self) -> Option<world::doors::Lengths> {
        let length = |name: &str| {
            self.sequences
                .iter()
                .find(|s| s.name.eq_ignore_ascii_case(name))
                .map(|s| (s.stop - s.start).max(0.0))
        };
        Some(world::doors::Lengths {
            open: length("Open")?,
            close: length("Close")?,
        })
    }

    /// The sequence of a state (`Open` when opening) at a moment of its
    /// own clock (seconds from its start, clamped to its length).
    pub fn sequence_at(&self, opening: bool, seconds: f32) -> Option<(&nif::Sequence, f32)> {
        let name = if opening { "Open" } else { "Close" };
        let s = self
            .sequences
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))?;
        Some((s, s.start + seconds.clamp(0.0, (s.stop - s.start).max(0.0))))
    }

    /// Where a sequence moment puts each leaf, as a rigid move in world
    /// space from where the collider holds it (the model's rest pose): one
    /// rotation and translation per leaf, for
    /// [`physics::Collider::move_owner`]. The whole door gets the first
    /// leaf's move (a door has one swinging part; `nvinspect collision`).
    pub fn leaf_move(&self, layer: Option<(&nif::Sequence, f32)>) -> Option<([[f32; 3]; 3], Vec3)> {
        let chain = self.leaves.first()?;
        let layers: Vec<(&nif::Sequence, f32)> = layer.into_iter().collect();
        let rest = nif::posed_chain(chain, &[]);
        let posed = nif::posed_chain(chain, &layers);
        // world = place × posed × rest⁻¹ × place⁻¹ × rest_world.
        let m = self
            .place
            .then_child(&posed)
            .then_child(&rest.inverse())
            .then_child(&self.place.inverse());
        Some((m.rotation, m.translation))
    }
}

impl CellScene {
    /// The cell's doors that open where they stand, with what moves them.
    pub fn swing_doors(&self, convention: RotationConvention) -> Vec<SwingDoor> {
        let mut out = Vec::new();
        for instance in &self.instances {
            let object = &self.cell.objects[instance.object];
            if !is_opening_door(object) || instance.part.is_some() {
                continue;
            }
            let model = &self.models[instance.model];
            let sequences = model.sequences.clone();
            let leaves: Vec<Vec<(String, Transform)>> = model
                .collision
                .iter()
                .filter(|p| p.keyframed && nif::collision::layers::blocks_walking(p.layer))
                .map(|p| p.nodes.clone())
                .collect();
            let place = self.transform(instance, convention);
            let boxes = model
                .collision
                .iter()
                .filter(|p| nif::collision::layers::blocks_walking(p.layer))
                .filter_map(|p| door_box(p, &place))
                .collect();
            out.push(SwingDoor {
                reference: object.form_id,
                base: object.base,
                name: object
                    .base_editor_id
                    .clone()
                    .unwrap_or_else(|| object.base.to_string()),
                open_by_default: object.open_by_default,
                place,
                sequences,
                leaves,
                boxes,
            });
        }
        out
    }
}

/// One piece of a model's collision, placed, into the collider, with its
/// body's friction and restitution (for rigid bodies resting on it) and
/// its Havok layer (which casts meet it, `physics::layers`).
fn add_part(
    collider: &mut physics::Collider,
    part: &nif::CollisionPart,
    place: &Transform,
    (owner, reference): (u32, u32),
) {
    let at = |v: &[f32; 3]| place.apply_point(*v);
    let surface = (part.body != nif::RigidBodyInfo::default()).then_some(physics::Surface {
        friction: part.body.friction,
        restitution: part.body.restitution,
    });
    let mut add = |v: &[[f32; 3]], t: &[[u32; 3]]| {
        collider.add_placed(
            v,
            t,
            (part.shell, owner, part.material),
            surface,
            part.layer,
            reference,
        );
    };
    match &part.shape {
        nif::CollisionShape::Triangles {
            vertices,
            triangles,
        } => {
            let v: Vec<[f32; 3]> = vertices.iter().map(at).collect();
            add(&v, triangles);
        }
        nif::CollisionShape::Convex { vertices, planes } => {
            let tris = physics::shapes::hull(vertices, planes);
            let v: Vec<[f32; 3]> = vertices.iter().map(at).collect();
            add(&v, &tris);
        }
        nif::CollisionShape::Sphere { center, radius } => {
            let (v, t) = physics::shapes::sphere(at(center), radius * place.scale);
            add(&v, &t);
        }
        nif::CollisionShape::Capsule { a, b, radius } => {
            let (v, t) = physics::shapes::capsule(at(a), at(b), radius * place.scale);
            add(&v, &t);
        }
    }
}

/// A placed reference whose model carries a body Havok moves (clutter,
/// weapons, props: `nif::RigidBodyInfo`, motion systems other than
/// keyframed and fixed), for [`physics::rigid::RigidWorld`]: its body in the
/// model's space (the reference's scale applied to the shapes, the centre
/// and the inertia; the mass as stored: how the game's scaled clone,
/// `00c8f2a0`, treats mass and inertia isn't traced) and where it's placed.
#[derive(Debug, Clone)]
pub struct DynamicBody {
    pub reference: esm::FormId,
    /// The base's editor ID, for messages.
    pub name: String,
    pub setup: physics::rigid::RigidSetup,
    /// Each shape's Havok material (what a shot striking it sounds like).
    pub materials: Vec<u32>,
    pub pose: physics::rigid::Pose,
}

/// The one body a model's moving parts belong to (the `bhkRigidBody`
/// block), when they all belong to one: a model with several moving
/// bodies (joined by constraints) isn't simulated here and stays solid
/// where it's placed. So does a body held by a constraint (a hinge to a
/// fixed bracket, a chain's links): simulated free it would fall off what
/// holds it, and constraints aren't simulated here.
fn moving_body(collision: &[nif::CollisionPart]) -> Option<usize> {
    let mut block = None;
    for p in collision.iter().filter(|p| moving_part(p)) {
        if p.body.constraints > 0 {
            return None;
        }
        match block {
            None => block = Some(p.body.block),
            Some(b) if b != p.body.block => return None,
            _ => {}
        }
    }
    block
}

/// Whether the game fixes a placed reference's bodies as its model gets
/// Havok: a static's or a static collection's (base form types 0x20,
/// 0x21) are set to the fixed motion (`TESObjectREFR::InitHavok`, Xbox
/// PDB, `005768b0` → `00c6a350(3D, 5, …)`), which changes a model's bodies
/// only when its `BSXFlags` has Havok (bit 1). The burnt fence pickets
/// south of Goodsprings' square (`NVFencePickBurntBroken01`, a `STAT`
/// with a moving clutter body and that flag) stay put so.
// Translated from 005768b0 (decompiled, FalloutNV.exe 1.4.0.525)
fn fixed_by_game(object: &world::Placement, model: &Model) -> bool {
    let kind = object.base_type;
    (kind == esm::FourCC::new(b"STAT") || kind == esm::FourCC::new(b"SCOL"))
        && model.bsx_flags & nif::collision::BSX_HAVOK != 0
}

/// The body Havok moves for a placed reference: its model's one moving
/// body ([`moving_body`]) unless the game fixes it ([`fixed_by_game`]).
fn simulated_body(object: &world::Placement, model: &Model) -> Option<usize> {
    if fixed_by_game(object, model) {
        return None;
    }
    moving_body(&model.collision)
}

/// Placed references whose models have moving bodies that aren't
/// simulated here ([`moving_body`]: several bodies, or constraints): their
/// editor IDs, for the log.
impl CellScene {
    pub fn unsimulated_bodies(&self) -> Vec<(esm::FormId, String)> {
        let mut out = Vec::new();
        for instance in &self.instances {
            let object = &self.cell.objects[instance.object];
            if instance.part.is_some() || is_opening_door(object) {
                continue;
            }
            let model = &self.models[instance.model];
            let moving = model.collision.iter().any(moving_part);
            if moving && !fixed_by_game(object, model) && moving_body(&model.collision).is_none() {
                out.push((
                    object.form_id,
                    object
                        .base_editor_id
                        .clone()
                        .unwrap_or_else(|| object.base.to_string()),
                ));
            }
        }
        out
    }
}

fn moving_part(p: &nif::CollisionPart) -> bool {
    p.dynamic && physics::impulses::moves(p.body.motion) && p.body.mass > 0.0
}

/// A collision part as a rigid body's shape, scaled by `s`.
fn body_shape(part: &nif::CollisionPart, s: f32) -> physics::rigid::Shape {
    let sc = |v: &[f32; 3]| v.map(|x| x * s);
    match &part.shape {
        nif::CollisionShape::Convex { vertices, planes } => {
            let planes: Vec<[f32; 4]> = planes
                .iter()
                .map(|p| [p[0], p[1], p[2], p[3] * s])
                .collect();
            physics::rigid::Shape::hull(vertices.iter().map(sc).collect(), &planes, part.shell)
        }
        nif::CollisionShape::Sphere { center, radius } => physics::rigid::Shape::Sphere {
            center: sc(center),
            radius: radius * s,
        },
        nif::CollisionShape::Capsule { a, b, radius } => physics::rigid::Shape::Capsule {
            a: sc(a),
            b: sc(b),
            radius: radius * s,
        },
        nif::CollisionShape::Triangles {
            vertices,
            triangles,
        } => physics::rigid::Shape::Mesh {
            vertices: vertices.iter().map(sc).collect(),
            triangles: triangles.clone(),
            shell: part.shell,
        },
    }
}

impl CellScene {
    /// The placed references Havok would move: each with its one moving
    /// body ([`DynamicBody`]). Doors that open where they stand and
    /// static collections' pieces aren't among them.
    pub fn dynamic_bodies(&self, convention: RotationConvention) -> Vec<DynamicBody> {
        let mut out = Vec::new();
        for instance in &self.instances {
            let object = &self.cell.objects[instance.object];
            if instance.part.is_some() || is_opening_door(object) {
                continue;
            }
            let model = &self.models[instance.model];
            let Some(block) = simulated_body(object, model) else {
                continue;
            };
            let parts: Vec<&nif::CollisionPart> = model
                .collision
                .iter()
                .filter(|p| moving_part(p) && p.body.block == block)
                .collect();
            let Some(first) = parts.first() else {
                continue;
            };
            let place = self.transform(instance, convention);
            let s = place.scale;
            let info = first.body;
            let setup = physics::rigid::RigidSetup {
                reference: object.form_id.0,
                layer: first.layer,
                mass: info.mass,
                center: info.center.map(|x| x * s),
                inertia: info.inertia.map(|row| row.map(|x| x * s * s)),
                linear_damping: info.linear_damping,
                angular_damping: info.angular_damping,
                friction: info.friction,
                restitution: info.restitution,
                max_linear_speed: info.max_linear_speed,
                max_angular_speed: info.max_angular_speed,
                motion: info.motion,
                quality: info.quality,
                wind: info.body_flags & nif::collision::BODY_WIND != 0,
                shapes: parts.iter().map(|p| body_shape(p, s)).collect(),
            };
            out.push(DynamicBody {
                reference: object.form_id,
                name: object
                    .base_editor_id
                    .clone()
                    .unwrap_or_else(|| object.base.to_string()),
                setup,
                materials: parts.iter().map(|p| p.material).collect(),
                pose: (place.rotation, place.translation),
            });
        }
        out
    }
}

/// The layer a collision marker's primitive is on without an `XTRI`: 3,
/// transparent (`0056eab0`).
const COLLISION_MARKER_LAYER: u8 = 3;

/// A collision marker's primitive as the game builds it (`0056eab0`,
/// reached from the reference's 3D setup for the `CollisionMarker` base,
/// `00576990`): a fixed body on the `XTRI` layer (else transparent) at the
/// reference's position and rotation, its scale set to 1 first
/// (`00567490`); a box of the primitive's half sizes, a sphere of radius
/// x, or a plane as a box 0.01 deep (`[01013ea4]`); boxes with Havok's
/// default box radius, 0.1 (`[010c72bc]`). Activators' primitives are
/// trigger phantoms (`0056d7e0`) and acoustic spaces' sound volumes
/// (`0056f140`): neither is solid.
fn add_collision_marker(
    collider: &mut physics::Collider,
    marker: &world::Placement,
    convention: RotationConvention,
) {
    if marker.base != world::COLLISION_MARKER {
        return;
    }
    let Some(primitive) = marker.primitive else {
        return;
    };
    let layer = primitive
        .layer
        .map_or(COLLISION_MARKER_LAYER, |l| l.min(255) as u8);
    if !nif::collision::layers::blocks_walking(layer) {
        return;
    }
    let place = convention.transform(marker.position, marker.rotation, 1.0);
    let shell = 0.1 * nif::collision::HAVOK_SCALE;
    let [x, y, z] = primitive.half;
    let half = match primitive.shape {
        1 => [x, y, z],
        3 => [x, 0.01, z],
        2 => {
            let (v, t) = physics::shapes::sphere(place.apply_point([0.0; 3]), x);
            collider.add_solid(&v, &t, 0.0, 0);
            return;
        }
        _ => return,
    };
    let part = nif::CollisionPart {
        layer,
        dynamic: false,
        keyframed: false,
        node: 0,
        nodes: Vec::new(),
        flags: 0,
        shell,
        shape: nif::collision::box_shape(half, &Transform::IDENTITY),
        // The marker body's Havok material isn't traced: none given.
        material: physics::NO_MATERIAL,
        body: Default::default(),
    };
    add_part(collider, &part, &place, (0, marker.form_id.0));
}

fn note_missing(loader: &mut Loader, model: Option<&str>) {
    if let Some(m) = model {
        let path = assets::mesh_path(m);
        if !loader.report.unreadable_models.contains_key(&path) {
            *loader.report.missing_models.entry(path).or_default() += 1;
        }
    }
}

impl CellScene {
    /// Model space to world space for an instance.
    pub fn transform(&self, instance: &Instance, convention: RotationConvention) -> Transform {
        let object = &self.cell.objects[instance.object];
        let placed = convention.transform(object.position, object.rotation, object.scale);
        match instance.part {
            Some(j) => {
                let part = &object.parts[j];
                placed.then_child(&convention.transform(part.position, part.rotation, part.scale))
            }
            None => placed,
        }
    }

    /// The self-lit color a placed copy of a mesh uses. Meshes marked for
    /// external emittance take it from the object's Emittance setting (a
    /// light's color, or a region's daytime sunlight), or the cell's
    /// default when it names nothing (`world::default_emittance`: the "on"
    /// wall lamps in Doc Mitchell's house), times the mesh's multiplier.
    /// The mesh's own color is used only when neither gives one.
    pub fn emissive_for(&self, instance: &Instance, mesh: &ModelMesh) -> Vec3 {
        if !mesh.external_emittance {
            return mesh.emissive;
        }
        match self
            .cell
            .emittance_color(&self.cell.objects[instance.object])
        {
            Some(color) => color.map(|c| c * mesh.emissive_mult),
            None => mesh.emissive,
        }
    }

    /// The cell's solid surfaces: every placed model's collision (see
    /// `nif::collision`) on the layers that stop a walking character (the
    /// game's layer matrix, `nif::collision::layers::blocks_walking`),
    /// moved to where the model is placed (its scale included: the game
    /// clones a scaled reference's bodies scaled), and the collision
    /// markers' boxes. Hulls, spheres and capsules become triangles.
    ///
    /// A door that isn't a load door owns its leaf (its keyframed parts,
    /// which the door's animation swings) under its form ID, so it can be
    /// moved where the animation puts it ([`physics::Collider::move_owner`]
    /// with [`SwingDoor::leaf_move`]); it's added where the model files it
    /// (closed), whatever the door's state. The frame stays put. Moving
    /// clutter with one body is left out: it's simulated
    /// ([`Self::dynamic_bodies`], `physics::rigid`), and the simulation
    /// adds its triangles under the reference's form ID and moves them
    /// with it. The game's character runs into clutter, weapons and props
    /// whatever they weigh (the contact callback `00c711d0` only stops
    /// lighter ones than `fMoveLimitMass` from being pushed by the
    /// contact's own speed).
    pub fn collider(&self, convention: RotationConvention) -> physics::Collider {
        let mut collider = physics::Collider::new();
        for instance in &self.instances {
            let model = &self.models[instance.model];
            if model.collision.is_empty() {
                continue;
            }
            let object = &self.cell.objects[instance.object];
            let place = self.transform(instance, convention);
            let door = is_opening_door(object);
            // A body Havok moves isn't here: whoever simulates it
            // ([`Self::dynamic_bodies`]) adds and moves its triangles.
            let moving = (!door && instance.part.is_none())
                .then(|| simulated_body(object, model))
                .flatten();
            for part in &model.collision {
                if !nif::collision::layers::blocks_walking(part.layer) {
                    continue;
                }
                if moving.is_some_and(|b| moving_part(part) && part.body.block == b) {
                    continue;
                }
                let owner = if door && part.keyframed {
                    object.form_id.0
                } else {
                    0
                };
                add_part(&mut collider, part, &place, (owner, object.form_id.0));
            }
        }
        for marker in &self.cell.markers {
            add_collision_marker(&mut collider, marker, convention);
        }
        collider
    }

    /// Whether an instance is turned about more than one axis, where the
    /// rotation conventions disagree.
    pub fn is_tilted(&self, instance: &Instance) -> bool {
        let object = &self.cell.objects[instance.object];
        is_tilted(object.rotation)
            || instance
                .part
                .is_some_and(|j| is_tilted(object.parts[j].rotation))
    }

    /// The world-space box around each object's geometry (by index into
    /// [`LoadedCell::objects`]), and whether everything it draws is a light
    /// effect.
    pub fn object_bounds(&self, convention: RotationConvention) -> Vec<Option<(Vec3, Vec3, bool)>> {
        let mut out: Vec<Option<(Vec3, Vec3, bool)>> = vec![None; self.cell.objects.len()];
        self.for_each_mesh(convention, |instance, mesh, positions, _| {
            for p in positions {
                let entry = out[instance.object].get_or_insert((
                    [f32::INFINITY; 3],
                    [f32::NEG_INFINITY; 3],
                    true,
                ));
                entry.0 = [0, 1, 2].map(|k| entry.0[k].min(p[k]));
                entry.1 = [0, 1, 2].map(|k| entry.1[k].max(p[k]));
            }
            if let Some(entry) = out[instance.object].as_mut() {
                entry.2 &= mesh.effect;
            }
        });
        out
    }

    pub fn triangle_count(&self) -> usize {
        self.instances
            .iter()
            .map(|i| {
                self.models[i.model]
                    .meshes
                    .iter()
                    .map(|m| m.triangles.len())
                    .sum::<usize>()
            })
            .sum()
    }

    /// Calls `f` with each drawable mesh in world space.
    fn for_each_mesh(
        &self,
        convention: RotationConvention,
        mut f: impl FnMut(&Instance, &ModelMesh, &[Vec3], &[Vec3]),
    ) {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        for instance in &self.instances {
            let t = self.transform(instance, convention);
            for mesh in &self.models[instance.model].meshes {
                positions.clear();
                positions.extend(mesh.positions.iter().map(|&p| t.apply_point(p)));
                normals.clear();
                normals.extend(mesh.normals.iter().map(|&n| t.apply_direction(n)));
                f(instance, mesh, &positions, &normals);
            }
        }
    }

    /// Heights with the most upward-facing surface area: the cell's floors.
    /// Returns (height, area) pairs, largest area first.
    pub fn floor_levels(&self, convention: RotationConvention) -> Vec<(f32, f32)> {
        const BIN: f32 = 8.0;
        let mut bins: HashMap<i64, f32> = HashMap::new();
        self.for_each_mesh(convention, |_, mesh, positions, normals| {
            if normals.len() != positions.len() || mesh.alpha.blend.is_some() {
                return;
            }
            for t in &mesh.triangles {
                let [a, b, c] = t.map(usize::from);
                if a.max(b).max(c) >= positions.len() {
                    continue;
                }
                let up = normals[a][2] + normals[b][2] + normals[c][2];
                if up < 3.0 * 0.85 {
                    continue;
                }
                let area = 0.5
                    * raster::length(raster::cross(
                        raster::sub(positions[b], positions[a]),
                        raster::sub(positions[c], positions[a]),
                    ));
                let z = (positions[a][2] + positions[b][2] + positions[c][2]) / 3.0;
                *bins.entry((z / BIN).round() as i64).or_default() += area;
            }
        });
        let mut levels: Vec<(f32, f32)> = bins
            .into_iter()
            .map(|(bin, area)| (bin as f32 * BIN, area))
            .collect();
        levels.sort_by(|a, b| b.1.total_cmp(&a.1));
        // Fold small neighbouring bins into the bigger level they belong to.
        let mut merged: Vec<(f32, f32)> = Vec::new();
        for (z, area) in levels {
            match merged.iter_mut().find(|(mz, _)| (mz - z).abs() <= 32.0) {
                Some(level) => level.1 += area,
                None => merged.push((z, area)),
            }
        }
        merged
    }

    /// The horizontal extent of the geometry at or below `max_z`, ignoring
    /// the outermost half percent of vertices (stray far-off pieces).
    pub fn plan_bounds(
        &self,
        convention: RotationConvention,
        max_z: f32,
    ) -> Option<([f32; 2], [f32; 2])> {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        self.for_each_mesh(convention, |_, mesh, positions, _| {
            if mesh.effect {
                return;
            }
            for p in positions.iter().filter(|p| p[2] <= max_z) {
                xs.push(p[0]);
                ys.push(p[1]);
            }
        });
        if xs.is_empty() {
            return None;
        }
        let pick = |v: &mut Vec<f32>| {
            v.sort_by(f32::total_cmp);
            let cut = v.len() / 200;
            (v[cut], v[v.len() - 1 - cut])
        };
        let (x0, x1) = pick(&mut xs);
        let (y0, y1) = pick(&mut ys);
        Some(([x0, y0], [x1, y1]))
    }
}

/// A short description of a model's top-node transform, such as
/// "turned 90° anticlockwise, moved 0,-12,0".
pub fn describe_transform(t: &Transform) -> String {
    let r = &t.rotation;
    let mut parts = Vec::new();
    if (r[2][2] - 1.0).abs() < 1e-3 {
        let angle = r[1][0].atan2(r[0][0]).to_degrees();
        if angle.abs() > 0.05 {
            let way = if angle > 0.0 {
                "anticlockwise"
            } else {
                "clockwise"
            };
            parts.push(format!("turned {:.0}° {way}", angle.abs()));
        }
    } else {
        parts.push("tilted".to_string());
    }
    if t.translation.iter().any(|v| v.abs() >= 0.5) {
        let [x, y, z] = t.translation;
        parts.push(format!("moved {x:.0},{y:.0},{z:.0}"));
    }
    if (t.scale - 1.0).abs() > 1e-3 {
        parts.push(format!("scaled {:.2}", t.scale));
    }
    if parts.is_empty() {
        "no visible change".to_string()
    } else {
        parts.join(", ")
    }
}

/// How to light a render.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightingMode {
    /// The cell's ambient and directional light plus its light sources.
    Cell { brightness: f32 },
    /// Even light from above, for reading a layout.
    Neutral,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    pub convention: RotationConvention,
    pub lighting: LightingMode,
    /// Supersampling factor (renders this many times larger, then shrinks).
    pub supersample: usize,
    pub cull: bool,
    /// Tint objects turned about more than one axis.
    pub mark_tilted: bool,
    /// Draw light effects (glows, beams, haze). Plans leave them out.
    pub effects: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            convention: RotationConvention::DEFAULT,
            lighting: LightingMode::Cell { brightness: 1.0 },
            supersample: 2,
            cull: true,
            mark_tilted: false,
            effects: true,
        }
    }
}

/// Light for a render, from the cell or neutral.
pub fn lighting_for(cell: &LoadedCell, mode: LightingMode) -> Lighting {
    match mode {
        LightingMode::Neutral => Lighting {
            ambient: [0.5; 3],
            directional: Some((normalize([-0.35, 0.45, 1.0]), [0.55; 3])),
            points: Vec::new(),
        },
        LightingMode::Cell { brightness } => {
            let rgb = |c: [u8; 3]| c.map(|v| f32::from(v) / 255.0 * brightness);
            // The ambient is used as it is: the game's final pass
            // (`ISHDRBLENDINSHADERCIN`) has no curve that would lift dark
            // corners. For the directional light's direction see
            // `world::Lighting::toward_directional`.
            let (ambient, directional) = match &cell.info.lighting {
                Some(l) => (
                    rgb(l.ambient),
                    Some((l.toward_directional(), rgb(l.directional))),
                ),
                None => ([0.3 * brightness; 3], None),
            };
            let points = cell
                .lights()
                .filter(|(_, l)| !l.is_off_by_default() && !l.is_negative())
                .filter_map(|(p, _)| cell.placed_light(p).map(|l| (p, l)))
                .filter(|(_, l)| l.radius > 0.0)
                .map(|(p, l)| PointLight {
                    position: p.position,
                    color: l.color.map(|c| c * brightness * l.fade.clamp(0.0, 4.0)),
                    radius: l.radius,
                })
                .collect();
            Lighting {
                ambient,
                directional,
                points,
            }
        }
    }
}

/// A finished image and what went into it.
pub struct Rendered {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
    pub stats: raster::Stats,
}

/// Draws the scene from a camera. With `cut`, everything above that height
/// is removed and the cut edges are outlined, like an architect's plan.
pub fn render(
    scene: &CellScene,
    camera: Camera,
    width: usize,
    height: usize,
    options: &RenderOptions,
    cut: Option<f32>,
    overlay: impl FnOnce(&mut Renderer, f32),
) -> Rendered {
    let ss = options.supersample.max(1);
    let background = if cut.is_some() {
        [0.08, 0.09, 0.11]
    } else {
        [0.0; 3]
    };
    let mut r = Renderer::new(width * ss, height * ss, background, camera);
    r.cull = options.cull;
    if let Some(z) = cut {
        r.clip_planes.push(Plane::below(z));
    }
    r.lighting = lighting_for(&scene.cell, options.lighting);
    let mut sections: Vec<(Vec3, Vec3)> = Vec::new();

    scene.for_each_mesh(options.convention, |instance, mesh, positions, normals| {
        if mesh.effect && !options.effects {
            return;
        }
        let material = Material {
            texture: mesh.texture.map(|t| &scene.textures[t]),
            // Untextured lit surfaces get a neutral grey; untextured unlit
            // ones (shadows, glows) take their color from the vertices.
            color: if mesh.texture.is_some() || !mesh.lit {
                [1.0, 1.0, 1.0, mesh.opacity]
            } else {
                [0.6, 0.6, 0.6, mesh.opacity]
            },
            alpha: mesh.alpha,
            double_sided: mesh.double_sided,
            lit: mesh.lit,
            emissive: scene.emissive_for(instance, mesh),
            glow: mesh.glow.map(|t| &scene.textures[t]),
            decal: mesh.decal,
            tint: (options.mark_tilted && scene.is_tilted(instance)).then_some([1.0, 0.1, 0.8]),
            falloff: mesh.falloff,
            depth_test: mesh.depth_test,
            depth_write: mesh.depth_write,
        };
        r.draw(
            &MeshInput {
                positions,
                normals,
                uvs: &mesh.uvs,
                colors: &mesh.colors,
                triangles: &mesh.triangles,
            },
            material,
        );
        if let Some(z) = cut {
            if mesh.alpha.blend.is_none() {
                for t in &mesh.triangles {
                    let [a, b, c] = t.map(usize::from);
                    if a.max(b).max(c) < positions.len() {
                        if let Some(s) = plane_segment(
                            [positions[a], positions[b], positions[c]],
                            Plane::below(z),
                        ) {
                            sections.push(s);
                        }
                    }
                }
            }
        }
    });
    r.finish();
    for (a, b) in sections {
        r.world_segment(a, b, 2.0 * ss as f32, [0.95, 0.95, 0.95]);
    }
    overlay(&mut r, ss as f32);
    let stats = r.stats;
    let (width, height, rgba) = r.to_rgba8(ss);
    Rendered {
        width,
        height,
        rgba,
        stats,
    }
}

/// A top-down camera framing an area, for an image `width` × `height`.
pub fn plan_camera(
    min: [f32; 2],
    max: [f32; 2],
    above: f32,
    width: usize,
    height: usize,
) -> Camera {
    let center = [(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5];
    let aspect = width as f32 / height as f32;
    let half_w = ((max[0] - min[0]) * 0.5).max((max[1] - min[1]) * 0.5 * aspect) * 1.05 + 32.0;
    Camera::top_down(center, above, half_w)
}

/// Picks a plan image size with the area's proportions, at most `longest`
/// pixels on its longer side.
pub fn plan_size(min: [f32; 2], max: [f32; 2], longest: usize) -> (usize, usize) {
    let (w, h) = ((max[0] - min[0]).max(1.0), (max[1] - min[1]).max(1.0));
    let longest = longest.max(16) as f32;
    if w >= h {
        (
            longest as usize,
            ((longest * h / w).round() as usize).max(16),
        )
    } else {
        (
            ((longest * w / h).round() as usize).max(16),
            longest as usize,
        )
    }
}

/// Markers drawn on a plan: where the view camera stands and what it sees,
/// lights, people, and a 256-unit scale bar.
pub fn draw_plan_markers(r: &mut Renderer, ss: f32, scene: &CellScene, view: Option<&Camera>) {
    let (w, h) = (r.width, r.height);
    let camera = r.camera;
    for (placement, _) in scene.cell.lights() {
        if let Some((x, y)) = camera.project(placement.position, w, h) {
            r.disc(x, y, 5.0 * ss, [1.0, 0.7, 0.2]);
        }
    }
    for actor in &scene.cell.actors {
        if let Some((x, y)) = camera.project(actor.position, w, h) {
            r.disc(x, y, 4.0 * ss, [0.9, 0.2, 0.9]);
        }
    }
    for arrival in &scene.cell.arrivals {
        let heading = arrival.rotation[2];
        let tip = add(
            arrival.position,
            [heading.sin() * 48.0, heading.cos() * 48.0, 0.0],
        );
        r.world_segment(arrival.position, tip, 2.0 * ss, [1.0, 0.9, 0.1]);
        if let Some((x, y)) = camera.project(arrival.position, w, h) {
            r.disc(x, y, 4.0 * ss, [1.0, 0.9, 0.1]);
        }
    }
    if let Some(view) = view {
        if let raster::Projection::Perspective { fov_x } = view.projection {
            let flat = normalize([view.forward[0], view.forward[1], 0.0]);
            let heading = flat[0].atan2(flat[1]);
            for side in [-0.5f32, 0.5] {
                let a = heading + side * fov_x;
                let far = add(view.eye, [a.sin() * 300.0, a.cos() * 300.0, 0.0]);
                r.world_segment(view.eye, far, 1.5 * ss, [0.2, 0.9, 1.0]);
            }
            if let Some((x, y)) = camera.project(view.eye, w, h) {
                r.disc(x, y, 4.0 * ss, [0.2, 0.9, 1.0]);
            }
        }
    }
    // Scale bar: 256 units, bottom left.
    if let raster::Projection::Orthographic { half_width } = camera.projection {
        let pixels = 256.0 / (2.0 * half_width) * w as f32;
        let (x, y) = (12.0 * ss, h as f32 - 12.0 * ss);
        r.segment(x, y, x + pixels, y, 3.0 * ss, [1.0; 3]);
    }
}

/// The first-person camera: where the player arrives, else the middle of
/// the plan. Returns the camera and a description of where it stands.
pub fn view_camera(
    scene: &CellScene,
    arrival: usize,
    fallback: ([f32; 2], f32),
    fov_x: f32,
) -> (Camera, String) {
    match scene.cell.arrivals.get(arrival) {
        Some(a) => {
            let eye = add(a.position, [0.0, 0.0, EYE_HEIGHT]);
            (
                Camera::first_person(eye, a.rotation[2], 0.0, fov_x),
                format!("where the player arrives via {}", a.via),
            )
        }
        None => {
            let ([x, y], floor) = fallback;
            let eye = [x, y, floor + EYE_HEIGHT];
            (
                Camera::first_person(eye, 0.0, 0.0, fov_x),
                "the middle of the plan, facing north".into(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sequence(looping: bool) -> nif::Sequence {
        nif::Sequence {
            name: "Backward".into(),
            start: 0.0,
            stop: 2.0,
            looping,
            tracks: Vec::new(),
            accum_root: None,
            materials: Vec::new(),
            text_keys: Vec::new(),
        }
    }

    #[test]
    fn a_one_shot_sequence_stops_playing_at_its_end_and_a_loop_never_does() {
        let once = sequence(false);
        assert!(sequence_playing(&once, 0.0));
        assert!(sequence_playing(&once, 1.9));
        assert!(!sequence_playing(&once, 2.0));
        assert!(!sequence_playing(&once, 300.0));
        let looped = sequence(true);
        assert!(sequence_playing(&looped, 0.0));
        assert!(sequence_playing(&looped, 300.0));
    }

    /// A mesh with a lit shader property with these flag sets and, when
    /// given, an alpha property.
    fn piece(flags: u32, flags2: u32, alpha: Option<nif::AlphaProperty>) -> nif::Mesh {
        nif::Mesh {
            name: "Piece".into(),
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
            shader: Some(nif::ShaderProperty {
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
                shader_flags2: flags2,
                env_map_scale: 1.0,
                texture_clamp: 3,
                texture_set: -1,
                file_name: None,
                layout_ok: true,
                falloff: None,
            }),
            material: None,
            alpha,
            zbuffer: None,
            double_sided: false,
            skinned: false,
            skin: None,
            property_types: Vec::new(),
        }
    }

    #[test]
    fn opaque_pieces_write_depth_whatever_their_flag_says() {
        // The BOS water crate: specular, environment map, alpha texture,
        // z test; second set 0x8000 only. Drawn writing depth at
        // Goodsprings.
        let crate_ = piece(0x8200_0181, 0x8000, None);
        assert_eq!(depth_of(&crate_), (true, true));
        // A plain opaque piece with the flag: the same.
        assert_eq!(depth_of(&piece(0x8200_0001, 0x1, None)), (true, true));
        // A blended piece follows its flag: the ceiling fan's blades
        // (written), a light beam (not).
        let blended = nif::AlphaProperty {
            flags: 0x0001 | (6 << 1) | (7 << 5),
            threshold: 0,
        };
        assert_eq!(
            depth_of(&piece(0x8200_0000, 0x1, Some(blended))),
            (true, true)
        );
        assert_eq!(
            depth_of(&piece(0xa200_0148, 0x0, Some(blended))),
            (true, false)
        );
        // Alpha-tested cut-outs are opaque: written.
        let tested = nif::AlphaProperty {
            flags: 0x0200 | (4 << 10),
            threshold: 128,
        };
        assert_eq!(
            depth_of(&piece(0x8200_0101, 0x0, Some(tested))),
            (true, true)
        );
        // Decals never write, flag or not.
        assert_eq!(
            depth_of(&piece(0x8c00_0000, 0x1, Some(tested))),
            (true, false)
        );
        // The "dynamic alpha" flag blends a piece without an alpha property
        // (the window glow cards): then the flag decides.
        assert_eq!(depth_of(&piece(0x8208_0101, 0x1, None)), (true, true));
        assert_eq!(depth_of(&piece(0x8208_0101, 0x0, None)), (true, false));
    }

    #[test]
    fn people_are_shaded_by_their_skin_and_hair_flags() {
        // A face (0x82000403), Sunny Smiles' bare arms (0x82000402), her
        // hair (0x82040101), her leather jacket (0x82000103).
        assert_eq!(Shading::from_flags(0x8200_0403), Shading::Skin);
        assert_eq!(Shading::from_flags(0x8200_0402), Shading::Skin);
        assert_eq!(Shading::from_flags(0x8204_0101), Shading::Hair);
        assert_eq!(Shading::from_flags(0x8200_0103), Shading::Plain);
    }
}
