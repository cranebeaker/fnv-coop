//! `BSMain/run_00c612b0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a99a90` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00A99A90: u32 = 0x00a9_9a90;

// Translated from 00c61450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35c8, the word before 010c35cc (this class's
/// table has no named neighbours; position by the object order of the neighbouring tables).
/// Forwards `this` and the stream to `00a99a90`, an extra-data `LoadBinary` body. Returns
/// nothing: Xbox PDB `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c61450(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00A99A90, &args![this, stream]);
}

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c61470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35cc, after the word 010c35c8. Forwards `this`
/// and the stream to `00a9af00`, a one-instruction jump to `NiExtraData::LinkObject`
/// (`00a7b3b0`). Returns nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is
/// void; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn fn_00c61470(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

/// `00a89210` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00A89210: u32 = 0x00a8_9210;

// Translated from 00c614c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c35d4, three words after 010c35c8 (the position
/// of `SaveBinary` in the neighbouring tables). Forwards `this` and the stream to `00a89210`
/// (`nifloatsextradata.cpp` in the engine map). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c614c0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00A89210, &args![this, stream]);
}

/// The `NiExtraData` default constructor (`00a7b2e0`) and destructor body (`00a7b300`),
/// both `thiscall`.
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `NiIntegersExtraData`'s constructor with its values (`00a99980`; `thiscall`, `RET 8`),
/// member copy (`00a99870`; `thiscall` on the source, `RET 8`: clone, cloning process) and
/// `IsEqual` (`00a998e0`, engine map; `thiscall`, `RET 4`).
const INTEGERS_CONSTRUCTOR: u32 = 0x00a9_9980;
const INTEGERS_COPY_MEMBERS: u32 = 0x00a9_9870;
const INTEGERS_IS_EQUAL: u32 = 0x00a9_98e0;
/// `NiBinaryExtraData::RegisterStreamables` (`00a89ae0`, engine map; the shared extra-data
/// body, `thiscall`, `RET 4`).
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a8_9ae0;
/// `00c61610` (`thiscall`): run by both `BSWArray` constructors after the virtual table.
const W_ARRAY_SET_UP: u32 = 0x00c6_1610;
/// The block free (`00aa10f0`, `cdecl`), `NiNew` (`00aa13e0`, `cdecl`: size) and the name
/// setter `00a5b950` (unit `niobjectnet.cpp`; `thiscall`, `RET 4`) with the word whose
/// address the factory passes (`011f49e0`).
const NI_FREE: u32 = 0x00aa_10f0;
const NI_NEW: u32 = 0x00aa_13e0;
const SET_NAME: u32 = 0x00a5_b950;
const CREATED_NAME: u32 = 0x011f_49e0;
/// The `NiIntegersExtraData` (`010a0c7c`) and `BSWArray` (`010c357c`) virtual tables (by
/// RTTI).
const INTEGERS_EXTRA_DATA_VTABLE: u32 = 0x010a_0c7c;
const W_ARRAY_VTABLE: u32 = 0x010c_357c;

// Translated from 00c612b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c612b0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f300f8, state variable) is not translated
/// The `BSWArray` default constructor (by its virtual table; unnamed in the engine map;
/// `thiscall`): `NiExtraData`'s constructor (`00a7b2e0`), the inlined `NiIntegersExtraData`
/// one (its virtual table, no values: +0x10 and +0xC cleared), the `BSWArray` virtual table,
/// then `00c61610`. Answers `this`.
pub fn fn_00c612b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(EXTRA_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), INTEGERS_EXTRA_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr(), W_ARRAY_VTABLE);
    e.call(W_ARRAY_SET_UP, &args![this]);
    this
}

// Translated from 00c61330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61330 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3011b, state variable) is not translated
/// The `BSWArray` constructor with values (unnamed in the engine map; `thiscall`, `RET 8`):
/// `NiIntegersExtraData`'s constructor (`00a99980`) with both words, the `BSWArray` virtual
/// table, then `00c61610`. Answers `this`.
pub fn fn_00c61330(e: &mut Engine, this: Ptr, count: u32, values: u32) -> Ptr {
    e.call(INTEGERS_CONSTRUCTOR, &args![this, count, values]);
    e.mem.set_u32(this.addr(), W_ARRAY_VTABLE);
    e.call(W_ARRAY_SET_UP, &args![this]);
    this
}

// Translated from 00c613a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c613a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3013e, state variable) is not translated
/// The `BSWArray` destructor (unnamed in the engine map; `thiscall`): the `BSWArray` and then
/// the `NiIntegersExtraData` virtual table, the values (+0x10) freed (`00aa10f0`) and
/// cleared, then `NiExtraData`'s destructor (`00a7b300`).
pub fn fn_00c613a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), W_ARRAY_VTABLE);
    e.mem.set_u32(this.addr(), INTEGERS_EXTRA_DATA_VTABLE);
    let values = e.mem.u32(this.addr() + 0x10);
    e.call(NI_FREE, &args![values]);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00c61420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the stack word is not read): answers
