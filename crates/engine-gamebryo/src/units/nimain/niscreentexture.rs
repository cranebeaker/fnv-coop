//! `NiMain/niscreentexture.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiScreenTexture::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_TEXTURE_RTTI: u32 = 0x011f_5ee0;

// Translated from 00a955f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_texture_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_TEXTURE_RTTI)
}

// Translated from 00a95740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a95590`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x20 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_screen_texture_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_5590;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x20u32]);
    }
    this
}

layout! {
    /// `NiScreenTexture` (Xbox PDB), 0x20 bytes; the PC offsets match.
    pub struct NiScreenTexture: 0x20 {
        /// `m_kScreenRects` (Xbox PDB), an `NiTPrimitiveSet<ScreenRect>`: its base
        /// (`m_pBase`), allocated count and used count.
        0x08 m_kScreenRects: u32,
        0x0C m_uiRectsAlloced: u32,
        0x10 m_uiRectsUsed: u32,
        /// `m_spTexProp` (Xbox PDB): the `NiTexturingProperty`.
        0x14 m_spTexProp: u32,
        /// `m_usDirtyFlags` (Xbox PDB).
        0x18 m_usDirtyFlags: u16,
        /// `m_pkBuffData` (Xbox PDB).
        0x1C m_pkBuffData: u32,
    }
}

/// The size of one `NiScreenTexture::ScreenRect` (Xbox PDB): six 16-bit fields
/// (`m_sPixTop`, `m_sPixLeft`, `m_usPixWidth`, `m_usPixHeight`, `m_usTexTop`, `m_usTexLeft`)
/// and the `NiColorA` `m_kColor` at +0xc.
const SCREEN_RECT_SIZE: u32 = 0x1c;
/// `NiScreenTexture`'s vtable (its RTTI names `NiScreenTexture`).
const NI_SCREEN_TEXTURE_VTABLE: u32 = 0x010a_048c;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1070` (`cdecl`, one word) and `00aa10f0` (`cdecl`, one word): the Ni allocation
/// and free.
const NEW_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `00a5d3a0` (`thiscall`, no stack word): the `NiObject` constructor; `00a5d3d0`: its
/// destructor body.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
/// `011f4748`: the renderer global; its virtual +0xfc is told about a screen texture being
/// destroyed.
const RENDERER_GLOBAL: u32 = 0x011f_4748;
/// The import slots of `InterlockedIncrement` and `InterlockedDecrement`; `DeleteThis` is
/// virtual +4.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiObject::RegisterStreamables` and `NiObject::IsEqual` (Xbox PDB, `thiscall`, one word).
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `00a89af0` (`thiscall`, the stream; a 3-byte body the `LinkObject`s and `SaveBinary`s
/// call first: the `NiObject` base's).
const OBJECT_STREAM_BASE: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (Xbox PDB, `thiscall`, no stack word) and `00a645b0`
/// (`thiscall` on the stream, no stack word): reads the next link id.
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiColorA::operator_` (engine map, `004a4c90`; `thiscall`, the other colour): the exe
/// treats a nonzero answer as the colours differing.
const COLOR_COMPARE: u32 = 0x004a_4c90;
/// `NiObject::LoadBinary` (Xbox PDB, `thiscall`, the stream).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiColorA::LoadBinary` and `NiColorA::SaveBinary` (Xbox PDB, `thiscall` on the colour,
/// the stream).
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
/// `NiStream`'s input (+0x24c, read function at +8) and output (+0x250, write function at
/// +0xc); a file's position is at +4.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;

// Translated from 00a954f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::RegisterStreamables` (Xbox PDB): false when `NiObject`'s refuses;
/// otherwise the texturing property registers itself (virtual +0x54, the stream; called
/// without a null check, as the exe does) and the answer is true.
pub fn ni_screen_texture_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let property = e.mem.u32(this.addr() + 0x14);
    e.vcall(property, 0x54, &args![stream]);
    true
}

// Translated from 00a95520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the screen-rect set, one word): by its body
/// `NiTPrimitiveSet<ScreenRect>`'s reallocation. When `size` differs from the allocated
/// count (+4): a new array of `size` rects (`size * 0x1c` bytes from `00aa1070`; null for
/// 0) gets the used rects (+8, all of them, as the exe copies them, whatever `size`); the
/// old array is freed (`00aa10f0`) and the new one and `size` stored.
pub fn fn_00a95520(e: &mut Engine, this: Ptr, size: u32) {
    let set = this.addr();
    if size == e.mem.u32(set + 4) {
        return;
    }
    let array = if size == 0 {
        0
    } else {
        let array = e
            .call(NEW_BLOCK, &args![size.wrapping_mul(SCREEN_RECT_SIZE)])
            .u32();
        let mut i = 0u32;
        let mut offset = 0u32;
        while i < e.mem.u32(set + 8) {
            let from = e.mem.u32(set).wrapping_add(offset);
            for word in 0..7 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(array.wrapping_add(offset) + 4 * word, value);
            }
            i += 1;
            offset += SCREEN_RECT_SIZE;
        }
        array
    };
    let old = e.mem.u32(set);
    e.call(FREE_BLOCK, &args![old]);
    e.mem.set_u32(set, array);
    e.mem.set_u32(set + 4, size);
}

