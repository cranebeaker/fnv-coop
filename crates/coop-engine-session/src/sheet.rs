//! A character in a guest's part of a party save (docs/coop/SPECS.md S30): the bytes of the
//! part's section 4. The character itself is read and set by `coop_engine::stats`.
//!
//! The section: the count of attributes and each as an `f32`, the count of skills and each,
//! the level (`u16`), experience and karma (`i32` each), the count of perks (`u16`) and each as
//! its form id and rank, the count of worn items (`u16`) and each as its form id. All little
//! endian. A section with other counts of attributes or skills than this build's is refused.

use coop_engine::stats::{Sheet, SKILLS, SPECIAL};

/// The section's bytes.
pub fn encode(s: &Sheet) -> Vec<u8> {
    let mut out = vec![SPECIAL as u8];
    for v in &s.special {
        out.extend(v.to_bits().to_le_bytes());
    }
    out.push(SKILLS as u8);
    for v in &s.skills {
        out.extend(v.to_bits().to_le_bytes());
    }
    out.extend(s.level.to_le_bytes());
    out.extend(s.xp.to_le_bytes());
    out.extend(s.karma.to_le_bytes());
    out.extend((s.perks.len().min(usize::from(u16::MAX)) as u16).to_le_bytes());
    for (id, rank) in s.perks.iter().take(usize::from(u16::MAX)) {
        out.extend(id.to_le_bytes());
        out.push(*rank);
    }
    out.extend((s.worn.len().min(usize::from(u16::MAX)) as u16).to_le_bytes());
    for id in s.worn.iter().take(usize::from(u16::MAX)) {
        out.extend(id.to_le_bytes());
    }
    out
}

fn take<'a>(b: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
    if b.len() < n {
        return None;
    }
    let (head, rest) = b.split_at(n);
    *b = rest;
    Some(head)
}

fn u16_of(b: &mut &[u8]) -> Option<u16> {
    Some(u16::from_le_bytes(take(b, 2)?.try_into().ok()?))
}

fn u32_of(b: &mut &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(take(b, 4)?.try_into().ok()?))
}

/// Reads a section's bytes; `None` for one that is cut, or has other counts than this build's.
pub fn decode(mut b: &[u8]) -> Option<Sheet> {
    let b = &mut b;
    let mut s = Sheet::default();
    if usize::from(take(b, 1)?[0]) != SPECIAL {
        return None;
    }
    for v in &mut s.special {
        *v = f32::from_bits(u32_of(b)?);
    }
    if usize::from(take(b, 1)?[0]) != SKILLS {
        return None;
    }
    for v in &mut s.skills {
        *v = f32::from_bits(u32_of(b)?);
    }
    s.level = u16_of(b)?;
    s.xp = u32_of(b)? as i32;
    s.karma = u32_of(b)? as i32;
    for _ in 0..u16_of(b)? {
        s.perks.push((u32_of(b)?, take(b, 1)?[0]));
    }
    for _ in 0..u16_of(b)? {
        s.worn.push(u32_of(b)?);
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet() -> Sheet {
        Sheet {
            special: [5.0, 6.0, 7.0, 4.0, 8.0, 5.0, 5.0],
            skills: [15.0, 20.5, 30.0, 45.0, 15.0, 15.0, 60.0, 15.0, 15.0, 15.0, 15.0, 75.0, 15.0, 100.0],
            level: 12,
            xp: 14_250,
            karma: -230,
            perks: vec![(0x0003_1dd8, 2), (0x0009_4eb9, 1)],
            worn: vec![0x0002_0420, 0x0000_4333],
        }
    }

    #[test]
    fn a_sheet_round_trips_and_an_empty_one_too() {
        let s = sheet();
        assert_eq!(decode(&encode(&s)), Some(s));
        assert_eq!(decode(&encode(&Sheet::default())), Some(Sheet::default()));
    }

    #[test]
    fn a_cut_section_and_other_counts_are_refused() {
        let bytes = encode(&sheet());
        for n in 0..bytes.len() {
            assert_eq!(decode(&bytes[..n]), None, "cut at {n}");
        }
        let mut other = bytes.clone();
        other[0] = 8;
        assert_eq!(decode(&other), None);
        let mut other = bytes;
        other[1 + 4 * SPECIAL] = 13;
        assert_eq!(decode(&other), None);
    }
}
