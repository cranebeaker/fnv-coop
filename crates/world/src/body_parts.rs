//! Body parts: which part of a person or creature a hit lands on, what it
//! does to that part, and what a crippled part does to them. Read from the
//! game's code (`%USERPROFILE%\nv-re\findings\hits.md` §3–§5, and the
//! functions named below) and its records.
//!
//! **Body part data** (`BPTD`): `MODL` the skeleton it's for, then one
//! group per part: `BPTN` its name ("Head"), `BPNN` the node it starts at
//! ("Bip01 Neck1"), `BPNT` the node VATS aims at, `BPNI` where its IK
//! starts, `BPND` (84 bytes, loaded as-is into the part at `+0x5c`: damage
//! multiplier f32 at 0, flags u8 at 4 — 0x01 severable, 0x08 explodable,
//! 0x40 its own explode chance — part type u8 at 5, health % u8 at 6, actor
//! value i8 at 7, V.A.T.S. to-hit chance u8 at 8, explode chance u8 at 9),
//! `NAM1` the limb's gore model,
//! `NAM4` the gore's bone, `NAM5` texture hashes; `RAGA` a ragdoll
//! (loader `005e4d80` / `005e4100`). Parts are kept by type (15 slots,
//! `005e5220`): a later part of a type already in use replaces it.
//!
//! Part types (the game's `BGSBodyPart::LIMB_ENUM`): 0 torso, 1 head, 2 a
//! second head part, 3–4 left arm, 5–6 right arm, 7–9 left leg, 10–12 right
//! leg, 13 brain, 14 the weapon held. People use `DefaultBodyPartData`
//! (`0000001D`; head ×2, 20% of health; torso 60%; arms and legs 25%), the
//! player `PlayerBodyPartData` (`0000001C`: head ×1, 75%; legs 150%; torso
//! 255%) when it exists, creatures their `CREA` `PNAM`, else the default
//! (`005f0f80`, `005fa2e0`, the defaults set up at `005e53a0`).
//!
//! **Where a hit lands** (`008b3ef0`): from the node the hit struck (the
//! bone carrying the Havok body it met), up through its parents to the
//! actor's root (not counted): the first node that is a part's `BPNN` node
//! (found by name in the actor's 3D when it loads, `0092b830`) gives that
//! part; a node whose parent is the weapon's node (the process's, taken
//! here as the skeleton's `Weapon` bone: not traced further) gives 14; none
//! gives no part. So a shot to `Bip01 Head` is the head (`Bip01 Neck1` is
//! its parent), to `Bip01 L Forearm` the left arm (`Bip01 L UpperArm`), to
//! the spine the torso (`Bip01`), and on a gecko the pelvis is no part at
//! all (its torso starts at `Bip01 Spine1`, the pelvis's child).
//!
//! **What it does** ([`part_hit`], `009b6620`): the hit's multiplier (0 to
//! begin with) becomes the part's damage multiplier for ranged hits, 1 for
//! melee weapons and fists; the part loses `100 × damage ÷ (health % ÷ 100
//! × the target's base health)` condition points (`00647a30`), × 0.5
//! (`fCombatPlayerLimbDamageMult`) for the player, × the weapon's limb
//! damage multiplier (`WEAP` `DNAM` f32 at 116); damage after armour,
//! before the multiplier. The multiplier comes last (`009b73d0`): above 0,
//! a sneak attack's bonus if it's at least 1, then the damage × it; with
//! no part nothing is multiplied, not even a sneak attack. A hit on the
//! weapon (14) moves all the damage onto the weapon.
//!
//! **Crippled** ([`hurt_part`], `0089a760`): the part's actor value (25–31,
//! `PerceptionCondition` … `BrainCondition`) goes from above 0 to 0 or
//! less. A person (not the player) whose right arm (5, 6) is crippled
//! drops their weapon, and with a two-handed one the left arm (3, 4) too,
//! unless they ignore crippled limbs (actor value 72). Crippled legs slow
//! them ([`leg_speed_mult`], `00647d10`): × `fMoveOneCrippledLegSpeedMult`
//! 0.85 for one, × `fMoveTwoCrippledLegsSpeedMult` 0.75 for both.

use esm::{FormId, FourCC, LoadOrder};

use crate::cell::le_f32;
use crate::combat::Weapon;
use crate::dialogue::{PLAYER_BASE, PLAYER_REF};
use crate::scripting::{base_of, game_setting, Facts, GameState};

