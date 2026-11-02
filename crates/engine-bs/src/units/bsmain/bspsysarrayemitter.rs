//! `BSMain/bspsysarrayemitter.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `00c31b40` (named `NiPSysVolumeEmitter::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31B40: u32 = 0x00c3_1b40;

// Translated from 00c61f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::LinkObject` (engine map vtable name, vtable word 010c3714): forwards
/// `this` and the stream to `00c31b40` (`NiPSysVolumeEmitter::LinkObject`). Returns nothing:
/// Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31B40, &args![this, stream]);
}

/// `00c2f4f0` (named `NiPSysAirFieldModifier::RegisterStreamables` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C2F4F0: u32 = 0x00c2_f4f0;

// Translated from 00c61f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::RegisterStreamables` (engine map vtable name, vtable word 010c3718):
/// forwards `this` and the stream to `00c2f4f0`, a one-instruction jump to `00c28b60`, itself
/// a jump to `NiPSysModifier::RegisterStreamables` (`00c31670`). Returns the `bool` in `AL`:
/// Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is
/// the `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C2F4F0, &args![this, stream])
        .bool()
}

/// `00c31b60` (named `NiPSysVolumeEmitter::SaveBinary` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00C31B60: u32 = 0x00c3_1b60;

// Translated from 00c61f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::SaveBinary` (engine map vtable name, vtable word 010c371c): forwards
/// `this` and the stream to `00c31b60` (`NiPSysVolumeEmitter::SaveBinary`). Returns nothing:
/// Xbox PDB `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00C31B60, &args![this, stream]);
}

/// `00c31b80` (named `NiPSysVolumeEmitter::IsEqual` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const IS_EQUAL_TARGET_00C31B80: u32 = 0x00c3_1b80;

// Translated from 00c61fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::IsEqual` (engine map vtable name, vtable word 010c3720): forwards
/// `this` and the other object to `00c31b80` (`NiPSysVolumeEmitter::IsEqual`). Returns the
/// `bool` in `AL`: Xbox PDB `?IsEqual@NiObject@@UAA_NPAV1@@Z` returns `_N` and the callee's
/// equal path ends `MOV AL,1`; `other` is the other `NiObject`. Parameter count from the `RET
/// 4` of both bodies.
pub fn bs_psys_array_emitter_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(IS_EQUAL_TARGET_00C31B80, &args![this, other]).bool()
}

// Translated from 00c632e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043b8 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00c632e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43b8
}

// Translated from 00c63320 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043b0 without reading the receiver. The map names this body `BSPSysArrayEmitter::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c63320(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43b0
}

/// `BSPSysArrayEmitter` (Xbox PDB, 0x6C bytes on PC): vtable `010c36c4`;
/// `NiPSysModifier::m_pkTarget` +0x10, `NiPSysVolumeEmitter::m_pkEmitterObj` +0x54, `eType`
/// +0x58 (0 vertices, 1 specific objects, 2 nodes, by the viewer strings) and `aObjects`
/// (`NiTPrimitiveArray<NiPointer<NiAVObject>>`, the shared `NiTArray` layout) +0x5C.
const VTABLE: u32 = 0x010c_36c4;
const EMITTER_SIZE: u32 = 0x6c;
const TARGET: u32 = 0x10;
const EMITTER_OBJECT: u32 = 0x54;
const ARRAY_TYPE: u32 = 0x58;
const OBJECTS: u32 = 0x5c;
/// The vtable the emitter's constructor gives its array (`010c36b8`) and the
/// `NiTPrimitiveArray` vtable (`010c377c`).
const OBJECTS_VTABLE: u32 = 0x010c_36b8;
const PRIMITIVE_ARRAY_VTABLE: u32 = 0x010c_377c;
/// `NiPSysVolumeEmitter`'s constructor (`00c31af0`), member copy (`00c31b10`, `(destination,
/// cloning)`), `ProcessClone` (`00c31c90`), `LoadBinary` (`00c31b20`) and `GetViewerStrings`
/// (`00c31db0`); `NiPSysModifier::~NiPSysModifier` (`00c31750`); all `thiscall`.
const VOLUME_EMITTER_CONSTRUCT: u32 = 0x00c3_1af0;
const VOLUME_EMITTER_COPY_MEMBERS: u32 = 0x00c3_1b10;
const VOLUME_EMITTER_PROCESS_CLONE: u32 = 0x00c3_1c90;
const VOLUME_EMITTER_LOAD_BINARY: u32 = 0x00c3_1b20;
const VOLUME_EMITTER_GET_VIEWER_STRINGS: u32 = 0x00c3_1db0;
const MODIFIER_DESTRUCT: u32 = 0x00c3_1750;
/// `00a89af0` (`thiscall`, `(stream)`), the base call of `PostLinkObject`.
const OBJECT_BASE_STREAM_CALL: u32 = 0x00a8_9af0;
/// The cloning process's object map lookup (`00a53c30`, `thiscall` on the map, `(key,
/// &value)`, answers a byte); the map is the cloning process's first word.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPointer<T>::operator=` (`006e5cc0`, `thiscall` on the slot, `(&source)`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// The block allocation and free of the array storage (`00aa1070`, `00aa10f0`, `cdecl`,
/// one word), the Ni allocator (`00aa13e0`) and the sized delete (`00aa1460`, `(block,
/// size)`).
const NI_MALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// The global memory manager (`011f6238`), its `Allocate` (`00aa3e40`, `(size)`) and
/// `Deallocate` (`00aa4060`, `(block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The imported `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The null pointer `SetAt` compares with (`012043c0`), its guard (`012043c4`, bit 0) and
/// the `atexit` registration (`_atexit` `00ec658f` with `00fddce0`).
const NULL_POINTER: u32 = 0x0120_43c0;
const NULL_POINTER_GUARD: u32 = 0x0120_43c4;
const ATEXIT: u32 = 0x00ec_658f;
const NULL_POINTER_EXIT: u32 = 0x00fd_dce0;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, `thiscall`, `(name)`) and the name a
/// specific object carries (`012043ac`, a fixed string).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const SPECIFIC_OBJECT_NAME: u32 = 0x0120_43ac;
/// The node update `SetupArray` runs (`00a59c60`, `thiscall`, `(&update)`).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `rand` (`00ecadb8`) and the double it is divided by (`01029790`, 32767.0 in the exe).
const RAND: u32 = 0x00ec_adb8;
const RAND_SCALE: u32 = 0x0102_9790;
/// `NiPoint3` helpers: `*=` a scalar (`00439180`, `(scale)`), `-` (`00439ef0`, `(&out,
/// &other)`), `+` (`00439e90`, `(&out, &other)`), `*` a scalar (`0045bb20`, `(&out,
/// scale)`), the length (`00457990`, a float in `ST0`) and the matrix product
/// (`004b4500` on an `NiMatrix3`, `(&out, &point)`); all `thiscall`.
const POINT_SCALE_IN_PLACE: u32 = 0x0043_9180;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_LENGTH: u32 = 0x0045_7990;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `NiPoint3::ZERO` (`011f426c`) and the doubles 1.0 (`01012070`) and 0.0 (`01012060`).
const POINT_ZERO: u32 = 0x011f_426c;
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `NiAVObject` (PC offsets): parent +0x18, world rotation +0x68, world translation +0x8C,
/// world scale +0x98; a geometry's data +0xB8 (its vertices +0x20, normals +0x24, and the
/// active vertex count through vtable slot 0x90).
const PARENT: u32 = 0x18;
const WORLD_ROTATE: u32 = 0x68;
const WORLD_TRANSLATE: u32 = 0x8c;
const WORLD_SCALE: u32 = 0x98;
const GEOMETRY_DATA: u32 = 0xb8;
/// `NiNode`'s children (`m_pBase` +0xA0, `m_usSize` +0xA6).
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(label, text)` form `00a7d5e0`, the
/// viewer-string array's add (`0096a610`, `thiscall`, `(&string)`), `strlen` and the
/// formatter `00406d00` (`cdecl`, `(buffer, size, format, text)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
const STRLEN: u32 = 0x00ec_6130;
const FORMAT: u32 = 0x0040_6d00;
/// `BSPSysArrayEmitter::ms_RTTI` (its first word is the class name), `"ArrayType"`, the
/// object label (`01063e84`) and the three type formats (`"%s = AT_VERTICES"`,
/// `"%s = AT_SPECIFICOBJS"`, `"%s = AT_NODES"`).
const RTTI: u32 = 0x0120_43b0;
const ARRAY_TYPE_LABEL: u32 = 0x010c_3780;
const OBJECT_LABEL: u32 = 0x0106_3e84;
const VERTICES_FORMAT: u32 = 0x010c_37b4;
const SPECIFIC_OBJECTS_FORMAT: u32 = 0x010c_379c;
const NODES_FORMAT: u32 = 0x010c_378c;
/// `NiExtraData`'s destructor body (`00a7b300`).
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;

