//! What a person or creature looks like: which models make it up, and how
//! they're textured, read from its base record.
//!
//! An NPC (`NPC_`) is assembled from its race (`RACE`: head and head parts,
//! upper body and hands, each with a skin texture), its hair (`HAIR`: a
//! model and its texture), eyes (`EYES`: a texture), head parts (`HDPT`:
//! beards, eyebrows) and the clothes and armour it carries (`ARMO`: a
//! model per sex and the body slots it covers, which hide the race's own
//! parts there). A template (`TPLT`) can supply the traits or the
//! inventory. A creature (`CREA`) lists its models next to its skeleton.
//! Everything is posed by a skeleton (the record's `MODL`) and its idle
//! animation.

use esm::{FormId, FourCC, LoadOrder, Record, RecordRef};

use crate::cell::{le_f32, le_u32};

const NPC_: FourCC = FourCC::new(b"NPC_");
const CREA: FourCC = FourCC::new(b"CREA");
const RACE: FourCC = FourCC::new(b"RACE");
const ARMO: FourCC = FourCC::new(b"ARMO");
const WEAP: FourCC = FourCC::new(b"WEAP");
const HAIR: FourCC = FourCC::new(b"HAIR");
const EYES: FourCC = FourCC::new(b"EYES");
const HDPT: FourCC = FourCC::new(b"HDPT");
const LVLN: FourCC = FourCC::new(b"LVLN");
const LVLC: FourCC = FourCC::new(b"LVLC");
const ACBS: FourCC = FourCC::new(b"ACBS");
const TPLT: FourCC = FourCC::new(b"TPLT");
const RNAM: FourCC = FourCC::new(b"RNAM");
const HNAM: FourCC = FourCC::new(b"HNAM");
const ENAM: FourCC = FourCC::new(b"ENAM");
const PNAM: FourCC = FourCC::new(b"PNAM");
const CNTO: FourCC = FourCC::new(b"CNTO");
const NAM6: FourCC = FourCC::new(b"NAM6");
const HCLR: FourCC = FourCC::new(b"HCLR");
const FGGS: FourCC = FourCC::new(b"FGGS");
const FGGA: FourCC = FourCC::new(b"FGGA");
const FGTS: FourCC = FourCC::new(b"FGTS");
const NIFZ: FourCC = FourCC::new(b"NIFZ");
const BMDT: FourCC = FourCC::new(b"BMDT");
const MOD3: FourCC = FourCC::new(b"MOD3");
const MODL: FourCC = FourCC::new(b"MODL");
const MODD: FourCC = FourCC::new(b"MODD");
const MOSD: FourCC = FourCC::new(b"MOSD");
const ICON: FourCC = FourCC::new(b"ICON");
const INDX: FourCC = FourCC::new(b"INDX");
const NAM0: FourCC = FourCC::new(b"NAM0");
const NAM1: FourCC = FourCC::new(b"NAM1");
const MNAM: FourCC = FourCC::new(b"MNAM");
const FNAM: FourCC = FourCC::new(b"FNAM");
const LVLO: FourCC = FourCC::new(b"LVLO");
const BIPL: FourCC = FourCC::new(b"BIPL");
const FLST: FourCC = FourCC::new(b"FLST");
const ARMA: FourCC = FourCC::new(b"ARMA");
const LNAM: FourCC = FourCC::new(b"LNAM");

/// Body slots an armour covers (`ARMO`'s `BMDT` flags).
pub mod slots {
    pub const HEAD: u32 = 0x01;
    pub const HAIR: u32 = 0x02;
    pub const UPPER_BODY: u32 = 0x04;
    pub const LEFT_HAND: u32 = 0x08;
    pub const RIGHT_HAND: u32 = 0x10;
    pub const HEADBAND: u32 = 0x200;
    pub const HAT: u32 = 0x400;
}

/// `ACBS` flags: the NPC is female.
const FEMALE: u32 = 0x01;
/// `ACBS` template flags: take traits (race, sex, face, hair) from the
/// template; take the inventory from it.
pub const USE_TRAITS: u16 = 0x01;
const USE_MODEL: u16 = 0x40;
pub const USE_INVENTORY: u16 = 0x100;
/// `ACBS` template flags: take the factions, the AI data (`AIDT`:
/// aggression…) from the template (a Powder Ganger's 0x03BE has both).
pub const USE_FACTIONS: u16 = 0x04;
pub const USE_AI_DATA: u16 = 0x10;
/// `ACBS` template flags: take the stats (`DATA`: SPECIAL, health, a
/// creature's skills and damage; people's skills, `DNAM`), and the base
/// data (taken to cover a creature's attack reach, `RNAM`: a guess), from
/// the template.
pub const USE_STATS: u16 = 0x02;
pub const USE_BASE_DATA: u16 = 0x80;

/// The record an actor takes some of its data from: its template's
/// (following `TPLT` and leveled lists) when its template flags have
/// `flag`, else its own.
/// (Decoded once and shared: asked for every frame.)
pub fn data_record(
    order: &LoadOrder,
    base: FormId,
    flag: u16,
) -> Option<(RecordRef<'_>, std::sync::Arc<Record>)> {
    let rr = order.get(base)?;
    let record = rr.record_shared().ok()?;
    match template_for(order, &rr, &record, flag, 0) {
        Some(t) => Some(t),
        None => Some((rr, record)),
    }
}

/// Deeper template and leveled-list chains are treated as broken.
const MAX_DEPTH: u8 = 8;

/// One model of an actor.
#[derive(Debug, Clone, PartialEq)]
pub struct ActorPart {
    /// As written in the record (relative to `meshes\`).
    pub model: String,
    /// Replaces the texture of the model's skin pieces (those whose shader
    /// has the face-and-skin flag, 0x400): the race's texture for that
    /// part.
    pub skin_texture: Option<String>,
    /// Replaces the texture of every piece (hair, eyes).
    pub texture: Option<String>,
    /// A piece to leave out: hair models hold a `NoHat` and a `Hat`
    /// version, and only one shows.
    pub hide_mesh: Option<String>,
    /// The NPC's hair colour (`HCLR`), for hair pieces (hair, beards,
    /// eyebrows): see `preview::actor`.
    pub hair_tint: Option<[u8; 3]>,
    /// The face's shape applies to this part (the head, its parts, hair
    /// and head parts): its `.egm` beside the model.
    pub facegen: bool,
    /// The NPC's own skin tint, made ahead by the game's editor and laid on
    /// the part's skin pieces (see `cellview::TextureData::plus_face_tint`):
    /// for the head its face tint (`textures\characters\facemods\<plugin>\
    /// <form id>_0.dds`), for the body, arms and hands its body tint
    /// ([`body_tint_path`]), when the game has the file.
    pub face_tint: Option<String>,
    /// A rigid piece held at this bone, in the bone's own axes (a weapon
    /// at `Weapon`); other rigid pieces name their bone themselves (`Prn`).
    pub bone: Option<String>,
    /// For clothes and armour: the bone the game hangs the model's
    /// unskinned pieces from, with its top node kept, by the body slot it
    /// fills ([`slot_parent_bone`]).
    pub parent_bone: Option<String>,
}

/// The bones the game hangs biped models' unskinned pieces from
/// (`01188b74`), and for each of the 20 body slots the one it uses, or
/// none (`01188be8`, −1: the piece should be skinned): head, hair →
/// `Bip01 Head`; upper body, hands → none; weapon → `Weapon`; Pip-Boy →
/// `Bip01 L ForeTwist`; backpack → `Bip01 Spine2`; necklace →
/// `Bip01 Neck1`; headband, hat, eyeglasses, nose ring, earrings, mask,
/// choker, mouth object → `Bip01 Head`; the three body add-ons → none.
/// The game's biped loop (`004ac1e0`) attaches a part's model under that
/// bone when the model isn't skinned (to the skeleton's root when the bone
/// isn't found), after any `Prn` attachment (`004ae250`).
pub const PARENT_BONES: [&str; 5] = [
    "Bip01 Head",
    "Weapon",
    "Bip01 L ForeTwist",
    "Bip01 Spine2",
    "Bip01 Neck1",
];
pub const SLOT_BONES: [i8; 20] = [
    0, 0, -1, -1, -1, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1,
];

