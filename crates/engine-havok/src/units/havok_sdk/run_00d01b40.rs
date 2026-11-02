//! `Havok SDK/run_00d01b40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d01b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a flag byte at `out` about the shape object the word at `this` points to (the class
/// is unconfirmed; the number at `+0xc` of the object looks like a shape type): 0 when there is
/// no object; otherwise 1, cleared when the type is 9, 0x17 or 7 and virtual method `+4` of the
/// sub-object at `object + 0x10` returns more than 100 (signed), and cleared when the type is
/// 0xf. The pointer in `this` and the type are read again before every test, as the exe does.
pub fn fn_00d01b40(e: &mut Engine, this: Ptr, out: Ptr) {
    let object = e.mem.u32(this.addr());
    if object == 0 {
        e.mem.set_u8(out.addr(), 0);
        return;
    }
    let mut flag = 1u8;
    for kind in [9u32, 0x17, 7] {
        let object = e.mem.u32(this.addr());
        if e.mem.u32(object + 0xc) == kind {
            let count = e.vcall(object + 0x10, 4, &args![]).i32();
            if count > 100 {
                flag = 0;
            }
        }
    }
    let object = e.mem.u32(this.addr());
    if e.mem.u32(object + 0xc) == 0xf {
        flag = 0;
    }
    e.mem.set_u8(out.addr(), flag);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d0_1b40, fn_00d01b40(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const COUNT_DOUBLE: u32 = 0x0900_0401;

    /// A holder pointing at an object of the given type whose sub-object (at `+0x10`) answers
    /// `count` to its slot 4; returns the engine, the holder and the output byte.
    fn rig(kind: u32, count: i32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register_double(COUNT_DOUBLE, move |_, _| Ret {
            eax: count as u32,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, COUNT_DOUBLE);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0xc, kind);
        e.mem.set_u32(object + 0x10, vtable);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, object);
        let out = e.mem.alloc(0x10);
        e.mem.set_u8(out, 0xee);
        (e, holder, out)
    }

    #[test]
    fn no_object_gives_zero() {
        let mut e = Engine::new();
        let holder = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        e.mem.set_u8(out, 0xee);
        e.call(0x00d0_1b40, &args![holder, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// Types 9, 0x17 and 7 are cleared above a count of 100 (signed); type 0xf always; others
    /// keep the flag.
    #[test]
    fn the_flag_depends_on_the_type_and_the_count() {
        for (kind, count, expected) in [
            (9u32, 100i32, 1u8),
            (9, 101, 0),
            (0x17, 101, 0),
            (0x17, 50, 1),
            (7, 1000, 0),
            (7, -5, 1),
            (0xf, 0, 0),
            (3, 5000, 1),
        ] {
            let (mut e, holder, out) = rig(kind, count);
            e.call(0x00d0_1b40, &args![holder, out]);
            assert_eq!(e.mem.u8(out), expected, "type {kind:#x} count {count}");
        }
    }
}
