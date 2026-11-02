//! `(unplaced)/run_00d54960` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkScaledMoppBvTreeShape` (RTTI
/// `.?AVhkScaledMoppBvTreeShape@@`).
const SCALED_MOPP_VTABLE: u32 = 0x010c_a330;
/// The referenced object at +0x40 and its release (`00c905b0`,
/// `hkreferencedobject.cpp`; thiscall, no stack words).
const REFERENCED: u32 = 0x40;
const REFERENCE_RELEASE_00C905B0: u32 = 0x00c9_05b0;
/// The base destructor body (`00d543e0`, `hkpmoppbvtreeshape.cpp`).
const BASE_DESTRUCT_00D543E0: u32 = 0x00d5_43e0;

// Translated from 00d54960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d54960 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// `hkScaledMoppBvTreeShape`'s destructor body (by the vtable it stores;
/// unnamed in the engine map): the vtable, the object at +0x40 released
/// (`00c905b0`) when non-null, then the base destructor (`00d543e0`).
pub fn fn_00d54960(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCALED_MOPP_VTABLE);
    let referenced = e.mem.u32(this.addr() + REFERENCED);
    if referenced != 0 {
        e.call(REFERENCE_RELEASE_00C905B0, &args![referenced]);
    }
    e.call(BASE_DESTRUCT_00D543E0, &args![this]);
}

/// The thread's memory router (`00c85750`, unit `hkmemoryrouter.cpp`; no arguments).
const MEMORY_ROUTER_INSTANCE: u32 = 0x00c8_5750;
/// The global object whose vtable slots +0x14 and +0x10 `00d54a40` calls with the key
/// `0x7dd65995` (`01268360`).
const KEYED_SERVICE: u32 = 0x0126_8360;
/// The key `00d54a40` passes to it.
const SERVICE_KEY: u32 = 0x7dd6_5995;
/// The compiler object's constructor, set-up calls and destructor (unit Havok SDK,
/// thiscall): `00dd4bf0` (one word), `00dd4b40`, `00dd4c70`, `00dd4b10` (each the address
/// of a parameter block), `00dd4b80` (the mediator; returns a size), `00dd4b00`.
const COMPILER_CTOR: u32 = 0x00dd_4bf0;
const COMPILER_SET_SPLITTER: u32 = 0x00dd_4b40;
const COMPILER_SET_COST: u32 = 0x00dd_4c70;
const COMPILER_SET_CHUNK: u32 = 0x00dd_4b10;
const COMPILER_SIZE: u32 = 0x00dd_4b80;
const COMPILER_DTOR: u32 = 0x00dd_4b00;
/// `hkpMoppCompiler::compile` (engine map, `00dd4cb0`; thiscall: mediator, buffer, size).
const COMPILER_COMPILE: u32 = 0x00dd_4cb0;
/// Two parameter-block constructors (`00dd5150`, `00dd50f0`; thiscall: one word).
const PARAMETERS_CTOR: u32 = 0x00dd_5150;
const CHUNK_PARAMETERS_CTOR: u32 = 0x00dd_50f0;
/// The copy `00d54a40` hands its output array to (`00d5ed80`; thiscall: allocator, 0,
/// data, size).
const ARRAY_COPY: u32 = 0x00d5_ed80;
/// The two mediators `00d54cd0` builds (unit `hkpmoppcachedshapemediator.cpp` and
/// `hkpmoppshapemediator.cpp`): constructors (one word) and destructors.
const CACHED_MEDIATOR_CTOR: u32 = 0x00dd_5d50;
const CACHED_MEDIATOR_DTOR: u32 = 0x00dd_5d90;
const MEDIATOR_CTOR: u32 = 0x00dd_52a0;
const MEDIATOR_DTOR: u32 = 0x00dd_51d0;
/// The floats of the parameter defaults: 0.5 (`01016248`), the weight at `0101df44`, the
/// tolerance at `010181bc` and 0 (`01011d78`); the settings defaults at `01017870`,
/// `010231a8`, `0101e2bc`.
const HALF: u32 = 0x0101_6248;
const WEIGHT_0101DF44: u32 = 0x0101_df44;
const TOLERANCE_010181BC: u32 = 0x0101_81bc;
const ZERO: u32 = 0x0101_1d78;
const SETTING_01017870: u32 = 0x0101_7870;
const SETTING_010231A8: u32 = 0x0102_31a8;
const SETTING_0101E2BC: u32 = 0x0101_e2bc;
/// `bhkHeightFieldShape`'s vtable (`010ca5c4`; RTTI `.?AVbhkHeightFieldShape@@`).
const VTABLE_BHK_HEIGHT_FIELD_SHAPE: u32 = 0x010c_a5c4;
/// `bhkSerializable::Kill` and `bhkShape::~bhkShape` (engine map), and the instance count
/// the destructor decrements.
const BHK_SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BHK_SHAPE_DTOR: u32 = 0x00c8_4c50;
const HEIGHT_FIELD_COUNT: u32 = 0x0126_8f8c;