/// The parent bone for an armour covering `slots` (`BMDT` flags): that of
/// its lowest slot ([`SLOT_BONES`]). The game walks the slots in order
/// (`004ac1e0`); taking the model as held by the first of an armour's
/// slots is an inference (every head slot names the same bone).
pub fn slot_parent_bone(slots: u32) -> Option<&'static str> {
    let slot = (0..20).find(|s| slots >> s & 1 == 1)?;
    let bone = SLOT_BONES[slot];
    (bone >= 0).then(|| PARENT_BONES[bone as usize])
}

/// An NPC's face tint file: `textures\characters\facemods\` + the plugin
/// that defines the NPC + its form ID within that plugin (8 hex digits,
/// load-order byte 00) + `_0.dds` (Doc Mitchell's is
/// `falloutnv.esm\00104c0c_0.dds`). The `_0` is the skin shaders'
/// `FaceGenMap0`.
pub fn face_tint_path(order: &LoadOrder, npc: FormId) -> Option<String> {
    let plugin = order.slot_name(npc.mod_index())?.to_ascii_lowercase();
    Some(format!(
        "textures\\characters\\facemods\\{plugin}\\{:08x}_0.dds",
        npc.0 & 0x00FF_FFFF
    ))
}

/// An NPC's body tint file, laid on its bare skin below the head (upper
/// body, arms, hands) as the face tint is on the head:
/// `textures\characters\bodymods\<plugin>\<form id>modbody<male|female>.dds`
/// (Sunny Smiles': `falloutnv.esm\00104e84modbodyfemale.dds`, an 8 × 8
/// DXT1). Read from the recording: the game's skin texture pass
/// (`SLS1005.pso`) on her bare arms and her gloves' fingers binds that
/// 8 × 8 DXT1 as `DecalMap`, the head's pass her 256 × 256 face tint.
pub fn body_tint_path(order: &LoadOrder, npc: FormId, female: bool) -> Option<String> {
    let plugin = order.slot_name(npc.mod_index())?.to_ascii_lowercase();
    Some(format!(
        "textures\\characters\\bodymods\\{plugin}\\{:08x}modbody{}.dds",
        npc.0 & 0x00FF_FFFF,
        if female { "female" } else { "male" }
    ))
}

/// A body tint the game makes when it has no file for it, as a texture
/// reference: see [`MadeBodyTint`].
pub const MADE_BODY_TINT: &str = "egttint:";

/// An NPC's body tint as the game finds or makes it (`006149b0`): its
/// file ([`body_tint_path`]) when the game has one; otherwise ("Failed to
/// find body mod texture … Creating from scratch", always so for the
/// player, who has none) made from the race's body texture morphs for the
/// sex (the race's body part 3, `UpperBodyHumanMale.egt`, `006131a0`) with
/// the face's texture values: the race's `FGTS` for the sex plus the NPC's
/// own (`00652af0` adds the two faces' values, race first), made into a
/// picture by `nif::Egt::tint`.
#[derive(Debug, Clone, PartialEq)]
pub struct MadeBodyTint {
    pub file: String,
    pub egt: String,
    pub values: Vec<f32>,
}

impl MadeBodyTint {
    /// As a texture reference: `egttint:<file>;<egt>;<values>`.
    pub fn reference(&self) -> String {
        let values: Vec<String> = self.values.iter().map(|v| v.to_string()).collect();
        format!(
            "{MADE_BODY_TINT}{};{};{}",
            self.file,
            self.egt,
            values.join(",")
        )
    }

    /// Back from [`MadeBodyTint::reference`].
    pub fn parse(reference: &str) -> Option<MadeBodyTint> {
        let mut parts = reference.strip_prefix(MADE_BODY_TINT)?.splitn(3, ';');
        let file = parts.next()?.to_string();
        let egt = parts.next()?.to_string();
        let values = parts
            .next()?
            .split(',')
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().ok())
            .collect::<Option<Vec<f32>>>()?;
        Some(MadeBodyTint { file, egt, values })
    }
}

/// An NPC's body tint reference (see [`MadeBodyTint`]): `record` the
/// record its face comes from, `npc` its form.
fn body_tint(
    order: &LoadOrder,
    npc: FormId,
    record: &Record,
    race: Option<&Race>,
    female: bool,
) -> Option<String> {
    let file = body_tint_path(order, npc, female)?;
    let made = race.and_then(|race| {
        let egt = race.body(female, 3).0?.clone();
        let own = record
            .get(FGTS)
            .map(|s| floats(&s.data))
            .unwrap_or_default();
        let theirs = &race.texture[usize::from(female)];
        let n = own.len().max(theirs.len());
        let values = (0..n)
            .map(|i| theirs.get(i).copied().unwrap_or(0.0) + own.get(i).copied().unwrap_or(0.0))
            .collect();
        Some(MadeBodyTint {
            file: file.clone(),
            egt,
            values,
        })
    });
    Some(made.map_or(file, |m| m.reference()))
}

/// An NPC's FaceGen face: the values of the shape controls (`FGGS`, 50
/// symmetric; `FGGA`, 30 asymmetric).
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub symmetric: Vec<f32>,
    pub asymmetric: Vec<f32>,
}

/// How an actor is put together.
#[derive(Debug, Clone, PartialEq)]
pub struct ActorLook {
    pub base: FormId,
    pub creature: bool,
    pub female: bool,
    /// The skeleton model, as written (relative to `meshes\`).
    pub skeleton: String,
    /// The idle animation, relative to `meshes\` (see [`idle_animation`]).
    pub idle: String,
    /// Walking forward, relative to `meshes\` (see [`walk_animation`]).
    pub walk: String,
    pub parts: Vec<ActorPart>,
    /// The face's shape, for NPCs that have one.
    pub face: Option<Face>,
    /// Overall size (`NAM6` height for NPCs; 1 when unset).
    pub scale: f32,
    /// How they fight: a person's weapon (its model is among `parts`, at
    /// the `Weapon` node) or a creature's own attacks, with the animations.
    pub fighting: Option<Fighting>,
}

/// The animations someone fights with, beside their skeleton (see
/// [`third_person_animation`]): a person's for their weapon's kind, a
/// creature's own (`creatures\nvgecko\h2haim.kf`, `h2hattackright.kf`,
/// named as people's unarmed ones).
#[derive(Debug, Clone, PartialEq)]
pub struct Fighting {
    /// The weapon, for a person who has one.
    pub weapon: Option<FormId>,
    /// Put away: `<kind>holster.kf` (see `nif::hang_weapon`).
    pub holster: Option<String>,
    /// Held ready: `<kind>aim.kf` (looping).
    pub aim: String,
    /// Running: `locomotion\<kind>fastforward.kf` (creatures'
    /// `locomotion\mtfastforward.kf`).
    pub run: String,
    /// The attack, named as in first person ([`first_person_attack`]).
    pub attack: String,
}

/// A third-person animation beside a skeleton for a weapon kind
/// ([`first_person_kind`]): the game's files are named as in first person
/// (`characters\_male\1hpaim.kf`, `2hrattack8.kf`, `1hpholster.kf`,
/// `locomotion\2hrfastforward.kf`).
pub fn third_person_animation(skeleton: &str, kind: &str, name: &str) -> String {
    let folder = skeleton
        .rsplit_once(['\\', '/'])
        .map_or("", |(folder, _)| folder);
    format!("{folder}\\{kind}{name}.kf")
}