type Objects = Ptr<NiTArray>;

/// `InterlockedIncrement` of the count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// Releases a counted reference held in a local: when set, `InterlockedDecrement` of its
/// count and its delete slot (+4) at zero.
fn release(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &[]);
    }
}

/// The emitter's object array.
fn objects(this: Ptr) -> Objects {
    Ptr::new(this.addr().wrapping_add(OBJECTS))
}

/// The address of slot `index` of an array.
fn slot(e: &Engine, array: Objects, index: u32) -> u32 {
    e.get(array, NiTArray::m_pBase)
        .wrapping_add(index.wrapping_mul(4))
}

/// A float drawn as `rand() / 32767.0` (x87 `FILD`, `FDIV`, `FSTP dword`).
fn random_fraction(e: &mut Engine) -> u32 {
    let drawn = e.call(RAND, &[]).i32();
    let scale = e.mem.f64(RAND_SCALE);
    x87_store32(x87_div(f64::from(drawn), scale))
}

/// The index `fraction * count` truncated (x87 `FMUL` by the count as a double, `FISTP` in
/// the truncating sequence), the low word.
fn scaled_index(fraction: u32, count: f64) -> u32 {
    x87_fistp64_chop(X87Ext::from(x87_mul(x87_load32(fraction), count))) as u32
}

// Translated from 00c61b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61b10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30236, state variable) is not translated
/// Unnamed in the engine map: the `BSPSysArrayEmitter` constructor. `NiPSysVolumeEmitter`'s
/// constructor (`00c31af0`), the vtable, `eType` 2, and the object array made empty with a
/// grow-by of 1 (`00c63260`) and given the vtable `010c36b8`. Returns `this`.
pub fn fn_00c61b10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(VOLUME_EMITTER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(ARRAY_TYPE), 2);
    let array = objects(this);
    fn_00c63260(e, array, 0, 1);
    e.mem.set_u32(array.addr(), OBJECTS_VTABLE);
    this
}

// Translated from 00c61ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::ComputeVolumeInitialPositionAndVelocity` (Xbox PDB, engine map;
/// three stack words): `GetAPoint` with the second and third (the first is not read).
pub fn bs_psys_array_emitter_compute_volume_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    position: u32,
    velocity: u32,
) {
    bs_psys_array_emitter_get_a_point(e, this, position, velocity);
}

// Translated from 00c61bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61bc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30260, state variable) is not translated
/// `BSPSysArrayEmitter::CreateClone` (Xbox PDB, engine map): allocates 0x6C bytes,
/// constructs an emitter in it when that succeeds, copies the members into it (`00c61c50`;
/// into address 0 when the allocation failed) and returns it.
pub fn bs_psys_array_emitter_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![EMITTER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c61b10(e, Ptr::new(block))
    };
    fn_00c61c50(e, this, clone, cloning);
    clone
}

// Translated from 00c61c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(destination, cloning)`, the member copy):
/// `NiPSysVolumeEmitter`'s member copy (`00c31b10`), then, with objects in the array, the
/// destination's array sized to their number (`00c628c0`) and each of them stored at its
/// index (`00c62b90`, from the last).
pub fn fn_00c61c50(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(
        VOLUME_EMITTER_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let mut count = u32::from(e.get(objects(this), NiTArray::m_usSize));
    if count == 0 {
        return;
    }
    fn_00c628c0(e, objects(destination), count);
    loop {
        let current = count;
        count = count.wrapping_sub(1);
        if current == 0 {
            break;
        }
        let source = slot(e, objects(this), count);
        fn_00c62b90(e, objects(destination), count, source);
    }
}

// Translated from 00c61cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61cc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30283, state variable) is not translated
/// `BSPSysArrayEmitter::ProcessClone` (Xbox PDB, engine map): `NiPSysVolumeEmitter::ProcessClone`,
/// then, when the cloning process's map holds this emitter's clone, each object of the
/// clone's array (from the last) is replaced by its own clone (`00c62b90`, through a counted
/// local) or, without one, removed (`00c62d10`, the removed reference released). When not
/// every object had a clone, the array is compacted (`00c63030`).
pub fn bs_psys_array_emitter_process_clone(e: &mut Engine, this: Ptr, cloning: u32) {
    e.call(VOLUME_EMITTER_PROCESS_CLONE, &args![this, cloning]);
    // Locals: the emitter's clone (+0), an object's clone (+4), the counted pointer (+8)
    // and the removed pointer (+0xC).
    e.with_stack(0x10, |e, locals| {
        let clone_slot = locals.addr();
        let object_clone = clone_slot.wrapping_add(4);
        let held = clone_slot.wrapping_add(8);
        let removed = clone_slot.wrapping_add(0xc);
        let map = e.mem.u32(cloning);
        if e.call(CLONE_MAP_LOOKUP, &args![map, this, clone_slot]).u8() == 0 {
            return;
        }
        let clone = Ptr::new(e.mem.u32(clone_slot));
        let array = objects(clone);
        let mut count = u32::from(e.get(array, NiTArray::m_usSize));
        let mut cloned = 0u32;
        loop {
            let current = count;
            count = count.wrapping_sub(1);
            if current == 0 {
                break;
            }
            let object = e.mem.u32(slot(e, array, count));
            let map = e.mem.u32(cloning);
            if e.call(CLONE_MAP_LOOKUP, &args![map, object, object_clone])
                .u8()
                != 0
            {
                cloned = cloned.wrapping_add(1);
                let replacement = e.mem.u32(object_clone);
                e.mem.set_u32(held, replacement);
                if replacement != 0 {
                    add_ref(e, replacement);
                }
                fn_00c62b90(e, array, count, held);
                let pointer = e.mem.u32(held);
                release(e, pointer);
            } else {
                fn_00c62d10(e, array, removed, count);
                let pointer = e.mem.u32(removed);
                release(e, pointer);
            }
        }
        if cloned != u32::from(e.get(array, NiTArray::m_usSize)) {
            fn_00c63030(e, array);
        }
    });
}

// Translated from 00c61e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c61e70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f302b0, state variable) is not translated
/// Unnamed in the engine map (the `BSPSysArrayEmitter` loader `BSStream::InitLoaders`
/// registers, no arguments): allocates 0x6C bytes and constructs an emitter in it when that
/// succeeds. Returns it, or 0.
pub fn fn_00c61e70(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![EMITTER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c61b10(e, Ptr::new(block))
}

// Translated from 00c61ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::LoadBinary` (Xbox PDB, engine map): `NiPSysVolumeEmitter::LoadBinary`.
pub fn bs_psys_array_emitter_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(VOLUME_EMITTER_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00c61f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::PostLinkObject` (Xbox PDB, engine map): `SetupArray`, then
/// `00a89af0` with the stream.
pub fn bs_psys_array_emitter_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    bs_psys_array_emitter_setup_array(e, this);
    e.call(OBJECT_BASE_STREAM_CALL, &args![this, stream]);
}

// Translated from 00c61fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::GetViewerStrings` (Xbox PDB, engine map):
/// `NiPSysVolumeEmitter::GetViewerStrings`, the class name, `"ArrayType"` with the type
/// (`GetViewerString`), then, from the last, each set object's name (+8) with the label at
/// `01063e84`, each added to the list.
pub fn bs_psys_array_emitter_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VOLUME_EMITTER_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(0xc, |e, slots| {
        let first = slots.addr().wrapping_add(8);
        let name = e.global::<u32>(RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(first, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, first]);
        let kind = e.mem.u32(this.addr().wrapping_add(ARRAY_TYPE));
        let text = bs_psys_array_emitter_get_viewer_string(e, this, ARRAY_TYPE_LABEL, kind);
        let second = slots.addr().wrapping_add(4);
        e.mem.set_u32(second, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, second]);
        let mut count = u32::from(e.get(objects(this), NiTArray::m_usSize));
        loop {
            let current = count;
            count = count.wrapping_sub(1);
            if current == 0 {
                break;
            }
            let object = e.mem.u32(slot(e, objects(this), count));
            if object != 0 {
                let object_name = e.mem.u32(object.wrapping_add(8));
                let text = e
                    .call(NI_GET_VIEWER_STRING_TEXT, &args![OBJECT_LABEL, object_name])
                    .u32();
                e.mem.set_u32(slots.addr(), text);
                e.call(VIEWER_STRINGS_ADD, &args![strings, slots]);
            }
        }
    });
}

// Translated from 00c62090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::GetViewerString` (Xbox PDB, engine map; `(label, type)`): a
/// memory-manager block of the label's length plus 0x10, filled (`00406d00`) with
/// `"%s = AT_VERTICES"`, `"%s = AT_SPECIFICOBJS"` or `"%s = AT_NODES"` for types 0, 1
/// and 2 (left as allocated for any other). Returns the block.
pub fn bs_psys_array_emitter_get_viewer_string(
    e: &mut Engine,
    _this: Ptr,
    label: u32,
    kind: u32,
) -> u32 {
    let size = e.call(STRLEN, &args![label]).u32().wrapping_add(0x10);
    let buffer = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let format = match kind {
        0 => Some(VERTICES_FORMAT),
        1 => Some(SPECIFIC_OBJECTS_FORMAT),
        2 => Some(NODES_FORMAT),
        _ => None,
    };
    if let Some(format) = format {
        e.call(FORMAT, &args![buffer, size, format, label]);
    }
    buffer
}

// Translated from 00c62140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::SetupArray` (Xbox PDB, engine map): empties the object array
/// (`RemoveAll`); when the emitter object is a node (its vtable slot 0xC), updates it
/// (`00a59c60` with a local update record: time 0.0 and five zero bytes) and gives each of
/// its children, from the last, to `SetObjectPointers`.
pub fn bs_psys_array_emitter_setup_array(e: &mut Engine, this: Ptr) {
    ni_t_array_remove_all(e, objects(this));
    let emitter_object = e.mem.u32(this.addr().wrapping_add(EMITTER_OBJECT));
    let node = if emitter_object != 0 {
        e.vcall(emitter_object, 0xc, &[]).u32()
    } else {
        0
    };
    if node == 0 {
        return;
    }
    e.with_stack(0xc, |e, update| {
        e.mem.set_u32(update.addr(), 0);
        for k in 4..9 {
            e.mem.set_u8(update.addr().wrapping_add(k), 0);
        }
        e.call(NODE_UPDATE, &args![node, update]);
    });
    let mut count = u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE)));
    loop {
        let current = count;
        count = count.wrapping_sub(1);
        if current == 0 {
            break;
        }
        let children = e.mem.u32(node.wrapping_add(CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(count.wrapping_mul(4)));
        bs_psys_array_emitter_set_object_pointers(e, this, child);
    }
}

