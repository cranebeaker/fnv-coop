//! What a hit looks and sounds like: impact effects, blood, and the sounds
//! people and creatures make when hurt or killed. Read from the game's code
//! (`%USERPROFILE%\nv-re\findings\hiteffects.md`, decompiled in
//! `%USERPROFILE%\nv-re\decomp\hiteffects`) and its records.
//!
//! **Records.** An impact (`IPCT`, loader `0058dea0`): `MODL` the effect's
//! model, `DATA` (24 bytes) its duration, orientation (0 along the
//! surface's normal, 1 back along the shot, 2 the shot's reflection), angle
//! threshold, placement radius, sound level (0 loud, 1 normal, 2 silent:
//! noise for detection), flags (0x01 no decal); `DODT` the decal's sizes,
//! depth, shininess, parallax and colour; `DNAM` the decal's texture set
//! (`TXST`); `SNAM` and `NAM1` two sounds. An impact data set (`IPDS`,
//! `0058ec00`): `DATA`, one impact per **impact material** (the game's
//! names, table `0118c4a0`): stone, dirt, grass, glass, metal, wood,
//! organic, cloth, water, hollow metal, organic bug, organic glow.
//!
//! **Which material.** A surface's Havok material maps to one
//! ([`Material::from_havok`], `0058e8f0`). People and creatures are their
//! record's `NAM4` (organic unless it says otherwise: `00601570`,
//! `005f7a40`), except that power armour makes a hit metal: on the head
//! when the helmet is power armour, elsewhere when the body armour is
//! (`0088e8d0`, set from the worn armour's `BMDT` flag 0x20 in `0092baa0`).
//!
//! **Which set.** The weapon's (`WEAP` `INAM`); a creature biting, its own
//! (`CREA` `CNAM`); a person without a weapon, the fists' (`000001F4`,
//! made by the engine if missing). `DefaultImpactDataSet` (`00000276`) when
//! nothing else gives an impact.
//!
//! **A hit on someone** ([`actor_hit`]): sounds from `0088e1e0`, blood from
//! `0088e8d0`, the hurt line from `0089a760`; on death `0089d900`. **A hit
//! on the world** ([`surface_impact`]): `009c20e0`.
//!
//! The viewer plays the sounds, lines and the player's hit modifier, and
//! draws the effect models (`cellview::impacts`) and world decals
//! ([`crate::decals`]) these rules choose; decals on skin and screen blood
//! aren't drawn.

use esm::{FormId, FourCC, LoadOrder, Record, RecordRef};

use crate::cell::{le_f32, le_u32};
use crate::scripting::{base_of, game_setting, GameState};

const IPCT: FourCC = FourCC::new(b"IPCT");
const IPDS: FourCC = FourCC::new(b"IPDS");
const TXST: FourCC = FourCC::new(b"TXST");
const WEAP: FourCC = FourCC::new(b"WEAP");
const CREA: FourCC = FourCC::new(b"CREA");
const NPC_: FourCC = FourCC::new(b"NPC_");
const ARMO: FourCC = FourCC::new(b"ARMO");
const BPTD: FourCC = FourCC::new(b"BPTD");
const BMDT: FourCC = FourCC::new(b"BMDT");
const BPND: FourCC = FourCC::new(b"BPND");
const INAM: FourCC = FourCC::new(b"INAM");
const CNAM: FourCC = FourCC::new(b"CNAM");
const NAM4: FourCC = FourCC::new(b"NAM4");
const NAM1: FourCC = FourCC::new(b"NAM1");
const CSCR: FourCC = FourCC::new(b"CSCR");
const CSDT: FourCC = FourCC::new(b"CSDT");
const CSDI: FourCC = FourCC::new(b"CSDI");
const CSDC: FourCC = FourCC::new(b"CSDC");
const ACBS: FourCC = FourCC::new(b"ACBS");
const MODL: FourCC = FourCC::new(b"MODL");
const DODT: FourCC = FourCC::new(b"DODT");
const DNAM: FourCC = FourCC::new(b"DNAM");
const SNAM: FourCC = FourCC::new(b"SNAM");
const TX00: FourCC = FourCC::new(b"TX00");
const TX01: FourCC = FourCC::new(b"TX01");

/// The engine's fists (`[011ca278]`, form `000001F4`, set up in
/// `0046a370`): the weapon of a person hitting with none.
pub const FISTS: FormId = FormId(0x1F4);
/// `DefaultImpactDataSet` (`0058ea10`): used when a hit on someone finds no
/// impact of its own.
pub const DEFAULT_IMPACT_SET: FormId = FormId(0x276);
/// The image space modifier the player gets when hit (`GetHit`, looked up
/// by `005d2860`).
pub const GET_HIT_MODIFIER: FormId = FormId(0x162);
/// The combat topics said when hurt and when dying (the game's combat
/// topic table at `0119a238`: subtype 2 `Hit`, subtype 6 `Death`).
pub const HIT_TOPIC: FormId = FormId(0xDD);
pub const DEATH_TOPIC: FormId = FormId(0xEF);

/// `ACBS` flags: the base has neither blood spray (0x800) nor blood decals
/// (0x1000) when both are set (`0087f260` through `0060b1d0`, `0060b1f0`).
const NO_BLOOD_SPRAY: u32 = 0x800;
const NO_BLOOD_DECAL: u32 = 0x1000;
/// `ACBS` flag 0x100 on a creature: it has its own sound lists; without it
/// its `CSCR` creature's are used (`005f92d0`).
const OWN_SOUNDS: u32 = 0x100;
/// `BMDT` general flags (byte 4): power armour (`00480d10`).
const POWER_ARMOUR: u8 = 0x20;
/// `BMDT` slots: head and hair (`00480af0(0)`, `(1)` in `0092baa0`).
const HEAD_SLOTS: u32 = 0x01 | 0x02;

