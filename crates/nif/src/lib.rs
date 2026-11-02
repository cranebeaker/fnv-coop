//! Reader for the NIF mesh files used by Fallout 3 and Fallout: New Vegas
//! (Gamebryo file version 20.2.0.7).
//!
//! A NIF file is a list of typed *blocks* that reference each other by
//! index: nodes form a tree, shapes hang off nodes, and shapes point to
//! their vertex data and to properties (shader, textures, material, alpha).
//! Files of this version record every block's size, so block types this
//! reader doesn't decode (collision, animation controllers, particles...)
//! are skipped safely.
//!
//! [`Nif::scene`] walks the tree and returns ready-to-draw meshes: vertices
//! in the model's space, triangles, texture paths and render settings.
//!
//! ```no_run
//! let nif = nif::Nif::open("10mmpistol.nif")?;
//! for mesh in nif.scene()?.meshes {
//!     println!("{}: {} triangles, textures {:?}", mesh.name, mesh.triangles.len(), mesh.textures);
//! }
//! # Ok::<(), nif::Error>(())
//! ```

pub mod additional;
pub mod anim;
mod blocks;
pub mod camera;
pub mod collision;
pub mod ctl;
pub mod egm;
pub mod egt;
mod error;
mod file;
mod header;
pub mod lights;
pub mod math;
pub mod obj;
pub mod particles;
pub mod ragdoll;
mod reader;
mod scene;
pub mod segments;
pub mod skin;
pub mod tri;

pub use anim::{
    hang_weapon, posed, posed_layers, posed_over, reparent_weapon, sample_curve, weapon_parent,
    Bone, FloatKey, MaterialKeys, MaterialTarget, MaterialTrack, Motion, Pose, Sequence, Track,
};
pub use blocks::{
    AlphaProperty, AvObject, Block, Falloff, Geometry, GeometryData, MaterialProperty, Node,
    ObjectNet, ShaderProperty, SourceTexture, StencilProperty, TextureSet, TexturingProperty,
    ZBufferProperty,
};
pub use collision::{Collision, CollisionPart, CollisionShape, RigidBodyInfo};
pub use ctl::Ctl;
pub use egm::Egm;
pub use egt::Egt;
pub use error::{Error, Result};
pub use file::{BlockInfo, Nif};
pub use header::{version_string, Header, V20_2_0_7};
pub use lights::{Light, LightKind};
pub use math::Transform;
pub use ragdoll::{JointLimit, Ragdoll, RagdollBody, RagdollJoint};
pub use scene::{posed_chain, Mesh, Scene};
pub use segments::Segment;
pub use skin::{Bound, FurnitureMarker, Partition, Skin};
pub use tri::Tri;