// Translated from 00c62200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c62200 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f302d3, state variable) is not translated
/// `BSPSysArrayEmitter::SetObjectPointers` (Xbox PDB, engine map; `(object)`): nothing for no
/// object or the emitter object itself. Otherwise the object is added to the array
/// (`00c62cc0`, through a counted local) when it fits the type: type 0 a geometry (vtable
/// slot 0x18), type 1 an object with the extra data named at `012043ac`
/// (`NiObjectNET::GetExtraData`), any other a node (vtable slot 0xC). A node's children are
/// then walked the same way, from the last.
pub fn bs_psys_array_emitter_set_object_pointers(e: &mut Engine, this: Ptr, object: u32) {
    if object == 0 {
        return;
    }
    let node = e.vcall(object, 0xc, &[]).u32();
    if object == e.mem.u32(this.addr().wrapping_add(EMITTER_OBJECT)) {
        return;
    }
    let keep = match e.mem.u32(this.addr().wrapping_add(ARRAY_TYPE)) {
        0 => e.vcall(object, 0x18, &[]).u32() != 0,
        1 => {
            let name = e.global::<u32>(SPECIFIC_OBJECT_NAME);
            e.call(GET_EXTRA_DATA, &args![object, name]).u32() != 0
        }
        _ => node != 0,
    };
    if keep {
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), object);
            add_ref(e, object);
            fn_00c62cc0(e, objects(this), held.addr());
            let pointer = e.mem.u32(held.addr());
            release(e, pointer);
        });
    }
    if node == 0 {
        return;
    }
    let mut count = u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE)));
    loop {
        let current = count;
        count = count.wrapping_sub(1);
        if current == 0 {
            break;
        }
        let children = e.mem.u32(node.wrapping_add(CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(count.wrapping_mul(4)));
        bs_psys_array_emitter_set_object_pointers(e, this, child);
    }
}

// Translated from 00c62390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::GetAPoint` (Xbox PDB, engine map; `(position, velocity)`): with
/// objects in the array, one is drawn (`rand() / 32767.0` times the count, truncated; the
/// count itself becomes the last index), else none; type 0 takes the point from a
/// geometry's vertices (`00c625f0`), any other type from the object's position
/// (`00c62470`).
pub fn bs_psys_array_emitter_get_a_point(e: &mut Engine, this: Ptr, position: u32, velocity: u32) {
    let count = u32::from(e.get(objects(this), NiTArray::m_usSize));
    let object = if count != 0 {
        let fraction = random_fraction(e);
        let mut index = scaled_index(fraction, f64::from(count));
        if index == count {
            index = index.wrapping_sub(1);
        }
        e.mem.u32(slot(e, objects(this), index))
    } else {
        0
    };
    if e.mem.u32(this.addr().wrapping_add(ARRAY_TYPE)) == 0 {
        fn_00c625f0(e, this, object, position, velocity);
    } else {
        fn_00c62470(e, this, object, position, velocity);
    }
}

/// Copies three words (an `NiPoint3`).
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for k in 0..3 {
        let word = e.mem.u32(from.wrapping_add(4 * k));
        e.mem.set_u32(to.wrapping_add(4 * k), word);
    }
}

// Translated from 00c62470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(object, position, velocity)`, the object
/// point of `GetAPoint`): no object gives `NiPoint3::ZERO`. For type 1 or an object without
/// a parent, the position is the object's world translation and, for type 1, the velocity
/// is rotated by the object's world rotation. Otherwise the position is a random point on
/// the segment from the parent's translation to the object's (`parent + (object - parent) *
/// rand() / 32767.0`), divided by the target's world scale (+0x98 of `m_pkTarget`) when
/// there is a target whose scale is neither 1.0 nor 0.0.
pub fn fn_00c62470(e: &mut Engine, this: Ptr, object: u32, position: u32, velocity: u32) {
    if object == 0 {
        copy_point(e, position, POINT_ZERO);
        return;
    }
    let parent = e.mem.u32(object.wrapping_add(PARENT));
    let kind = e.mem.u32(this.addr().wrapping_add(ARRAY_TYPE));
    if kind == 1 || parent == 0 {
        copy_point(e, position, object.wrapping_add(WORLD_TRANSLATE));
        if e.mem.u32(this.addr().wrapping_add(ARRAY_TYPE)) == 1 {
            e.with_stack(0xc, |e, rotated| {
                let result = e
                    .call(
                        MATRIX_TIMES_POINT,
                        &args![object.wrapping_add(WORLD_ROTATE), rotated, velocity],
                    )
                    .u32();
                copy_point(e, velocity, result);
            });
        }
        return;
    }
    e.with_stack(0x24, |e, points| {
        let difference = points.addr();
        let scaled = difference.wrapping_add(0xc);
        let sum = difference.wrapping_add(0x18);
        let result = e
            .call(
                POINT_SUBTRACT,
                &args![
                    object.wrapping_add(WORLD_TRANSLATE),
                    difference,
                    parent.wrapping_add(WORLD_TRANSLATE)
                ],
            )
            .u32();
        copy_point(e, position, result);
        let fraction = x87_quiet_bits(random_fraction(e));
        let product = e
            .call(POINT_SCALE, &args![position, scaled, fraction])
            .u32();
        let result = e
            .call(
                POINT_ADD,
                &args![parent.wrapping_add(WORLD_TRANSLATE), sum, product],
            )
            .u32();
        copy_point(e, position, result);
    });
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    if target == 0 {
        return;
    }
    let scale = x87_store32(x87_load32(e.mem.u32(target.wrapping_add(WORLD_SCALE))));
    let value = x87_load32(scale);
    // FCOMP; TEST AH,0x44; JNP: only an ordered equal skips.
    if value == e.mem.f64(ONE_DOUBLE) || value == e.mem.f64(ZERO_DOUBLE) {
        return;
    }
    let inverse = x87_store32(x87_div(1.0, value));
    e.call(
        POINT_SCALE_IN_PLACE,
        &args![position, x87_quiet_bits(inverse)],
    );
}

