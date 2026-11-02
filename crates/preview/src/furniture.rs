//! Sitting in furniture, from the game's files: the furniture model's
//! markers (`nif::FurnitureMarker`) and which of them its `MNAM` lets
//! people use (`world::furniture`), the idle tree's furniture branch for
//! the entry and exit animations and the seated loop (`world::idles`),
//! and the animations' root travel.

use esm::{FormId, LoadOrder};
use world::idles::{IdleTree, InFurniture};

/// Where and how someone sits in a piece of furniture.
#[derive(Debug, Clone, PartialEq)]
pub struct Sitting {
    /// The seat in the furniture model's space (at floor height), and the
    /// heading there (radians clockwise from the model's +y), from the
    /// first usable marker (the front one, number 14, when usable).
    pub seat: [f32; 3],
    pub heading: f32,
    /// The seated loop the idle tree gives (`DynamicIdle_ChairSit.kf`).
    pub idle: Option<String>,
    pub markers: Vec<nif::FurnitureMarker>,
    /// The furniture's `MNAM`: which markers may be used (bit per marker)
    /// and whether it's for sitting (0x40000000) or a bed (0x80000000).
    pub flags: u32,
    /// Each marker's entry and exit animations and their root travel.
    pub entries: Vec<(u8, String, [f32; 3])>,
    pub exits: Vec<(u8, String, [f32; 3])>,
}

fn sequence(assets: &assets::Assets, path: &str) -> Option<nif::Sequence> {
    let bytes = assets.read(&assets::mesh_path(path)).ok()??;
    nif::Nif::parse(bytes)
        .ok()?
        .sequences()
        .ok()?
        .into_iter()
        .next()
}

/// A piece of furniture's markers (its base record, `FURN`): from its
/// model's `BSFurnitureMarker`, in the model's space. `None` when the
/// model can't be read; empty when it has none.
pub fn markers(
    assets: &assets::Assets,
    order: &LoadOrder,
    furniture: FormId,
) -> Option<Vec<nif::FurnitureMarker>> {
    let record = order.get(furniture)?.record().ok()?;
    let model = record.get(esm::FourCC::new(b"MODL"))?.zstring();
    let nif = nif::Nif::parse(assets.read(&assets::mesh_path(&model)).ok()??).ok()?;
    Some(nif.furniture_markers())
}

/// Whether a piece of furniture's model (its base record, `FURN`) has
/// collision: what `00920fd0` asks of its 3D (`004b66d0`, a count of its
/// collision objects above 0). False when the model can't be read.
pub fn has_collision(assets: &assets::Assets, order: &LoadOrder, furniture: FormId) -> bool {
    let Some(record) = order.get(furniture).and_then(|r| r.record().ok()) else {
        return false;
    };
    let Some(model) = record.get(esm::FourCC::new(b"MODL")).map(|s| s.zstring()) else {
        return false;
    };
    let Some(nif) = assets
        .read(&assets::mesh_path(&model))
        .ok()
        .flatten()
        .and_then(|b| nif::Nif::parse(b).ok())
    else {
        return false;
    };
    nif.collision()
        .is_ok_and(|c| !c.parts.is_empty() || c.no_collision > 0)
}

/// How a person sits in a piece of furniture (its base record, `FURN`):
/// `None` when its model has no markers or no exit the idle tree knows.
pub fn sitting(
    assets: &assets::Assets,
    order: &LoadOrder,
    tree: &IdleTree,
    furniture: FormId,
    female: bool,
) -> Option<Sitting> {
    let markers = markers(assets, order, furniture)?;
    let flags = world::furniture::marker_flags(order, furniture);
    let asked = |sitting: u8, marker: u8| InFurniture {
        sitting,
        marker,
        female,
        player: false,
        child: false,
    };
    let with_travel = |sitting: u8| {
        let mut out = Vec::new();
        for m in &markers {
            let Some(idle) = tree.furniture_idle(asked(sitting, m.marker)) else {
                continue;
            };
            let Some(travel) = sequence(assets, &idle.model).and_then(|s| s.root_travel()) else {
                continue;
            };
            out.push((m.marker, idle.model.clone(), travel));
        }
        out
    };
    let entries = with_travel(2);
    let exits = with_travel(4);
    // Sat from the front when it's usable, else the first usable marker
    // (else the first): the game's settings for that marker, else the exit
    // animations.
    let usable = |i: usize| flags & (1 << i.min(31)) != 0;
    let entry = markers
        .iter()
        .enumerate()
        .find(|(i, m)| m.marker == 14 && usable(*i))
        .or_else(|| markers.iter().enumerate().find(|(i, _)| usable(*i)))
        .map(|(_, m)| m)
        .or(markers.first());
    let by_settings = entry.and_then(|m| {
        world::idles::seat_by_settings(m, |name| world::scripting::game_setting(order, name))
    });
    let (seat, heading) = match by_settings {
        Some(s) => s,
        None => world::idles::seat(&markers, |marker| {
            exits.iter().find(|e| e.0 == marker).map(|e| e.2)
        })?,
    };
    let marker = entry.map_or(14, |m| m.marker);
    let idle = tree
        .furniture_idle(asked(1, marker))
        .map(|i| i.model.clone());
    Some(Sitting {
        seat,
        heading,
        idle,
        markers,
        flags,
        entries,
        exits,
    })
}