/// An impact material (`IPDS` order; the game's names). People and
/// creatures are organic unless their record says otherwise.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Material {
    Stone = 0,
    Dirt,
    Grass,
    Glass,
    Metal,
    Wood,
    #[default]
    Organic,
    Cloth,
    Water,
    HollowMetal,
    OrganicBug,
    OrganicGlow,
}

impl Material {
    pub const ALL: [Material; 12] = [
        Material::Stone,
        Material::Dirt,
        Material::Grass,
        Material::Glass,
        Material::Metal,
        Material::Wood,
        Material::Organic,
        Material::Cloth,
        Material::Water,
        Material::HollowMetal,
        Material::OrganicBug,
        Material::OrganicGlow,
    ];

    pub fn from_index(i: u32) -> Option<Material> {
        Material::ALL.get(i as usize).copied()
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// The game's name (its table at `0118c4a0`).
    pub fn name(self) -> &'static str {
        [
            "Stone",
            "Dirt",
            "Grass",
            "Glass",
            "Metal",
            "Wood",
            "Organic",
            "Cloth",
            "Water",
            "Hollow Metal",
            "Organic Bug",
            "Organic Glow",
        ][self.index()]
    }

    /// A material by name (any case, spaces optional) or number.
    pub fn parse(text: &str) -> Option<Material> {
        let squash = |s: &str| s.replace(' ', "").to_ascii_lowercase();
        if let Ok(i) = text.parse::<u32>() {
            return Material::from_index(i);
        }
        Material::ALL
            .into_iter()
            .find(|m| squash(m.name()) == squash(text))
    }

    /// The impact material of a Havok material (the struck shape's, kept to
    /// its low five bits by `00c84f10`), as `0058e8f0` maps them: stone,
    /// heavy stone and broken concrete stone; cloth; dirt and sand; glass
    /// and bottles; grass; organic and skin organic; water; wood, heavy
    /// wood, baby rattles and rubber balls wood; hollow and sheet metal,
    /// hollow vehicle parts, barrels, soda cans, shopping carts and
    /// lunchboxes hollow metal; everything else metal.
    pub fn from_havok(havok: u32) -> Material {
        match havok {
            0 | 10 | 19 => Material::Stone,
            1 => Material::Cloth,
            2 | 18 => Material::Dirt,
            3 | 24 => Material::Glass,
            4 => Material::Grass,
            6 | 7 => Material::Organic,
            8 => Material::Water,
            9 | 12 | 30 | 31 => Material::Wood,
            16 | 17 | 22 | 23 | 25 | 28 | 29 => Material::HollowMetal,
            _ => Material::Metal,
        }
    }
}

/// Which way an impact's effect points (`IPCT` `DATA` u32 at 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    SurfaceNormal,
    /// Back along the shot.
    ProjectileVector,
    /// The shot's reflection off the surface.
    ProjectileReflection,
}

impl Orientation {
    fn from_u32(v: u32) -> Orientation {
        match v {
            1 => Orientation::ProjectileVector,
            2 => Orientation::ProjectileReflection,
            // The code's other case uses the normal too (`009c20e0`).
            _ => Orientation::SurfaceNormal,
        }
    }
}

/// An impact's (or texture set's) decal (`DODT`, 36 bytes, `00593450`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decal {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub depth: f32,
    pub shininess: f32,
    pub parallax_scale: f32,
    pub parallax_passes: u8,
    /// 0x01 parallax, 0x02 alpha blending, 0x04 alpha testing.
    pub flags: u8,
    pub color: [u8; 4],
}

impl Decal {
    pub const PARALLAX: u8 = 0x01;
    pub const ALPHA_BLENDING: u8 = 0x02;
    pub const ALPHA_TESTING: u8 = 0x04;

    pub fn parse(d: &[u8]) -> Option<Decal> {
        if d.len() < 36 {
            return None;
        }
        Some(Decal {
            min_width: le_f32(d, 0),
            max_width: le_f32(d, 4),
            min_height: le_f32(d, 8),
            max_height: le_f32(d, 12),
            depth: le_f32(d, 16),
            shininess: le_f32(d, 20),
            parallax_scale: le_f32(d, 24),
            parallax_passes: d[28],
            flags: d[29],
            color: [d[32], d[33], d[34], d[35]],
        })
    }

    /// The size the game gives a decal: its width U(min, max), used for
    /// both sides (`0088e8d0`, `009c20e0`: the same value twice), × 1.5
    /// when the attacker has Bloody Mess (blood only). `t` is 0..1.
    pub fn size(&self, t: f32, bloody_mess: bool) -> f32 {
        let s = self.min_width + (self.max_width - self.min_width) * t;
        if bloody_mess {
            s * 1.5
        } else {
            s
        }
    }
}

/// An impact (`IPCT`).
#[derive(Debug, Clone, PartialEq)]
pub struct Impact {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// The effect's model (relative to `meshes\`).
    pub model: Option<String>,
    /// Seconds (the effect's lifetime when its model doesn't animate).
    pub duration: f32,
    pub orientation: Orientation,
    pub angle_threshold: f32,
    pub placement_radius: f32,
    /// 0 loud, 1 normal, 2 silent.
    pub sound_level: u32,
    /// 0x01: no decal.
    pub flags: u32,
    pub decal: Option<Decal>,
    pub texture_set: Option<FormId>,
    pub sound: Option<FormId>,
    pub sound2: Option<FormId>,
}

fn form_in(rr: &RecordRef<'_>, record: &Record, kind: FourCC) -> Option<FormId> {
    record
        .get(kind)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .filter(|f| f.0 != 0)
}