// Translated from 00c625f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(object, position, velocity)`, the vertex point
/// of `GetAPoint`): an object that is no geometry (vtable slot 0x18) gives
/// `NiPoint3::ZERO`. Otherwise a vertex is drawn from the geometry data's active vertices
/// (its vtable slot 0x90, 16 bits; `rand() / 32767.0` times the count, truncated, the count
/// itself becoming the last index): the position is that vertex scaled by the object's
/// world scale (`00439180`), and with normals, the velocity becomes the vertex's normal
/// rotated by the world rotation times the velocity's length (`00457990`).
pub fn fn_00c625f0(e: &mut Engine, _this: Ptr, object: u32, position: u32, velocity: u32) {
    let geometry = if object != 0 {
        e.vcall(object, 0x18, &[]).u32()
    } else {
        0
    };
    if geometry == 0 {
        copy_point(e, position, POINT_ZERO);
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(GEOMETRY_DATA));
    let count = e.vcall(data, 0x90, &[]).u16();
    let fraction = random_fraction(e);
    let mut index = scaled_index(fraction, f64::from(count));
    if index == u32::from(count) {
        index = index.wrapping_sub(1);
    }
    let data = e.mem.u32(geometry.wrapping_add(GEOMETRY_DATA));
    let vertices = e.mem.u32(data.wrapping_add(0x20));
    copy_point(e, position, vertices.wrapping_add(index.wrapping_mul(0xc)));
    let scale = x87_quiet_bits(x87_quiet_bits(e.mem.u32(object.wrapping_add(WORLD_SCALE))));
    e.call(POINT_SCALE_IN_PLACE, &args![position, scale]);
    let data = e.mem.u32(geometry.wrapping_add(GEOMETRY_DATA));
    if e.mem.u32(data.wrapping_add(0x24)) == 0 {
        return;
    }
    let normals = e.mem.u32(data.wrapping_add(0x24));
    e.with_stack(0x24, |e, points| {
        let normal = points.addr();
        let rotated = normal.wrapping_add(0xc);
        let scaled = normal.wrapping_add(0x18);
        copy_point(e, normal, normals.wrapping_add(index.wrapping_mul(0xc)));
        let result = e
            .call(
                MATRIX_TIMES_POINT,
                &args![object.wrapping_add(WORLD_ROTATE), rotated, normal],
            )
            .u32();
        copy_point(e, normal, result);
        let length = e.call(POINT_LENGTH, &args![velocity]).f64();
        let result = e
            .call(POINT_SCALE, &args![normal, scaled, x87_store32(length)])
            .u32();
        copy_point(e, velocity, result);
    });
}

// Translated from 00c627e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::SetEmitterObj` (Xbox PDB, engine map): stores the emitter object
/// and runs `SetupArray`.
pub fn bs_psys_array_emitter_set_emitter_obj(e: &mut Engine, this: Ptr, object: u32) {
    e.mem
        .set_u32(this.addr().wrapping_add(EMITTER_OBJECT), object);
    bs_psys_array_emitter_setup_array(e, this);
}

// Translated from 00c62800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c62800 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f302f8, state variable) is not translated
/// `BSPSysArrayEmitter::scalar deleting destructor` (Xbox PDB, engine map): the array's
/// `NiTPrimitiveArray` vtable and its storage freed (`00aa10f0`), then
/// `NiPSysModifier::~NiPSysModifier`, and the sized delete of 0x6C bytes with bit 0 of
/// `flags`. Returns `this`.
pub fn bs_psys_array_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let array = objects(this);
    e.mem.set_u32(array.addr(), PRIMITIVE_ARRAY_VTABLE);
    let base = e.get(array, NiTArray::m_pBase);
    e.call(NI_FREE, &args![base]);
    e.call(MODIFIER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, EMITTER_SIZE]);
    }
    this
}

// Translated from 00c62890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTPrimitiveArray<NiPointer<NiAVObject>>` destructor
/// body. Its vtable, then its storage freed (`00aa10f0`).
pub fn fn_00c62890(e: &mut Engine, this: Objects) {
    e.mem.set_u32(this.addr(), PRIMITIVE_ARRAY_VTABLE);
    let base = e.get(this, NiTArray::m_pBase);
    e.call(NI_FREE, &args![base]);
}

/// The slot holds a pointer other than null (the code compares it with a null local
/// pointer whose construction and release do nothing).
fn slot_set(e: &Engine, array: Objects, index: u32) -> bool {
    e.mem.u32(slot(e, array, index)) != 0
}

/// `NiPointer::operator=` of the slot with a null local pointer.
fn clear_slot(e: &mut Engine, array: Objects, index: u32) {
    e.with_stack(4, |e, null| {
        e.mem.set_u32(null.addr(), 0);
        let target = slot(e, array, index);
        e.call(NI_POINTER_ASSIGN, &args![target, null]);
        let pointer = e.mem.u32(null.addr());
        release(e, pointer);
    });
}

// Translated from 00c628c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c628c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30330, state variable) is not translated
/// Unnamed in the engine map (`thiscall` on the object array, `(new_max)`, its `SetSize`):
/// nothing when the capacity is already `new_max`. Below the size, the set slots from
/// `new_max` on are cleared (`NiPointer::operator=` with null, the element count lowered)
/// and the size cut. Then the capacity becomes `new_max` (16 bits) and, for a non-zero
/// one, new storage (`00aa1070`) takes the old slots by assignment and the rest are
/// assigned null; else the storage is null. The old storage is freed (`00aa10f0`).
pub fn fn_00c628c0(e: &mut Engine, this: Objects, new_max: u32) {
    if new_max == u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        return;
    }
    if new_max < u32::from(e.get(this, NiTArray::m_usSize)) {
        let mut index = new_max as u16;
        while index < e.get(this, NiTArray::m_usSize) {
            if slot_set(e, this, u32::from(index)) {
                clear_slot(e, this, u32::from(index));
                let present = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, present.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, new_max as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, new_max as u16);
    if new_max == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let storage = e.call(NI_MALLOC, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, storage);
        let mut index = 0u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let source = old.wrapping_add(u32::from(index) * 4);
            let target = slot(e, this, u32::from(index));
            e.call(NI_POINTER_ASSIGN, &args![target, source]);
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        while index < e.get(this, NiTArray::m_usMaxSize) {
            clear_slot(e, this, u32::from(index));
            index = index.wrapping_add(1);
        }
    }
    e.call(NI_FREE, &args![old]);
}

