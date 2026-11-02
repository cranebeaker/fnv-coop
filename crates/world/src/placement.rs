//! The objects placed in a cell: where they are, what they look like, and
//! which of them the game shows when the cell first loads.

use std::collections::{BTreeMap, HashMap};

use esm::{flags, sig, FormId, FourCC, LoadOrder, LoadedPlugin, Record, RecordRef};

use crate::cell::{cell_info, le_f32, le_u32, CellInfo};
use crate::Result;

const XSCL: FourCC = FourCC::new(b"XSCL");
const XESP: FourCC = FourCC::new(b"XESP");
const XTEL: FourCC = FourCC::new(b"XTEL");
const XRDS: FourCC = FourCC::new(b"XRDS");
const XEMI: FourCC = FourCC::new(b"XEMI");
const REGN: FourCC = FourCC::new(b"REGN");
const MOD2: FourCC = FourCC::new(b"MOD2");
const ONAM: FourCC = FourCC::new(b"ONAM");
const FNAM: FourCC = FourCC::new(b"FNAM");
const STAT: FourCC = FourCC::new(b"STAT");
const ARMO: FourCC = FourCC::new(b"ARMO");
const SCOL: FourCC = FourCC::new(b"SCOL");
const LIGH: FourCC = FourCC::new(b"LIGH");
const XPRM: FourCC = FourCC::new(b"XPRM");
const XTRI: FourCC = FourCC::new(b"XTRI");
const XACT: FourCC = FourCC::new(b"XACT");

/// The engine's `CollisionMarker` static (form 0x21, which the game makes
/// itself if a plugin lacks it: `0046a370`): a reference to it with a
/// primitive (`XPRM`) is a solid box, sphere or plane.
pub const COLLISION_MARKER: FormId = FormId(0x21);

/// A reference's "open by default" action flag (`XACT` 0x08, written with
/// an empty `ONAM` marker): the door starts open. The game's door code asks
/// the reference's action flags for 0x08 (`00561d90`, through `0041b3a0`).
pub const OPEN_BY_DEFAULT: u32 = 0x08;

/// Enable parents can chain; deeper than this is treated as enabled.
const MAX_PARENT_DEPTH: u8 = 16;

/// Light `DATA` flags.
pub mod light_flags {
    pub const NEGATIVE: u32 = 0x0004;
    pub const FLICKER: u32 = 0x0008;
    pub const OFF_BY_DEFAULT: u32 = 0x0020;
    pub const SPOT: u32 = 0x0200;
}

/// A light source, from a `LIGH` base object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    /// Reach in game units.
    pub radius: f32,
    pub color: [u8; 3],
    pub flags: u32,
    pub falloff: f32,
    /// Spotlight cone, in degrees.
    pub fov: f32,
    /// Brightness multiplier (`FNAM`).
    pub fade: f32,
}

impl Light {
    fn parse(record: &Record) -> Option<Self> {
        let data = &record.get(sig::DATA)?.data;
        if data.len() < 16 {
            return None;
        }
        let f = |at: usize, default: f32| {
            if data.len() >= at + 4 {
                le_f32(data, at)
            } else {
                default
            }
        };
        Some(Self {
            radius: le_u32(data, 4) as f32,
            color: [data[8], data[9], data[10]],
            flags: le_u32(data, 12),
            falloff: f(16, 1.0),
            fov: f(20, 90.0),
            fade: record
                .get(FNAM)
                .filter(|s| s.data.len() >= 4)
                .map_or(1.0, |s| le_f32(&s.data, 0)),
        })
    }

    pub fn is_negative(&self) -> bool {
        self.flags & light_flags::NEGATIVE != 0
    }

    pub fn is_off_by_default(&self) -> bool {
        self.flags & light_flags::OFF_BY_DEFAULT != 0
    }
}