impl Impact {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Impact> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == IPCT)?;
        let record = rr.record().ok()?;
        let data = record
            .get(esm::sig::DATA)
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let f = |at: usize| (data.len() >= at + 4).then(|| le_f32(&data, at));
        let u = |at: usize| (data.len() >= at + 4).then(|| le_u32(&data, at));
        Some(Impact {
            form_id: id,
            editor_id: record.editor_id(),
            model: record
                .get(MODL)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty()),
            duration: f(0).unwrap_or(0.0),
            orientation: Orientation::from_u32(u(4).unwrap_or(0)),
            angle_threshold: f(8).unwrap_or(0.0),
            placement_radius: f(12).unwrap_or(0.0),
            sound_level: u(16).unwrap_or(1),
            flags: u(20).unwrap_or(0),
            decal: record.get(DODT).and_then(|s| Decal::parse(&s.data)),
            texture_set: form_in(&rr, &record, DNAM),
            sound: form_in(&rr, &record, SNAM),
            sound2: form_in(&rr, &record, NAM1),
        })
    }

    /// Its two sounds (`SNAM`, then `NAM1`).
    pub fn sounds(&self) -> impl Iterator<Item = FormId> + '_ {
        self.sound.into_iter().chain(self.sound2)
    }

    /// Whether it leaves a decal: a texture set, and not flagged "no decal".
    pub fn has_decal(&self) -> bool {
        self.texture_set.is_some() && self.flags & 0x01 == 0
    }
}

/// An impact data set (`IPDS`).
#[derive(Debug, Clone, PartialEq)]
pub struct ImpactSet {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// By [`Material`].
    pub impacts: [Option<FormId>; 12],
}

impl ImpactSet {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<ImpactSet> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == IPDS)?;
        let record = rr.record().ok()?;
        let data = record
            .get(esm::sig::DATA)
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let mut impacts = [None; 12];
        for (i, slot) in impacts.iter_mut().enumerate() {
            if data.len() >= i * 4 + 4 {
                let f = FormId(le_u32(&data, i * 4));
                *slot = (f.0 != 0).then(|| rr.plugin.to_global(f));
            }
        }
        Some(ImpactSet {
            form_id: id,
            editor_id: record.editor_id(),
            impacts,
        })
    }

    /// The impact for a material (`0058e9d0`).
    pub fn get(&self, material: Material) -> Option<FormId> {
        self.impacts[material.index()]
    }
}

/// The impact a set gives for a material, if the set exists.
pub fn impact_in(order: &LoadOrder, set: FormId, material: Material) -> Option<FormId> {
    ImpactSet::load(order, set)?.get(material)
}

/// A texture set (`TXST`): what a decal draws.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureSet {
    pub form_id: FormId,
    /// `TX00` and `TX01`, relative to `textures\`.
    pub diffuse: Option<String>,
    pub normal: Option<String>,
    pub decal: Option<Decal>,
}

impl TextureSet {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<TextureSet> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == TXST)?;
        let record = rr.record().ok()?;
        let text = |kind| {
            record
                .get(kind)
                .map(|s| s.zstring())
                .filter(|s| !s.is_empty())
        };
        Some(TextureSet {
            form_id: id,
            diffuse: text(TX00),
            normal: text(TX01),
            decal: record.get(DODT).and_then(|s| Decal::parse(&s.data)),
        })
    }
}

/// A weapon's impact data set (`WEAP` `INAM`, `TESObjectWEAP+0x24c`).
pub fn weapon_impact_set(order: &LoadOrder, weapon: FormId) -> Option<FormId> {
    let rr = order.get(weapon).filter(|r| r.entry.header.kind == WEAP)?;
    form_in(&rr, &rr.record().ok()?, INAM)
}

/// The record an actor's look-related data comes from: a templated actor
/// takes it from its template when "use model/animation" (0x40) is set (a
/// guess at which template flag covers the impact material, impact set and
/// sounds: the game's own split isn't traced; the spawned creatures, with
/// every flag set, are the same either way).
fn look_record(order: &LoadOrder, who: FormId) -> Option<(RecordRef<'_>, std::sync::Arc<Record>)> {
    let is_base = order
        .get(who)
        .is_some_and(|r| r.entry.header.kind == NPC_ || r.entry.header.kind == CREA);
    let base = if is_base { who } else { base_of(order, who)? };
    crate::actor::data_record(order, base, 0x40)
}

/// A creature's own impact data set, for its bites (`CREA` `CNAM`).
pub fn creature_impact_set(order: &LoadOrder, who: FormId) -> Option<FormId> {
    let (rr, record) = look_record(order, who)?;
    (rr.entry.header.kind == CREA)
        .then(|| form_in(&rr, &record, CNAM))
        .flatten()
}

/// What someone is made of when hit: their record's `NAM4`, organic when
/// it has none (the default both loaders set).
pub fn actor_material(order: &LoadOrder, who: FormId) -> Material {
    look_record(order, who)
        .and_then(|(_, record)| {
            let s = record.get(NAM4).filter(|s| s.data.len() >= 4)?;
            Material::from_index(le_u32(&s.data, 0))
        })
        .unwrap_or(Material::Organic)
}

/// What someone wears: what the game's state says they have equipped,
/// else (people who haven't changed) the armour their record gives them,
/// the first per body slot (the look's guess, `world::actor`).
pub fn worn_armour(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<FormId> {
    if let Some(list) = state.equipped.get(&who) {
        return list.clone();
    }
    let base = if who == crate::dialogue::PLAYER_REF {
        crate::dialogue::PLAYER_BASE
    } else {
        match base_of(order, who) {
            Some(b) => b,
            None => return Vec::new(),
        }
    };
    let mut covered = 0u32;
    let mut out = Vec::new();
    for (item, _) in crate::actor::record_inventory(order, base) {
        if let Some(a) = crate::actor::Armor::load(order, item) {
            if a.slots & covered == 0 {
                covered |= a.slots;
                out.push(item);
            }
        }
    }
    out
}

/// Power armour worn (`0092baa0`, the process's bytes `+0x128`/`+0x129`):
/// a power armour piece (`BMDT` general flag 0x20) covering the head or
/// hair counts as the helmet, any other as the body.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PowerArmour {
    pub body: bool,
    pub helmet: bool,
}