const BPTD: FourCC = FourCC::new(b"BPTD");
const CREA: FourCC = FourCC::new(b"CREA");
const MODL: FourCC = FourCC::new(b"MODL");
const BPTN: FourCC = FourCC::new(b"BPTN");
const BPNN: FourCC = FourCC::new(b"BPNN");
const BPNT: FourCC = FourCC::new(b"BPNT");
const BPNI: FourCC = FourCC::new(b"BPNI");
const BPND: FourCC = FourCC::new(b"BPND");
const NAM1: FourCC = FourCC::new(b"NAM1");
const NAM2: FourCC = FourCC::new(b"NAM2");
const NAM3: FourCC = FourCC::new(b"NAM3");
const NAM4: FourCC = FourCC::new(b"NAM4");
const NAM5: FourCC = FourCC::new(b"NAM5");
const PNAM: FourCC = FourCC::new(b"PNAM");
const RAGA: FourCC = FourCC::new(b"RAGA");

/// People's body part data (`DefaultBodyPartData`), also for creatures
/// without their own.
pub const DEFAULT_BODY_PART_DATA: FormId = FormId(0x1D);
/// The player's (`PlayerBodyPartData`), when the game's files have it.
pub const PLAYER_BODY_PART_DATA: FormId = FormId(0x1C);

/// Part types (`BGSBodyPart::LIMB_ENUM`, as `GetHitLocation` names them).
pub mod part {
    pub const TORSO: u8 = 0;
    pub const HEAD: u8 = 1;
    pub const HEAD2: u8 = 2;
    pub const LEFT_ARM: u8 = 3;
    pub const LEFT_ARM2: u8 = 4;
    pub const RIGHT_ARM: u8 = 5;
    pub const RIGHT_ARM2: u8 = 6;
    pub const LEFT_LEG: u8 = 7;
    pub const LEFT_LEG2: u8 = 8;
    pub const LEFT_LEG3: u8 = 9;
    pub const RIGHT_LEG: u8 = 10;
    pub const RIGHT_LEG2: u8 = 11;
    pub const RIGHT_LEG3: u8 = 12;
    pub const BRAIN: u8 = 13;
    /// The weapon in their hands.
    pub const WEAPON: u8 = 14;
    /// How many types there are (the game's 15 slots).
    pub const COUNT: usize = 15;
}

/// `BPND` flags the code reads.
pub mod flags {
    pub const SEVERABLE: u8 = 0x01;
    /// The part has IK data (the parts `005e5320` walks).
    pub const IK_DATA: u8 = 0x02;
    /// `008b4360`.
    pub const EXPLODABLE: u8 = 0x08;
    /// Uses its own explode chance (`008b4cd0`).
    pub const OWN_EXPLODE_CHANCE: u8 = 0x40;
    /// Head tracking (`0087e940`): the part whose IK start sets up LookIK.
    pub const HEAD_TRACKING: u8 = 0x20;
}

/// Actor values for the body part conditions (`PerceptionCondition` 25 …
/// `BrainCondition` 31) and for ignoring crippled limbs (72).
pub mod av {
    pub const FIRST_CONDITION: u16 = 25;
    pub const LAST_CONDITION: u16 = 31;
    pub const LEFT_MOBILITY: u16 = 29;
    pub const RIGHT_MOBILITY: u16 = 30;
    pub const IGNORE_CRIPPLED_LIMBS: u16 = 72;
}

/// One part of a body.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyPart {
    /// `BPTN` ("Left Arm").
    pub name: String,
    /// `BPNN`: the node the part starts at ("Bip01 L UpperArm").
    pub node: String,
    /// `BPNT`: the node VATS aims at.
    pub target: String,
    /// `BPNI`: the node its IK starts at (`Bip01 Head` for the human
    /// head; part `+0x14`, read by `0063d040`).
    pub ik_start: Option<String>,
    /// `BPND` f32 at 0x14: the head-tracking maximum angle in degrees
    /// (part `+0x70`, `0051b860`, read by `0087e580`).
    pub tracking_max_angle: f32,
    /// `BPND` f32 at 0: the damage multiplier for ranged hits (head ×2).
    pub damage_mult: f32,
    /// `BPND` u8 at 4 ([`flags`]).
    pub flags: u8,
    /// `BPND` u8 at 5 ([`part`]).
    pub part_type: u8,
    /// `BPND` u8 at 6: the part's share of the body's health, in percent.
    pub health_percent: u8,
    /// `BPND` i8 at 7: its condition's actor value (25–31; -1 none).
    pub actor_value: i8,
    /// `BPND` u8 at 8: V.A.T.S.'s chance to hit it, in percent (part
    /// `+0x64`, read by `005dc980` for `world::vats::hit_chance`;
    /// `DefaultBodyPartData`: head 30, torso 60, arms 45, legs 50).
    pub to_hit_chance: u8,
    /// `BPND` u8 at 9.
    pub explode_chance: u8,
    /// `NAM1`: the limb's gore model.
    pub limb_model: Option<String>,
}

