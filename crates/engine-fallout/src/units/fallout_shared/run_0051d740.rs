//! `fallout shared/run_0051d740` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051d770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x28 (0x388 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_c51c`) is `TESObjectWEAP`, so this is the constructor of `TESObjectWEAP`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051d770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_30a0, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x84]);
    e.call(0x0048_eac0, &args![base + 0x8c]);
    e.call(0x0048_71d0, &args![base + 0x94]);
    e.call(0x0047_fdc0, &args![base + 0x9c]);
    e.call(0x0047_48f0, &args![base + 0xa4]);
    e.call(0x0047_50b0, &args![base + 0xac]);
    e.call(0x0047_5350, &args![base + 0xb4]);
    e.call(0x0047_b850, &args![base + 0xbc]);
    e.call(0x0047_9320, &args![base + 0xc4]);
    e.call(0x004f_9d90, &args![base + 0xcc]);
    e.call(0x0047_a130, &args![base + 0xd0]);
    e.call(0x0047_4d10, &args![base + 0xe0]);
    e.call(0x0047_b4f0, &args![base + 0xe8]);
    e.mem.set_u32(base, 0x0102_c51c);
    e.mem.set_u32(base + 0x30, 0x0102_c504);
    e.mem.set_u32(base + 0x3c, 0x0102_c4e0);
    e.mem.set_u32(base + 0x5c, 0x0102_c4c0);
    e.mem.set_u32(base + 0x68, 0x0102_c4ac);
    e.mem.set_u32(base + 0x74, 0x0102_c494);
    e.mem.set_u32(base + 0x84, 0x0102_c474);
    e.mem.set_u32(base + 0x8c, 0x0102_c460);
    e.mem.set_u32(base + 0x94, 0x0102_c448);
    e.mem.set_u32(base + 0x9c, 0x0102_c430);
    e.mem.set_u32(base + 0xa4, 0x0102_c41c);
    e.mem.set_u32(base + 0xac, 0x0102_c408);
    e.mem.set_u32(base + 0xb4, 0x0102_c3f4);
    e.mem.set_u32(base + 0xbc, 0x0102_c3e0);
    e.mem.set_u32(base + 0xc4, 0x0102_c3cc);
    e.mem.set_u32(base + 0xcc, 0x0102_c3b4);
    e.mem.set_u32(base + 0xd0, 0x0102_c3a0);
    e.mem.set_u32(base + 0xe0, 0x0102_c38c);
    e.mem.set_u32(base + 0xe8, 0x0102_c378);
    e.call(0x0048_8f50, &args![base + 0x1d0]);
    e.call(0x0048_8f50, &args![base + 0x1e8]);
    e.call(0x0048_8f50, &args![base + 0x200]);
    e.call(0x0048_a3d0, &args![base + 0x270]);
    e.call(0x0048_a3d0, &args![base + 0x290]);
    e.call(0x0048_a3d0, &args![base + 0x2b0]);
    e.call(0x0048_a3d0, &args![base + 0x2d0]);
    e.call(0x0048_a3d0, &args![base + 0x2f0]);
    e.call(0x0048_a3d0, &args![base + 0x310]);
    e.call(0x0048_a3d0, &args![base + 0x330]);
    e.call(0x0040_37b0, &args![base + 0x35c]);
    e.call(0x0040_37b0, &args![base + 0x368]);
    e.call(0x004f_15a0, &args![this, 0x28u32]);
    e.call(0x0051_ddf0, &args![this]);
    this
}