/// The exe's inlined `hkArray` release (8-byte elements) at `array` (data, size, capacity
/// and flags): size 0, the data freed through the thread's memory router's allocator
/// (+0x10, vtable +0x10) unless the capacity word is negative, then data null and
/// capacity `0x80000000`.
fn release_array(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity >= 0 {
        let bytes = (capacity as u32 & 0x3fff_ffff).wrapping_mul(8);
        let data = e.mem.u32(array);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// A float copied through the x87 stack (`FLD`/`FSTP`: a signalling NaN comes out quiet).
fn copy_float(e: &mut Engine, to: u32, from: u32) {
    let bits = e.mem.u32(from);
    e.mem.set_u32(to, x87_quiet_bits(bits));
}

// Translated from 00d549d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the block `00d54a40` keeps its output in:
/// releases its `hkArray`s at +0x10 and then +4 (8-byte elements, see `release_array`).
pub fn fn_00d549d0(e: &mut Engine, this: Ptr) {
    release_array(e, this.addr() + 0x10);
    release_array(e, this.addr() + 4);
}

// Translated from 00d54a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words; its callers are `00d54cd0`): compiles MOPP
/// code for `mediator` with the settings block `settings`. In a local frame laid out as the
/// exe's: a compiler object (`00dd4bf0` with 0), a splitter block (`00dd5150` with 0, first
/// float 1.0, +0x10 set to 0.0 unless `settings` +0x1c is set and +0x1e clear) handed to
/// `00dd4b40`; a cost block (0.5, the weight at `0101df44`, 1.0, a vector, 5, a byte 1) whose
/// first three floats are the settings' +0x10, +0x14, +0x18 (`00d54db0`, `00d54dd0`,
/// `00d54d70`), its vector the settings' first 16 bytes (`00d54d80`) and its byte the
/// "+0x1f set and +0x1e clear" flag, handed to `00dd4c70`; a chunk block (`00dd50f0` with
/// 0; +8 = 0x32 when settings +0x1d is set, else +4 and +8 cleared; +0xc = 5, +0x14 = the
/// flag) handed to `00dd4b10`. Then a buffer of `00dd4b80(mediator)` bytes from the router's
/// heap (+0xc, vtable +4), an output block (two empty `hkArray`s, 0x200, 0x14) linked into
/// the compiler (+4) when settings +0x1e is set, the keyed service at `01268360` called
/// (slot +0x14 with a local and the key, slot +0x10 with the key and a word whose low byte
/// is 0 over the service pointer's upper bytes), `hkpMoppCompiler::compile`, the service's
/// slot +0x10 again with the key and that local; with `copy_to` set the output's first
/// array is handed to it (`00d5ed80`); the buffer is freed (vtable +8), the output block and
/// the compiler destroyed. Answers `compile`'s result. `_unused_1` is never read.
pub fn fn_00d54a40(
    e: &mut Engine,
    mediator: Ptr,
    _unused_1: u32,
    settings: Ptr,
    copy_to: Ptr,
) -> u32 {
    let settings = settings.addr();
    e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let compiler = frame + 0xa0;
        let block = frame + 0x20;
        let cost = frame + 0x70;
        let output = frame + 0x4c;
        let buffer_slot = frame + 0x18;
        let service_local = frame + 0x48;
        e.call(COMPILER_CTOR, &args![compiler, 0u32]);
        e.call(PARAMETERS_CTOR, &args![block, 0u32]);
        e.mem.set_f32(block, 1.0);
        if e.mem.u8(settings + 0x1c) == 0 || e.mem.u8(settings + 0x1e) != 0 {
            e.mem.set_f32(frame + 0x30, 0.0);
        }
        e.call(COMPILER_SET_SPLITTER, &args![compiler, block]);
        let mut flag = u8::from(e.mem.u8(settings + 0x1f) != 0);
        if e.mem.u8(settings + 0x1e) != 0 {
            flag = 0;
        }
        let weight = e.mem.u32(WEIGHT_0101DF44);
        let tolerance = e.mem.u32(TOLERANCE_010181BC);
        let zero = e.mem.u32(ZERO);
        copy_float(e, cost, HALF);
        copy_float(e, cost + 4, WEIGHT_0101DF44);
        e.mem.set_f32(cost + 8, 1.0);
        e.mem.set_u32(frame + 0x90, 5);
        e.mem.set_u8(frame + 0x94, 1);
        for (lane, bits) in [weight, weight, tolerance, zero].iter().enumerate() {
            e.mem.set_u32(frame + 0x80 + 4 * lane as u32, *bits);
        }
        let value = fn_00d54db0(e, Ptr::new(settings));
        e.mem.set_u32(cost, value.to_bits());
        let value = fn_00d54dd0(e, Ptr::new(settings));
        e.mem.set_u32(cost + 4, value.to_bits());
        let value = fn_00d54d70(e, Ptr::new(settings));
        e.mem.set_u32(cost + 8, value.to_bits());
        let copied = fn_00d54d80(e, Ptr::new(settings), Ptr::new(block)).addr();
        let vector = e.mem.bytes(copied, 0x10);
        e.mem.write(frame + 0x80, &vector);
        e.mem.set_u8(frame + 0x94, flag);
        e.call(COMPILER_SET_COST, &args![compiler, cost]);
        e.call(CHUNK_PARAMETERS_CTOR, &args![block, 0u32]);
        if e.mem.u8(settings + 0x1d) != 0 {
            e.mem.set_u32(frame + 0x28, 0x32);
        } else {
            e.mem.set_u32(frame + 0x24, 0);
            e.mem.set_u32(frame + 0x28, 0);
        }
        e.mem.set_u8(frame + 0x34, flag);
        e.mem.set_u32(frame + 0x2c, 5);
        e.call(COMPILER_SET_CHUNK, &args![compiler, block]);
        let size = e.call(COMPILER_SIZE, &args![compiler, mediator]).u32();
        let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
        let heap = e.mem.u32(router + 0xc);
        let buffer = e.vcall(heap, 0x4, &args![size]).u32();
        e.mem.set_u32(buffer_slot, buffer);
        for (offset, value) in [
            (0x00u32, 0x200u32),
            (0x04, 0),
            (0x08, 0),
            (0x0c, 0x8000_0000),
            (0x10, 0),
            (0x14, 0),
            (0x18, 0x8000_0000),
            (0x1c, 0),
            (0x20, 0x14),
        ] {
            e.mem.set_u32(output + offset, value);
        }
        if e.mem.u8(settings + 0x1e) != 0 {
            e.mem.set_u32(compiler + 4, output);
        }
        let service = e.global::<u32>(KEYED_SERVICE);
        e.vcall(service, 0x14, &args![service_local, SERVICE_KEY]);
        let service = e.global::<u32>(KEYED_SERVICE);
        // `PUSH ECX` then `MOV byte ptr [ESP],0`: the service pointer with its low byte 0.
        e.vcall(service, 0x10, &args![SERVICE_KEY, service & 0xffff_ff00]);
        let buffer = e.mem.u32(buffer_slot);
        let code = e
            .call(COMPILER_COMPILE, &args![compiler, mediator, buffer, size])
            .u32();
        let service = e.global::<u32>(KEYED_SERVICE);
        let local = e.mem.u32(service_local);
        e.vcall(service, 0x10, &args![SERVICE_KEY, local]);
        if copy_to.addr() != 0 {
            let data = e.mem.u32(output + 4);
            let count = e.mem.u32(output + 8);
            let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_COPY, &args![copy_to, allocator, 0u32, data, count]);
        }
        let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
        let heap = e.mem.u32(router + 0xc);
        let buffer = e.mem.u32(buffer_slot);
        e.vcall(heap, 0x8, &args![buffer, size]);
        fn_00d549d0(e, Ptr::new(output));
        e.call(COMPILER_DTOR, &args![compiler]);
        code
    })
}