/// A body part data record (`BPTD`).
#[derive(Debug, Clone, PartialEq)]
pub struct BodyPartData {
    pub form_id: FormId,
    pub editor_id: Option<String>,
    /// `MODL`: the skeleton it's made for.
    pub model: Option<String>,
    /// The parts by type (the game's 15 slots).
    pub parts: Vec<Option<BodyPart>>,
    /// `RAGA`: the ragdoll record.
    pub ragdoll: Option<FormId>,
}

impl BodyPartData {
    pub fn load(order: &LoadOrder, id: FormId) -> Option<BodyPartData> {
        let rr = order.get(id).filter(|r| r.entry.header.kind == BPTD)?;
        let record = rr.record().ok()?;
        let mut data = BodyPartData::from_subrecords(id, &record.subrecords);
        data.ragdoll = data.ragdoll.map(|r| rr.plugin.to_global(r));
        Some(data)
    }

    /// The parts from a record's subrecords, as the game's loader reads
    /// them (`005e4d80`): a `BPTN`, `BPNN` or `PNAM` starts a part, read in
    /// order (`005e4100`): `BPTN` and `PNAM` if there, then `BPNN` and
    /// `BPNT`, `BPNI` if there, `BPND`, then the gore subrecords. A part
    /// missing `BPNN`, `BPNT` or `BPND` stops the loading there ("Load
    /// failed for body part data"); the parts before it stay.
    pub fn from_subrecords(id: FormId, subrecords: &[esm::Subrecord]) -> BodyPartData {
        let mut data = BodyPartData {
            form_id: id,
            editor_id: None,
            model: None,
            parts: vec![None; part::COUNT],
            ragdoll: None,
        };
        let mut i = 0;
        while let Some(sub) = subrecords.get(i) {
            if sub.kind == BPTN || sub.kind == BPNN || sub.kind == PNAM {
                let Some(p) = read_part(subrecords, &mut i) else {
                    break;
                };
                // An invalid type is left out; a type in use is replaced.
                if let Some(slot) = data.parts.get_mut(usize::from(p.part_type)) {
                    *slot = Some(p);
                }
                continue;
            }
            if sub.kind == esm::sig::EDID {
                data.editor_id = Some(sub.zstring());
            } else if sub.kind == MODL {
                data.model = Some(sub.zstring());
            } else if sub.kind == RAGA && sub.data.len() >= 4 {
                data.ragdoll = Some(FormId(crate::cell::le_u32(&sub.data, 0))).filter(|f| f.0 != 0);
            }
            i += 1;
        }
        data
    }

    /// The body part data a person or creature uses (see the module notes).
    pub fn of(order: &LoadOrder, who: FormId) -> Option<BodyPartData> {
        BodyPartData::load(order, data_form(order, who)?)
    }

    /// The part that sets up head tracking (`0087e130`): the first, in
    /// slot order, with IK data and the head-tracking flag.
    pub fn head_tracking_part(&self) -> Option<&BodyPart> {
        self.parts
            .iter()
            .flatten()
            .find(|p| p.flags & flags::IK_DATA != 0 && p.flags & flags::HEAD_TRACKING != 0)
    }

    /// The part of a type (`005e50f0`).
    pub fn part(&self, part_type: u8) -> Option<&BodyPart> {
        self.parts.get(usize::from(part_type))?.as_ref()
    }

    /// The part whose condition is an actor value (`005e5130`).
    pub fn part_by_actor_value(&self, actor_value: u16) -> Option<&BodyPart> {
        self.parts
            .iter()
            .flatten()
            .find(|p| i32::from(p.actor_value) == i32::from(actor_value))
    }

