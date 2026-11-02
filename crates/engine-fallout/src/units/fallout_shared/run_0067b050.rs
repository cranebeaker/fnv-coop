//! `fallout shared/run_0067b050` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESDialoguePackageData`'s vtable (RTTI `.?AVTESDialoguePackageData@@`).
const TES_DIALOGUE_PACKAGE_DATA_VTABLE: u32 = 0x0106_88ac;
/// The `float` the constructor loads for +0x4 (100.0 in the exe).
const DEFAULT_DISTANCE: u32 = 0x0101_6410;

// Translated from 0067b170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDialoguePackageData` constructor (the class of the vtable `010688ac`
/// it stores, by its RTTI): the `TESPackageData` constructor (`0067ad20`),
/// the vtable, the `float` at `01016410` into +0x4 (`FLD`/`FSTP`), words
/// +0x8, +0xC, +0x1C = 0, bytes +0x10, +0x11, +0x18 = 0 and the float
/// +0x14 = 0.0 (`FLDZ`). Returns `this`.
pub fn fn_0067b170(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(base, TES_DIALOGUE_PACKAGE_DATA_VTABLE);
    let distance = e.mem.u32(DEFAULT_DISTANCE);
    e.mem.set_u32(base + 0x4, x87_quiet_bits(distance));
    e.mem.set_u32(base + 0x8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    e.mem.set_u8(base + 0x11, 0);
    e.mem.set_u8(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_f32(base + 0x14, 0.0);
    this
}

// Translated from 0067b050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067b050 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Loads from the load game buffer (`RET 4`; the engine map gives it no
/// name): a byte (`00864980`); when set, a new 0xC-byte object
/// (`00401000`, constructed with `0067f030`, or 0 when the allocation
/// fails) is stored at +4 and loads itself (`0067fbe0` with the buffer).
pub fn fn_0067b050(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let present = e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        e.call(0x0086_4980, &args![buffer, flag, 1u32]);
        e.mem.u8(flag.addr())
    });
    if present == 0 {
        return;
    }
    let block = e.call(0x0040_1000, &args![0xcu32]).u32();
    let object = if block != 0 {
        e.call(0x0067_f030, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 4, object);
    let object = e.mem.u32(this.addr() + 4);
    e.call(0x0067_fbe0, &args![object, buffer]);
}

// Translated from 0067b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears four parts (`thiscall`, no stack words): `00401080` (`cdecl`)
/// with a zero byte on +0, +4, +0x14 and +0xC, in that order.
pub fn fn_0067b100(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0, 4, 0x14, 0xc] {
        e.call(0x0040_1080, &args![base + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_b170, fn_0067b170(Ptr) -> Ptr),
        entry!(0x0067_b050, fn_0067b050(Ptr, Ptr)),
        entry!(0x0067_b100, fn_0067b100(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_0067b170_builds_the_dialogue_package_data() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(DEFAULT_DISTANCE, 100.0);
        e.register(0x0067_ad20, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        for i in 0..0x24 {
            e.mem.set_u8(this + i, 0xcc);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_b170, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), TES_DIALOGUE_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.f32(this + 4), 100.0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.bytes(this + 0x10, 4), vec![0, 0, 0xcc, 0xcc]);
        assert_eq!(e.mem.f32(this + 0x14), 0.0);
        assert_eq!(e.mem.bytes(this + 0x18, 4), vec![0, 0xcc, 0xcc, 0xcc]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0xcccc_cccc);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn load_makes_the_object_when_flagged() {
        let mut e = Engine::new();
        e.register(0x0086_4980, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        e.register(0x0040_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0067_f030, |_, a| (a[0] + 1).into_ret());
        e.register(0x0067_fbe0, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0067_b050, &args![this, 0x8800u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 4), 0x7001);
        assert_eq!(log.last().unwrap(), &(0x0067_fbe0, vec![0x7001, 0x8800]));
        // A failed allocation stores 0 and still loads.
        e.register(0x0040_1000, |_, _| Ret::default());
        e.call(0x0067_b050, &args![this, 0x8800u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        // Without the flag nothing more is read.
        e.register(0x0086_4980, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b050, &args![this, 0x8800u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn clears_four_parts() {
        let mut e = Engine::new();
        e.register(0x0040_1080, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b100, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        let cleared: Vec<u32> = log[1..].iter().map(|c| c.1[0]).collect();
        assert_eq!(cleared, vec![0x5000, 0x5004, 0x5014, 0x500c]);
    }
}