// Translated from 00a95590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `NiScreenTexture`'s destructor
/// body. Sets the class's vtable, tells the renderer (`011f4748`, when set; its virtual
/// +0xfc with `this`), releases the texturing property (+0x14; `InterlockedDecrement`, at
/// zero its virtual +4), frees the rect array (`00aa10f0`) and continues in `NiObject`'s
/// destructor body (`00a5d3d0`, a `JMP`; its result is passed on).
pub fn fn_00a95590(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), NI_SCREEN_TEXTURE_VTABLE);
    let renderer = e.mem.u32(RENDERER_GLOBAL);
    if renderer != 0 {
        e.vcall(renderer, 0xfc, &args![this]);
    }
    let property = e.mem.u32(this.addr() + 0x14);
    if property != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![property.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(property, 4, &args![]);
        }
    }
    let rects = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![rects]);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]).u32()
}

// Translated from 00a95600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::LinkObject` (Xbox PDB): the base's link step (`00a89af0`), then the
/// texturing property (+0x14) becomes the stream's object for the next link id
/// (`GetObjectFromLinkID`), assigned as a counted pointer.
pub fn ni_screen_texture_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    let property = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    crate::units::nimain::nimainsdm::assign_ref_counted(e, this.addr() + 0x14, property);
}

// Translated from 00a95660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::IsEqual` (Xbox PDB): `NiObject::IsEqual` and the same used rect count
/// (+0x10), then for each rect pair: when none of the six 16-bit fields is equal and the
/// colour comparison (`004a4c90`, this rect's colour against the other's) answers nonzero,
/// false (a rect with any equal field is passed over, as the exe's tests do). Then the
/// texturing properties' `IsEqual` (virtual +0x5c, no null check) decides.
pub fn ni_screen_texture_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.mem.u32(this.addr() + 0x10);
    if count != e.mem.u32(other.addr() + 0x10) {
        return false;
    }
    if count != 0 {
        let mine = e.mem.u32(this.addr() + 8);
        let mut theirs = e.mem.u32(other.addr() + 8);
        let gap = mine.wrapping_sub(theirs);
        let mut cursor = mine.wrapping_add(4);
        for _ in 0..count {
            let all_differ = e.mem.u16(gap.wrapping_add(theirs)) != e.mem.u16(theirs)
                && e.mem.u16(cursor.wrapping_sub(2)) != e.mem.u16(theirs.wrapping_add(2))
                && e.mem.u16(cursor) != e.mem.u16(theirs.wrapping_add(4))
                && e.mem.u16(cursor.wrapping_add(2)) != e.mem.u16(theirs.wrapping_add(6))
                && e.mem.u16(cursor.wrapping_add(4)) != e.mem.u16(theirs.wrapping_add(8))
                && e.mem.u16(cursor.wrapping_add(6)) != e.mem.u16(theirs.wrapping_add(0xa));
            if all_differ
                && e.call(
                    COLOR_COMPARE,
                    &args![cursor.wrapping_add(8), theirs.wrapping_add(0xc)],
                )
                .bool()
            {
                return false;
            }
            cursor = cursor.wrapping_add(SCREEN_RECT_SIZE);
            theirs = theirs.wrapping_add(SCREEN_RECT_SIZE);
        }
    }
    let property = e.mem.u32(this.addr() + 0x14);
    let other_property = e.mem.u32(other.addr() + 0x14);
    e.vcall(property, 0x5c, &args![other_property]).bool()
}