pub fn power_armour(order: &LoadOrder, state: &GameState, who: FormId) -> PowerArmour {
    let mut out = PowerArmour::default();
    for item in worn_armour(order, state, who) {
        let Some(rr) = order.get(item).filter(|r| r.entry.header.kind == ARMO) else {
            continue;
        };
        let Some(b) = rr
            .record()
            .ok()
            .and_then(|r| r.get(BMDT).map(|s| s.data.clone()))
        else {
            continue;
        };
        if b.len() < 5 || b[4] & POWER_ARMOUR == 0 {
            continue;
        }
        if le_u32(&b, 0) & HEAD_SLOTS != 0 {
            out.helmet = true;
        } else {
            out.body = true;
        }
    }
    out
}

/// Whether a hit on a part lands on power armour (`0088e8d0`): a head
/// part (1 or 2) on a power armour helmet, any other part (or none) on
/// power armour on the body.
pub fn on_power_armour(
    order: &LoadOrder,
    state: &GameState,
    target: FormId,
    part: Option<u8>,
) -> bool {
    let armour = power_armour(order, state, target);
    if matches!(part, Some(1 | 2)) {
        armour.helmet
    } else {
        armour.body
    }
}

/// What a hit on someone strikes, for its blood and decals (`0088e8d0`):
/// metal on power armour ([`on_power_armour`]), else their own material.
pub fn hit_material(
    order: &LoadOrder,
    state: &GameState,
    target: FormId,
    part: Option<u8>,
) -> Material {
    if on_power_armour(order, state, target, part) {
        Material::Metal
    } else {
        actor_material(order, target)
    }
}

/// Whether hits on someone show blood (`0087f260`): not when gore is off
/// (`[General] bDisableAllGore`), nor when their record has both "no blood
/// spray" and "no blood decal".
pub fn shows_blood(order: &LoadOrder, who: FormId, gore_off: bool) -> bool {
    if gore_off {
        return false;
    }
    let flags = look_record(order, who)
        .and_then(|(_, r)| {
            r.get(ACBS)
                .filter(|s| s.data.len() >= 4)
                .map(|s| le_u32(&s.data, 0))
        })
        .unwrap_or(0);
    !(flags & NO_BLOOD_SPRAY != 0 && flags & NO_BLOOD_DECAL != 0)
}

/// A body part's own impact data set: `BPND` form at 68 ("severable
/// impact data set", `+0xa0` of the part, `004fd400`), of the part of
/// that type in the body part data someone uses. A later part of a type
/// replaces an earlier one, as the loader keeps them.
pub fn body_part_impact_set(order: &LoadOrder, who: FormId, part: u8) -> Option<FormId> {
    let data = crate::body_parts::data_form(order, who)?;
    let rr = order.get(data).filter(|r| r.entry.header.kind == BPTD)?;
    let record = rr.record().ok()?;
    let mut found = None;
    for s in record.get_all(BPND).filter(|s| s.data.len() >= 84) {
        if s.data[5] == part {
            let f = FormId(le_u32(&s.data, 68));
            found = (f.0 != 0).then(|| rr.plugin.to_global(f));
        }
    }
    found
}

/// Creature sound kinds (`CSDT`; the game's 22 lists, `005f0ac0`).
pub mod creature_sound {
    pub const LEFT_FOOT: u32 = 0;
    pub const IDLE: u32 = 4;
    pub const AWARE: u32 = 5;
    pub const ATTACK: u32 = 6;
    /// Listed in records, but nothing in the engine plays it: every caller
    /// of the picker (`005f92d0`) asks for another kind.
    pub const HIT: u32 = 7;
    pub const DEATH: u32 = 8;
    /// Played with a creature's own blows (`0088e1e0`).
    pub const WEAPON: u32 = 9;
    pub const COUNT: u32 = 22;
}

/// A creature's sounds of a kind, in order, with their chances (percent):
/// its own (`CSDT`, `CSDI`, `CSDC`) when its `ACBS` flags have 0x100, else
/// its `CSCR` creature's, followed further (`005f92d0`).
pub fn creature_sounds(order: &LoadOrder, who: FormId, kind: u32) -> Vec<(FormId, u8)> {
    let mut at = match look_record(order, who) {
        Some(x) => x,
        None => return Vec::new(),
    };
    for _ in 0..8 {
        let (rr, record) = &at;
        if rr.entry.header.kind != CREA {
            return Vec::new();
        }
        let flags = record
            .get(ACBS)
            .filter(|s| s.data.len() >= 4)
            .map_or(0, |s| le_u32(&s.data, 0));
        if flags & OWN_SOUNDS == 0 {
            let Some(next) = form_in(rr, record, CSCR) else {
                return Vec::new();
            };
            let Some(r) = order.get(next) else {
                return Vec::new();
            };
            let Ok(rec) = r.record_shared() else {
                return Vec::new();
            };
            at = (r, rec);
            continue;
        }
        let mut out = Vec::new();
        let mut current: Option<u32> = None;
        let mut pending: Option<FormId> = None;
        for s in &record.subrecords {
            if s.kind == CSDT && s.data.len() >= 4 {
                current = Some(le_u32(&s.data, 0));
            } else if s.kind == CSDI && s.data.len() >= 4 && current == Some(kind) {
                pending = Some(rr.plugin.to_global(FormId(le_u32(&s.data, 0))));
            } else if s.kind == CSDC && current == Some(kind) {
                if let Some(id) = pending.take() {
                    out.push((id, s.data.first().copied().unwrap_or(0)));
                }
            }
        }
        if let Some(id) = pending {
            out.push((id, 0));
        }
        return out;
    }
    Vec::new()
}