    /// Where a hit on a bone of the skeleton lands (`008b3ef0`): from that
    /// bone up through its parents, the root not counted, the first that is
    /// a part's node (`BPNN`; names compared ignoring case) gives the part;
    /// a bone whose parent is `Weapon` (the weapon's own model) gives the
    /// weapon; `None` if none is.
    pub fn part_of_bone(&self, bones: &[nif::Bone], bone: usize) -> Option<u8> {
        let mut at = Some(bone);
        let mut steps = 0;
        while let Some(i) = at {
            let b = bones.get(i)?;
            // The actor's root isn't looked at.
            let parent = b.parent?;
            if bones
                .get(parent)
                .is_some_and(|p| p.name.eq_ignore_ascii_case("Weapon"))
            {
                return Some(part::WEAPON);
            }
            if let Some(p) = self
                .parts
                .iter()
                .flatten()
                .find(|p| p.node.eq_ignore_ascii_case(&b.name))
            {
                return Some(p.part_type);
            }
            at = Some(parent);
            steps += 1;
            if steps > bones.len() {
                return None;
            }
        }
        None
    }
}

/// One part, read in the loader's order from `subrecords[*i]`; `*i` ends
/// past it. `None` when a required subrecord isn't where it should be.
fn read_part(subrecords: &[esm::Subrecord], i: &mut usize) -> Option<BodyPart> {
    fn take<'a>(
        subrecords: &'a [esm::Subrecord],
        i: &mut usize,
        kind: FourCC,
    ) -> Option<&'a esm::Subrecord> {
        let s = subrecords.get(*i).filter(|s| s.kind == kind)?;
        *i += 1;
        Some(s)
    }
    let s = subrecords;
    let name = take(s, i, BPTN).map(|s| s.zstring()).unwrap_or_default();
    take(s, i, PNAM);
    let node = take(s, i, BPNN)?.zstring();
    let target = take(s, i, BPNT)?.zstring();
    let ik_start = take(s, i, BPNI).map(|s| s.zstring());
    let d = &take(s, i, BPND)?.data;
    if d.len() < 84 {
        return None;
    }
    let limb_model = take(s, i, NAM1)
        .map(|s| s.zstring())
        .filter(|s| !s.is_empty());
    // Older forms' NAM2 and NAM3, the gore's bone, the texture hashes.
    for kind in [NAM2, NAM3, NAM4, NAM5] {
        take(s, i, kind);
    }
    Some(BodyPart {
        name,
        node,
        target,
        ik_start,
        tracking_max_angle: le_f32(d, 0x14),
        damage_mult: le_f32(d, 0),
        flags: d[4],
        part_type: d[5],
        health_percent: d[6],
        actor_value: d[7] as i8,
        to_hit_chance: d[8],
        explode_chance: d[9],
        limb_model,
    })
}

/// Which body part data someone (a placed person or creature, or their
/// base record) uses: a creature its `PNAM`, else `DefaultBodyPartData`;
/// the player `PlayerBodyPartData` when the files have it; other people
/// the default.
pub fn data_form(order: &LoadOrder, who: FormId) -> Option<FormId> {
    let is_base = order
        .get(who)
        .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"NPC_" | b"CREA"));
    let base = if is_base { who } else { base_of(order, who)? };
    let exists = |f: FormId| order.get(f).is_some_and(|r| r.entry.header.kind == BPTD);
    let rr = order.get(base)?;
    if rr.entry.header.kind == CREA {
        let own = rr
            .record()
            .ok()?
            .get(PNAM)
            .filter(|s| s.data.len() >= 4)
            .map(|s| rr.plugin.to_global(FormId(crate::cell::le_u32(&s.data, 0))))
            .filter(|f| f.0 != 0 && exists(*f));
        return own.or(Some(DEFAULT_BODY_PART_DATA).filter(|f| exists(*f)));
    }
    if base == PLAYER_BASE && exists(PLAYER_BODY_PART_DATA) {
        return Some(PLAYER_BODY_PART_DATA);
    }
    Some(DEFAULT_BODY_PART_DATA).filter(|f| exists(*f))
}

