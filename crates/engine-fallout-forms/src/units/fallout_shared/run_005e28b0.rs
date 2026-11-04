//! `fallout shared/run_005e28b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The string constructor run on `this + 0x18` (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;

// Translated from 005e28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an object of the `ScriptVariable` layout (Xbox PDB: a
/// `SCRIPT_LOCAL` at `+0`, a `BSStringT` name at `+0x18`): zeroes the data
/// ([`fn_005e28e0`]) and constructs the string at `+0x18` (`004037b0`).
/// Returns `this`. The engine map has no name for it (the decompiler's
/// `SafeSQueue` is a folded library name); the stage item loader
/// `0060f6a0` calls it.
pub fn fn_005e28b0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005e28e0(e, this);
    e.call(STRING_CONSTRUCT, &args![this.addr() + 0x18]);
    this
}

// Translated from 005e28e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes a `SCRIPT_LOCAL` (Xbox PDB layout): `uiID` (`+0`), the double
/// `fValue` (`+8`, stored from `FLDZ`, all bits zero) and `bIsInteger`
/// (`+0x10`). Returns `this`.
pub fn fn_005e28e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    this
}

// Translated from 005e2910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2910 - the exception-unwinding frame (FS:[0]) around the name's construction is not translated.
/// The `ScriptVariable` copy constructor (`RET 4`: the source; the engine
/// map has no name): zeroes the data ([`fn_005e28e0`]), constructs the name
/// at +0x18 (`004037b0`), copies `uiID` (+0), the `double` `fValue` (+8,
/// FLD/FSTP) and `bIsInteger` (+0x10), then the name from the source's
/// (`00501c10`). Returns `this`.
pub fn fn_005e2910(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    fn_005e28e0(e, this);
    e.call(STRING_CONSTRUCT, &args![base + 0x18]);
    let id = e.mem.u32(source);
    e.mem.set_u32(base, id);
    let value = x87_load64(e.mem.u64(source + 8));
    e.mem.set_f64(base + 8, value);
    let flag = e.mem.u8(source + 0x10);
    e.mem.set_u8(base + 0x10, flag);
    e.call(0x0050_1c10, &args![base + 0x18, source + 0x18]);
    this
}

// Translated from 005e29a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `ScriptVariable` differs from another (`RET 4`: the other;
/// the engine map has no name): true without one, when `bIsInteger`
/// (+0x10), `uiID` (+0) or the `double` `fValue` (+8, unordered counts as
/// different) differ, or when the names differ (`00501c30` on +0x18).
pub fn fn_005e29a0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    if other == 0 {
        return true;
    }
    if e.mem.u8(base + 0x10) != e.mem.u8(other + 0x10) {
        return true;
    }
    if e.mem.u32(base) != e.mem.u32(other) {
        return true;
    }
    let theirs = x87_load64(e.mem.u64(other + 8));
    let mine = f64::from_bits(e.mem.u64(base + 8));
    // FCOMP; TEST AH,0x44; JNP: on only when equal and ordered.
    if theirs != mine {
        return true;
    }
    e.call(0x0050_1c30, &args![base + 0x18, other + 0x18])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_2910, fn_005e2910(Ptr, u32) -> Ptr),
        entry!(0x005e_29a0, fn_005e29a0(Ptr, u32) -> bool),
        entry!(0x005e28b0, fn_005e28b0(Ptr) -> Ptr),
        entry!(0x005e28e0, fn_005e28e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroes_the_data_fields_only() {
        let mut e = Engine::new();
        let local = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(local + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x005e_28e0, &[local]).u32(), local);
        assert_eq!(e.mem.u32(local), 0);
        // The padding word at +4 and the three bytes after the bool stay.
        assert_eq!(e.mem.u32(local + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(local + 8), 0);
        assert_eq!(e.mem.u32(local + 0xc), 0);
        assert_eq!(e.mem.u32(local + 0x10), 0xffff_ff00);
        assert_eq!(e.mem.u32(local + 0x14), 0xffff_ffff);
    }

    #[test]
    fn constructor_zeroes_the_data_and_builds_the_name() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let variable = e.mem.alloc(0x20);
        e.mem.set_u32(variable, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_28b0, &[variable]).u32(), variable);
        assert_eq!(e.mem.u32(variable), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(STRING_CONSTRUCT, vec![variable + 0x18])]);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_005e2910_copies_a_variable() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(0x0050_1c10, |_, _| Ret::default());
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source, 7);
        e.mem.set_u64(source + 8, 0x7ff0_0000_0000_0001);
        e.mem.set_u8(source + 0x10, 1);
        let copy = e.mem.alloc(0x20);
        e.mem.set_u32(copy + 4, 0xdead);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_2910, &args![copy, source]).u32(), copy);
        assert_eq!(e.mem.u32(copy), 7);
        assert_eq!(e.mem.u64(copy + 8), 0x7ff8_0000_0000_0001);
        assert_eq!(e.mem.u8(copy + 0x10), 1);
        assert_eq!(e.mem.u32(copy + 4), 0xdead);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (STRING_CONSTRUCT, vec![copy + 0x18]),
                (0x0050_1c10, vec![copy + 0x18, source + 0x18])
            ]
        );
    }

    #[test]
    fn fn_005e29a0_compares_variables() {
        let mut e = Engine::new();
        e.register(0x0050_1c30, |_, _| ret(0));
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        assert!(!e.call(0x005e_29a0, &args![a, b]).bool());
        assert!(e.call(0x005e_29a0, &args![a, 0u32]).bool());
        e.mem.set_u8(b + 0x10, 1);
        assert!(e.call(0x005e_29a0, &args![a, b]).bool());
        e.mem.set_u8(b + 0x10, 0);
        e.mem.set_u32(b, 1);
        assert!(e.call(0x005e_29a0, &args![a, b]).bool());
        e.mem.set_u32(b, 0);
        e.mem.set_f64(b + 8, -0.0);
        assert!(!e.call(0x005e_29a0, &args![a, b]).bool());
        e.mem.set_f64(a + 8, f64::NAN);
        e.mem.set_f64(b + 8, f64::NAN);
        assert!(e.call(0x005e_29a0, &args![a, b]).bool());
        e.mem.set_f64(a + 8, 0.0);
        e.mem.set_f64(b + 8, 0.0);
        e.register(0x0050_1c30, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_29a0, &args![a, b]).bool());
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x0050_1c30, vec![a + 0x18, b + 0x18])
        );
    }
}