// Translated from 00c62b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the object array, `(index, &pointer)`, its
/// `SetAt`): on first use the null pointer at `012043c0` is set up (guard bit at
/// `012043c4`, its `atexit` destructor `00fddce0` registered). At or beyond the size, the
/// size becomes `index + 1` and a non-null pointer counts an element; inside it, the element
/// count follows the slot changing between null and set. Then `NiPointer::operator=` stores
/// the pointer.
pub fn fn_00c62b90(e: &mut Engine, this: Objects, index: u32, pointer: u32) {
    let guard = e.global::<u32>(NULL_POINTER_GUARD);
    if guard & 1 == 0 {
        e.set_global::<u32>(NULL_POINTER_GUARD, guard | 1);
        e.set_global::<u32>(NULL_POINTER, 0);
        let null = e.global::<u32>(NULL_POINTER);
        if null != 0 {
            add_ref(e, null);
        }
        e.call(ATEXIT, &args![NULL_POINTER_EXIT]);
    }
    let wanted = e.mem.u32(pointer);
    let null = e.global::<u32>(NULL_POINTER);
    if index >= u32::from(e.get(this, NiTArray::m_usSize)) {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if wanted != null {
            let present = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, present.wrapping_add(1));
        }
    } else if wanted != null {
        if e.mem.u32(slot(e, this, index)) == e.global::<u32>(NULL_POINTER) {
            let present = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, present.wrapping_add(1));
        }
    } else if e.mem.u32(slot(e, this, index)) != e.global::<u32>(NULL_POINTER) {
        let present = e.get(this, NiTArray::m_usESize);
        e.set(this, NiTArray::m_usESize, present.wrapping_sub(1));
    }
    let target = slot(e, this, index);
    e.call(NI_POINTER_ASSIGN, &args![target, pointer]);
}

// Translated from 00c62cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the object array, `(&pointer)`, its `Add`): at
/// full capacity the array grows by its grow-by (`00c628c0`); the pointer is stored at the
/// old size (`00c62b90`), which is returned.
pub fn fn_00c62cc0(e: &mut Engine, this: Objects, pointer: u32) -> u32 {
    let index = u32::from(e.get(this, NiTArray::m_usSize));
    if index >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        fn_00c628c0(e, this, grow_by.wrapping_add(index));
    }
    fn_00c62b90(e, this, index, pointer);
    index
}

// Translated from 00c62d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c62d10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30379, state variable) is not translated
/// Unnamed in the engine map (`thiscall` on the object array, `(&out, index)`, its
/// `RemoveAt`, the pointer returned by value): beyond the size `out` is null. Otherwise the
/// slot's pointer (counted in a local) is taken out: the slot is assigned null, the element
/// count lowered when it was set, the size cut when it was the last slot, and `out` gets the
/// pointer (with its own reference; the local's is released). Returns `out`.
pub fn fn_00c62d10(e: &mut Engine, this: Objects, out: u32, index: u32) -> u32 {
    if index >= u32::from(e.get(this, NiTArray::m_usSize)) {
        e.mem.set_u32(out, 0);
        let pointer = e.mem.u32(out);
        if pointer != 0 {
            add_ref(e, pointer);
        }
        return out;
    }
    let held = e.mem.u32(slot(e, this, index));
    if held != 0 {
        add_ref(e, held);
    }
    clear_slot(e, this, index);
    if held != 0 {
        let present = e.get(this, NiTArray::m_usESize);
        e.set(this, NiTArray::m_usESize, present.wrapping_sub(1));
    }
    let last = u32::from(e.get(this, NiTArray::m_usSize).wrapping_sub(1));
    if index == last {
        let size = e.get(this, NiTArray::m_usSize);
        e.set(this, NiTArray::m_usSize, size.wrapping_sub(1));
    }
    e.mem.set_u32(out, held);
    if held != 0 {
        add_ref(e, held);
    }
    release(e, held);
    out
}

// Translated from 00c62f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c62f40 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f039f8, state variable) is not translated
/// `NiTArray<NiPointer<NiAVObject>,NiTMallocInterface<NiPointer<NiAVObject>>>::RemoveAll`
/// (Xbox PDB, engine map): every slot up to the size assigned null, then the size and the
/// element count zeroed.
pub fn ni_t_array_remove_all(e: &mut Engine, this: Objects) {
    let mut index = 0u16;
    while index < e.get(this, NiTArray::m_usSize) {
        clear_slot(e, this, u32::from(index));
        index = index.wrapping_add(1);
    }
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
}