// Translated from 00d54cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words; called from `Init` and `00cca360`): builds
/// a mediator for `shape` in a local (the cached one, `00dd5d50`, when the settings byte
/// +0x20 is set, else `00dd52a0`), compiles through `00d54a40` (mediator, `shape`,
/// `settings`, `copy_to`), destroys the mediator, and sets the result's byte +0x2c to
/// "settings +0x1e is clear". Answers the result.
pub fn fn_00d54cd0(e: &mut Engine, shape: Ptr, settings: Ptr, copy_to: Ptr) -> u32 {
    let cached = e.mem.u8(settings.addr() + 0x20) != 0;
    let code = e.with_stack(0x30, |e, frame| {
        // The exe's two mediator locals: the cached one at +0x14, the other at +4.
        if cached {
            let mediator = frame.addr() + 0x14;
            e.call(CACHED_MEDIATOR_CTOR, &args![mediator, shape]);
            let code = fn_00d54a40(e, Ptr::new(mediator), shape.addr(), settings, copy_to);
            e.call(CACHED_MEDIATOR_DTOR, &args![mediator]);
            code
        } else {
            let mediator = frame.addr() + 4;
            e.call(MEDIATOR_CTOR, &args![mediator, shape]);
            let code = fn_00d54a40(e, Ptr::new(mediator), shape.addr(), settings, copy_to);
            e.call(MEDIATOR_DTOR, &args![mediator]);
            code
        }
    });
    let clear = u8::from(e.mem.u8(settings.addr() + 0x1e) == 0);
    e.mem.set_u8(code + 0x2c, clear);
    code
}

