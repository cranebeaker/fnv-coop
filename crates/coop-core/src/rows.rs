//! The rows of one player, and their wire format.
//!
//! A game that keeps rows for each player sends them as one blob for a player. The blob
//! travels in the shared state as `Field::Custom` chunks on a form of its own
//! ([`rows_form`]), under the tags [`TAG_BASE`] and up. Each game adapter writes the same
//! bytes for the same rows, or the hashes of two machines differ.
//!
//! [`PlayerRows`] is the model, [`encode`] and [`decode`] are the codec, and [`chunks_of`]
//! and [`blob_of`] cut the blob into fields and join it again. The module names no engine
//! type.

use std::collections::{BTreeMap, BTreeSet};

use crate::ids::FormId;
use crate::schema::{Field, Key};

/// The form `ROWS_FORM_BASE + slot` carries the rows of player `slot`.
pub const ROWS_FORM_BASE: u32 = 0xFC00_0000;
/// The number of players that have a rows form.
pub const ROWS_FORMS: u32 = 8;
/// The custom tag of the first chunk of a blob.
pub const TAG_BASE: u16 = 0x0100;
/// One past the tag of the last chunk.
pub const TAG_END: u16 = 0x0F00;
/// The number of bytes in a chunk. It is less than `schema::MAX_CUSTOM_BYTES`.
pub const CHUNK: usize = 480;
/// The version of the wire format. It is the first byte of each blob.
pub const VERSION: u8 = 1;

/// The form that carries the rows of player `slot`.
pub fn rows_form(slot: u8) -> FormId {
    FormId(ROWS_FORM_BASE + u32::from(slot))
}

/// The player whose rows a form carries. `None`: the form is not a rows form.
pub fn slot_of_rows_form(form: FormId) -> Option<u8> {
    let d = form.0.wrapping_sub(ROWS_FORM_BASE);
    (d < ROWS_FORMS).then_some(d as u8)
}

/// Each row that belongs to one player.
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerRows {
    /// Actor values. Karma is not here: the party shares it.
    pub actor_values: BTreeMap<u16, f64>,
    /// Damage of actor values (health, radiation and others).
    pub value_damage: BTreeMap<u16, f64>,
    /// Health that the player lost. `None`: none.
    pub damage: Option<f64>,
    /// True when the player is dead.
    pub dead: bool,
    /// The items that the player holds: item, to count.
    pub items: BTreeMap<FormId, i32>,
    /// True when the inventory was copied from the player's record.
    pub stocked: bool,
    /// The equipped items. `None`: the game did not set them.
    pub equipped: Option<Vec<FormId>>,
    /// The condition of items (the bits of an `f32`).
    pub weapon_health: BTreeMap<FormId, u32>,
    /// The mods that are on weapons.
    pub weapon_mods: BTreeMap<FormId, u8>,
    /// The weapons that the player dropped.
    pub dropped: BTreeSet<FormId>,
    /// The items that the player cannot take off.
    pub equip_locked: BTreeSet<FormId>,
    /// The ammunition that the player chose.
    pub ammo_loaded: Option<FormId>,
    /// The hot keys.
    pub hotkeys: [Option<FormId>; 8],
    /// The statistics of the Pip-Boy.
    pub misc_stats: BTreeMap<u8, u32>,
    /// The skill points that the player put into skills.
    pub skill_points: BTreeMap<u16, u32>,
    /// The tag skills.
    pub tag_skills: BTreeSet<u16>,
    /// The tag slots.
    pub tag_slots: BTreeMap<u8, u16>,
    /// The level.
    pub level: u16,
    /// True when a level-up waits.
    pub level_up_pending: bool,
    /// The perks.
    pub perks: BTreeSet<FormId>,
    /// The ranks of perks after the first rank.
    pub perk_ranks: BTreeMap<FormId, u8>,
    /// The teammates.
    pub teammates: BTreeSet<FormId>,
    /// The cell of the player.
    pub cell: Option<FormId>,
    /// The worldspace of a player who is outdoors.
    pub world: Option<FormId>,
    /// The position of the feet (the bits of three `f32`).
    pub position: Option<[u32; 3]>,
    /// The heading (the bits of an `f32`).
    pub heading: u32,
    /// The name.
    pub name: Option<String>,
    /// The sex.
    pub female: Option<bool>,
    /// The current stage of each quest that each player runs separately.
    pub quest_stage: BTreeMap<FormId, u16>,
    /// The done stages of those quests.
    pub stages_done: BTreeSet<(FormId, u16)>,
    /// The flag bits of those quests (1 running, 2 completed, 4 failed, 8 announced).
    pub quest_flags: BTreeMap<FormId, u8>,
    /// The objectives of those quests (2: completed and hidden).
    pub objectives: BTreeMap<(FormId, i32), u8>,
    /// The quest variables that each player has separately (the bits of an `f64`).
    pub quest_vars: BTreeMap<(FormId, String), u64>,
}

