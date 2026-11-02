//! `BSHavok/bhkconvexsweepshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConvexSweepShape` is a `bhkShape` (0x14 bytes) whose construction info is a 0x14-byte
//! block (`CreateCinfo`). The vtable `010caaf4` is the one the RTTI names
//! `bhkConvexSweepShape`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{enter_scope, leave_scope};

/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate method `(size)` (`00aa3e40`).
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill` (`00c876a0`) and the `bhkConvexShape` destructor body (`00c9fb00`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CONVEX_SHAPE_DESTRUCT: u32 = 0x00c9_fb00;
/// `bhkConvexShape::LoadBinary` (`00c9fbc0`) and `bhkShape::LinkObject` (`00c84cc0`, a jump).
const CONVEX_SHAPE_LOAD_BINARY: u32 = 0x00c9_fbc0;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
/// `NiStream::ResolveLinkID` (`00a64620`, `this` is the stream).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The vtables of the classes the constructor passes through (RTTI-confirmed), and their
/// live-object counts (the sweep shape's own class has none).
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const VTABLE_CONVEX_SWEEP_SHAPE: u32 = 0x010c_aaf4;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
/// The floats copied into the construction info.
const FIRST_FLOAT: u32 = 0x010c_72bc;
const SECOND_FLOAT: u32 = 0x0101_7868;
const THIRD_FLOAT: u32 = 0x0101_712c;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

// Translated from 00cc74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QCinfoSize` of the sweep shape (the address is shared with other classes' identical
/// code, so the name is unconfirmed): takes no arguments and answers 0x14, the size of the
/// construction info this class builds.
pub fn fn_00cc74d0(_e: &mut Engine) -> u32 {
    0x14
}

// Translated from 00ccf4f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf4f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexSweepShape` destructor body (the sized-delete wrapper at `00ccf5b0` calls it
/// as such; Ghidra's own name for the address is an unrelated one): sets the class vtable,
/// runs `bhkSerializable::Kill` and the `bhkConvexShape` destructor body (`00c9fb00`).
pub fn bhk_convex_sweep_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_SWEEP_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    e.call(CONVEX_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00ccf560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexSweepShape::Init` (Xbox PDB): a tail jump through the shape's virtual slot
/// `0xBC` with `ECX` still `this`, so the caller's stack words (`forwarded`) reach the
/// virtual function unchanged and its result is returned.
pub fn bhk_convex_sweep_shape_init(e: &mut Engine, this: Ptr, forwarded: &[u32]) -> Ret {
    e.vcall(this.addr(), 0xbc, forwarded)
}

/// Uniform form of [`bhk_convex_sweep_shape_init`]: `this`, then the forwarded words.
fn bhk_convex_sweep_shape_init_entry(e: &mut Engine, words: &[u32]) -> Ret {
    bhk_convex_sweep_shape_init(e, Ptr::new(words[0]), &words[1..])
}

// Translated from 00ccf570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexSweepShape::LoadBinary` (Xbox PDB): reads a link id from the stream
/// (`NiStream::ResolveLinkID`, `00a64620`), runs `bhkConvexShape::LoadBinary`, then calls the
/// stream's virtual slot `0x28` with the id.
pub fn bhk_convex_sweep_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let link_id = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    e.call(CONVEX_SHAPE_LOAD_BINARY, &args![this, stream]);
    e.vcall(stream, 0x28, &args![link_id]);
}

// Translated from 00ccf5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexSweepShape::LinkObject` (Xbox PDB): a tail jump to `bhkShape::LinkObject`
/// (`00c84cc0`) with the same `(this, stream)`; its result is returned.
pub fn bhk_convex_sweep_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]).u32()
}

// Translated from 00ccf5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf5d0 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates a 0x14-byte block (`operator new`) and constructs a `bhkConvexSweepShape` in
/// it (unnamed in the engine map): the base constructor (`00c87c30`), `+0xC` and `+0x10`
/// cleared, then the vtable of each class on the way down (`bhkShape`, `bhkSphereRepShape`,
/// `bhkConvexShape`, `bhkConvexSweepShape`) with the live-object count of the first three
/// raised. Answers the object, or 0 when the allocation fails.
pub fn fn_00ccf5d0(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, VTABLE_SHAPE);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    increment_count(e, COUNT_SHAPE);
    e.mem.set_u32(block, VTABLE_SPHERE_REP_SHAPE);
    increment_count(e, COUNT_SPHERE_REP_SHAPE);
    e.mem.set_u32(block, VTABLE_CONVEX_SHAPE);
    increment_count(e, COUNT_CONVEX_SHAPE);
    e.mem.set_u32(block, VTABLE_CONVEX_SWEEP_SHAPE);
    Ptr::new(block)
}