/// Whether someone (a placed person or creature, or their base record)
/// is a creature (`Actor::IsCreature`, vtable `+0x21c`: `Creature` only).
pub fn is_creature(order: &LoadOrder, who: FormId) -> bool {
    let is_base = order
        .get(who)
        .is_some_and(|r| matches!(r.entry.header.kind.as_bytes(), b"NPC_" | b"CREA"));
    let base = if is_base {
        Some(who)
    } else {
        base_of(order, who)
    };
    base.and_then(|b| order.get(b))
        .is_some_and(|r| r.entry.header.kind == CREA)
}

/// The condition points a hit takes off a part (`00647a30`): `100 ×
/// damage ÷ (health % ÷ 100 × base health)`; nothing for a part with no
/// share of health; the damage itself when the base health isn't above 0.
pub fn limb_damage(base_health: f32, health_percent: u8, damage: f32) -> f32 {
    if base_health <= 0.0 {
        return damage;
    }
    let share = f32::from(health_percent) * 0.01;
    if share > 0.0 {
        damage / (share * base_health) * 100.0
    } else {
        0.0
    }
}

/// What a hit does at the part it landed on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PartHit {
    /// The part's type, if one was found.
    pub part: Option<u8>,
    /// The hit's multiplier (the game's hit data `+0x5c`): 0 when no part
    /// of the body was found (then nothing is multiplied).
    pub multiplier: f32,
    /// The health damage left (none when the weapon took it).
    pub health_damage: f32,
    /// Condition points the part loses.
    pub limb_damage: f32,
    /// Condition points the target's weapon loses (a hit on it, part 14).
    pub weapon_damage: f32,
}

/// What a hit of `damage` (after armour) does landing on `part`
/// (`009b6620`; see the module notes): the multiplier — the part's damage
/// multiplier for ranged hits, 1 for melee weapons, fists and bites, never
/// below 0 — and the condition points the part loses, × 0.5 for the player
/// (`fCombatPlayerLimbDamageMult`) and × the weapon's limb damage
/// multiplier. A part type this body hasn't got counts as the part hit but
/// does nothing.
pub fn part_hit(
    order: &LoadOrder,
    state: &GameState,
    target: FormId,
    weapon: Option<&Weapon>,
    part_type: Option<u8>,
    damage: f32,
) -> PartHit {
    let mut hit = PartHit {
        part: part_type,
        multiplier: 0.0,
        health_damage: damage,
        limb_damage: 0.0,
        weapon_damage: 0.0,
    };
    let Some(p) = part_type else {
        return hit;
    };
    if p == part::WEAPON {
        hit.weapon_damage = damage;
        hit.health_damage = 0.0;
        return hit;
    }
    let Some(data) = BodyPartData::of(order, target) else {
        return hit;
    };
    let Some(bp) = data.part(p) else {
        return hit;
    };
    let base = crate::combat::base_health(order, state, target).unwrap_or(0.0) as f32;
    let mut limb = limb_damage(base, bp.health_percent, damage);
    if target == PLAYER_REF {
        limb *= game_setting(order, "fCombatPlayerLimbDamageMult").unwrap_or(0.5);
    }
    if let Some(w) = weapon {
        limb *= w.limb_damage_mult;
    }
    hit.limb_damage = limb;
    let melee = weapon.map_or(true, Weapon::is_melee);
    hit.multiplier = if melee { 1.0 } else { bp.damage_mult }.max(0.0);
    hit
}

/// What hurting a part did.
#[derive(Debug, Clone, PartialEq)]
pub struct PartHurt {
    /// The part's name (`BPTN`).
    pub name: String,
    /// The condition points it lost.
    pub lost: f32,
    /// It went from above 0 to 0 or less.
    pub crippled: bool,
    /// The weapon they dropped for it.
    pub dropped: Option<FormId>,
    /// The hit staggered them (`0089a760`): the part newly crippled, an
    /// already crippled torso hit again (a chance), or an unarmed
    /// uppercut (`world::melee::Special::Stagger`). The stagger itself is
    /// the animation's.
    pub staggered: bool,
}

