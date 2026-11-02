//! `BSHavok/bhkboxshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBoxShape` (Xbox PDB) is a 0x14-byte `bhkConvexShape`: `+0x8` is the Havok box shape
//! (`hkpBoxShape`, 0x30 bytes, its half extents vector at `+0x20`) and `+0xC` the cinfo that
//! `CreateCinfo` builds on demand (a 0x30-byte block aligned to 16 bytes, its vector at
//! `+0x10` the half extents).
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! or a `/GS` cookie check (`00ec408c`), neither of which is modeled. The constructor `00c9fde0`
//! stores the vtables of the base classes one after the other (the inlined base constructors);
//! only the last one survives, and the intermediate stores and counters are written here too.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `MemoryManager` allocation (`00aa3e40`, `this` = the manager at `011f6238`, `(size)`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `bhkRefObject` constructor (`00c87c30`: vtable `010c5b54`, clears `+8`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill`, `CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConvexShape` functions (Xbox PDB): the destructor, `CopyCinfo` (`this, cinfo`),
/// `LoadBinary` and `SaveBinary` (`this, stream`) and `GetViewerStrings` (`this, strings`).
const CONVEX_DESTRUCTOR: u32 = 0x00c9_fb00;
const CONVEX_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape::CopyMembers` (Xbox PDB, `this`, `(clone, process)`).
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
/// `hkpBoxShape::hkpBoxShape` (Xbox PDB, `this` = the 0x30-byte block, `(half extents,
/// radius)`; answers `this`).
const HK_BOX_SHAPE_CONSTRUCT: u32 = 0x00c9_ddb0;
/// `hkReferencedObject` release (`00c905b0`, `this` only).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Unnamed (`004b4db0`, `this` = the four floats written, `(vector)`): converts the four lanes
/// of the vector at its argument.
const COPY_VECTOR: u32 = 0x004b_4db0;
/// Unnamed (`00458620`, cdecl `(out, vector)`): writes three floats at `out` from the vector
/// and answers `out`.
const VECTOR_TO_FLOATS: u32 = 0x0045_8620;
/// `NiPoint3::GetViewerString` (Xbox PDB, `00a59af0`, `this` = the three floats, `(prefix)`).
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `BSTestObjects::CreateBox` (Xbox PDB, cdecl `(three floats, 0)`): answers a geometry.
const CREATE_BOX: u32 = 0x00c5_5920;
/// Interns a string (`00a5b690`, cdecl `(text)`, answers the handle) and `_atexit`
/// (`00ec658f`, cdecl `(function)`).
const INTERN_STRING: u32 = 0x00a5_b690;
const AT_EXIT: u32 = 0x00ec_658f;
/// Unnamed (`00a5b950` in `NiMain/niobjectnet.cpp`, `this` = the geometry, `(pointer to a
/// string handle)`): stores the handle at `+8`.
const SET_NAME: u32 = 0x00a5_b950;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The word the class passes to `NiGetViewerString`, the text `GetViewerString` of the half
/// extents gets as its prefix, and the vector (`1267e30`) used when the Havok shape is
/// missing.
const CLASS_NAME_POINTER: u32 = 0x0126_8264;
const HALF_EXTENT_PREFIX: u32 = 0x010c_72dc;
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
/// The float at `010c72bc` (the box's radius), and the double 2.0 at `01011590`.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const TWO: f64 = 2.0;
/// The name of the display geometry: the text at `010c40a0` (`"bhkBoxShape"`), its interned
/// handle at `0126826c`, the one-time guard at `01268270` (bit 0) and the `_atexit` function.
const NAME_TEXT: u32 = 0x010c_40a0;
const NAME_HANDLE: u32 = 0x0126_826c;
const NAME_GUARD: u32 = 0x0126_8270;
const NAME_CLEANUP: u32 = 0x00fd_e1b0;

/// Class vtable and the three base-class vtables the inlined constructors store before it,
/// each with its object counter.
const VTABLE_CLASS: u32 = 0x0103_093c;
const COUNT_CLASS: u32 = 0x0126_825c;
const BASE_VTABLES: [(u32, u32); 3] = [
    (0x0103_0bf4, 0x0126_80f8),
    (0x0103_0b0c, 0x0126_8274),
    (0x0103_0a24, 0x0126_8250),
];

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

fn count_up(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The byte swap of the four cinfo words (`+0x10..+0x1C`) that `LoadBinary` and `SaveBinary`
/// apply when the stream's flag byte is not 1.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        let word = e.mem.u32(cinfo.wrapping_add(offset));
        e.mem.set_u32(cinfo.wrapping_add(offset), word.swap_bytes());
    }
}