// Translated from 00ccf680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf680 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexSweepShape::CreateCinfo` (Xbox PDB), `(this, created)`: in the Havok
/// allocation scope stores into the byte at `created` whether the shape had no cinfo
/// (`+0xC` null); in that case allocates a 0x14-byte block from the pool at `011f6238`
/// (null stays null), fills it (`+0` and `+8` zero, `+4`, `+0xC` and `+0x10` the floats at
/// `010c72bc`, `01017868` and `0101712c` through the x87 stack) and stores it at `+0xC`.
/// Answers the cinfo at `+0xC`.
pub fn bhk_convex_sweep_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let cinfo = e.call(POOL_ALLOCATE, &args![POOL, 0x14u32]).u32();
        if cinfo != 0 {
            e.mem.set_u32(cinfo, 0);
            let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
            e.mem.set_u32(cinfo + 4, first);
            e.mem.set_u32(cinfo + 8, 0);
            let second = x87_quiet_bits(e.mem.u32(SECOND_FLOAT));
            e.mem.set_u32(cinfo + 0xc, second);
            let third = x87_quiet_bits(e.mem.u32(THIRD_FLOAT));
            e.mem.set_u32(cinfo + 0x10, third);
        }
        e.mem.set_u32(this.addr() + 0xc, cinfo);
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00ccf5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexSweepShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccf4f0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_convex_sweep_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_f4f0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccf550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexSweepShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_86b8.
pub fn bhk_convex_sweep_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_86b8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc74d0, fn_00cc74d0() -> u32),
        entry!(0x00ccf4f0, bhk_convex_sweep_shape_destructor(Ptr)),
        entry!(0x00ccf550, bhk_convex_sweep_shape_get_r_t_t_i() -> u32),
        (0x00ccf560, bhk_convex_sweep_shape_init_entry as AbiFn),
        entry!(0x00ccf570, bhk_convex_sweep_shape_load_binary(Ptr, u32)),
        entry!(0x00ccf5a0, bhk_convex_sweep_shape_link_object(Ptr, u32) -> u32),
        entry!(0x00ccf5b0, bhk_convex_sweep_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ccf5d0, fn_00ccf5d0() -> Ptr),
        entry!(0x00ccf680, bhk_convex_sweep_shape_create_cinfo(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn bhk_convex_sweep_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_f4f0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ccf5b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ccf5b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_f4f0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_convex_sweep_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_f550, &args![]).u32(), 0x0126_86b8);
    }

    #[test]
    fn fn_00cc74d0_answers_0x14() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc74d0, &args![]).u32(), 0x14);
    }

    #[test]
    fn destructor_sets_the_vtable_then_kills_and_destructs() {
        let mut e = Engine::new();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(CONVEX_SHAPE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00ccf4f0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_SWEEP_SHAPE);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![0x00ccf4f0, SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]
        );
        assert_eq!(calls_to(&e, CONVEX_SHAPE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn init_jumps_to_slot_0xbc_with_the_forwarded_words() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0xbc, 0x0900_0100);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccf560, &args![this, 41u32]).u32(), 42);
        assert_eq!(calls_to(&e, 0x0900_0100), vec![vec![this, 41]]);
    }

    #[test]
    fn load_binary_resolves_the_id_loads_the_base_and_hands_the_id_to_the_stream() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x28, 0x0900_0101);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream, vtable);
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(CONVEX_SHAPE_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x0900_0101, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00ccf570, &args![this, stream]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x00ccf570,
                STREAM_RESOLVE_LINK_ID,
                CONVEX_SHAPE_LOAD_BINARY,
                0x0900_0101
            ]
        );
        assert_eq!(calls_to(&e, STREAM_RESOLVE_LINK_ID), vec![vec![stream]]);
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![stream, 0x1234]]);
    }

    #[test]
    fn link_object_is_a_tail_jump_to_the_shape_link_object() {
        let mut e = Engine::new();
        e.register(SHAPE_LINK_OBJECT, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccf5a0, &args![0x10u32, 0x20u32]).u32(), 9);
        assert_eq!(calls_to(&e, SHAPE_LINK_OBJECT), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn constructor_builds_a_shape_chain_without_a_sweep_count() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let made = e.call(0x00ccf5d0, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u32(made), VTABLE_CONVEX_SWEEP_SHAPE);
        assert_eq!((e.mem.u32(made + 0xc), e.mem.u32(made + 0x10)), (0, 0));
        for counter in [COUNT_SHAPE, COUNT_SPHERE_REP_SHAPE, COUNT_CONVEX_SHAPE] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
        // A failed allocation answers null and constructs nothing.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccf5d0, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }

    #[test]
    fn create_cinfo_builds_the_missing_cinfo_once() {
        let mut e = Engine::new();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        for (addr, bits) in [
            (FIRST_FLOAT, 0x3f80_0000u32),
            (SECOND_FLOAT, 0x7f80_0001),
            (THIRD_FLOAT, 0x4000_0000),
        ] {
            e.map(addr, 4);
            e.mem.set_u32(addr, bits);
        }
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00ccf680, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x14]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.u32(cinfo + 4), 0x3f80_0000);
        assert_eq!(e.mem.u32(cinfo + 8), 0);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x4000_0000);
        assert_eq!(e.mem.u32(word), 0x77);
        // The second call finds the cinfo.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccf680, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        // A failed pool allocation stores null.
        e.mem.set_u32(this + 0xc, 0);
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ccf680, &args![this, created]).u32(), 0);
        assert_eq!(e.mem.u8(created), 1);
    }
}