/// The sound a creature makes of a kind (`005f0a30`): its list in order,
/// the first whose roll (`rand % 100`) is below its chance; none if none
/// passes. `roll` gives a fresh random number each time.
pub fn creature_sound(
    order: &LoadOrder,
    who: FormId,
    kind: u32,
    roll: &mut dyn FnMut() -> u64,
) -> Option<FormId> {
    creature_sounds(order, who, kind)
        .into_iter()
        .find(|&(id, chance)| id.0 != 0 && roll() % 100 < u64::from(chance))
        .map(|(id, _)| id)
}

/// The impact data set behind a blow (`0088e1e0`): the weapon's; with
/// none, a creature's own (and its "weapon" sounds are heard too), a
/// person's fists'.
pub fn blow_impact_set(
    order: &LoadOrder,
    attacker: FormId,
    weapon: Option<FormId>,
) -> (Option<FormId>, bool) {
    match weapon {
        Some(w) => (weapon_impact_set(order, w), false),
        None if crate::combat::is_creature(order, attacker) => {
            (creature_impact_set(order, attacker), true)
        }
        None => (weapon_impact_set(order, FISTS), false),
    }
}

/// What a hit on a person or creature does besides its damage.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActorHitEffects {
    /// What the blood and decals strike ([`hit_material`]).
    pub material: Material,
    /// Sounds to play at the point, each only if the listener is nearer
    /// than that sound's largest distance (`0088e1e0`).
    pub sounds: Vec<FormId>,
    /// The impact whose model sprays at the point and whose decal goes on
    /// them (none when they show no blood).
    pub blood: Option<FormId>,
    /// The impact whose decal may go on a wall behind them: their body
    /// part's own set ([`body_part_impact_set`]), not on power armour.
    pub spatter: Option<FormId>,
}

/// What `attacker` hitting `target` (with `weapon`, the hit's; `held`,
/// what the attacker holds; `part` where it landed) plays and shows, as
/// the game's hit handler does (`0089a760`):
///
/// - sounds (`0088e1e0`): the blow's set ([`blow_impact_set`]) at the
///   target's own material, its `SNAM` and `NAM1`; a creature's bite adds
///   its "weapon" sound ([`creature_sound`]); power armour on the body adds
///   the set's metal impact's two sounds;
/// - blood (`0088e8d0`, when [`shows_blood`]): the impact the attacker's
///   held weapon's set gives at [`hit_material`], else the default set's
///   ([`DEFAULT_IMPACT_SET`]); and the body part's own set's impact at that
///   material for a spatter on the wall, unless power armour took the hit.
#[allow(clippy::too_many_arguments)]
pub fn actor_hit(
    order: &LoadOrder,
    state: &GameState,
    attacker: FormId,
    target: FormId,
    weapon: Option<FormId>,
    held: Option<FormId>,
    part: Option<u8>,
    gore_off: bool,
    roll: &mut dyn FnMut() -> u64,
) -> ActorHitEffects {
    let own = actor_material(order, target);
    let (set, bite) = blow_impact_set(order, attacker, weapon);
    let mut sounds = Vec::new();
    if let Some(i) = set
        .and_then(|s| impact_in(order, s, own))
        .and_then(|i| Impact::load(order, i))
    {
        sounds.extend(i.sounds());
    }
    if bite {
        sounds.extend(creature_sound(
            order,
            attacker,
            creature_sound::WEAPON,
            roll,
        ));
    }
    let armour = power_armour(order, state, target);
    if armour.body {
        if let Some(i) = set
            .and_then(|s| impact_in(order, s, Material::Metal))
            .and_then(|i| Impact::load(order, i))
        {
            sounds.extend(i.sounds());
        }
    }
    let on_armour = on_power_armour(order, state, target, part);
    let material = if on_armour { Material::Metal } else { own };
    let mut out = ActorHitEffects {
        material,
        sounds,
        blood: None,
        spatter: None,
    };
    if !shows_blood(order, target, gore_off) {
        return out;
    }
    let from_held = held
        .and_then(|w| weapon_impact_set(order, w))
        .and_then(|s| impact_in(order, s, material));
    out.blood = from_held.or_else(|| impact_in(order, DEFAULT_IMPACT_SET, material));
    if !on_armour {
        out.spatter = part
            .and_then(|p| body_part_impact_set(order, target, p))
            .and_then(|s| impact_in(order, s, material))
            .filter(|&i| Impact::load(order, i).is_some_and(|i| i.texture_set.is_some()));
    }
    out
}

