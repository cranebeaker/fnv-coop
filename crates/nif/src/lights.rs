//! Lights stored in a model (`NiAmbientLight`, `NiDirectionalLight`,
//! `NiPointLight`, `NiSpotLight`), where they are in the model's space and
//! their colours. Layout for version 20.2.0.7 (Bethesda version 34): the
//! scene object's fields, then `NiDynamicEffect` (switched on: a byte;
//! the nodes it affects: a counted list of references), `NiLight` (dimmer,
//! ambient, diffuse and specular colours) and for point and spot lights
//! the constant, linear and quadratic attenuation (spot lights then their
//! cutoff angle, an unknown float and the exponent).
//!
//! The game itself takes a model's point and directional lights into a
//! scene by code (`00b5ca70` → `00b5c940` for the lockpicking menu's
//! models): what this gives is what the file says.

use crate::blocks::{read_av_object, Block};
use crate::error::Result;
use crate::file::Nif;
use crate::math::{Transform, Vec3};
use crate::reader::Reader;

/// A light's kind, by its block type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightKind {
    Ambient,
    Directional,
    Point,
    Spot,
}

/// A light in a model.
#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    pub kind: LightKind,
    pub name: String,
    /// The light's block.
    pub block: usize,
    /// From the light's own space to the model's (every node above it, the
    /// top node's included).
    pub transform: Transform,
    /// `NiDynamicEffect`'s switch: on.
    pub on: bool,
    pub dimmer: f32,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    /// Point and spot lights: constant, linear, quadratic.
    pub attenuation: Option<[f32; 3]>,
}

impl Light {
    /// Where it is in the model's space.
    pub fn position(&self) -> Vec3 {
        self.transform.translation
    }
}

fn kind_of(type_name: &str) -> Option<LightKind> {
    match type_name {
        "NiAmbientLight" => Some(LightKind::Ambient),
        "NiDirectionalLight" => Some(LightKind::Directional),
        "NiPointLight" => Some(LightKind::Point),
        "NiSpotLight" => Some(LightKind::Spot),
        _ => None,
    }
}

impl Nif {
    /// Every light under the model's top nodes, with its place in the
    /// model's space.
    pub fn lights(&self) -> Result<Vec<Light>> {
        let mut out = Vec::new();
        let mut visited = vec![false; self.blocks().len()];
        for &root in self.roots() {
            self.collect_lights(root, &Transform::IDENTITY, &mut visited, &mut out)?;
        }
        Ok(out)
    }

    fn collect_lights(
        &self,
        reference: i32,
        parent: &Transform,
        visited: &mut [bool],
        out: &mut Vec<Light>,
    ) -> Result<()> {
        let Some(index) = usize::try_from(reference)
            .ok()
            .filter(|&i| i < self.blocks().len())
        else {
            return Ok(());
        };
        if visited[index] {
            return Ok(());
        }
        visited[index] = true;
        if let Some(kind) = kind_of(self.block_type(index)) {
            out.push(self.light(index, kind, parent)?);
            return Ok(());
        }
        if let Block::Node(node) = self.block(index)? {
            let world = parent.then_child(&node.av.transform);
            for child in node.children {
                self.collect_lights(child, &world, visited, out)?;
            }
        }
        Ok(())
    }

    /// Any scene object's name (its `NiAVObject` fields).
    pub fn av_name(&self, index: usize) -> Option<String> {
        if index >= self.blocks().len() {
            return None;
        }
        let ctx = self.ctx();
        let mut r = Reader::new(self.block_bytes(index), self.blocks()[index].offset);
        read_av_object(&mut r, &ctx).ok().map(|av| av.net.name)
    }

    /// Any scene object's own transform (its `NiAVObject` fields), also
    /// for blocks [`Nif::block`] doesn't decode (lights, cameras).
    pub fn av_transform(&self, index: usize) -> Option<Transform> {
        if index >= self.blocks().len() {
            return None;
        }
        let ctx = self.ctx();
        let mut r = Reader::new(self.block_bytes(index), self.blocks()[index].offset);
        read_av_object(&mut r, &ctx).ok().map(|av| av.transform)
    }

    fn light(&self, index: usize, kind: LightKind, parent: &Transform) -> Result<Light> {
        let ctx = self.ctx();
        let mut r = Reader::new(self.block_bytes(index), self.blocks()[index].offset);
        let av = read_av_object(&mut r, &ctx)?;
        let on = r.u8("the switch state")? != 0;
        let affected = r.u32("the affected node count")?;
        r.take(affected as usize * 4, "the affected nodes")?;
        let dimmer = r.f32("the dimmer")?;
        let ambient = r.vec3("the ambient colour")?;
        let diffuse = r.vec3("the diffuse colour")?;
        let specular = r.vec3("the specular colour")?;
        let attenuation = match kind {
            LightKind::Point | LightKind::Spot => Some([
                r.f32("the constant attenuation")?,
                r.f32("the linear attenuation")?,
                r.f32("the quadratic attenuation")?,
            ]),
            _ => None,
        };
        Ok(Light {
            kind,
            name: av.net.name.clone(),
            block: index,
            transform: parent.then_child(&av.transform),
            on,
            dimmer,
            ambient,
            diffuse,
            specular,
            attenuation,
        })
    }
}