/// A weapon's model and animations for someone on `skeleton`.
fn held_weapon(order: &LoadOrder, skeleton: &str, id: FormId) -> Option<(String, Fighting)> {
    let rr = order.get(id).filter(|r| r.entry.header.kind == WEAP)?;
    let model = zstring(&rr.record().ok()?, MODL)?;
    let weapon = crate::combat::Weapon::load(order, id)?;
    let kind = first_person_kind(Some(weapon.animation));
    Some((
        model,
        Fighting {
            weapon: Some(id),
            holster: Some(third_person_animation(skeleton, kind, "holster")),
            aim: third_person_animation(skeleton, kind, "aim"),
            run: third_person_animation(skeleton, &format!("locomotion\\{kind}"), "fastforward"),
            attack: third_person_animation(skeleton, kind, attack_name(weapon.attack_animation)),
        },
    ))
}

/// Fighting unarmed beside a skeleton: a person's fists or a creature's
/// own attacks, both named `h2h` (aim and right attack; people's attacks
/// come as `_a`/`_b` variants). Running: people's `locomotion\
/// h2hfastforward.kf`, creatures' `locomotion\mtfastforward.kf`. Which of
/// its attacks the game picks isn't read yet (the right one here).
fn unarmed(skeleton: &str, creature: bool) -> Fighting {
    let run = if creature {
        "locomotion\\mt"
    } else {
        "locomotion\\h2h"
    };
    Fighting {
        weapon: None,
        holster: None,
        aim: third_person_animation(skeleton, "h2h", "aim"),
        run: third_person_animation(skeleton, run, "fastforward"),
        attack: third_person_animation(skeleton, "h2h", "attackright"),
    }
}

/// The idle animation next to a skeleton: `locomotion\mtidle.kf` for
/// people (`characters\_male\locomotion\mtidle.kf`), `mtidle.kf` beside a
/// creature's skeleton. The game picks animations by these folder
/// conventions; the exact idle it plays first isn't traced.
pub fn idle_animation(skeleton: &str, creature: bool) -> String {
    let folder = skeleton
        .rsplit_once(['\\', '/'])
        .map_or("", |(folder, _)| folder);
    if creature {
        format!("{folder}\\mtidle.kf")
    } else {
        format!("{folder}\\locomotion\\mtidle.kf")
    }
}

/// Walking forward, next to a skeleton: people's under `locomotion\male\`
/// or `locomotion\female\` (`characters\_male\locomotion\male\
/// mtforward.kf`; the folder also has `mtfastforward.kf`, running),
/// creatures' under `locomotion\` beside their skeleton
/// (`creatures\molerat\locomotion\mtforward.kf`). By the game's folder
/// conventions; how it picks among them isn't traced.
pub fn walk_animation(skeleton: &str, creature: bool, female: bool) -> String {
    let folder = skeleton
        .rsplit_once(['\\', '/'])
        .map_or("", |(folder, _)| folder);
    if creature {
        format!("{folder}\\locomotion\\mtforward.kf")
    } else if female {
        format!("{folder}\\locomotion\\female\\mtforward.kf")
    } else {
        format!("{folder}\\locomotion\\male\\mtforward.kf")
    }
}

fn form(rr: &RecordRef<'_>, record: &Record, kind: FourCC) -> Option<FormId> {
    record
        .get(kind)
        .filter(|s| s.data.len() >= 4)
        .map(|s| rr.plugin.to_global(FormId(le_u32(&s.data, 0))))
        .filter(|id| id.0 != 0)
}

fn zstring(record: &Record, kind: FourCC) -> Option<String> {
    record
        .get(kind)
        .map(|s| s.zstring())
        .filter(|s| !s.is_empty())
}

/// The first-person skeleton: arms, hands and a camera node (`Camera1st`,
/// under `Bip01 Looking`, the pivot at eye height 118; a `Weapon` node in
/// the right hand).
pub const FIRST_PERSON_SKELETON: &str = "Characters\\_1stPerson\\Skeleton.NIF";

/// The first-person pose for holding a weapon, by its animation type
/// (`WEAP` `DNAM`): `<kind>aim.kf` beside the first-person skeleton, the
/// kinds the game ships (h2h fists, 1hm and 2hm melee, 1hp pistols, 2hr
/// rifles, 2ha automatic rifles, 2hh handles, 2hl launchers, 1gt thrown,
/// 1md mines, 1lm lunchbox mines): the game's table of weapon kinds by
/// animation type (`0118a838`, `world::animation::groups::weapon_kind`).
pub fn first_person_pose(animation: Option<u32>) -> String {
    format!(
        "Characters\\_1stPerson\\{}aim.kf",
        first_person_kind(animation)
    )
}

/// The animations' prefix for a weapon's animation type: its weapon kind's
/// name (`0118a838`, the names at `011977a4`): energy pistols as pistols,
/// energy rifles as rifles, thrown weapons as grenades, `OneHandMine` 1md
/// and `OneHandLunchboxMine` 1lm.
pub fn first_person_kind(animation: Option<u32>) -> &'static str {
    match animation {
        Some(1) => "1hm",
        Some(2) => "2hm",
        Some(3 | 4) => "1hp",
        Some(5 | 7) => "2hr",
        Some(6) => "2ha",
        Some(8) => "2hh",
        Some(9) => "2hl",
        Some(10 | 13) => "1gt",
        Some(11) => "1md",
        Some(12) => "1lm",
        _ => "h2h",
    }
}

/// The first-person attack for a weapon: its attack animation number
/// (`DNAM` 41) in the editor's steps of six (26 AttackLeft, 32
/// AttackRight, 38 Attack3 … 68 Attack8, 74 AttackLoop, 80 AttackSpin,
/// 86 AttackSpin2, 144 Attack9; 255 the default, AttackRight), named as
/// the files are: the 9mm pistol's 32 is `1hpattackright.kf`, the varmint
/// rifle's 68 `2hrattack8.kf`, the service rifle's 38 `2haattack3.kf`.
/// Melee and fists ship `…_a`/`…_b` variants instead (`1hmattackright_a`).
pub fn first_person_attack(animation: Option<u32>, attack: u8) -> String {
    format!(
        "Characters\\_1stPerson\\{}{}.kf",
        first_person_kind(animation),
        attack_name(attack)
    )
}

/// An attack animation number's name in the files (see
/// [`first_person_attack`]).
pub fn attack_name(attack: u8) -> &'static str {
    match attack {
        26..=31 => "attackleft",
        38..=43 => "attack3",
        44..=49 => "attack4",
        50..=55 => "attack5",
        56..=61 => "attack6",
        62..=67 => "attack7",
        68..=73 => "attack8",
        74..=79 => "attackloop",
        80..=85 => "attackspin",
        86..=91 => "attackspin2",
        144..=149 => "attack9",
        _ => "attackright",
    }
}

/// The first-person reload for a weapon: its reload animation number
/// (`DNAM` 15) as a letter, A–S for 0–18, then W, X, Y, Z (the 9mm
/// pistol's 11 is `1hpreloadl.kf`, the varmint rifle's 12
/// `2hrreloadm.kf`, the .357's 20 `1hpreloadx.kf`, the hunting shotgun's
/// 21 `2hrreloady.kf`). X, Y and Z come with a `…start` part (loading
/// rounds one by one); only the main part is named here.
pub fn first_person_reload(animation: Option<u32>, reload: u8) -> String {
    let letter = match reload {
        0..=18 => (b'a' + reload) as char,
        19 => 'w',
        20 => 'x',
        21 => 'y',
        _ => 'z',
    };
    format!(
        "Characters\\_1stPerson\\{}reload{letter}.kf",
        first_person_kind(animation)
    )
}

/// The first-person animation for drawing a weapon of this kind
/// (`<kind>equip.kf`, the `Equip` animation group) and for putting it away
/// (`<kind>unequip.kf`, `Unequip`); the game ships both for every kind
/// (`h2hequip.kf` … `1mdunequip.kf`). The `<kind>holster.kf` files are the
/// put-away pose, not an animation.
pub fn first_person_ready(animation: Option<u32>, drawing: bool) -> String {
    format!(
        "Characters\\_1stPerson\\{}{}.kf",
        first_person_kind(animation),
        if drawing { "equip" } else { "unequip" }
    )
}