impl Default for PlayerRows {
    fn default() -> Self {
        PlayerRows::fresh()
    }
}

impl PlayerRows {
    /// The rows of a player who joined a moment ago: level 1, and nothing else.
    pub fn fresh() -> PlayerRows {
        PlayerRows {
            actor_values: BTreeMap::new(),
            value_damage: BTreeMap::new(),
            damage: None,
            dead: false,
            items: BTreeMap::new(),
            stocked: false,
            equipped: None,
            weapon_health: BTreeMap::new(),
            weapon_mods: BTreeMap::new(),
            dropped: BTreeSet::new(),
            equip_locked: BTreeSet::new(),
            ammo_loaded: None,
            hotkeys: [None; 8],
            misc_stats: BTreeMap::new(),
            skill_points: BTreeMap::new(),
            tag_skills: BTreeSet::new(),
            tag_slots: BTreeMap::new(),
            level: 1,
            level_up_pending: false,
            perks: BTreeSet::new(),
            perk_ranks: BTreeMap::new(),
            teammates: BTreeSet::new(),
            cell: None,
            world: None,
            position: None,
            heading: 0,
            name: None,
            female: None,
            quest_stage: BTreeMap::new(),
            stages_done: BTreeSet::new(),
            quest_flags: BTreeMap::new(),
            objectives: BTreeMap::new(),
            quest_vars: BTreeMap::new(),
        }
    }
}

struct Writer(Vec<u8>);