/// A placed light as the game lights with it, confirmed against the shader
/// constants the game sends (an apitrace recording of Doc Mitchell's
/// house, where every value matched): the radius is the base light's plus
/// the reference's own Radius (`XRDS`), and when the reference's Emittance
/// (`XEMI`) names a light, the color is multiplied by that light's color
/// (that light's fade isn't used). The fade multiplies the color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedLight {
    /// 0..1 per channel, before the fade.
    pub color: [f32; 3],
    /// Brightness multiplier (the base light's `FNAM`).
    pub fade: f32,
    /// Reach in game units.
    pub radius: f32,
}

/// Where a placed object's glow takes its color from: the reference's
/// Emittance setting (`XEMI`). For meshes whose shader has the external
/// emittance flag it's the glow's color; for a placed light it tints the
/// light (see [`PlacedLight`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Emittance {
    /// A light's color (a `LIGH` record), which doesn't change.
    Light([u8; 3]),
    /// A region (a `REGN` record), whose color follows its weather's
    /// sunlight through the day (`weather::EmittanceNow`).
    Region(FormId),
    /// A record that's neither, or that doesn't exist.
    Other(FormId),
}

/// One piece of a static collection, placed relative to the collection.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub base: FormId,
    pub model: Option<String>,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: f32,
}

/// A primitive placed with a reference (`XPRM`, 32 bytes: three sizes,
/// a colour for the editor, a float, the shape), and the collision layer
/// the reference names (`XTRI`, none in the game's own files).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Primitive {
    /// Half sizes along the reference's own x, y and z (game units).
    pub half: [f32; 3],
    /// 1 box, 2 sphere (radius `half[0]`), 3 plane (`half[0]` wide and
    /// `half[2]` tall, in the reference's x-z plane): the game's primitive
    /// factory, `004a4e90`.
    pub shape: u32,
    pub layer: Option<u32>,
}

/// Where a door leads, and where the player arrives on the other side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Teleport {
    pub door: FormId,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

/// A placed object, actor or marker.
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub form_id: FormId,
    /// `REFR`, `ACHR`, `ACRE`, `PGRE` or `PMIS`.
    pub record_type: FourCC,
    pub editor_id: Option<String>,
    pub base: FormId,
    pub base_type: FourCC,
    pub base_editor_id: Option<String>,
    /// Position in game units.
    pub position: [f32; 3],
    /// Stored angles in radians; see [`crate::RotationConvention`].
    pub rotation: [f32; 3],
    pub scale: f32,
    /// Model path as written in the base record (relative to `meshes\`).
    pub model: Option<String>,
    /// Static collection pieces, for drawing it when its combined model is
    /// missing.
    pub parts: Vec<Part>,
    /// For a light, its base object's light. [`Placement::placed_light`]
    /// gives the light the game actually uses.
    pub light: Option<Light>,
    /// The reference's Radius field (`XRDS`), as stored. For a light the
    /// game adds it to the base light's radius.
    pub radius: Option<f32>,
    pub teleport: Option<Teleport>,
    /// Where glowing parts marked for external emittance take their color.
    pub emittance: Option<Emittance>,
    pub flags: u32,
    /// The reference whose enabled state this one follows, and whether it
    /// does the opposite.
    pub enable_parent: Option<(FormId, bool)>,
    /// The plugin whose version of this reference wins.
    pub plugin: String,
    /// For people and creatures: the models that make them up.
    pub actor: Option<crate::actor::ActorLook>,
    /// The reference's primitive (`XPRM`): a trigger's volume, a collision
    /// marker's box.
    pub primitive: Option<Primitive>,
    /// The reference's action flags (`XACT`) hold "open by default"
    /// ([`OPEN_BY_DEFAULT`]): a door that starts open.
    pub open_by_default: bool,
}

/// A reference that isn't shown, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct LeftOut {
    pub form_id: FormId,
    pub record_type: FourCC,
    pub base: Option<FormId>,
    pub base_editor_id: Option<String>,
    pub reason: String,
}

/// A spot the player can arrive at in the cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Arrival {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    /// How the player gets here, for display.
    pub via: String,
}