/// `this`.
pub fn fn_00c61420(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 00c61430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSWArray` member copy (unnamed in the engine map; `thiscall` on the source, `RET 8`:
/// clone, cloning process): forwards to `NiIntegersExtraData`'s (`00a99870`).
pub fn fn_00c61430(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(INTEGERS_COPY_MEMBERS, &args![this, clone, cloning]);
}

// Translated from 00c61490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: stream): answers 1 when `00a89ae0`
/// answers a zero byte and 0 otherwise (`NEG`/`SBB`/`ADD 1` on the byte).
pub fn fn_00c61490(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        == 0
}

// Translated from 00c614e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSWArray` `IsEqual` (unnamed in the engine map; `thiscall`, `RET 4`): the byte
/// `NiIntegersExtraData::IsEqual` (`00a998e0`) answers.
pub fn fn_00c614e0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(INTEGERS_IS_EQUAL, &args![this, other]).u8() != 0
}

// Translated from 00c61510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61510 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30168, state variable) is not translated
/// The `BSWArray` factory (no arguments): a 0x14-byte `NiNew` block constructed
/// ([`fn_00c612b0`]), or null when the allocation fails; then `00a5b950` runs on the result
/// (also when it is null) with `011f49e0`. Answers the result.
pub fn fn_00c61510(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x14u32]).u32();
    let object = if block == 0 {
        0
    } else {
        fn_00c612b0(e, Ptr::new(block)).addr()
    };
    e.call(SET_NAME, &args![object, CREATED_NAME]);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_12b0, fn_00c612b0(Ptr) -> Ptr),
        entry!(0x00c6_1330, fn_00c61330(Ptr, u32, u32) -> Ptr),
        entry!(0x00c6_13a0, fn_00c613a0(Ptr)),
        entry!(0x00c6_1420, fn_00c61420(Ptr, u32) -> Ptr),
        entry!(0x00c6_1430, fn_00c61430(Ptr, Ptr, Ptr)),
        entry!(0x00c6_1490, fn_00c61490(Ptr, Ptr) -> bool),
        entry!(0x00c6_14e0, fn_00c614e0(Ptr, Ptr) -> bool),
        entry!(0x00c6_1510, fn_00c61510() -> u32),
        entry!(0x00c6_1450, fn_00c61450(Ptr, Ptr)),
        entry!(0x00c6_1470, fn_00c61470(Ptr, Ptr)),
        entry!(0x00c6_14c0, fn_00c614c0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c61450_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00A99A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1450, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00A99A90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c61470_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1470, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c614c0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00A89210, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_14c0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00A89210)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            EXTRA_DATA_CONSTRUCTOR,
            EXTRA_DATA_DESTRUCTOR,
            INTEGERS_CONSTRUCTOR,
            INTEGERS_COPY_MEMBERS,
            W_ARRAY_SET_UP,
            NI_FREE,
            SET_NAME,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != top)
            .collect()
    }

    #[test]
    fn constructors_and_destructor() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xaa; 0x14]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_12b0, &args![this]).u32(), this);
        assert_eq!(
            log_of(&mut e, 0x00c6_12b0),
            vec![
                (EXTRA_DATA_CONSTRUCTOR, vec![this]),
                (W_ARRAY_SET_UP, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), W_ARRAY_VTABLE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c6_1330, &args![this, 3u32, 0x500u32]).u32(),
            this
        );
        assert_eq!(
            log_of(&mut e, 0x00c6_1330),
            vec![
                (INTEGERS_CONSTRUCTOR, vec![this, 3, 0x500]),
                (W_ARRAY_SET_UP, vec![this])
            ]
        );
        e.mem.set_u32(this + 0x10, 0x600);
        e.call_log = Some(vec![]);
        e.call(0x00c6_13a0, &args![this]);
        assert_eq!(
            log_of(&mut e, 0x00c6_13a0),
            vec![(NI_FREE, vec![0x600]), (EXTRA_DATA_DESTRUCTOR, vec![this])]
        );
        assert_eq!(e.mem.u32(this), INTEGERS_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn small_forwarders() {
        let mut e = engine();
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, a| a[1].into_ret());
        e.register(INTEGERS_IS_EQUAL, |_, a| a[1].into_ret());
        assert_eq!(e.call(0x00c6_1420, &args![0x100u32, 7u32]).u32(), 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00c6_1430, &args![0x100u32, 0x200u32, 0x300u32]);
        assert_eq!(
            log_of(&mut e, 0x00c6_1430),
            vec![(INTEGERS_COPY_MEMBERS, vec![0x100, 0x200, 0x300])]
        );
        assert!(e.call(0x00c6_1490, &args![0x100u32, 0x200u32]).bool());
        assert!(!e.call(0x00c6_1490, &args![0x100u32, 0x201u32]).bool());
        assert!(e.call(0x00c6_14e0, &args![0x100u32, 0x301u32]).bool());
        assert!(!e.call(0x00c6_14e0, &args![0x100u32, 0x300u32]).bool());
    }

    #[test]
    fn the_factory_names_even_a_null_object() {
        let mut e = engine();
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_1510, &args![]).u32(), 0);
        assert_eq!(
            log_of(&mut e, 0x00c6_1510),
            vec![(NI_NEW, vec![0x14]), (SET_NAME, vec![0, CREATED_NAME])]
        );
        let block = e.mem.alloc(0x14);
        e.register_double(NI_NEW, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_1510, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), W_ARRAY_VTABLE);
        assert!(log_of(&mut e, 0x00c6_1510).contains(&(SET_NAME, vec![block, CREATED_NAME])));
    }
}
