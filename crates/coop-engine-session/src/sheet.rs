//! A character in a guest's part of a party save (docs/coop/SPECS.md S32): the bytes of the
//! part's section 4. The character itself is read and set by `coop_engine::stats`.
//!
//! The section is a list of parts, each a tag (`u8`), a length (`u32`) and its bytes, so that a
//! later build can add one; a part with a tag this build does not know is skipped. All little
//! endian.
//!
//! - **1, the character**: the count of attributes (`u8`) and each as an `f32`, the same for the
//!   skills and for the tag skills (`i32` each), the level (`u16`), experience (`i32`), the count
//!   of perks (`u16`) and each as its form id and rank (`u8`).
//! - **2, the condition**: health, action points, radiation (`f32` each), the count of body
//!   conditions (`u8`) and each as an `f32`, karma (`i32`).
//! - **3, what is worn**: the count (`u16`) and each item's form id.
//! - **4, the look** (anim, docs/coop/SPECS.md S41): the face, hair, race, sex, body and name, as
//!   the avatar's appearance carries them ([`crate::look::encode`]); one this build cannot read
//!   is read as missing.
//!
//! A part whose counts are not this build's is read as missing (its values are left alone); a
//! part that is cut makes the whole section unreadable.

use coop_engine::stats::{Character, Condition, Sheet, CONDITIONS, SKILLS, SPECIAL, TAGS};

const PART_CHARACTER: u8 = 1;
const PART_CONDITION: u8 = 2;
const PART_WORN: u8 = 3;
const PART_LOOK: u8 = 4; // anim

fn part(out: &mut Vec<u8>, tag: u8, body: &[u8]) {
    out.push(tag);
    out.extend((body.len() as u32).to_le_bytes());
    out.extend(body);
}

fn floats(out: &mut Vec<u8>, v: &[f32]) {
    out.push(v.len() as u8);
    for x in v {
        out.extend(x.to_bits().to_le_bytes());
    }
}

fn list_len(n: usize) -> u16 {
    n.min(usize::from(u16::MAX)) as u16
}

/// The section's bytes.
pub fn encode(s: &Sheet) -> Vec<u8> {
    let mut out = Vec::new();
    if let Some(c) = &s.character {
        let mut b = Vec::new();
        floats(&mut b, &c.special);
        floats(&mut b, &c.skills);
        b.push(TAGS as u8);
        for t in &c.tags {
            b.extend(t.to_le_bytes());
        }
        b.extend(c.level.to_le_bytes());
        b.extend(c.xp.to_le_bytes());
        b.extend(list_len(c.perks.len()).to_le_bytes());
        for (id, rank) in c.perks.iter().take(usize::from(u16::MAX)) {
            b.extend(id.to_le_bytes());
            b.push(*rank);
        }
        part(&mut out, PART_CHARACTER, &b);
    }
    if let Some(c) = &s.condition {
        let mut b = Vec::new();
        for v in [c.health, c.action_points, c.radiation] {
            b.extend(v.to_bits().to_le_bytes());
        }
        floats(&mut b, &c.body);
        b.extend(c.karma.to_le_bytes());
        part(&mut out, PART_CONDITION, &b);
    }
    if let Some(w) = &s.worn {
        let mut b = list_len(w.len()).to_le_bytes().to_vec();
        for id in w.iter().take(usize::from(u16::MAX)) {
            b.extend(id.to_le_bytes());
        }
        part(&mut out, PART_WORN, &b);
    }
    // anim
    if let Some(b) = s.look.as_ref().and_then(crate::look::encode) {
        part(&mut out, PART_LOOK, &b);
    }
    out
}

struct Cursor<'a>(&'a [u8]);

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.0.len() < n {
            return None;
        }
        let (head, rest) = self.0.split_at(n);
        self.0 = rest;
        Some(head)
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
    fn f32(&mut self) -> Option<f32> {
        self.u32().map(f32::from_bits)
    }
    /// `N` floats after their count; `Some(None)` when the count is another build's.
    fn floats<const N: usize>(&mut self) -> Option<Option<[f32; N]>> {
        let n = usize::from(self.u8()?);
        let mut v = [0.0; N];
        if n != N {
            self.take(4 * n)?;
            return Some(None);
        }
        for x in &mut v {
            *x = self.f32()?;
        }
        Some(Some(v))
    }
}

/// A part's values; `Some(None)` when its counts are another build's (the rest of the part is
/// then not read).
fn character(c: &mut Cursor) -> Option<Option<Character>> {
    let Some(special) = c.floats::<SPECIAL>()? else {
        return Some(None);
    };
    let Some(skills) = c.floats::<SKILLS>()? else {
        return Some(None);
    };
    if usize::from(c.u8()?) != TAGS {
        return Some(None);
    }
    let mut tags = [-1; TAGS];
    for t in &mut tags {
        *t = c.u32()? as i32;
    }
    let level = c.u16()?;
    let xp = c.u32()? as i32;
    let mut perks = Vec::new();
    for _ in 0..c.u16()? {
        perks.push((c.u32()?, c.u8()?));
    }
    Some(Some(Character {
        special,
        skills,
        tags,
        level,
        xp,
        perks,
    }))
}