/// A cell with its contents, as the game shows them when it first loads.
#[derive(Debug, Clone)]
pub struct LoadedCell {
    pub info: CellInfo,
    /// Objects to draw: anything with a model, plus lights.
    pub objects: Vec<Placement>,
    /// NPCs and creatures (not drawn yet).
    pub actors: Vec<Placement>,
    /// Editor markers the game doesn't draw.
    pub markers: Vec<Placement>,
    /// References left out, by reason.
    pub skipped: BTreeMap<String, usize>,
    /// The same references, one by one.
    pub left_out: Vec<LeftOut>,
    /// Where the player arrives: the cell's `coc` marker and the far side of
    /// each load door.
    pub arrivals: Vec<Arrival>,
    /// The colors the Emittance sources give (the regions named here, and
    /// the player's weather region for references without one): as the
    /// game starts unless set from the game's state
    /// ([`LoadedCell::set_emittance`]).
    pub emittance: crate::weather::EmittanceNow,
}

impl LoadedCell {
    /// A "cell" holding only these people, where they're placed: for
    /// drawing people who come into a place after it loaded.
    pub fn actors_only(actors: Vec<Placement>) -> LoadedCell {
        let mut cell = LoadedCell::lone_actor_with(None);
        cell.actors = actors;
        cell
    }

    /// A "cell" holding one actor at the origin facing north, and nothing
    /// else: for drawing an actor on its own (the first-person view).
    pub fn lone_actor(look: crate::actor::ActorLook) -> LoadedCell {
        LoadedCell::lone_actor_with(Some(look))
    }

    fn lone_actor_with(look: Option<crate::actor::ActorLook>) -> LoadedCell {
        let actor = look.map(|look| Placement {
            form_id: FormId(0),
            record_type: FourCC::new(b"ACHR"),
            editor_id: None,
            base: look.base,
            base_type: FourCC::new(b"NPC_"),
            base_editor_id: None,
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0,
            model: None,
            parts: Vec::new(),
            light: None,
            radius: None,
            teleport: None,
            emittance: None,
            flags: 0,
            enable_parent: None,
            plugin: String::new(),
            actor: Some(look),
            primitive: None,
            open_by_default: false,
        });
        LoadedCell {
            info: CellInfo {
                form_id: FormId(0),
                editor_id: Some("LoneActor".into()),
                name: None,
                plugin: String::new(),
                flags: 0,
                interior: true,
                grid: None,
                world: None,
                lighting: None,
                lighting_source: String::new(),
                water_height: None,
                image_space: None,
                sky: None,
                weather: None,
            },
            objects: Vec::new(),
            actors: actor.into_iter().collect(),
            markers: Vec::new(),
            skipped: BTreeMap::new(),
            left_out: Vec::new(),
            arrivals: Vec::new(),
            emittance: Default::default(),
        }
    }
}

impl Placement {
    /// For a light, the light the game uses (see [`PlacedLight`]), its
    /// color × `tint` (the color its Emittance gives, if any).
    pub fn placed_light_tinted(&self, tint: Option<[f32; 3]>) -> Option<PlacedLight> {
        let light = self.light?;
        let mut color = light.color.map(|c| f32::from(c) / 255.0);
        if let Some(tint) = tint {
            for (c, t) in color.iter_mut().zip(tint) {
                *c *= t;
            }
        }
        Some(PlacedLight {
            color,
            fade: light.fade,
            radius: light.radius + self.radius.unwrap_or(0.0),
        })
    }
}

impl LoadedCell {
    pub fn lights(&self) -> impl Iterator<Item = (&Placement, &Light)> {
        self.objects
            .iter()
            .filter_map(|p| p.light.as_ref().map(|l| (p, l)))
    }

    /// The color (0..1 per channel) that a placed object's meshes marked
    /// for external emittance glow with, before their multiplier: its
    /// Emittance's color (a light's, a region's now), or with none the
    /// player's weather region's (the game's code, `005453b0`). `None`
    /// (the mesh keeps its own color) when that gives no color.
    pub fn emittance_color(&self, object: &Placement) -> Option<[f32; 3]> {
        match object.emittance {
            Some(e) => self.color_of(e),
            None => self.emittance.player_region,
        }
    }

