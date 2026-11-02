//! `NiMain/nipalette.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPalette::ms_RTTI` (the class's `NiRTTI` record).
const NI_PALETTE_RTTI: u32 = 0x011f_5c08;

// Translated from 00a8b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_palette_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PALETTE_RTTI)
}

// Translated from 00a8b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a5d6e0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5c08`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_palette_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
    const CLASS_NAME_POINTER: u32 = 0x011f_5c08;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// `NiPalette`'s vtable (`0109f3fc`).
const NI_PALETTE_VTABLE: u32 = 0x0109_f3fc;
/// `NiPalette` (Xbox PDB) fields as the PC code reads them: `m_bAlpha` (+8, a byte),
/// `m_uiEntries` (+0xc), `m_uiRevID` (+0x10), `m_pkPalette` (+0x14, four bytes per entry)
/// and the renderer data (+0x18, deleted through its vtable slot 0), and the links of the
/// list of every palette: previous (+0x1c) and next (+0x20).
const ALPHA: u32 = 0x8;
const ENTRIES: u32 = 0xc;
const REVISION: u32 = 0x10;
const PALETTE: u32 = 0x14;
const RENDERER_DATA: u32 = 0x18;
const PREVIOUS: u32 = 0x1c;
const NEXT: u32 = 0x20;
/// The list of every palette: its critical section (`011f5c80`), head (`011f5c00`) and tail
/// (`011f5c04`).
const LIST_LOCK: u32 = 0x011f_5c80;
const LIST_HEAD: u32 = 0x011f_5c00;
const LIST_TAIL: u32 = 0x011f_5c04;
/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection` (`stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `NiObject`'s constructor (`00a5d3a0`), destructor body (`00a5d3d0`), `IsEqual`
/// (`00a5d470`) and `LoadBinary` (`00a5d610`), and the empty `00a89af0` the palette's
/// `SaveBinary` starts with (all `thiscall`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The memory manager singleton (`011f6238`) with its `Allocate` (`00aa3e40`) and
/// `Deallocate` (`00aa4060`), both `thiscall` with one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The Ni allocator (`00aa13e0`, `cdecl`, the size), the sized delete (`00aa1460`, `cdecl`:
/// pointer, size) and `memcpy` (`00ec44d0`, `cdecl`: destination, source, size).
const NI_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiRenderer::ms_pkRenderer` (`011f4748`), told about a palette through its virtual
/// +0x150 (the palette).
const RENDERER_INSTANCE: u32 = 0x011f_4748;
const RENDERER_PALETTE_SLOT: u32 = 0x150;

/// `count * 4` as the code sizes the entry block: `0xffffffff` when the product overflows
/// (`MUL`, `SETO`, `NEG`, `OR`).
fn entry_bytes(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

fn deallocate(e: &mut Engine, block: u32) {
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// Tells the renderer (when there is one) about the palette (its virtual +0x150).
fn notify_renderer(e: &mut Engine, palette: u32) {
    let renderer = e.mem.u32(RENDERER_INSTANCE);
    if renderer != 0 {
        e.vcall(renderer, RENDERER_PALETTE_SLOT, &args![palette]);
    }
}

// Translated from 00a8af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::IsEqual` (Xbox PDB): `NiObject::IsEqual` (`00a5d470`), the same alpha byte
/// and entry count, then the entries compared as the inlined `memcmp` does. The compiler
/// made its length `entries * 4 != 0` (0 or 1), so at most the first byte is compared.
pub fn ni_palette_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.mem.u8(a + ALPHA) != e.mem.u8(b + ALPHA) {
        return false;
    }
    let count = e.mem.u32(a + ENTRIES);
    if count != e.mem.u32(b + ENTRIES) {
        return false;
    }
    let mut theirs = e.mem.u32(b + PALETTE);
    let mut mine = e.mem.u32(a + PALETTE);
    let mut length = u32::from(count.wrapping_mul(4) != 0);
    while length >= 4 {
        if e.mem.u32(mine) != e.mem.u32(theirs) {
            return false;
        }
        length -= 4;
        theirs = theirs.wrapping_add(4);
        mine = mine.wrapping_add(4);
    }
    for offset in 0..length.min(3) {
        if e.mem.u8(theirs.wrapping_add(offset)) != e.mem.u8(mine.wrapping_add(offset)) {
            return false;
        }
    }
    true
}