/// The impact a shot leaves on the world (`009c20e0`): the weapon's set at
/// the struck surface's material.
pub fn surface_impact(order: &LoadOrder, weapon: FormId, material: Material) -> Option<FormId> {
    impact_in(order, weapon_impact_set(order, weapon)?, material)
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-9 {
        v.map(|c| c / l)
    } else {
        [0.0, 0.0, 1.0]
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Which way a world impact's effect points (`009c20e0`): the surface's
/// normal; back along the shot; or the reflection `2(d·n)n − d` of `d` =
/// back along the shot.
pub fn effect_axis(orientation: Orientation, normal: [f32; 3], travel: [f32; 3]) -> [f32; 3] {
    let n = normalize(normal);
    let back = normalize(travel.map(|c| -c));
    match orientation {
        Orientation::SurfaceNormal => n,
        Orientation::ProjectileVector => back,
        Orientation::ProjectileReflection => {
            let k = 2.0 * (back[0] * n[0] + back[1] * n[1] + back[2] * n[2]);
            normalize([k * n[0] - back[0], k * n[1] - back[1], k * n[2] - back[2]])
        }
    }
}

/// Blood's spray direction on someone hit (`0088e8d0`): normalize(2 × the
/// hit's direction + `jitter`), the jitter U(−0.8, 0.8) on each axis.
pub fn spray_direction(direction: [f32; 3], jitter: [f32; 3]) -> [f32; 3] {
    let d = normalize(direction);
    normalize([
        2.0 * d[0] + jitter[0],
        2.0 * d[1] + jitter[1],
        2.0 * d[2] + jitter[2],
    ])
}

/// How far a wall spatter looks behind someone hit (`0088e8d0`).
pub const SPATTER_REACH: f32 = 512.0;

/// The way a wall spatter looks (`0088e8d0`): the spray tipped down by
/// half a unit and normalized (then followed [`SPATTER_REACH`] units).
pub fn spatter_direction(spray: [f32; 3]) -> [f32; 3] {
    normalize([spray[0], spray[1], spray[2] - 0.5])
}

/// A temporary effect's rotation (`00c4b8a0`, `004a0c90`), as a matrix
/// (`m[row][col]`, taking the model's axes to the world's): its Z axis
/// along `axis`, X and Y from the axis least like it (`(−y, x, 0)` when z
/// is the smallest component, `(−z, 0, x)` when y is, `(0, −z, y)` when x
/// is; Y = that × axis, X = Y × axis), then turned about its own Z by
/// `roll` radians the way the game's matrices turn (`[[c, s, 0], [−s, c,
/// 0], [0, 0, 1]]`). The game rolls by U(0, 1) radians (`004a4240`'s 0..1
/// value straight into the sine and cosine). Turning about the effect's
/// own axis (rather than the world's) is inferred.
pub fn effect_rotation(axis: [f32; 3], roll: f32) -> [[f32; 3]; 3] {
    let d = normalize(axis);
    let (ax, ay, az) = (d[0].abs(), d[1].abs(), d[2].abs());
    let u = if ay < ax || az < ax {
        if ax < ay || az < ay {
            [-d[1], d[0], 0.0]
        } else {
            [-d[2], 0.0, d[0]]
        }
    } else {
        [0.0, -d[2], d[1]]
    };
    let u = normalize(u);
    let y = normalize(cross(u, d));
    let x = normalize(cross(y, d));
    let base = [[x[0], y[0], d[0]], [x[1], y[1], d[1]], [x[2], y[2], d[2]]];
    let (s, c) = roll.sin_cos();
    let r = [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]];
    let mut out = [[0.0f32; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = (0..3).map(|k| base[i][k] * r[k][j]).sum();
        }
    }
    out
}

/// How long a temporary effect stays (`00689310`, flag 4): its model's
/// animation time when it has one (at least 0.1 s), else `given` (an
/// impact's duration on the world, 1 s for blood).
pub fn effect_lifetime(animation: Option<f32>, given: f32) -> f32 {
    match animation {
        Some(t) if t >= 0.0 => t.max(0.1),
        _ => given,
    }
}

/// How many blood splatters the player's screen gets when hit (`00647ae0`):
/// `max(0, trunc(fBloodSplatterCountBase + damage ×
/// fBloodSplatterCountDamageMult + fBloodSplatterCountDamageBase + r))`,
/// `r` U(−margin, margin) with `fBloodSplatterCountRandomMargin` (exe 2,
/// 0.1, 0, 1). `t` is 0..1.
pub fn screen_blood_count(order: &LoadOrder, damage: f32, t: f32) -> u32 {
    let s = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
    let margin = s("fBloodSplatterCountRandomMargin", 1.0);
    let r = -margin + 2.0 * margin * t;
    let n = s("fBloodSplatterCountBase", 2.0)
        + damage * s("fBloodSplatterCountDamageMult", 0.1)
        + s("fBloodSplatterCountDamageBase", 0.0)
        + r;
    (n.trunc() as i64).max(0) as u32
}

/// The screen blood's settings (`004deb30`, `004df040`, `004e0330`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenBlood {
    pub duration: f32,
    pub fade_start: f32,
    pub min_size: f32,
    pub max_size: f32,
    pub min_opacity: f32,
    pub max_opacity: f32,
    pub max_count: u32,
}

impl ScreenBlood {
    pub fn load(order: &LoadOrder) -> ScreenBlood {
        let s = |name: &str, default: f32| game_setting(order, name).unwrap_or(default);
        ScreenBlood {
            duration: s("fBloodSplatterDuration", 5.0),
            fade_start: s("fBloodSplatterFadeStart", 0.8),
            min_size: s("fBloodSplatterMinSize", 0.025),
            max_size: s("fBloodSplatterMaxSize", 0.125),
            min_opacity: s("fBloodSplatterMinOpacity", 0.1),
            max_opacity: s("fBloodSplatterMaxOpacity", 1.0),
            max_count: s("iBloodSplatterMaxCount", 10.0).max(0.0) as u32,
        }
    }
}

/// Whether someone hit says their "hurt" line (`0089a760`): when knocked
/// out, or when the hit took more than `fCombatSpeakHitThreshold` (data
/// 0.01) of their health (health as a whole number), else with chance
/// `fCombatSpeakHitChance` (data 0.01). `t` is 0..1.
pub fn says_hurt_line(
    order: &LoadOrder,
    damage: f32,
    health: f32,
    knocked_out: bool,
    t: f32,
) -> bool {
    if knocked_out {
        return true;
    }
    let whole = health.trunc();
    let share = if whole != 0.0 {
        damage / whole
    } else {
        f32::INFINITY
    };
    let threshold = game_setting(order, "fCombatSpeakHitThreshold").unwrap_or(0.1);
    share > threshold || t < game_setting(order, "fCombatSpeakHitChance").unwrap_or(0.35)
}