// Translated from 0051d760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `0051b900`. `0051b900` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0051d760(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_b900, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 0051ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of `TESObjectWEAP` (the class `0051d770` constructs; the RTTI name of its
/// vtable is `TESObjectWEAP`): clears the `0xcc` bytes at `+0xf4` (`memset`, `00403d30`), stores
/// the default weapon fields (floats copied from the exe's constants at `0x0101712c`,
/// `0x01017b78`, `0x01013d84`, `0x01013970`, `0x01016248`, `0x01026994`, `0x0101e2bc`), runs
/// virtual slot 0 of the ten components at `+0x1d0`, `+0x1e8`, `+0x200`, `+0x270`, `+0x290`,
/// `+0x2b0`, `+0x2d0`, `+0x2f0`, `+0x310` and `+0x330`, sets the current spell of the
/// component at `+0x74` to 2 (`NonActorMagicCaster::SetCurrentSpell`, `0041fd00`) and sets the
/// byte at `+0x384` when `0051e2a0` answers 0xc4 or more (signed).
pub fn fn_0051ddf0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_3d30, &args![base + 0xf4, 0u32, 0xccu32]);
    e.mem.set_u8(base + 0x102, 1);
    copy_float(e, 0x0101_712c, base + 0x134);
    e.mem.set_f32(base + 0x104, 0.0);
    copy_float(e, 0x0101_7b78, base + 0x108);
    e.mem.set_f32(base + 0x10c, 1.0);
    e.mem.set_u8(base + 0x11c, 5);
    e.mem.set_u8(base + 0x103, 0);
    e.mem.set_u8(base + 0x11d, 0xff);
    e.mem.set_u8(base + 0x101, 0xff);
    e.mem.set_u32(base + 0x118, 0);
    e.mem.set_u8(base + 0x11e, 1);
    copy_float(e, 0x0101_3d84, base + 0x120);
    copy_float(e, 0x0101_3970, base + 0x124);
    e.mem.set_u32(base + 0x128, 0);
    e.mem.set_f32(base + 0x138, 0.0);
    copy_float(e, 0x0101_6248, base + 0x13c);
    e.mem.set_f32(base + 0x140, 1.0);
    copy_float(e, 0x0102_6994, base + 0x144);
    e.mem.set_f32(base + 0x148, 1.0);
    e.mem.set_u32(base + 0x15c, 0x2d);
    e.mem.set_f32(base + 0x168, 1.0);
    e.mem.set_f32(base + 0x170, 0.0);
    e.mem.set_u32(base + 0x16c, 0xffff_ffff);
    copy_float(e, 0x0101_e2bc, base + 0x174);
    copy_float(e, 0x0101_6248, base + 0x178);
    e.mem.set_u32(base + 0x19c, 0);
    e.mem.set_u32(base + 0x1bc, 0);
    e.mem.set_f32(base + 0x18c, 0.0);
    e.mem.set_f32(base + 0x190, 0.0);
    e.mem.set_f32(base + 0x194, 0.0);
    e.mem.set_f32(base + 0x1ac, 0.0);
    e.mem.set_f32(base + 0x1b0, 0.0);
    e.mem.set_f32(base + 0x1b4, 0.0);
    e.mem.set_f32(base + 0x1a8, 1.0);
    e.mem.set_u8(base + 0x198, 0xff);
    e.mem.set_u8(base + 0x1a1, 0xff);
    e.mem.set_u8(base + 0x1a0, 0xff);
    e.mem.set_u16(base + 0x1c0, 0);
    e.mem.set_f32(base + 0x1c4, 1.0);
    e.mem.set_u8(base + 0x1c8, 1);
    e.mem.set_u32(base + 0x1cc, 0);
    e.mem.set_f32(base + 0x374, 0.0);
    e.mem.set_f32(base + 0x378, 0.0);
    e.mem.set_u32(base + 0x370, 0);
    e.mem.set_f32(base + 0x37c, 0.0);
    e.mem.set_u8(base + 0x380, 1);
    e.mem.set_u8(base + 0x381, 0);
    for offset in (0x218..=0x26c).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0x350, 0x354, 0x358] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [
        0x1d0, 0x1e8, 0x200, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330,
    ] {
        e.vcall(base + offset, 0, &args![]);
    }
    e.call(0x0041_fd00, &args![base + 0x74, 2u32]);
    e.mem.set_u32(base + 0x364, 1);
    e.mem.set_u8(base + 0x384, 0);
    let value = e.call(0x0051_e2a0, &args![this, 0u32]).i32();
    if value >= 0xc4 {
        e.mem.set_u8(base + 0x384, 1);
    }
}

