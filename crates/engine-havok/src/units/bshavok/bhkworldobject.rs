//! `BSHavok/bhkworldobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkWorldObject` keeps its Havok object at `+8`; most functions reach, through its `+0x10`
//! area, a linked object (`[[hk + 0x10] + 8]`) and a 32-bit word at `hk + 0x10 + 0x1C` whose
//! upper half and bit 14 the setters change. The functions marked `Translation incomplete`
//! carry an `FS:[0]` exception-unwinding frame or a `/GS` cookie check (`00ec408c`), neither of
//! which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, push_viewer_string, release_ni_object, FORMAT_TEXT, HEAP_ALLOCATE,
    INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT, MEMORY_ROUTER, NI_GET_VIEWER_STRING,
    NI_GET_VIEWER_STRING_TEXT, SERIALIZABLE_KILL,
};

/// The key used when the caller passes none (an address in `.data`, used as
/// the key value itself).
const DEFAULT_KEY_011F4280: u32 = 0x011f_4280;
/// `00c6d840`: thiscall on the Havok world object, (output pair, key, flag)
/// with `RET 0xc`; returns the output pair, whose first word is the value.
const GET_PROPERTY: u32 = 0x00c6_d840;
/// `hkpWorldObject::removeProperty` (`00cd2a50`, `this` = the Havok object, `(out, key)`) and
/// `addProperty` (`00cd2be0`, `(key, value, 0)`), Xbox PDB names.
const REMOVE_PROPERTY: u32 = 0x00cd_2a50;
const ADD_PROPERTY: u32 = 0x00cd_2be0;
/// The vtable `bhkSerializable` stores, its live-object counter and its destructor body
/// (`00c877b0`), which `~bhkWorldObject` runs.
const SERIALIZABLE_VTABLE: u32 = 0x0103_0394;
const SERIALIZABLE_COUNT: u32 = 0x0126_810c;
const SERIALIZABLE_DESTRUCT: u32 = 0x00c8_77b0;
/// `NiObject::IsEqual` (`00ba87d0`, a jump to `00a5d470`), `(this, other)`.
const NI_OBJECT_IS_EQUAL: u32 = 0x00ba_87d0;
/// `bhkWorld::Add` (`00c6b0a0`, `this` = the world, `(havok object)`).
const WORLD_ADD: u32 = 0x00c6_b0a0;
/// `bhkSerializable::CopyMembers` (`00c87600`, `(this, clone, context)`), `LinkObject`
/// (`00c87670`, `(this, stream)`), `LoadBinary` (`00c87650`), `SaveBinary` (`00c87850`) and
/// `GetViewerStrings` (`00c878c0`), all `(this, argument)`.
const SERIALIZABLE_COPY_MEMBERS: u32 = 0x00c8_7600;
const SERIALIZABLE_LINK_OBJECT: u32 = 0x00c8_7670;
const SERIALIZABLE_LOAD_BINARY: u32 = 0x00c8_7650;
const SERIALIZABLE_SAVE_BINARY: u32 = 0x00c8_7850;
const SERIALIZABLE_GET_VIEWER_STRINGS: u32 = 0x00c8_78c0;
/// `NiObject::RegisterStreamables` thunk (`00a81fe0`, a jump to `00a5d2f0`; `(this, stream)`).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
/// `bhkRefObject::Link` (`00c87c90`, `(this, flag word)`).
const REF_OBJECT_LINK: u32 = 0x00c8_7c90;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `this` = the stream, no further argument) as
/// `LinkObject` calls it, and the dynamic-cast test `00653290` (`this` = an object, `(rtti)`,
/// answers `this` or 0).
const STREAM_GET_LINKED_OBJECT: u32 = 0x00a6_3430;
const OBJECT_CAST: u32 = 0x0065_3290;
/// The RTTI records `LinkObject` tests the object and the linked object against.
const RTTI_THIS: u32 = 0x0126_81c0;
const RTTI_LINKED_FIRST: u32 = 0x0126_82e8;
const RTTI_LINKED_SECOND: u32 = 0x0126_82dc;
/// `bhkMoppBvTreeShape::bhkMoppBvTreeShape` (`00cca4e0`, `(this, shape)`, answers `this`).
const MOPP_TREE_SHAPE_CONSTRUCT: u32 = 0x00cc_a4e0;
/// The stream call `LoadBinary` makes first (`00a645b0`, `this` = the stream, no argument).
const STREAM_PREPARE: u32 = 0x00a6_45b0;
/// `bhkStream::SwapHKArray` (`00cf0930`, cdecl `(cinfo, description, cinfo + 0xC)`).
const SWAP_HK_ARRAY: u32 = 0x00cf_0930;
/// The display node `CreateDisplayGeometry` builds: its size, constructor (`00a5ecb0`, `(this,
/// 0)`), the string it names it with (`00a5b690`, cdecl, answers a string handle, whose
/// reference count sits 8 bytes before it), the name setter (`00a5b950`, `(this, &handle)`)
/// and the word holding the empty string's handle (`0109b220`).
const DISPLAY_NODE_SIZE: u32 = 0xac;
const DISPLAY_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const STRING_CREATE: u32 = 0x00a5_b690;
const DISPLAY_NODE_SET_NAME: u32 = 0x00a5_b950;
const DISPLAY_NODE_NAME: u32 = 0x010c_4420;
const EMPTY_STRING_HANDLE: u32 = 0x0109_b220;
/// Fills a three-float vector from a collision filter word (`00c84470`, cdecl `(word,
/// vector)`).
const FILTER_COLOR: u32 = 0x00c8_4470;
/// Appends the strings of a collision filter word (`00c849b0`, `this` = the address of the
/// word, `(strings)`).
const FILTER_VIEWER_STRINGS: u32 = 0x00c8_49b0;
/// The word `GetViewerStrings` passes to `NiGetViewerString` first, the format and name of
/// the second string.
const CLASS_NAME_POINTER: u32 = 0x0126_8110;
const LINKED_FORMAT: u32 = 0x010c_59e4;
const LINKED_NAME: u32 = 0x0105_c390;

/// `[[[this + 8] + 0x10] + 8]`: the object linked through the Havok object's `+0x10` area,
/// 0 when `this` or any link of the chain is 0.
fn linked_object(e: &Engine, this: u32) -> u32 {
    if this == 0 {
        return 0;
    }
    let havok = e.mem.u32(this + 8);
    if havok == 0 || havok.wrapping_add(0x10) == 0 {
        return 0;
    }
    let holder = e.mem.u32(havok + 0x10);
    if holder == 0 {
        return 0;
    }
    e.mem.u32(holder + 8)
}

/// The filter word at `+0x1C` of the area at `havok + 0x10` of the Havok object at
/// `this + 8`; 0 when there is none.
fn filter_word(e: &Engine, this: u32) -> u32 {
    let havok = e.mem.u32(this + 8);
    if havok != 0 && havok.wrapping_add(0x10) != 0 {
        e.mem.u32(havok + 0x2c)
    } else {
        0
    }
}

/// Stores the filter word when the Havok object exists.
fn set_filter_word(e: &mut Engine, this: u32, value: u32) {
    let havok = e.mem.u32(this + 8);
    if havok != 0 && havok.wrapping_add(0x10) != 0 {
        e.mem.set_u32(havok + 0x2c, value);
    }
}

/// Frees the array the cinfo at `cinfo + 0xC` owns (`LoadBinary` and `SaveBinary` both do
/// it after the byte swap): clears `+0x10`, hands the pointer and the byte size (the low 30
/// bits of the word at `+0x14`, times 8) to the current Havok allocator's slot 0x10 when
/// the sign bit of that word is clear, then clears `+0xC` and sets `+0x14` to 0x80000000.
fn free_cinfo_array(e: &mut Engine, cinfo: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(cinfo + 0x14);
    e.mem.set_u32(cinfo + 0x10, 0);
    if capacity as i32 >= 0 {
        let size = (capacity & 0x3fff_ffff).wrapping_mul(8);
        let array = e.mem.u32(cinfo + 0xc);
        e.vcall(allocator, 0x10, &args![array, size]);
    }
    e.mem.set_u32(cinfo + 0xc, 0);
    e.mem.set_u32(cinfo + 0x14, 0x8000_0000);
}