    /// What an Emittance gives now (0..1 per channel).
    pub fn color_of(&self, e: Emittance) -> Option<[f32; 3]> {
        match e {
            Emittance::Light(c) => Some(c.map(|v| f32::from(v) / 255.0)),
            Emittance::Region(r) => self.emittance.regions.get(&r).copied(),
            Emittance::Other(_) => None,
        }
    }

    /// A placed light as the game uses it (see [`PlacedLight`]): tinted by
    /// the color its Emittance gives; lights without one aren't tinted
    /// (the recordings' lights had their own colors).
    pub fn placed_light(&self, object: &Placement) -> Option<PlacedLight> {
        object.placed_light_tinted(object.emittance.and_then(|e| self.color_of(e)))
    }

    /// The regions this cell's references name as their Emittance.
    pub fn emittance_regions(&self) -> std::collections::BTreeSet<FormId> {
        self.objects
            .iter()
            .chain(&self.actors)
            .filter_map(|p| match p.emittance {
                Some(Emittance::Region(r)) => Some(r),
                _ => None,
            })
            .collect()
    }

    /// Takes the Emittance colors from the game's weather state at `hour`
    /// (see [`crate::weather::EmittanceNow`]).
    pub fn set_emittance(
        &mut self,
        order: &LoadOrder,
        state: &crate::weather::WeatherState,
        hour: f32,
    ) {
        self.emittance =
            crate::weather::EmittanceNow::new(order, state, hour, self.emittance_regions());
    }
}

#[derive(Debug, Clone)]
struct BaseInfo {
    kind: FourCC,
    editor_id: Option<String>,
    model: Option<String>,
    parts: Vec<Part>,
    light: Option<Light>,
}

/// References scripts enabled (false) or disabled (true), by form ID
/// (`scripting::GameState::disabled`).
pub type Disabled = HashMap<FormId, bool>;

/// Reads a cell and everything placed in it, as when it first loads.
pub fn load_cell(order: &LoadOrder, cell: FormId) -> Result<LoadedCell> {
    load_cell_with(order, cell, Vec::new(), &Disabled::new())
}

/// [`load_cell`] with what scripts have enabled and disabled since.
pub fn load_cell_now(order: &LoadOrder, cell: FormId, disabled: &Disabled) -> Result<LoadedCell> {
    load_cell_with(order, cell, Vec::new(), disabled)
}