/// `FLD dword [source]` then `FSTP dword [destination]`.
fn copy_float(e: &mut Engine, source: u32, destination: u32) {
    let value = x87_quiet(e.global::<f32>(source));
    e.mem.set_f32(destination, value);
}

// Translated from 0051dad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0051db00` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0051dad0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_db00, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0051d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at the address `00408d60` returns for the global at
/// `011d5ce8` (`cdecl`, no arguments).
pub fn fn_0051d740(e: &mut Engine) -> u8 {
    let at = e.call(0x0040_8d60, &args![0x011d_5ce8u32]).u32();
    e.mem.u8(at)
}

// Translated from 0051e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0xB1` plus one of the weapon's bytes: with `alternate` false the byte
/// at +0x103 (zero-extended); otherwise the signed byte at +0x1A1, unless
/// that byte sign-extended equals 0xFF, when the byte at +0x103 again. The
/// exe compares the sign-extended byte with 0xFF (not -1), so that test
/// never holds and a 0xFF byte gives `0xB1 - 1`; translated as the exe
/// does it.
pub fn fn_0051e2a0(e: &mut Engine, this: Ptr, alternate: u8) -> i32 {
    let base = this.addr();
    if alternate == 0 {
        return i32::from(e.mem.u8(base + 0x103)) + 0xb1;
    }
    let signed = i32::from(e.mem.i8(base + 0x1a1));
    if signed == 0xff {
        i32::from(e.mem.u8(base + 0x103)) + 0xb1
    } else {
        signed + 0xb1
    }
}

/// The offsets of the ten components `0051e300` clears (each through its
/// own vtable slot 4).
const CLEARED_COMPONENTS: [u32; 10] = [
    0x1d0, 0x1e8, 0x200, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330,
];

// Translated from 0051e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the weapon's ten model components (vtable slot 4,
/// `ClearDataComponent`, of each at +0x1D0 .. +0x330) and empties the two
/// strings at +0x35C and +0x368 (`004037f0` with 0, 0).
pub fn fn_0051e300(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in CLEARED_COMPONENTS {
        e.vcall(base + offset, 4, &[]);
    }
    e.call(0x0040_37f0, &args![base + 0x35c, 0u32, 0u32]);
    e.call(0x0040_37f0, &args![base + 0x368, 0u32, 0u32]);
}

