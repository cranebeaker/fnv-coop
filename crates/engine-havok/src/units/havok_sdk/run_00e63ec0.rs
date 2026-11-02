//! `Havok SDK/run_00e63ec0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkString::strCasecmp` (`00cb0fe0`, engine map name; cdecl `(left, right)`): 0 when equal.
const STRING_CASE_COMPARE: u32 = 0x00cb_0fe0;

// Translated from 00e63ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: looks `name` up in the table of `[+0xc]` pairs of words at `*(this + 8)`
/// (a value, then a string pointer whose lowest bit is a flag): the first entry whose string
/// (flag cleared) equals `name` ignoring case (`00cb0fe0`) gives its value to `*out` and the
/// result 0. No match gives 1.
pub fn fn_00e63ec0(e: &mut Engine, this: Ptr, name: u32, out: u32) -> u32 {
    let this = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.wrapping_add(0xc)) {
        let table = e.mem.u32(this.wrapping_add(8));
        let string = e.mem.u32(table.wrapping_add(8 * index).wrapping_add(4)) & 0xffff_fffe;
        if e.call(STRING_CASE_COMPARE, &args![name, string]).u32() == 0 {
            let table = e.mem.u32(this.wrapping_add(8));
            let value = e.mem.u32(table.wrapping_add(8 * index));
            e.mem.set_u32(out, value);
            return 0;
        }
        index += 1;
    }
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e6_3ec0, fn_00e63ec0(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRING_CASE_COMPARE, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            Ret {
                eax: u32::from(left != right),
                ..Ret::default()
            }
        });
        e
    }

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 8);
        e.mem.set_cstr(at, text);
        at
    }

    /// A table of the given `(value, text)` pairs; the flag bit of every string pointer is set.
    fn table(e: &mut Engine, entries: &[(u32, &[u8])]) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(8 * entries.len() as u32 + 8);
        for (index, (value, text)) in entries.iter().enumerate() {
            let at = string(e, text);
            e.mem.set_u32(data + 8 * index as u32, *value);
            e.mem.set_u32(data + 8 * index as u32 + 4, at | 1);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, entries.len() as u32);
        this
    }

    #[test]
    fn first_case_insensitive_match_gives_its_value() {
        let mut e = engine();
        let this = table(&mut e, &[(10, b"Alpha"), (20, b"Beta"), (30, b"BETA")]);
        let name = string(&mut e, b"bEtA");
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00e6_3ec0, &args![this, name, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 20);
    }

    #[test]
    fn no_match_returns_one_and_leaves_the_output() {
        let mut e = engine();
        let this = table(&mut e, &[(10, b"Alpha")]);
        let name = string(&mut e, b"Gamma");
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0x55);
        assert_eq!(e.call(0x00e6_3ec0, &args![this, name, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0x55);
        let empty = table(&mut e, &[]);
        assert_eq!(e.call(0x00e6_3ec0, &args![empty, name, out]).u32(), 1);
    }
}
