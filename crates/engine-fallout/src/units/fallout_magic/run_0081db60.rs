//! `fallout/magic/run_0081db60` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008646f0` (`thiscall`, no stack words): the `BGSLoadGameBuffer` destructor body (stores
/// the vtable `01082048`, RTTI `.?AVBGSLoadGameBuffer@@`, and frees the buffer at +4).
const LOAD_GAME_BUFFER_DESTRUCT: u32 = 0x0086_46f0;

// Translated from 0081db90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs the `BGSLoadGameBuffer`
/// destructor body (`008646f0`) on `this`.
pub fn fn_0081db90(e: &mut Engine, this: Ptr) {
    e.call(LOAD_GAME_BUFFER_DESTRUCT, &args![this]);
}

/// `MemoryManager` allocation and free (`00401000`, `00401030`, `cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `MobileObject::MobileObject` (engine map, `0092eb50`), `0046a010` (`thiscall`, one
/// word), `HighProcess::HighProcess` (`008d7510`), `007af430` (`thiscall`, no stack words;
/// `BGSSaveFormBuffer::GetForm` in the engine map) and `TESObjectREFR::SetObjectReference`
/// (`00575690`, one word).
const MOBILE_OBJECT_CONSTRUCT: u32 = 0x0092_eb50;
const REFERENCE_0046A010: u32 = 0x0046_a010;
const HIGH_PROCESS_CONSTRUCT: u32 = 0x008d_7510;
const FORM_OF_007AF430: u32 = 0x007a_f430;
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// The vtables the constructor stores (`0107b394` at +0, `0107b388` at +0x18); both carry
/// the RTTI `.?AVMagicProjectile@@`.
const VTABLE_0107B394: u32 = 0x0107_b394;
const VTABLE_0107B388: u32 = 0x0107_b388;
/// `MagicProjectile::~MagicProjectile` (engine map, `0081de70`).
const MAGIC_PROJECTILE_DESTRUCT: u32 = 0x0081_de70;

// Translated from 0081db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): a scalar deleting destructor around `0081db90`
/// (the destructor body), freeing the memory (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0081db60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0081db90(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0081dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0081dbb0 - the C++ exception frame (FS:[0] chain, handler 00f114b3, unwind states 0 and 1 for the MobileObject base and the new process) is not translated
/// Unnamed in the engine map (`thiscall`): a constructor on the `MobileObject` base
/// (`0092eb50`) storing the vtables `0107b394` (+0) and `0107b388` (+0x18), running
/// `0046a010(1)`, clearing the `float`s at +0x88, +0x8C and +0x90 and the words +0x94 to
/// +0xA0, giving the object a new `HighProcess` (`00401000(0x46c)`, `008d7510`; 0 when the
/// allocation fails) at +0x68, and setting its object reference to its own form
/// (`00575690` with `007af430`). Returns `this`.
pub fn fn_0081dbb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MOBILE_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, VTABLE_0107B394);
    e.mem.set_u32(base + 0x18, VTABLE_0107B388);
    e.call(REFERENCE_0046A010, &args![this, 1u32]);
    e.mem.set_u32(base + 0x90, 0.0f32.to_bits());
    for offset in [0x94u32, 0x98, 0x9c, 0xa0] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x88, 0.0f32.to_bits());
    e.mem.set_u32(base + 0x8c, 0.0f32.to_bits());
    let memory = e.call(ALLOCATE, &args![0x46cu32]).u32();
    let process = if memory != 0 {
        e.call(HIGH_PROCESS_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x68, process);
    let form = e.call(FORM_OF_007AF430, &args![this]).u32();
    e.call(SET_OBJECT_REFERENCE, &args![this, form]);
    this
}

// Translated from 0081dcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): a scalar deleting destructor around
/// `MagicProjectile::~MagicProjectile` (`0081de70`), freeing the memory (`00401030`) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_0081dcd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(MAGIC_PROJECTILE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_db90, fn_0081db90(Ptr)),
        entry!(0x0081_db60, fn_0081db60(Ptr, u32) -> Ptr),
        entry!(0x0081_dbb0, fn_0081dbb0(Ptr) -> Ptr),
        entry!(0x0081_dcd0, fn_0081dcd0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0081db90_runs_the_destructor_body() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_BUFFER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0081_db90, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(LOAD_GAME_BUFFER_DESTRUCT, vec![0x5000])]);
    }
}

#[cfg(test)]
mod n087_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for callee in [
            ALLOCATE,
            FREE,
            MOBILE_OBJECT_CONSTRUCT,
            REFERENCE_0046A010,
            HIGH_PROCESS_CONSTRUCT,
            FORM_OF_007AF430,
            SET_OBJECT_REFERENCE,
            MAGIC_PROJECTILE_DESTRUCT,
            LOAD_GAME_BUFFER_DESTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0081db60_destroys_and_frees_with_bit_0() {
        let mut e = rig();
        assert_eq!(e.call(0x0081_db60, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls(&e, LOAD_GAME_BUFFER_DESTRUCT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000]]);
        e.call(0x0081_db60, &args![0x5000u32, 0u32]);
        assert_eq!(calls(&e, FREE).len(), 1);
    }

    #[test]
    fn fn_0081dbb0_constructs_with_a_process() {
        let mut e = rig();
        let object = e.mem.alloc(0xa4);
        e.mem.set_u32(object + 0x94, 7);
        e.register(ALLOCATE, |_, _| 0x0c00_0000u32.into_ret());
        e.register(HIGH_PROCESS_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(FORM_OF_007AF430, |_, _| 0x0c10_0000u32.into_ret());
        assert_eq!(e.call(0x0081_dbb0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), VTABLE_0107B394);
        assert_eq!(e.mem.u32(object + 0x18), VTABLE_0107B388);
        assert_eq!(e.mem.u32(object + 0x94), 0);
        assert_eq!(e.mem.u32(object + 0x68), 0x0c00_0001);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x46c]]);
        assert_eq!(calls(&e, REFERENCE_0046A010), vec![vec![object, 1]]);
        assert_eq!(
            calls(&e, SET_OBJECT_REFERENCE),
            vec![vec![object, 0x0c10_0000]]
        );
        // A failed allocation leaves no process.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call(0x0081_dbb0, &args![object]);
        assert_eq!(e.mem.u32(object + 0x68), 0);
    }

    #[test]
    fn fn_0081dcd0_runs_the_projectile_destructor() {
        let mut e = rig();
        assert_eq!(e.call(0x0081_dcd0, &args![0x5000u32, 3u32]).u32(), 0x5000);
        assert_eq!(calls(&e, MAGIC_PROJECTILE_DESTRUCT), vec![vec![0x5000]]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000]]);
    }
}