/// The part a hit landed on loses its condition points (`0089a760`): the
/// limb damage (× `fCrossSkillDamageMultiplier` 2.5 for an unarmed
/// cross, `world::melee::Special::Cross`), then the one hit's perks'
/// "Adjust Limb Damage" (entry point 6); the part's actor value is
/// damaged by the rest. Crippled when it goes from above 0 to 0 or less.
/// The hit staggers when the part was newly crippled, or, on a torso
/// already crippled, with a chance of `iCombatCrippledTorsoHitStaggerChance`
/// (50: a roll % 100 at most it) — neither for someone who ignores
/// crippled limbs (actor value 72) — and always for an unarmed uppercut
/// (`world::melee::Special::Stagger`). A staggered person (not the
/// player, not essential) holding a weapon drops it when the right arm
/// was hit, or the left holding a two-handed weapon. A hit on the weapon
/// (14) takes its condition points off the weapon in their hands, and a
/// critical one makes a person drop it (`iWeaponCriticalHitDropChance`
/// 100%).
///
/// Dropping, here: they stop using that weapon (it stays in their
/// inventory, its model hidden; the game lays it on the ground beside
/// them, which isn't done).
// Translated from 0089a760 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn hurt_part(
    order: &LoadOrder,
    state: &mut GameState,
    target: FormId,
    hit: &PartHit,
    critical: bool,
    attacker: (FormId, Option<FormId>),
    special: crate::melee::Special,
) -> Option<PartHurt> {
    let p = hit.part?;
    if p == part::WEAPON {
        let held = crate::combat::weapon_in_hand(order, state, target)?;
        // Through the actor's `DamageItem` (`00891360`), with their perks.
        crate::combat::damage_weapon(order, state, target, &held, hit.weapon_damage);
        let mut dropped = None;
        if critical && target != PLAYER_REF {
            let chance = game_setting(order, "iWeaponCriticalHitDropChance").unwrap_or(100.0);
            if ((state.roll() % 100) as f32) < chance {
                drop_weapon(state, target, held.form_id);
                dropped = Some(held.form_id);
            }
        }
        return Some(PartHurt {
            name: "Weapon".into(),
            lost: hit.weapon_damage,
            crippled: false,
            dropped,
            staggered: false,
        });
    }
    let data = BodyPartData::of(order, target)?;
    let bp = data.part(p)?;
    let value = u16::try_from(bp.actor_value).ok().filter(|v| *v < 77)?;
    // The one hit's perks' "Adjust Limb Damage" (entry point 6:
    // Adamantium Skeleton × 0.5), asked about the attacker and the
    // attacker's weapon (the fists when none, as `0089a760` passes it).
    let specials = crate::melee::UnarmedSpecials::read(order);
    let limb_damage = if special == crate::melee::Special::Cross {
        hit.limb_damage * specials.cross_limb_mult
    } else {
        hit.limb_damage
    };
    let lost = crate::perks::apply_for(
        order,
        state,
        target,
        crate::perks::entry::ADJUST_LIMB_DAMAGE,
        limb_damage,
        &[
            crate::perks::Tab::Target(attacker.0),
            crate::perks::weapon_tab(attacker.1),
        ],
    );
    let condition = |state: &GameState| {
        Facts {
            order,
            state,
            speaker: None,
        }
        .current_actor_value(target, value)
        .unwrap_or(0.0)
    };
    let before = condition(state);
    if lost != 0.0 {
        *state.value_damage.entry((target, value)).or_insert(0.0) += f64::from(lost);
    }
    let now = condition(state);
    let crippled = before > 0.0 && now <= 0.0;
    let mut staggered = crippled;
    if p == part::TORSO && !crippled && now <= 0.0 {
        staggered = ((state.roll() % 100) as f32) <= specials.torso_stagger_chance;
    }
    if ignores_crippled_limbs(order, state, target) {
        staggered = false;
    }
    if special == crate::melee::Special::Stagger {
        staggered = true;
    }
    let mut dropped = None;
    if staggered
        && target != PLAYER_REF
        && !crate::more_functions::is_essential(order, state, target)
    {
        if let Some(w) = crate::combat::weapon_in_hand(order, state, target) {
            let arm = matches!(p, part::RIGHT_ARM | part::RIGHT_ARM2)
                || (w.two_handed() && matches!(p, part::LEFT_ARM | part::LEFT_ARM2));
            if arm {
                drop_weapon(state, target, w.form_id);
                dropped = Some(w.form_id);
            }
        }
    }
    Some(PartHurt {
        name: bp.name.clone(),
        lost,
        crippled,
        dropped,
        staggered,
    })
}

/// Someone stops using a weapon (see [`hurt_part`]).
fn drop_weapon(state: &mut GameState, who: FormId, weapon: FormId) {
    state.unequip(who, weapon);
    state.dropped.insert((who, weapon));
}