/// The half extents vector of the box: `+0x20` of the Havok shape at `this + 8`, or the
/// default vector when `this` or the shape is missing.
fn half_extents(e: &mut Engine, this: u32) -> u32 {
    let shape = if this == 0 { 0 } else { e.mem.u32(this + 8) };
    if shape == 0 {
        DEFAULT_VECTOR
    } else {
        shape + 0x20
    }
}

/// Appends `text` to the viewer-string array: grown by the step at `+0xE` plus the size when
/// the 16-bit size at `+0xA` has reached the capacity at `+0x8`, then stored at the old size.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    let capacity = u32::from(e.mem.u16(strings + 8));
    if size >= capacity {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c9fd70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9fd70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBoxShape::~bhkBoxShape` (Xbox PDB): stores the class vtable, runs `Kill` (`00c876a0`),
/// decrements the class counter and runs the `bhkConvexShape` destructor (`00c9fb00`).
pub fn bhk_box_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(CONVEX_DESTRUCTOR, &args![this]);
}

// Translated from 00c9fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the constructor, `this` only): runs the `bhkRefObject` constructor (`00c87c30`),
/// clears `+0xC` and `+0x10`, stores the vtable and counts the object for each of the three
/// base classes and then for the class itself. Answers `this`.
pub fn fn_00c9fde0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in BASE_VTABLES {
        e.mem.set_u32(this.addr(), vtable);
        count_up(e, counter);
    }
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    this
}

// Translated from 00c9fe30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9fe30 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkBoxShape`, a 0x14-byte block from the game heap
/// built by `00c9fde0`; 0 when the heap answers 0.
pub fn fn_00c9fe30(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    if object == 0 {
        return 0;
    }
    fn_00c9fde0(e, Ptr::new(object)).addr()
}

// Translated from 00c9fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::CopyMembers` (Xbox PDB): takes the cinfo (virtual slot 0xB4, given a scratch
/// out-pointer), multiplies its four floats at `+0x10..+0x1C` by the float at `process +
/// 0x10` and runs `bhkShape::CopyMembers` (`00c84df0`) on `(this, clone, process)`.
pub fn bhk_box_shape_copy_members(e: &mut Engine, this: Ptr, clone: u32, process: Ptr) {
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    let scale = x87_quiet(e.mem.f32(process.addr() + 0x10));
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        let value = x87_quiet(e.mem.f32(cinfo + offset));
        e.mem.set_f32(cinfo + offset, value * scale);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00c9ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::LoadBinary` (Xbox PDB): `bhkConvexShape::LoadBinary` (`00c9fbc0`), then
/// the cinfo (virtual slot 0xB4, given a scratch out-pointer): when the byte at `stream +
/// 0x1E5` is not 1 its four words at `+0x10..+0x1C` are byte swapped.
pub fn bhk_box_shape_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONVEX_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00c9ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBoxShape::SaveBinary` (Xbox PDB): gets the cinfo (virtual slot 0xB4), byte swaps its
/// four words at `+0x10..+0x1C` when the byte at `stream + 0x1E4` is not 1, runs
/// `bhkConvexShape::SaveBinary` (`00c9fc00`) and swaps them back under the same test.
pub fn bhk_box_shape_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(CONVEX_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00c9ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ffd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBoxShape::CreateClone` (Xbox PDB), with the allocation scope word set to 0x10 and the
/// clone lock held (`00c876f0`, `00c87700`): builds a new object (a 0x14-byte block from the
/// game heap and `00c9fde0`; 0 when the heap answers 0) and runs virtual slot 0xC4
/// (`CopyMembers`) on `(clone, process)`. Answers the clone.
pub fn bhk_box_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let object = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    let clone = if object == 0 {
        0
    } else {
        fn_00c9fde0(e, Ptr::new(object)).addr()
    };
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ca0090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0090 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBoxShape::CreateCinfo` (Xbox PDB), with the allocation scope word set to 0x10:
/// stores in the byte at `created` whether the cinfo at `+0xC` was missing; then builds it:
/// a 0x30-byte block from the memory manager (`00aa3e40`, the answer is not checked) moved
/// up to the next 16-byte boundary (the distance, 1 to 16, is stored in the byte before the
/// cinfo), with `+0` cleared, `+4` the float at `010c72bc` and the vector at `+0x10` set to
/// (1, 1, 1, 0). When the Havok shape (`+8`) exists, `bhkConvexShape::CopyCinfo`
/// (`00c9fb70`) runs and the vector at `+0x10` is converted (`004b4db0`) from the shape's
/// half extents. Answers the cinfo.
pub fn bhk_box_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
            .u32();
        let pad = 0x10 - (block & 0xf);
        let cinfo = block.wrapping_add(pad);
        e.mem.set_u8(cinfo.wrapping_sub(1), pad as u8);
        e.mem.set_u32(cinfo, 0);
        let radius = x87_quiet_bits(e.global::<u32>(DEFAULT_RADIUS));
        e.mem.set_u32(cinfo + 4, radius);
        let one = 1.0f32.to_bits();
        for lane in 0..3u32 {
            e.mem.set_u32(cinfo + 0x10 + 4 * lane, one);
        }
        e.mem.set_u32(cinfo + 0x1c, 0);
        let has_shape = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_shape {
            e.call(CONVEX_COPY_CINFO, &args![this, cinfo]);
            let source = half_extents(e, this.addr());
            e.call(COPY_VECTOR, &args![cinfo + 0x10, source]);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00ca0190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0190 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkBoxShape::Init` (Xbox PDB), with the allocation scope word set to 0x10: builds the
/// 0x30-byte `hkpBoxShape` (`00c9ddb0`, `(vector, radius)`; the memory-size word at `+4` is
/// set to 0x30) from the four floats at `info + 0x10` and the float at `010c72bc`, hands it
/// to virtual slot 0x8C, releases the creation reference (`00c905b0`) and runs virtual slot
/// 0xBC with the cinfo.
pub fn bhk_box_shape_init(e: &mut Engine, this: Ptr, info: Ptr) {
    let (word, saved) = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block + 4, 0x30);
    let radius = x87_quiet_bits(e.global::<u32>(DEFAULT_RADIUS));
    let shape = e.with_stack(0x10, |e, vector| {
        for lane in 0..4u32 {
            let value = e.mem.u32(info.addr() + 0x10 + 4 * lane);
            e.mem.set_u32(vector.addr() + 4 * lane, value);
        }
        e.call(HK_BOX_SHAPE_CONSTRUCT, &args![block, vector, radius])
            .u32()
    });
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ca02b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca02b0 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkBoxShape::BuildDisplayGeometry` (Xbox PDB), with the allocation scope word set to
/// 0x10: converts the half extents vector (the Havok shape's at `+0x20`, or the default at
/// `01267e30`; copied to the stack first) to three floats (`00458620`), doubles each (the
/// double 2.0 at `01011590`) and builds a box geometry from them (`00c55920`, `(floats,
/// 0)`). When it exists: the first time only, interns the text at `010c40a0` into the
/// handle at `0126826c` (guard bit 0 of `01268270`; `_atexit` registers `00fde1b0`), stores
/// the handle in the geometry (`00a5b950`) and runs virtual slot 0xDC of `node` with `(geometry,
/// 1)`.
/// The second stack word (`RET 8`) is not read.
pub fn bhk_box_shape_build_display_geometry(e: &mut Engine, this: Ptr, node: Ptr, _unused_2: u32) {
    let (word, saved) = enter_scope(e);
    let source = half_extents(e, this.addr());
    let geometry = e.with_stack(0x20, |e, buffer| {
        let vector = buffer.addr() + 0x10;
        for lane in 0..4u32 {
            let value = e.mem.u32(source + 4 * lane);
            e.mem.set_u32(vector + 4 * lane, value);
        }
        e.call(VECTOR_TO_FLOATS, &args![buffer, vector]);
        for lane in 0..3u32 {
            let value = x87_quiet(e.mem.f32(buffer.addr() + 4 * lane));
            e.mem
                .set_f32(buffer.addr() + 4 * lane, (f64::from(value) * TWO) as f32);
        }
        e.call(CREATE_BOX, &args![buffer, 0u32]).u32()
    });
    if geometry != 0 {
        let guard: u32 = e.global(NAME_GUARD);
        if guard & 1 == 0 {
            e.set_global(NAME_GUARD, guard | 1);
            let handle = e.call(INTERN_STRING, &args![NAME_TEXT]).u32();
            e.set_global(NAME_HANDLE, handle);
            e.call(AT_EXIT, &args![NAME_CLEANUP]);
        }
        e.call(SET_NAME, &args![geometry, NAME_HANDLE]);
        e.vcall(node.addr(), 0xdc, &args![geometry, 1u32]);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00ca0410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0410 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBoxShape::GetViewerStrings` (Xbox PDB), with the allocation scope word set to 0x10:
/// runs `bhkConvexShape::GetViewerStrings` (`00c9fc50`) on `(this, strings)`, appends the
/// string `NiGetViewerString` makes of the word at `01268264`, then converts the half
/// extents vector (the Havok shape's at `+0x20`, or the default at `01267e30`) to three
/// floats (`00458620`) and appends the string `NiPoint3::GetViewerString` (`00a59af0`) makes
/// of them with the prefix text at `010c72dc` (each append grows the array by the step at
/// `+0xE` plus the size when the 16-bit size at `+0xA` has reached the capacity at `+0x8`).
pub fn bhk_box_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (word, saved) = enter_scope(e);
    e.call(CONVEX_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, strings.addr(), text);
    let source = half_extents(e, this.addr());
    let extents_text = e.with_stack(0x18, |e, buffer| {
        let floats = e.call(VECTOR_TO_FLOATS, &args![buffer, source]).u32();
        let copy = buffer.addr() + 0xc;
        for lane in 0..3u32 {
            let value = e.mem.u32(floats + 4 * lane);
            e.mem.set_u32(copy + 4 * lane, value);
        }
        e.call(POINT_GET_VIEWER_STRING, &args![copy, HALF_EXTENT_PREFIX])
            .u32()
    });
    append_viewer_string(e, strings.addr(), extents_text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9fd70, bhk_box_shape_destructor(Ptr)),
        entry!(0x00c9fde0, fn_00c9fde0(Ptr) -> Ptr),
        entry!(0x00c9fe30, fn_00c9fe30() -> u32),
        entry!(0x00c9fea0, bhk_box_shape_copy_members(Ptr, u32, Ptr)),
        entry!(0x00c9ff00, bhk_box_shape_load_binary(Ptr, Ptr)),
        entry!(0x00c9ff50, bhk_box_shape_save_binary(Ptr, Ptr)),
        entry!(0x00c9ffd0, bhk_box_shape_create_clone(Ptr, u32) -> u32),
        entry!(0x00ca0090, bhk_box_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00ca0190, bhk_box_shape_init(Ptr, Ptr)),
        entry!(
            0x00ca02b0,
            bhk_box_shape_build_display_geometry(Ptr, Ptr, u32)
        ),
        entry!(0x00ca0410, bhk_box_shape_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn map_counters(e: &mut Engine) {
        e.map(COUNT_CLASS, 4);
        e.set_global(COUNT_CLASS, 10u32);
        for (_, counter) in BASE_VTABLES {
            e.map(counter, 4);
            e.set_global(counter, 10u32);
        }
    }

    /// A vtable of `slots` (byte offset, target) in front of a 0x40-byte object.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(CONVEX_DESTRUCTOR, nothing);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x00c9fd70, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c9fd70, KILL, CONVEX_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
        assert_eq!(calls(&e, CONVEX_DESTRUCTOR), [[this]]);
    }

    #[test]
    fn constructor_counts_every_class_and_keeps_the_last_vtable() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0xc, 0x1111);
        e.mem.set_u32(this + 0x10, 0x2222);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9fde0, &args![this]).u32(), this);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[this]]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        for (_, counter) in BASE_VTABLES {
            assert_eq!(e.global::<u32>(counter), 11);
        }
    }

    #[test]
    fn new_builds_the_object_or_answers_zero() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x14);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9fe30, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x14]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00c9fe30, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
    }

    /// A box whose slot 0xB4 answers `cinfo` (four floats 1.0 to 4.0 at `+0x10`).
    fn cinfo_setup() -> (Engine, Ptr, u32) {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x30);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        let this = Ptr::new(object(&mut e, &[(0xb4, 0x0900_00b4)]));
        for lane in 0..4u32 {
            e.mem.set_f32(cinfo + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        (e, this, cinfo)
    }

    #[test]
    fn copy_members_scales_the_cinfo_vector_and_runs_the_shape_copy() {
        let (mut e, this, cinfo) = cinfo_setup();
        e.register(SHAPE_COPY_MEMBERS, nothing);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 2.5);
        e.call_log = Some(vec![]);
        e.call(0x00c9fea0, &args![this, 0x5150u32, process]);
        for lane in 0..4u32 {
            assert_eq!(
                e.mem.f32(cinfo + 0x10 + 4 * lane),
                (1.0 + lane as f32) * 2.5
            );
        }
        assert_eq!(
            calls(&e, SHAPE_COPY_MEMBERS),
            [[this.addr(), 0x5150, process]]
        );
    }

    #[test]
    fn load_binary_swaps_the_cinfo_words_unless_the_flag_is_one() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let (mut e, this, cinfo) = cinfo_setup();
            e.register(CONVEX_LOAD_BINARY, nothing);
            for lane in 0..4u32 {
                e.mem.set_u32(cinfo + 0x10 + 4 * lane, 0x0102_0304 + lane);
            }
            let stream = e.mem.alloc(0x200);
            e.mem.set_u8(stream + 0x1e5, flag);
            e.call_log = Some(vec![]);
            e.call(0x00c9ff00, &args![this, stream]);
            assert_eq!(calls(&e, CONVEX_LOAD_BINARY), [[this.addr(), stream]]);
            if swapped {
                assert_eq!(e.mem.u32(cinfo + 0x10), 0x0403_0201);
                assert_eq!(e.mem.u32(cinfo + 0x1c), (0x0102_0304u32 + 3).swap_bytes());
            } else {
                assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0304);
            }
        }
    }

    #[test]
    fn save_binary_swaps_around_the_base_save_and_restores_the_words() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let (mut e, this, cinfo) = cinfo_setup();
            for lane in 0..4u32 {
                e.mem.set_u32(cinfo + 0x10 + 4 * lane, 0x0102_0304 + lane);
            }
            e.register_double(CONVEX_SAVE_BINARY, move |e, _| {
                let seen = e.mem.u32(cinfo + 0x10);
                e.mem.set_u32(cinfo + 0x2c, seen);
                Ret::default()
            });
            let stream = e.mem.alloc(0x200);
            e.mem.set_u8(stream + 0x1e4, flag);
            e.call(0x00c9ff50, &args![this, stream]);
            let during = e.mem.u32(cinfo + 0x2c);
            if swapped {
                assert_eq!(during, 0x0403_0201);
            } else {
                assert_eq!(during, 0x0102_0304);
            }
            assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0304);
            assert_eq!(e.mem.u32(cinfo + 0x1c), 0x0102_0304 + 3);
        }
    }

    #[test]
    fn create_clone_copies_members_between_the_clone_lock_calls() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x66);
        let block = e.mem.alloc(0x14);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        for address in [REF_OBJECT_CONSTRUCT, CLONE_LOCK, CLONE_UNLOCK, 0x0900_00c4] {
            e.register(address, nothing);
        }
        let this = object(&mut e, &[(0xc4, 0x0900_00c4)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9ffd0, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c9ffd0,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_00c4,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls(&e, 0x0900_00c4), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(word), 0x66);
        // A failed allocation still runs the copy step, with 0.
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00c9ffd0, &args![this, 0x7701u32]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_00c4)[1], [this, 0, 0x7701]);
    }

    #[test]
    fn create_cinfo_builds_an_aligned_block_once() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x66);
        e.map(DEFAULT_RADIUS, 4);
        e.set_global(DEFAULT_RADIUS, 0.1f32.to_bits());
        let block = e.mem.alloc(0x80) + 5;
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(CONVEX_COPY_CINFO, nothing);
        e.register(COPY_VECTOR, nothing);
        let this = e.mem.alloc(0x14);
        let created = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00ca0090, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(cinfo, block + pad);
        assert_eq!(e.mem.u8(cinfo - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.f32(cinfo + 4), 0.1);
        for lane in 0..3u32 {
            assert_eq!(e.mem.f32(cinfo + 0x10 + 4 * lane), 1.0);
        }
        assert_eq!(e.mem.f32(cinfo + 0x1c), 0.0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x30]]);
        // No Havok shape yet: nothing is copied.
        assert!(calls(&e, CONVEX_COPY_CINFO).is_empty());
        assert!(calls(&e, COPY_VECTOR).is_empty());
        // The second time the cinfo exists: nothing is allocated.
        assert_eq!(e.call(0x00ca0090, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        // With a Havok shape, a fresh cinfo is filled from its half extents.
        let other = e.mem.alloc(0x14);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(other + 8, shape);
        let fresh = e.call(0x00ca0090, &args![other, created]).u32();
        assert_eq!(calls(&e, CONVEX_COPY_CINFO), [[other, fresh]]);
        assert_eq!(calls(&e, COPY_VECTOR), [[fresh + 0x10, shape + 0x20]]);
        assert_eq!(e.mem.u32(word), 0x66);
    }

    #[test]
    fn init_builds_the_box_inside_the_scope_word() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.map(DEFAULT_RADIUS, 4);
        e.set_global(DEFAULT_RADIUS, 0.1f32.to_bits());
        let allocator = object(&mut e, &[(4, 0x0900_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let shape = e.mem.alloc(0x30);
        e.register_double(0x0900_0004, move |e, a| {
            assert_eq!(a[1], 0x30);
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret {
                eax: shape,
                ..Ret::default()
            }
        });
        let seen = Rc::new(Cell::new([0.0f32; 4]));
        let seen_in = seen.clone();
        e.register_double(HK_BOX_SHAPE_CONSTRUCT, move |e, a| {
            let mut lanes = [0.0f32; 4];
            for (lane, value) in lanes.iter_mut().enumerate() {
                *value = e.mem.f32(a[1] + 4 * lane as u32);
            }
            seen_in.set(lanes);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x0900_008c, nothing);
        e.register(0x0900_00bc, nothing);
        e.register(REMOVE_REFERENCE, nothing);
        let this = object(&mut e, &[(0x8c, 0x0900_008c), (0xbc, 0x0900_00bc)]);
        let info = e.mem.alloc(0x40);
        for lane in 0..4u32 {
            e.mem.set_f32(info + 0x10 + 4 * lane, 0.5 + lane as f32);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ca0190, &args![this, info]);
        assert_eq!(e.mem.u16(shape + 4), 0x30);
        assert_eq!(seen.get(), [0.5, 1.5, 2.5, 3.5]);
        assert_eq!(calls(&e, HK_BOX_SHAPE_CONSTRUCT)[0][0], shape);
        assert_eq!(calls(&e, HK_BOX_SHAPE_CONSTRUCT)[0][2], 0.1f32.to_bits());
        assert_eq!(calls(&e, 0x0900_008c), [[this, shape]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), [[shape]]);
        assert_eq!(calls(&e, 0x0900_00bc), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    /// An engine for `BuildDisplayGeometry`: `00458620` writes (1.5, 2.5, -3.0) and
    /// `00c55920` answers `geometry` (recording the floats it was given).
    fn display_setup(geometry: u32) -> (Engine, Rc<Cell<[f32; 3]>>) {
        let mut e = Engine::new();
        e.map(NAME_GUARD, 4);
        e.map(NAME_HANDLE, 4);
        e.map(DEFAULT_VECTOR, 16);
        e.register_double(VECTOR_TO_FLOATS, |e, a| {
            for (lane, value) in [1.5f32, 2.5, -3.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let given = Rc::new(Cell::new([0.0f32; 3]));
        let given_in = given.clone();
        e.register_double(CREATE_BOX, move |e, a| {
            assert_eq!(a[1], 0);
            let mut lanes = [0.0f32; 3];
            for (lane, value) in lanes.iter_mut().enumerate() {
                *value = e.mem.f32(a[0] + 4 * lane as u32);
            }
            given_in.set(lanes);
            Ret {
                eax: geometry,
                ..Ret::default()
            }
        });
        e.register(INTERN_STRING, |_, _| Ret {
            eax: 0x7ee0,
            ..Ret::default()
        });
        for address in [AT_EXIT, SET_NAME, 0x0900_00dc] {
            e.register(address, nothing);
        }
        (e, given)
    }

    #[test]
    fn build_display_geometry_doubles_the_extents_and_names_the_geometry_once() {
        let (mut e, given) = display_setup(0x6600);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
        let this = e.mem.alloc(0x14);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, shape);
        let node = object(&mut e, &[(0xdc, 0x0900_00dc)]);
        e.call_log = Some(vec![]);
        e.call(0x00ca02b0, &args![this, node, 0u32]);
        assert_eq!(given.get(), [3.0, 5.0, -6.0]);
        assert_eq!(
            calls(&e, VECTOR_TO_FLOATS)[0][1] - calls(&e, VECTOR_TO_FLOATS)[0][0],
            0x10
        );
        assert_eq!(calls(&e, INTERN_STRING), [[NAME_TEXT]]);
        assert_eq!(e.global::<u32>(NAME_HANDLE), 0x7ee0);
        assert_eq!(e.global::<u32>(NAME_GUARD) & 1, 1);
        assert_eq!(calls(&e, AT_EXIT), [[NAME_CLEANUP]]);
        assert_eq!(calls(&e, SET_NAME), [[0x6600, NAME_HANDLE]]);
        assert_eq!(calls(&e, 0x0900_00dc), [[node, 0x6600, 1]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time the name is already interned.
        e.call(0x00ca02b0, &args![this, node, 0u32]);
        assert_eq!(calls(&e, INTERN_STRING).len(), 1);
        assert_eq!(calls(&e, AT_EXIT).len(), 1);
        assert_eq!(calls(&e, SET_NAME).len(), 2);
        assert_eq!(calls(&e, 0x0900_00dc).len(), 2);
    }

    #[test]
    fn build_display_geometry_does_nothing_more_without_a_geometry() {
        let (mut e, _) = display_setup(0);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
        let this = e.mem.alloc(0x14);
        let node = object(&mut e, &[(0xdc, 0x0900_00dc)]);
        e.call_log = Some(vec![]);
        // The Havok shape is missing: the vector at 01267e30 is converted.
        e.call(0x00ca02b0, &args![this, node, 0u32]);
        assert_eq!(
            calls(&e, VECTOR_TO_FLOATS)[0][1] - calls(&e, VECTOR_TO_FLOATS)[0][0],
            0x10
        );
        assert!(calls(&e, INTERN_STRING).is_empty());
        assert!(calls(&e, SET_NAME).is_empty());
        assert!(calls(&e, 0x0900_00dc).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn build_display_geometry_copies_the_default_vector_when_the_shape_is_missing() {
        let (mut e, _) = display_setup(0);
        for lane in 0..4u32 {
            e.mem.set_u32(DEFAULT_VECTOR + 4 * lane, 0x40 + lane);
        }
        let seen = Rc::new(Cell::new(0u32));
        let seen_in = seen.clone();
        e.register_double(VECTOR_TO_FLOATS, move |e, a| {
            seen_in.set(e.mem.u32(a[1] + 0xc));
            for lane in 0..3u32 {
                e.mem.set_f32(a[0] + 4 * lane, 1.0);
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x14);
        let node = object(&mut e, &[]);
        e.call(0x00ca02b0, &args![this, node, 0u32]);
        assert_eq!(seen.get(), 0x43);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_string_and_the_half_extents() {
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(CLASS_NAME_POINTER, 4);
            e.set_global(CLASS_NAME_POINTER, 0x5151_0001u32);
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x66);
            e.register(CONVEX_GET_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register_double(VECTOR_TO_FLOATS, |e, a| {
                // The source vector pointer is the Havok shape's half extents.
                assert_eq!(a[1], 0x4000 + 0x20);
                for lane in 0..3u32 {
                    e.mem.set_u32(a[0] + 4 * lane, 0xa0 + lane);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            let copy = Rc::new(Cell::new([0u32; 3]));
            let copy_in = copy.clone();
            e.register_double(POINT_GET_VIEWER_STRING, move |e, a| {
                assert_eq!(a[1], HALF_EXTENT_PREFIX);
                let mut words = [0u32; 3];
                for (lane, word) in words.iter_mut().enumerate() {
                    *word = e.mem.u32(a[0] + 4 * lane as u32);
                }
                copy_in.set(words);
                Ret {
                    eax: 0x7777_0002,
                    ..Ret::default()
                }
            });
            e.register(STRINGS_GROW, nothing);
            let stored = Rc::new(Cell::new((0u32, 0u32)));
            let stored_in = stored.clone();
            e.register_double(STRINGS_SET_AT, move |e, a| {
                let text = e.mem.u32(a[2]);
                if a[1] == u32::from(size) {
                    stored_in.set((text, stored_in.get().1));
                } else {
                    stored_in.set((stored_in.get().0, text));
                }
                let count = e.mem.u16(a[0] + 0xa);
                e.mem.set_u16(a[0] + 0xa, count + 1);
                Ret::default()
            });
            let this = e.mem.alloc(0x14);
            e.mem.set_u32(this + 8, 0x4000);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00ca0410, &args![this, strings]);
            assert_eq!(calls(&e, CONVEX_GET_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            assert_eq!(copy.get(), [0xa0, 0xa1, 0xa2]);
            assert_eq!(stored.get(), (0x7777_0001, 0x7777_0002));
            let grown = calls(&e, STRINGS_GROW);
            if size >= capacity {
                // The test double raises the size by one per store, so the second
                // append grows the array again.
                assert_eq!(
                    grown,
                    [
                        [strings, 5 + u32::from(size)],
                        [strings, 6 + u32::from(size)]
                    ]
                );
            } else {
                assert!(grown.is_empty());
            }
            assert_eq!(calls(&e, STRINGS_SET_AT).len(), 2);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }
}