// Translated from 00d54d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the float at +0x18 in `ST0` (`FLD`: quiet NaN).
pub fn fn_00d54d70(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x18))
}

// Translated from 00d54d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the 16 bytes at `this` to `destination` and answers
/// `destination`.
pub fn fn_00d54d80(e: &mut Engine, this: Ptr, destination: Ptr) -> Ptr {
    for word in 0..4 {
        let value = e.mem.u32(this.addr() + 4 * word);
        e.mem.set_u32(destination.addr() + 4 * word, value);
    }
    destination
}

// Translated from 00d54db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the float at +0x10 in `ST0` (`FLD`: quiet NaN).
pub fn fn_00d54db0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x10))
}

// Translated from 00d54dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the float at +0x14 in `ST0` (`FLD`: quiet NaN).
pub fn fn_00d54dd0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x14))
}

// Translated from 00d54de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called from `Init` and `00cca360`): the settings block's
/// defaults: the vector (t, t, t, 0) from the floats at `010181bc` and `01011d78` (SSE moves,
/// bits kept), the floats +0x18, +0x10, +0x14 from `01017870`, `010231a8`, `0101e2bc`
/// (x87 copies), the bytes +0x1c, +0x1d, +0x1f set to 1 and +0x20, +0x1e to 0. Answers
/// `this`.
pub fn fn_00d54de0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let tolerance = e.mem.u32(TOLERANCE_010181BC);
    let zero = e.mem.u32(ZERO);
    copy_float(e, base + 0x18, SETTING_01017870);
    copy_float(e, base + 0x10, SETTING_010231A8);
    copy_float(e, base + 0x14, SETTING_0101E2BC);
    for (lane, bits) in [tolerance, tolerance, tolerance, zero].iter().enumerate() {
        e.mem.set_u32(base + 4 * lane as u32, *bits);
    }
    e.mem.set_u8(base + 0x1c, 1);
    e.mem.set_u8(base + 0x1d, 1);
    e.mem.set_u8(base + 0x1f, 1);
    e.mem.set_u8(base + 0x20, 0);
    e.mem.set_u8(base + 0x1e, 0);
    this
}