// Translated from 00c63030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the object array, its `Compact`): nothing when
/// every slot is set. Otherwise the set slots are moved to the front (assigned when they
/// differ), the size and the capacity become the element count, and new storage
/// (`00aa1070`; null for a count of 0) takes the slots by assignment; the old storage is
/// freed (`00aa10f0`) without releasing its pointers.
pub fn fn_00c63030(e: &mut Engine, this: Objects) {
    if e.get(this, NiTArray::m_usESize) == e.get(this, NiTArray::m_usSize) {
        return;
    }
    if e.get(this, NiTArray::m_usESize) != 0 {
        let mut index = 0u16;
        let mut next = 0u16;
        while index < e.get(this, NiTArray::m_usSize) {
            if slot_set(e, this, u32::from(index)) {
                let source = slot(e, this, u32::from(index));
                let target = slot(e, this, u32::from(next));
                if e.mem.u32(target) != e.mem.u32(source) {
                    e.call(NI_POINTER_ASSIGN, &args![target, source]);
                }
                next = next.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
    }
    let old = e.get(this, NiTArray::m_pBase);
    let present = e.get(this, NiTArray::m_usESize);
    e.set(this, NiTArray::m_usSize, present);
    e.set(this, NiTArray::m_usMaxSize, present);
    let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if capacity == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let storage = e.call(NI_MALLOC, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, storage);
        let mut index = 0u16;
        while index < e.get(this, NiTArray::m_usSize) {
            let source = old.wrapping_add(u32::from(index) * 4);
            let target = slot(e, this, u32::from(index));
            e.call(NI_POINTER_ASSIGN, &args![target, source]);
            index = index.wrapping_add(1);
        }
    }
    e.call(NI_FREE, &args![old]);
}

// Translated from 00c63210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiPointer<NiAVObject>>::scalar deleting destructor` (Xbox PDB, engine
/// map): the vtable, the storage freed (`00aa10f0`), and with bit 0 of `flags` the array
/// itself given back to the memory manager (`MemoryManager::Deallocate`). Returns `this`.
pub fn ni_t_primitive_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Objects,
    flags: u32,
) -> Objects {
    fn_00c62890(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c63260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTPrimitiveArray<NiPointer<NiAVObject>>` constructor
/// (`thiscall`, `(max, grow_by)`, 16 bits each): the vtable, the capacity and grow-by, an
/// empty size and element count, and storage for the capacity (`00aa1070`) or null.
/// Returns `this`.
pub fn fn_00c63260(e: &mut Engine, this: Objects, max: u16, grow_by: u16) -> Objects {
    e.mem.set_u32(this.addr(), PRIMITIVE_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if capacity == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let storage = e.call(NI_MALLOC, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, storage);
    }
    this
}

// Translated from 00c632f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor of a 0xC-byte `NiExtraData`
/// class of this unit (vtable word `010c361c`; its `GetRTTI` answers `012043b8`):
/// `NiExtraData`'s destructor body and the sized delete of 0xC bytes with bit 0 of `flags`.
/// Returns `this`.
pub fn fn_00c632f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_1b10, fn_00c61b10(Ptr) -> Ptr),
        entry!(
            0x00c6_1ba0,
            bs_psys_array_emitter_compute_volume_initial_position_and_velocity(Ptr, u32, u32, u32)
        ),
        entry!(0x00c6_1bc0, bs_psys_array_emitter_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c6_1c50, fn_00c61c50(Ptr, Ptr, u32)),
        entry!(0x00c6_1cc0, bs_psys_array_emitter_process_clone(Ptr, u32)),
        entry!(0x00c6_1e70, fn_00c61e70() -> Ptr),
        entry!(0x00c6_1ef0, bs_psys_array_emitter_load_binary(Ptr, Ptr)),
        entry!(
            0x00c6_1f30,
            bs_psys_array_emitter_post_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c6_1fc0,
            bs_psys_array_emitter_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c6_2090,
            bs_psys_array_emitter_get_viewer_string(Ptr, u32, u32) -> u32
        ),
        entry!(0x00c6_2140, bs_psys_array_emitter_setup_array(Ptr)),
        entry!(
            0x00c6_2200,
            bs_psys_array_emitter_set_object_pointers(Ptr, u32)
        ),
        entry!(
            0x00c6_2390,
            bs_psys_array_emitter_get_a_point(Ptr, u32, u32)
        ),
        entry!(0x00c6_2470, fn_00c62470(Ptr, u32, u32, u32)),
        entry!(0x00c6_25f0, fn_00c625f0(Ptr, u32, u32, u32)),
        entry!(0x00c6_27e0, bs_psys_array_emitter_set_emitter_obj(Ptr, u32)),
        entry!(
            0x00c6_2800,
            bs_psys_array_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c6_2890, fn_00c62890(Ptr<NiTArray>)),
        entry!(0x00c6_28c0, fn_00c628c0(Ptr<NiTArray>, u32)),
        entry!(0x00c6_2b90, fn_00c62b90(Ptr<NiTArray>, u32, u32)),
        entry!(0x00c6_2cc0, fn_00c62cc0(Ptr<NiTArray>, u32) -> u32),
        entry!(0x00c6_2d10, fn_00c62d10(Ptr<NiTArray>, u32, u32) -> u32),
        entry!(0x00c6_2f40, ni_t_array_remove_all(Ptr<NiTArray>)),
        entry!(0x00c6_3030, fn_00c63030(Ptr<NiTArray>)),
        entry!(
            0x00c6_3210,
            ni_t_primitive_array_scalar_deleting_destructor(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x00c6_3260,
            fn_00c63260(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x00c6_32f0, fn_00c632f0(Ptr, u32) -> Ptr),
        entry!(0x00c6_32e0, fn_00c632e0(Ptr) -> u32),
        entry!(0x00c6_3320, fn_00c63320(Ptr) -> u32),
        entry!(0x00c6_1f10, bs_psys_array_emitter_link_object(Ptr, Ptr)),
        entry!(0x00c6_1f60, bs_psys_array_emitter_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c6_1f80, bs_psys_array_emitter_save_binary(Ptr, Ptr)),
        entry!(0x00c6_1fa0, bs_psys_array_emitter_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_psys_array_emitter_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31B40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1f10, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31B40)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_register_streamables_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C2F4F0, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_1f60, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C2F4F0)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_save_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00C31B60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1f80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00C31B60)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_is_equal_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(IS_EQUAL_TARGET_00C31B80, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_1fa0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == IS_EQUAL_TARGET_00C31B80)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c632e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_32e0, &args![0x1000u32]).u32(), 0x0120_43b8);
        assert_eq!(e.call(0x00c6_32e0, &args![0u32]).u32(), 0x0120_43b8);
    }

    #[test]
    fn fn_00c63320_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_3320, &args![0x1000u32]).u32(), 0x0120_43b0);
        assert_eq!(e.call(0x00c6_3320, &args![0u32]).u32(), 0x0120_43b0);
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Counting, `NiPointer::operator=`, the storage allocation and the free as small
    /// working doubles (the free is logged).
    fn engine(log: &Log) -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let held = e.mem.u32(a[0]);
            let wanted = e.mem.u32(a[1]);
            if held != wanted {
                if held != 0 {
                    let count = e.mem.u32(held + 4) - 1;
                    e.mem.set_u32(held + 4, count);
                }
                e.mem.set_u32(a[0], wanted);
                if wanted != 0 {
                    let count = e.mem.u32(wanted + 4) + 1;
                    e.mem.set_u32(wanted + 4, count);
                }
            }
            a[0].into_ret()
        });
        e.register(NI_MALLOC, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        logged(&mut e, log, NI_FREE, 0);
        e.map(NULL_POINTER, 8);
        logged(&mut e, log, ATEXIT, 0);
        e
    }

    /// A counted object (its delete slot logged) with `count` references.
    fn object(e: &mut Engine, log: &Log, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        let delete = 0x0040_7000 + vtable;
        e.mem.set_u32(vtable + 4, delete);
        logged(e, log, delete, 0);
        let object = e.mem.alloc(0xc0);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// An object array with capacity `max` holding `items` (each counted once more).
    fn array(e: &mut Engine, max: u16, items: &[u32]) -> Objects {
        let array = Ptr::<NiTArray>::new(e.mem.alloc(0x10));
        let base = e.mem.alloc(u32::from(max.max(1)) * 4);
        for (i, item) in (0u32..).zip(items) {
            e.mem.set_u32(base + 4 * i, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        let present = items.iter().filter(|i| **i != 0).count() as u16;
        e.set(array, NiTArray::m_usESize, present);
        e.set(array, NiTArray::m_usGrowBy, 2);
        array
    }

    fn items(e: &Engine, array: Objects) -> Vec<u32> {
        let base = e.get(array, NiTArray::m_pBase);
        (0..u32::from(e.get(array, NiTArray::m_usSize)))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect()
    }

    fn counts(e: &Engine, array: Objects) -> (u16, u16, u16) {
        (
            e.get(array, NiTArray::m_usMaxSize),
            e.get(array, NiTArray::m_usSize),
            e.get(array, NiTArray::m_usESize),
        )
    }

    #[test]
    fn array_constructor_and_destructors() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        let array = Ptr::<NiTArray>::new(e.mem.alloc(0x10));
        assert_eq!(
            e.call(0x00c6_3260, &args![array, 3u32, 5u32]).u32(),
            array.addr()
        );
        assert_eq!(e.mem.u32(array.addr()), PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(counts(&e, array), (3, 0, 0));
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 5);
        assert_ne!(e.get(array, NiTArray::m_pBase), 0);
        e.call(0x00c6_3260, &args![array, 0u32, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        e.set(array, NiTArray::m_pBase, 0x7000);
        e.call(0x00c6_3210, &args![array, 1u32]);
        e.call(0x00c6_3210, &args![array, 0u32]);
        assert_eq!(calls(&log, NI_FREE), vec![vec![0x7000], vec![0x7000]]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array.addr()]]
        );
    }

    #[test]
    fn set_at_counts_elements_and_registers_the_null_pointer_once() {
        let log = Log::default();
        let mut e = engine(&log);
        let a = object(&mut e, &log, 1);
        let array = array(&mut e, 4, &[0, 0]);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, a);
        e.call(0x00c6_2b90, &args![array, 1u32, holder]);
        assert_eq!(counts(&e, array), (4, 2, 1));
        e.call(0x00c6_2b90, &args![array, 3u32, holder]);
        assert_eq!(counts(&e, array), (4, 4, 2));
        assert_eq!(items(&e, array), vec![0, a, 0, a]);
        assert_eq!(e.mem.u32(a + 4), 3);
        e.mem.set_u32(holder, 0);
        e.call(0x00c6_2b90, &args![array, 1u32, holder]);
        assert_eq!(counts(&e, array), (4, 4, 1));
        assert_eq!(calls(&log, ATEXIT), vec![vec![NULL_POINTER_EXIT]]);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD) & 1, 1);
    }

    #[test]
    fn add_grows_by_the_grow_by_when_full() {
        let log = Log::default();
        let mut e = engine(&log);
        let (a, b) = (object(&mut e, &log, 1), object(&mut e, &log, 1));
        let array = array(&mut e, 1, &[a]);
        let old = e.get(array, NiTArray::m_pBase);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, b);
        assert_eq!(e.call(0x00c6_2cc0, &args![array, holder]).u32(), 1);
        assert_eq!(counts(&e, array), (3, 2, 2));
        assert_eq!(items(&e, array), vec![a, b]);
        assert_eq!(calls(&log, NI_FREE), vec![vec![old]]);
    }

    #[test]
    fn set_size_cuts_releasing_the_dropped_slots() {
        let log = Log::default();
        let mut e = engine(&log);
        let (a, b, c) = (
            object(&mut e, &log, 1),
            object(&mut e, &log, 1),
            object(&mut e, &log, 1),
        );
        let array = array(&mut e, 4, &[a, 0, b, c]);
        e.call(0x00c6_28c0, &args![array, 4u32]);
        assert!(calls(&log, NI_FREE).is_empty());
        e.call(0x00c6_28c0, &args![array, 2u32]);
        assert_eq!(counts(&e, array), (2, 2, 1));
        assert_eq!(items(&e, array), vec![a, 0]);
        assert_eq!((e.mem.u32(b + 4), e.mem.u32(c + 4)), (0, 0));
        assert_eq!(e.mem.u32(a + 4), 2);
        e.call(0x00c6_28c0, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        assert_eq!(calls(&log, NI_FREE).len(), 2);
    }

    #[test]
    fn remove_at_takes_the_pointer_out() {
        let log = Log::default();
        let mut e = engine(&log);
        let (a, b) = (object(&mut e, &log, 1), object(&mut e, &log, 1));
        let array = array(&mut e, 4, &[a, b]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00c6_2d10, &args![array, out, 1u32]).u32(), out);
        assert_eq!(e.mem.u32(out), b);
        assert_eq!(counts(&e, array), (4, 1, 1));
        // The slot gave its reference up and `out` holds one.
        assert_eq!(e.mem.u32(b + 4), 1);
        e.call(0x00c6_2d10, &args![array, out, 0u32]);
        assert_eq!(counts(&e, array), (4, 0, 0));
        e.call(0x00c6_2d10, &args![array, out, 5u32]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn remove_all_and_compact() {
        let log = Log::default();
        let mut e = engine(&log);
        let (a, b) = (object(&mut e, &log, 1), object(&mut e, &log, 1));
        let array = array(&mut e, 4, &[0, a, 0, b]);
        let old = e.get(array, NiTArray::m_pBase);
        e.call(0x00c6_3030, &args![array]);
        assert_eq!(counts(&e, array), (2, 2, 2));
        assert_eq!(items(&e, array), vec![a, b]);
        assert_eq!(calls(&log, NI_FREE), vec![vec![old]]);
        // Already compact: nothing happens.
        e.call(0x00c6_3030, &args![array]);
        assert_eq!(calls(&log, NI_FREE).len(), 1);
        e.call(0x00c6_2f40, &args![array]);
        assert_eq!(counts(&e, array), (2, 0, 0));
        let base = e.get(array, NiTArray::m_pBase);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 4)), (0, 0));
    }

    fn emitter(e: &mut Engine, kind: u32, items: &[u32]) -> Ptr {
        let this = e.mem.alloc(0x6c);
        e.mem.set_u32(this + ARRAY_TYPE, kind);
        let source = array(e, 8, items);
        for k in (0..0x10).step_by(4) {
            let word = e.mem.u32(source.addr() + k);
            e.mem.set_u32(this + OBJECTS + k, word);
        }
        Ptr::new(this)
    }

    #[test]
    fn constructor_factory_clone_and_deleting_destructor() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, VOLUME_EMITTER_CONSTRUCT, 0);
        logged(&mut e, &log, VOLUME_EMITTER_COPY_MEMBERS, 0);
        logged(&mut e, &log, MODIFIER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let block = e.mem.alloc(0x6c);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00c6_1e70, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + ARRAY_TYPE), 2);
        assert_eq!(e.mem.u32(block + OBJECTS), OBJECTS_VTABLE);
        assert_eq!(e.mem.u16(block + OBJECTS + 0xe), 1);
        let a = object(&mut e, &log, 1);
        let b = object(&mut e, &log, 1);
        let this = emitter(&mut e, 0, &[a, b]);
        assert_eq!(e.call(0x00c6_1bc0, &args![this, 0x5555u32]).u32(), block);
        assert_eq!(items(&e, Ptr::new(block + OBJECTS)), vec![a, b]);
        assert_eq!(
            calls(&log, VOLUME_EMITTER_COPY_MEMBERS),
            vec![vec![this.addr(), block, 0x5555]]
        );
        let base = e.mem.u32(block + OBJECTS + 4);
        assert_eq!(e.call(0x00c6_2800, &args![block, 1u32]).u32(), block);
        assert_eq!(e.mem.u32(block + OBJECTS), PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(calls(&log, NI_FREE).last().unwrap(), &vec![base]);
        assert_eq!(calls(&log, NI_SIZED_DELETE), vec![vec![block, 0x6c]]);
        e.call(0x00c6_2890, &args![block + OBJECTS]);
        assert_eq!(calls(&log, MODIFIER_DESTRUCT), vec![vec![block]]);
    }

    #[test]
    fn process_clone_swaps_in_clones_and_drops_the_rest() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, VOLUME_EMITTER_PROCESS_CLONE, 0);
        let (a, b, a_clone) = (
            object(&mut e, &log, 1),
            object(&mut e, &log, 1),
            object(&mut e, &log, 1),
        );
        let this = emitter(&mut e, 2, &[]);
        let clone = emitter(&mut e, 2, &[a, b]);
        let (me, copy) = (this.addr(), clone.addr());
        e.register_double(CLONE_MAP_LOOKUP, move |e, x| {
            let found = if x[1] == me {
                copy
            } else if x[1] == a {
                a_clone
            } else {
                0
            };
            if found != 0 {
                e.mem.set_u32(x[2], found);
            }
            u32::from(found != 0).into_ret()
        });
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00c6_1cc0, &args![this, cloning]);
        assert_eq!(items(&e, Ptr::new(copy + OBJECTS)), vec![a_clone]);
        assert_eq!(e.mem.u32(a_clone + 4), 2);
        assert_eq!(e.mem.u32(b + 4), 0);
    }

    #[test]
    fn set_emitter_obj_collects_objects_by_type() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, NODE_UPDATE, 0);
        // Slot 0xC answers the word at +0x10 (a node answers itself), slot 0x18 the word at
        // +0x14 (a geometry answers itself).
        e.register(0x0040_800c, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0040_8018, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        let make = |e: &mut Engine, node: bool, geometry: bool| {
            let o = object(e, &log, 1);
            let vtable = e.mem.u32(o);
            e.mem.set_u32(vtable + 0xc, 0x0040_800c);
            e.mem.set_u32(vtable + 0x18, 0x0040_8018);
            if node {
                e.mem.set_u32(o + 0x10, o);
            }
            if geometry {
                e.mem.set_u32(o + 0x14, o);
            }
            o
        };
        let root = make(&mut e, true, false);
        let shape = make(&mut e, false, true);
        let inner = make(&mut e, true, false);
        let leaf = make(&mut e, false, true);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, shape);
        e.mem.set_u32(children + 4, inner);
        e.mem.set_u32(root + CHILDREN_BASE, children);
        e.mem.set_u16(root + CHILDREN_SIZE, 2);
        let grandchildren = e.mem.alloc(4);
        e.mem.set_u32(grandchildren, leaf);
        e.mem.set_u32(inner + CHILDREN_BASE, grandchildren);
        e.mem.set_u16(inner + CHILDREN_SIZE, 1);
        let this = emitter(&mut e, 0, &[]);
        e.call(0x00c6_27e0, &args![this, root]);
        assert_eq!(e.mem.u32(this.addr() + EMITTER_OBJECT), root);
        // Children from the last: inner (its leaf) before shape.
        assert_eq!(items(&e, objects(this)), vec![leaf, shape]);
        assert_eq!(calls(&log, NODE_UPDATE).len(), 1);
        // Type 2: nodes.
        e.mem.set_u32(this.addr() + ARRAY_TYPE, 2);
        e.call(0x00c6_2140, &args![this]);
        assert_eq!(items(&e, objects(this)), vec![inner]);
        // Type 1: objects with the named extra data.
        e.mem.set_u32(this.addr() + ARRAY_TYPE, 1);
        e.map(SPECIFIC_OBJECT_NAME, 4);
        e.register_double(GET_EXTRA_DATA, move |_, a| {
            u32::from(a[0] == shape).into_ret()
        });
        e.call(0x00c6_2140, &args![this]);
        assert_eq!(items(&e, objects(this)), vec![shape]);
    }

    #[test]
    fn get_a_point_draws_an_object_and_dispatches_on_the_type() {
        let log = Log::default();
        let mut e = engine(&log);
        e.map(RAND_SCALE, 8);
        e.mem.set_f64(RAND_SCALE, 32767.0);
        e.map(POINT_ZERO, 0xc);
        e.mem.set_f32(POINT_ZERO, 0.0);
        let mut draws = vec![32767i32, 0, 16383].into_iter();
        e.register_double(RAND, move |_, _| (draws.next().unwrap() as u32).into_ret());
        let a = object(&mut e, &log, 1);
        let b = object(&mut e, &log, 1);
        e.mem.set_f32(a + WORLD_TRANSLATE, 1.0);
        e.mem.set_f32(b + WORLD_TRANSLATE, 2.0);
        let this = emitter(&mut e, 1, &[a, b]);
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        e.register(MATRIX_TIMES_POINT, |e, a| {
            e.mem.set_f32(a[1], 9.0);
            a[1].into_ret()
        });
        // A draw of 1.0 gives the count itself, then the last index.
        e.call(0x00c6_1ba0, &args![this, 0u32, position, velocity]);
        assert_eq!(e.mem.f32(position), 2.0);
        assert_eq!(e.mem.f32(velocity), 9.0);
        e.call(0x00c6_2390, &args![this, position, velocity]);
        assert_eq!(e.mem.f32(position), 1.0);
        // 16383 / 32767 * 2 truncates to 0.
        e.call(0x00c6_2390, &args![this, position, velocity]);
        assert_eq!(e.mem.f32(position), 1.0);
        // No objects: no draw, `NiPoint3::ZERO`.
        let empty = emitter(&mut e, 1, &[]);
        e.mem.set_f32(position, 5.0);
        e.call(0x00c6_2390, &args![empty, position, velocity]);
        assert_eq!(e.mem.f32(position), 0.0);
    }

    #[test]
    fn object_point_between_parent_and_object_and_target_scale() {
        let log = Log::default();
        let mut e = engine(&log);
        e.map(RAND_SCALE, 8);
        e.mem.set_f64(RAND_SCALE, 32767.0);
        e.map(ONE_DOUBLE, 8);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.map(ZERO_DOUBLE, 8);
        e.register(RAND, |_, _| 32767u32.into_ret());
        // Point helpers on the x coordinate only.
        e.register(POINT_SUBTRACT, |e, a| {
            let v = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            e.mem.set_f32(a[1], v);
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let v = e.mem.f32(a[0]) * f32::from_bits(a[2]);
            e.mem.set_f32(a[1], v);
            a[1].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            let v = e.mem.f32(a[0]) + e.mem.f32(a[2]);
            e.mem.set_f32(a[1], v);
            a[1].into_ret()
        });
        logged(&mut e, &log, POINT_SCALE_IN_PLACE, 0);
        let parent = e.mem.alloc(0xc0);
        e.mem.set_f32(parent + WORLD_TRANSLATE, 1.0);
        let child = e.mem.alloc(0xc0);
        e.mem.set_u32(child + PARENT, parent);
        e.mem.set_f32(child + WORLD_TRANSLATE, 5.0);
        let this = emitter(&mut e, 2, &[]);
        let position = e.mem.alloc(0xc);
        e.call(0x00c6_2470, &args![this, child, position, 0u32]);
        assert_eq!(e.mem.f32(position), 5.0);
        assert!(calls(&log, POINT_SCALE_IN_PLACE).is_empty());
        let target = e.mem.alloc(0xa0);
        e.mem.set_u32(this.addr() + TARGET, target);
        e.mem.set_f32(target + WORLD_SCALE, 1.0);
        e.call(0x00c6_2470, &args![this, child, position, 0u32]);
        assert!(calls(&log, POINT_SCALE_IN_PLACE).is_empty());
        e.mem.set_f32(target + WORLD_SCALE, 4.0);
        e.call(0x00c6_2470, &args![this, child, position, 0u32]);
        assert_eq!(
            calls(&log, POINT_SCALE_IN_PLACE),
            vec![vec![position, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn vertex_point_scales_the_vertex_and_turns_the_normal() {
        let log = Log::default();
        let mut e = engine(&log);
        e.map(RAND_SCALE, 8);
        e.mem.set_f64(RAND_SCALE, 32767.0);
        e.map(POINT_ZERO, 0xc);
        e.register(RAND, |_, _| 32767u32.into_ret());
        e.register(0x0040_8018, |_, a| a[0].into_ret());
        e.register(0x0040_8090, |_, _| 2u32.into_ret());
        logged(&mut e, &log, POINT_SCALE_IN_PLACE, 0);
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let v = e.mem.f32(a[2]) * 10.0;
            e.mem.set_f32(a[1], v);
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |_, _| 3.0f64.into_ret());
        e.register(POINT_SCALE, |e, a| {
            let v = e.mem.f32(a[0]) * f32::from_bits(a[2]);
            e.mem.set_f32(a[1], v);
            a[1].into_ret()
        });
        let shape = object(&mut e, &log, 1);
        let vtable = e.mem.u32(shape);
        e.mem.set_u32(vtable + 0x18, 0x0040_8018);
        e.mem.set_f32(shape + WORLD_SCALE, 2.0);
        let data = e.mem.alloc(0x30);
        let data_vtable = e.mem.alloc(0x94);
        e.mem.set_u32(data_vtable + 0x90, 0x0040_8090);
        e.mem.set_u32(data, data_vtable);
        let vertices = e.mem.alloc(0x18);
        e.mem.set_f32(vertices + 0xc, 7.0);
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(shape + GEOMETRY_DATA, data);
        let this = emitter(&mut e, 0, &[shape]);
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        e.call(0x00c6_2390, &args![this, position, velocity]);
        // The draw of 1.0 picks vertex 2, which becomes the last one (1).
        assert_eq!(e.mem.f32(position), 7.0);
        assert_eq!(
            calls(&log, POINT_SCALE_IN_PLACE),
            vec![vec![position, 2.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(velocity), 0.0);
        let normals = e.mem.alloc(0x18);
        e.mem.set_f32(normals + 0xc, 0.5);
        e.mem.set_u32(data + 0x24, normals);
        e.call(0x00c6_2390, &args![this, position, velocity]);
        assert_eq!(e.mem.f32(velocity), 15.0);
        // No geometry: `NiPoint3::ZERO`.
        e.call(0x00c6_25f0, &args![this, 0u32, position, velocity]);
        assert_eq!(e.mem.f32(position), 0.0);
    }

    #[test]
    fn viewer_strings_and_type_string() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, VOLUME_EMITTER_GET_VIEWER_STRINGS, 0);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0101_0000);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, a| (a[0] + a[1]).into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        logged(&mut e, &log, FORMAT, 0);
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        e.map(ARRAY_TYPE_LABEL, 0x10);
        e.mem.set_cstr(ARRAY_TYPE_LABEL, b"ArrayType");
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, 0x10);
        let this = emitter(&mut e, 1, &[first, 0]);
        e.call(0x00c6_1fc0, &args![this, 0x6000u32]);
        let added = calls(&log, 0);
        assert_eq!(added.len(), 3);
        assert_eq!(added[0], vec![0x6000, 0x0101_0001]);
        assert_eq!(added[2], vec![0x6000, OBJECT_LABEL + 0x10]);
        let format = calls(&log, FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(
            format[0][1..],
            [9 + 0x10, SPECIFIC_OBJECTS_FORMAT, ARRAY_TYPE_LABEL]
        );
        assert_eq!(added[1][1], format[0][0]);
        for (kind, expected) in [
            (0u32, Some(VERTICES_FORMAT)),
            (2, Some(NODES_FORMAT)),
            (3, None),
        ] {
            log.borrow_mut().clear();
            e.call(0x00c6_2090, &args![this, ARRAY_TYPE_LABEL, kind]);
            assert_eq!(calls(&log, FORMAT).first().map(|c| c[2]), expected);
        }
    }

    #[test]
    fn load_post_link_and_the_extra_data_destructor() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, VOLUME_EMITTER_LOAD_BINARY, 0);
        logged(&mut e, &log, OBJECT_BASE_STREAM_CALL, 0);
        logged(&mut e, &log, EXTRA_DATA_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = emitter(&mut e, 2, &[]);
        e.call(0x00c6_1ef0, &args![this, 0x6000u32]);
        e.call(0x00c6_1f30, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, VOLUME_EMITTER_LOAD_BINARY),
            vec![vec![this.addr(), 0x6000]]
        );
        assert_eq!(
            calls(&log, OBJECT_BASE_STREAM_CALL),
            vec![vec![this.addr(), 0x6000]]
        );
        assert_eq!(e.call(0x00c6_32f0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00c6_32f0, &args![0x5000u32, 0u32]);
        assert_eq!(calls(&log, EXTRA_DATA_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, NI_SIZED_DELETE), vec![vec![0x5000, 0xc]]);
    }
}