// Translated from 00c85790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c85790 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldObject::~bhkWorldObject` (Xbox PDB): stores the `bhkSerializable` vtable
/// (`01030394`), runs `bhkSerializable::Kill`, decrements the live counter at `0126810c` and
/// runs the `bhkSerializable` destructor body (`00c877b0`).
pub fn bhk_world_object_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SERIALIZABLE_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(SERIALIZABLE_COUNT);
    e.mem.set_u32(SERIALIZABLE_COUNT, count.wrapping_sub(1));
    e.call(SERIALIZABLE_DESTRUCT, &args![this]);
}

// Translated from 00c85800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a jump to `00ba87d0` (itself a jump to `NiObject::IsEqual`, `00a5d470`) with
/// `this` and the one argument unchanged. Answers its `bool`.
pub fn fn_00c85800(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00c85810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::Add` (Xbox PDB): takes `world`. Answers false when `this` has no Havok
/// object (`+8`). With no world, runs virtual slot 0xA0 of `this` and answers false. Otherwise
/// asks virtual slot 0x94 for the world the object is in; when that is the word at
/// `world + 8` answers false; else runs slot 0xA0, then `bhkWorld::Add` (`00c6b0a0`) on
/// `(world, havok object)` and answers true.
pub fn bhk_world_object_add(e: &mut Engine, this: Ptr, world: u32) -> bool {
    let havok = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if havok == 0 {
        return false;
    }
    if world == 0 {
        e.vcall(this.addr(), 0xa0, &args![]);
        return false;
    }
    let current = e.vcall(this.addr(), 0x94, &args![]).u32();
    if current == e.mem.u32(world + 8) {
        return false;
    }
    e.vcall(this.addr(), 0xa0, &args![]);
    e.call(WORLD_ADD, &args![world, havok]);
    true
}

// Translated from 00c85880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::CopyCinfo` (Xbox PDB): with the allocation scope word at 0x10, copies
/// from the Havok object (`+8`, when there is one) the word at `+0x10` to `cinfo + 4` and the
/// byte at `+0x28` to `cinfo + 8`, clears `cinfo + 0x10` and stores the word at `+0x2C` of the
/// Havok object in `cinfo + 0` (0 when there is no Havok object).
pub fn bhk_world_object_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    let (word, saved) = enter_scope(e);
    let havok = e.mem.u32(this.addr() + 8);
    if havok != 0 {
        let area = havok.wrapping_add(0x10);
        if area != 0 {
            let first = e.mem.u32(area);
            e.mem.set_u32(cinfo + 4, first);
            let flag = e.mem.u8(area.wrapping_add(0x18));
            e.mem.set_u8(cinfo + 8, flag);
        }
    }
    e.mem.set_u32(cinfo + 0x10, 0);
    let havok = e.mem.u32(this.addr() + 8);
    let area = havok.wrapping_add(0x10);
    if havok != 0 && area != 0 {
        let value = e.mem.u32(area.wrapping_add(0x1c));
        e.mem.set_u32(cinfo, value);
    } else {
        e.mem.set_u32(cinfo, 0);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c85900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c85900 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldObject::CopyMembers` (Xbox PDB), with the allocation scope word at 0x10: gets
/// the cinfo (virtual slot 0xB4, with a scratch out byte), clears its word at `+0x10`; when
/// its word at `+4` is an object whose `+8` is not 0 (call it the target) and the target's
/// virtual slot 0xD0 on `context` answers false, replaces the cinfo's `+4` with the word at
/// `+8` of what the target's slot 0x48 answers on `context` (0 when it answers 0). Clears the
/// 16-bit word at cinfo `+2` and runs `bhkSerializable::CopyMembers` (`00c87600`) on `(this,
/// clone, context)`.
pub fn bhk_world_object_copy_members(e: &mut Engine, this: Ptr, clone: u32, context: u32) {
    let (word, saved) = enter_scope(e);
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    e.mem.set_u32(cinfo + 0x10, 0);
    let entry = e.mem.u32(cinfo + 4);
    let target = if entry != 0 { e.mem.u32(entry + 8) } else { 0 };
    if target != 0 && !e.vcall(target, 0xd0, &args![context]).bool() {
        let converted = e.vcall(target, 0x48, &args![context]).u32();
        let value = if converted != 0 {
            e.mem.u32(converted + 8)
        } else {
            0
        };
        e.mem.set_u32(cinfo + 4, value);
    }
    e.mem.set_u16(cinfo + 2, 0);
    e.call(SERIALIZABLE_COPY_MEMBERS, &args![this, clone, context]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c859e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c859e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldObject::LinkObject` (Xbox PDB): takes the stream. Gets the linked object from the
/// stream (`00a63430`) and the cinfo (virtual slot 0xB4, scratch out byte). When there is a
/// linked object it stores its word at `+8` in the cinfo's `+4`; then, when `this` casts
/// (`00653290`) to the record at `012681c0` and the linked object casts to the record at
/// `012682e8` or `012682dc`, it builds a 0x14-byte object from the heap with `00cca4e0` on the
/// linked object and stores that object's `+8` instead (when the allocation worked). At the
/// end runs `bhkSerializable::LinkObject` (`00c87670`) on `(this, stream)`.
pub fn bhk_world_object_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let linked = e.call(STREAM_GET_LINKED_OBJECT, &args![stream]).u32();
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if linked != 0 {
        let shape = e.mem.u32(linked + 8);
        e.mem.set_u32(cinfo + 4, shape);
        let wrap = e.call(OBJECT_CAST, &args![this, RTTI_THIS]).u32() != 0
            && (e.call(OBJECT_CAST, &args![linked, RTTI_LINKED_FIRST]).u32() != 0
                || e.call(OBJECT_CAST, &args![linked, RTTI_LINKED_SECOND])
                    .u32()
                    != 0);
        if wrap {
            let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
            let wrapper = if block != 0 {
                e.call(MOPP_TREE_SHAPE_CONSTRUCT, &args![block, linked])
                    .u32()
            } else {
                0
            };
            if wrapper != 0 {
                let inner = e.mem.u32(wrapper + 8);
                e.mem.set_u32(cinfo + 4, inner);
            }
        }
    }
    e.call(SERIALIZABLE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00c85ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::RegisterStreamables` (Xbox PDB; a `bool` there): takes the stream. When
/// the chain `[[[this + 8] + 0x10] + 8]` leads to an object, runs its virtual slot 0x54 on
/// the stream; then the base `NiObject::RegisterStreamables` thunk (`00a81fe0`) on `(this,
/// stream)`, whose value it answers (the exe ends on that call and leaves its `EAX`, which
/// the callers test).
pub fn bhk_world_object_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    let linked = linked_object(e, this.addr());
    if linked != 0 {
        e.vcall(linked, 0x54, &args![stream]);
    }
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00c85b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c85b00 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (`thiscall`, no arguments), with the allocation scope word set to 0x10: when the
/// chain `[[[this + 8] + 0x10] + 8]` leads to an object, drops one reference from the
/// count at its `+4` (`InterlockedDecrement`) and, when it reached 0, runs its virtual slot 4.
pub fn fn_00c85b00(e: &mut Engine, this: Ptr) {
    let (word, saved) = enter_scope(e);
    let linked = linked_object(e, this.addr());
    if linked != 0 {
        release_ni_object(e, linked);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c85ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::Link` (Xbox PDB): takes the flag word (its low byte decides). With the
/// flag set, takes a reference on the linked object (`InterlockedIncrement` of its `+4`,
/// when the chain `[[[this + 8] + 0x10] + 8]` leads to one) and stores `this` in the Havok
/// object's `+0xC`; with it clear, runs virtual slot 0xA0, `00c85b00` and stores 0 there
/// instead. Then `bhkRefObject::Link` (`00c87c90`) on `(this, flag word)`.
pub fn bhk_world_object_link(e: &mut Engine, this: Ptr, flag: u32) {
    let owner = if flag as u8 != 0 {
        let linked = linked_object(e, this.addr());
        if linked != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![linked + 4]);
        }
        this.addr()
    } else {
        e.vcall(this.addr(), 0xa0, &args![]);
        fn_00c85b00(e, this);
        0
    };
    if !this.is_null() {
        let havok = e.mem.u32(this.addr() + 8);
        if havok != 0 {
            e.mem.set_u32(havok + 0xc, owner);
        }
    }
    e.call(REF_OBJECT_LINK, &args![this, flag]);
}

// Translated from 00c85c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::SetAVObject` (map name): takes `(object, key)`. The key is
/// `011f4280` when 0. When the Havok object (`+8`) has a property with that key in its table
/// (array at `+0x78`, count at `+0x7C`, 0x10 bytes each, the key first), removes it
/// (`hkpWorldObject::removeProperty`, `00cd2a50`, `(out pair, key)`). Then, when `object` is
/// not 0 and there is a Havok object, adds the property (`00cd2be0`, `(key, object, 0)`).
pub fn bhk_world_object_set_av_object(e: &mut Engine, this: Ptr, object: u32, key: u32) {
    let key = if key == 0 { DEFAULT_KEY_011F4280 } else { key };
    let havok = e.mem.u32(this.addr() + 8);
    if havok != 0 {
        let count = e.mem.i32(havok + 0x7c);
        let table = e.mem.u32(havok + 0x78);
        let found = count > 0 && (0..count as u32).any(|i| e.mem.u32(table + 0x10 * i) == key);
        if found {
            e.with_stack(8, |e, out| {
                e.call(REMOVE_PROPERTY, &args![havok, out, key]);
            });
        }
    }
    if object != 0 {
        let havok = e.mem.u32(this.addr() + 8);
        if havok != 0 {
            e.call(ADD_PROPERTY, &args![havok, key, object, 0u32]);
        }
    }
}

// Translated from 00c85c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GetAVObject` (map name). Thiscall, one word (`RET 4`).
/// Looks `key` (the address `011f4280` when null) up in the property table
/// of the Havok world object at `this + 8` (array at `+0x78`, count at
/// `+0x7c`, 0x10 bytes each, the key in the first word); when found, asks
/// `00c6d840` for the property and returns its first word, otherwise 0.
pub fn bhk_world_object_get_av_object(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let key = if key == 0 { DEFAULT_KEY_011F4280 } else { key };
    let world_object = e.mem.u32(this.addr() + 8);
    if world_object == 0 {
        return 0;
    }
    let count = e.mem.i32(world_object + 0x7c);
    if count <= 0 {
        return 0;
    }
    let table = e.mem.u32(world_object + 0x78);
    let found = (0..count as u32).any(|i| e.mem.u32(table + 0x10 * i) == key);
    if !found {
        return 0;
    }
    e.with_stack(8, |e, out| {
        let property = e
            .call(GET_PROPERTY, &args![world_object, out, key, 0u32])
            .u32();
        e.mem.u32(property)
    })
}

// Translated from 00c85ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, one word): replaces the upper 16 bits of the filter word (the word at
/// `+0x1C` of the area at `+0x10` of the Havok object at `+8`; 0 when there is no Havok object)
/// by the low 16 bits of the argument, stores it back when there is a Havok object, then runs
/// virtual slot 0xC4 of `this`.
pub fn fn_00c85ce0(e: &mut Engine, this: Ptr, group: u32) {
    let current = filter_word(e, this.addr());
    let value = (current & 0xffff) | (group << 16);
    set_filter_word(e, this.addr(), value);
    e.vcall(this.addr(), 0xc4, &args![]);
}

// Translated from 00c85d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, one word whose low byte is the flag): sets bit 14 of the filter word
/// (see `fn_00c85ce0`) when the byte is 0 and clears it otherwise, stores the word back when
/// there is a Havok object, then runs virtual slot 0xC4 of `this`.
pub fn fn_00c85d20(e: &mut Engine, this: Ptr, flag: u32) {
    let current = filter_word(e, this.addr());
    let value = if flag as u8 == 0 {
        current | 0x4000
    } else {
        current & 0xffff_bfff
    };
    set_filter_word(e, this.addr(), value);
    e.vcall(this.addr(), 0xc4, &args![]);
}

// Translated from 00c85d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c85d70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldObject::CreateDisplayGeometry` (Xbox PDB): takes `node`. When the chain
/// `[[[this + 8] + 0x10] + 8]` leads to an object (the target), uses `node` or, when it is 0,
/// builds a 0xAC-byte one from the heap (`00a5ecb0` on `(block, 0)`). A node whose `+8` is 0
/// gets its name: the string `010c4420` through `00a5b690`, set with `00a5b950` on `(node,
/// &handle)`, the handle's reference count (8 bytes before it) then dropped by
/// `InterlockedDecrement` unless it is the empty string's handle (`0109b220`). Then fills a
/// zeroed three-float vector from the filter word (`00c84470`, `(word, vector)`), runs the
/// target's virtual slot 0xD4 on `(node, vector)` and answers the node. Without a target it
/// answers `node` unchanged.
pub fn bhk_world_object_create_display_geometry(e: &mut Engine, this: Ptr, node: u32) -> u32 {
    let target = linked_object(e, this.addr());
    if target == 0 {
        return node;
    }
    let node = if node == 0 {
        let block = e.call(HEAP_ALLOCATE, &args![DISPLAY_NODE_SIZE]).u32();
        if block != 0 {
            e.call(DISPLAY_NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        }
    } else {
        node
    };
    if e.mem.u32(node + 8) == 0 {
        let handle = e.call(STRING_CREATE, &args![DISPLAY_NODE_NAME]).u32();
        let handle = e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), handle);
            e.call(DISPLAY_NODE_SET_NAME, &args![node, cell]);
            e.mem.u32(cell.addr())
        });
        if handle != e.mem.u32(EMPTY_STRING_HANDLE) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    }
    e.with_stack(12, |e, vector| {
        let word = filter_word(e, this.addr());
        e.call(FILTER_COLOR, &args![word, vector]);
        e.vcall(target, 0xd4, &args![node, vector]);
    });
    node
}

// Translated from 00c85ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::DestroyDisplayGeometry` (Xbox PDB): when the chain
/// `[[[this + 8] + 0x10] + 8]` leads to an object, jumps to its virtual slot 0xD8 (no
/// arguments); otherwise nothing.
pub fn bhk_world_object_destroy_display_geometry(e: &mut Engine, this: Ptr) {
    let linked = linked_object(e, this.addr());
    if linked != 0 {
        e.vcall(linked, 0xd8, &args![]);
    }
}

// Translated from 00c85ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GetProperty` (map name; cdecl, three words): `out` is a 64-bit result,
/// `object` the Havok world object and `key` the property key. With an object it asks
/// `00c6d840` (`this` = the object, `(pair, key, 0)`) and takes the first word of the pair; with
/// none the value is 0. Stores the value as a signed 64-bit integer at `out` (low word first)
/// and answers `out`.
pub fn bhk_world_object_get_property(e: &mut Engine, out: Ptr, object: u32, key: u32) -> Ptr {
    let value = if object != 0 {
        e.with_stack(8, |e, pair| {
            let property = e.call(GET_PROPERTY, &args![object, pair, key, 0u32]).u32();
            e.mem.u32(property)
        })
    } else {
        0
    };
    e.mem.set_u32(out.addr(), value);
    e.mem.set_u32(out.addr() + 4, ((value as i32) >> 31) as u32);
    out
}

// Translated from 00c85f20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c85f20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkWorldObject::LoadBinary` (Xbox PDB), with the allocation scope word at 0x10: runs
/// `00a645b0` on the stream, `bhkSerializable::LoadBinary` (`00c87650`) on `(this, stream)`,
/// gets the cinfo (virtual slot 0xB4, scratch out byte); when the stream's byte at `+0x1E5` is
/// not 1 swaps the cinfo's array (`bhkStream::SwapHKArray`, `00cf0930`, cdecl `(cinfo,
/// virtual slot 0xA8's answer, cinfo + 0xC)`). Frees the cinfo's array (`free_cinfo_array`),
/// byte-swaps the cinfo's first word when the byte is not 1, and, for a stream whose word at
/// `+4` is below 9, turns bit 5 of that first word into bit 15. When the word's low seven bits
/// are 8 or 0x1D, clears its bit 14.
pub fn bhk_world_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (word, saved) = enter_scope(e);
    e.call(STREAM_PREPARE, &args![stream]);
    e.call(SERIALIZABLE_LOAD_BINARY, &args![this, stream]);
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    let native_order = e.mem.u8(stream.addr() + 0x1e5) == 1;
    if !native_order {
        let description = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![cinfo, description, cinfo + 0xc]);
    }
    free_cinfo_array(e, cinfo);
    if !native_order {
        let first = e.mem.u32(cinfo);
        e.mem.set_u32(cinfo, first.swap_bytes());
    }
    if e.mem.u32(stream.addr() + 4) < 9 {
        let first = e.mem.u32(cinfo);
        if first & 0x20 != 0 {
            e.mem.set_u32(cinfo, (first & 0xffff_ffdf) | 0x8000);
        }
    }
    let first = e.mem.u32(cinfo);
    if matches!(first & 0x7f, 8 | 0x1d) {
        e.mem.set_u32(cinfo, first & 0xffff_bfff);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c86060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86060 - the FS:[0] exception-unwinding frame is not modeled, and the three bytes of the stack word after the created flag (passed on to virtual slot 0xA4) are uninitialised in the exe and passed as zeros.
/// `bhkWorldObject::SaveBinary` (Xbox PDB), with the allocation scope word at 0x10: gets the
/// cinfo (virtual slot 0xB4, out byte); when the stream's byte at `+0x1E4` is not 1, swaps
/// the cinfo's array (`00cf0930`, as `LoadBinary`) and byte-swaps its first word. Frees the
/// cinfo's array, then takes the word at `+8` of the object at cinfo `+4` (0 when that is 0) and
/// gives it to the stream's virtual slot 0x30; runs `bhkSerializable::SaveBinary`
/// (`00c87850`) on `(this, stream)` and finally virtual slot 0xA4 of `this` with the stack
/// word that held the created flag.
pub fn bhk_world_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (word, saved) = enter_scope(e);
    e.with_stack(4, |e, flag| {
        let cinfo = e.vcall(this.addr(), 0xb4, &args![flag]).u32();
        let created = u32::from(e.mem.u8(flag.addr()));
        if e.mem.u8(stream.addr() + 0x1e4) != 1 {
            let description = e.vcall(this.addr(), 0xa8, &args![]).u32();
            e.call(SWAP_HK_ARRAY, &args![cinfo, description, cinfo + 0xc]);
            let first = e.mem.u32(cinfo);
            e.mem.set_u32(cinfo, first.swap_bytes());
        }
        free_cinfo_array(e, cinfo);
        let object = e.mem.u32(cinfo + 4);
        let value = if object != 0 {
            e.mem.u32(object + 8)
        } else {
            0
        };
        e.vcall(stream.addr(), 0x30, &args![value]);
        e.call(SERIALIZABLE_SAVE_BINARY, &args![this, stream]);
        e.vcall(this.addr(), 0xa4, &args![created]);
    });
    e.mem.set_u32(word, saved);
}

// Translated from 00c86190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86190 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkWorldObject::GetViewerStrings` (Xbox PDB), with the allocation scope word at 0x10: runs
/// `bhkSerializable::GetViewerStrings` (`00c878c0`) on `(this, strings)`, appends the string
/// `NiGetViewerString` makes of the word at `01268110`, then the filter strings (`00c849b0`,
/// `this` = the address of a local holding the filter word, `(strings)`). When the chain
/// `[[[this + 8] + 0x10] + 8]` leads to an object it formats `"<name> %p"`-style text with
/// `00406d00` into a 0x80-byte buffer (format `010c59e4`, the first word of what the object's
/// virtual slot 8 answers, the object) and appends `NiGetViewerString` of it with the name
/// `0105c390` (`00a7d5e0`).
pub fn bhk_world_object_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(SERIALIZABLE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let filter = filter_word(e, this.addr());
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), filter);
        e.call(FILTER_VIEWER_STRINGS, &args![cell, strings]);
    });
    let linked = linked_object(e, this.addr());
    if linked != 0 {
        let rtti = e.vcall(linked, 8, &args![]).u32();
        let name = e.mem.u32(rtti);
        e.with_stack(0x80, |e, buffer| {
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x80u32, LINKED_FORMAT, name, linked],
            );
            let text = e
                .call(NI_GET_VIEWER_STRING_TEXT, &args![LINKED_NAME, buffer])
                .u32();
            push_viewer_string(e, strings, text);
        });
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c87a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GethkWorldRw` (Xbox PDB): the word at `+8` of the Havok object (`+8`) of
/// `this`; 0 when `this` or the Havok object is 0.
pub fn bhk_world_object_gethk_world_rw(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    let havok = e.mem.u32(this.addr() + 8);
    if havok == 0 {
        return 0;
    }
    e.mem.u32(havok + 8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c85790, bhk_world_object_destruct(Ptr)),
        entry!(0x00c85800, fn_00c85800(Ptr, u32) -> bool),
        entry!(0x00c85810, bhk_world_object_add(Ptr, u32) -> bool),
        entry!(0x00c85880, bhk_world_object_copy_cinfo(Ptr, u32)),
        entry!(0x00c85900, bhk_world_object_copy_members(Ptr, u32, u32)),
        entry!(0x00c859e0, bhk_world_object_link_object(Ptr, u32)),
        entry!(0x00c85ac0, bhk_world_object_register_streamables(Ptr, u32) -> u32),
        entry!(0x00c85b00, fn_00c85b00(Ptr)),
        entry!(0x00c85ba0, bhk_world_object_link(Ptr, u32)),
        entry!(0x00c85c10, bhk_world_object_set_av_object(Ptr, u32, u32)),
        entry!(0x00c85c80, bhk_world_object_get_av_object(Ptr, u32) -> u32),
        entry!(0x00c85ce0, fn_00c85ce0(Ptr, u32)),
        entry!(0x00c85d20, fn_00c85d20(Ptr, u32)),
        entry!(
            0x00c85d70,
            bhk_world_object_create_display_geometry(Ptr, u32) -> u32
        ),
        entry!(0x00c85ec0, bhk_world_object_destroy_display_geometry(Ptr)),
        entry!(0x00c85ef0, bhk_world_object_get_property(Ptr, u32, u32) -> Ptr),
        entry!(0x00c85f20, bhk_world_object_load_binary(Ptr, Ptr)),
        entry!(0x00c86060, bhk_world_object_save_binary(Ptr, Ptr)),
        entry!(0x00c86190, bhk_world_object_get_viewer_strings(Ptr, u32)),
        entry!(0x00c87a50, bhk_world_object_gethk_world_rw(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::SCOPE_WORD;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn world_object_00c85c80(e: &mut Engine, keys: &[u32]) -> Ptr {
        let table = e.mem.alloc(0x10 * keys.len() as u32 + 0x10);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(table + 0x10 * i as u32, *key);
        }
        let world_object = e.mem.alloc(0x100);
        e.mem.set_u32(world_object + 0x78, table);
        e.mem.set_i32(world_object + 0x7c, keys.len() as i32);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, world_object);
        Ptr::<()>::new(this)
    }

    /// An object whose vtable has the given `(slot offset, target)` entries.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    /// A world object whose Havok object (`+8`) points through its `+0x10` area at a holder
    /// whose `+8` is `linked`. Answers `(this, havok, linked)`.
    fn chain(e: &mut Engine, this_slots: &[(u32, u32)], linked: u32) -> (u32, u32, u32) {
        let this = object_with_slots(e, this_slots);
        let havok = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, havok);
        e.mem.set_u32(havok + 0x10, holder);
        e.mem.set_u32(holder + 8, linked);
        (this, havok, linked)
    }

    /// Doubles for the base routines every function calls through the constructor chain.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn bhk_world_object_get_av_object_returns_the_found_propertys_value() {
        let mut e = Engine::new();
        e.register(GET_PROPERTY, |e, a| {
            e.mem.set_u32(a[1], a[2] + 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = world_object_00c85c80(&mut e, &[0x10, 0x20, DEFAULT_KEY_011F4280]);
        assert_eq!(e.call(0x00c8_5c80, &args![this, 0x20u32]).u32(), 0x21);
        // A null key means the default key.
        assert_eq!(
            e.call(0x00c8_5c80, &args![this, 0u32]).u32(),
            DEFAULT_KEY_011F4280 + 1
        );
    }

    #[test]
    fn bhk_world_object_get_av_object_is_zero_when_nothing_matches() {
        let mut e = Engine::new();
        let this = world_object_00c85c80(&mut e, &[0x10, 0x20]);
        assert_eq!(e.call(0x00c8_5c80, &args![this, 0x30u32]).u32(), 0);
        let empty = world_object_00c85c80(&mut e, &[]);
        assert_eq!(e.call(0x00c8_5c80, &args![empty, 0x30u32]).u32(), 0);
        let none = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00c8_5c80, &args![Ptr::<()>::new(none), 0x30u32])
                .u32(),
            0
        );
    }

    #[test]
    fn destruct_installs_the_serializable_vtable_and_counts_down() {
        let mut e = logging_engine();
        quiet(&mut e, &[SERIALIZABLE_KILL, SERIALIZABLE_DESTRUCT]);
        e.map(SERIALIZABLE_COUNT, 4);
        e.mem.set_u32(SERIALIZABLE_COUNT, 3);
        let this = e.mem.alloc(0x20);
        e.call(0x00c8_5790, &args![this]);
        assert_eq!(e.mem.u32(this), SERIALIZABLE_VTABLE);
        assert_eq!(e.mem.u32(SERIALIZABLE_COUNT), 2);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, SERIALIZABLE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn equality_thunk_forwards_to_the_base_test_and_answers_its_bool() {
        let mut e = logging_engine();
        e.register(NI_OBJECT_IS_EQUAL, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert!(e.call(0x00c8_5800, &args![this, 1u32]).bool());
        assert!(!e.call(0x00c8_5800, &args![this, 0u32]).bool());
        assert_eq!(
            calls_to(&e, NI_OBJECT_IS_EQUAL),
            vec![vec![this, 1], vec![this, 0]]
        );
    }

    #[test]
    fn add_moves_the_object_into_a_different_world() {
        let mut e = logging_engine();
        e.register(0x0900_0094, |_, _| Ret {
            eax: 0x4444,
            ..Ret::default()
        });
        quiet(&mut e, &[0x0900_00a0, WORLD_ADD]);
        let this = object_with_slots(&mut e, &[(0x94, 0x0900_0094), (0xa0, 0x0900_00a0)]);
        let world = e.mem.alloc(0x20);
        // No Havok object: nothing happens, whatever the world.
        assert!(!e.call(0x00c8_5810, &args![this, world]).bool());
        assert!(calls_to(&e, 0x0900_00a0).is_empty());
        let havok = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, havok);
        // No world: only the removal slot runs.
        assert!(!e.call(0x00c8_5810, &args![this, 0u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_00a0).len(), 1);
        // Already in that world.
        e.mem.set_u32(world + 8, 0x4444);
        assert!(!e.call(0x00c8_5810, &args![this, world]).bool());
        assert_eq!(calls_to(&e, 0x0900_00a0).len(), 1);
        assert!(calls_to(&e, WORLD_ADD).is_empty());
        // Another world: removed, then added.
        e.mem.set_u32(world + 8, 0x5555);
        assert!(e.call(0x00c8_5810, &args![this, world]).bool());
        assert_eq!(calls_to(&e, 0x0900_00a0).len(), 2);
        assert_eq!(calls_to(&e, WORLD_ADD), vec![vec![world, havok]]);
        // A null `this` has no Havok object either.
        assert!(!e.call(0x00c8_5810, &args![0u32, world]).bool());
    }

    #[test]
    fn copy_cinfo_copies_the_filter_fields_and_restores_the_scope_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let havok = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, havok);
        e.mem.set_u32(havok + 0x10, 0x1234);
        e.mem.set_u8(havok + 0x28, 7);
        e.mem.set_u32(havok + 0x2c, 0xabcd);
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 0x10, 99);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_5880, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1234);
        assert_eq!(e.mem.u8(cinfo + 8), 7);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
        assert_eq!(e.mem.u32(cinfo), 0xabcd);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // Without a Havok object only the +0x10 word and the first word are written.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(cinfo + 0x10, 99);
        e.call(0x00c8_5880, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo), 0);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1234);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
    }

    /// A world object whose slot 0xB4 answers `cinfo` (and sets the out byte) and whose
    /// `bhkSerializable::CopyMembers` is a double; `target` is the object in the cinfo's
    /// `+4`, with slot 0xD0 answering `accepts` and slot 0x48 answering `converted`.
    fn copy_members_fixture(accepts: u32, converted: u32) -> (Engine, u32, u32, u32) {
        let mut e = logging_engine();
        quiet(&mut e, &[SERIALIZABLE_COPY_MEMBERS]);
        let cinfo = e.mem.alloc(0x20);
        e.register_double(0x0900_00b4, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: cinfo,
                ..Ret::default()
            }
        });
        e.register_double(0x0900_00d0, move |_, _| Ret {
            eax: accepts,
            ..Ret::default()
        });
        e.register_double(0x0900_0048, move |_, _| Ret {
            eax: converted,
            ..Ret::default()
        });
        let this = object_with_slots(&mut e, &[(0xb4, 0x0900_00b4)]);
        let target = object_with_slots(&mut e, &[(0xd0, 0x0900_00d0), (0x48, 0x0900_0048)]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, target);
        e.mem.set_u32(cinfo + 4, entry);
        e.mem.set_u32(cinfo + 0x10, 5);
        e.mem.set_u16(cinfo + 2, 9);
        (e, this, cinfo, target)
    }

    #[test]
    fn copy_members_replaces_the_cinfo_entry_unless_the_target_accepts() {
        let (mut e, this, cinfo, target) = copy_members_fixture(1, 0);
        let entry = e.mem.u32(cinfo + 4);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_5900, &args![this, 0x111u32, 0x222u32]);
        assert_eq!(e.mem.u32(cinfo + 4), entry);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
        assert_eq!(e.mem.u16(cinfo + 2), 0);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_COPY_MEMBERS),
            vec![vec![this, 0x111, 0x222]]
        );
        assert_eq!(calls_to(&e, 0x0900_00d0), vec![vec![target, 0x222]]);
        assert!(calls_to(&e, 0x0900_0048).is_empty());
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // The target refuses: the entry becomes the converted object's +8, or 0.
        let (mut e, this, cinfo, target) = copy_members_fixture(0, 0);
        e.call(0x00c8_5900, &args![this, 0x111u32, 0x222u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(calls_to(&e, 0x0900_0048), vec![vec![target, 0x222]]);
        let (mut e, this, cinfo, _) = copy_members_fixture(0, 0);
        let converted = e.mem.alloc(0x20);
        e.mem.set_u32(converted + 8, 0x7777);
        e.register_double(0x0900_0048, move |_, _| Ret {
            eax: converted,
            ..Ret::default()
        });
        e.call(0x00c8_5900, &args![this, 0x111u32, 0x222u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7777);
        // No object in the cinfo's +4: nothing is asked.
        let (mut e, this, cinfo, _) = copy_members_fixture(0, 0);
        e.mem.set_u32(cinfo + 4, 0);
        e.call(0x00c8_5900, &args![this, 0x111u32, 0x222u32]);
        assert!(calls_to(&e, 0x0900_00d0).is_empty());
        assert_eq!(e.mem.u16(cinfo + 2), 0);
    }

    /// The pieces `LinkObject` needs: the stream's linked object (`linked`, whose `+8` is
    /// 0x7777), a cinfo, the casts that accept the listed records and a heap that builds
    /// the wrapper object whose `+8` is 0x9999.
    fn link_fixture(linked: bool, accepted: Vec<u32>) -> (Engine, u32, u32, u32) {
        let mut e = logging_engine();
        quiet(&mut e, &[SERIALIZABLE_LINK_OBJECT]);
        let linked_object = e.mem.alloc(0x20);
        e.mem.set_u32(linked_object + 8, 0x7777);
        let answer = if linked { linked_object } else { 0 };
        e.register_double(STREAM_GET_LINKED_OBJECT, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        e.register_double(OBJECT_CAST, move |_, a| Ret {
            eax: if accepted.contains(&a[1]) { a[0] } else { 0 },
            ..Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MOPP_TREE_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x9999);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let cinfo = e.mem.alloc(0x20);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        let this = object_with_slots(&mut e, &[(0xb4, 0x0900_00b4)]);
        (e, this, cinfo, linked_object)
    }

    #[test]
    fn link_object_stores_the_linked_shape_or_a_wrapper_around_it() {
        // Casts that do not match: the linked object's +8 goes to the cinfo.
        let (mut e, this, cinfo, _) = link_fixture(true, vec![RTTI_THIS]);
        e.call(0x00c8_59e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7777);
        assert!(calls_to(&e, MOPP_TREE_SHAPE_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LINK_OBJECT),
            vec![vec![this, 0x55]]
        );
        assert_eq!(calls_to(&e, STREAM_GET_LINKED_OBJECT), vec![vec![0x55]]);
        // `this` casts and the linked object casts to the second record: a wrapper.
        let (mut e, this, cinfo, linked) = link_fixture(true, vec![RTTI_THIS, RTTI_LINKED_SECOND]);
        e.call(0x00c8_59e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x9999);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, MOPP_TREE_SHAPE_CONSTRUCT)[0][1], linked);
        // The first record is enough.
        let (mut e, this, cinfo, _) = link_fixture(true, vec![RTTI_THIS, RTTI_LINKED_FIRST]);
        e.call(0x00c8_59e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x9999);
        // `this` does not cast: no second test is made.
        let (mut e, this, cinfo, _) = link_fixture(true, vec![RTTI_LINKED_FIRST]);
        e.call(0x00c8_59e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7777);
        assert_eq!(calls_to(&e, OBJECT_CAST).len(), 1);
        // No linked object: the cinfo is left alone.
        let (mut e, this, cinfo, _) = link_fixture(false, vec![RTTI_THIS]);
        e.call(0x00c8_59e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert!(calls_to(&e, OBJECT_CAST).is_empty());
        assert_eq!(calls_to(&e, SERIALIZABLE_LINK_OBJECT).len(), 1);
    }

    #[test]
    fn register_streamables_asks_the_linked_object_first() {
        let mut e = logging_engine();
        quiet(&mut e, &[0x0900_0054, NI_OBJECT_REGISTER_STREAMABLES]);
        let linked = object_with_slots(&mut e, &[(0x54, 0x0900_0054)]);
        let (this, havok, _) = chain(&mut e, &[], linked);
        e.call(0x00c8_5ac0, &args![this, 0x66u32]);
        assert_eq!(calls_to(&e, 0x0900_0054), vec![vec![linked, 0x66]]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES),
            vec![vec![this, 0x66]]
        );
        // The chain is cut: only the base runs.
        e.mem.set_u32(havok + 0x10, 0);
        e.call(0x00c8_5ac0, &args![this, 0x66u32]);
        assert_eq!(calls_to(&e, 0x0900_0054).len(), 1);
        assert_eq!(calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES).len(), 2);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_5ac0, &args![this, 0x66u32]);
        assert_eq!(calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES).len(), 3);
    }

    #[test]
    fn release_drops_a_reference_and_deletes_at_zero() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        quiet(&mut e, &[0x0900_0004]);
        let linked = ni_object(&mut e, 2, 0x0900_0004);
        let (this, _, _) = chain(&mut e, &[], linked);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_5b00, &args![this]);
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert!(calls_to(&e, 0x0900_0004).is_empty());
        e.call(0x00c8_5b00, &args![this]);
        assert_eq!(e.mem.u32(linked + 4), 0);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![linked]]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // Without a linked object nothing is released.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_5b00, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn link_takes_or_drops_a_reference_and_records_the_owner() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        quiet(&mut e, &[0x0900_0004, 0x0900_00a0, REF_OBJECT_LINK]);
        let linked = ni_object(&mut e, 1, 0x0900_0004);
        let (this, havok, _) = chain(&mut e, &[(0xa0, 0x0900_00a0)], linked);
        e.call(0x00c8_5ba0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(linked + 4), 2);
        assert_eq!(e.mem.u32(havok + 0xc), this);
        assert_eq!(calls_to(&e, REF_OBJECT_LINK), vec![vec![this, 1]]);
        assert!(calls_to(&e, 0x0900_00a0).is_empty());
        // Unlinking (the low byte is 0, the whole word goes on): slot 0xA0, then a release.
        e.mem.set_u32(linked + 4, 1);
        e.call(0x00c8_5ba0, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x0900_00a0).len(), 1);
        assert_eq!(e.mem.u32(linked + 4), 0);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![linked]]);
        assert_eq!(e.mem.u32(havok + 0xc), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_LINK)[1], vec![this, 0x100]);
        // Without a Havok object only the base runs.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_5ba0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, REF_OBJECT_LINK).len(), 3);
    }

    #[test]
    fn set_av_object_removes_a_present_key_and_adds_the_object() {
        let mut e = logging_engine();
        quiet(&mut e, &[REMOVE_PROPERTY, ADD_PROPERTY]);
        let this = world_object_00c85c80(&mut e, &[0x10, 0x20]);
        let havok = e.mem.u32(this.addr() + 8);
        e.call(0x00c8_5c10, &args![this, 0x99u32, 0x20u32]);
        let removed = calls_to(&e, REMOVE_PROPERTY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (havok, 0x20));
        assert_eq!(calls_to(&e, ADD_PROPERTY), vec![vec![havok, 0x20, 0x99, 0]]);
        // A key that is not there is not removed; no object adds nothing.
        e.call(0x00c8_5c10, &args![this, 0u32, 0x30u32]);
        assert_eq!(calls_to(&e, REMOVE_PROPERTY).len(), 1);
        assert_eq!(calls_to(&e, ADD_PROPERTY).len(), 1);
        // A null key means the default key.
        e.call(0x00c8_5c10, &args![this, 0x98u32, 0u32]);
        assert_eq!(
            calls_to(&e, ADD_PROPERTY)[1],
            vec![havok, DEFAULT_KEY_011F4280, 0x98, 0]
        );
        // No Havok object: nothing.
        e.mem.set_u32(this.addr() + 8, 0);
        e.call(0x00c8_5c10, &args![this, 0x98u32, 0x20u32]);
        assert_eq!(calls_to(&e, ADD_PROPERTY).len(), 2);
    }

    #[test]
    fn filter_word_setters_change_the_upper_half_and_bit_14() {
        let mut e = logging_engine();
        quiet(&mut e, &[0x0900_00c4]);
        let (this, havok, _) = chain(&mut e, &[(0xc4, 0x0900_00c4)], 0);
        e.mem.set_u32(havok + 0x2c, 0xaaaa_5555);
        e.call(0x00c8_5ce0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0x1234_5555);
        // Only the low 16 bits of the argument count.
        e.call(0x00c8_5ce0, &args![this, 0x1_2345u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0x2345_5555);
        assert_eq!(calls_to(&e, 0x0900_00c4).len(), 2);
        e.mem.set_u32(havok + 0x2c, 0);
        e.call(0x00c8_5d20, &args![this, 0u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0x4000);
        e.call(0x00c8_5d20, &args![this, 1u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0);
        // Only the low byte of the argument is the flag.
        e.call(0x00c8_5d20, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0x4000);
        assert_eq!(calls_to(&e, 0x0900_00c4).len(), 5);
        // Without a Havok object the word reads as 0 and nothing is stored.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_5ce0, &args![this, 7u32]);
        assert_eq!(e.mem.u32(havok + 0x2c), 0x4000);
        assert_eq!(calls_to(&e, 0x0900_00c4).len(), 6);
    }

    /// The doubles `CreateDisplayGeometry` calls; `fill` records the vector it is given and
    /// the target's slot 0xD4 records `(node, vector words)`.
    type Log = Rc<RefCell<Vec<Vec<u32>>>>;
    fn display_fixture() -> (Engine, u32, u32, Log) {
        let mut e = logging_engine();
        e.map(EMPTY_STRING_HANDLE, 4);
        e.mem.set_u32(EMPTY_STRING_HANDLE, 0x5000);
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(DISPLAY_NODE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(STRING_CREATE, |_, _| Ret {
            eax: 0x6008,
            ..Ret::default()
        });
        e.register(DISPLAY_NODE_SET_NAME, |e, a| {
            e.mem.set_u32(a[0] + 8, 1);
            Ret::default()
        });
        quiet(&mut e, &[INTERLOCKED_DECREMENT]);
        let seen: Log = Rc::default();
        let log = seen.clone();
        e.register_double(FILTER_COLOR, move |e, a| {
            let words = (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push(words);
            e.mem.set_u32(a[1], 0x1111);
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(0x0900_00d4, move |e, a| {
            let mut words = vec![a[1]];
            words.extend((0..3).map(|i| e.mem.u32(a[2] + 4 * i)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        let target = object_with_slots(&mut e, &[(0xd4, 0x0900_00d4)]);
        let (this, havok, _) = chain(&mut e, &[], target);
        e.mem.set_u32(havok + 0x2c, 0x42);
        (e, this, target, seen)
    }

    #[test]
    fn create_display_geometry_names_a_new_node_and_fills_the_color() {
        let (mut e, this, target, seen) = display_fixture();
        let node = e.call(0x00c8_5d70, &args![this, 0u32]).u32();
        assert_ne!(node, 0);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![DISPLAY_NODE_SIZE]]);
        assert_eq!(calls_to(&e, DISPLAY_NODE_CONSTRUCT), vec![vec![node, 0]]);
        assert_eq!(calls_to(&e, STRING_CREATE), vec![vec![DISPLAY_NODE_NAME]]);
        assert_eq!(calls_to(&e, DISPLAY_NODE_SET_NAME)[0][0], node);
        // The name handle is not the empty string's: its count (8 bytes before) is dropped.
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x6000]]);
        assert_eq!(calls_to(&e, FILTER_COLOR)[0][0], 0x42);
        let seen = seen.borrow();
        assert_eq!(seen[0], vec![0, 0, 0]);
        assert_eq!(seen[1], vec![node, 0x1111, 0, 0]);
        assert_eq!(calls_to(&e, 0x0900_00d4)[0][0], target);
    }

    #[test]
    fn create_display_geometry_keeps_a_named_node_and_the_empty_name_handle() {
        let (mut e, this, _, seen) = display_fixture();
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node + 8, 5);
        assert_eq!(e.call(0x00c8_5d70, &args![this, node]).u32(), node);
        assert!(calls_to(&e, STRING_CREATE).is_empty());
        assert_eq!(seen.borrow().len(), 2);
        // An unnamed node whose new name is the empty string's handle keeps its count.
        let (mut e, this, _, _) = display_fixture();
        e.register(STRING_CREATE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        let node = e.mem.alloc(0x20);
        e.call(0x00c8_5d70, &args![this, node]);
        assert_eq!(calls_to(&e, DISPLAY_NODE_SET_NAME).len(), 1);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        // No target: the argument comes back untouched.
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x00c8_5d70, &args![this, 0x1234u32]).u32(), 0x1234);
        assert_eq!(calls_to(&e, FILTER_COLOR).len(), 1);
    }

    #[test]
    fn destroy_display_geometry_runs_slot_0xd8_of_the_linked_object() {
        let mut e = logging_engine();
        quiet(&mut e, &[0x0900_00d8]);
        let linked = object_with_slots(&mut e, &[(0xd8, 0x0900_00d8)]);
        let (this, havok, _) = chain(&mut e, &[], linked);
        e.call(0x00c8_5ec0, &args![this]);
        assert_eq!(calls_to(&e, 0x0900_00d8), vec![vec![linked]]);
        e.mem.set_u32(havok + 0x10, 0);
        e.call(0x00c8_5ec0, &args![this]);
        e.call(0x00c8_5ec0, &args![0u32]);
        assert_eq!(calls_to(&e, 0x0900_00d8).len(), 1);
    }

    #[test]
    fn get_property_stores_the_value_as_a_sign_extended_64_bit_word() {
        let mut e = logging_engine();
        e.register(GET_PROPERTY, |e, a| {
            e.mem.set_u32(a[1], a[2]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(8);
        let object = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c8_5ef0, &args![out, object, 5u32]).u32(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (5, 0));
        assert_eq!(calls_to(&e, GET_PROPERTY)[0][0], object);
        assert_eq!(&calls_to(&e, GET_PROPERTY)[0][2..], &[5, 0]);
        e.call(0x00c8_5ef0, &args![out, object, 0xffff_fff0u32]);
        assert_eq!(
            (e.mem.u32(out), e.mem.u32(out + 4)),
            (0xffff_fff0, u32::MAX)
        );
        // No object: 0 and no call.
        e.call(0x00c8_5ef0, &args![out, 0u32, 5u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
        assert_eq!(calls_to(&e, GET_PROPERTY).len(), 2);
    }

    /// The Havok allocator route with a slot 0x10 double that logs `(pointer, size)`.
    fn array_free_fixture() -> Engine {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let vtable = e.mem.u32(allocator);
        e.mem.set_u32(vtable + 0x10, 0x0900_0f10);
        quiet(&mut e, &[0x0900_0f10]);
        e
    }

    /// A stream with the flag bytes and the version word `LoadBinary` and `SaveBinary` read
    /// and a vtable whose slot 0x30 is the fake `0900_0030`.
    fn stream_with(e: &mut Engine, native_load: u8, native_save: u8, version: u32) -> u32 {
        let stream = e.mem.alloc(0x200);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x30, 0x0900_0030);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u8(stream + 0x1e5, native_load);
        e.mem.set_u8(stream + 0x1e4, native_save);
        e.mem.set_u32(stream + 4, version);
        stream
    }

    /// A cinfo with an array of `capacity` words at `array` and the first word `first`.
    fn cinfo_with(e: &mut Engine, first: u32, capacity: u32, array: u32) -> u32 {
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(cinfo, first);
        e.mem.set_u32(cinfo + 0xc, array);
        e.mem.set_u32(cinfo + 0x10, 3);
        e.mem.set_u32(cinfo + 0x14, capacity);
        cinfo
    }

    fn load_fixture(first: u32, capacity: u32) -> (Engine, u32, u32) {
        let mut e = array_free_fixture();
        quiet(
            &mut e,
            &[STREAM_PREPARE, SERIALIZABLE_LOAD_BINARY, SWAP_HK_ARRAY],
        );
        let cinfo = cinfo_with(&mut e, first, capacity, 0x1111);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        e.register(0x0900_00a8, |_, _| Ret {
            eax: 0x3333,
            ..Ret::default()
        });
        let this = object_with_slots(&mut e, &[(0xb4, 0x0900_00b4), (0xa8, 0x0900_00a8)]);
        (e, this, cinfo)
    }

    #[test]
    fn load_binary_frees_the_array_and_fixes_the_flag_word() {
        // Native stream, version 9: only the array is freed.
        let (mut e, this, cinfo) = load_fixture(0x4028, 4);
        let stream = stream_with(&mut e, 1, 1, 9);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_5f20, &args![this, stream]);
        assert_eq!(calls_to(&e, STREAM_PREPARE), vec![vec![stream]]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert!(calls_to(&e, SWAP_HK_ARRAY).is_empty());
        assert_eq!(calls_to(&e, 0x0900_0f10)[0][1..], [0x1111, 32]);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(cinfo), 0x4028);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // A foreign stream of version 8: the array is swapped, the first word byte swapped,
        // bit 5 becomes bit 15 and the word type 8 loses bit 14.
        let (mut e, this, cinfo) = load_fixture(0x2840_0000, 0x8000_0000);
        let stream = stream_with(&mut e, 0, 1, 8);
        e.call(0x00c8_5f20, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SWAP_HK_ARRAY),
            vec![vec![cinfo, 0x3333, cinfo + 0xc]]
        );
        assert_eq!(e.mem.u32(cinfo), 0x8008);
        // A sign bit in the capacity word means the array is not freed.
        assert!(calls_to(&e, 0x0900_0f10).is_empty());
        // Word type 0x1D loses bit 14 as well, whatever the version.
        let (mut e, this, cinfo) = load_fixture(0x401d, 0);
        let stream = stream_with(&mut e, 1, 1, 12);
        e.call(0x00c8_5f20, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo), 0x001d);
        assert_eq!(calls_to(&e, 0x0900_0f10)[0][1..], [0x1111, 0]);
    }

    #[test]
    fn save_binary_swaps_for_a_foreign_stream_and_writes_the_linked_shape() {
        let mut e = array_free_fixture();
        quiet(
            &mut e,
            &[
                SERIALIZABLE_SAVE_BINARY,
                SWAP_HK_ARRAY,
                0x0900_0030,
                0x0900_00a4,
            ],
        );
        let cinfo = cinfo_with(&mut e, 0x0000_0028, 4, 0x1111);
        let shape_holder = e.mem.alloc(0x20);
        e.mem.set_u32(shape_holder + 8, 0x6666);
        e.mem.set_u32(cinfo + 4, shape_holder);
        e.register_double(0x0900_00b4, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: cinfo,
                ..Ret::default()
            }
        });
        e.register(0x0900_00a8, |_, _| Ret {
            eax: 0x3333,
            ..Ret::default()
        });
        let this = object_with_slots(
            &mut e,
            &[
                (0xb4, 0x0900_00b4),
                (0xa8, 0x0900_00a8),
                (0xa4, 0x0900_00a4),
            ],
        );
        let stream = stream_with(&mut e, 1, 0, 9);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_6060, &args![this, stream]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|call| call.0)
            .filter(|&address| address != MEMORY_ROUTER && address != 0x00c8_6060)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0900_00b4,
                0x0900_00a8,
                SWAP_HK_ARRAY,
                0x0900_0f10,
                0x0900_0030,
                SERIALIZABLE_SAVE_BINARY,
                0x0900_00a4
            ]
        );
        assert_eq!(
            calls_to(&e, SWAP_HK_ARRAY),
            vec![vec![cinfo, 0x3333, cinfo + 0xc]]
        );
        assert_eq!(e.mem.u32(cinfo), 0x2800_0000);
        assert_eq!(calls_to(&e, 0x0900_0f10)[0][1..], [0x1111, 32]);
        assert_eq!(calls_to(&e, 0x0900_0030), vec![vec![stream, 0x6666]]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        // The created flag the cinfo routine set is handed to slot 0xA4, as a whole word.
        assert_eq!(calls_to(&e, 0x0900_00a4), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // Native stream (byte 1) and no object in the cinfo: nothing swapped, 0 written.
        e.mem.set_u8(stream + 0x1e4, 1);
        e.mem.set_u32(cinfo + 4, 0);
        e.mem.set_u32(cinfo + 0x14, 4);
        e.call(0x00c8_6060, &args![this, stream]);
        assert_eq!(calls_to(&e, SWAP_HK_ARRAY).len(), 1);
        assert_eq!(calls_to(&e, 0x0900_0030)[1], vec![stream, 0]);
    }

    #[test]
    fn get_viewer_strings_lists_the_class_the_filter_and_the_linked_object() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        quiet(&mut e, &[SERIALIZABLE_GET_VIEWER_STRINGS, FORMAT_TEXT]);
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, a| Ret {
            eax: 0x9000 + a[0],
            ..Ret::default()
        });
        let seen: Log = Rc::default();
        let log = seen.clone();
        e.register_double(FILTER_VIEWER_STRINGS, move |e, a| {
            log.borrow_mut().push(vec![e.mem.u32(a[0]), a[1]]);
            Ret::default()
        });
        e.map(CLASS_NAME_POINTER, 4);
        e.mem.set_u32(CLASS_NAME_POINTER, 0x1234);
        let rtti = e.mem.alloc(0x10);
        e.mem.set_u32(rtti, 0x4242);
        e.register_double(0x0900_0008, move |_, _| Ret {
            eax: rtti,
            ..Ret::default()
        });
        let linked = object_with_slots(&mut e, &[(8, 0x0900_0008)]);
        let (this, havok, _) = chain(&mut e, &[], linked);
        e.mem.set_u32(havok + 0x2c, 0x77);
        let strings = strings_header(&mut e, 0, 8, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00c8_6190, &args![this, strings]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(*seen.borrow(), vec![vec![0x77, strings]]);
        let format = calls_to(&e, FORMAT_TEXT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [0x80, LINKED_FORMAT, 0x4242, linked]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_TEXT),
            vec![vec![LINKED_NAME, format[0][0]]]
        );
        assert_eq!(
            pushed_texts(&e, strings, 0, 2),
            vec![0x7000 + 0x1234, 0x9000 + LINKED_NAME]
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        // No linked object: the class and the filter only.
        e.mem.set_u32(this + 8, 0);
        let strings = strings_header(&mut e, 0, 8, 2);
        e.call(0x00c8_6190, &args![this, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 1);
        assert_eq!(seen.borrow()[1], vec![0, strings]);
    }

    #[test]
    fn gethk_world_rw_follows_the_havok_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let havok = e.mem.alloc(0x20);
        e.mem.set_u32(havok + 8, 0x4321);
        assert_eq!(e.call(0x00c8_7a50, &args![this]).u32(), 0);
        e.mem.set_u32(this + 8, havok);
        assert_eq!(e.call(0x00c8_7a50, &args![this]).u32(), 0x4321);
        assert_eq!(e.call(0x00c8_7a50, &args![0u32]).u32(), 0);
    }

    #[test]
    fn register_streamables_answers_the_base_registration() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c85ac0, &args![this, 0x5000u32]).u32(), 1);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| Ret::default());
        assert_eq!(e.call(0x00c85ac0, &args![this, 0x5000u32]).u32(), 0);
    }
}