/// The two clocks combat lines wait on (milliseconds, the game's
/// `GetTickCount`): the hurt lines' shared cooldown (`[011df684]`) and the
/// last combat line anyone said (`[011f170c]`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CombatVoice {
    pub hurt_ready_at: u64,
    pub last_line_at: Option<u64>,
}

/// No combat line within this many milliseconds of the last (`009839b0`).
pub const COMBAT_LINE_GAP_MS: u64 = 1500;

impl CombatVoice {
    /// A hit that would make someone say their hurt line (`0089a760`):
    /// allowed only once the cooldown has run out; whenever a hit comes
    /// after it ran out, it starts again with a random length between
    /// `[Audio] fDialogHitSoundCooldownMin` and `…Max` (2, 4 s; `pick`
    /// gives a whole number of milliseconds in the range it's given).
    pub fn hurt_allowed(
        &mut self,
        now: u64,
        min_s: f32,
        max_s: f32,
        pick: &mut dyn FnMut(u64, u64) -> u64,
    ) -> bool {
        let allowed = now >= self.hurt_ready_at;
        if now > self.hurt_ready_at {
            let lo = (min_s * 1000.0).trunc().max(0.0) as u64;
            let hi = (max_s * 1000.0).trunc().max(0.0) as u64;
            self.hurt_ready_at = now + pick(lo, hi.max(lo));
        }
        allowed
    }

    /// Whether a combat line can be said now, and if so it counts as said
    /// (`009839b0`: at least 1.5 s after the last one anyone said).
    pub fn line_allowed(&mut self, now: u64) -> bool {
        if self
            .last_line_at
            .is_some_and(|t| now.wrapping_sub(t) < COMBAT_LINE_GAP_MS)
        {
            return false;
        }
        self.last_line_at = Some(now);
        true
    }
}

/// What someone dying sounds like (`0089d900`): a creature its "death"
/// sound; a person (or a creature without one) their `Death` line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathCry {
    Sound(FormId),
    Line,
}

pub fn death_cry(order: &LoadOrder, who: FormId, roll: &mut dyn FnMut() -> u64) -> DeathCry {
    if crate::combat::is_creature(order, who) {
        if let Some(s) = creature_sound(order, who, creature_sound::DEATH, roll) {
            return DeathCry::Sound(s);
        }
    }
    DeathCry::Line
}

/// How far from the player a death is heard at all (`fDeathSoundMaxDistance`,
/// exe 1000; the game tests it only in some states, `[011dea2a]`).
pub fn death_sound_distance(order: &LoadOrder) -> f32 {
    game_setting(order, "fDeathSoundMaxDistance").unwrap_or(1000.0)
}

/// Blood, sprays and impact effects are made only for hits within
/// `fGunParticleCameraDistance` (data 2048) of the camera (`0088e8d0`,
/// `009c20e0`; beyond it the game tests whether the point is in view, and
/// only effects out of view are made for hits on people).
pub fn effect_distance(order: &LoadOrder) -> f32 {
    game_setting(order, "fGunParticleCameraDistance").unwrap_or(1024.0)
}

/// The image space modifier the player gets when hit, and its strength
/// (`fGetHitPainMult`, exe 1.5).
pub fn get_hit_modifier(order: &LoadOrder) -> (FormId, f32) {
    (
        GET_HIT_MODIFIER,
        game_setting(order, "fGetHitPainMult").unwrap_or(1.5),
    )
}