// Translated from 00d54e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (nothing references it): copies the 16-byte vector at
/// `source` to `this` (`MOVAPS`).
pub fn fn_00d54e40(e: &mut Engine, this: Ptr, source: Ptr) {
    let bytes = e.mem.bytes(source.addr(), 0x10);
    e.mem.write(this.addr(), &bytes);
}

// Translated from 00d54e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d54e60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f34a18, state variable) is not translated
/// The `bhkHeightFieldShape` destructor (unnamed in the engine map; class from the vtable it
/// sets): the vtable, `bhkSerializable::Kill`, the instance count at `01268f8c` decremented,
/// then `bhkShape::~bhkShape`.
pub fn bhk_height_field_shape_dtor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BHK_HEIGHT_FIELD_SHAPE);
    e.call(BHK_SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(HEIGHT_FIELD_COUNT);
    e.set_global::<u32>(HEIGHT_FIELD_COUNT, count.wrapping_sub(1));
    e.call(BHK_SHAPE_DTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_4960, fn_00d54960(Ptr)),
        entry!(0x00d5_49d0, fn_00d549d0(Ptr)),
        entry!(0x00d5_4a40, fn_00d54a40(Ptr, u32, Ptr, Ptr) -> u32),
        entry!(0x00d5_4cd0, fn_00d54cd0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d5_4d70, fn_00d54d70(Ptr) -> f32),
        entry!(0x00d5_4d80, fn_00d54d80(Ptr, Ptr) -> Ptr),
        entry!(0x00d5_4db0, fn_00d54db0(Ptr) -> f32),
        entry!(0x00d5_4dd0, fn_00d54dd0(Ptr) -> f32),
        entry!(0x00d5_4de0, fn_00d54de0(Ptr) -> Ptr),
        entry!(0x00d5_4e40, fn_00d54e40(Ptr, Ptr)),
        entry!(0x00d5_4e60, bhk_height_field_shape_dtor(Ptr)),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .cloned()
            .collect()
    }

    /// A router whose allocator (+0x10, slot +0x10 at `0079_0010`) and heap (+0xc, slots
    /// +4 / +8 at `0078_0004` / `0078_0008`, allocating answers 0x5500) are doubles.
    fn router(e: &mut Engine) -> (u32, u32) {
        let allocator_vtable = e.mem.alloc(0x14);
        e.mem.set_u32(allocator_vtable + 0x10, 0x0079_0010);
        e.register(0x0079_0010, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, allocator_vtable);
        let heap_vtable = e.mem.alloc(0xc);
        e.mem.set_u32(heap_vtable + 4, 0x0078_0004);
        e.mem.set_u32(heap_vtable + 8, 0x0078_0008);
        e.register(0x0078_0004, |_, _| 0x5500u32.into_ret());
        e.register(0x0078_0008, |_, _| Ret::default());
        let heap = e.mem.alloc(4);
        e.mem.set_u32(heap, heap_vtable);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0xc, heap);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_INSTANCE, move |_, _| router.into_ret());
        (allocator, heap)
    }

    #[test]
    fn release_struct_frees_both_arrays() {
        let mut e = Engine::new();
        let (allocator, _) = router(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, 0x6000);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 4);
        e.mem.set_u32(this + 0x10, 0x7000);
        e.mem.set_u32(this + 0x14, 1);
        e.mem.set_u32(this + 0x18, 0x8000_0002);
        e.call_log = Some(vec![]);
        e.call(0x00d5_49d0, &args![this]);
        assert_eq!(
            calls_to(&e, 0x0079_0010),
            vec![(0x0079_0010, vec![allocator, 0x6000, 32])]
        );
        for off in [4u32, 8, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
        assert_eq!(e.mem.u32(this + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
    }

    #[test]
    fn float_getters_and_vector_copies() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0x10, 1.5);
        e.mem.set_f32(this + 0x14, 2.5);
        e.mem.set_u32(this + 0x18, 0x7f80_0001);
        assert_eq!(e.call(0x00d5_4db0, &args![this]).f32(), 1.5);
        assert_eq!(e.call(0x00d5_4dd0, &args![this]).f32(), 2.5);
        assert_eq!(
            e.call(0x00d5_4d70, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
        for w in 0..4 {
            e.mem.set_u32(this + 4 * w, 0x100 + w);
        }
        let other = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d5_4d80, &args![this, other]).u32(), other);
        assert_eq!(e.mem.bytes(other, 16), e.mem.bytes(this, 16));
        let third = e.mem.alloc(0x10);
        e.call(0x00d5_4e40, &args![third, this]);
        assert_eq!(e.mem.bytes(third, 16), e.mem.bytes(this, 16));
    }

    #[test]
    fn settings_defaults() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x0003_0000);
        e.mem.set_f32(TOLERANCE_010181BC, 0.05);
        e.mem.set_f32(SETTING_01017870, 0.3);
        e.mem.set_f32(SETTING_010231A8, 0.4);
        e.mem.set_u32(SETTING_0101E2BC, 0x7f80_0001);
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xff; 0x24]);
        assert_eq!(e.call(0x00d5_4de0, &args![this]).u32(), this);
        assert_eq!(
            [0, 4, 8, 12].map(|o| e.mem.f32(this + o)),
            [0.05, 0.05, 0.05, 0.0]
        );
        assert_eq!(e.mem.f32(this + 0x10), 0.4);
        assert_eq!(e.mem.u32(this + 0x14), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x18), 0.3);
        assert_eq!(e.mem.bytes(this + 0x1c, 5), vec![1, 1, 0, 1, 0]);
        assert_eq!(e.mem.bytes(this + 0x21, 3), vec![0xff; 3]);
    }

    #[test]
    fn height_field_dtor_counts_down() {
        let mut e = Engine::new();
        e.map(HEIGHT_FIELD_COUNT, 4);
        e.mem.set_u32(HEIGHT_FIELD_COUNT, 3);
        e.register(BHK_SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(BHK_SHAPE_DTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d5_4e60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BHK_SERIALIZABLE_KILL, vec![this]));
        assert_eq!(log[2], (BHK_SHAPE_DTOR, vec![this]));
        assert_eq!(e.mem.u32(HEIGHT_FIELD_COUNT), 2);
        assert_eq!(e.mem.u32(this), VTABLE_BHK_HEIGHT_FIELD_SHAPE);
    }

    /// Doubles for every callee of `00d54a40`; the compiler's size is 0x40 and `compile`
    /// answers a block of 0x30 bytes.
    fn compile_doubles(e: &mut Engine) -> u32 {
        e.map(0x0101_0000, 0x0003_0000);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(WEIGHT_0101DF44, 0.2);
        e.mem.set_f32(TOLERANCE_010181BC, 0.05);
        for addr in [
            COMPILER_CTOR,
            COMPILER_SET_SPLITTER,
            COMPILER_SET_CHUNK,
            COMPILER_DTOR,
            PARAMETERS_CTOR,
            CHUNK_PARAMETERS_CTOR,
            ARRAY_COPY,
            CACHED_MEDIATOR_CTOR,
            CACHED_MEDIATOR_DTOR,
            MEDIATOR_CTOR,
            MEDIATOR_DTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(COMPILER_SIZE, |_, _| 0x40u32.into_ret());
        e.register(COMPILER_SET_COST, |e, a| {
            // The cost block: the settings' three floats, their vector, the flag byte.
            assert_eq!(e.mem.f32(a[1]), 4.0);
            assert_eq!(e.mem.f32(a[1] + 4), 5.0);
            assert_eq!(e.mem.f32(a[1] + 8), 6.0);
            assert_eq!(e.mem.f32(a[1] + 0x10), 1.0);
            assert_eq!(e.mem.u32(a[1] + 0x20), 5);
            Ret::default()
        });
        let code = e.mem.alloc(0x30);
        e.register_double(COMPILER_COMPILE, move |_, _| code.into_ret());
        let service_vtable = e.mem.alloc(0x18);
        e.mem.set_u32(service_vtable + 0x10, 0x0077_0010);
        e.mem.set_u32(service_vtable + 0x14, 0x0077_0014);
        e.register(0x0077_0010, |_, _| Ret::default());
        e.register(0x0077_0014, |e, a| {
            e.mem.set_u32(a[1], 0x99);
            Ret::default()
        });
        let service = e.mem.alloc(0x10) + 0x5;
        e.mem.set_u32(service, service_vtable);
        e.map(KEYED_SERVICE, 4);
        e.mem.set_u32(KEYED_SERVICE, service);
        code
    }

    fn settings(e: &mut Engine, bytes: [u8; 5]) -> u32 {
        let settings = e.mem.alloc(0x24);
        for (lane, v) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
            e.mem.set_f32(settings + 4 * lane as u32, *v);
        }
        e.mem.set_f32(settings + 0x10, 4.0);
        e.mem.set_f32(settings + 0x14, 5.0);
        e.mem.set_f32(settings + 0x18, 6.0);
        e.mem.write(settings + 0x1c, &bytes);
        settings
    }

    #[test]
    fn compile_runs_the_compiler_and_frees_the_buffer() {
        let mut e = Engine::new();
        let code = compile_doubles(&mut e);
        let (allocator, heap) = router(&mut e);
        let settings = settings(&mut e, [1, 1, 0, 1, 0]);
        e.call_log = Some(vec![]);
        let r = e
            .call(
                0x00d5_4a40,
                &args![0x4000u32, 0x9999u32, settings, 0x4400u32],
            )
            .u32();
        assert_eq!(r, code);
        let service = e.mem.u32(KEYED_SERVICE);
        let compile = calls_to(&e, COMPILER_COMPILE);
        assert_eq!(&compile[0].1[1..], &[0x4000, 0x5500, 0x40]);
        assert_eq!(
            calls_to(&e, 0x0077_0010)
                .iter()
                .map(|c| c.1.clone())
                .collect::<Vec<_>>(),
            vec![
                vec![service, SERVICE_KEY, service & 0xffff_ff00],
                vec![service, SERVICE_KEY, 0x99]
            ]
        );
        assert_eq!(
            calls_to(&e, ARRAY_COPY)[0].1,
            vec![0x4400, allocator, 0, 0, 0]
        );
        assert_eq!(
            calls_to(&e, 0x0078_0008),
            vec![(0x0078_0008, vec![heap, 0x5500, 0x40])]
        );
        assert_eq!(calls_to(&e, COMPILER_DTOR).len(), 1);
        // Without a copy target nothing is copied.
        e.call_log = Some(vec![]);
        e.call(0x00d5_4a40, &args![0x4000u32, 0u32, settings, 0u32]);
        assert!(calls_to(&e, ARRAY_COPY).is_empty());
    }

    #[test]
    fn build_picks_the_mediator_and_marks_the_result() {
        for (cached, keep, mark) in [(1u8, 0u8, 1u8), (0, 1, 0)] {
            let mut e = Engine::new();
            let code = compile_doubles(&mut e);
            router(&mut e);
            let settings = settings(&mut e, [1, 1, keep, 1, cached]);
            e.call_log = Some(vec![]);
            let r = e.call(0x00d5_4cd0, &args![0x3000u32, settings, 0u32]).u32();
            assert_eq!(r, code);
            assert_eq!(e.mem.u8(code + 0x2c), mark);
            let (ctor, dtor) = if cached == 1 {
                (CACHED_MEDIATOR_CTOR, CACHED_MEDIATOR_DTOR)
            } else {
                (MEDIATOR_CTOR, MEDIATOR_DTOR)
            };
            let built = calls_to(&e, ctor);
            assert_eq!(built.len(), 1);
            assert_eq!(built[0].1[1], 0x3000);
            assert_eq!(calls_to(&e, dtor)[0].1[0], built[0].1[0]);
            assert_eq!(calls_to(&e, COMPILER_SIZE)[0].1[1], built[0].1[0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn others(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a != 0x00d5_4960)
            .cloned()
            .collect()
    }

    #[test]
    fn releases_the_referenced_object_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(REFERENCE_RELEASE_00C905B0, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00D543E0, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + REFERENCED, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x00d5_4960, &args![this]);
        assert_eq!(e.mem.u32(this), SCALED_MOPP_VTABLE);
        assert_eq!(
            others(&e),
            vec![
                (REFERENCE_RELEASE_00C905B0, vec![0x0700_0000]),
                (BASE_DESTRUCT_00D543E0, vec![this]),
            ]
        );
        e.mem.set_u32(this + REFERENCED, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_4960, &args![this]);
        assert_eq!(others(&e), vec![(BASE_DESTRUCT_00D543E0, vec![this])]);
    }
}