// Translated from 00a8afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a palette, no stack words): adds the palette at
/// the tail of the list of every palette (head `011f5c00`, tail `011f5c04`, links +0x1c and
/// +0x20) under its critical section (`011f5c80`).
pub fn fn_00a8afc0(e: &mut Engine, this: Ptr) {
    let palette = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![LIST_LOCK]);
    if e.mem.u32(LIST_HEAD) == 0 {
        e.mem.set_u32(LIST_HEAD, palette);
    }
    let tail = e.mem.u32(LIST_TAIL);
    if tail != 0 {
        e.mem.set_u32(tail + NEXT, palette);
        let tail = e.mem.u32(LIST_TAIL);
        e.mem.set_u32(palette + PREVIOUS, tail);
    } else {
        e.mem.set_u32(palette + PREVIOUS, 0);
    }
    e.mem.set_u32(LIST_TAIL, palette);
    e.mem.set_u32(palette + NEXT, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![LIST_LOCK]);
}

// Translated from 00a8b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a palette, no stack words): takes the palette
/// out of the list of every palette (`fn_00a8afc0`'s list) under its critical section.
pub fn fn_00a8b010(e: &mut Engine, this: Ptr) {
    let palette = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![LIST_LOCK]);
    if e.mem.u32(LIST_HEAD) == palette {
        let next = e.mem.u32(palette + NEXT);
        e.mem.set_u32(LIST_HEAD, next);
    }
    if e.mem.u32(LIST_TAIL) == palette {
        let previous = e.mem.u32(palette + PREVIOUS);
        e.mem.set_u32(LIST_TAIL, previous);
    }
    let previous = e.mem.u32(palette + PREVIOUS);
    if previous != 0 {
        let next = e.mem.u32(palette + NEXT);
        e.mem.set_u32(previous + NEXT, next);
    }
    let next = e.mem.u32(palette + NEXT);
    if next != 0 {
        let previous = e.mem.u32(palette + PREVIOUS);
        e.mem.set_u32(next + PREVIOUS, previous);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![LIST_LOCK]);
}

// Translated from 00a8b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::NiPalette` (Xbox PDB; the copy constructor): `NiObject`'s constructor, the
/// vtable, the alpha byte and entry count of `other`, revision 1, a new entry block copied
/// from `other`'s (its count read again), no renderer data; the renderer is told and the
/// palette added to the list. Answers `this`.
pub fn ni_palette_ni_palette(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (palette, source) = (this.addr(), other.addr());
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(palette, NI_PALETTE_VTABLE);
    let alpha = e.mem.u8(source + ALPHA);
    e.mem.set_u8(palette + ALPHA, alpha);
    let count = e.mem.u32(source + ENTRIES);
    e.mem.set_u32(palette + ENTRIES, count);
    e.mem.set_u32(palette + REVISION, 1);
    let block = allocate(e, entry_bytes(count));
    e.mem.set_u32(palette + PALETTE, block);
    let bytes = e.mem.u32(source + ENTRIES).wrapping_mul(4);
    let entries = e.mem.u32(source + PALETTE);
    e.call(MEMCPY, &args![block, entries, bytes]);
    e.mem.set_u32(palette + RENDERER_DATA, 0);
    notify_renderer(e, palette);
    fn_00a8afc0(e, this);
    this
}

// Translated from 00a8b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (alpha, entry count, entries); by its body the
/// `NiPalette` constructor from entries): `NiObject`'s constructor, the alpha byte (the
/// word's low byte) and count, the vtable, revision 1, a new entry block filled from
/// `entries`, no renderer data; the renderer is told and the palette added to the list.
/// Answers `this`.
pub fn fn_00a8b110(e: &mut Engine, this: Ptr, alpha: u32, count: u32, entries: Ptr) -> Ptr {
    let palette = this.addr();
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u8(palette + ALPHA, alpha as u8);
    e.mem.set_u32(palette + ENTRIES, count);
    e.mem.set_u32(palette, NI_PALETTE_VTABLE);
    e.mem.set_u32(palette + REVISION, 1);
    let block = allocate(e, entry_bytes(count));
    e.mem.set_u32(palette + PALETTE, block);
    let bytes = e.mem.u32(palette + ENTRIES).wrapping_mul(4);
    e.call(MEMCPY, &args![block, entries, bytes]);
    e.mem.set_u32(palette + RENDERER_DATA, 0);
    notify_renderer(e, palette);
    fn_00a8afc0(e, this);
    this
}