// Translated from 0051db00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051db00 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESObjectWEAP`'s destructor body: stores the class's 19 vtables (at +0
/// and in each component), clears the components (`0051e300`), then
/// destroys the members and bases last to first by address and finally the
/// base destructor `0050e9f0` (tesobject.cpp).
pub fn fn_0051db00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vtables: [(u32, u32); 19] = [
        (0x00, 0x0102_c51c),
        (0x30, 0x0102_c504),
        (0x3c, 0x0102_c4e0),
        (0x5c, 0x0102_c4c0),
        (0x68, 0x0102_c4ac),
        (0x74, 0x0102_c494),
        (0x84, 0x0102_c474),
        (0x8c, 0x0102_c460),
        (0x94, 0x0102_c448),
        (0x9c, 0x0102_c430),
        (0xa4, 0x0102_c41c),
        (0xac, 0x0102_c408),
        (0xb4, 0x0102_c3f4),
        (0xbc, 0x0102_c3e0),
        (0xc4, 0x0102_c3cc),
        (0xcc, 0x0102_c3b4),
        (0xd0, 0x0102_c3a0),
        (0xe0, 0x0102_c38c),
        (0xe8, 0x0102_c378),
    ];
    for (offset, vtable) in vtables {
        e.mem.set_u32(base + offset, vtable);
    }
    fn_0051e300(e, this);
    let members: [(u32, u32); 24] = [
        (0x368, 0x0040_37d0),
        (0x35c, 0x0040_37d0),
        (0x330, 0x0048_a490),
        (0x310, 0x0048_a490),
        (0x2f0, 0x0048_a490),
        (0x2d0, 0x0048_a490),
        (0x2b0, 0x0048_a490),
        (0x290, 0x0048_a490),
        (0x270, 0x0048_a490),
        (0x200, 0x0048_9070),
        (0x1e8, 0x0048_9070),
        (0x1d0, 0x0048_9070),
        (0xe8, 0x0047_b520),
        (0xe0, 0x0047_4d40),
        (0xd0, 0x0047_a1a0),
        (0xbc, 0x0047_b880),
        (0xb4, 0x0047_5380),
        (0xac, 0x0047_50e0),
        (0x9c, 0x0047_fdf0),
        (0x94, 0x0048_7200),
        (0x8c, 0x0048_eaf0),
        (0x84, 0x0048_e7d0),
        (0x5c, 0x0040_2c40),
        (0x3c, 0x0048_a490),
    ];
    for (offset, destructor) in members {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

/// The table of name pointers `0051dab0` indexes (`0118a870`, in the exe's
/// data; its first entry points at the text `HandToHandMelee`).
const NAME_TABLE: u32 = 0x0118_a870;

// Translated from 0051dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): the name pointer the table at `0118a870`
/// holds for the signed byte at +0xF4 (`MOVSX`, so a negative byte reads
/// before the table, as the exe does).
pub fn fn_0051dab0(e: &mut Engine, this: Ptr) -> u32 {
    let index = e.mem.u8(this.addr() + 0xf4) as i8 as i32 as u32;
    e.mem.u32(NAME_TABLE.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_d770, fn_0051d770(Ptr) -> Ptr),
        entry!(0x0051_d760, fn_0051d760(Ptr, u32) -> u32),
        entry!(0x0051_ddf0, fn_0051ddf0(Ptr)),
        entry!(0x0051_dad0, fn_0051dad0(Ptr, u32) -> Ptr),
        entry!(0x0051_d740, fn_0051d740() -> u8),
        entry!(0x0051_e2a0, fn_0051e2a0(Ptr, u8) -> i32),
        entry!(0x0051_e300, fn_0051e300(Ptr)),
        entry!(0x0051_db00, fn_0051db00(Ptr)),
        entry!(0x0051_dab0, fn_0051dab0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051d770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_d770,
            size: 0x388,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_30a0, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x84)]),
                (0x0048_eac0, &[A::T(0x8c)]),
                (0x0048_71d0, &[A::T(0x94)]),
                (0x0047_fdc0, &[A::T(0x9c)]),
                (0x0047_48f0, &[A::T(0xa4)]),
                (0x0047_50b0, &[A::T(0xac)]),
                (0x0047_5350, &[A::T(0xb4)]),
                (0x0047_b850, &[A::T(0xbc)]),
                (0x0047_9320, &[A::T(0xc4)]),
                (0x004f_9d90, &[A::T(0xcc)]),
                (0x0047_a130, &[A::T(0xd0)]),
                (0x0047_4d10, &[A::T(0xe0)]),
                (0x0047_b4f0, &[A::T(0xe8)]),
                (0x0048_8f50, &[A::T(0x1d0)]),
                (0x0048_8f50, &[A::T(0x1e8)]),
                (0x0048_8f50, &[A::T(0x200)]),
                (0x0048_a3d0, &[A::T(0x270)]),
                (0x0048_a3d0, &[A::T(0x290)]),
                (0x0048_a3d0, &[A::T(0x2b0)]),
                (0x0048_a3d0, &[A::T(0x2d0)]),
                (0x0048_a3d0, &[A::T(0x2f0)]),
                (0x0048_a3d0, &[A::T(0x310)]),
                (0x0048_a3d0, &[A::T(0x330)]),
                (0x0040_37b0, &[A::T(0x35c)]),
                (0x0040_37b0, &[A::T(0x368)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x28)]),
                (0x0051_ddf0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_c51c),
                (0x30, 4, 0x0102_c504),
                (0x3c, 4, 0x0102_c4e0),
                (0x5c, 4, 0x0102_c4c0),
                (0x68, 4, 0x0102_c4ac),
                (0x74, 4, 0x0102_c494),
                (0x84, 4, 0x0102_c474),
                (0x8c, 4, 0x0102_c460),
                (0x94, 4, 0x0102_c448),
                (0x9c, 4, 0x0102_c430),
                (0xa4, 4, 0x0102_c41c),
                (0xac, 4, 0x0102_c408),
                (0xb4, 4, 0x0102_c3f4),
                (0xbc, 4, 0x0102_c3e0),
                (0xc4, 4, 0x0102_c3cc),
                (0xcc, 4, 0x0102_c3b4),
                (0xd0, 4, 0x0102_c3a0),
                (0xe0, 4, 0x0102_c38c),
                (0xe8, 4, 0x0102_c378),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0051d760() {
        assert_eq!(
            check(0x0051_d760, 0x0051_b900, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    const SIZE: u32 = 0x388;
    const COMPONENTS: [u32; 10] = [
        0x1d0, 0x1e8, 0x200, 0x270, 0x290, 0x2b0, 0x2d0, 0x2f0, 0x310, 0x330,
    ];
    const FLOAT_GLOBALS: [(u32, u32); 7] = [
        (0x0101_712c, 0x4020_0000),
        (0x0101_7b78, 0x4100_0000),
        (0x0101_3d84, 0x4380_0000),
        (0x0101_3970, 0x4480_0000),
        (0x0101_6248, 0x3f00_0000),
        (0x0102_6994, 0x7f80_0001),
        (0x0101_e2bc, 0x3dcc_cccd),
    ];

    type Calls = Vec<(u32, Vec<u32>)>;

    /// The object after the call, the logged calls and the object's address.
    fn run(spell_answer: u32) -> (Vec<u8>, Calls, u32) {
        let mut e = Engine::new();
        for (address, bits) in FLOAT_GLOBALS {
            e.map(address, 4);
            e.mem.set_u32(address, bits);
        }
        let this = e.mem.alloc(SIZE);
        e.mem.write(this, &[0xcd; SIZE as usize]);
        // Each component's vtable has the same slot 0.
        e.put_vtable(0x0090_1000, &[0x0090_0100]);
        for offset in COMPONENTS {
            e.mem.set_u32(this + offset, 0x0090_1000);
        }
        for address in [0x0090_0100, 0x0040_3d30, 0x0041_fd00] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(0x0051_e2a0, move |_, _| Ret {
            eax: spell_answer,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0051_ddf0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        (e.mem.bytes(this, SIZE), log, this)
    }

    fn expected_bytes() -> Vec<u8> {
        let mut bytes = vec![0xcd_u8; SIZE as usize];
        let mut put = |offset: usize, width: usize, value: u32| {
            bytes[offset..offset + width].copy_from_slice(&value.to_le_bytes()[..width]);
        };
        let one = 0x3f80_0000;
        for offset in COMPONENTS {
            put(offset as usize, 4, 0x0090_1000);
        }
        put(0x102, 1, 1);
        put(0x134, 4, 0x4020_0000);
        put(0x104, 4, 0);
        put(0x108, 4, 0x4100_0000);
        put(0x10c, 4, one);
        put(0x11c, 1, 5);
        put(0x103, 1, 0);
        put(0x11d, 1, 0xff);
        put(0x101, 1, 0xff);
        put(0x118, 4, 0);
        put(0x11e, 1, 1);
        put(0x120, 4, 0x4380_0000);
        put(0x124, 4, 0x4480_0000);
        put(0x128, 4, 0);
        put(0x138, 4, 0);
        put(0x13c, 4, 0x3f00_0000);
        put(0x140, 4, one);
        // A signalling NaN picks up the quiet bit through FLD/FSTP.
        put(0x144, 4, 0x7fc0_0001);
        put(0x148, 4, one);
        put(0x15c, 4, 0x2d);
        put(0x168, 4, one);
        put(0x170, 4, 0);
        put(0x16c, 4, 0xffff_ffff);
        put(0x174, 4, 0x3dcc_cccd);
        put(0x178, 4, 0x3f00_0000);
        for offset in [0x19c, 0x1bc, 0x18c, 0x190, 0x194, 0x1ac, 0x1b0, 0x1b4] {
            put(offset, 4, 0);
        }
        put(0x1a8, 4, one);
        put(0x198, 1, 0xff);
        put(0x1a1, 1, 0xff);
        put(0x1a0, 1, 0xff);
        put(0x1c0, 2, 0);
        put(0x1c4, 4, one);
        put(0x1c8, 1, 1);
        put(0x1cc, 4, 0);
        put(0x374, 4, 0);
        put(0x378, 4, 0);
        put(0x370, 4, 0);
        put(0x37c, 4, 0);
        put(0x380, 1, 1);
        put(0x381, 1, 0);
        for offset in (0x218..=0x26c).step_by(4) {
            put(offset, 4, 0);
        }
        for offset in [0x350, 0x354, 0x358] {
            put(offset, 4, 0);
        }
        put(0x364, 4, 1);
        put(0x384, 1, 0);
        bytes
    }

    #[test]
    fn fn_0051ddf0_sets_the_start_values_and_runs_the_components() {
        let (bytes, log, this) = run(0xc3);
        assert_eq!(bytes, expected_bytes());
        let calls: Calls = log[1..].to_vec();
        let mut want: Calls = vec![(0x0040_3d30, vec![this + 0xf4, 0, 0xcc])];
        for offset in COMPONENTS {
            want.push((0x0090_0100, vec![this + offset]));
        }
        want.push((0x0041_fd00, vec![this + 0x74, 2]));
        want.push((0x0051_e2a0, vec![this, 0]));
        assert_eq!(calls, want);
    }

    #[test]
    fn fn_0051ddf0_sets_the_flag_byte_from_0xc4_up_and_not_for_negatives() {
        let mut want = expected_bytes();
        want[0x384] = 1;
        assert_eq!(run(0xc4).0, want);
        assert_eq!(run(0x1000).0, want);
        want[0x384] = 0;
        assert_eq!(run(0xffff_ff00).0, want);
    }
    #[test]
    fn at_0051dad0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_db00, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_dad0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_dad0)
                .collect();
            let mut expected = vec![(0x0051_db00, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    #[test]
    fn b020_0051d740_reads_the_byte() {
        let mut e = Engine::new();
        let p = e.mem.alloc(4);
        e.mem.set_u8(p, 0x9a);
        e.register_double(0x0040_8d60, move |_, a| {
            assert_eq!(a, &[0x011d_5ce8]);
            Ret {
                eax: p,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0051_d740, &[]).u8(), 0x9a);
    }

    #[test]
    fn b020_0051e2a0_picks_the_byte() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x200);
        e.mem.set_u8(p + 0x103, 0x0c);
        e.mem.set_u8(p + 0x1a1, 0x05);
        assert_eq!(e.call(0x0051_e2a0, &args![p, 0u32]).i32(), 0xbd);
        assert_eq!(e.call(0x0051_e2a0, &args![p, 0x100u32]).i32(), 0xbd);
        assert_eq!(e.call(0x0051_e2a0, &args![p, 1u32]).i32(), 0xb6);
        e.mem.set_u8(p + 0x1a1, 0xff);
        assert_eq!(e.call(0x0051_e2a0, &args![p, 1u32]).i32(), 0xb0);
        e.mem.set_u8(p + 0x1a1, 0x80);
        assert_eq!(e.call(0x0051_e2a0, &args![p, 1u32]).i32(), 0xb1 - 0x80);
    }

    #[test]
    fn b020_0051e300_clears_the_components_and_strings() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0x0000_1004]);
        e.register(0x0000_1004, |_, _| Ret::default());
        e.register(0x0040_37f0, |_, _| Ret::default());
        let p = e.mem.alloc(0x388);
        for o in CLEARED_COMPONENTS {
            e.mem.set_u32(p + o, 0x0150_0000);
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_e300, &args![p]);
        let mut want: Vec<(u32, Vec<u32>)> = CLEARED_COMPONENTS
            .iter()
            .map(|o| (0x0000_1004, vec![p + o]))
            .collect();
        want.push((0x0040_37f0, vec![p + 0x35c, 0, 0]));
        want.push((0x0040_37f0, vec![p + 0x368, 0, 0]));
        assert_eq!(calls(&mut e, 0x0051_e300), want);
    }

    #[test]
    fn b020_0051db00_destroys_the_weapon() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0x0000_1004]);
        let p = e.mem.alloc(0x388);
        // The ten cleared components (not given a vtable by the
        // destructor) use the test vtable.
        for o in CLEARED_COMPONENTS {
            e.mem.set_u32(p + o, 0x0150_0000);
        }
        e.register(0x0000_1004, |_, _| Ret::default());
        e.register(0x0040_37f0, |_, _| Ret::default());
        for a in [
            0x0040_37d0,
            0x0048_a490,
            0x0048_9070,
            0x0047_b520,
            0x0047_4d40,
            0x0047_a1a0,
            0x0047_b880,
            0x0047_5380,
            0x0047_50e0,
            0x0047_fdf0,
            0x0048_7200,
            0x0048_eaf0,
            0x0048_e7d0,
            0x0040_2c40,
            0x0059_1300,
            0x0050_e9f0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_db00, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0102_c51c);
        assert_eq!(e.mem.u32(p + 0x30), 0x0102_c504);
        assert_eq!(e.mem.u32(p + 0xa4), 0x0102_c41c);
        assert_eq!(e.mem.u32(p + 0xe8), 0x0102_c378);
        let log = calls(&mut e, 0x0051_db00);
        assert_eq!(log.len(), 38);
        assert_eq!(log[0], (0x0000_1004, vec![p + 0x1d0]));
        assert_eq!(log[11], (0x0040_37f0, vec![p + 0x368, 0, 0]));
        assert_eq!(log[12], (0x0040_37d0, vec![p + 0x368]));
        assert_eq!(log[21], (0x0048_9070, vec![p + 0x200]));
        assert_eq!(log[34], (0x0040_2c40, vec![p + 0x5c]));
        assert_eq!(log[35], (0x0048_a490, vec![p + 0x3c]));
        assert_eq!(log[36], (0x0059_1300, vec![p + 0x30]));
        assert_eq!(log[37], (0x0050_e9f0, vec![p]));
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_0051dab0_reads_the_name_table_by_signed_byte() {
        let mut e = Engine::new();
        e.map(0x0118_a000, 0x1000);
        e.set_global(NAME_TABLE, 0x0100_0000u32);
        e.set_global(NAME_TABLE + 8, 0x0100_0008u32);
        e.set_global(NAME_TABLE - 4, 0x00ff_fffcu32);
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x0051_dab0, &[this]).u32(), 0x0100_0000);
        e.mem.set_u8(this + 0xf4, 2);
        assert_eq!(e.call(0x0051_dab0, &[this]).u32(), 0x0100_0008);
        e.mem.set_u8(this + 0xf4, 0xff);
        assert_eq!(e.call(0x0051_dab0, &[this]).u32(), 0x00ff_fffc);
    }
}