/// A hand model's first-person version: `LeftHand.NIF` →
/// `LeftHand1st.NIF` (the game ships `lefthand1st.nif`,
/// `righthand1st.nif` and the female ones beside the normal hands).
pub fn first_person_hand(model: &str) -> String {
    match model.rsplit_once('.') {
        Some((stem, ext)) => format!("{stem}1st.{ext}"),
        None => format!("{model}1st"),
    }
}

/// The player as seen in first person: on the first-person skeleton, in
/// the hold pose for the weapon's kind, the clothes worn (else the race's
/// upper body) and the first-person hands, with the weapon's own model
/// held at the `Weapon` node. No head: the camera is inside it.
pub fn first_person_look(
    order: &LoadOrder,
    female: bool,
    worn: &[FormId],
    weapon: Option<(String, u32)>,
) -> Option<ActorLook> {
    let player = order.get(crate::dialogue::PLAYER_BASE)?;
    let record = player.record().ok()?;
    let race = form(&player, &record, RNAM).and_then(|id| Race::load(order, id));
    // The player's body tint on the arms and hands, as on any NPC's bare
    // skin below the head (made by the game for the player: see
    // [`MadeBodyTint`]).
    let tint = body_tint(order, player.form_id, &record, race.as_ref(), female);
    let mut covered = 0u32;
    let mut parts = Vec::new();
    let part = |model: String, skin_texture: Option<String>, bone: Option<String>| ActorPart {
        model,
        skin_texture,
        texture: None,
        hide_mesh: None,
        hair_tint: None,
        facegen: false,
        face_tint: tint.clone(),
        bone,
        parent_bone: None,
    };
    for &item in worn {
        let Some(armor) = Armor::load(order, item) else {
            continue;
        };
        if armor.slots & covered != 0 {
            continue;
        }
        // Gloves (armour addons) ship no first-person versions: their
        // own models are worn.
        for (slots, model) in armor.pieces(female) {
            covered |= slots;
            parts.push(part(model, skin_for(race.as_ref(), female, slots), None));
        }
    }
    if let Some(race) = &race {
        for (index, slot) in [
            (0, slots::UPPER_BODY),
            (1, slots::LEFT_HAND),
            (2, slots::RIGHT_HAND),
        ] {
            if covered & slot != 0 {
                continue;
            }
            if let (Some(model), texture) = race.body(female, index) {
                let model = if index == 0 {
                    model.clone()
                } else {
                    first_person_hand(model)
                };
                parts.push(part(model, texture.cloned(), None));
            }
        }
    }
    let animation = weapon.as_ref().map(|(_, a)| *a);
    if let Some((model, _)) = weapon {
        parts.push(part(model, None, Some("Weapon".into())));
    }
    let idle = first_person_pose(animation);
    Some(ActorLook {
        base: crate::dialogue::PLAYER_BASE,
        creature: false,
        female,
        skeleton: FIRST_PERSON_SKELETON.into(),
        walk: idle.clone(),
        idle,
        parts,
        face: None,
        scale: 1.0,
        fighting: None,
    })
}

/// What an actor's base record says it looks like. `None` for records
/// that aren't people or creatures, or that can't be read.
pub fn actor_look(order: &LoadOrder, base: FormId) -> Option<ActorLook> {
    let rr = order.get(base)?;
    let record = rr.record().ok()?;
    match rr.entry.header.kind {
        k if k == NPC_ => npc_look(order, &rr, &record),
        k if k == CREA => creature_look(order, &rr, &record),
        _ => None,
    }
}

fn creature_look(order: &LoadOrder, rr: &RecordRef<'_>, record: &Record) -> Option<ActorLook> {
    // A creature whose model comes from a template.
    let template = template_for(order, rr, record, USE_MODEL, 0);
    let record = template.as_ref().map_or(record, |(_, r)| r);
    let skeleton = zstring(record, MODL)?;
    let folder = skeleton
        .rsplit_once(['\\', '/'])
        .map_or(String::new(), |(f, _)| format!("{f}\\"));
    let parts = record
        .get(NIFZ)
        .map(|s| {
            s.data
                .split(|&b| b == 0)
                .filter(|n| !n.is_empty())
                .map(|n| ActorPart {
                    model: format!("{folder}{}", esm::text::decode_cp1252(n)),
                    skin_texture: None,
                    texture: None,
                    hide_mesh: None,
                    hair_tint: None,
                    facegen: false,
                    face_tint: None,
                    bone: None,
                    parent_bone: None,
                })
                .collect()
        })
        .unwrap_or_default();
    Some(ActorLook {
        base: rr.form_id,
        creature: true,
        female: false,
        idle: idle_animation(&skeleton, true),
        walk: walk_animation(&skeleton, true, false),
        fighting: Some(unarmed(&skeleton, true)),
        skeleton,
        parts,
        face: None,
        scale: 1.0,
    })
}

/// The record a template chain leads to for the traits named by `flag`:
/// the template (`TPLT`) when the actor's template flags (`ACBS`) say so,
/// following leveled lists to their first entry.
fn template_for<'a>(
    order: &'a LoadOrder,
    rr: &RecordRef<'a>,
    record: &Record,
    flag: u16,
    depth: u8,
) -> Option<(RecordRef<'a>, std::sync::Arc<Record>)> {
    if depth > MAX_DEPTH {
        return None;
    }
    let flags = record
        .get(ACBS)
        .filter(|s| s.data.len() >= 24)
        .map_or(0, |s| u16::from_le_bytes([s.data[22], s.data[23]]));
    if flags & flag == 0 {
        return None;
    }
    let template = form(rr, record, TPLT)?;
    let (trr, trecord) = first_actor(order, template, depth + 1)?;
    // The template may itself use a template for these traits.
    template_for(order, &trr, &trecord, flag, depth + 1).or(Some((trr, trecord)))
}

/// An actor record, following leveled actor lists to their first entry.
fn first_actor(
    order: &LoadOrder,
    id: FormId,
    depth: u8,
) -> Option<(RecordRef<'_>, std::sync::Arc<Record>)> {
    if depth > MAX_DEPTH {
        return None;
    }
    let rr = order.get(id)?;
    let record = rr.record_shared().ok()?;
    let kind = rr.entry.header.kind;
    if kind == LVLN || kind == LVLC {
        let entry = record.get_all(LVLO).find(|s| s.data.len() >= 8)?;
        let next = rr.plugin.to_global(FormId(le_u32(&entry.data, 4)));
        return first_actor(order, next, depth + 1);
    }
    (kind == NPC_ || kind == CREA).then_some((rr, record))
}

/// The player as seen in third person: assembled as people are
/// ([`actor_look`]) from the player's record (`PLAYER_BASE`: race, face,
/// hair, eyes, head parts, height), but with the sex the game has
/// (`SexChange`, the face menu), the clothes and armour the player wears
/// (`worn`, in order) instead of the record's inventory, and the weapon in
/// hand (`weapon`), if any.
pub fn player_look(
    order: &LoadOrder,
    female: bool,
    worn: &[FormId],
    weapon: Option<FormId>,
) -> Option<ActorLook> {
    let rr = order.get(crate::dialogue::PLAYER_BASE)?;
    let record = rr.record().ok()?;
    npc_look_dressed(
        order,
        &rr,
        &record,
        Some(Dressing {
            female: Some(female),
            worn,
            weapon,
        }),
    )
}

/// What the game state says someone is and wears ([`player_look`],
/// [`npc_look_wearing`]): the sex (`None`: the record's), the clothes and
/// armour worn, in order, and the weapon in hand.
struct Dressing<'a> {
    female: Option<bool>,
    worn: &'a [FormId],
    weapon: Option<FormId>,
}

