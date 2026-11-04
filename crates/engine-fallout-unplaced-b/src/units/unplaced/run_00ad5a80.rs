//! `(unplaced)/run_00ad5a80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `std::string` assignment from part of another string (`0044a580`,
/// thiscall: source, position, count).
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `std::string::npos` (`010173f0`, all ones in the exe).
const STRING_NPOS: u32 = 0x0101_73f0;
/// The size of a `std::string`.
const STRING_SIZE: u32 = 0x1c;

// Translated from 00ad5a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the strings `[first, last)` over the ones from `dest` on (cdecl,
/// six words; a `std::copy` shape, called by `00aceb80`, `00ad61e0` and
/// `00ad6690`; the last three words are the iterator tags, never read):
/// assigns each with `0044a580(source, 0, npos)` and answers the end of the
/// copied range, `dest + (last - first) / 0x1C * 0x1C` (signed division).
pub fn fn_00ad5a80(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let count = (last.wrapping_sub(first) as i32) / STRING_SIZE as i32;
    let end = (count as u32).wrapping_mul(STRING_SIZE).wrapping_add(dest);
    let mut to = dest;
    let mut from = first;
    while from != last {
        let npos = e.mem.u32(STRING_NPOS);
        e.call(STRING_ASSIGN, &[to, from, 0, npos]);
        to = to.wrapping_add(STRING_SIZE);
        from = from.wrapping_add(STRING_SIZE);
    }
    end
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ad_5a80, fn_00ad5a80(u32, u32, u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_are_assigned_in_order() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(STRING_NPOS, u32::MAX);
        e.register(STRING_ASSIGN, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let end = e
            .call(0x00ad_5a80, &[0x1000, 0x1038, 0x2000, 0, 0, 0])
            .u32();
        assert_eq!(end, 0x2038);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (STRING_ASSIGN, vec![0x2000, 0x1000, 0, u32::MAX]),
                (STRING_ASSIGN, vec![0x201c, 0x101c, 0, u32::MAX]),
            ]
        );
        assert_eq!(
            e.call(0x00ad_5a80, &[0x1000, 0x1000, 0x2000, 0, 0, 0])
                .u32(),
            0x2000
        );
    }
}