/// See [`character`].
fn condition(c: &mut Cursor) -> Option<Option<Condition>> {
    let (health, action_points, radiation) = (c.f32()?, c.f32()?, c.f32()?);
    let Some(body) = c.floats::<CONDITIONS>()? else {
        return Some(None);
    };
    let karma = c.u32()? as i32;
    Some(Some(Condition {
        health,
        action_points,
        radiation,
        body,
        karma,
    }))
}

/// Reads a section's bytes; `None` for one that is cut.
pub fn decode(b: &[u8]) -> Option<Sheet> {
    let mut c = Cursor(b);
    let mut s = Sheet::default();
    while !c.0.is_empty() {
        let tag = c.u8()?;
        let len = c.u32()? as usize;
        let mut body = Cursor(c.take(len)?);
        match tag {
            PART_CHARACTER => s.character = character(&mut body)?,
            PART_CONDITION => s.condition = condition(&mut body)?,
            PART_WORN => {
                let mut w = Vec::new();
                for _ in 0..body.u16()? {
                    w.push(body.u32()?);
                }
                s.worn = Some(w);
            }
            PART_LOOK => s.look = crate::look::decode(body.0), // anim
            _ => {}
        }
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet() -> Sheet {
        Sheet {
            character: Some(Character {
                special: [5.0, 6.0, 7.0, 4.0, 8.0, 5.0, 5.0],
                skills: [
                    15.0, 20.5, 30.0, 45.0, 15.0, 15.0, 60.0, 15.0, 15.0, 15.0, 15.0, 75.0, 15.0,
                    100.0,
                ],
                tags: [0x2b, 0x29, 0x20, -1],
                level: 12,
                xp: 14_250,
                perks: vec![(0x0003_1dd8, 2), (0x0009_4eb9, 1)],
            }),
            condition: Some(Condition {
                health: 182.5,
                action_points: 71.0,
                radiation: 230.0,
                body: [100.0, 100.0, 40.0, 100.0, 0.0, 100.0, 100.0],
                karma: -230,
            }),
            worn: Some(vec![0x0002_0420, 0x0000_4333]),
            // anim
            look: Some(coop_engine::look::Face {
                female: true,
                race: 0x0000_424a,
                hair: 0x0003_2b4e,
                hair_length: 0.25,
                eyes: 0x0004_bc2a,
                hair_color: 0x0040_2010,
                height: 1.0,
                weight: 0.5,
                head_parts: vec![0x0001_0101],
                coord: [
                    (2, 1, vec![0.5, -1.25]),
                    (1, 1, vec![0.125]),
                    (0, 0, Vec::new()),
                    (0, 0, Vec::new()),
                ],
                name: "Courier Two".into(),
            }),
        }
    }

    #[test]
    fn a_sheet_round_trips_and_an_empty_one_too() {
        let s = sheet();
        assert_eq!(decode(&encode(&s)), Some(s));
        assert_eq!(encode(&Sheet::default()), Vec::<u8>::new());
        assert_eq!(decode(&[]), Some(Sheet::default()));
        let only_worn = Sheet {
            worn: Some(vec![]),
            ..Sheet::default()
        };
        assert_eq!(decode(&encode(&only_worn)), Some(only_worn));
    }

    #[test]
    fn a_cut_section_is_refused_and_an_unknown_part_skipped() {
        let bytes = encode(&sheet());
        for n in 1..bytes.len() {
            assert_ne!(decode(&bytes[..n]), Some(sheet()), "cut at {n}");
        }
        assert_eq!(decode(&bytes[..3]), None);
        let mut later = bytes.clone();
        part(&mut later, 0x7f, &[1, 2, 3]);
        assert_eq!(decode(&later), Some(sheet()));
    }

    #[test]
    fn a_part_with_another_builds_counts_is_read_as_missing() {
        let s = sheet();
        let mut bytes = encode(&s);
        // The character's count of attributes, after its tag and length.
        assert_eq!(bytes[5], SPECIAL as u8);
        bytes[5] = SPECIAL as u8 - 1;
        // The character is not read; the rest of the section is.
        let read = decode(&bytes).unwrap();
        assert_eq!(read.character, None);
        assert_eq!(read.condition, s.condition);
        assert_eq!(read.worn, s.worn);
    }

    // anim
    #[test]
    fn a_look_this_build_cannot_read_is_read_as_missing() {
        let s = sheet();
        let bytes = encode(&s);
        let mut other = bytes.clone();
        // The look's appearance version, after its tag, its length and its magic byte.
        let at = bytes.len() - crate::look::encode(s.look.as_ref().unwrap()).unwrap().len();
        assert_eq!(other[at], crate::look::APPEARANCE_MAGIC);
        other[at + 1] = crate::look::APPEARANCE_VERSION + 1;
        let read = decode(&other).unwrap();
        assert_eq!(read.look, None);
        assert_eq!(read.worn, s.worn);
    }
}