/// Appends one rect to the screen-rect set at `set` (`m_pBase`, allocated, used): when the
/// used count has reached the allocated one the set is reallocated ([`fn_00a95520`]) to
/// twice the allocated count (1 when it is 0), then the seven words at `rect` are copied to
/// the next slot and the used count grows.
fn append_rect(e: &mut Engine, set: u32, rect: u32) {
    let allocated = e.mem.u32(set + 4);
    if e.mem.u32(set + 8) == allocated {
        let size = if allocated == 0 { 1 } else { allocated * 2 };
        fn_00a95520(e, Ptr::new(set), size);
    }
    let used = e.mem.u32(set + 8);
    let to = e
        .mem
        .u32(set)
        .wrapping_add(used.wrapping_mul(SCREEN_RECT_SIZE));
    for word in 0..7 {
        let value = e.mem.u32(rect + 4 * word);
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
    e.mem.set_u32(set + 8, used.wrapping_add(1));
}

// Translated from 00a95760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (clone, cloning process)): by its body
/// `NiScreenTexture::CopyMembers`. Records the clone (`00a5d450`), appends a copy of every
/// used rect to the clone's set (each copied to a stack local first), assigns the
/// texturing property as a counted pointer and copies the dirty flags (+0x18).
pub fn fn_00a95760(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(OBJECT_RECORD_CLONE, &args![this, clone, cloning_process]);
    let set = clone.addr().wrapping_add(8);
    e.with_stack(SCREEN_RECT_SIZE, |e, rect| {
        let mut i = 0u32;
        let mut offset = 0u32;
        while i < e.mem.u32(this.addr() + 0x10) {
            let from = e.mem.u32(this.addr() + 8).wrapping_add(offset);
            for word in 0..7 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(rect.addr() + 4 * word, value);
            }
            offset += SCREEN_RECT_SIZE;
            append_rect(e, set, rect.addr());
            i += 1;
        }
    });
    // The counted assignment, with this one's property read again after the release.
    let slot = clone.addr().wrapping_add(0x14);
    let old = e.mem.u32(slot);
    if old != e.mem.u32(this.addr() + 0x14) {
        if old != 0 {
            let count = e
                .call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                .u32();
            if count == 0 {
                e.vcall(old, 4, &args![]);
            }
        }
        let property = e.mem.u32(this.addr() + 0x14);
        e.mem.set_u32(slot, property);
        if property != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![property.wrapping_add(4)]);
        }
    }
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.addr().wrapping_add(0x18), flags);
}

/// A new screen texture's fields: the `NiObject` constructor, the vtable, an empty rect
/// set, no texturing property, no dirty flags and no buffer data.
fn construct(e: &mut Engine, block: u32) {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, NI_SCREEN_TEXTURE_VTABLE);
    for offset in [8u32, 0xc, 0x10, 0x14] {
        e.mem.set_u32(block + offset, 0);
    }
}

// Translated from 00a958b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiScreenTexture`, `0109f044`): a new, empty screen texture (0x20
/// bytes from `00aa13e0`), or null when the allocation fails.
pub fn fn_00a958b0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct(e, block);
    e.mem.set_u32(block + 0x1c, 0);
    e.mem.set_u16(block + 0x18, 0);
    Ptr::new(block)
}

/// One read or write through the stream's input (read function at +8) or output (write
/// function at +0xc): `function(file, at, size, count, 1)` with the word at `count` set to
/// `count_value` first, the answer added to the file's position.
fn stream_transfer(
    e: &mut Engine,
    stream: Ptr,
    write: bool,
    at: u32,
    size: u32,
    count: u32,
    count_value: u32,
) {
    let (file_slot, function_slot) = if write {
        (STREAM_OUTPUT, 0xc)
    } else {
        (STREAM_INPUT, 8)
    };
    let file = e.mem.u32(stream.addr().wrapping_add(file_slot));
    let function = e.mem.u32(file.wrapping_add(function_slot));
    e.mem.set_u32(count, count_value);
    let done = e.call(function, &args![file, at, size, count, 1u32]).u32();
    let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(file.wrapping_add(4), position);
}