/// A person as drawn when the game state has them wear `worn` (clothes and
/// armour, in order: the first on a body slot wins, as [`worn_of`]) and
/// hold `weapon`: assembled as [`actor_look`] assembles them from their
/// record, with these instead of the record's inventory
/// (`world::outfit::worn_armour`, `world::combat::weapon_in_hand`). `None`
/// for creatures and records that can't be read.
pub fn npc_look_wearing(
    order: &LoadOrder,
    base: FormId,
    worn: &[FormId],
    weapon: Option<FormId>,
) -> Option<ActorLook> {
    let rr = order.get(base).filter(|r| r.entry.header.kind == NPC_)?;
    let record = rr.record().ok()?;
    npc_look_dressed(
        order,
        &rr,
        &record,
        Some(Dressing {
            female: None,
            worn,
            weapon,
        }),
    )
}

/// Whether a person is a woman: their traits' (`ACBS` flag 0x01, from the
/// template when its flags say the traits come from there).
pub fn is_female(order: &LoadOrder, base: FormId) -> bool {
    data_record(order, base, USE_TRAITS).is_some_and(|(_, record)| {
        record
            .get(ACBS)
            .filter(|s| s.data.len() >= 4)
            .is_some_and(|s| le_u32(&s.data, 0) & FEMALE != 0)
    })
}

/// What a person puts on from what they carry (`006047c0`, the armour
/// part; `InventoryChanges::GetBestArmor` (Xbox PDB) `004c8220` for
/// each body slot 0 to 19): for each slot, the best armour covering it
/// ([`best_armour`]) is put on unless the upper-body armour already
/// chosen (slot 2) covers that slot, it's on already, or, for a slot
/// other than 2, it covers the upper body too (only slot 2 puts such a
/// piece on). Putting one on takes off what shares a slot with it
/// (`EquipItem`, vtable `+0x184`; that rule isn't traced here).
/// `entries` are the record's container entries with what each gives:
/// items named directly are looked at first (the base container, in
/// order), then what leveled lists gave (the inventory changes; their
/// order among themselves isn't traced).
// Translated from 006047c0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn pick_worn(order: &LoadOrder, entries: &[(FormId, Vec<(FormId, i32)>)]) -> Vec<FormId> {
    let direct = entries
        .iter()
        .flat_map(|(entry, items)| items.iter().filter(move |(i, _)| i == entry));
    let given = entries
        .iter()
        .flat_map(|(entry, items)| items.iter().filter(move |(i, _)| i != entry));
    let candidates: Vec<FormId> = direct
        .chain(given)
        .filter(|(_, count)| *count > 0)
        .map(|(item, _)| *item)
        .collect();
    let slots_of = |item: FormId| Armor::load(order, item).map_or(0, |a| a.slots);
    let mut worn: Vec<(FormId, u32)> = Vec::new();
    let mut upper: Option<u32> = None;
    for slot in 0..20u32 {
        let Some(best) = best_armour(order, &candidates, slot) else {
            continue;
        };
        let bit = 1u32 << slot;
        if upper.is_some_and(|s| s & bit != 0) || worn.iter().any(|(w, _)| *w == best) {
            continue;
        }
        let slots = slots_of(best);
        if slot == slots::UPPER_BODY.trailing_zeros() {
            upper = Some(slots);
        } else if slots & slots::UPPER_BODY != 0 {
            continue;
        }
        worn.retain(|(_, s)| s & slots == 0);
        worn.push((best, slots));
    }
    worn.into_iter().map(|(item, _)| item).collect()
}

/// The best armour among `candidates` for a body slot
/// (`InventoryChanges::GetBestArmor` (Xbox PDB), `004c8220`): of those
/// covering it (`BMDT` slot bit, `00480af0`), the highest damage
/// resistance (`DNAM` u16 at 0 ÷ 100, `004be080`, truncated: the FPU's
/// control word | 0xc00) × the condition factor (`00646360` →
/// `00646d40`, given the item's health: 1 above 0.5, so for any item at
/// its full `DATA` health of 1 or more) + damage threshold (`DNAM` f32
/// at 4, `004be180`); the first of equals. For what a person carries in
/// the game state (a companion after trading) see
/// [`crate::companions::best_armour`], the same function.
// Translated from 004c8220 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn best_armour(order: &LoadOrder, candidates: &[FormId], slot: u32) -> Option<FormId> {
    let mut best: Option<(FormId, f32)> = None;
    for &item in candidates {
        let Some(armor) = Armor::load(order, item) else {
            continue;
        };
        if armor.slots & (1 << slot) == 0 {
            continue;
        }
        let Some(record) = order.get(item).and_then(|r| r.record().ok()) else {
            continue;
        };
        let dnam = record
            .get(FourCC::new(b"DNAM"))
            .map(|s| s.data.clone())
            .unwrap_or_default();
        let dr = if dnam.len() >= 2 {
            f32::from(u16::from_le_bytes([dnam[0], dnam[1]])) / 100.0
        } else {
            0.0
        };
        let dt = if dnam.len() >= 8 {
            le_f32(&dnam, 4)
        } else {
            0.0
        };
        let health = record
            .get(esm::sig::DATA)
            .filter(|s| s.data.len() >= 8)
            .map_or(0.0, |s| le_u32(&s.data, 4) as f32);
        let factor = if health <= 0.5 {
            1.0 - (0.5 - health)
        } else {
            1.0
        };
        let score = f32::from(dr.trunc() as i32 as u16) * factor + dt;
        if best.map_or(true, |(_, b)| b < score) {
            best = Some((item, score));
        }
    }
    best.map(|(item, _)| item)
}

/// Which of `items` (in order) are drawn, each with its record: the
/// clothes and armour that have a model for their sex, the first on a body
/// slot taking it (the slots of its own model and its addons'). What is
/// worn is picked by [`pick_worn`].
pub fn worn_of(
    order: &LoadOrder,
    items: impl IntoIterator<Item = FormId>,
    female: bool,
) -> Vec<(FormId, Armor)> {
    let mut covered = 0u32;
    let mut out = Vec::new();
    for item in items {
        let Some(armor) = Armor::load(order, item) else {
            continue;
        };
        if armor.slots & covered != 0 {
            continue;
        }
        let pieces = armor.pieces(female);
        if pieces.is_empty() {
            continue;
        }
        for (slots, _) in &pieces {
            covered |= slots;
        }
        out.push((item, armor));
    }
    out
}

fn npc_look(order: &LoadOrder, rr: &RecordRef<'_>, record: &Record) -> Option<ActorLook> {
    npc_look_dressed(order, rr, record, None)
}