/// A modifier's values applied at a strength (the game's modifier
/// instances, `00531720`, `00b8cb30`): each image space value goes only
/// `strength` of the way from what it was to `value × multiply + add`
/// (so multiply `1 + s(m − 1)`, add `s·a`); the tint's and the fade's
/// amounts × strength, at most 1; blur × strength, at most 7; double
/// vision × strength. (Radial blur and depth of field, also scaled, aren't
/// in [`crate::modifier::ModifierValues`].)
pub fn with_strength(
    values: &crate::modifier::ModifierValues,
    strength: f32,
) -> crate::modifier::ModifierValues {
    let s = strength;
    let mut out = values.clone();
    for m in &mut out.multiply {
        *m = 1.0 + s * (*m - 1.0);
    }
    for a in &mut out.add {
        *a *= s;
    }
    out.tint[3] = (values.tint[3] * s).clamp(0.0, 1.0);
    out.fade[3] = (values.fade[3] * s).clamp(0.0, 1.0);
    out.blur = (values.blur * s).clamp(0.0, 7.0);
    out.double_vision = values.double_vision * s;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modifiers_strength_scales_its_change() {
        let mut v = crate::modifier::ModifierValues::none();
        v.multiply[3] = 0.5;
        v.add[3] = 0.2;
        v.tint = [1.0, 0.0, 0.0, 0.8];
        v.fade[3] = 0.2;
        v.blur = 6.0;
        let s = with_strength(&v, 1.5);
        // An image space value of 1 goes 1.5 of the way to 0.7: to 0.55.
        assert!((1.0 * s.multiply[3] + s.add[3] - 0.55).abs() < 1e-6);
        assert_eq!(s.tint, [1.0, 0.0, 0.0, 1.0]);
        assert!((s.fade[3] - 0.3).abs() < 1e-6);
        assert_eq!(s.blur, 7.0);
        // Strength 1 changes nothing.
        assert_eq!(with_strength(&v, 1.0), v);
    }

    #[test]
    fn havok_materials_map_as_the_game_switches_them() {
        use Material::*;
        let expected = [
            Stone,
            Cloth,
            Dirt,
            Glass,
            Grass,
            Metal,
            Organic,
            Organic,
            Water,
            Wood,
            Stone,
            Metal,
            Wood,
            Metal,
            Metal,
            Metal,
            HollowMetal,
            HollowMetal,
            Dirt,
            Stone,
            Metal,
            Metal,
            HollowMetal,
            HollowMetal,
            Glass,
            HollowMetal,
            Metal,
            Metal,
            HollowMetal,
            HollowMetal,
            Wood,
            Wood,
        ];
        for (h, m) in expected.iter().enumerate() {
            assert_eq!(Material::from_havok(h as u32), *m, "Havok material {h}");
        }
        // Past the table: metal.
        assert_eq!(Material::from_havok(32), Metal);
        assert_eq!(Material::parse("hollow metal"), Some(HollowMetal));
        assert_eq!(Material::parse("6"), Some(Organic));
        assert_eq!(Material::OrganicGlow.name(), "Organic Glow");
    }

    #[test]
    fn decals_read_the_game_layout_and_size_by_width() {
        let mut d = Vec::new();
        for v in [10.0f32, 28.0, 8.0, 32.0, 16.0, 4.0, 0.04] {
            d.extend(v.to_le_bytes());
        }
        d.extend([1, 0x02, 0, 0, 255, 128, 64, 255]);
        let decal = Decal::parse(&d).unwrap();
        assert_eq!((decal.min_width, decal.max_width), (10.0, 28.0));
        assert_eq!((decal.min_height, decal.max_height), (8.0, 32.0));
        assert_eq!(decal.parallax_passes, 1);
        assert_eq!(decal.flags, Decal::ALPHA_BLENDING);
        assert_eq!(decal.color, [255, 128, 64, 255]);
        assert_eq!(decal.size(0.5, false), 19.0);
        assert_eq!(decal.size(0.5, true), 28.5);
        assert!(Decal::parse(&d[..35]).is_none());
    }

    #[test]
    fn effects_point_by_their_orientation() {
        let normal = [0.0, 0.0, 1.0];
        // A shot going north and down at 45°.
        let travel = [0.0, 1.0, -1.0];
        let n = effect_axis(Orientation::SurfaceNormal, normal, travel);
        assert_eq!(n, [0.0, 0.0, 1.0]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let back = effect_axis(Orientation::ProjectileVector, normal, travel);
        assert!((back[1] + h).abs() < 1e-3 && (back[2] - h).abs() < 1e-3);
        // Reflected: on north and up.
        let r = effect_axis(Orientation::ProjectileReflection, normal, travel);
        assert!((r[1] - h).abs() < 1e-3 && (r[2] - h).abs() < 1e-3, "{r:?}");
    }

    #[test]
    fn an_effects_z_axis_follows_its_direction_and_rolls_about_it() {
        let m = effect_rotation([1.0, 0.0, 0.0], 0.0);
        let col = |j: usize| [m[0][j], m[1][j], m[2][j]];
        assert_eq!(col(2), [1.0, 0.0, 0.0]);
        // x is the largest: u = (−y, x, 0) or (−z, 0, x) by the smaller of
        // y and z (equal here: the second test picks (−z, 0, x)).
        let x = col(0);
        let y = col(1);
        let dot = |a: [f32; 3], b: [f32; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        assert!(dot(x, y).abs() < 1e-5 && dot(x, col(2)).abs() < 1e-5);
        // A roll keeps the axis and turns X within the plane.
        let r = effect_rotation([1.0, 0.0, 0.0], 0.5);
        assert_eq!([r[0][2], r[1][2], r[2][2]], [1.0, 0.0, 0.0]);
        let rx = [r[0][0], r[1][0], r[2][0]];
        assert!((dot(rx, x) - 0.5f32.cos()).abs() < 1e-5);
        // Straight up: z the largest, u = (−y, x, 0) from the smaller x.
        let up = effect_rotation([0.0, 0.0, 1.0], 0.0);
        assert_eq!([up[0][2], up[1][2], up[2][2]], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn blood_sprays_on_and_spatters_lower() {
        let s = spray_direction([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        assert_eq!(s, [1.0, 0.0, 0.0]);
        let s = spray_direction([1.0, 0.0, 0.0], [0.0, 0.8, 0.0]);
        assert!((s[1] - 0.8 / (4.0f32 + 0.64).sqrt()).abs() < 1e-5);
        let d = spatter_direction([1.0, 0.0, 0.0]);
        assert!((d[2] + 0.5 / 1.25f32.sqrt()).abs() < 1e-5);
    }

    #[test]
    fn a_models_animation_sets_an_effects_lifetime() {
        assert_eq!(effect_lifetime(Some(2.5), 0.25), 2.5);
        assert_eq!(effect_lifetime(Some(0.02), 0.25), 0.1);
        assert_eq!(effect_lifetime(None, 0.25), 0.25);
        assert_eq!(effect_lifetime(Some(-1.0), 1.0), 1.0);
    }

    #[test]
    fn hurt_lines_wait_for_the_shared_cooldown() {
        let mut v = CombatVoice::default();
        let mut pick = |lo: u64, hi: u64| {
            assert_eq!((lo, hi), (2000, 4000));
            3000
        };
        assert!(v.hurt_allowed(10_000, 2.0, 4.0, &mut pick));
        assert_eq!(v.hurt_ready_at, 13_000);
        assert!(!v.hurt_allowed(12_000, 2.0, 4.0, &mut pick));
        assert_eq!(v.hurt_ready_at, 13_000);
        // Exactly at the time: allowed, and not restarted until past it.
        assert!(v.hurt_allowed(13_000, 2.0, 4.0, &mut pick));
        assert_eq!(v.hurt_ready_at, 13_000);
        // Any combat line: 1.5 s apart.
        assert!(v.line_allowed(20_000));
        assert!(!v.line_allowed(21_499));
        assert!(v.line_allowed(21_500));
    }
}