/// [`load_cell_now`], plus references stored elsewhere that stand in the
/// cell (an exterior square's persistent objects).
pub(crate) fn load_cell_with<'a>(
    order: &'a LoadOrder,
    cell: FormId,
    extra: Vec<RecordRef<'a>>,
    disabled: &Disabled,
) -> Result<LoadedCell> {
    let info = cell_info(order, cell)?;
    let mut bases: HashMap<FormId, Option<BaseInfo>> = HashMap::new();
    let mut loaded = LoadedCell {
        info,
        objects: Vec::new(),
        actors: Vec::new(),
        markers: Vec::new(),
        skipped: BTreeMap::new(),
        left_out: Vec::new(),
        arrivals: Vec::new(),
        emittance: Default::default(),
    };
    let mut skip = |rr: &RecordRef<'_>,
                    base: Option<FormId>,
                    base_editor_id: Option<String>,
                    reason: String| {
        *loaded.skipped.entry(reason.clone()).or_default() += 1;
        loaded.left_out.push(LeftOut {
            form_id: rr.form_id,
            record_type: rr.entry.header.kind,
            base,
            base_editor_id,
            reason,
        });
    };

    let mut refs = order.references_in_cell(cell);
    refs.extend(extra);
    refs.sort_by_key(|r| r.form_id);
    for rr in refs {
        if rr.entry.header.is_deleted() {
            skip(&rr, None, None, "deleted".into());
            continue;
        }
        let record = rr.record()?;
        let Some(base_local) = record.get(sig::NAME).filter(|s| s.data.len() >= 4) else {
            skip(&rr, None, None, "no base object".into());
            continue;
        };
        let base = rr.plugin.to_global(FormId(le_u32(&base_local.data, 0)));
        let info = bases
            .entry(base)
            .or_insert_with(|| base_info(order, base))
            .clone();
        let base_editor_id = info.as_ref().and_then(|i| i.editor_id.clone());
        if !enabled_with(order, &rr, &record, 0, disabled) {
            skip(
                &rr,
                Some(base),
                base_editor_id,
                "disabled when the cell loads".into(),
            );
            continue;
        }
        let Some(info) = info else {
            skip(&rr, Some(base), None, "base object missing".into());
            continue;
        };

        let placement = read_placement(order, &rr, &record, base, info.clone());
        let kind = rr.entry.header.kind;
        if kind == sig::ACHR || kind == sig::ACRE {
            loaded.actors.push(placement);
        } else if is_marker(base, info.kind, info.model.as_deref()) {
            loaded.markers.push(placement);
        } else if info.model.is_none() && info.parts.is_empty() && info.light.is_none() {
            skip(
                &rr,
                Some(base),
                info.editor_id.clone(),
                format!("no model ({})", info.kind),
            );
        } else {
            loaded.objects.push(placement);
        }
    }

    loaded.arrivals = arrivals(order, &loaded);
    loaded.emittance = crate::weather::EmittanceNow::at_start(order, loaded.emittance_regions());
    Ok(loaded)
}

fn read_placement(
    order: &LoadOrder,
    rr: &RecordRef<'_>,
    record: &Record,
    base: FormId,
    info: BaseInfo,
) -> Placement {
    let floats = |kind: FourCC, count: usize| -> Option<Vec<f32>> {
        let s = record.get(kind).filter(|s| s.data.len() >= count * 4)?;
        Some((0..count).map(|i| le_f32(&s.data, i * 4)).collect())
    };
    let data = floats(sig::DATA, 6).unwrap_or_else(|| vec![0.0; 6]);
    Placement {
        form_id: rr.form_id,
        record_type: rr.entry.header.kind,
        editor_id: record.editor_id(),
        base,
        base_type: info.kind,
        base_editor_id: info.editor_id,
        position: [data[0], data[1], data[2]],
        rotation: [data[3], data[4], data[5]],
        scale: floats(XSCL, 1).map_or(1.0, |s| s[0]),
        model: info.model,
        parts: info.parts,
        light: info.light,
        radius: floats(XRDS, 1).map(|r| r[0]),
        teleport: read_teleport(rr.plugin, record),
        emittance: read_emittance(order, rr, record),
        flags: rr.entry.header.flags,
        enable_parent: enable_parent(rr, record),
        plugin: rr.plugin.name.clone(),
        actor: if rr.entry.header.kind == sig::ACHR || rr.entry.header.kind == sig::ACRE {
            crate::actor::actor_look(order, base)
        } else {
            None
        },
        primitive: read_primitive(record),
        // The editor writes both the flag and an empty `ONAM` marker (the
        // reference's own save writes the marker, `0055ae70`).
        open_by_default: record.get(ONAM).is_some()
            || record
                .get(XACT)
                .filter(|s| s.data.len() >= 4)
                .is_some_and(|s| le_u32(&s.data, 0) & OPEN_BY_DEFAULT != 0),
    }
}

fn read_primitive(record: &Record) -> Option<Primitive> {
    let s = record.get(XPRM).filter(|s| s.data.len() >= 32)?;
    Some(Primitive {
        half: [le_f32(&s.data, 0), le_f32(&s.data, 4), le_f32(&s.data, 8)],
        shape: le_u32(&s.data, 28),
        layer: record
            .get(XTRI)
            .filter(|s| s.data.len() >= 4)
            .map(|s| le_u32(&s.data, 0)),
    })
}

