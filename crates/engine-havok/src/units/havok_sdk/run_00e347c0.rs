//! `Havok SDK/run_00e347c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e347c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor (`this` in `ECX`, no stack words). It writes the
/// 16-bit word 1 at `+6`, the vtable word `010eb770`, 0 at `+8`, runs `hkStringPtr::hkStringPtr`
/// (`00cb0a60`, `this` = `this + 0xc`), clears the word at `+0x10` and empties two
/// `hkArray`-shaped triples (data 0, size 0, capacity and flags `0x80000000`) at `+0x14` and
/// `+0x20`. Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e347c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, 0x010e_b770);
    e.mem.set_u32(base + 8, 0);
    e.call(0x00cb_0a60, &args![base + 0xc]);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0x8000_0000);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0x8000_0000);
    this
}

// Translated from 00e34800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: writes the vtable word `010eb770` and runs `00cb0a70` (an
/// `hkStringPtr` function, `this` = `this + 0xc`, one stack word `extra`). Returns `this`
/// (`EAX`). The class is unconfirmed (the same as `00e347c0`'s).
pub fn fn_00e34800(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010e_b770);
    e.call(0x00cb_0a70, &args![this.addr() + 0xc, extra]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_47c0, fn_00e347c0(Ptr) -> Ptr),
        entry!(0x00e3_4800, fn_00e34800(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The constructor writes its fields around the string constructor (which runs on
    /// `this + 0xc`).
    #[test]
    fn fn_00e347c0_sets_up_the_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x77; 0x40]);
        e.register_double(0x00cb_0a60, move |e, a| {
            assert_eq!(a, [this + 0xc]);
            assert_eq!(e.mem.u32(this), 0x010e_b770);
            assert_eq!(e.mem.u32(this + 0x10), 0x7777_7777);
            Ret::default()
        });
        assert_eq!(e.call(0x00e3_47c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0x7777);
        assert_eq!(e.mem.u32(this + 8), 0);
        for offset in [0x10, 0x14, 0x18, 0x20, 0x24] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x28), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x2c), 0x7777_7777);
    }

    /// The vtable word is written first, then the string function runs on `this + 0xc`.
    #[test]
    fn fn_00e34800_writes_the_vtable_and_runs_the_string_function() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [this + 0xc, 3]);
            assert_eq!(e.mem.u32(this), 0x010e_b770);
            Ret::default()
        });
        assert_eq!(e.call(0x00e3_4800, &args![this, 3u32]).u32(), this);
    }
}