fn npc_look_dressed(
    order: &LoadOrder,
    rr: &RecordRef<'_>,
    record: &Record,
    dressing: Option<Dressing>,
) -> Option<ActorLook> {
    let base = rr.form_id;
    let traits = template_for(order, rr, record, USE_TRAITS, 0);
    let (trr, traits_record): (&RecordRef<'_>, &Record) = match &traits {
        Some((r, rec)) => (r, rec),
        None => (rr, record),
    };
    let inventory = template_for(order, rr, record, USE_INVENTORY, 0);
    let (irr, inventory_record): (&RecordRef<'_>, &Record) = match &inventory {
        Some((r, rec)) => (r, rec),
        None => (rr, record),
    };
    let model_from = template_for(order, rr, record, USE_MODEL, 0);
    let skeleton = model_from
        .as_ref()
        .and_then(|(_, rec)| zstring(rec, MODL))
        .or_else(|| zstring(record, MODL))
        .unwrap_or_else(|| "Characters\\_Male\\Skeleton.NIF".into());

    let female = dressing.as_ref().and_then(|d| d.female).unwrap_or_else(|| {
        traits_record
            .get(ACBS)
            .filter(|s| s.data.len() >= 4)
            .is_some_and(|s| le_u32(&s.data, 0) & FEMALE != 0)
    });
    let race = form(trr, traits_record, RNAM).and_then(|id| Race::load(order, id));
    let hair_tint = traits_record
        .get(HCLR)
        .filter(|s| s.data.len() >= 3)
        .map(|s| [s.data[0], s.data[1], s.data[2]]);

    // Clothes and armour first: what they cover hides the race's parts.
    let mut covered = 0u32;
    let mut parts = Vec::new();
    // The NPC's body tint, for the skin pieces of everything below the head.
    let body_tint = body_tint(order, trr.form_id, traits_record, race.as_ref(), female);
    let carried: Vec<FormId> = match &dressing {
        Some(d) => d.worn.to_vec(),
        None => pick_worn(order, &inventory_entries(order, irr, inventory_record)),
    };
    for (_, armor) in worn_of(order, carried.iter().copied(), female) {
        let pieces = armor.pieces(female);
        let facegen = armor.pieces_facegen(female);
        for ((slots, model), facegen) in pieces.into_iter().zip(facegen) {
            covered |= slots;
            parts.push(ActorPart {
                model,
                skin_texture: skin_for(race.as_ref(), female, slots),
                texture: None,
                hide_mesh: None,
                hair_tint: None,
                facegen,
                face_tint: body_tint.clone(),
                bone: None,
                parent_bone: slot_parent_bone(slots).map(str::to_string),
            });
        }
    }

    if let Some(race) = &race {
        // Upper body, left hand, right hand (`INDX` 0, 1, 2).
        for (index, slot) in [
            (0, slots::UPPER_BODY),
            (1, slots::LEFT_HAND),
            (2, slots::RIGHT_HAND),
        ] {
            if covered & slot != 0 {
                continue;
            }
            if let (Some(model), texture) = race.body(female, index) {
                parts.push(ActorPart {
                    model: model.clone(),
                    skin_texture: texture.cloned(),
                    texture: None,
                    hide_mesh: None,
                    hair_tint: None,
                    facegen: false,
                    face_tint: body_tint.clone(),
                    bone: None,
                    parent_bone: None,
                });
            }
        }
        if covered & slots::HEAD == 0 {
            let eyes = form(trr, traits_record, ENAM)
                .and_then(|id| order.get(id))
                .filter(|r| r.entry.header.kind == EYES)
                .and_then(|r| r.record().ok())
                .and_then(|r| zstring(&r, ICON));
            // Head, mouth, teeth, tongue, eyes (`INDX` 0, 2–7; 1, the ears,
            // has no model of its own).
            for index in [0, 2, 3, 4, 5, 6, 7] {
                if let (Some(model), texture) = race.head(female, index) {
                    let eye = index >= 6;
                    parts.push(ActorPart {
                        model: model.clone(),
                        skin_texture: if eye { None } else { texture.cloned() },
                        texture: if eye { eyes.clone() } else { None },
                        hide_mesh: None,
                        hair_tint: None,
                        facegen: true,
                        face_tint: if index == 0 {
                            face_tint_path(order, trr.form_id)
                        } else {
                            None
                        },
                        bone: None,
                        parent_bone: None,
                    });
                }
            }
            for id in traits_record
                .get_all(PNAM)
                .filter(|s| s.data.len() >= 4)
                .map(|s| trr.plugin.to_global(FormId(le_u32(&s.data, 0))))
            {
                let Some(part) = order
                    .get(id)
                    .filter(|r| r.entry.header.kind == HDPT)
                    .and_then(|r| r.record().ok())
                else {
                    continue;
                };
                if let Some(model) = zstring(&part, MODL) {
                    parts.push(ActorPart {
                        model,
                        skin_texture: None,
                        texture: None,
                        hide_mesh: None,
                        hair_tint,
                        facegen: true,
                        face_tint: None,
                        bone: None,
                        parent_bone: None,
                    });
                }
            }
        }
    }
    if covered & (slots::HAIR | slots::HEAD) == 0 {
        let hair = form(trr, traits_record, HNAM)
            .and_then(|id| order.get(id))
            .filter(|r| r.entry.header.kind == HAIR)
            .and_then(|r| r.record().ok());
        if let Some(hair) = hair {
            if let Some(model) = zstring(&hair, MODL) {
                let hat = covered & (slots::HAT | slots::HEADBAND) != 0;
                parts.push(ActorPart {
                    model,
                    skin_texture: None,
                    texture: zstring(&hair, ICON),
                    hide_mesh: Some(if hat { "NoHat" } else { "Hat" }.into()),
                    hair_tint,
                    facegen: true,
                    face_tint: None,
                    bone: None,
                    parent_bone: None,
                });
            }
        }
    }
    // The weapon they fight with ([`best_weapon`]), at the right hand's
    // `Weapon` node.
    let weapon = match &dressing {
        Some(d) => d.weapon,
        None => best_weapon(order, &inventory_items(order, irr, inventory_record)),
    }
    .and_then(|item| held_weapon(order, &skeleton, item));
    if let Some((model, _)) = &weapon {
        parts.push(ActorPart {
            model: model.clone(),
            skin_texture: None,
            texture: None,
            hide_mesh: None,
            hair_tint: None,
            facegen: false,
            face_tint: None,
            bone: Some("Weapon".into()),
            parent_bone: None,
        });
    }
    let height = traits_record
        .get(NAM6)
        .filter(|s| s.data.len() >= 4)
        .map(|s| le_f32(&s.data, 0))
        .filter(|h| *h > 0.0)
        .unwrap_or(1.0);
    Some(ActorLook {
        base,
        creature: false,
        female,
        idle: idle_animation(&skeleton, false),
        walk: walk_animation(&skeleton, false, female),
        fighting: Some(weapon.map_or_else(|| unarmed(&skeleton, false), |(_, w)| w)),
        skeleton,
        parts,
        face: face(
            traits_record,
            race.as_ref()
                .and_then(|r| r.face[usize::from(female)].as_ref()),
        ),
        scale: height,
    })
}

/// Little-endian floats.
fn floats(data: &[u8]) -> Vec<f32> {
    data.chunks_exact(4).map(|c| le_f32(c, 0)).collect()
}

/// An NPC's face shape: its own values (`FGGS`, `FGGA`) **added to its
/// race's** for its sex (the race's `FGGS`/`FGGA` after `MNAM` or `FNAM`).
/// Read from the game's own vertex buffer for Sunny Smiles' head (an
/// apitrace recording): her morphed head is the model's vertices plus the
/// `.egm` morphs times exactly Hispanic-female + her own values (all 50
/// symmetric coefficients agree to 0.001; with her values alone the face
/// was up to 1 unit off).
fn face(record: &Record, race: Option<&Face>) -> Option<Face> {
    let own = |kind: FourCC| -> Vec<f32> {
        record
            .get(kind)
            .map(|s| floats(&s.data))
            .unwrap_or_default()
    };
    let add = |a: Vec<f32>, b: &[f32]| -> Vec<f32> {
        let n = a.len().max(b.len());
        (0..n)
            .map(|i| a.get(i).copied().unwrap_or(0.0) + b.get(i).copied().unwrap_or(0.0))
            .collect()
    };
    let (race_symmetric, race_asymmetric) = race.map_or((&[][..], &[][..]), |r| {
        (r.symmetric.as_slice(), r.asymmetric.as_slice())
    });
    let symmetric = add(own(FGGS), race_symmetric);
    let asymmetric = add(own(FGGA), race_asymmetric);
    (!symmetric.is_empty() || !asymmetric.is_empty()).then_some(Face {
        symmetric,
        asymmetric,
    })
}