impl Writer {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn u16(&mut self, v: u16) {
        self.0.extend(v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.0.extend(v.to_le_bytes());
    }
    fn i32(&mut self, v: i32) {
        self.0.extend(v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.0.extend(v.to_le_bytes());
    }
    fn f64(&mut self, v: f64) {
        self.u64(v.to_bits());
    }
    fn form(&mut self, v: FormId) {
        self.u32(v.0);
    }
    fn opt_form(&mut self, v: Option<FormId>) {
        self.u8(v.is_some() as u8);
        self.u32(v.map_or(0, |f| f.0));
    }
    fn str(&mut self, s: &str) {
        self.u32(s.len() as u32);
        self.0.extend(s.as_bytes());
    }
    fn len(&mut self, n: usize) {
        self.u32(n as u32);
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.bytes.get(self.at..self.at.checked_add(n)?)?;
        self.at += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn i32(&mut self) -> Option<i32> {
        Some(self.u32()? as i32)
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    fn f64(&mut self) -> Option<f64> {
        Some(f64::from_bits(self.u64()?))
    }
    fn form(&mut self) -> Option<FormId> {
        Some(FormId(self.u32()?))
    }
    fn opt_form(&mut self) -> Option<Option<FormId>> {
        let on = self.u8()? != 0;
        let v = self.u32()?;
        Some(on.then_some(FormId(v)))
    }
    fn str(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
    /// A count of entries. Each entry has one byte or more, so a count that is more than
    /// the blob is refused before a loop starts.
    fn len(&mut self) -> Option<usize> {
        let n = self.u32()? as usize;
        (n <= self.bytes.len()).then_some(n)
    }
}

/// The blob of `rows`. The same rows always give the same bytes.
pub fn encode(rows: &PlayerRows) -> Vec<u8> {
    let s = rows;
    let mut w = Writer(vec![VERSION]);
    w.len(s.actor_values.len());
    for (k, v) in &s.actor_values {
        w.u16(*k);
        w.f64(*v);
    }
    w.len(s.value_damage.len());
    for (k, v) in &s.value_damage {
        w.u16(*k);
        w.f64(*v);
    }
    w.u8(s.damage.is_some() as u8);
    w.f64(s.damage.unwrap_or(0.0));
    w.u8(s.dead as u8);
    w.len(s.items.len());
    for (k, v) in &s.items {
        w.form(*k);
        w.i32(*v);
    }
    w.u8(s.stocked as u8);
    w.u8(s.equipped.is_some() as u8);
    let equipped = s.equipped.as_deref().unwrap_or(&[]);
    w.len(equipped.len());
    for f in equipped {
        w.form(*f);
    }
    w.len(s.weapon_health.len());
    for (k, v) in &s.weapon_health {
        w.form(*k);
        w.u32(*v);
    }
    w.len(s.weapon_mods.len());
    for (k, v) in &s.weapon_mods {
        w.form(*k);
        w.u8(*v);
    }
    for set in [&s.dropped, &s.equip_locked, &s.perks, &s.teammates] {
        w.len(set.len());
        for f in set.iter() {
            w.form(*f);
        }
    }
    w.opt_form(s.ammo_loaded);
    for h in &s.hotkeys {
        w.opt_form(*h);
    }
    w.len(s.misc_stats.len());
    for (k, v) in &s.misc_stats {
        w.u8(*k);
        w.u32(*v);
    }
    w.len(s.skill_points.len());
    for (k, v) in &s.skill_points {
        w.u16(*k);
        w.u32(*v);
    }
    w.len(s.tag_skills.len());
    for k in &s.tag_skills {
        w.u16(*k);
    }
    w.len(s.tag_slots.len());
    for (k, v) in &s.tag_slots {
        w.u8(*k);
        w.u16(*v);
    }
    w.u16(s.level);
    w.u8(s.level_up_pending as u8);
    w.len(s.perk_ranks.len());
    for (k, v) in &s.perk_ranks {
        w.form(*k);
        w.u8(*v);
    }
    w.opt_form(s.cell);
    w.opt_form(s.world);
    w.u8(s.position.is_some() as u8);
    for c in s.position.unwrap_or([0; 3]) {
        w.u32(c);
    }
    w.u32(s.heading);
    w.u8(s.name.is_some() as u8);
    w.str(s.name.as_deref().unwrap_or(""));
    w.u8(match s.female {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    });
    w.len(s.quest_stage.len());
    for (k, v) in &s.quest_stage {
        w.form(*k);
        w.u16(*v);
    }
    w.len(s.stages_done.len());
    for (quest, stage) in &s.stages_done {
        w.form(*quest);
        w.u16(*stage);
    }
    w.len(s.quest_flags.len());
    for (k, v) in &s.quest_flags {
        w.form(*k);
        w.u8(*v);
    }
    w.len(s.objectives.len());
    for ((quest, index), v) in &s.objectives {
        w.form(*quest);
        w.i32(*index);
        w.u8(*v);
    }
    w.len(s.quest_vars.len());
    for ((quest, name), v) in &s.quest_vars {
        w.form(*quest);
        w.str(name);
        w.u64(*v);
    }
    w.0
}

/// The rows of a blob that [`encode`] made. `None`: the blob is damaged, or it has
/// another version.
pub fn decode(blob: &[u8]) -> Option<PlayerRows> {
    let mut r = Reader { bytes: blob, at: 0 };
    if r.u8()? != VERSION {
        return None;
    }
    let mut s = PlayerRows::fresh();
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.actor_values.insert(k, r.f64()?);
    }
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.value_damage.insert(k, r.f64()?);
    }
    let has = r.u8()? != 0;
    let damage = r.f64()?;
    s.damage = has.then_some(damage);
    s.dead = r.u8()? != 0;
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.items.insert(k, r.i32()?);
    }
    s.stocked = r.u8()? != 0;
    let has = r.u8()? != 0;
    let mut equipped = Vec::new();
    for _ in 0..r.len()? {
        equipped.push(r.form()?);
    }
    s.equipped = has.then_some(equipped);
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.weapon_health.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.weapon_mods.insert(k, r.u8()?);
    }
    for which in 0..4 {
        for _ in 0..r.len()? {
            let f = r.form()?;
            match which {
                0 => s.dropped.insert(f),
                1 => s.equip_locked.insert(f),
                2 => s.perks.insert(f),
                _ => s.teammates.insert(f),
            };
        }
    }
    s.ammo_loaded = r.opt_form()?;
    for h in s.hotkeys.iter_mut() {
        *h = r.opt_form()?;
    }
    for _ in 0..r.len()? {
        let k = r.u8()?;
        s.misc_stats.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        let k = r.u16()?;
        s.skill_points.insert(k, r.u32()?);
    }
    for _ in 0..r.len()? {
        s.tag_skills.insert(r.u16()?);
    }
    for _ in 0..r.len()? {
        let k = r.u8()?;
        s.tag_slots.insert(k, r.u16()?);
    }
    s.level = r.u16()?;
    s.level_up_pending = r.u8()? != 0;
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.perk_ranks.insert(k, r.u8()?);
    }
    s.cell = r.opt_form()?;
    s.world = r.opt_form()?;
    let has = r.u8()? != 0;
    let position = [r.u32()?, r.u32()?, r.u32()?];
    s.position = has.then_some(position);
    s.heading = r.u32()?;
    let has = r.u8()? != 0;
    let name = r.str()?;
    s.name = has.then_some(name);
    s.female = match r.u8()? {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    };
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.quest_stage.insert(k, r.u16()?);
    }
    for _ in 0..r.len()? {
        let quest = r.form()?;
        s.stages_done.insert((quest, r.u16()?));
    }
    for _ in 0..r.len()? {
        let k = r.form()?;
        s.quest_flags.insert(k, r.u8()?);
    }
    for _ in 0..r.len()? {
        let quest = r.form()?;
        let index = r.i32()?;
        s.objectives.insert((quest, index), r.u8()?);
    }
    for _ in 0..r.len()? {
        let quest = r.form()?;
        let name = r.str()?;
        s.quest_vars.insert((quest, name), r.u64()?);
    }
    (r.at == blob.len()).then_some(s)
}