/// A reference made while playing (`PlaceAtMe`, `world::more_functions::
/// placed`), as a placed reference of its base would be: the base's model,
/// light and looks (a person or creature's), at a place and turn, scale 1.
pub fn made_placement(
    order: &LoadOrder,
    form_id: FormId,
    base: FormId,
    position: [f32; 3],
    rotation: [f32; 3],
) -> Option<Placement> {
    let info = base_info(order, base)?;
    let record_type = match info.kind.as_bytes() {
        b"NPC_" => sig::ACHR,
        b"CREA" => sig::ACRE,
        _ => FourCC::new(b"REFR"),
    };
    let actor = (record_type != FourCC::new(b"REFR"))
        .then(|| crate::actor::actor_look(order, base))
        .flatten();
    Some(Placement {
        form_id,
        record_type,
        editor_id: None,
        base,
        base_type: info.kind,
        base_editor_id: info.editor_id,
        position,
        rotation,
        scale: 1.0,
        model: info.model,
        parts: info.parts,
        light: info.light,
        radius: None,
        teleport: None,
        emittance: None,
        flags: 0,
        enable_parent: None,
        plugin: String::new(),
        actor,
        primitive: None,
        open_by_default: false,
    })
}

/// One placed reference, read as a cell's loading reads it.
pub fn placement_of(order: &LoadOrder, reference: FormId) -> Option<Placement> {
    let rr = order.get(reference)?;
    let record = rr.record().ok()?;
    let base_local = record.get(sig::NAME).filter(|s| s.data.len() >= 4)?;
    let base = rr.plugin.to_global(FormId(le_u32(&base_local.data, 0)));
    let info = base_info(order, base)?;
    Some(read_placement(order, &rr, &record, base, info))
}

/// A reference's door link (`XTEL`), if it's a load door.
pub fn teleport_of(rr: &RecordRef<'_>) -> Option<Teleport> {
    let record = rr.record().ok()?;
    read_teleport(rr.plugin, &record)
}

fn read_teleport(plugin: &LoadedPlugin, record: &Record) -> Option<Teleport> {
    let s = record.get(XTEL).filter(|s| s.data.len() >= 28)?;
    let f = |i: usize| le_f32(&s.data, 4 + i * 4);
    Some(Teleport {
        door: plugin.to_global(FormId(le_u32(&s.data, 0))),
        position: [f(0), f(1), f(2)],
        rotation: [f(3), f(4), f(5)],
    })
}

fn read_emittance(order: &LoadOrder, rr: &RecordRef<'_>, record: &Record) -> Option<Emittance> {
    let s = record.get(XEMI).filter(|s| s.data.len() >= 4)?;
    let id = rr.plugin.to_global(FormId(le_u32(&s.data, 0)));
    if id.0 == 0 {
        return None;
    }
    Some(resolve_emittance(order, id))
}

/// What an Emittance setting (`XEMI`) naming `id` is: a light (its color),
/// or a region (whose color follows its weather through the day: the game
/// sent (255, 227, 170) from `NVInteriorRegion`'s `NVWastelandInterior`
/// weather by day, as the color of Doc Mitchell's light beams).
pub fn resolve_emittance(order: &LoadOrder, id: FormId) -> Emittance {
    let Some(source) = order.get(id) else {
        return Emittance::Other(id);
    };
    match source.entry.header.kind {
        LIGH => match source.record().ok().as_ref().and_then(Light::parse) {
            Some(light) => Emittance::Light(light.color),
            None => Emittance::Other(id),
        },
        REGN => Emittance::Region(id),
        _ => Emittance::Other(id),
    }
}

/// Whether a reference is shown now, with scripts' `Enable` / `Disable`
/// (`disabled`: true for disabled) applied to it or to the enable parent
/// it follows; otherwise as when its cell first loads.
pub fn enabled_now(order: &LoadOrder, reference: FormId, disabled: &Disabled) -> bool {
    if let Some(&d) = disabled.get(&reference) {
        return !d;
    }
    let Some(rr) = order.get(reference) else {
        return true;
    };
    // Only its enable parent is read (not the whole record): this is asked
    // for many references every frame.
    match enable_parent_of(&rr) {
        Ok(parent) => enabled_given(order, &rr, parent, 0, disabled),
        Err(_) => true,
    }
}