/// The armour and clothes an NPC carries (`CNTO`), leveled item lists
/// followed to their first entry. **A guess at what's worn**: the game
/// equips the best item per slot; here the first item per slot is.
/// What a person's record gives them to carry, with counts: its `CNTO`s,
/// leveled lists picked without chance at level 1 (a "use all" list gives
/// everything: Sunny Smiles' `WithAmmoNVVarmintRifleLoot` is the rifle and
/// its rounds), as a new game first draws them.
fn inventory_items(order: &LoadOrder, rr: &RecordRef<'_>, record: &Record) -> Vec<(FormId, i32)> {
    inventory_entries(order, rr, record)
        .into_iter()
        .flat_map(|(_, items)| items)
        .collect()
}

/// [`inventory_items`] by the record's entry each came from: the entry
/// (`CNTO`: an item or a leveled list) and what it gives.
pub(crate) fn inventory_entries(
    order: &LoadOrder,
    rr: &RecordRef<'_>,
    record: &Record,
) -> Vec<(FormId, Vec<(FormId, i32)>)> {
    record
        .get_all(CNTO)
        .filter(|s| s.data.len() >= 8)
        .map(|entry| {
            let id = rr.plugin.to_global(FormId(le_u32(&entry.data, 0)));
            let count = le_u32(&entry.data, 4) as i32;
            (
                id,
                crate::leveled::resolve_first(order, id, count.max(1), 1),
            )
        })
        .collect()
}

/// What a person's base record carries (from its template when its
/// template flags say the inventory comes from there), drawn as
/// [`inventory_items`] does.
pub fn record_inventory(order: &LoadOrder, base: FormId) -> Vec<(FormId, i32)> {
    match data_record(order, base, USE_INVENTORY) {
        Some((rr, record)) => inventory_items(order, &rr, &record),
        None => Vec::new(),
    }
}

/// The weapon someone fights with out of what they carry: one they have
/// ammunition for (or that needs none) before one they don't, ranged before
/// melee, then the most damage. A guess at the game's choice, which isn't
/// traced.
pub fn best_weapon(order: &LoadOrder, items: &[(FormId, i32)]) -> Option<FormId> {
    let has = |id: FormId| items.iter().any(|(i, n)| *i == id && *n > 0);
    items
        .iter()
        .enumerate()
        .filter(|(_, (_, n))| *n > 0)
        .filter_map(|(i, (id, _))| Some((i, crate::combat::Weapon::load(order, *id)?)))
        .map(|(i, w)| {
            let loaded = w.ammo.is_empty() || w.ammo.iter().any(|a| has(*a));
            ((loaded, !w.is_melee()), w.damage, i, w.form_id)
        })
        .max_by(|a, b| {
            a.0.cmp(&b.0)
                .then(a.1.total_cmp(&b.1))
                // The first listed wins a tie.
                .then(b.2.cmp(&a.2))
        })
        .map(|(_, _, _, id)| id)
}

/// The weapon a person's base record has them fight with.
pub fn carried_weapon(order: &LoadOrder, base: FormId) -> Option<FormId> {
    best_weapon(order, &record_inventory(order, base))
}

/// Clothes or armour (`ARMO`): the body slots it covers (`BMDT`), its
/// models for men and women, and the addons worn with it.
pub struct Armor {
    pub slots: u32,
    pub male: Option<String>,
    pub female: Option<String>,
    /// The biped model's flags' bit 0 (`MODD`, `MOSD`; the game's
    /// `TESModel` `+0x14` bit 0, read by `004ae8f0`): a FaceGen part,
    /// given the NPC's face shape and hung as the head's parts are (a
    /// hat, `CowboyHat02`).
    pub male_facegen: bool,
    pub female_facegen: bool,
    /// Armour addons (`ARMA`) its biped model list names (`BIPL`, a form
    /// list): pieces put on with it, each with its own slots and models.
    /// Leather armour's are its gloves (`LeatherArmorGloveR`, slot 0x10,
    /// `Armor\LeatherArmor\F\glover.NIF`; `...GloveL`, 0x08); in the
    /// recording Sunny Smiles is drawn with them and without the race's
    /// bare hands.
    pub addons: Vec<ArmorAddon>,
}

/// One armour addon (`ARMA`): its slots (`BMDT`) and models.
#[derive(Debug, Clone, PartialEq)]
pub struct ArmorAddon {
    pub slots: u32,
    pub male: Option<String>,
    pub female: Option<String>,
    pub male_facegen: bool,
    pub female_facegen: bool,
}

/// A record's one-byte flags subrecord (0 without it).
fn flag_byte(record: &Record, kind: FourCC) -> u8 {
    record
        .get(kind)
        .and_then(|s| s.data.first().copied())
        .unwrap_or(0)
}

impl Armor {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<Armor> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == ARMO)?;
        let record = rr.record().ok()?;
        let slots = record
            .get(BMDT)
            .filter(|s| s.data.len() >= 4)
            .map_or(0, |s| le_u32(&s.data, 0));
        let addons = form(&rr, &record, BIPL)
            .and_then(|list| order.get(list))
            .filter(|r| r.entry.header.kind == FLST)
            .and_then(|list| {
                let record = list.record().ok()?;
                Some(
                    record
                        .get_all(LNAM)
                        .filter(|s| s.data.len() >= 4)
                        .map(|s| list.plugin.to_global(FormId(le_u32(&s.data, 0))))
                        .filter_map(|id| order.get(id))
                        .filter(|r| r.entry.header.kind == ARMA)
                        .filter_map(|r| r.record().ok())
                        .map(|r| ArmorAddon {
                            slots: r
                                .get(BMDT)
                                .filter(|s| s.data.len() >= 4)
                                .map_or(0, |s| le_u32(&s.data, 0)),
                            male: zstring(&r, MODL),
                            female: zstring(&r, MOD3),
                            male_facegen: flag_byte(&r, MODD) & 1 != 0,
                            female_facegen: flag_byte(&r, MOSD) & 1 != 0,
                        })
                        .collect(),
                )
            })
            .unwrap_or_default();
        Some(Armor {
            slots,
            male: zstring(&record, MODL),
            female: zstring(&record, MOD3),
            male_facegen: flag_byte(&record, MODD) & 1 != 0,
            female_facegen: flag_byte(&record, MOSD) & 1 != 0,
            addons,
        })
    }

    /// The models it puts on someone (its own, then its addons'), each
    /// with the slots it covers: the woman's model where it has one.
    pub fn pieces(&self, female: bool) -> Vec<(u32, String)> {
        let pick = |male: &Option<String>, f: &Option<String>| {
            if female {
                f.clone().or_else(|| male.clone())
            } else {
                male.clone()
            }
        };
        pick(&self.male, &self.female)
            .map(|m| (self.slots, m))
            .into_iter()
            .chain(
                self.addons
                    .iter()
                    .filter_map(|a| pick(&a.male, &a.female).map(|m| (a.slots, m))),
            )
            .collect()
    }

    /// For each of [`Self::pieces`], whether its model is a FaceGen part
    /// (the flag of the model picked).
    pub fn pieces_facegen(&self, female: bool) -> Vec<bool> {
        let pick = |male: &Option<String>, f: &Option<String>, mf: bool, ff: bool| {
            if female && f.is_some() {
                Some(ff)
            } else {
                male.as_ref().map(|_| mf)
            }
        };
        pick(
            &self.male,
            &self.female,
            self.male_facegen,
            self.female_facegen,
        )
        .into_iter()
        .chain(
            self.addons
                .iter()
                .filter_map(|a| pick(&a.male, &a.female, a.male_facegen, a.female_facegen)),
        )
        .collect()
    }
}

/// The race's skin texture for the skin pieces of something worn on
/// `slots`: the hand's for gloves (an addon on a hand slot and not the
/// upper body: the left hand's `INDX` 1 for the left, else the right's 2),
/// the upper body's otherwise. Which texture the game puts on a glove's
/// bare fingers isn't traced; the hand's is what those pieces name in
/// their own files (`glover.nif`'s `RightHand:0`: `HandFemale.dds`).
fn skin_for(race: Option<&Race>, female: bool, slots: u32) -> Option<String> {
    let race = race?;
    let index = if slots & slots::UPPER_BODY != 0 {
        0
    } else if slots & slots::LEFT_HAND != 0 {
        1
    } else if slots & slots::RIGHT_HAND != 0 {
        2
    } else {
        0
    };
    race.body(female, index).1.cloned()
}