/// Whether someone ignores crippled limbs (actor value 72 above 0).
pub fn ignores_crippled_limbs(order: &LoadOrder, state: &GameState, who: FormId) -> bool {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, av::IGNORE_CRIPPLED_LIMBS)
    .is_some_and(|v| v as i64 != 0)
}

/// Whether a body part condition (actor value 25–31) is crippled: at most
/// 0 (`00646800`).
pub fn is_crippled(order: &LoadOrder, state: &GameState, who: FormId, value: u16) -> bool {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, value)
    .is_some_and(|v| v.min(100.0) <= 0.0)
}

/// How much crippled legs slow someone (`00647d10`): × 0.85
/// (`fMoveOneCrippledLegSpeedMult`) with one crippled, × 0.75
/// (`fMoveTwoCrippledLegsSpeedMult`) with both, × 1 when they ignore
/// crippled limbs.
pub fn leg_speed_mult(order: &LoadOrder, state: &GameState, who: FormId) -> f32 {
    let crippled = [av::LEFT_MOBILITY, av::RIGHT_MOBILITY]
        .into_iter()
        .filter(|&v| is_crippled(order, state, who, v))
        .count();
    if crippled == 0 || ignores_crippled_limbs(order, state, who) {
        return 1.0;
    }
    if crippled == 1 {
        game_setting(order, "fMoveOneCrippledLegSpeedMult").unwrap_or(0.85)
    } else {
        game_setting(order, "fMoveTwoCrippledLegsSpeedMult").unwrap_or(0.75)
    }
}