/// The references among `refs` shown with `now` but not with `before`
/// (two states of scripts' `Enable` / `Disable`): enabled themselves, or
/// following an enable parent that was. The game's `Enable` (`005c43d0`)
/// queues the reference's own enabling (`005aa5d0`) and ignores a
/// reference that has an enable parent (it follows the parent), so a
/// parent's `Enable` is what brings its children in: `VCG02BottleMarkerREF`
/// for the tutorial's sarsaparilla bottles.
pub fn newly_enabled(
    order: &LoadOrder,
    refs: impl IntoIterator<Item = FormId>,
    before: &Disabled,
    now: &Disabled,
) -> Vec<FormId> {
    refs.into_iter()
        .filter(|&r| enabled_now(order, r, now) && !enabled_now(order, r, before))
        .collect()
}

/// Whether the game shows a reference: a script's `Enable` / `Disable` of
/// it, else its enable parent's state (the reference follows the parent,
/// or does the opposite, and its own "initially disabled" flag is then
/// ignored), else that flag.
fn enabled_with(
    order: &LoadOrder,
    rr: &RecordRef<'_>,
    record: &Record,
    depth: u8,
    disabled: &Disabled,
) -> bool {
    enabled_given(order, rr, enable_parent(rr, record), depth, disabled)
}

/// [`enabled_with`], given the reference's enable parent.
fn enabled_given(
    order: &LoadOrder,
    rr: &RecordRef<'_>,
    parent: Option<(FormId, bool)>,
    depth: u8,
    disabled: &Disabled,
) -> bool {
    if let Some(&d) = disabled.get(&rr.form_id) {
        return !d;
    }
    match parent {
        Some((parent, opposite)) if depth < MAX_PARENT_DEPTH => {
            let parent_enabled = match disabled.get(&parent) {
                Some(&d) => !d,
                None => match order.get(parent) {
                    Some(prr) => match enable_parent_of(&prr) {
                        Ok(grand) => enabled_given(order, &prr, grand, depth + 1, disabled),
                        Err(_) => true,
                    },
                    None => true,
                },
            };
            parent_enabled != opposite
        }
        _ => rr.entry.header.flags & flags::INITIALLY_DISABLED == 0,
    }
}

/// A reference's enable parent read from its `XESP` alone.
fn enable_parent_of(rr: &RecordRef<'_>) -> esm::Result<Option<(FormId, bool)>> {
    Ok(rr
        .subrecord(XESP)?
        .filter(|d| d.len() >= 5)
        .map(|d| (rr.plugin.to_global(FormId(le_u32(&d, 0))), d[4] & 1 != 0)))
}

fn enable_parent(rr: &RecordRef<'_>, record: &Record) -> Option<(FormId, bool)> {
    record.get(XESP).filter(|s| s.data.len() >= 5).map(|s| {
        (
            rr.plugin.to_global(FormId(le_u32(&s.data, 0))),
            s.data[4] & 1 != 0,
        )
    })
}

fn base_info(order: &LoadOrder, base: FormId) -> Option<BaseInfo> {
    let rr = order.get(base)?;
    let kind = rr.entry.header.kind;
    let record = rr.record().ok()?;
    let zstring = |k: FourCC| {
        record
            .get(k)
            .map(|s| s.zstring())
            .filter(|s| !s.trim().is_empty())
    };
    // Armor's MODL is the model worn on the body; MOD2 is the one lying on
    // the ground.
    let model = if kind == ARMO {
        zstring(MOD2).or_else(|| zstring(sig::MODL))
    } else {
        zstring(sig::MODL)
    };
    let parts = if kind == SCOL {
        collection_parts(order, rr.plugin, &record)
    } else {
        Vec::new()
    };
    let light = if kind == LIGH {
        Light::parse(&record)
    } else {
        None
    };
    Some(BaseInfo {
        kind,
        editor_id: record.editor_id(),
        model,
        parts,
        light,
    })
}