/// One race part: its index, model and texture.
type RacePart = (u32, Option<String>, Option<String>);

/// A race's head and body parts: model and texture by part index, for
/// each sex, and its default face per sex.
struct Race {
    /// [male, female] × index → (model, texture).
    head: [Vec<RacePart>; 2],
    body: [Vec<RacePart>; 2],
    /// [male, female]: the race's own FaceGen shape (`FGGS`, `FGGA` after
    /// the `MNAM` / `FNAM` that follow its hair and eye lists).
    face: [Option<Face>; 2],
    /// [male, female]: the race's own FaceGen texture values (FGTS,
    /// there too).
    texture: [Vec<f32>; 2],
}

impl Race {
    fn load(order: &LoadOrder, id: FormId) -> Option<Race> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == RACE)?;
        let record = rr.record().ok()?;
        let mut race = Race {
            head: [Vec::new(), Vec::new()],
            body: [Vec::new(), Vec::new()],
            face: [None, None],
            texture: [Vec::new(), Vec::new()],
        };
        // NAM0 (head) or NAM1 (body), then MNAM (male) or FNAM (female),
        // then INDX with its MODL and ICON. After the hair and eye lists,
        // MNAM / FNAM again, each followed by that sex's FaceGen values.
        let mut section: Option<bool> = None; // Some(true) for body
        let mut sex = 0usize;
        let mut after_lists = false;
        for sub in &record.subrecords {
            match sub.kind {
                k if after_lists && k == FGTS => race.texture[sex] = floats(&sub.data),
                k if after_lists && (k == FGGS || k == FGGA) => {
                    let values = floats(&sub.data);
                    let face = race.face[sex].get_or_insert_with(|| Face {
                        symmetric: Vec::new(),
                        asymmetric: Vec::new(),
                    });
                    if k == FGGS {
                        face.symmetric = values;
                    } else {
                        face.asymmetric = values;
                    }
                }
                k if k == NAM0 => section = Some(false),
                k if k == NAM1 => section = Some(true),
                k if k == MNAM => sex = 0,
                k if k == FNAM => sex = 1,
                k if k == INDX && sub.data.len() >= 4 => {
                    let list = match section {
                        Some(true) => &mut race.body[sex],
                        Some(false) => &mut race.head[sex],
                        None => continue,
                    };
                    list.push((le_u32(&sub.data, 0), None, None));
                }
                k if k == MODL || k == ICON => {
                    let list = match section {
                        Some(true) => &mut race.body[sex],
                        Some(false) => &mut race.head[sex],
                        None => continue,
                    };
                    if let Some(last) = list.last_mut() {
                        let value = Some(sub.zstring()).filter(|s| !s.is_empty());
                        if k == MODL {
                            last.1 = value;
                        } else {
                            last.2 = value;
                        }
                    }
                }
                // The hair and eye lists end the part sections.
                k if k == HNAM || k == ENAM => {
                    section = None;
                    after_lists = true;
                }
                _ => {}
            }
        }
        Some(race)
    }

    fn part(list: &[RacePart], index: u32) -> (Option<&String>, Option<&String>) {
        list.iter()
            .find(|p| p.0 == index)
            .map_or((None, None), |p| (p.1.as_ref(), p.2.as_ref()))
    }

    fn head(&self, female: bool, index: u32) -> (Option<&String>, Option<&String>) {
        Self::part(&self.head[usize::from(female)], index)
    }

    fn body(&self, female: bool, index: u32) -> (Option<&String>, Option<&String>) {
        Self::part(&self.body[usize::from(female)], index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_made_body_tint_goes_through_its_reference() {
        let made = MadeBodyTint {
            file: "textures\\characters\\bodymods\\falloutnv.esm\\00000007modbodymale.dds".into(),
            egt: "Characters\\_Male\\UpperBodyHumanMale.egt".into(),
            values: vec![0.5, -1.25, 0.0],
        };
        let reference = made.reference();
        assert!(reference.starts_with(MADE_BODY_TINT));
        // It sits in a `facetint:<tint>|<base>` reference: no `|`.
        assert!(!reference.contains('|'));
        assert_eq!(MadeBodyTint::parse(&reference), Some(made));
        assert_eq!(MadeBodyTint::parse("textures\\a.dds"), None);
    }

    #[test]
    fn unskinned_biped_pieces_hang_from_their_slots_bone() {
        // Easy Pete's Cattleman Cowboy Hat (`CowboyHat02`, `BMDT` 0x602:
        // hair, headband, hat): the head.
        assert_eq!(slot_parent_bone(0x602), Some("Bip01 Head"));
        // Body clothes should be skinned: no bone.
        assert_eq!(slot_parent_bone(slots::UPPER_BODY), None);
        // Pip-Boy (slot 6), backpack (7), necklace (8).
        assert_eq!(slot_parent_bone(1 << 6), Some("Bip01 L ForeTwist"));
        assert_eq!(slot_parent_bone(1 << 7), Some("Bip01 Spine2"));
        assert_eq!(slot_parent_bone(1 << 8), Some("Bip01 Neck1"));
        assert_eq!(slot_parent_bone(0), None);
    }

    #[test]
    fn first_person_animations_are_named_as_the_games_files() {
        let file = |s: String| s.rsplit('\\').next().unwrap().to_string();
        // The 9mm pistol (type 3), varmint rifle (5), service rifle (6),
        // .357 (3), hunting shotgun (5), fists.
        assert_eq!(file(first_person_pose(Some(5))), "2hraim.kf");
        assert_eq!(file(first_person_pose(None)), "h2haim.kf");
        assert_eq!(file(first_person_attack(Some(3), 32)), "1hpattackright.kf");
        assert_eq!(file(first_person_attack(Some(5), 68)), "2hrattack8.kf");
        assert_eq!(file(first_person_attack(Some(6), 38)), "2haattack3.kf");
        assert_eq!(file(first_person_attack(Some(1), 255)), "1hmattackright.kf");
        assert_eq!(file(first_person_reload(Some(3), 11)), "1hpreloadl.kf");
        assert_eq!(file(first_person_reload(Some(5), 12)), "2hrreloadm.kf");
        assert_eq!(file(first_person_reload(Some(3), 20)), "1hpreloadx.kf");
        assert_eq!(file(first_person_reload(Some(5), 21)), "2hrreloady.kf");
        assert_eq!(file(first_person_ready(Some(3), true)), "1hpequip.kf");
        assert_eq!(file(first_person_ready(Some(3), false)), "1hpunequip.kf");
        assert_eq!(file(first_person_ready(None, true)), "h2hequip.kf");
        assert_eq!(file(first_person_ready(Some(1), false)), "1hmunequip.kf");
        assert_eq!(
            first_person_hand("Characters\\_Male\\LeftHand.NIF"),
            "Characters\\_Male\\LeftHand1st.NIF"
        );
    }

    #[test]
    fn idles_sit_by_their_skeleton() {
        assert_eq!(
            idle_animation("Characters\\_Male\\Skeleton.NIF", false),
            "Characters\\_Male\\locomotion\\mtidle.kf"
        );
        assert_eq!(
            idle_animation("Creatures\\Gecko\\Skeleton.NIF", true),
            "Creatures\\Gecko\\mtidle.kf"
        );
        assert_eq!(
            walk_animation("Characters\\_Male\\Skeleton.NIF", false, true),
            "Characters\\_Male\\locomotion\\female\\mtforward.kf"
        );
        assert_eq!(
            walk_animation("Creatures\\Molerat\\Skeleton.NIF", true, false),
            "Creatures\\Molerat\\locomotion\\mtforward.kf"
        );
    }
}