/// The names of someone's crippled parts, by their body part data (for
/// messages and the HUD).
pub fn crippled_parts(order: &LoadOrder, state: &GameState, who: FormId) -> Vec<String> {
    let Some(data) = BodyPartData::of(order, who) else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for value in av::FIRST_CONDITION..=av::LAST_CONDITION {
        if !is_crippled(order, state, who, value) {
            continue;
        }
        if let Some(p) = data.part_by_actor_value(value) {
            names.push(p.name.clone());
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use esm::Subrecord;

    fn sub(kind: &[u8; 4], data: &[u8]) -> Subrecord {
        Subrecord {
            kind: FourCC::new(kind),
            data: data.to_vec(),
        }
    }

    fn zstr(s: &str) -> Vec<u8> {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        v
    }

    /// One part's subrecords, as the game's records lay them out.
    fn part(name: &str, node: &str, kind: u8, mult: f32, health: u8, value: i8) -> Vec<Subrecord> {
        let mut bpnd = vec![0u8; 84];
        bpnd[0..4].copy_from_slice(&mult.to_le_bytes());
        bpnd[4] = flags::EXPLODABLE;
        bpnd[5] = kind;
        bpnd[6] = health;
        bpnd[7] = value as u8;
        bpnd[8] = 45;
        bpnd[9] = 30;
        vec![
            sub(b"BPTN", &zstr(name)),
            sub(b"BPNN", &zstr(node)),
            sub(b"BPNT", &zstr(node)),
            sub(b"BPNI", &zstr(node)),
            sub(b"BPND", &bpnd),
            sub(b"NAM1", &zstr("Gore\\Bits.NIF")),
            sub(b"NAM4", &zstr(node)),
            sub(b"NAM5", &[]),
        ]
    }

    fn bones(names: &[(&str, Option<usize>)]) -> Vec<nif::Bone> {
        names
            .iter()
            .map(|&(name, parent)| nif::Bone {
                name: name.into(),
                parent,
                local: nif::Transform::IDENTITY,
            })
            .collect()
    }

    #[test]
    fn parts_are_read_by_type_and_a_later_one_replaces_an_earlier() {
        let mut subs = vec![
            sub(b"EDID", &zstr("TestBody")),
            sub(b"MODL", &zstr("Characters\\_Male\\skeleton.NIF")),
        ];
        subs.extend(part("Head", "Bip01 Neck1", 1, 2.0, 20, 25));
        subs.extend(part("Torso", "Bip01", 0, 1.0, 60, 26));
        subs.extend(part("Old Left Arm", "Bip01 L Clavicle", 3, 1.0, 10, 27));
        subs.extend(part("Left Arm", "Bip01 L UpperArm", 3, 1.0, 25, 27));
        subs.push(sub(b"RAGA", &0x658D6u32.to_le_bytes()));
        let data = BodyPartData::from_subrecords(FormId(0x1D), &subs);
        assert_eq!(data.editor_id.as_deref(), Some("TestBody"));
        assert_eq!(
            data.model.as_deref(),
            Some("Characters\\_Male\\skeleton.NIF")
        );
        assert_eq!(data.ragdoll, Some(FormId(0x658D6)));
        let head = data.part(part::HEAD).unwrap();
        assert_eq!(
            (head.name.as_str(), head.node.as_str(), head.damage_mult),
            ("Head", "Bip01 Neck1", 2.0)
        );
        assert_eq!((head.health_percent, head.actor_value), (20, 25));
        assert_eq!(head.explode_chance, 30);
        // V.A.T.S.'s to-hit chance, the byte before it.
        assert_eq!(head.to_hit_chance, 45);
        assert_eq!(head.limb_model.as_deref(), Some("Gore\\Bits.NIF"));
        // The later left arm replaced the earlier.
        assert_eq!(data.part(part::LEFT_ARM).unwrap().name, "Left Arm");
        assert!(data.part(part::RIGHT_ARM).is_none());
        assert_eq!(data.part_by_actor_value(26).unwrap().name, "Torso");
    }

    #[test]
    fn a_part_missing_its_data_stops_the_loading() {
        let mut subs = part("Head", "Bip01 Neck1", 1, 2.0, 20, 25);
        // A part with no BPND, then one that would be fine.
        subs.push(sub(b"BPTN", &zstr("Broken")));
        subs.push(sub(b"BPNN", &zstr("Bip01")));
        subs.push(sub(b"BPNT", &zstr("Bip01")));
        subs.extend(part("Torso", "Bip01", 0, 1.0, 60, 26));
        let data = BodyPartData::from_subrecords(FormId(0x1D), &subs);
        assert!(data.part(part::HEAD).is_some());
        // The game stops at the broken part: the torso after it isn't read.
        assert!(data.part(part::TORSO).is_none());
    }

    #[test]
    fn hits_land_on_the_first_part_node_above_the_bone() {
        let mut subs = part("Head", "Bip01 Neck1", 1, 2.0, 20, 25);
        subs.extend(part("Torso", "Bip01", 0, 1.0, 60, 26));
        subs.extend(part("Left Arm", "Bip01 L UpperArm", 3, 1.0, 25, 27));
        let data = BodyPartData::from_subrecords(FormId(0x1D), &subs);
        let skeleton = bones(&[
            ("Scene Root", None),          // 0
            ("Bip01", Some(0)),            // 1
            ("Bip01 Spine2", Some(1)),     // 2
            ("Bip01 Neck1", Some(2)),      // 3
            ("Bip01 Head", Some(3)),       // 4
            ("Bip01 L UpperArm", Some(2)), // 5
            ("Bip01 L Forearm", Some(5)),  // 6
            ("Bip01 R Hand", Some(2)),     // 7
            ("Weapon", Some(7)),           // 8
            ("WeaponModel", Some(8)),      // 9
            ("Camera3rd", Some(0)),        // 10
        ]);
        assert_eq!(data.part_of_bone(&skeleton, 4), Some(part::HEAD));
        assert_eq!(data.part_of_bone(&skeleton, 6), Some(part::LEFT_ARM));
        assert_eq!(data.part_of_bone(&skeleton, 2), Some(part::TORSO));
        // No right arm here: up to the torso.
        assert_eq!(data.part_of_bone(&skeleton, 7), Some(part::TORSO));
        // The weapon's own model.
        assert_eq!(data.part_of_bone(&skeleton, 9), Some(part::WEAPON));
        // Straight under the root, and the root itself: nothing.
        assert_eq!(data.part_of_bone(&skeleton, 10), None);
        assert_eq!(data.part_of_bone(&skeleton, 0), None);
    }

    #[test]
    fn limb_damage_is_the_hits_share_of_the_parts_health() {
        // 10 damage on a head (20%) of a 100-health body: 50 points.
        assert!((limb_damage(100.0, 20, 10.0) - 50.0).abs() < 1e-4);
        // A leg (25%) of 200: 10 → 20.
        assert!((limb_damage(200.0, 25, 10.0) - 20.0).abs() < 1e-4);
        // No share: nothing; no base health: the damage itself.
        assert_eq!(limb_damage(100.0, 0, 10.0), 0.0);
        assert_eq!(limb_damage(0.0, 20, 10.0), 10.0);
    }
}