/// A static collection lists each piece (`ONAM`) followed by its
/// placements (`DATA`, 28 bytes each: position, rotation, scale).
fn collection_parts(order: &LoadOrder, plugin: &LoadedPlugin, record: &Record) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut current: Option<(FormId, Option<String>)> = None;
    for s in &record.subrecords {
        if s.kind == ONAM && s.data.len() >= 4 {
            let id = plugin.to_global(FormId(le_u32(&s.data, 0)));
            let model = order
                .get(id)
                .and_then(|r| r.record().ok())
                .and_then(|r| r.get(sig::MODL).map(|m| m.zstring()));
            current = Some((id, model));
        } else if s.kind == sig::DATA {
            let Some((id, model)) = &current else {
                continue;
            };
            for chunk in s.data.chunks_exact(28) {
                let f = |i: usize| le_f32(chunk, i * 4);
                parts.push(Part {
                    base: *id,
                    model: model.clone(),
                    position: [f(0), f(1), f(2)],
                    rotation: [f(3), f(4), f(5)],
                    scale: f(6),
                });
            }
        }
    }
    parts
}

/// Editor-only markers: the engine's built-in marker objects (statics in
/// FalloutNV.esm below 0x800, such as XMarker and the door and room markers),
/// and anything using a marker model.
pub fn is_marker(base: FormId, kind: FourCC, model: Option<&str>) -> bool {
    if base.mod_index() == 0 && base.object_id() < 0x800 && kind == STAT {
        return true;
    }
    model.is_some_and(|m| {
        let path = m.to_ascii_lowercase().replace('/', "\\");
        let file = path.rsplit('\\').next().unwrap_or(&path);
        file.starts_with("marker") || path.starts_with("markers\\") || path.contains("\\markers\\")
    })
}

/// Where the player can arrive: a `COCMarkerHeading` (where the `coc`
/// console command puts you), then the arrival point of each load door
/// leading here, read from the door on the other side.
fn arrivals(order: &LoadOrder, cell: &LoadedCell) -> Vec<Arrival> {
    let mut out = Vec::new();
    for marker in &cell.markers {
        if marker
            .base_editor_id
            .as_deref()
            .is_some_and(|e| e.eq_ignore_ascii_case("COCMarkerHeading"))
        {
            out.push(Arrival {
                position: marker.position,
                rotation: marker.rotation,
                via: "the coc marker".into(),
            });
        }
    }
    for door in cell.objects.iter().filter(|p| p.teleport.is_some()) {
        let far_door = door.teleport.unwrap().door;
        let Some(frr) = order.get(far_door) else {
            continue;
        };
        let Ok(frecord) = frr.record() else {
            continue;
        };
        let Some(back) = read_teleport(frr.plugin, &frecord) else {
            continue;
        };
        if back.door != door.form_id {
            continue;
        }
        let from = order
            .cell_of(&frr)
            .and_then(|c| cell_info(order, c).ok())
            .map(|c| match (c.interior, c.editor_id, c.world) {
                (true, Some(e), _) | (false, Some(e), None) => e,
                (false, name, Some(w)) => {
                    let world = order
                        .get(w)
                        .and_then(|r| r.editor_id().ok().flatten())
                        .unwrap_or_else(|| w.to_string());
                    match (name, c.grid) {
                        (Some(n), _) => format!("{n} in {world}"),
                        (None, Some((x, y))) => format!("{world} {x},{y}"),
                        (None, None) => world,
                    }
                }
                (_, None, _) => c.form_id.to_string(),
            })
            .unwrap_or_else(|| "another cell".into());
        out.push(Arrival {
            position: back.position,
            rotation: back.rotation,
            via: format!("door {} from {from}", door.form_id),
        });
    }
    out
}