// Translated from 00a8b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::operator=` (Xbox PDB): nothing when assigning to itself. Otherwise the
/// renderer data is deleted (its vtable slot 0 with 1) and cleared, the alpha byte copied,
/// the revision set to 1, the entry block replaced by a new one when the counts differ, and
/// `other`'s entries copied in. Answers `this`.
pub fn ni_palette_operator_assign(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (palette, source) = (this.addr(), other.addr());
    if palette == source {
        return this;
    }
    let data = e.mem.u32(palette + RENDERER_DATA);
    if data != 0 {
        e.vcall(data, 0, &args![1u32]);
        e.mem.set_u32(palette + RENDERER_DATA, 0);
    }
    let alpha = e.mem.u8(source + ALPHA);
    let count = e.mem.u32(palette + ENTRIES);
    e.mem.set_u8(palette + ALPHA, alpha);
    e.mem.set_u32(palette + REVISION, 1);
    if count != e.mem.u32(source + ENTRIES) {
        let old = e.mem.u32(palette + PALETTE);
        deallocate(e, old);
        let count = e.mem.u32(source + ENTRIES);
        e.mem.set_u32(palette + ENTRIES, count);
        let block = allocate(e, entry_bytes(count));
        e.mem.set_u32(palette + PALETTE, block);
    }
    let bytes = e.mem.u32(source + ENTRIES).wrapping_mul(4);
    let entries = e.mem.u32(source + PALETTE);
    let block = e.mem.u32(palette + PALETTE);
    e.call(MEMCPY, &args![block, entries, bytes]);
    this
}

// Translated from 00a8b220 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiPalette`): allocates 0x24 bytes (`00aa13e0`) and, when that succeeds,
/// runs `NiObject`'s constructor, sets the vtable, no alpha, no entries, revision 1, no
/// renderer data, and adds the palette to the list. Answers the palette, or null.
pub fn fn_00a8b220(e: &mut Engine) -> Ptr {
    let palette = e.call(NI_NEW, &args![0x24u32]).u32();
    if palette == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![palette]);
    e.mem.set_u32(palette, NI_PALETTE_VTABLE);
    e.mem.set_u8(palette + ALPHA, 0);
    e.mem.set_u32(palette + ENTRIES, 0);
    e.mem.set_u32(palette + PALETTE, 0);
    e.mem.set_u32(palette + REVISION, 1);
    e.mem.set_u32(palette + RENDERER_DATA, 0);
    fn_00a8afc0(e, Ptr::new(palette));
    Ptr::new(palette)
}

// Translated from 00a8b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the entry block given
/// back to the memory manager, the renderer data deleted (slot 0 with 1) when present, the
/// palette taken out of the list (`00a8b010`), `NiObject`'s destructor body, then the sized
/// delete of 0x24 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_palette_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let palette = this.addr();
    let block = e.mem.u32(palette + PALETTE);
    e.mem.set_u32(palette, NI_PALETTE_VTABLE);
    deallocate(e, block);
    let data = e.mem.u32(palette + RENDERER_DATA);
    if data != 0 {
        e.vcall(data, 0, &args![1u32]);
    }
    fn_00a8b010(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x24u32]);
    }
    this
}