/// The fields that carry the blob of player `slot`: one `Field::Custom` for each chunk,
/// with its key. A blob that needs more chunks than the tags allow is cut.
pub fn chunks_of(slot: u8, blob: &[u8]) -> Vec<(Key, Field)> {
    blob.chunks(CHUNK)
        .take(usize::from(TAG_END - TAG_BASE))
        .enumerate()
        .map(|(i, chunk)| {
            let field = Field::Custom {
                tag: TAG_BASE + i as u16,
                data: chunk.to_vec(),
            };
            (field.key(rows_form(slot)), field)
        })
        .collect()
}

/// The blob of player `slot`, joined from its chunks. `field` gives the field of a key
/// (from an export, or from a store). The blob ends at the first tag that has no chunk.
/// An empty blob: the fields hold no rows for that player.
pub fn blob_of<'a>(slot: u8, field: impl Fn(&Key) -> Option<&'a Field>) -> Vec<u8> {
    let mut blob = Vec::new();
    for tag in TAG_BASE..TAG_END {
        let probe = Field::Custom {
            tag,
            data: Vec::new(),
        };
        match field(&probe.key(rows_form(slot))) {
            Some(Field::Custom { data, .. }) if !data.is_empty() => blob.extend_from_slice(data),
            _ => break,
        }
    }
    blob
}