// Translated from 00a958f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the rect count (4
/// bytes into a local, the byte-count word being the stream argument's stack word), then
/// for each rect: a stack local whose colour is zeroed, its six 16-bit fields read one by
/// one (byte count 2, a second local), its colour loaded (`NiColorA::LoadBinary`), and the
/// rect appended to the set. Then the texturing property's link id (`00a645b0`).
pub fn ni_screen_texture_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0x28, |e, frame| {
        let count_local = frame.addr();
        let byte_count = count_local + 4;
        let rect = count_local + 8;
        let argument = count_local + 0x24;
        stream_transfer(e, stream, false, count_local, 4, argument, 4);
        let set = this.addr() + 8;
        let mut i = 0u32;
        while i < e.mem.u32(count_local) {
            for offset in [0xcu32, 0x10, 0x14, 0x18] {
                e.mem.set_u32(rect + offset, x87_store32(0.0));
            }
            for field in 0..6 {
                stream_transfer(e, stream, false, rect + 2 * field, 2, byte_count, 2);
            }
            e.call(COLOR_LOAD_BINARY, &args![rect + 0xc, stream]);
            append_rect(e, set, rect);
            i += 1;
        }
    });
    e.call(READ_LINK_ID, &args![stream]);
}

// Translated from 00a95a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::SaveBinary` (Xbox PDB): the base's step (`00a89af0`), the used rect
/// count copied to a local and written (4 bytes; the byte-count word is the stream
/// argument's stack word), then for each of that many rects its six 16-bit fields (byte
/// count 2, a local) and its colour, copied to a stack local, through
/// `NiColorA::SaveBinary`. Then the texturing property's link id (the stream's virtual
/// +0x30).
pub fn ni_screen_texture_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_STREAM_BASE, &args![this, stream]);
    e.with_stack(0x24, |e, frame| {
        let count_local = frame.addr();
        let byte_count = count_local + 0xc;
        let color = count_local + 0x10;
        let argument = count_local + 0x20;
        let used = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(count_local, used);
        stream_transfer(e, stream, true, count_local, 4, argument, 4);
        let mut i = 0u32;
        let mut offset = 0u32;
        while i < e.mem.u32(count_local) {
            let rect = e.mem.u32(this.addr() + 8).wrapping_add(offset);
            for field in 0..6 {
                stream_transfer(e, stream, true, rect + 2 * field, 2, byte_count, 2);
            }
            for word in 0..4 {
                let value = e.mem.u32(rect + 0xc + 4 * word);
                e.mem.set_u32(color + 4 * word, value);
            }
            e.call(COLOR_SAVE_BINARY, &args![color, stream]);
            offset += SCREEN_RECT_SIZE;
            i += 1;
        }
    });
    let property = e.mem.u32(this.addr() + 0x14);
    e.vcall(stream.addr(), 0x30, &args![property]);
}