/// One read from the stream's input (`NiStream` +0x24c, read function +8, position +4):
/// `size` bytes into `destination`, the component size word at `component` set first.
fn read(e: &mut Engine, stream: Ptr, destination: u32, size: u32, component: u32, unit: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(component, unit);
    let done = e
        .call(function, &args![input, destination, size, component, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

/// One write to the stream's output (`NiStream` +0x250, write function +0xc, position +4).
fn write(e: &mut Engine, stream: Ptr, source: u32, size: u32, component: u32, unit: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(component, unit);
    let done = e
        .call(function, &args![output, source, size, component, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a8b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then from the stream the
/// alpha flag (one byte, stored as `!= 0`), the entry count (4 bytes; a different count
/// replaces the entry block with a new one) and the entries (count times 4 bytes, component
/// size 1); then the renderer is told.
pub fn ni_palette_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let palette = this.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let (count, component, flag) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        read(e, stream, flag, 1, component, 1);
        let alpha = e.mem.u8(flag) != 0;
        e.mem.set_u8(palette + ALPHA, u8::from(alpha));
        read(e, stream, count, 4, component, 4);
        let entries = e.mem.u32(count);
        if entries != e.mem.u32(palette + ENTRIES) {
            let old = e.mem.u32(palette + PALETTE);
            deallocate(e, old);
            e.mem.set_u32(palette + ENTRIES, entries);
            let block = allocate(e, entry_bytes(entries));
            e.mem.set_u32(palette + PALETTE, block);
        }
        let bytes = e.mem.u32(palette + ENTRIES).wrapping_mul(4);
        let block = e.mem.u32(palette + PALETTE);
        read(e, stream, block, bytes, component, 1);
    });
    notify_renderer(e, palette);
}

// Translated from 00a8b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPalette::SaveBinary` (Xbox PDB): the empty base `00a89af0`, then to the stream the
/// alpha byte, the entry count (4 bytes) and the entries (count times 4 bytes, component
/// size 1).
pub fn ni_palette_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let palette = this.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, frame| {
        let (component, byte) = (frame.addr(), frame.addr() + 4);
        let alpha = e.mem.u8(palette + ALPHA);
        e.mem.set_u8(byte, alpha);
        write(e, stream, byte, 1, component, 1);
        write(e, stream, palette + ENTRIES, 4, component, 4);
        let bytes = e.mem.u32(palette + ENTRIES).wrapping_mul(4);
        let block = e.mem.u32(palette + PALETTE);
        write(e, stream, block, bytes, component, 1);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_af40, ni_palette_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a8_afc0, fn_00a8afc0(Ptr)),
        entry!(0x00a8_b010, fn_00a8b010(Ptr)),
        entry!(0x00a8_b080, ni_palette_ni_palette(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_b110, fn_00a8b110(Ptr, u32, u32, Ptr) -> Ptr),
        entry!(0x00a8_b190, ni_palette_operator_assign(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_b220, fn_00a8b220() -> Ptr),
        entry!(
            0x00a8_b270,
            ni_palette_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8_b2c0, ni_palette_load_binary(Ptr, Ptr)),
        entry!(0x00a8_b3c0, ni_palette_save_binary(Ptr, Ptr)),
        entry!(0x00a8b070, ni_palette_get_rtti() -> Ptr),
        entry!(0x00a8b460, ni_palette_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_palette_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8b070, &args![]).u32(), NI_PALETTE_RTTI);
    }

    #[test]
    fn ni_palette_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_d6e0;
        const NAME_POINTER: u32 = 0x011f_5c08;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_palette_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The list globals, the lock imports, a memory manager that hands out fresh blocks,
    /// `memcpy`, and the `NiObject` constructor and destructor as no-ops.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        for addr in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            NI_OBJECT_CONSTRUCT,
            NI_OBJECT_DESTRUCT,
            MEMORY_DEALLOCATE,
            SIZED_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_ALLOCATE, |e, a| {
            let size = if a[1] == u32::MAX { 4 } else { a[1] + 4 };
            e.mem.alloc(size).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e
    }

    fn palette(e: &mut Engine, alpha: u8, entries: &[u32]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u8(p + ALPHA, alpha);
        e.mem.set_u32(p + ENTRIES, entries.len() as u32);
        let block = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (i, v) in entries.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *v);
        }
        e.mem.set_u32(p + PALETTE, block);
        p
    }

    #[test]
    fn is_equal_compares_alpha_count_and_only_the_first_entry_byte() {
        let mut e = engine();
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = palette(&mut e, 1, &[0x1122_3344, 5]);
        let b = palette(&mut e, 1, &[0xff22_3344, 6]);
        let c = palette(&mut e, 1, &[0x1122_3345, 5]);
        let eq = |e: &mut Engine, x: u32, y: u32| e.call(0x00a8_af40, &args![x, y]).bool();
        // Only the first byte (0x44) takes part.
        assert!(eq(&mut e, a, b));
        assert!(!eq(&mut e, a, c));
        let d = palette(&mut e, 0, &[0x1122_3344, 5]);
        assert!(!eq(&mut e, a, d));
        let shorter = palette(&mut e, 1, &[0x1122_3344]);
        assert!(!eq(&mut e, a, shorter));
        let (empty, empty2) = (palette(&mut e, 1, &[]), palette(&mut e, 1, &[]));
        assert!(eq(&mut e, empty, empty2));
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!eq(&mut e, a, a));
    }

    #[test]
    fn the_list_of_every_palette_adds_at_the_tail_and_unlinks() {
        let mut e = engine();
        let a = palette(&mut e, 0, &[]);
        let b = palette(&mut e, 0, &[]);
        let c = palette(&mut e, 0, &[]);
        for p in [a, b, c] {
            e.call(0x00a8_afc0, &args![p]);
        }
        let links = |e: &Engine, p: u32| (e.mem.u32(p + PREVIOUS), e.mem.u32(p + NEXT));
        assert_eq!((e.mem.u32(LIST_HEAD), e.mem.u32(LIST_TAIL)), (a, c));
        assert_eq!(links(&e, a), (0, b));
        assert_eq!(links(&e, b), (a, c));
        assert_eq!(links(&e, c), (b, 0));
        e.call(0x00a8_b010, &args![b]);
        assert_eq!(links(&e, a), (0, c));
        assert_eq!(links(&e, c), (a, 0));
        e.call(0x00a8_b010, &args![a]);
        assert_eq!(e.mem.u32(LIST_HEAD), c);
        e.call(0x00a8_b010, &args![c]);
        assert_eq!((e.mem.u32(LIST_HEAD), e.mem.u32(LIST_TAIL)), (0, 0));
    }

    #[test]
    fn constructors_copy_the_entries_tell_the_renderer_and_join_the_list() {
        let mut e = engine();
        let renderer = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(renderer, vtable);
        e.mem.set_u32(vtable + RENDERER_PALETTE_SLOT, 0x0340_0000);
        e.register(0x0340_0000, |_, _| Ret::default());
        e.mem.set_u32(RENDERER_INSTANCE, renderer);
        e.call_log = Some(vec![]);
        let source = palette(&mut e, 1, &[7, 8, 9]);
        let copy = e.mem.alloc(0x24);
        assert_eq!(e.call(0x00a8_b080, &args![copy, source]).u32(), copy);
        assert_eq!(e.mem.u32(copy), NI_PALETTE_VTABLE);
        assert_eq!(e.mem.u8(copy + ALPHA), 1);
        assert_eq!(e.mem.u32(copy + ENTRIES), 3);
        assert_eq!(e.mem.u32(copy + REVISION), 1);
        let block = e.mem.u32(copy + PALETTE);
        let original = e.mem.u32(source + PALETTE);
        assert_ne!(block, original);
        assert_eq!(e.mem.bytes(block, 12), e.mem.bytes(original, 12));
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        assert_eq!(calls(&e, 0x0340_0000), vec![vec![renderer, copy]]);
        assert_eq!(e.mem.u32(LIST_TAIL), copy);

        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries, 0xabcd);
        e.mem.set_u32(entries + 4, 0x1234);
        let built = e.mem.alloc(0x24);
        let back = e
            .call(0x00a8_b110, &args![built, 0x101u32, 2u32, entries])
            .u32();
        assert_eq!(back, built);
        assert_eq!(e.mem.u8(built + ALPHA), 1);
        assert_eq!(e.mem.u32(built + ENTRIES), 2);
        let block = e.mem.u32(built + PALETTE);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0xabcd, 0x1234));
        assert_eq!(e.mem.u32(LIST_TAIL), built);
        assert_eq!(e.mem.u32(copy + NEXT), built);
        // An entry count whose size overflows asks for 0xffffffff bytes.
        e.register(MEMCPY, |_, _| Ret::default());
        let huge = e.mem.alloc(0x24);
        e.call(0x00a8_b110, &args![huge, 0u32, 0x4000_0000u32, entries]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).last().unwrap()[1], u32::MAX);
    }

    #[test]
    fn create_builds_an_empty_palette_or_answers_null() {
        let mut e = engine();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let p = e.call(0x00a8_b220, &args![]).u32();
        assert_ne!(p, 0);
        assert_eq!(e.mem.u32(p), NI_PALETTE_VTABLE);
        assert_eq!(e.mem.u32(p + REVISION), 1);
        assert_eq!(e.mem.u32(p + ENTRIES), 0);
        assert_eq!(e.mem.u32(LIST_HEAD), p);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_b220, &args![]).u32(), 0);
    }

    #[test]
    fn assignment_replaces_the_block_only_for_a_new_count() {
        let mut e = engine();
        let data = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(vtable, 0x0340_0100);
        e.register(0x0340_0100, |_, _| Ret::default());
        let target = palette(&mut e, 0, &[1, 2]);
        e.mem.set_u32(target + RENDERER_DATA, data);
        let old = e.mem.u32(target + PALETTE);
        let same_count = palette(&mut e, 1, &[3, 4]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a8_b190, &args![target, same_count]).u32(),
            target
        );
        assert_eq!(calls(&e, 0x0340_0100), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(target + RENDERER_DATA), 0);
        assert_eq!(e.mem.u32(target + PALETTE), old);
        assert_eq!((e.mem.u32(old), e.mem.u32(old + 4)), (3, 4));
        assert_eq!(e.mem.u8(target + ALPHA), 1);
        assert_eq!(e.mem.u32(target + REVISION), 1);
        let longer = palette(&mut e, 0, &[5, 6, 7]);
        e.call(0x00a8_b190, &args![target, longer]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old]]
        );
        let block = e.mem.u32(target + PALETTE);
        assert_eq!(e.mem.u32(target + ENTRIES), 3);
        assert_eq!(e.mem.u32(block + 8), 7);
        // Assigning to itself does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a8_b190, &args![target, target]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn destructor_frees_unlinks_and_deletes_on_bit_0() {
        let mut e = engine();
        let p = palette(&mut e, 0, &[1]);
        e.call(0x00a8_afc0, &args![p]);
        let block = e.mem.u32(p + PALETTE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_b270, &args![p, 0u32]).u32(), p);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(e.mem.u32(LIST_HEAD), 0);
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCT), vec![vec![p]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a8_b270, &args![p, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![p, 0x24]]);
    }

    #[test]
    fn load_and_save_binary_stream_alpha_count_and_entries() {
        let mut e = engine();
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 8, 0x0340_0200);
        e.mem.set_u32(io + 0xc, 0x0340_0300);
        // The input: alpha 5 (not 0), two entries, then their bytes.
        let input: Vec<u8> = [5u8]
            .into_iter()
            .chain(2u32.to_le_bytes())
            .chain(0x0a0b_0c0du32.to_le_bytes())
            .chain(0x0102_0304u32.to_le_bytes())
            .collect();
        let cursor = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let c = cursor.clone();
        e.register_double(0x0340_0200, move |e, a| {
            let start = c.get();
            e.mem.write(a[1], &input[start..start + a[2] as usize]);
            c.set(start + a[2] as usize);
            a[2].into_ret()
        });
        let output = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let o = output.clone();
        e.register_double(0x0340_0300, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            o.borrow_mut().push((bytes, e.mem.u32(a[3])));
            a[2].into_ret()
        });
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        let p = palette(&mut e, 0, &[9]);
        e.call(0x00a8_b2c0, &args![p, stream]);
        assert_eq!(e.mem.u8(p + ALPHA), 1);
        assert_eq!(e.mem.u32(p + ENTRIES), 2);
        let block = e.mem.u32(p + PALETTE);
        assert_eq!(
            (e.mem.u32(block), e.mem.u32(block + 4)),
            (0x0a0b_0c0d, 0x0102_0304)
        );
        assert_eq!(e.mem.u32(io + 4), 13);
        e.call(0x00a8_b3c0, &args![p, stream]);
        let output = output.borrow();
        assert_eq!(output[0], (vec![1], 1));
        assert_eq!(output[1], (2u32.to_le_bytes().to_vec(), 4));
        assert_eq!(output[2].0, e.mem.bytes(block, 8));
        assert_eq!(output[2].1, 1);
        assert_eq!(e.mem.u32(io + 4), 26);
    }
}