/// The rows of player `slot` from fields: [`blob_of`], then [`decode`]. `None`: the
/// fields hold no rows for that player, or the blob is damaged.
pub fn rows_of<'a>(slot: u8, field: impl Fn(&Key) -> Option<&'a Field>) -> Option<PlayerRows> {
    decode(&blob_of(slot, field))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rows with each field set, and with a blob of more than one chunk.
    fn full() -> PlayerRows {
        let mut s = PlayerRows::fresh();
        s.actor_values = [(5u16, 7.0), (32, 45.5)].into_iter().collect();
        s.value_damage = [(16u16, 12.25)].into_iter().collect();
        s.damage = Some(3.5);
        s.dead = true;
        s.items = (0..90u32)
            .map(|i| (FormId(0x1000 + i), i as i32 - 3))
            .collect();
        s.stocked = true;
        s.equipped = Some(vec![FormId(0x2002), FormId(0x2001)]);
        s.weapon_health = [(FormId(0x2001), 0.75f32.to_bits())].into_iter().collect();
        s.weapon_mods = [(FormId(0x2001), 5u8)].into_iter().collect();
        s.dropped = [FormId(0x3001)].into_iter().collect();
        s.equip_locked = [FormId(0x3002)].into_iter().collect();
        s.ammo_loaded = Some(FormId(0x4001));
        s.hotkeys[2] = Some(FormId(0x2001));
        s.misc_stats = [(3u8, 9u32)].into_iter().collect();
        s.skill_points = [(32u16, 14u32)].into_iter().collect();
        s.tag_skills = [32u16, 40, 41].into_iter().collect();
        s.tag_slots = [(0u8, 32u16)].into_iter().collect();
        s.level = 7;
        s.level_up_pending = true;
        s.perks = [FormId(0x5001), FormId(0x5002)].into_iter().collect();
        s.perk_ranks = [(FormId(0x5002), 2u8)].into_iter().collect();
        s.teammates = [FormId(0x6001)].into_iter().collect();
        s.cell = Some(FormId(0x0010_4C0F));
        s.world = Some(FormId(0x000D_A726));
        s.position = Some([1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]);
        s.heading = 0.5f32.to_bits();
        s.name = Some("Bo".into());
        s.female = Some(true);
        s.quest_stage = [(FormId(0x0010_4C1C), 105u16)].into_iter().collect();
        s.stages_done = [(FormId(0x0010_4C1C), 10u16)].into_iter().collect();
        s.quest_flags = [(FormId(0x0010_4C1C), 3u8)].into_iter().collect();
        s.objectives = [((FormId(0x0010_4C1C), 10), 1u8)].into_iter().collect();
        s.quest_vars = [(
            (FormId(0x0010_4C1C), "bruntimer".to_string()),
            1.0f64.to_bits(),
        )]
        .into_iter()
        .collect();
        s
    }

    #[test]
    fn rows_come_back_equal_from_their_blob() {
        for rows in [PlayerRows::fresh(), full()] {
            let blob = encode(&rows);
            assert_eq!(blob[0], VERSION);
            assert_eq!(decode(&blob), Some(rows.clone()));
            // the same rows give the same bytes
            assert_eq!(encode(&rows), blob);
        }
        // the twin: other rows give another blob, and each sex has its own byte
        let mut other = full();
        other.level = 8;
        assert_ne!(encode(&other), encode(&full()));
        for female in [None, Some(false), Some(true)] {
            let rows = PlayerRows {
                female,
                ..PlayerRows::fresh()
            };
            assert_eq!(decode(&encode(&rows)).map(|r| r.female), Some(female));
        }
    }

    #[test]
    fn a_damaged_blob_gives_no_rows() {
        let blob = encode(&full());
        assert!(decode(&blob).is_some());
        // a cut blob, a blob with one byte more, another version, and no blob
        assert_eq!(decode(&blob[..blob.len() - 1]), None);
        let mut longer = blob.clone();
        longer.push(0);
        assert_eq!(decode(&longer), None);
        let mut newer = blob.clone();
        newer[0] = VERSION + 1;
        assert_eq!(decode(&newer), None);
        assert_eq!(decode(&[]), None);
        // a count that is more than the blob does not start a long loop
        let mut absurd = vec![VERSION];
        absurd.extend(u32::MAX.to_le_bytes());
        assert_eq!(decode(&absurd), None);
    }

    #[test]
    fn a_blob_goes_into_chunks_and_comes_back() {
        let blob = encode(&full());
        assert!(blob.len() > CHUNK, "the sample must need two chunks");
        let fields: BTreeMap<Key, Field> = chunks_of(3, &blob).into_iter().collect();
        assert_eq!(fields.len(), blob.len().div_ceil(CHUNK));
        for (key, field) in &fields {
            assert_eq!(key.form, rows_form(3));
            assert_eq!(*key, field.key(rows_form(3)));
            assert!(field.is_sane() && !field.is_tombstone());
        }
        assert_eq!(blob_of(3, |k| fields.get(k)), blob);
        assert_eq!(rows_of(3, |k| fields.get(k)), Some(full()));
        // the twin: another player has no chunk in these fields
        assert!(blob_of(4, |k| fields.get(k)).is_empty());
        assert_eq!(rows_of(4, |k| fields.get(k)), None);
        // a blob with a hole ends at the hole: the rows do not decode
        let mut holed = fields.clone();
        let first = Field::Custom {
            tag: TAG_BASE,
            data: Vec::new(),
        };
        holed.remove(&first.key(rows_form(3)));
        assert_eq!(rows_of(3, |k| holed.get(k)), None);
    }

    #[test]
    fn a_rows_form_names_its_player() {
        for slot in 0..ROWS_FORMS as u8 {
            assert_eq!(slot_of_rows_form(rows_form(slot)), Some(slot));
        }
        // the twin: a form of the game, and the form after the last rows form
        assert_eq!(slot_of_rows_form(FormId(0x0010_4C1C)), None);
        assert_eq!(slot_of_rows_form(FormId(ROWS_FORM_BASE + ROWS_FORMS)), None);
        assert_eq!(slot_of_rows_form(FormId(ROWS_FORM_BASE - 1)), None);
    }
}