// Translated from 00a95c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenTexture::CreateClone` (Xbox PDB): a new, empty screen texture (0x20 bytes from
/// `00aa13e0`) given this one's members ([`fn_00a95760`]). When the allocation fails the
/// member copy still runs on null (and stops at the clone's first field the exe touches)
/// and the answer is null.
pub fn ni_screen_texture_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    if block == 0 {
        fn_00a95760(e, this, Ptr::new(0), cloning_process);
        return Ptr::new(0);
    }
    construct(e, block);
    e.mem.set_u16(block + 0x18, 0);
    e.mem.set_u32(block + 0x1c, 0);
    fn_00a95760(e, this, Ptr::new(block), cloning_process);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_54f0,
            ni_screen_texture_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a9_5520, fn_00a95520(Ptr, u32)),
        entry!(0x00a9_5590, fn_00a95590(Ptr) -> u32),
        entry!(0x00a9_5600, ni_screen_texture_link_object(Ptr, Ptr)),
        entry!(0x00a9_5660, ni_screen_texture_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_5760, fn_00a95760(Ptr, Ptr, Ptr)),
        entry!(0x00a9_58b0, fn_00a958b0() -> Ptr),
        entry!(0x00a9_58f0, ni_screen_texture_load_binary(Ptr, Ptr)),
        entry!(0x00a9_5a90, ni_screen_texture_save_binary(Ptr, Ptr)),
        entry!(
            0x00a9_5c10,
            ni_screen_texture_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a955f0, ni_screen_texture_get_rtti() -> Ptr),
        entry!(0x00a95740, ni_screen_texture_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_texture_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a955f0, &args![]).u32(), NI_SCREEN_TEXTURE_RTTI);
    }

    #[test]
    fn ni_screen_texture_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_5590, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a95740, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a95740, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x20]]);
        assert_eq!(calls_to(&e, 0x00a9_5590), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A counted object (count `count`) whose vtable has `slot_target` at `slot`.
    fn counted(e: &mut Engine, count: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A screen texture with `rects` rects (rect `i` holds the words `0x100 * (i + 1) + w`).
    fn texture(e: &mut Engine, rects: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        let array = e.mem.alloc(SCREEN_RECT_SIZE * rects.max(1));
        for i in 0..rects {
            for w in 0..7 {
                e.mem
                    .set_u32(array + SCREEN_RECT_SIZE * i + 4 * w, 0x100 * (i + 1) + w);
            }
        }
        e.mem.set_u32(this + 8, array);
        e.mem.set_u32(this + 0xc, rects);
        e.mem.set_u32(this + 0x10, rects);
        this
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.register(NEW_BLOCK, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(OBJECT_RECORD_CLONE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn register_streamables_registers_the_property() {
        let mut e = engine();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        e.register(0x0300_1000, |_, _| Ret::default());
        let this = texture(&mut e, 0);
        let property = counted(&mut e, 1, &[(0x54, 0x0300_1000)]);
        e.mem.set_u32(this + 0x14, property);
        assert!(!e.call(0x00a9_54f0, &args![this, 0x4444u32]).bool());
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_54f0, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![property, 0x4444]]);
    }

    #[test]
    fn reallocation_copies_the_used_rects() {
        let mut e = engine();
        let this = texture(&mut e, 2);
        let set = this + 8;
        let old = e.mem.u32(set);
        e.call(0x00a9_5520, &args![set, 5u32]);
        assert_eq!(calls_to(&e, NEW_BLOCK), vec![vec![5 * 0x1c]]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![old]]);
        let array = e.mem.u32(set);
        assert_ne!(array, old);
        assert_eq!(e.mem.u32(set + 4), 5);
        assert_eq!(e.mem.u32(array + 0x1c + 0x18), 0x206);
        // The same size does nothing; size 0 frees and stores null.
        e.call(0x00a9_5520, &args![set, 5u32]);
        assert_eq!(calls_to(&e, FREE_BLOCK).len(), 1);
        e.call(0x00a9_5520, &args![set, 0u32]);
        assert_eq!(e.mem.u32(set), 0);
        assert_eq!(e.mem.u32(set + 4), 0);
        assert_eq!(calls_to(&e, NEW_BLOCK).len(), 1);
    }

    #[test]
    fn destructor_body_tells_the_renderer_and_releases() {
        let mut e = engine();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| 0x77u32.into_ret());
        e.register(0x0300_1000, |_, _| Ret::default());
        e.register(0x0300_2000, |_, _| Ret::default());
        let renderer = counted(&mut e, 1, &[(0xfc, 0x0300_1000)]);
        e.mem.set_u32(RENDERER_GLOBAL, renderer);
        let this = texture(&mut e, 1);
        let rects = e.mem.u32(this + 8);
        let property = counted(&mut e, 1, &[(4, 0x0300_2000)]);
        e.mem.set_u32(this + 0x14, property);
        assert_eq!(e.call(0x00a9_5590, &args![this]).u32(), 0x77);
        assert_eq!(e.mem.u32(this), NI_SCREEN_TEXTURE_VTABLE);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![renderer, this]]);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![property]]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![rects]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this]]);
        // Without a renderer or a property only the free and the base run.
        e.mem.set_u32(RENDERER_GLOBAL, 0);
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_5590, &args![this]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(calls_to(&e, FREE_BLOCK).len(), 1);
    }

    #[test]
    fn link_object_assigns_the_linked_property() {
        let mut e = engine();
        e.register(OBJECT_STREAM_BASE, |_, _| Ret::default());
        let property = counted(&mut e, 0, &[]);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| property.into_ret());
        let this = texture(&mut e, 0);
        e.call(0x00a9_5600, &args![this, 0x4444u32]);
        assert_eq!(calls_to(&e, OBJECT_STREAM_BASE), vec![vec![this, 0x4444]]);
        assert_eq!(e.mem.u32(this + 0x14), property);
        assert_eq!(e.mem.u32(property + 4), 1);
    }

    fn equal_engine() -> Engine {
        let mut e = engine();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(COLOR_COMPARE, |_, _| 1u32.into_ret());
        e.register(0x0300_3000, |_, _| 1u32.into_ret());
        e
    }

    fn with_property(e: &mut Engine, this: u32) {
        let property = counted(e, 1, &[(0x5c, 0x0300_3000)]);
        e.mem.set_u32(this + 0x14, property);
    }

    #[test]
    fn is_equal_checks_counts_rects_and_the_property() {
        let mut e = equal_engine();
        let a = texture(&mut e, 2);
        let b = texture(&mut e, 2);
        with_property(&mut e, a);
        e.mem.set_u32(b + 0x14, 0x5555);
        // Rects with equal fields are passed over: the colour is never asked.
        assert!(e.call(0x00a9_5660, &args![a, b]).bool());
        assert!(calls_to(&e, COLOR_COMPARE).is_empty());
        let property = e.mem.u32(a + 0x14);
        assert_eq!(calls_to(&e, 0x0300_3000), vec![vec![property, 0x5555]]);
        // The property decides at the end.
        e.register(0x0300_3000, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00a9_5660, &args![a, b]).bool());
        e.register(0x0300_3000, |_, _| 1u32.into_ret());
        // Different counts, or the base refusing.
        e.mem.set_u32(b + 0x10, 1);
        assert!(!e.call(0x00a9_5660, &args![a, b]).bool());
        e.mem.set_u32(b + 0x10, 2);
        e.register(OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_5660, &args![a, b]).bool());
    }

    #[test]
    fn is_equal_asks_the_colour_only_when_every_field_differs() {
        let mut e = equal_engine();
        let a = texture(&mut e, 2);
        let b = texture(&mut e, 2);
        with_property(&mut e, a);
        // The second rects differ in all six fields.
        let array = e.mem.u32(b + 8);
        for f in 0..6 {
            let v = e.mem.u16(array + 0x1c + 2 * f);
            e.mem.set_u16(array + 0x1c + 2 * f, v ^ 0x8000);
        }
        assert!(!e.call(0x00a9_5660, &args![a, b]).bool());
        let a_array = e.mem.u32(a + 8);
        assert_eq!(
            calls_to(&e, COLOR_COMPARE),
            vec![vec![a_array + 0x1c + 0xc, array + 0x1c + 0xc]]
        );
        // A colour answer of zero lets it go on to the property.
        e.register(COLOR_COMPARE, |_, _| 0x100u32.into_ret());
        assert!(e.call(0x00a9_5660, &args![a, b]).bool());
    }

    #[test]
    fn copy_members_appends_the_rects_and_assigns_the_property() {
        let mut e = engine();
        e.register(0x0300_2000, |_, _| Ret::default());
        let this = texture(&mut e, 3);
        let property = counted(&mut e, 1, &[]);
        e.mem.set_u32(this + 0x14, property);
        e.mem.set_u16(this + 0x18, 0xabcd);
        let clone = e.mem.alloc(0x20);
        let old = counted(&mut e, 1, &[(4, 0x0300_2000)]);
        e.mem.set_u32(clone + 0x14, old);
        e.call(0x00a9_5760, &args![this, clone, 0x4444u32]);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, clone, 0x4444]]
        );
        // The empty set grows to 1, 2, then 4.
        let sizes: Vec<u32> = calls_to(&e, NEW_BLOCK).iter().map(|c| c[0]).collect();
        assert_eq!(sizes, vec![0x1c, 0x38, 0x70]);
        assert_eq!(e.mem.u32(clone + 0xc), 4);
        assert_eq!(e.mem.u32(clone + 0x10), 3);
        let array = e.mem.u32(clone + 8);
        assert_eq!(e.mem.u32(array + 2 * 0x1c + 0x18), 0x306);
        assert_eq!(e.mem.u32(clone + 0x14), property);
        assert_eq!(e.mem.u32(property + 4), 2);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![old]]);
        assert_eq!(e.mem.u16(clone + 0x18), 0xabcd);
    }

    #[test]
    fn creation_and_clone_build_empty_textures() {
        let mut e = engine();
        let object = e.call(0x00a9_58b0, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x20]]);
        assert_eq!(e.mem.u32(object), NI_SCREEN_TEXTURE_VTABLE);
        assert_eq!(e.mem.bytes(object + 8, 0x18), vec![0; 0x18]);
        let this = texture(&mut e, 1);
        let clone = e.call(0x00a9_5c10, &args![this, 0x4444u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SCREEN_TEXTURE_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x10), 1);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, clone, 0x4444]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_58b0, &args![]).u32(), 0);
    }

    type Transfers = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    /// A stream whose input reads the next of `values` into each 4- or 2-byte read and
    /// whose output records each write (address, size, count address, count value).
    fn stream(e: &mut Engine, values: Vec<u32>) -> (u32, Transfers) {
        let log: Transfers = Rc::default();
        let reads = log.clone();
        let mut queue = values.into_iter();
        e.register_double(0x0700_0008, move |e, a| {
            reads.borrow_mut().push((a[1], a[2], a[3], e.mem.u32(a[3])));
            let value = queue.next().unwrap();
            if a[2] == 4 {
                e.mem.set_u32(a[1], value);
            } else {
                e.mem.set_u16(a[1], value as u16);
            }
            a[2].into_ret()
        });
        let writes = log.clone();
        e.register_double(0x0700_000c, move |e, a| {
            writes
                .borrow_mut()
                .push((a[1], a[2], a[3], e.mem.u32(a[3])));
            a[2].into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0700_0008);
        e.mem.set_u32(file + 0xc, 0x0700_000c);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_INPUT, file);
        e.mem.set_u32(stream + STREAM_OUTPUT, file);
        (stream, log)
    }

    #[test]
    fn load_binary_reads_and_appends_each_rect() {
        let mut e = engine();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |e, a| {
            // The colour is zero when it is loaded.
            assert_eq!(e.mem.bytes(a[0], 16), vec![0; 16]);
            e.mem.set_f32(a[0] + 12, 0.5);
            Ret::default()
        });
        let (stream, log) = stream(&mut e, vec![2, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let this = e.mem.alloc(0x20);
        e.call(0x00a9_58f0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let array = e.mem.u32(this + 8);
        let fields: Vec<u16> = (0..6).map(|f| e.mem.u16(array + 0x1c + 2 * f)).collect();
        assert_eq!(fields, vec![7, 8, 9, 10, 11, 12]);
        assert_eq!(e.mem.f32(array + 0x18), 0.5);
        let log = log.borrow();
        assert_eq!((log[0].1, log[0].3), (4, 4));
        assert!(log[1..]
            .iter()
            .all(|r| r.1 == 2 && r.3 == 2 && r.2 == log[1].2));
        assert_ne!(log[0].2, log[1].2);
        assert_eq!(calls_to(&e, COLOR_LOAD_BINARY).len(), 2);
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
    }

    #[test]
    fn save_binary_writes_each_rect_and_the_link() {
        let mut e = engine();
        e.register(OBJECT_STREAM_BASE, |_, _| Ret::default());
        e.register(COLOR_SAVE_BINARY, |e, a| {
            // A copy of the rect colour on the stack.
            assert_eq!(e.mem.u32(a[0]), e.mem.u32(a[0] + 4) - 1);
            Ret::default()
        });
        e.register(0x0300_4000, |_, _| Ret::default());
        let (stream, log) = stream(&mut e, vec![]);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0300_4000);
        e.mem.set_u32(stream, vtable);
        let this = texture(&mut e, 2);
        e.mem.set_u32(this + 0x14, 0x5555);
        e.call(0x00a9_5a90, &args![this, stream]);
        let log = log.borrow();
        assert_eq!(log.len(), 13);
        assert_eq!((log[0].1, log[0].3), (4, 4));
        let array = e.mem.u32(this + 8);
        assert_eq!(log[7].0, array + 0x1c);
        assert_eq!(log[12].0, array + 0x1c + 0xa);
        assert!(log[1..].iter().all(|w| w.1 == 2 && w.3 == 2));
        let colors = calls_to(&e, COLOR_SAVE_BINARY);
        assert_eq!(colors.len(), 2);
        assert_eq!(calls_to(&e, 0x0300_4000), vec![vec![stream, 0x5555]]);
    }
}
