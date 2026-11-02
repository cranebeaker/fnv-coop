//! `NiXenonRenderer/nid3dshaderconstantmap.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e81d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderconstantmap.cpp`): the constant map's renderer
/// set-up, `thiscall` with one stack word (the renderer), stored in `m_pkD3DRenderer` (+0x3c).
/// With a renderer: the old device in `m_pkD3DDevice` (+0x38) is released (its vtable slot +8,
/// `stdcall`), the renderer's device (+0x288) is kept there and referenced (slot +4) when not
/// null, and `m_pkD3DRenderState` (+0x40) takes the renderer's +0x8b8. Without one: the old
/// device is released, and the device and render state are cleared (Xbox PDB names).
pub fn fn_00e81d00(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 0x3c, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer + 0x288);
        let old = e.mem.u32(base + 0x38);
        if old != 0 {
            e.vcall(old, 8, &args![]);
        }
        e.mem.set_u32(base + 0x38, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(base + 0x3c);
        let state = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(base + 0x40, state);
        return;
    }
    let old = e.mem.u32(base + 0x38);
    if old != 0 {
        e.vcall(old, 8, &args![]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0);
}

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject`'s vtable (`0101dce4`) and its object count `NiRefObject::ms_uiObjects`
/// (`011f4400`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
/// `NiShaderConstantMapEntry`'s vtable (`010ef4d8`, RTTI `.?AVNiShaderConstantMapEntry@@`),
/// `NiD3DShaderConstantMap`'s (`010ef544`, RTTI `.?AVNiD3DShaderConstantMap@@`) and the vtable of
/// its two entry arrays (`010ef53c`, RTTI
/// `.?AV?$NiTArray@V?$NiPointer@VNiShaderConstantMapEntry@@@@...`).
const ENTRY_VTABLE_010EF4D8: u32 = 0x010e_f4d8;
const MAP_VTABLE_010EF544: u32 = 0x010e_f544;
const ENTRY_ARRAY_VTABLE_010EF53C: u32 = 0x010e_f53c;
/// The `NiFixedString` handle the code compares every handle with before it touches the count
/// 8 bytes in front of it (`0109b220`), and the maker of a handle from a C string (`00a5b690`,
/// unnamed, `cdecl`, one word; it answers the handle with its count raised).
const FIXED_STRING_NULL_0109B220: u32 = 0x0109_b220;
const MAKE_FIXED_STRING_00A5B690: u32 = 0x00a5_b690;
/// The Ni allocator's calls the entries use (unnamed, `cdecl`; each goes through the allocator
/// at `011f6080`): `00aa13e0` (size; the entry objects), `00aa1070` (size; owned data),
/// `00aa10f0` (block; frees owned data) and `00aa1460` (block, size; frees an entry).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const NI_ALLOCATE_DATA_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_DATA_00AA10F0: u32 = 0x00aa_10f0;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// CRT `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiStricmp` (`00aa1360`, `cdecl`: two strings).
const NI_STRICMP: u32 = 0x00aa_1360;
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (`00aa04e0`, `cdecl`: the address of a
/// fixed-string handle).
const RELEASE_GLOBAL_SHADER_CONSTANT: u32 = 0x00aa_04e0;
/// The entry-array helpers outside this unit (unnamed): `00b79a60` makes an array of
/// `NiPointer`s (`cdecl`, count), `00b7ce30` deletes one (`cdecl`, elements), `00b799c0`
/// empties an array (`thiscall`, no stack words).
const NEW_POINTER_ARRAY_00B79A60: u32 = 0x00b7_9a60;
const DELETE_POINTER_ARRAY_00B7CE30: u32 = 0x00b7_ce30;
const ARRAY_REMOVE_ALL_00B799C0: u32 = 0x00b7_99c0;
/// `SetAt`'s function-local `static` null element (`0126f910`), its guard word (bit 0,
/// `0126f914`), CRT `_atexit` (`00ec658f`) and the element's exit destructor (`00fde390`).
const NULL_ELEMENT_0126F910: u32 = 0x0126_f910;
const NULL_ELEMENT_GUARD_0126F914: u32 = 0x0126_f914;
const ATEXIT: u32 = 0x00ec_658f;
const NULL_ELEMENT_DESTRUCT_00FDE390: u32 = 0x00fd_e390;
/// The word table `AddEntry` reads for an object entry, indexed by bits 8 to 15 of the flags
/// (`011f5f90`), and the table `AddConstantEntry` reads, indexed by the low byte of the entry's
/// flags (`011f5fc4`), and the byte table `SetupPredefinedEntry` reads (`011f5fb4`), all
/// unnamed.
const OBJECT_TABLE_011F5F90: u32 = 0x011f_5f90;
const CONSTANT_TYPE_TABLE_011F5FC4: u32 = 0x011f_5fc4;
const PREDEFINED_TYPE_TABLE_011F5FB4: u32 = 0x011f_5fb4;
/// `NiShaderConstantMap::LookUpPredefinedMapping` (`00a9bfd0`, `cdecl`: key, out mapping, out
/// count; answers a `bool`) and `::LookUpPredefinedMappingType` (`00a9bbf0`, `cdecl`: mapping,
/// count) (engine map).
const LOOK_UP_PREDEFINED_MAPPING: u32 = 0x00a9_bfd0;
const LOOK_UP_PREDEFINED_MAPPING_TYPE: u32 = 0x00a9_bbf0;
/// The data the two predefined mappings 0x50 and 0x51 point their entries at (`011bf63c`,
/// `011bf64c`, 0x10 bytes each).
const PREDEFINED_DATA_011BF63C: u32 = 0x011b_f63c;
const PREDEFINED_DATA_011BF64C: u32 = 0x011b_f64c;

fn interlocked_increment(e: &mut Engine, count: u32) {
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![count]);
}

fn interlocked_decrement(e: &mut Engine, count: u32) -> u32 {
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![count]).u32()
}

/// A reference-counted object released: its count at +4 decremented and the object deleted
/// through its vtable slot +4 when that answers 0.
fn release_object(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An element slot of an `NiPointer` array given `object`, as `SetAt` and `AddFirstEmpty`
/// expand it: nothing when it holds `object` already; else the old object released when not
/// null, then the slot takes the word at `element` (read again), referenced when not null.
fn assign_slot(e: &mut Engine, slot: u32, object: u32, element: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        release_object(e, old);
    }
    let new = e.mem.u32(element);
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

/// A fixed-string field set from a C string as the entry builders expand it: the handle made
/// by `00a5b690` (the null handle `0109b220` for a null string); when the field holds another
/// handle the new one is referenced and the old one released (each unless it is the null
/// handle) and the field set; then the made handle's own reference is dropped.
fn set_fixed_string(e: &mut Engine, field: u32, text: u32) {
    let handle = if text == 0 {
        e.global::<u32>(FIXED_STRING_NULL_0109B220)
    } else {
        e.call(MAKE_FIXED_STRING_00A5B690, &args![text]).u32()
    };
    if e.mem.u32(field) != handle {
        if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_increment(e, handle.wrapping_sub(8));
        }
        let old = e.mem.u32(field);
        if old != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_decrement(e, old.wrapping_sub(8));
        }
        e.mem.set_u32(field, handle);
    }
    if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
        interlocked_decrement(e, handle.wrapping_sub(8));
    }
}

/// An entry's data source set as the entry builders do it: with `copy`, the entry owns a copy
/// (`m_bOwnData` set, the old block freed through `00aa10f0`, `size` bytes from `00aa1070`
/// stored at `m_pvDataSource` and filled by `memcpy`); without, it points at `source`.
fn set_data_source(e: &mut Engine, entry: u32, size: u32, source: u32, copy: bool) {
    if copy {
        let old = e.mem.u32(entry + 0x30);
        e.mem.set_u8(entry + 0x34, 1);
        e.call(NI_FREE_DATA_00AA10F0, &args![old]);
        let block = e.call(NI_ALLOCATE_DATA_00AA1070, &args![size]).u32();
        e.mem.set_u32(entry + 0x30, block);
        e.call(MEMCPY, &args![block, source, size]);
    } else {
        e.mem.set_u8(entry + 0x34, 0);
        e.mem.set_u32(entry + 0x30, source);
    }
}

/// A new entry: 0x38 bytes from `00aa13e0` and the constructor (`00e81ba0`) when the
/// allocation succeeded, else null (which the callers then use as it is).
fn new_entry(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x38u32]).u32();
    if block != 0 {
        ni_shader_constant_map_entry_ni_shader_constant_map_entry(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

// Translated from 00e81ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMapEntry::NiShaderConstantMapEntry` (Xbox PDB), `thiscall`, no stack words.
/// The `NiRefObject` part (vtable `0101dce4`, count cleared, `ms_uiObjects` at `011f4400`
/// raised), then the entry's vtable `010ef4d8`, `m_bEnabled` set, `m_kKey` and
/// `m_kVariableName` the null handle (`0109b220`), the words +0x14 to +0x20 and +0x28 to +0x30
/// and the three bytes +0x34 to +0x36 cleared (+0x10 is left as it is). Answers `this`.
pub fn ni_shader_constant_map_entry_ni_shader_constant_map_entry(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.mem.set_u32(base + 4, 0);
    interlocked_increment(e, NI_REF_OBJECT_COUNT_011F4400);
    e.mem.set_u32(base, ENTRY_VTABLE_010EF4D8);
    e.mem.set_u8(base + 8, 1);
    let null = e.global::<u32>(FIXED_STRING_NULL_0109B220);
    e.mem.set_u32(base + 0xc, null);
    for offset in [0x14u32, 0x18, 0x1c, 0x20] {
        e.mem.set_u32(base + offset, 0);
    }
    let null = e.global::<u32>(FIXED_STRING_NULL_0109B220);
    e.mem.set_u32(base + 0x24, null);
    for offset in [0x28u32, 0x2c, 0x30] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0x34u32, 0x35, 0x36] {
        e.mem.set_u8(base + offset, 0);
    }
    this
}

// Translated from 00e81c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiShaderConstantMapEntry`'s destructor body (from its scalar
/// deleting destructor), `thiscall`, no stack words. The entry's vtable `010ef4d8`; owned data
/// (`m_bOwnData`, +0x34) freed through `00aa10f0`; `m_kVariableName` (+0x24) and `m_kKey`
/// (+0xc) released (the count 8 bytes before each handle, unless it is the null handle
/// `0109b220`); then the `NiRefObject` vtable and `ms_uiObjects` lowered.
pub fn fn_00e81c00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ENTRY_VTABLE_010EF4D8);
    if e.mem.u8(base + 0x34) != 0 {
        let data = e.mem.u32(base + 0x30);
        e.call(NI_FREE_DATA_00AA10F0, &args![data]);
    }
    for field in [0x24u32, 0xc] {
        let handle = e.mem.u32(base + field);
        if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_decrement(e, handle.wrapping_sub(8));
        }
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    interlocked_decrement(e, NI_REF_OBJECT_COUNT_011F4400);
}

// Translated from 00e83630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderConstantMapEntry::~NiShaderConstantMapEntry` scalar deleting destructor (Xbox PDB
/// `_scalar_deleting_destructor_`), `thiscall` with one stack word (flags). The destructor body
/// (`00e81c00`), then with flag bit 0 the 0x38 bytes freed through `00aa1460`. Answers `this`.
pub fn ni_shader_constant_map_entry_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e81c00(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00e86f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiShaderConstantMapEntry>, NiTNewInterface<...>>::SetAt` (Xbox PDB),
/// `thiscall` with two stack words: the index and the address of the element. On first use the
/// function-local null element (`0126f910`) is cleared and its destructor `00fde390` given to
/// `_atexit`. An index at or past `m_usSize` makes the size index + 1 and counts a non-null
/// element in `m_usESize`; below it `m_usESize` follows the change from null to non-null and
/// back (null meaning equal to that null element). Then the slot is assigned as an `NiPointer`
/// (old object released, new one referenced).
pub fn ni_t_array_set_at(e: &mut Engine, this: Ptr, index: u32, element: Ptr) {
    let array = this.addr();
    let element = element.addr();
    let guard = e.global::<u32>(NULL_ELEMENT_GUARD_0126F914);
    if guard & 1 == 0 {
        e.set_global(NULL_ELEMENT_GUARD_0126F914, guard | 1);
        e.set_global(NULL_ELEMENT_0126F910, 0u32);
        e.call(ATEXIT, &args![NULL_ELEMENT_DESTRUCT_00FDE390]);
    }
    let size = u32::from(e.mem.u16(array + 0xa));
    if index >= size {
        e.mem.set_u16(array + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(element) != e.global::<u32>(NULL_ELEMENT_0126F910) {
            let count = e.mem.u16(array + 0xc).wrapping_add(1);
            e.mem.set_u16(array + 0xc, count);
        }
    } else {
        let null = e.global::<u32>(NULL_ELEMENT_0126F910);
        let elements = e.mem.u32(array + 4);
        let current = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(element) != null {
            if current == null {
                let count = e.mem.u16(array + 0xc).wrapping_add(1);
                e.mem.set_u16(array + 0xc, count);
            }
        } else if current != null {
            let count = e.mem.u16(array + 0xc).wrapping_add(0xffff);
            e.mem.set_u16(array + 0xc, count);
        }
    }
    let elements = e.mem.u32(array + 4);
    let slot = elements.wrapping_add(index.wrapping_mul(4));
    let object = e.mem.u32(element);
    assign_slot(e, slot, object, element);
}

// Translated from 00e87510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the entry array's `SetSize` (`NiTArray` of entry pointers),
/// `thiscall` with one stack word (the new capacity; nothing when it equals `m_usMaxSize`).
/// A capacity below `m_usSize` first releases and clears the elements from the new capacity
/// to the size (each non-null one lowering `m_usESize`) and makes it the size. The capacity is
/// stored; capacity 0 leaves no elements, else a new array from `00b79a60` takes the old
/// elements (each slot assigned as an `NiPointer`) and the slots past the size are released
/// and cleared. The old element array goes to `00b7ce30` either way (its elements are not
/// released there).
pub fn fn_00e87510(e: &mut Engine, this: Ptr, capacity: u32) {
    let array = this.addr();
    if capacity == u32::from(e.mem.u16(array + 8)) {
        return;
    }
    let size = e.mem.u16(array + 0xa);
    let capacity16 = capacity as u16;
    if capacity < u32::from(size) {
        let mut index = capacity16;
        if index < size {
            loop {
                let elements = e.mem.u32(array + 4);
                let slot = elements.wrapping_add(u32::from(index) * 4);
                if e.mem.u32(slot) != 0 {
                    let object = e.mem.u32(slot);
                    if object != 0 {
                        release_object(e, object);
                        e.mem.set_u32(slot, 0);
                    }
                    let count = e.mem.u16(array + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(array + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(array + 0xa, capacity16);
    }
    let old = e.mem.u32(array + 4);
    e.mem.set_u16(array + 8, capacity16);
    if capacity == 0 {
        e.mem.set_u32(array + 4, 0);
    } else {
        let elements = e
            .call(NEW_POINTER_ARRAY_00B79A60, &args![u32::from(capacity16)])
            .u32();
        e.mem.set_u32(array + 4, elements);
        let mut index: u16 = 0;
        if index < e.mem.u16(array + 0xa) {
            loop {
                let elements = e.mem.u32(array + 4);
                let offset = u32::from(index) * 4;
                let slot = elements.wrapping_add(offset);
                let source = old.wrapping_add(offset);
                let object = e.mem.u32(source);
                assign_slot(e, slot, object, source);
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 0xa) {
                    break;
                }
            }
        }
        let mut index = e.mem.u16(array + 0xa);
        if index < e.mem.u16(array + 8) {
            loop {
                let elements = e.mem.u32(array + 4);
                let slot = elements.wrapping_add(u32::from(index) * 4);
                let object = e.mem.u32(slot);
                if object != 0 {
                    release_object(e, object);
                    e.mem.set_u32(slot, 0);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(array + 8) {
                    break;
                }
            }
        }
    }
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![old]);
}

// Translated from 00e87800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiShaderConstantMapEntry>, NiTNewInterface<...>>::AddFirstEmpty` (Xbox
/// PDB), `thiscall` with one stack word (the address of the element). A null element answers
/// -1. Else the first null slot below `m_usSize` takes it (assigned as an `NiPointer`,
/// `m_usESize` raised) and its index is answered; without one the array grows by `m_usGrowBy`
/// (`00e87510`) when the size has reached `m_usMaxSize`, `SetAt` puts the element at the size,
/// and that index is answered.
pub fn ni_t_array_add_first_empty(e: &mut Engine, this: Ptr, element: Ptr) -> u32 {
    let array = this.addr();
    let element = element.addr();
    let object = e.mem.u32(element);
    if object == 0 {
        return 0xffff_ffff;
    }
    let size = e.mem.u16(array + 0xa);
    let mut index: u16 = 0;
    if index < size {
        let elements = e.mem.u32(array + 4);
        loop {
            let slot = elements.wrapping_add(u32::from(index) * 4);
            if e.mem.u32(slot) == 0 {
                assign_slot(e, slot, object, element);
                let count = e.mem.u16(array + 0xc).wrapping_add(1);
                e.mem.set_u16(array + 0xc, count);
                return u32::from(index);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u16(array + 0xa) {
                break;
            }
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow = u32::from(e.mem.u16(array + 0xe));
        fn_00e87510(e, this, grow + size);
    }
    ni_t_array_set_at(e, this, size, Ptr::new(element));
    size
}

/// The element of `InsertEntry` and `InsertDefinedEntry`: the entry held for the call (count
/// raised), `AddFirstEmpty` on the array at `array` (+0xc or +0x1c) with the address of the
/// argument's stack word, the entry released, `m_eLastError` (+0x34) 0x30 when it answered -1;
/// answers `m_eLastError`.
fn insert_into(e: &mut Engine, this: Ptr, array: u32, entry: Ptr) -> u32 {
    let base = this.addr();
    let entry = entry.addr();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), entry);
        if entry != 0 {
            interlocked_increment(e, entry + 4);
        }
        let index = ni_t_array_add_first_empty(e, Ptr::new(base + array), local);
        let failed = index == 0xffff_ffff;
        if entry != 0 {
            release_object(e, entry);
        }
        if failed {
            e.mem.set_u32(base + 0x34, 0x30);
        }
        e.mem.u32(base + 0x34)
    })
}

// Translated from 00e878c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::InsertEntry` (Xbox PDB), `thiscall` with one stack word (the entry):
/// adds it to `m_aspEntries` (+0xc) through `AddFirstEmpty`; `m_eLastError` (+0x34) becomes
/// 0x30 when that fails, and is answered.
pub fn ni_d3d_shader_constant_map_insert_entry(e: &mut Engine, this: Ptr, entry: Ptr) -> u32 {
    insert_into(e, this, 0xc, entry)
}

// Translated from 00e87920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::InsertDefinedEntry` (Xbox PDB): as `InsertEntry`, into
/// `m_aspDefinedEntries` (+0x1c).
pub fn ni_d3d_shader_constant_map_insert_defined_entry(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
) -> u32 {
    insert_into(e, this, 0x1c, entry)
}

// Translated from 00e870c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the per-array step of `SortByEnabled`, `thiscall` on the map
/// (not read) with one stack word, the entry array. Arrays of fewer than two elements stay.
/// Walking the elements with one entry held (count raised while held), it remembers the first
/// disabled entry's index (`m_bEnabled`, +8, not 1); an enabled entry after it is swapped there
/// (`SetAt` of the disabled one at the enabled one's index, then of the held one at the
/// remembered index) and the remembered index moves on past the following enabled entries. The
/// held entry is released at the end. As in the exe, the step after a swap releases and reads
/// the held entry without a null check.
pub fn fn_00e870c0(e: &mut Engine, _unused_0: Ptr, array: Ptr) {
    let size = u32::from(e.mem.u16(array.addr() + 0xa));
    if size <= 1 {
        return;
    }
    let array_addr = array.addr();
    e.with_stack(4, |e, local| {
        let held_slot = local.addr();
        let mut held = 0u32;
        e.mem.set_u32(held_slot, held);
        let mut disabled = u32::MAX;
        let mut index = 0u32;
        loop {
            let elements = e.mem.u32(array_addr + 4);
            let slot = elements.wrapping_add(index.wrapping_mul(4));
            let mut skip = false;
            if held != e.mem.u32(slot) {
                if held != 0 {
                    release_object(e, held);
                }
                held = e.mem.u32(slot);
                e.mem.set_u32(held_slot, held);
                if held == 0 {
                    skip = true;
                } else {
                    interlocked_increment(e, held + 4);
                }
            }
            if !skip && held != 0 {
                if e.mem.u8(held + 8) == 1 {
                    if disabled < index {
                        let elements = e.mem.u32(array_addr + 4);
                        let other = elements.wrapping_add(disabled.wrapping_mul(4));
                        ni_t_array_set_at(e, array, index, Ptr::new(other));
                        ni_t_array_set_at(e, array, disabled, local);
                        disabled = disabled.wrapping_add(1);
                        if disabled >= size {
                            release_object(e, held);
                            return;
                        }
                        loop {
                            let elements = e.mem.u32(array_addr + 4);
                            let slot = elements.wrapping_add(disabled.wrapping_mul(4));
                            if held != e.mem.u32(slot) {
                                release_object(e, held);
                                held = e.mem.u32(slot);
                                e.mem.set_u32(held_slot, held);
                                if held != 0 {
                                    interlocked_increment(e, held + 4);
                                }
                            }
                            if e.mem.u8(held.wrapping_add(8)) == 0 {
                                break;
                            }
                            disabled = disabled.wrapping_add(1);
                            if disabled >= size {
                                release_object(e, held);
                                return;
                            }
                        }
                        if disabled >= size {
                            release_object(e, held);
                            return;
                        }
                    }
                } else if disabled > index {
                    disabled = index;
                }
            }
            index += 1;
            if index >= size {
                break;
            }
        }
        if held != 0 {
            release_object(e, held);
        }
    });
}

// Translated from 00e87200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SortByEnabled` (Xbox PDB), `thiscall`, no stack words: the step
/// `00e870c0` on `m_aspEntries` (+0xc), then on `m_aspDefinedEntries` (+0x1c).
pub fn ni_d3d_shader_constant_map_sort_by_enabled(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_00e870c0(e, this, Ptr::new(base + 0xc));
    fn_00e870c0(e, this, Ptr::new(base + 0x1c));
}

// Translated from 00e82640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::GetEntry` (Xbox PDB), `thiscall` with one stack word (the key):
/// the first entry of `m_aspDefinedEntries` (+0x1c), then of `m_aspEntries` (+0xc), whose key
/// (+0xc) `NiStricmp` finds equal, else null. Each array is walked up to its size as read at
/// the start.
pub fn ni_d3d_shader_constant_map_get_entry(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let base = this.addr();
    for array in [0x1cu32, 0xc] {
        let size = u32::from(e.mem.u16(base + array + 0xa));
        let mut index = 0u32;
        while index < size {
            let elements = e.mem.u32(base + array + 4);
            let entry = e.mem.u32(elements.wrapping_add(index * 4));
            if entry != 0 {
                let name = e.mem.u32(entry + 0xc);
                if e.call(NI_STRICMP, &args![key, name]).u32() == 0 {
                    return entry;
                }
            }
            index += 1;
        }
    }
    0
}

// Translated from 00e81d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddEntry` (Xbox PDB), `thiscall` with ten stack words: the key,
/// the flags, the extra word, then seven more passed on. `m_eLastError` (+0x34) is cleared;
/// an entry `GetEntry` (vtable +0x70) finds whose `m_uiExtra` (+0x18) equals the extra word
/// gives 0x20. Else the flags' top four bits choose the adder (Xbox PDB slot names):
/// 0x3 `AddAttributeEntry` (+0x10) and 0x1 `AddConstantEntry` (+0x14) and 0x4
/// `AddGlobalEntry` (+0x18) with all ten words, 0x2 `AddPredefinedEntry` (+0x68) with the key,
/// the extra word, the fourth and sixth words and four zeros, 0x5 `AddOperatorEntry` (+0x1c)
/// with the first six words, 0x6 `AddObjectEntry` (+0x20) with the key, the fourth, sixth and
/// third words and the word of the table at `011f5f90` that bits 8 to 15 of the flags index;
/// any other kind gives 0x40. The result is stored in `m_eLastError` and answered.
pub fn ni_d3d_shader_constant_map_add_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: u32,
) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base + 0x34, 0);
    let entry = e.vcall(base, 0x70, &args![key]).u32();
    if entry != 0 && e.mem.u32(entry + 0x18) == extra {
        e.mem.set_u32(base + 0x34, 0x20);
        return 0x20;
    }
    let all = args![
        key,
        flags,
        extra,
        register,
        register_count,
        variable,
        data_size,
        data_stride,
        data_source,
        copy_data
    ];
    let result = match flags & 0xf000_0000 {
        0x3000_0000 => e.vcall(base, 0x10, &all).u32(),
        0x1000_0000 => e.vcall(base, 0x14, &all).u32(),
        0x2000_0000 => e
            .vcall(
                base,
                0x68,
                &args![key, extra, register, variable, 0u32, 0u32, 0u32, 0u32],
            )
            .u32(),
        0x4000_0000 => e.vcall(base, 0x18, &all).u32(),
        0x5000_0000 => e
            .vcall(
                base,
                0x1c,
                &args![key, flags, extra, register, register_count, variable],
            )
            .u32(),
        0x6000_0000 => {
            let object = e.mem.u32(OBJECT_TABLE_011F5F90 + ((flags >> 8) & 0xff) * 4);
            e.vcall(base, 0x20, &args![key, register, variable, extra, object])
                .u32()
        }
        _ => {
            e.mem.set_u32(base + 0x34, 0x40);
            return e.mem.u32(base + 0x34);
        }
    };
    e.mem.set_u32(base + 0x34, result);
    result
}

// Translated from 00e81f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddPredefinedEntry` (Xbox PDB), `thiscall` with eight stack words:
/// the key, the extra word, the register, the variable name, the data size and stride, the
/// data source and whether to copy it (low byte). A new entry (`00aa13e0`, constructor
/// `00e81ba0`) gets the key and the variable name as fixed strings, `m_uiExtra` (+0x18),
/// `m_uiShaderRegister` (+0x1c), size (+0x28), stride (+0x2c), the data (copied into an owned
/// block, or pointed at) and the flags 0x20000000. `SetupPredefinedEntry` (vtable +0x88)
/// answers into `m_eLastError` (+0x34); 0 inserts the entry (`InsertDefinedEntry`, +0x84,
/// answered and stored), else the entry is deleted (its vtable slot 0 with 1) and the error
/// answered. A failed allocation is used as the null entry, as in the exe.
pub fn ni_d3d_shader_constant_map_add_predefined_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    extra: u32,
    register: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: bool,
) -> u32 {
    let base = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x28), data_size);
    e.mem.set_u32(entry.wrapping_add(0x2c), data_stride);
    set_data_source(e, entry, data_size, data_source, copy_data);
    e.mem.set_u32(entry.wrapping_add(0x14), 0x2000_0000);
    let error = e.vcall(base, 0x88, &args![entry]).u32();
    e.mem.set_u32(base + 0x34, error);
    if error == 0 {
        let error = e.vcall(base, 0x84, &args![entry]).u32();
        e.mem.set_u32(base + 0x34, error);
        return error;
    }
    e.vcall(entry, 0, &args![1u32]);
    e.mem.u32(base + 0x34)
}

// Translated from 00e83650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddConstantEntry` (Xbox PDB), `thiscall` with ten stack words: the
/// key, the flags, the extra word, the register, the register count, the variable name, the
/// data size and stride, the data source and whether to copy it (low byte). A new entry gets the
/// key; its flags are the given ones' low 28 bits with 0x10000000, and by data size 8: 5,
/// 0xc: 6, 0x10: 7, 0x24: 8, 0x20/0x30/0x40: 9 or'ed in. Its `m_uiExtra` (+0x18) is 1 (program
/// type 0, the map's +8) or 2 (program type 2) for a size that is a multiple of 4 with stride 4
/// whose type in the table at `011f5fc4` (the flags' low byte) is neither 1 nor 3, else the
/// given extra word. Register (+0x1c), count (+0x20), variable name, stride (+0x2c), size
/// (+0x28) and the data follow. `InsertEntry` (vtable +0x80) answers into `m_eLastError`
/// (+0x34); on an error the entry is deleted (slot 0 with 1) and the error answered, else 0.
pub fn ni_d3d_shader_constant_map_add_constant_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: bool,
) -> u32 {
    let base = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    let mut entry_flags = (flags & 0x0fff_ffff) | 0x1000_0000;
    match data_size {
        8 => entry_flags |= 5,
        0xc => entry_flags |= 6,
        0x10 => entry_flags |= 7,
        0x24 => entry_flags |= 8,
        0x20 | 0x30 | 0x40 => entry_flags |= 9,
        _ => {}
    }
    e.mem.set_u32(entry.wrapping_add(0x14), entry_flags);
    let mut chosen = None;
    if data_size & 3 == 0 && data_stride == 4 {
        let kind = e
            .mem
            .u32(CONSTANT_TYPE_TABLE_011F5FC4 + (entry_flags & 0xff) * 4);
        if kind != 1 && kind != 3 {
            match e.mem.u32(base + 8) {
                0 => chosen = Some(data_stride.wrapping_sub(3)),
                2 => chosen = Some(2),
                _ => {}
            }
        }
    }
    e.mem
        .set_u32(entry.wrapping_add(0x18), chosen.unwrap_or(extra));
    e.mem.set_u32(entry.wrapping_add(0x20), register_count);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x2c), data_stride);
    e.mem.set_u32(entry.wrapping_add(0x28), data_size);
    set_data_source(e, entry, data_size, data_source, copy_data);
    let error = e.vcall(base, 0x80, &args![entry]).u32();
    e.mem.set_u32(base + 0x34, error);
    if error != 0 {
        e.vcall(entry, 0, &args![1u32]);
        return e.mem.u32(base + 0x34);
    }
    0
}

// Translated from 00e829b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupPredefinedEntry` (Xbox PDB), `thiscall` (the map is not read)
/// with one stack word, the entry. `LookUpPredefinedMapping` (`00a9bfd0`) of the entry's key
/// gives the mapping and a count (4 to start with); not found or mapping 0 answers 0x40. The
/// low byte of the entry's flags (+0x14) becomes the byte of the table at `011f5fb4` that
/// `LookUpPredefinedMappingType` (`00a9bbf0`) indexes. Below mapping 0x45 the register count
/// (+0x20) is set: 0x21/0x23 three times the extra word's high half (+0x1a), 0x22/0x24 four
/// times, 0x19 to 0x20 four (the arms for 0x60 to 0x63 are never reached there but are kept),
/// else the looked-up count. `m_uiInternal` (+0x10) takes the mapping; mapping 0x50 or 0x51
/// points the entry at its 0x10 bytes (`011bf63c`, `011bf64c`; not owned, size and stride
/// 0x10). Answers 0.
pub fn ni_d3d_shader_constant_map_setup_predefined_entry(
    e: &mut Engine,
    _unused_0: Ptr,
    entry: Ptr,
) -> u32 {
    let entry = entry.addr();
    e.with_stack(8, |e, locals| {
        let mapping_word = locals.addr();
        let count_word = locals.addr() + 4;
        e.mem.set_u32(count_word, 4);
        let key = e.mem.u32(entry + 0xc);
        let found = e
            .call(
                LOOK_UP_PREDEFINED_MAPPING,
                &args![key, mapping_word, count_word],
            )
            .bool();
        if !found {
            return 0x40;
        }
        let mapping = e.mem.u32(mapping_word);
        let count = e.mem.u32(count_word);
        let kind = e
            .call(LOOK_UP_PREDEFINED_MAPPING_TYPE, &args![mapping, count])
            .u32();
        let byte = u32::from(e.mem.u8(PREDEFINED_TYPE_TABLE_011F5FB4.wrapping_add(kind)));
        let flags = e.mem.u32(entry + 0x14);
        e.mem.set_u32(entry + 0x14, byte | (flags & 0xffff_ff00));
        let mapping = e.mem.u32(mapping_word);
        if mapping == 0 {
            return 0x40;
        }
        if mapping < 0x45 {
            let registers = match mapping {
                0x21 | 0x23 => u32::from(e.mem.u16(entry + 0x1a)) * 3,
                0x22 | 0x24 => u32::from(e.mem.u16(entry + 0x1a)) * 4,
                0x19..=0x20 | 0x60..=0x63 => 4,
                _ => e.mem.u32(count_word),
            };
            e.mem.set_u32(entry + 0x20, registers);
        }
        e.mem.set_u32(entry + 0x10, mapping);
        let data = match mapping {
            0x50 => PREDEFINED_DATA_011BF63C,
            0x51 => PREDEFINED_DATA_011BF64C,
            _ => return 0,
        };
        e.mem.set_u32(entry + 0x30, data);
        e.mem.set_u8(entry + 0x34, 0);
        e.mem.set_u32(entry + 0x2c, 0x10);
        e.mem.set_u32(entry + 0x28, 0x10);
        0
    })
}

// Translated from 00e826d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetShaderConstants` (Xbox PDB), `thiscall` with eleven stack words:
/// the shader program (kept in `m_pkLastShaderProgram`, +0x30) and ten words passed on. Every
/// enabled entry (`m_bEnabled`, +8) with a register (+0x1c not -1) or a variable name (+0x24)
/// is set. First the predefined ones (kind 0x2) of `m_aspDefinedEntries` through
/// `SetDefinedConstant` (vtable +0x8c: program, entry, the ten words). Then by kind those of
/// `m_aspEntries`: constants (0x1) with extra word 1 or 2 go to the device (+0x38) as vertex
/// (slot +0x178) or pixel (slot +0x1b4) float constants (register, data, register count; a
/// negative result fails), other constants through `SetConstantConstant` (+0x90: program,
/// entry, the tenth word); attributes (0x3, +0x94: program, entry, the ten words, 0), globals
/// (0x4, +0x98: program, entry, the tenth word), operators (0x5, +0x9c: program, entry, the
/// second, third and sixth to tenth words) and objects (0x6, +0xa4: program, entry, the ten
/// words); predefined ones are skipped, any other kind fails. A failure is a non-zero result.
/// Answers whether any failed.
pub fn ni_d3d_shader_constant_map_set_shader_constants(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + 0x30, program);
    let mut ok = true;
    let size = u32::from(e.mem.u16(base + 0x26));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base + 0x20);
        let entry = e.mem.u32(elements.wrapping_add(index * 4));
        index += 1;
        if entry == 0 || e.mem.u8(entry + 8) == 0 {
            continue;
        }
        if e.mem.u32(entry + 0x1c) == 0xffff_ffff && e.mem.u32(entry + 0x24) == 0 {
            continue;
        }
        if e.mem.u32(entry + 0x14) & 0xf000_0000 != 0x2000_0000 {
            continue;
        }
        let result = e
            .vcall(
                base,
                0x8c,
                &args![
                    program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8, word_9,
                    word_10
                ],
            )
            .u32();
        if result != 0 {
            ok = false;
        }
    }
    let size = u32::from(e.mem.u16(base + 0x16));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(base + 0x10);
        let entry = e.mem.u32(elements.wrapping_add(index * 4));
        index += 1;
        if entry == 0 || e.mem.u8(entry + 8) == 0 {
            continue;
        }
        let register = e.mem.u32(entry + 0x1c);
        if register == 0xffff_ffff && e.mem.u32(entry + 0x24) == 0 {
            continue;
        }
        let result = match e.mem.u32(entry + 0x14) & 0xf000_0000 {
            0x2000_0000 => continue,
            0x1000_0000 => match e.mem.u32(entry + 0x18) {
                1 | 2 => {
                    let slot = if e.mem.u32(entry + 0x18) == 1 {
                        0x178
                    } else {
                        0x1b4
                    };
                    let data = e.mem.u32(entry + 0x30);
                    let count = e.mem.u32(entry + 0x20);
                    let device = e.mem.u32(base + 0x38);
                    let answer = e.vcall(device, slot, &args![register, data, count]).i32();
                    u32::from(answer < 0)
                }
                _ => e.vcall(base, 0x90, &args![program, entry, word_10]).u32(),
            },
            0x3000_0000 => e
                .vcall(
                    base,
                    0x94,
                    &args![
                        program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8,
                        word_9, word_10, word_11, 0u32
                    ],
                )
                .u32(),
            0x4000_0000 => e.vcall(base, 0x98, &args![program, entry, word_10]).u32(),
            0x5000_0000 => e
                .vcall(
                    base,
                    0x9c,
                    &args![program, entry, word_2, word_3, word_6, word_7, word_8, word_9, word_10],
                )
                .u32(),
            0x6000_0000 => e
                .vcall(
                    base,
                    0xa4,
                    &args![
                        program, entry, word_2, word_3, word_4, word_5, word_6, word_7, word_8,
                        word_9, word_10
                    ],
                )
                .u32(),
            _ => 1,
        };
        if result != 0 {
            ok = false;
        }
    }
    !ok
}

// Translated from 00e87690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiD3DShaderConstantMap`'s destructor body (from the scalar
/// deleting destructor at `00b7e413`), `thiscall`, no stack words. The map's vtable
/// `010ef544`; every global entry (kind 0x4) of `m_aspEntries` gives its key to
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (through a held copy of the handle) and of
/// `m_aspDefinedEntries` the address of its key (each entry held across it); each array is then
/// emptied (`00b799c0`). The device (+0x38) is released (its slot +8) and the renderer, device
/// and render state cleared; the two arrays get their vtable `010ef53c` and their elements go to
/// `00b7ce30`; last the `NiRefObject` vtable and `ms_uiObjects` lowered.
pub fn fn_00e87690(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_VTABLE_010EF544);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let mut index = 0u32;
        if e.mem.u16(base + 0x14) != 0 {
            loop {
                let elements = e.mem.u32(base + 0x10);
                let entry = e.mem.u32(elements.wrapping_add(index * 4));
                if entry != 0 {
                    interlocked_increment(e, entry + 4);
                    if e.mem.u32(entry + 0x14) & 0xf000_0000 == 0x4000_0000 {
                        let key = e.mem.u32(entry + 0xc);
                        if key != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                            interlocked_increment(e, key.wrapping_sub(8));
                        }
                        let key = e.mem.u32(entry + 0xc);
                        e.mem.set_u32(local, key);
                        e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![local]);
                        let key = e.mem.u32(local);
                        if key != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                            interlocked_decrement(e, key.wrapping_sub(8));
                        }
                    }
                    release_object(e, entry);
                }
                index += 1;
                if index >= u32::from(e.mem.u16(base + 0x14)) {
                    break;
                }
            }
        }
    });
    e.call(ARRAY_REMOVE_ALL_00B799C0, &args![base + 0xc]);
    let mut index = 0u32;
    if e.mem.u16(base + 0x24) != 0 {
        loop {
            let elements = e.mem.u32(base + 0x20);
            let entry = e.mem.u32(elements.wrapping_add(index * 4));
            if entry != 0 {
                interlocked_increment(e, entry + 4);
                if e.mem.u32(entry + 0x14) & 0xf000_0000 == 0x4000_0000 {
                    e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![entry + 0xc]);
                }
                release_object(e, entry);
            }
            index += 1;
            if index >= u32::from(e.mem.u16(base + 0x24)) {
                break;
            }
        }
    }
    e.call(ARRAY_REMOVE_ALL_00B799C0, &args![base + 0x1c]);
    let device = e.mem.u32(base + 0x38);
    e.mem.set_u32(base + 0x3c, 0);
    if device != 0 {
        e.vcall(device, 8, &args![]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0);
    let elements = e.mem.u32(base + 0x20);
    e.mem.set_u32(base + 0x1c, ENTRY_ARRAY_VTABLE_010EF53C);
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![elements]);
    let elements = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, ENTRY_ARRAY_VTABLE_010EF53C);
    e.call(DELETE_POINTER_ARRAY_00B7CE30, &args![elements]);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    interlocked_decrement(e, NI_REF_OBJECT_COUNT_011F4400);
}

/// What `SetDefinedConstant` calls: D3DX's `D3DXMatrixInverse` (`00ee6dda`; `stdcall`: out,
/// determinant, matrix), `D3DXMatrixTranspose` (`00ee6dd4`: out, matrix), `D3DXMatrixMultiply`
/// (`00ee6de0`: out, first, second) and `00ee6dfe` (unnamed; `stdcall`: out and two matrices);
/// `NiD3DShaderConstantMap::SetupTextureTransformMatrix` (`00e83550`, engine map; `cdecl`: out,
/// matrix, two bytes, a count); `NiTextureTransform::UpdateMatrix` (`00bc5c20`, engine map;
/// `thiscall`, answers its matrix); `00a463e0` (unit `nitexturetransformcontroller.cpp`;
/// `thiscall` on a texturing property with an index, answers a map); the extra data name
/// handle (`011bf65c`) with the `NiFixedString` constructor `00438170` (`thiscall`, the
/// handle) and destructor `004381b0`; `NiObjectNET::GetExtraData` (`00a5bdd0`, engine map;
/// `thiscall`, the address of the name); the CRT's x87 helpers `00eca0a0`, `00ec9f70` and
/// `00eca5d0` (the value in and out in ST0); `00653290` (unit `bsfacegenmanager.cpp`;
/// `thiscall` with the record `011f5f6c`, answers an object or 0) and `00633b40` (unnamed;
/// `thiscall` with the address of three floats and a float).
const MATRIX_INVERSE_00EE6DDA: u32 = 0x00ee_6dda;
const MATRIX_TRANSPOSE_00EE6DD4: u32 = 0x00ee_6dd4;
const MATRIX_MULTIPLY_00EE6DE0: u32 = 0x00ee_6de0;
const MATRIX_PRODUCT_00EE6DFE: u32 = 0x00ee_6dfe;
const SETUP_TEXTURE_TRANSFORM_00E83550: u32 = 0x00e8_3550;
const UPDATE_MATRIX_00BC5C20: u32 = 0x00bc_5c20;
const MAP_AT_00A463E0: u32 = 0x00a4_63e0;
const EXTRA_DATA_NAME_011BF65C: u32 = 0x011b_f65c;
const FIXED_STRING_CONSTRUCT_00438170: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT_004381B0: u32 = 0x0043_81b0;
const GET_EXTRA_DATA_00A5BDD0: u32 = 0x00a5_bdd0;
const CRT_HELPER_00ECA0A0: u32 = 0x00ec_a0a0;
const CRT_HELPER_00EC9F70: u32 = 0x00ec_9f70;
const CRT_HELPER_00ECA5D0: u32 = 0x00ec_a5d0;
const LIGHT_CAST_00653290: u32 = 0x0065_3290;
const LIGHT_RECORD_011F5F6C: u32 = 0x011f_5f6c;
const LIGHT_OFFSET_00633B40: u32 = 0x0063_3b40;
/// Three floats `SetDefinedConstant` reads for the constants 73 and 74 (`011f4998` to
/// `011f49a0`) and 2^32 as a float (`01099378`).
const VECTOR_011F4998: u32 = 0x011f_4998;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// The results `SetDefinedConstant` answers: the extra data not found (0x10) and the program
/// refusing the constant (0x50).
const NO_EXTRA_DATA: u32 = 0x10;
const NOT_SET: u32 = 0x50;
/// 1.0 as a float (what `FLD1` and a float store leave).
const ONE: u32 = 0x3f80_0000;

/// `FLD dword` then `FSTP dword` of the float at `at`: its bits, a signalling NaN quieted.
fn copy_float(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

/// 16 words copied (`REP MOVSD`).
fn copy_matrix(e: &mut Engine, from: u32, to: u32) {
    for word in 0..16u32 {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

/// The matrix at `matrix` inverted in place (`D3DXMatrixInverse` with no determinant) when
/// `inverse`, then transposed in place when `transpose`.
fn invert_transpose(e: &mut Engine, matrix: u32, inverse: bool, transpose: bool) {
    if inverse {
        e.call(MATRIX_INVERSE_00EE6DDA, &args![matrix, 0u32, matrix]);
    }
    if transpose {
        e.call(MATRIX_TRANSPOSE_00EE6DD4, &args![matrix, matrix]);
    }
}

/// Stores four float bit patterns at `at`.
fn store_vector(e: &mut Engine, at: u32, values: [u32; 4]) {
    for (i, value) in values.iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * i as u32), *value);
    }
}

/// The value of the 16-bit dimension a texture's slot answers as `FILD` takes it (2^32 added,
/// `FADD` of the float at `01099378`, when it is negative as a signed word), stored as a float.
fn unsigned_to_float(e: &Engine, word: u32) -> u32 {
    let mut value = f64::from(word as i32);
    if (word as i32) < 0 {
        value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
    }
    x87_store32(value)
}

/// A CRT helper on the float `time` (`FLD`, the call with the value in ST0, its result stored
/// as a float, loaded and stored again): the result's bits.
fn crt_on(e: &mut Engine, helper: u32, time: u32) -> u32 {
    let result = e.call(helper, &args![x87_load32(time)]).f64();
    x87_quiet_bits(x87_store32(result))
}

// Translated from 00e83db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetDefinedConstant` (engine map), `thiscall` with eleven words (the
/// sixth and the last three not read): the shader program, the constant map entry, an object
/// with extra data, two words for the skinning constants, the property state and the dynamic
/// effect state. The entry's predefined constant (+0x10) picks the data handed to the program's
/// `SetShaderConstant` (its slot 0x74: the entry, the data, 0); answers 0, or 0x50 when the
/// program answers false. The compiler's switch over the constant (a byte map and a jump
/// table in the code) is the `match` below. By constant (the renderer is `m_pkD3DRenderer`
/// +0x3c, Xbox PDB; "inverted and transposed" means the second and fourth constant of a group
/// of four inverted with `D3DXMatrixInverse`, the third and fourth transposed with
/// `D3DXMatrixTranspose`):
/// - 1/2/13/14, 3/4/15/16, 5/6/17/18: a copy of the renderer's matrix at +0x9c0, +0x980 or
///   +0x940, inverted for 2/14, 4/16, 6/18 and transposed for 13/14, 15/16, 17/18.
/// - 7/8/19/20: the product (`D3DXMatrixMultiply`) of copies of +0x940 and +0x980, inverted for
///   8/20 and transposed for 19/20; 9/10/21/22 the product of +0x980 and +0x9c0 (10/22, 21/22).
/// - 11/12/23/24: `00ee6dfe` of +0x940 and +0xa00 into the stack.
/// - 25 to 28 (needing the fourth word): a copy of +0x980; 29 to 32 (needing it): the product of
///   +0x980 and +0x9c0; inverted and transposed.
/// - 33/35 (needing the fourth and fifth words) and 34/36 (needing the fourth): the program's
///   `SetShaderConstantArray` (slot 0x78) with the entry, the fourth word's +0x28, the fifth's
///   16-bit +0x20, the fourth's +0x20 and the fifth's +4.
/// - 37 to 60 and 88 to 95 in groups of four: the texture maps 0 to 5, then 6 and 7, of the
///   property state's texturing property (+0x14, its map array at +0x20); 61 to 64 the map at
///   the entry's +0x18 in the array from +0x20; 65 to 68 the map `00a463e0` answers for that
///   index. A map with a transform (+0xc) gives its matrix (`UpdateMatrix`) and the two flags
///   (inverted, transposed) to `SetupTextureTransformMatrix` (`00e83550`, with 4); without a
///   map neither.
/// - 73/74, 75, 76, 77: four floats from the property state's +8 object (none: answers 0): the
///   floats at `011f4998` and its +0x3c; its +0x1c to +0x24 and +0x3c; three floats from its
///   +0x34 (or else its +0x28) and +0x3c; its +0x38 four times.
/// - 78, 79: the last and the third row of a copy of the renderer's +0xa40 matrix.
/// - 80, 81: no data.
/// - 82 to 86: the float +0xc of the object's extra data named by `011bf65c` (none: the result
///   becomes 0x10 and the read is made at address 0xc, as the code makes it): four times as it
///   is, four times through each of the CRT helpers `00eca0a0`, `00ec9f70`, `00eca5d0`, or the
///   value and the three.
/// - 87: (0, 0, 0, 1) plus, for each light of the effect state's list (+0xc) that `00653290`
///   answers an object for, the three floats `00633b40` gives from its +0xc8 and +0xc4.
/// - 96 to 99 (needing the fourth word): the identity, inverted and transposed.
/// - 103, 104, 105: map 5's four floats at +0x18 (else 1s); its +0x14 and +0x10 (else 0 and
///   1); map 7's +0x10 (else 0).
/// - 106 to 115 (needing the texturing property): the width and height (slots 0x94 and 0x98 of
///   the map's texture +8, as unsigned) for maps 0 to 7, the map at the entry's index or the one
///   `00a463e0` answers (else 0 and 0).
/// - Any other: answers 0.
pub fn ni_d3d_shader_constant_map_set_defined_constant(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    entry: u32,
    object: u32,
    fourth: u32,
    fifth: u32,
    _unused_6: u32,
    properties: u32,
    effects: u32,
    _unused_9: u32,
    _unused_10: u32,
    _unused_11: u32,
) -> u32 {
    let base = this.addr();
    let id = e.mem.u32(entry.wrapping_add(0x10));
    // The inverted and transposed members of the group of four from `first`.
    let flags = |first: u32| {
        (
            id == first + 1 || id == first + 3,
            id == first + 2 || id == first + 3,
        )
    };
    let renderer = |e: &Engine| e.mem.u32(base.wrapping_add(0x3c));
    let texturing = |e: &Engine| e.mem.u32(properties.wrapping_add(0x14));
    // The texturing property's map array (+0x20, not checked) word at `offset`.
    let map_word = |e: &Engine, texturing: u32, offset: u32| {
        let maps = e.mem.u32(texturing.wrapping_add(0x20));
        e.mem.u32(maps.wrapping_add(offset))
    };
    let entry_index = |e: &Engine| e.mem.u32(entry.wrapping_add(0x18));
    e.with_stack(0x110, |e, frame| {
        let frame = frame.addr();
        let pair = frame + 0x1c;
        let single = frame + 0x24;
        let vector = frame + 0x30;
        let first_matrix = frame + 0x40;
        let name = frame + 0x8c;
        let second_matrix = frame + 0x90;
        let third_matrix = frame + 0xd0;
        let mut result = 0u32;
        // A copy of the renderer's matrix at `offset`, inverted and transposed as asked.
        let renderer_matrix = |e: &mut Engine, offset: u32, inverse: bool, transpose: bool| {
            let from = renderer(e).wrapping_add(offset);
            copy_matrix(e, from, first_matrix);
            invert_transpose(e, first_matrix, inverse, transpose);
            first_matrix
        };
        // The product of the renderer's matrices at `left` and `right` (copied into the second
        // and third stack matrices, the left one in whichever the code uses), inverted and
        // transposed as asked.
        let product = |e: &mut Engine,
                       left: (u32, u32),
                       right: (u32, u32),
                       inverse: bool,
                       transpose: bool| {
            let renderer = renderer(e);
            copy_matrix(e, renderer.wrapping_add(left.0), left.1);
            copy_matrix(e, renderer.wrapping_add(right.0), right.1);
            e.call(
                MATRIX_MULTIPLY_00EE6DE0,
                &args![first_matrix, left.1, right.1],
            );
            invert_transpose(e, first_matrix, inverse, transpose);
            first_matrix
        };
        // The texture transform of `map` (0 for none) set up into the second stack matrix.
        let texture_transform = |e: &mut Engine, map: u32, first: u32| {
            let mut matrix = 0;
            let (mut inverse, mut transpose) = (false, false);
            if map != 0 {
                let transform = e.mem.u32(map.wrapping_add(0xc));
                if transform != 0 {
                    matrix = e.call(UPDATE_MATRIX_00BC5C20, &args![transform]).u32();
                    (inverse, transpose) = flags(first);
                }
            }
            // The flags go as whole stack words whose upper three bytes the code never wrote;
            // the callee reads only their low bytes.
            e.call(
                SETUP_TEXTURE_TRANSFORM_00E83550,
                &args![
                    second_matrix,
                    matrix,
                    u32::from(inverse),
                    u32::from(transpose),
                    4u32
                ],
            );
            second_matrix
        };
        // A texture map of the texturing property, 0 without the property.
        let slot_map = |e: &Engine, offset: u32| {
            let texturing = texturing(e);
            if texturing == 0 {
                0
            } else {
                map_word(e, texturing, offset)
            }
        };
        let data = match id {
            1 | 2 | 13 | 14 => renderer_matrix(e, 0x9c0, id == 2 || id == 14, id >= 13),
            3 | 4 | 15 | 16 => renderer_matrix(e, 0x980, id == 4 || id == 16, id >= 15),
            5 | 6 | 17 | 18 => renderer_matrix(e, 0x940, id == 6 || id == 18, id >= 17),
            7 | 8 | 19 | 20 => product(
                e,
                (0x940, second_matrix),
                (0x980, third_matrix),
                id == 8 || id == 20,
                id >= 19,
            ),
            9 | 10 | 21 | 22 => product(
                e,
                (0x980, third_matrix),
                (0x9c0, second_matrix),
                id == 10 || id == 22,
                id >= 21,
            ),
            11 | 12 | 23 | 24 => {
                let renderer = renderer(e);
                e.call(
                    MATRIX_PRODUCT_00EE6DFE,
                    &args![
                        second_matrix,
                        renderer.wrapping_add(0x940),
                        renderer.wrapping_add(0xa00)
                    ],
                );
                second_matrix
            }
            25..=28 => {
                if fourth == 0 {
                    return 0;
                }
                let (inverse, transpose) = flags(0x19);
                renderer_matrix(e, 0x980, inverse, transpose)
            }
            29..=32 => {
                if fourth == 0 {
                    return 0;
                }
                let (inverse, transpose) = flags(0x1d);
                product(
                    e,
                    (0x980, third_matrix),
                    (0x9c0, second_matrix),
                    inverse,
                    transpose,
                )
            }
            33..=36 => {
                if fourth == 0 || ((id == 33 || id == 35) && fifth == 0) {
                    return 0;
                }
                let matrices = e.mem.u32(fourth.wrapping_add(0x28));
                let bones = u32::from(e.mem.u16(fifth.wrapping_add(0x20)));
                let registers = e.mem.u32(fourth.wrapping_add(0x20));
                let indices = e.mem.u32(fifth.wrapping_add(4));
                let set = e
                    .vcall(
                        program,
                        0x78,
                        &args![entry, matrices, bones, registers, indices],
                    )
                    .bool();
                return if set { result } else { NOT_SET };
            }
            37..=40 => texture_transform(e, slot_map(e, 0), 0x25),
            41..=44 => texture_transform(e, slot_map(e, 4), 0x29),
            45..=48 => texture_transform(e, slot_map(e, 8), 0x2d),
            49..=52 => texture_transform(e, slot_map(e, 0xc), 0x31),
            53..=56 => texture_transform(e, slot_map(e, 0x10), 0x35),
            57..=60 => texture_transform(e, slot_map(e, 0x14), 0x39),
            88..=91 => texture_transform(e, slot_map(e, 0x18), 0x58),
            92..=95 => texture_transform(e, slot_map(e, 0x1c), 0x5c),
            61..=64 => {
                let index = entry_index(e);
                let map = slot_map(e, index.wrapping_mul(4).wrapping_add(0x20));
                texture_transform(e, map, 0x3d)
            }
            65..=68 => {
                let texturing = texturing(e);
                let map = if texturing != 0 {
                    let index = entry_index(e);
                    e.call(MAP_AT_00A463E0, &args![texturing, index]).u32()
                } else {
                    0
                };
                texture_transform(e, map, 0x41)
            }
            73..=77 => {
                let source = e.mem.u32(properties.wrapping_add(8));
                if source == 0 {
                    return 0;
                }
                let at = |offset: u32| source.wrapping_add(offset);
                let values = match id {
                    73 | 74 => [
                        copy_float(e, VECTOR_011F4998),
                        copy_float(e, VECTOR_011F4998 + 4),
                        copy_float(e, VECTOR_011F4998 + 8),
                        copy_float(e, at(0x3c)),
                    ],
                    75 => [
                        copy_float(e, at(0x1c)),
                        copy_float(e, at(0x20)),
                        copy_float(e, at(0x24)),
                        copy_float(e, at(0x3c)),
                    ],
                    76 => {
                        let held = e.mem.u32(at(0x34));
                        let from = if held != 0 { held } else { at(0x28) };
                        [
                            copy_float(e, from),
                            copy_float(e, from.wrapping_add(4)),
                            copy_float(e, from.wrapping_add(8)),
                            copy_float(e, at(0x3c)),
                        ]
                    }
                    _ => [copy_float(e, at(0x38)); 4],
                };
                store_vector(e, vector, values);
                vector
            }
            78 | 79 => {
                let from = renderer(e).wrapping_add(0xa40);
                copy_matrix(e, from, first_matrix);
                let row = first_matrix + if id == 78 { 0x30 } else { 0x20 };
                let values = [0u32, 4, 8, 0xc].map(|offset| copy_float(e, row + offset));
                store_vector(e, vector, values);
                vector
            }
            80 | 81 => 0,
            82..=86 => {
                let handle = e.mem.u32(EXTRA_DATA_NAME_011BF65C);
                let held = e
                    .call(FIXED_STRING_CONSTRUCT_00438170, &args![name, handle])
                    .u32();
                let extra = e.call(GET_EXTRA_DATA_00A5BDD0, &args![object, held]).u32();
                e.call(FIXED_STRING_DESTRUCT_004381B0, &args![name]);
                if extra == 0 {
                    result = NO_EXTRA_DATA;
                }
                let time = copy_float(e, extra.wrapping_add(0xc));
                e.mem.set_u32(pair, time);
                let values = match id - 0x52 {
                    0 => [time; 4],
                    1 => [crt_on(e, CRT_HELPER_00ECA0A0, time); 4],
                    2 => [crt_on(e, CRT_HELPER_00EC9F70, time); 4],
                    3 => [crt_on(e, CRT_HELPER_00ECA5D0, time); 4],
                    4 => [
                        time,
                        crt_on(e, CRT_HELPER_00ECA0A0, time),
                        crt_on(e, CRT_HELPER_00EC9F70, time),
                        crt_on(e, CRT_HELPER_00ECA5D0, time),
                    ],
                    // The code's inner switch answers 0x10 for any other value (82 to 86 never
                    // reach it).
                    _ => return NO_EXTRA_DATA,
                };
                store_vector(e, vector, values);
                vector
            }
            87 => {
                store_vector(e, vector, [0, 0, 0, ONE]);
                let mut node = if effects != 0 {
                    e.mem.u32(effects.wrapping_add(0xc))
                } else {
                    0
                };
                while node != 0 {
                    let light = e.mem.u32(node.wrapping_add(4));
                    node = e.mem.u32(node);
                    if light == 0 {
                        continue;
                    }
                    let found = e
                        .call(LIGHT_CAST_00653290, &args![light, LIGHT_RECORD_011F5F6C])
                        .u32();
                    if found == 0 {
                        continue;
                    }
                    let strength = copy_float(e, found.wrapping_add(0xc4));
                    e.call(
                        LIGHT_OFFSET_00633B40,
                        &args![found.wrapping_add(0xc8), single, strength],
                    );
                    for component in 0..3u32 {
                        let at = vector + 4 * component;
                        let sum = x87_add(
                            x87_load32(e.mem.u32(at)),
                            x87_m32(e.mem.u32(single + 4 * component)),
                        );
                        e.mem.set_u32(at, x87_store32(sum));
                    }
                }
                vector
            }
            96..=99 => {
                if fourth == 0 {
                    return 0;
                }
                for word in 0..16u32 {
                    let value = if word % 5 == 0 { ONE } else { 0 };
                    e.mem.set_u32(first_matrix + 4 * word, value);
                }
                let (inverse, transpose) = flags(0x60);
                invert_transpose(e, first_matrix, inverse, transpose);
                first_matrix
            }
            103 => {
                store_vector(e, vector, [ONE; 4]);
                let map = slot_map(e, 0x14);
                if map != 0 {
                    let values = [0x18u32, 0x1c, 0x20, 0x24]
                        .map(|offset| copy_float(e, map.wrapping_add(offset)));
                    store_vector(e, vector, values);
                }
                vector
            }
            104 => {
                e.mem.set_u32(single, 0);
                e.mem.set_u32(single + 4, ONE);
                let map = slot_map(e, 0x14);
                if map != 0 {
                    let first = copy_float(e, map.wrapping_add(0x14));
                    e.mem.set_u32(single, first);
                    let second = copy_float(e, map.wrapping_add(0x10));
                    e.mem.set_u32(single + 4, second);
                }
                single
            }
            105 => {
                e.mem.set_u32(single, 0);
                let map = slot_map(e, 0x1c);
                if map != 0 {
                    let value = copy_float(e, map.wrapping_add(0x10));
                    e.mem.set_u32(single, value);
                }
                single
            }
            106..=115 => {
                let texturing = texturing(e);
                if texturing == 0 {
                    return 0;
                }
                let map = match id - 0x6a {
                    slot @ 0..=7 => map_word(e, texturing, 4 * slot),
                    8 => {
                        let index = entry_index(e);
                        map_word(e, texturing, index.wrapping_mul(4).wrapping_add(0x20))
                    }
                    9 => {
                        let index = entry_index(e);
                        e.call(MAP_AT_00A463E0, &args![texturing, index]).u32()
                    }
                    // The inner switch's default leaves no map (106 to 115 never reach it).
                    _ => 0,
                };
                e.mem.set_u32(pair, 0);
                e.mem.set_u32(pair + 4, 0);
                if map != 0 {
                    let texture = e.mem.u32(map.wrapping_add(8));
                    if texture != 0 {
                        for (slot, at) in [(0x94u32, pair), (0x98, pair + 4)] {
                            let size = e.vcall(texture, slot, &args![]).u32();
                            e.mem.set_u32(single, size);
                            let value = unsigned_to_float(e, size);
                            e.mem.set_u32(at, value);
                        }
                    }
                }
                pair
            }
            _ => return 0,
        };
        if !e.vcall(program, 0x74, &args![entry, data, 0u32]).bool() {
            result = NOT_SET;
        }
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_1d00, fn_00e81d00(Ptr, Ptr)),
        entry!(
            0x00e8_1ba0,
            ni_shader_constant_map_entry_ni_shader_constant_map_entry(Ptr) -> Ptr
        ),
        entry!(0x00e8_1c00, fn_00e81c00(Ptr)),
        entry!(
            0x00e8_3630,
            ni_shader_constant_map_entry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_6f40, ni_t_array_set_at(Ptr, u32, Ptr)),
        entry!(0x00e8_7510, fn_00e87510(Ptr, u32)),
        entry!(0x00e8_7800, ni_t_array_add_first_empty(Ptr, Ptr) -> u32),
        entry!(
            0x00e8_78c0,
            ni_d3d_shader_constant_map_insert_entry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_7920,
            ni_d3d_shader_constant_map_insert_defined_entry(Ptr, Ptr) -> u32
        ),
        entry!(0x00e8_70c0, fn_00e870c0(Ptr, Ptr)),
        entry!(0x00e8_7200, ni_d3d_shader_constant_map_sort_by_enabled(Ptr)),
        entry!(
            0x00e8_2640,
            ni_d3d_shader_constant_map_get_entry(Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_1d70,
            ni_d3d_shader_constant_map_add_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_1f10,
            ni_d3d_shader_constant_map_add_predefined_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, bool
            ) -> u32
        ),
        entry!(
            0x00e8_3650,
            ni_d3d_shader_constant_map_add_constant_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, bool
            ) -> u32
        ),
        entry!(
            0x00e8_29b0,
            ni_d3d_shader_constant_map_setup_predefined_entry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_26d0,
            ni_d3d_shader_constant_map_set_shader_constants(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> bool
        ),
        entry!(0x00e8_7690, fn_00e87690(Ptr)),
        entry!(
            0x00e8_3db0,
            ni_d3d_shader_constant_map_set_defined_constant(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(0x00a9_beb0, fn_00a9beb0(Ptr, u32) -> Ptr),
        entry!(0x00e8_1b00, fn_00e81b00(Ptr, Ptr)),
        entry!(
            0x00e8_1b60,
            ni_scm_extra_data_get_next_entry(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e8_1c60, fn_00e81c60(Ptr, Ptr)),
        entry!(0x00e8_1ca0, fn_00e81ca0(Ptr, u32, u32, u32, u8)),
        entry!(
            0x00e8_26c0,
            ni_d3d_shader_constant_map_get_entry_at_index(Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_2960,
            ni_d3d_shader_constant_map_get_entry_index(Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_2ac0,
            ni_d3d_shader_constant_map_setup_object_entry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_4b60,
            ni_d3d_shader_constant_map_set_constant_constant(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_7010,
            ni_d3d_shader_constant_map_remove_entry(Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_3a90,
            ni_d3d_shader_constant_map_map_constant_value(Ptr) -> u32
        ),
        entry!(
            0x00e8_2090,
            ni_d3d_shader_constant_map_add_attribute_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u8
            ) -> u32
        ),
        entry!(
            0x00e8_21e0,
            ni_d3d_shader_constant_map_add_global_entry(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u8
            ) -> u32
        ),
        entry!(
            0x00e8_23d0,
            ni_d3d_shader_constant_map_add_operator_entry(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_2500,
            ni_d3d_shader_constant_map_add_object_entry(Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_2b40,
            ni_d3d_shader_constant_map_perform_operator_multiply(u32, u32, u32, u32, u8, u8) -> u32
        ),
        entry!(
            0x00e8_2e70,
            ni_d3d_shader_constant_map_perform_operator_divide(u32, u32, u32, u32, u8, u8) -> u32
        ),
        entry!(0x00e8_30f0, fn_00e830f0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_3320, fn_00e83320(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00e8_3550,
            ni_d3d_shader_constant_map_setup_texture_transform_matrix(Ptr, Ptr, u8, u8, u32)
        ),
        entry!(
            0x00e8_3840,
            ni_d3d_shader_constant_map_map_attribute_value(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00e8_4ba0,
            ni_d3d_shader_constant_map_set_attribute_constant(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_4d00,
            ni_d3d_shader_constant_map_set_global_constant(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e8_5c40,
            ni_d3d_shader_constant_map_set_object_constant(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_6a50,
            ni_d3d_shader_constant_map_setup_constant_constant_value(u32, Ptr, u32) -> u32
        ),
        entry!(
            0x00e8_6ca0,
            ni_d3d_shader_constant_map_setup_attribute_constant_value(
                Ptr, u32, Ptr, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_7220,
            ni_d3d_shader_constant_map_set_operator_constant(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_5d10,
            ni_d3d_shader_constant_map_setup_defined_constant_value(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> u32
        ),
        entry!(
            0x00e8_4ed0,
            ni_d3d_shader_constant_map_get_dynamic_effect_data(
                Ptr, u32, u32, Ptr, u32, u32, u32, u32, u32, u32, Ptr, Ptr, u32
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e00_0000;
    const ADD_REFERENCE: u32 = 0x0e01_0004;
    const RELEASE: u32 = 0x0e01_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e81d00_swaps_the_device_and_takes_the_render_state() {
        let mut e = Engine::new();
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE, RELEASE]);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let old = e.mem.alloc(4);
        let device = e.mem.alloc(4);
        e.mem.set_u32(old, DEVICE_VTABLE);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x0e02_0000);
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map + 0x38, old);
        e.call(0x00e8_1d00, &args![map, renderer]);
        assert_eq!(e.mem.u32(map + 0x3c), renderer);
        assert_eq!(e.mem.u32(map + 0x38), device);
        assert_eq!(e.mem.u32(map + 0x40), 0x0e02_0000);
        assert_eq!(calls(&e, RELEASE), vec![vec![old]]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![device]]);
        // No renderer: the device released and both words cleared.
        e.call(0x00e8_1d00, &args![map, 0u32]);
        assert_eq!(calls(&e, RELEASE), vec![vec![old], vec![device]]);
        assert_eq!(e.mem.u32(map + 0x38), 0);
        assert_eq!(e.mem.u32(map + 0x40), 0);
        assert_eq!(e.mem.u32(map + 0x3c), 0);
    }
}

#[cfg(test)]
mod b054_tests {
    use super::*;

    /// A vtable whose delete slot (+4) is a recording double, and the slot.
    const OBJECT_VTABLE: u32 = 0x0e30_0000;
    const OBJECT_DELETE: u32 = 0x0e31_0004;
    /// The map's own vtable for the tests (slots by offset at `MAP_SLOT + offset`).
    const MAP_TEST_VTABLE: u32 = 0x0e32_0000;
    const MAP_SLOT: u32 = 0x0e33_0000;
    /// The device's vtable (slots at `DEVICE_SLOT + offset`).
    const DEVICE_VTABLE: u32 = 0x0e34_0000;
    const DEVICE_SLOT: u32 = 0x0e35_0000;
    /// The null fixed-string handle the tests map at `0109b220`.
    const NULL_HANDLE: u32 = 0x0e36_0008;
    /// The delete slot (0) of the entry vtable in the tests.
    const ENTRY_DELETE: u32 = 0x0e37_0000;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(OBJECT_DELETE, |_, _| Ret::default());
        e.register(ENTRY_DELETE, |_, _| Ret::default());
        e.put_vtable(OBJECT_VTABLE, &[0, OBJECT_DELETE]);
        e.put_vtable(ENTRY_VTABLE_010EF4D8, &[ENTRY_DELETE, OBJECT_DELETE]);
        let map_slots: Vec<u32> = (0..0x2c).map(|slot| MAP_SLOT + 4 * slot).collect();
        e.put_vtable(MAP_TEST_VTABLE, &map_slots);
        let device_slots: Vec<u32> = (0..0x70).map(|slot| DEVICE_SLOT + 4 * slot).collect();
        e.put_vtable(DEVICE_VTABLE, &device_slots);
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_NULL_0109B220, NULL_HANDLE);
        e.map(0x011f_4000, 0x2000);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// A reference-counted object with the given count.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// An entry-like object (count 1) with `m_bEnabled`, register, flags and extra word.
    fn entry(e: &mut Engine, enabled: bool, register: u32, flags: u32, extra: u32) -> u32 {
        let entry = object(e, 1);
        e.mem.set_u8(entry + 8, u8::from(enabled));
        e.mem.set_u32(entry + 0x1c, register);
        e.mem.set_u32(entry + 0x14, flags);
        e.mem.set_u32(entry + 0x18, extra);
        entry
    }

    /// An array (0x10 bytes) over the given elements, capacity `max`, grow-by 2.
    fn array(e: &mut Engine, elements: &[u32], max: u16) -> u32 {
        let array = e.mem.alloc(0x10);
        let block = e.mem.alloc(4 * u32::from(max).max(1));
        for (index, element) in elements.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *element);
        }
        e.mem.set_u32(array + 4, block);
        e.mem.set_u16(array + 8, max);
        e.mem.set_u16(array + 0xa, elements.len() as u16);
        let used = elements.iter().filter(|x| **x != 0).count() as u16;
        e.mem.set_u16(array + 0xc, used);
        e.mem.set_u16(array + 0xe, 2);
        array
    }

    fn elements(e: &Engine, array: u32) -> Vec<u32> {
        let size = u32::from(e.mem.u16(array + 0xa));
        let block = e.mem.u32(array + 4);
        (0..size)
            .map(|index| e.mem.u32(block + 4 * index))
            .collect()
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A map (0x44 bytes) with the test vtable and its two arrays (capacity 0x10 each).
    fn map(e: &mut Engine, entries: &[u32], defined: &[u32]) -> u32 {
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map, MAP_TEST_VTABLE);
        let entries = array(e, entries, 0x10);
        let defined = array(e, defined, 0x10);
        let bytes = e.mem.bytes(entries, 0x10);
        e.mem.write(map + 0xc, &bytes);
        let bytes = e.mem.bytes(defined, 0x10);
        e.mem.write(map + 0x1c, &bytes);
        map
    }

    #[test]
    fn entry_constructor_sets_up_the_entry() {
        let mut e = rig();
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xcc; 0x38]);
        let answer = e.call(0x00e8_1ba0, &args![this]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), ENTRY_VTABLE_010EF4D8);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), NULL_HANDLE);
        assert_eq!(e.mem.u32(this + 0x10), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 0x14, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(this + 0x24), NULL_HANDLE);
        assert_eq!(e.mem.bytes(this + 0x28, 0xf), vec![0; 0xf]);
        assert_eq!(e.mem.u8(this + 0x37), 0xcc);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 1);
    }

    #[test]
    fn entry_destructor_frees_owned_data_and_releases_the_names() {
        let mut e = rig();
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.set_global(NI_REF_OBJECT_COUNT_011F4400, 3u32);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 2);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0xc, NULL_HANDLE);
        e.mem.set_u32(this + 0x24, name);
        e.mem.set_u32(this + 0x30, 0x0e40_0000);
        e.mem.set_u8(this + 0x34, 1);
        e.call(0x00e8_1c00, &args![this]);
        assert_eq!(calls(&e, NI_FREE_DATA_00AA10F0), vec![vec![0x0e40_0000]]);
        assert_eq!(e.mem.u32(name - 8), 1);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 2);
        // Without owned data nothing is freed.
        e.mem.set_u8(this + 0x34, 0);
        e.call(0x00e8_1c00, &args![this]);
        assert_eq!(calls(&e, NI_FREE_DATA_00AA10F0).len(), 1);
    }

    #[test]
    fn entry_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0xc, NULL_HANDLE);
        e.mem.set_u32(this + 0x24, NULL_HANDLE);
        assert_eq!(e.call(0x00e8_3630, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.call(0x00e8_3630, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x38]]);
    }

    #[test]
    fn set_at_counts_and_assigns() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let old = object(&mut e, 1);
        let new = object(&mut e, 0);
        let array = array(&mut e, &[old, 0], 4);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, new);
        // Below the size: a null slot made non-null raises the element count.
        e.call(0x00e8_6f40, &args![array, 1u32, element]);
        assert_eq!(elements(&e, array), vec![old, new]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(e.mem.u32(new + 4), 1);
        // A non-null slot made null lowers it and releases the old object.
        e.mem.set_u32(element, 0);
        e.call(0x00e8_6f40, &args![array, 0u32, element]);
        assert_eq!(elements(&e, array), vec![0, new]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![old]]);
        // Past the size: the size follows the index.
        e.mem.set_u32(element, old);
        e.call(0x00e8_6f40, &args![array, 3u32, element]);
        assert_eq!(e.mem.u16(array + 0xa), 4);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(elements(&e, array)[3], old);
        // The null element is set up once.
        assert_eq!(
            calls(&e, ATEXIT),
            vec![vec![NULL_ELEMENT_DESTRUCT_00FDE390]]
        );
        assert_eq!(e.global::<u32>(NULL_ELEMENT_GUARD_0126F914) & 1, 1);
    }

    #[test]
    fn set_size_shrinks_and_moves_the_elements() {
        let mut e = rig();
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(NEW_POINTER_ARRAY_00B79A60, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        let (a, b, c) = (object(&mut e, 1), object(&mut e, 1), object(&mut e, 1));
        let array = array(&mut e, &[a, b, c], 4);
        let old = e.mem.u32(array + 4);
        // The same capacity does nothing.
        e.call(0x00e8_7510, &args![array, 4u32]);
        assert!(calls(&e, DELETE_POINTER_ARRAY_00B7CE30).is_empty());
        e.call(0x00e8_7510, &args![array, 2u32]);
        assert_eq!(e.mem.u16(array + 8), 2);
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![c]]);
        assert_eq!(elements(&e, array), vec![a, b]);
        assert_ne!(e.mem.u32(array + 4), old);
        // The moved elements are referenced again; the old block is not released.
        assert_eq!(e.mem.u32(a + 4), 2);
        assert_eq!(calls(&e, NEW_POINTER_ARRAY_00B79A60), vec![vec![2]]);
        assert_eq!(calls(&e, DELETE_POINTER_ARRAY_00B7CE30), vec![vec![old]]);
        // Capacity 0 leaves no elements.
        e.call(0x00e8_7510, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 0xa), 0);
    }

    #[test]
    fn add_first_empty_fills_a_hole_or_appends() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(NEW_POINTER_ARRAY_00B79A60, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        let (a, b) = (object(&mut e, 1), object(&mut e, 0));
        let array = array(&mut e, &[a, 0], 2);
        let element = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00e8_7800, &args![array, element]).u32(),
            0xffff_ffff
        );
        e.mem.set_u32(element, b);
        assert_eq!(e.call(0x00e8_7800, &args![array, element]).u32(), 1);
        assert_eq!(elements(&e, array), vec![a, b]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        // Full: grows by the grow-by count (2) and appends at the size.
        assert_eq!(e.call(0x00e8_7800, &args![array, element]).u32(), 2);
        assert_eq!(calls(&e, NEW_POINTER_ARRAY_00B79A60), vec![vec![4]]);
        assert_eq!(elements(&e, array), vec![a, b, b]);
        assert_eq!(e.mem.u16(array + 0xc), 3);
    }

    #[test]
    fn insert_entry_reports_a_failed_add() {
        let mut e = rig();
        let entry = object(&mut e, 1);
        let map = map(&mut e, &[0], &[]);
        e.mem.set_u32(map + 0x34, 7);
        assert_eq!(e.call(0x00e8_78c0, &args![map, entry]).u32(), 7);
        let block = e.mem.u32(map + 0x10);
        assert_eq!(e.mem.u32(block), entry);
        assert_eq!(e.mem.u32(entry + 4), 2);
        assert_eq!(e.call(0x00e8_78c0, &args![map, 0u32]).u32(), 0x30);
    }

    #[test]
    fn insert_defined_entry_adds_to_the_defined_array() {
        let mut e = rig();
        let entry = object(&mut e, 1);
        let map = map(&mut e, &[], &[0]);
        e.mem.set_u32(map + 0x34, 0);
        assert_eq!(e.call(0x00e8_7920, &args![map, entry]).u32(), 0);
        let block = e.mem.u32(map + 0x20);
        assert_eq!(e.mem.u32(block), entry);
        assert_eq!(e.call(0x00e8_7920, &args![map, 0u32]).u32(), 0x30);
    }

    #[test]
    fn sort_step_moves_enabled_entries_first() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let disabled_a = entry(&mut e, false, 0, 0, 0);
        let enabled_b = entry(&mut e, true, 0, 0, 0);
        let disabled_c = entry(&mut e, false, 0, 0, 0);
        let enabled_d = entry(&mut e, true, 0, 0, 0);
        let sorted = array(&mut e, &[disabled_a, enabled_b, disabled_c, enabled_d], 4);
        e.call(0x00e8_70c0, &args![0u32, sorted]);
        assert_eq!(
            elements(&e, sorted),
            vec![enabled_b, enabled_d, disabled_c, disabled_a]
        );
        for entry in [disabled_a, enabled_b, disabled_c, enabled_d] {
            assert_eq!(e.mem.u32(entry + 4), 1);
        }
        assert_eq!(e.mem.u16(sorted + 0xc), 4);
        // One element: nothing moves.
        let single = array(&mut e, &[disabled_a], 1);
        e.call(0x00e8_70c0, &args![0u32, single]);
        assert_eq!(elements(&e, single), vec![disabled_a]);
    }

    #[test]
    fn sort_by_enabled_sorts_both_arrays() {
        let mut e = rig();
        e.register(ATEXIT, |_, _| Ret::default());
        let disabled = entry(&mut e, false, 0, 0, 0);
        let enabled = entry(&mut e, true, 0, 0, 0);
        let map = map(&mut e, &[disabled, enabled], &[disabled, enabled]);
        e.call(0x00e8_7200, &args![map]);
        let entries = e.mem.u32(map + 0x10);
        let defined = e.mem.u32(map + 0x20);
        for block in [entries, defined] {
            assert_eq!(e.mem.u32(block), enabled);
            assert_eq!(e.mem.u32(block + 4), disabled);
        }
    }

    #[test]
    fn get_entry_looks_in_the_defined_entries_first() {
        let mut e = rig();
        e.register(NI_STRICMP, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let key = |e: &mut Engine, word: u32| {
            let text = e.mem.alloc(4);
            e.mem.set_u32(text, word);
            text
        };
        let (first, second, third) = (key(&mut e, 1), key(&mut e, 2), key(&mut e, 3));
        let plain = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(plain + 0xc, first);
        let defined = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(defined + 0xc, first);
        let other = entry(&mut e, true, 0, 0, 0);
        e.mem.set_u32(other + 0xc, second);
        let map = map(&mut e, &[0, other, plain], &[defined]);
        assert_eq!(e.call(0x00e8_2640, &args![map, first]).u32(), defined);
        assert_eq!(e.call(0x00e8_2640, &args![map, second]).u32(), other);
        assert_eq!(e.call(0x00e8_2640, &args![map, third]).u32(), 0);
    }

    #[test]
    fn add_entry_dispatches_by_kind() {
        let mut e = rig();
        let found = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x70, move |e, _| e.mem.u32(found).into_ret());
        for slot in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x68] {
            e.register(MAP_SLOT + slot, |_, a| (a.len() as u32).into_ret());
        }
        e.map(0x011f_5000, 0x1000);
        e.set_global(OBJECT_TABLE_011F5F90 + 4 * 3, 0x0e50_0000u32);
        let map = map(&mut e, &[], &[]);
        let words = |flags: u32| {
            args![map, 0x11u32, flags, 0x13u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32]
        };
        // Each kind's adder answers its word count (with `this`).
        for (flags, slot, count) in [
            (0x3000_0000u32, 0x10u32, 11u32),
            (0x1000_0000, 0x14, 11),
            (0x4000_0000, 0x18, 11),
            (0x5000_0000, 0x1c, 7),
            (0x2000_0000, 0x68, 9),
            (0x6000_0300, 0x20, 6),
        ] {
            assert_eq!(e.call(0x00e8_1d70, &words(flags)).u32(), count);
            assert_eq!(e.mem.u32(map + 0x34), count);
            assert!(!calls(&e, MAP_SLOT + slot).is_empty());
        }
        assert_eq!(
            calls(&e, MAP_SLOT + 0x68)[0],
            vec![map, 0x11, 0x13, 4, 6, 0, 0, 0, 0]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x20)[0],
            vec![map, 0x11, 4, 6, 0x13, 0x0e50_0000]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x1c)[0],
            vec![map, 0x11, 0x5000_0000, 0x13, 4, 5, 6]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x10)[0],
            vec![map, 0x11, 0x3000_0000, 0x13, 4, 5, 6, 7, 8, 9, 10]
        );
        // Any other kind is error 0x40.
        assert_eq!(e.call(0x00e8_1d70, &words(0x7000_0000)).u32(), 0x40);
        // An entry with the same extra word is error 0x20.
        let existing = entry(&mut e, true, 0, 0, 0x13);
        e.mem.set_u32(found, existing);
        assert_eq!(e.call(0x00e8_1d70, &words(0x1000_0000)).u32(), 0x20);
        assert_eq!(e.mem.u32(map + 0x34), 0x20);
    }

    /// The doubles for the entry builders: the 0x38-byte allocation, the fixed-string maker
    /// (handle = a block + 8 over a count of 1, holding the text address), the data allocator,
    /// free and `memcpy`.
    fn builder_rig() -> Engine {
        let mut e = rig();
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MAKE_FIXED_STRING_00A5B690, |e, a| {
            let handle = e.mem.alloc(0x10) + 8;
            e.mem.set_u32(handle - 8, 1);
            e.mem.set_u32(handle, a[0]);
            handle.into_ret()
        });
        e.register(NI_ALLOCATE_DATA_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(NI_FREE_DATA_00AA10F0, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn add_predefined_entry_builds_and_inserts() {
        let mut e = builder_rig();
        let answer = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x88, move |e, _| e.mem.u32(answer).into_ret());
        e.register(MAP_SLOT + 0x84, |_, _| 0x55u32.into_ret());
        let map = map(&mut e, &[], &[]);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0xdead_beef);
        let result = e
            .call(
                0x00e8_1f10,
                &args![
                    map,
                    0x0e60_0000u32,
                    2u32,
                    3u32,
                    0u32,
                    8u32,
                    4u32,
                    source,
                    1u32
                ],
            )
            .u32();
        assert_eq!(result, 0x55);
        assert_eq!(e.mem.u32(map + 0x34), 0x55);
        let entry = calls(&e, MAP_SLOT + 0x84)[0][1];
        let key = e.mem.u32(entry + 0xc);
        assert_eq!(e.mem.u32(key), 0x0e60_0000);
        // The kept key holds one reference: the made one's was dropped.
        assert_eq!(e.mem.u32(key - 8), 1);
        assert_eq!(e.mem.u32(entry + 0x24), NULL_HANDLE);
        assert_eq!(e.mem.u32(entry + 0x14), 0x2000_0000);
        assert_eq!(e.mem.u32(entry + 0x18), 2);
        assert_eq!(e.mem.u32(entry + 0x1c), 3);
        assert_eq!(e.mem.u32(entry + 0x28), 8);
        assert_eq!(e.mem.u32(entry + 0x2c), 4);
        assert_eq!(e.mem.u8(entry + 0x34), 1);
        let copy = e.mem.u32(entry + 0x30);
        assert_ne!(copy, source);
        assert_eq!(e.mem.u32(copy), 0xdead_beef);
        assert!(calls(&e, ENTRY_DELETE).is_empty());
        // A failed set-up deletes the entry (slot 0 with 1) and answers the error.
        e.mem.set_u32(answer, 0x40);
        let result = e
            .call(
                0x00e8_1f10,
                &args![map, 0u32, 2u32, 3u32, 0u32, 8u32, 4u32, source, 0u32],
            )
            .u32();
        assert_eq!(result, 0x40);
        let entry = calls(&e, MAP_SLOT + 0x88)[1][1];
        assert_eq!(e.mem.u32(entry + 0xc), NULL_HANDLE);
        assert_eq!(e.mem.u32(entry + 0x30), source);
        assert_eq!(e.mem.u8(entry + 0x34), 0);
        assert_eq!(calls(&e, ENTRY_DELETE), vec![vec![entry, 1]]);
        assert_eq!(calls(&e, MAP_SLOT + 0x84).len(), 1);
    }

    #[test]
    fn add_constant_entry_picks_the_flags_and_extra_word() {
        let mut e = builder_rig();
        let answer = e.mem.alloc(4);
        e.register_double(MAP_SLOT + 0x80, move |e, _| e.mem.u32(answer).into_ret());
        e.map(0x011f_5000, 0x1000);
        // Type 1 for flags byte 7: such entries keep the extra word.
        e.set_global(CONSTANT_TYPE_TABLE_011F5FC4 + 4 * 7, 1u32);
        let map = map(&mut e, &[], &[]);
        e.mem.set_u32(map + 8, 0);
        let build = |e: &mut Engine, size: u32, stride: u32| {
            e.call(
                0x00e8_3650,
                &args![
                    map,
                    0u32,
                    0xf000_0100u32,
                    9u32,
                    4u32,
                    2u32,
                    0u32,
                    size,
                    stride,
                    0x0e70_0000u32,
                    0u32
                ],
            )
            .u32()
        };
        let last = |e: &Engine| calls(e, MAP_SLOT + 0x80).last().unwrap()[1];
        assert_eq!(build(&mut e, 8, 4), 0);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0105);
        assert_eq!(e.mem.u32(entry + 0x18), 1);
        assert_eq!(e.mem.u32(entry + 0x1c), 4);
        assert_eq!(e.mem.u32(entry + 0x20), 2);
        assert_eq!(e.mem.u32(entry + 0x28), 8);
        assert_eq!(e.mem.u32(entry + 0x2c), 4);
        assert_eq!(e.mem.u32(entry + 0x30), 0x0e70_0000);
        // Size 0x10 is type 7 (table 1): the extra word stays; so for stride 8.
        build(&mut e, 0x10, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0107);
        assert_eq!(e.mem.u32(entry + 0x18), 9);
        build(&mut e, 8, 8);
        assert_eq!(e.mem.u32(last(&e) + 0x18), 9);
        // Program type 2 gives 2; another program type keeps the extra word.
        e.mem.set_u32(map + 8, 2);
        build(&mut e, 0x40, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0109);
        assert_eq!(e.mem.u32(entry + 0x18), 2);
        e.mem.set_u32(map + 8, 1);
        build(&mut e, 0x24, 4);
        let entry = last(&e);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1000_0108);
        assert_eq!(e.mem.u32(entry + 0x18), 9);
        build(&mut e, 0x14, 4);
        assert_eq!(e.mem.u32(last(&e) + 0x14), 0x1000_0100);
        // An insert error deletes the entry and is answered.
        e.mem.set_u32(answer, 0x30);
        assert_eq!(build(&mut e, 0xc, 4), 0x30);
        assert_eq!(e.mem.u32(map + 0x34), 0x30);
        assert_eq!(calls(&e, ENTRY_DELETE), vec![vec![last(&e), 1]]);
    }

    #[test]
    fn setup_predefined_entry_sets_the_mapping() {
        let mut e = rig();
        let lookup = e.mem.alloc(8);
        e.register_double(LOOK_UP_PREDEFINED_MAPPING, move |e, a| {
            let mapping = e.mem.u32(lookup);
            let count = e.mem.u32(lookup + 4);
            assert_eq!(e.mem.u32(a[2]), 4);
            e.mem.set_u32(a[1], mapping);
            e.mem.set_u32(a[2], count);
            u32::from(mapping != 0xffff).into_ret()
        });
        e.register(LOOK_UP_PREDEFINED_MAPPING_TYPE, |_, _| 2u32.into_ret());
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u8(PREDEFINED_TYPE_TABLE_011F5FB4 + 2, 0x0b);
        let entry = entry(&mut e, true, 0, 0x1234_5600, 0x0003_0000);
        let run = |e: &mut Engine, mapping: u32, count: u32| {
            e.mem.set_u32(lookup, mapping);
            e.mem.set_u32(lookup + 4, count);
            e.mem.set_u32(entry + 0x20, 0xaaaa);
            e.call(0x00e8_29b0, &args![0u32, entry]).u32()
        };
        assert_eq!(run(&mut e, 0xffff, 1), 0x40);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1234_5600);
        assert_eq!(run(&mut e, 0, 1), 0x40);
        assert_eq!(e.mem.u32(entry + 0x14), 0x1234_560b);
        assert_eq!(run(&mut e, 0x21, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 9);
        assert_eq!(e.mem.u32(entry + 0x10), 0x21);
        assert_eq!(run(&mut e, 0x24, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 12);
        assert_eq!(run(&mut e, 0x1c, 1), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 4);
        assert_eq!(run(&mut e, 0x30, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 7);
        assert_eq!(run(&mut e, 0x50, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x20), 0xaaaa);
        assert_eq!(e.mem.u32(entry + 0x30), PREDEFINED_DATA_011BF63C);
        assert_eq!(e.mem.u32(entry + 0x28), 0x10);
        assert_eq!(e.mem.u32(entry + 0x2c), 0x10);
        assert_eq!(run(&mut e, 0x51, 7), 0);
        assert_eq!(e.mem.u32(entry + 0x30), PREDEFINED_DATA_011BF64C);
    }

    #[test]
    fn set_shader_constants_sets_each_kind() {
        let mut e = rig();
        let failing = e.mem.alloc(4);
        for slot in [0x8cu32, 0x90, 0x94, 0x98, 0x9c, 0xa4] {
            e.register_double(MAP_SLOT + slot, move |e, a| {
                u32::from(a[2] == e.mem.u32(failing)).into_ret()
            });
        }
        e.register(DEVICE_SLOT + 0x178, |_, _| 0u32.into_ret());
        e.register(DEVICE_SLOT + 0x1b4, |_, _| 0x8000_0000u32.into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let defined = entry(&mut e, true, 1, 0x2000_0000, 0);
        let skipped = entry(&mut e, true, 0xffff_ffff, 0x1000_0000, 1);
        let disabled = entry(&mut e, false, 1, 0x1000_0000, 1);
        let vertex = entry(&mut e, true, 3, 0x1000_0000, 1);
        e.mem.set_u32(vertex + 0x30, 0x0e80_0000);
        e.mem.set_u32(vertex + 0x20, 2);
        let constant = entry(&mut e, true, 3, 0x1000_0000, 5);
        let attribute = entry(&mut e, true, 3, 0x3000_0000, 0);
        let global = entry(&mut e, true, 3, 0x4000_0000, 0);
        let operator = entry(&mut e, true, 3, 0x5000_0000, 0);
        let object_entry = entry(&mut e, true, 3, 0x6000_0000, 0);
        let predefined = entry(&mut e, true, 3, 0x2000_0000, 0);
        let map = map(
            &mut e,
            &[
                skipped,
                0,
                disabled,
                vertex,
                constant,
                attribute,
                global,
                operator,
                object_entry,
                predefined,
            ],
            &[defined],
        );
        e.mem.set_u32(map + 0x38, device);
        let words: Vec<u32> = (2..=11).collect();
        let mut all = vec![map, 0x0e90_0000];
        all.extend(&words);
        e.mem.set_u32(failing, 0xffff);
        assert!(!e.call(0x00e8_26d0, &all).bool());
        assert_eq!(e.mem.u32(map + 0x30), 0x0e90_0000);
        let mut expected = vec![map, 0x0e90_0000, defined];
        expected.extend(&words[..9]);
        assert_eq!(calls(&e, MAP_SLOT + 0x8c), vec![expected]);
        assert_eq!(
            calls(&e, DEVICE_SLOT + 0x178),
            vec![vec![device, 3, 0x0e80_0000, 2]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x90),
            vec![vec![map, 0x0e90_0000, constant, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x94)[0],
            vec![
                map,
                0x0e90_0000,
                attribute,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                0
            ]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x98),
            vec![vec![map, 0x0e90_0000, global, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0x9c),
            vec![vec![map, 0x0e90_0000, operator, 2, 3, 6, 7, 8, 9, 10]]
        );
        assert_eq!(
            calls(&e, MAP_SLOT + 0xa4)[0],
            vec![map, 0x0e90_0000, object_entry, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        // A failing setter, a negative device result or an unknown kind is reported.
        e.mem.set_u32(failing, global);
        assert!(e.call(0x00e8_26d0, &all).bool());
        e.mem.set_u32(failing, 0xffff);
        e.mem.set_u32(vertex + 0x18, 2);
        assert!(e.call(0x00e8_26d0, &all).bool());
        e.mem.set_u32(vertex + 0x18, 1);
        e.mem.set_u32(global + 0x14, 0x7000_0000);
        assert!(e.call(0x00e8_26d0, &all).bool());
    }

    #[test]
    fn map_destructor_releases_globals_and_the_device() {
        let mut e = rig();
        e.register(RELEASE_GLOBAL_SHADER_CONSTANT, |_, _| Ret::default());
        e.register(ARRAY_REMOVE_ALL_00B799C0, |_, _| Ret::default());
        e.register(DELETE_POINTER_ARRAY_00B7CE30, |_, _| Ret::default());
        e.register(DEVICE_SLOT + 8, |_, _| Ret::default());
        e.set_global(NI_REF_OBJECT_COUNT_011F4400, 5u32);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        let global = entry(&mut e, true, 0, 0x4000_0000, 0);
        e.mem.set_u32(global + 0xc, name);
        let plain = entry(&mut e, true, 0, 0x1000_0000, 0);
        let defined_global = entry(&mut e, true, 0, 0x4000_0000, 0);
        let map = map(&mut e, &[global, 0, plain], &[defined_global]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(map + 0x38, device);
        e.mem.set_u32(map + 0x3c, 0x1234);
        e.mem.set_u32(map + 0x40, 0x5678);
        let entries = e.mem.u32(map + 0x10);
        let defined = e.mem.u32(map + 0x20);
        e.call(0x00e8_7690, &args![map]);
        let released = calls(&e, RELEASE_GLOBAL_SHADER_CONSTANT);
        assert_eq!(released.len(), 2);
        assert_eq!(released[1], vec![defined_global + 0xc]);
        // The handle was held across the call and dropped again.
        assert_eq!(e.mem.u32(name - 8), 1);
        for entry in [global, plain, defined_global] {
            assert_eq!(e.mem.u32(entry + 4), 1);
        }
        assert_eq!(
            calls(&e, ARRAY_REMOVE_ALL_00B799C0),
            vec![vec![map + 0xc], vec![map + 0x1c]]
        );
        assert_eq!(calls(&e, DEVICE_SLOT + 8), vec![vec![device]]);
        for offset in [0x38u32, 0x3c, 0x40] {
            assert_eq!(e.mem.u32(map + offset), 0);
        }
        assert_eq!(
            calls(&e, DELETE_POINTER_ARRAY_00B7CE30),
            vec![vec![defined], vec![entries]]
        );
        assert_eq!(e.mem.u32(map + 0xc), ENTRY_ARRAY_VTABLE_010EF53C);
        assert_eq!(e.mem.u32(map + 0x1c), ENTRY_ARRAY_VTABLE_010EF53C);
        assert_eq!(e.mem.u32(map), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT_011F4400), 4);
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const PROGRAM_VTABLE: u32 = 0x0e00_0000;
    const SET_CONSTANT: u32 = 0x0e01_0074;
    const SET_CONSTANT_ARRAY: u32 = 0x0e01_0078;
    const TEXTURE_VTABLE: u32 = 0x0e02_0000;
    const TEXTURE_WIDTH: u32 = 0x0e03_0094;
    const TEXTURE_HEIGHT: u32 = 0x0e03_0098;

    /// What the program's `SetShaderConstant` saw: the data address and its first 16 words.
    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        map: u32,
        program: u32,
        entry: u32,
        renderer: u32,
        properties: u32,
        seen: Seen,
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A constant map whose renderer holds the words `0x9400 + i` at +0x940, `0x9800 + i` at
    /// +0x980, `0x9c00 + i` at +0x9c0, `0xa000 + i` at +0xa00 and `0xa400 + i` at +0xa40; a
    /// program that accepts (`accepts`) every constant.
    fn rig(accepts: bool) -> Rig {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x2000);
        e.map(0x0109_9000, 0x1000);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        let mut slots = vec![0u32; 0x1f];
        slots[0x74 / 4] = SET_CONSTANT;
        slots[0x78 / 4] = SET_CONSTANT_ARRAY;
        e.put_vtable(PROGRAM_VTABLE, &slots);
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(SET_CONSTANT, move |e, a| {
            let words = if a[2] != 0 {
                (0..16).map(|i| e.mem.u32(a[2] + 4 * i)).collect()
            } else {
                vec![]
            };
            log.borrow_mut().push((a[2], words));
            u32::from(accepts).into_ret()
        });
        e.register_double(SET_CONSTANT_ARRAY, move |_, _| {
            u32::from(accepts).into_ret()
        });
        for callee in [
            MATRIX_INVERSE_00EE6DDA,
            MATRIX_TRANSPOSE_00EE6DD4,
            MATRIX_MULTIPLY_00EE6DE0,
            MATRIX_PRODUCT_00EE6DFE,
            SETUP_TEXTURE_TRANSFORM_00E83550,
            FIXED_STRING_DESTRUCT_004381B0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let renderer = e.mem.alloc(0xa80);
        for (offset, tag) in [
            (0x940u32, 0x9400u32),
            (0x980, 0x9800),
            (0x9c0, 0x9c00),
            (0xa00, 0xa000),
            (0xa40, 0xa400),
        ] {
            for i in 0..16 {
                e.mem.set_u32(renderer + offset + 4 * i, tag + i);
            }
        }
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map + 0x3c, renderer);
        let program = e.mem.alloc(4);
        e.mem.set_u32(program, PROGRAM_VTABLE);
        let entry = e.mem.alloc(0x38);
        let properties = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        Rig {
            e,
            map,
            program,
            entry,
            renderer,
            properties,
            seen,
        }
    }

    impl Rig {
        /// The function for constant `id` with the fourth, fifth and effect words.
        fn run(&mut self, id: u32, fourth: u32, fifth: u32, effects: u32) -> u32 {
            self.e.mem.set_u32(self.entry + 0x10, id);
            self.seen.borrow_mut().clear();
            self.e.call_log = Some(vec![]);
            let call = args![
                self.map,
                self.program,
                self.entry,
                0x0e40_0000u32,
                fourth,
                fifth,
                0u32,
                self.properties,
                effects,
                0u32,
                0u32,
                0u32
            ];
            self.e.call(0x00e8_3db0, &call).u32()
        }

        fn seen(&self) -> Vec<(u32, Vec<u32>)> {
            self.seen.borrow().clone()
        }

        fn tags(&self, tag: u32) -> Vec<u32> {
            (0..16).map(|i| tag + i).collect()
        }

        fn floats(&self, count: usize) -> Vec<f32> {
            self.seen()[0].1[..count]
                .iter()
                .map(|w| f32::from_bits(*w))
                .collect()
        }
    }

    #[test]
    fn renderer_matrices_are_copied_inverted_and_transposed() {
        let mut r = rig(true);
        assert_eq!(r.run(1, 0, 0, 0), 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9c00));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        r.run(14, 0, 0, 0);
        let data = r.seen()[0].0;
        assert_eq!(
            calls(&r.e, MATRIX_INVERSE_00EE6DDA),
            vec![vec![data, 0, data]]
        );
        assert_eq!(
            calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4),
            vec![vec![data, data]]
        );
        r.run(16, 0, 0, 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9800));
        r.run(17, 0, 0, 0);
        assert_eq!(r.seen()[0].1, r.tags(0x9400));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        // The program refusing: 0x50.
        let mut r = rig(false);
        assert_eq!(r.run(2, 0, 0, 0), NOT_SET);
    }

    #[test]
    fn products_multiply_the_renderer_matrices_in_order() {
        let mut r = rig(true);
        let factors: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = factors.clone();
        r.e.register_double(MATRIX_MULTIPLY_00EE6DE0, move |e, a| {
            log.borrow_mut().push((e.mem.u32(a[1]), e.mem.u32(a[2])));
            Ret::default()
        });
        r.run(8, 0, 0, 0);
        assert_eq!(factors.borrow()[0], (0x9400, 0x9800));
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        r.run(21, 0, 0, 0);
        assert_eq!(factors.borrow()[1], (0x9800, 0x9c00));
        assert!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).is_empty());
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        let multiplied = calls(&r.e, MATRIX_MULTIPLY_00EE6DE0)[0][0];
        assert_eq!(r.seen()[0].0, multiplied);
        // 11 to 24: 00ee6dfe of +0x940 and +0xa00, handed over as it is.
        r.run(12, 0, 0, 0);
        let product = &calls(&r.e, MATRIX_PRODUCT_00EE6DFE)[0];
        assert_eq!(product[1..], [r.renderer + 0x940, r.renderer + 0xa00]);
        assert_eq!(r.seen()[0].0, product[0]);
    }

    #[test]
    fn constants_needing_the_fourth_word() {
        let mut r = rig(true);
        for id in [25u32, 30, 96] {
            assert_eq!(r.run(id, 0, 0, 0), 0);
            assert!(r.seen().is_empty());
        }
        r.run(28, 1, 0, 0);
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert_eq!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        r.run(97, 1, 0, 0);
        let one = 1.0f32.to_bits();
        let identity: Vec<u32> = (0..16).map(|i| if i % 5 == 0 { one } else { 0 }).collect();
        assert_eq!(r.seen()[0].1, identity);
        assert_eq!(calls(&r.e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert!(calls(&r.e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
    }

    #[test]
    fn skinning_constants_go_to_the_array_slot() {
        let mut r = rig(true);
        let fourth = r.e.mem.alloc(0x2c);
        r.e.mem.set_u32(fourth + 0x20, 8);
        r.e.mem.set_u32(fourth + 0x28, 0x0e50_0000);
        let fifth = r.e.mem.alloc(0x24);
        r.e.mem.set_u32(fifth + 4, 0x0e51_0000);
        r.e.mem.set_u16(fifth + 0x20, 3);
        assert_eq!(r.run(33, fourth, 0, 0), 0);
        assert!(calls(&r.e, SET_CONSTANT_ARRAY).is_empty());
        assert_eq!(r.run(36, fourth, fifth, 0), 0);
        assert_eq!(
            calls(&r.e, SET_CONSTANT_ARRAY),
            vec![vec![r.program, r.entry, 0x0e50_0000, 3, 8, 0x0e51_0000]]
        );
        let mut r2 = rig(false);
        let fourth = r2.e.mem.alloc(0x2c);
        let fifth = r2.e.mem.alloc(0x24);
        assert_eq!(r2.run(35, fourth, fifth, 0), NOT_SET);
    }

    /// A texturing property with map arrays: the map at `slot` holds a transform.
    fn texturing(r: &mut Rig, slot: u32) -> u32 {
        let texturing = r.e.mem.alloc(0x24);
        let maps = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(texturing + 0x20, maps);
        let map = r.e.mem.alloc(0x14);
        r.e.mem.set_u32(map + 0xc, 0x0e60_0000);
        r.e.mem.set_u32(maps + slot, map);
        r.e.mem.set_u32(r.properties + 0x14, texturing);
        map
    }

    #[test]
    fn texture_transforms_set_up_the_map_matrix() {
        let mut r = rig(true);
        r.e.register(UPDATE_MATRIX_00BC5C20, |_, _| 0x0e61_0000u32.into_ret());
        // No texturing property: no matrix, no flags.
        r.run(40, 0, 0, 0);
        let setup = &calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0];
        assert_eq!(setup[1..], [0, 0, 0, 4]);
        assert_eq!(r.seen()[0].0, setup[0]);
        texturing(&mut r, 4);
        r.run(42, 0, 0, 0);
        assert_eq!(calls(&r.e, UPDATE_MATRIX_00BC5C20), vec![vec![0x0e60_0000]]);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 0, 4]
        );
        r.run(44, 0, 0, 0);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 1, 4]
        );
        // Map 6 for 88 to 91; the entry's index for 61 to 64.
        texturing(&mut r, 0x18);
        r.run(90, 0, 0, 0);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 0, 1, 4]
        );
        r.e.mem.set_u32(r.entry + 0x18, 2);
        texturing(&mut r, 0x28);
        r.run(62, 0, 0, 0);
        assert_eq!(calls(&r.e, UPDATE_MATRIX_00BC5C20).len(), 1);
        // 65 to 68: the map 00a463e0 answers for the index.
        let map = texturing(&mut r, 0x3c);
        r.e.register_double(MAP_AT_00A463E0, move |_, _| map.into_ret());
        r.run(68, 0, 0, 0);
        let texturing = r.e.mem.u32(r.properties + 0x14);
        assert_eq!(calls(&r.e, MAP_AT_00A463E0), vec![vec![texturing, 2]]);
        assert_eq!(
            calls(&r.e, SETUP_TEXTURE_TRANSFORM_00E83550)[0][1..],
            [0x0e61_0000, 1, 1, 4]
        );
    }

    #[test]
    fn property_vectors_and_renderer_rows() {
        let mut r = rig(true);
        assert_eq!(r.run(75, 0, 0, 0), 0);
        assert!(r.seen().is_empty());
        let source = r.e.mem.alloc(0x40);
        for i in 0..16u32 {
            r.e.mem.set_u32(source + 4 * i, (i as f32).to_bits());
        }
        r.e.mem.set_u32(r.properties + 8, source);
        for i in 0..3u32 {
            r.e.mem
                .set_u32(VECTOR_011F4998 + 4 * i, (100.0 + i as f32).to_bits());
        }
        r.run(74, 0, 0, 0);
        assert_eq!(r.floats(4), vec![100.0, 101.0, 102.0, 15.0]);
        r.run(75, 0, 0, 0);
        assert_eq!(r.floats(4), vec![7.0, 8.0, 9.0, 15.0]);
        // 76 without a word at +0x34: the floats from +0x28 on.
        r.e.mem.set_u32(source + 0x34, 0);
        r.run(76, 0, 0, 0);
        assert_eq!(r.floats(4), vec![10.0, 11.0, 12.0, 15.0]);
        let held = r.e.mem.alloc(0xc);
        r.e.mem.set_u32(held, 0x7f80_0001);
        r.e.mem.set_u32(source + 0x34, held);
        r.run(76, 0, 0, 0);
        // The copy through FLD/FSTP quiets the signalling NaN.
        assert_eq!(r.seen()[0].1[0], 0x7fc0_0001);
        r.run(77, 0, 0, 0);
        assert_eq!(r.floats(4), vec![14.0; 4]);
        r.run(78, 0, 0, 0);
        assert_eq!(r.seen()[0].1[..4], [0xa40c, 0xa40d, 0xa40e, 0xa40f]);
        r.run(79, 0, 0, 0);
        assert_eq!(r.seen()[0].1[..4], [0xa408, 0xa409, 0xa40a, 0xa40b]);
        r.run(81, 0, 0, 0);
        assert_eq!(r.seen()[0].0, 0);
    }

    #[test]
    fn time_constants_read_the_extra_data() {
        let mut r = rig(true);
        r.e.mem.set_u32(EXTRA_DATA_NAME_011BF65C, 0x0e70_0000);
        r.e.register(FIXED_STRING_CONSTRUCT_00438170, |_, a| a[0].into_ret());
        let extra = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(extra + 0xc, 2.0f32.to_bits());
        r.e.register_double(GET_EXTRA_DATA_00A5BDD0, move |_, _| extra.into_ret());
        r.e.register(CRT_HELPER_00ECA0A0, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) + 1.0,
            ..Ret::default()
        });
        r.e.register(CRT_HELPER_00EC9F70, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        r.e.register(CRT_HELPER_00ECA5D0, |_, _| Ret {
            st0: -4.0,
            ..Ret::default()
        });
        assert_eq!(r.run(82, 0, 0, 0), 0);
        assert_eq!(r.floats(4), vec![2.0; 4]);
        let name = calls(&r.e, FIXED_STRING_CONSTRUCT_00438170)[0].clone();
        assert_eq!(name[1], 0x0e70_0000);
        assert_eq!(
            calls(&r.e, GET_EXTRA_DATA_00A5BDD0),
            vec![vec![0x0e40_0000, name[0]]]
        );
        assert_eq!(
            calls(&r.e, FIXED_STRING_DESTRUCT_004381B0),
            vec![vec![name[0]]]
        );
        r.run(83, 0, 0, 0);
        assert_eq!(r.floats(4), vec![3.0; 4]);
        r.run(86, 0, 0, 0);
        assert_eq!(r.floats(4), vec![2.0, 3.0, 0.5, -4.0]);
        // No extra data: the result 0x10, the time read at address 0xc.
        r.e.map(0, 0x1000);
        r.e.mem.set_u32(0xc, 5.0f32.to_bits());
        r.e.register(GET_EXTRA_DATA_00A5BDD0, |_, _| 0u32.into_ret());
        assert_eq!(r.run(84, 0, 0, 0), NO_EXTRA_DATA);
        assert_eq!(r.floats(4), vec![0.5; 4]);
    }

    #[test]
    fn light_constant_sums_the_lights() {
        let mut r = rig(true);
        let light = r.e.mem.alloc(0xd0);
        r.e.mem.set_u32(light + 0xc4, 0.25f32.to_bits());
        let second = r.e.mem.alloc(4);
        r.e.register_double(LIGHT_CAST_00653290, move |_, a| {
            if a[0] == second {
                0u32.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        r.e.register(LIGHT_OFFSET_00633B40, |e, a| {
            assert_eq!(f32::from_bits(a[2]), 0.25);
            for i in 0..3u32 {
                e.mem.set_u32(a[1] + 4 * i, (i as f32 + 1.0).to_bits());
            }
            Ret::default()
        });
        let effects = r.e.mem.alloc(0x10);
        let first_node = r.e.mem.alloc(8);
        let second_node = r.e.mem.alloc(8);
        let third_node = r.e.mem.alloc(8);
        r.e.mem.set_u32(effects + 0xc, first_node);
        r.e.mem.set_u32(first_node, second_node);
        r.e.mem.set_u32(first_node + 4, light);
        r.e.mem.set_u32(second_node, third_node);
        r.e.mem.set_u32(second_node + 4, second);
        r.e.mem.set_u32(third_node + 4, light);
        r.run(87, 0, 0, effects);
        assert_eq!(r.floats(4), vec![2.0, 4.0, 6.0, 1.0]);
        assert_eq!(calls(&r.e, LIGHT_OFFSET_00633B40)[0][0], light + 0xc8);
        r.run(87, 0, 0, 0);
        assert_eq!(r.floats(4), vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn map_values_and_texture_sizes() {
        let mut r = rig(true);
        r.run(103, 0, 0, 0);
        assert_eq!(r.floats(4), vec![1.0; 4]);
        r.run(104, 0, 0, 0);
        assert_eq!(r.floats(2), vec![0.0, 1.0]);
        r.run(105, 0, 0, 0);
        assert_eq!(r.floats(1), vec![0.0]);
        assert_eq!(r.run(106, 0, 0, 0), 0);
        assert!(r.seen().is_empty());
        let fifth_map = texturing(&mut r, 0x14);
        let seventh_map = r.e.mem.alloc(0x14);
        let maps = r.e.mem.u32(r.e.mem.u32(r.properties + 0x14) + 0x20);
        r.e.mem.set_u32(maps + 0x1c, seventh_map);
        let fifth = r.e.mem.alloc(0x28);
        for i in 0..10u32 {
            r.e.mem.set_u32(fifth + 4 * i, (i as f32).to_bits());
        }
        r.e.mem.set_u32(maps + 0x14, fifth);
        let _ = fifth_map;
        r.e.mem.set_u32(seventh_map + 0x10, 9.5f32.to_bits());
        r.run(103, 0, 0, 0);
        assert_eq!(r.floats(4), vec![6.0, 7.0, 8.0, 9.0]);
        r.run(104, 0, 0, 0);
        assert_eq!(r.floats(2), vec![5.0, 4.0]);
        r.run(105, 0, 0, 0);
        assert_eq!(r.floats(1), vec![9.5]);
        // The texture sizes: map 5 (constant 111) and the indexed map (114).
        let texture = r.e.mem.alloc(4);
        r.e.mem.set_u32(texture, TEXTURE_VTABLE);
        let slots: Vec<u32> = (0..0x27)
            .map(|i| match i * 4 {
                0x94 => TEXTURE_WIDTH,
                0x98 => TEXTURE_HEIGHT,
                _ => 0,
            })
            .collect();
        r.e.put_vtable(TEXTURE_VTABLE, &slots);
        r.e.register(TEXTURE_WIDTH, |_, _| 256u32.into_ret());
        r.e.register(TEXTURE_HEIGHT, |_, _| 0x8000_0000u32.into_ret());
        r.e.mem.set_u32(fifth + 8, texture);
        r.run(111, 0, 0, 0);
        assert_eq!(r.floats(2), vec![256.0, 2_147_483_648.0]);
        r.e.mem.set_u32(r.entry + 0x18, 1);
        r.e.mem.set_u32(maps + 0x24, fifth);
        r.run(114, 0, 0, 0);
        assert_eq!(r.floats(2), vec![256.0, 2_147_483_648.0]);
        // No map 6: zeros.
        r.run(112, 0, 0, 0);
        assert_eq!(r.floats(2), vec![0.0, 0.0]);
    }

    #[test]
    fn unknown_constants_answer_zero() {
        let mut r = rig(true);
        for id in [0u32, 70, 101, 116] {
            assert_eq!(r.run(id, 0, 0, 0), 0);
        }
        assert!(r.seen().is_empty());
    }
}

// ---- Batch n084 (a02) ----

/// `NiShaderConstantMap::LookUpObjectMapping` (`00a9c0e0`, engine map; `cdecl`: the key and
/// where the mapping goes; answers a `bool`) and `::LookUpObjectMappingType` (`00a9bcd0`,
/// `cdecl`: the mapping and two out words; answers a type index).
const LOOK_UP_OBJECT_MAPPING: u32 = 0x00a9_c0e0;
const LOOK_UP_OBJECT_MAPPING_TYPE: u32 = 0x00a9_bcd0;
/// The shift `NiSCMExtraData::GetNextEntry` moves the register by (`01101294`).
const ENTRY_KEY_SHIFT_01101294: u32 = 0x0110_1294;

// Translated from 00a9beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `NiShaderConstantMap` scalar deleting destructor (the body has nothing
/// of that class: it may be folded code), `thiscall`, `RET 4` (the flags): the `NiRefObject`
/// vtable (`0101dce4`), `ms_uiObjects` (`011f4400`) lowered, and with bit 0 of the flags the
/// 0xC bytes freed through `00aa1460`. Answers `this`.
pub fn fn_00a9beb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE_0101DCE4);
    interlocked_decrement(e, NI_REF_OBJECT_COUNT_011F4400);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00e81b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a register-argument helper of `GetDynamicEffectData`, the
/// destination in `ECX` and the source in `EAX` (taken here in that order): sixteen floats
/// copied (`FLD`/`FSTP`, a signalling NaN quieted).
pub fn fn_00e81b00(e: &mut Engine, destination: Ptr, source: Ptr) {
    for i in 0..16u32 {
        let bits = copy_float(e, source.addr().wrapping_add(4 * i));
        e.mem.set_u32(destination.addr().wrapping_add(4 * i), bits);
    }
}

// Translated from 00e81b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSCMExtraData::GetNextEntry` (engine map), `thiscall`, four stack words: the register,
/// the program type (shifted left by the word at `01101294`), the entry list's index (0 to 2)
/// and one not read. The list at `+0x24 + 4 * index` holds 8-byte pairs (a key, a value),
/// its cursor at `+0x18 + 4 * index`; when the pair under the cursor has the key
/// `type << shift | register`, the cursor moves on and its value is answered, else 0.
pub fn ni_scm_extra_data_get_next_entry(
    e: &mut Engine,
    this: Ptr,
    register: u32,
    program_type: u32,
    index: u32,
    _unused_4: u32,
) -> u32 {
    let t = this.addr();
    let shift = e.mem.u32(ENTRY_KEY_SHIFT_01101294);
    let key = program_type.wrapping_shl(shift) | register;
    let pairs = e.mem.u32(t.wrapping_add(0x24 + 4 * index));
    let cursor_at = t.wrapping_add(0x18 + 4 * index);
    let cursor = e.mem.u32(cursor_at);
    let pair = pairs.wrapping_add(cursor.wrapping_mul(8));
    if e.mem.u32(pair) != key {
        return 0;
    }
    let value = e.mem.u32(pair.wrapping_add(4));
    e.mem.set_u32(cursor_at, cursor.wrapping_add(1));
    value
}

// Translated from 00e81c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constant map entry's variable-name setter (`thiscall`, one
/// stack word: the address of an `NiFixedString` handle). When the entry's `m_kVariableName`
/// (+0x24) holds another handle, the new one's count goes up and the old one's down (neither
/// for the null handle `0109b220`) and the field takes the new handle.
pub fn fn_00e81c60(e: &mut Engine, this: Ptr, name: Ptr) {
    let field = this.addr().wrapping_add(0x24);
    let handle = e.mem.u32(name.addr());
    if e.mem.u32(field) == handle {
        return;
    }
    if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
        interlocked_increment(e, handle.wrapping_sub(8));
    }
    let old = e.mem.u32(field);
    if old != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
        interlocked_decrement(e, old.wrapping_sub(8));
    }
    e.mem.set_u32(field, handle);
}

// Translated from 00e81ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constant map entry's data setter (`thiscall`, four stack
/// words: the size, the stride, the source and whether to copy, its low byte). The size
/// (+0x28) and stride (+0x2C) are stored, then the data: copied into an owned block (the old
/// one freed, `m_bOwnData` set) or pointed at.
pub fn fn_00e81ca0(e: &mut Engine, this: Ptr, size: u32, stride: u32, source: u32, copy: u8) {
    let t = this.addr();
    e.mem.set_u32(t.wrapping_add(0x28), size);
    e.mem.set_u32(t.wrapping_add(0x2c), stride);
    set_data_source(e, t, size, source, copy != 0);
}

// Translated from 00e826c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::GetEntryAtIndex` (engine map), `thiscall`, one stack word: the
/// element of `m_aspEntries` (elements at +0x10) at that index (not checked).
pub fn ni_d3d_shader_constant_map_get_entry_at_index(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let elements = e.mem.u32(this.addr().wrapping_add(0x10));
    e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00e82960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::GetEntryIndex` (engine map), `thiscall`, one stack word (the key):
/// the index of the first entry of `m_aspEntries` (its 16-bit size at +0x16, read at the
/// start) whose key (+0xC) `NiStricmp` finds equal, else -1.
pub fn ni_d3d_shader_constant_map_get_entry_index(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let t = this.addr();
    let size = u32::from(e.mem.u16(t.wrapping_add(0x16)));
    let mut index = 0u32;
    while index < size {
        let elements = e.mem.u32(t.wrapping_add(0x10));
        let entry = e.mem.u32(elements.wrapping_add(index * 4));
        if entry != 0 {
            let name = e.mem.u32(entry.wrapping_add(0xc));
            if e.call(NI_STRICMP, &args![key, name]).u32() == 0 {
                return index;
            }
        }
        index += 1;
    }
    0xffff_ffff
}

// Translated from 00e82ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupObjectEntry` (engine map), `thiscall` (the map is not read)
/// with one stack word, the entry. `LookUpObjectMapping` of the entry's key (+0xC) into the
/// stack word of the entry argument; not found answers 0x40. `LookUpObjectMappingType` of the
/// mapping (with two out words, the first cleared first) gives the index of the byte (table
/// `011f5fb4`) that becomes the low byte of the entry's flags (+0x14); a mapping of 0 then
/// answers 0x40, else the mapping goes to +0x10, the first out word to the register count
/// (+0x20), and the answer is 0.
pub fn ni_d3d_shader_constant_map_setup_object_entry(
    e: &mut Engine,
    _this: Ptr,
    entry: Ptr,
) -> u32 {
    let entry = entry.addr();
    e.with_stack(0xc, |e, locals| {
        let mapping_word = locals.addr();
        let first = mapping_word + 4;
        let second = mapping_word + 8;
        let key = e.mem.u32(entry.wrapping_add(0xc));
        e.mem.set_u32(mapping_word, entry);
        e.mem.set_u32(first, 0);
        if !e
            .call(LOOK_UP_OBJECT_MAPPING, &args![key, mapping_word])
            .bool()
        {
            return 0x40;
        }
        let mapping = e.mem.u32(mapping_word);
        let kind = e
            .call(LOOK_UP_OBJECT_MAPPING_TYPE, &args![mapping, first, second])
            .u32();
        let byte = u32::from(e.mem.u8(PREDEFINED_TYPE_TABLE_011F5FB4.wrapping_add(kind)));
        let flags = e.mem.u32(entry.wrapping_add(0x14));
        e.mem
            .set_u32(entry.wrapping_add(0x14), byte | (flags & 0xffff_ff00));
        let mapping = e.mem.u32(mapping_word);
        if mapping == 0 {
            return 0x40;
        }
        let count = e.mem.u32(first);
        e.mem.set_u32(entry.wrapping_add(0x10), mapping);
        e.mem.set_u32(entry.wrapping_add(0x20), count);
        0
    })
}

// Translated from 00e84b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetConstantConstant` (engine map), `thiscall`, three stack words
/// (the program, the entry, one not read): `MapConstantValue` (`00e83a90`) of the entry; no
/// data answers 1; otherwise the program's `SetShaderConstant` (vtable +0x74: entry, data, 0)
/// answers 0 when it accepts and 0x50 when not.
pub fn ni_d3d_shader_constant_map_set_constant_constant(
    e: &mut Engine,
    _this: Ptr,
    program: u32,
    entry: u32,
    _unused_3: u32,
) -> u32 {
    let data = ni_d3d_shader_constant_map_map_constant_value(e, Ptr::new(entry));
    if data == 0 {
        return 1;
    }
    if e.vcall(program, 0x74, &args![entry, data, 0u32]).bool() {
        0
    } else {
        0x50
    }
}

// Translated from 00e87010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::RemoveEntry` (engine map), `thiscall`, one stack word (the key).
/// `GetEntryIndex` (vtable +0x7C) not finding it sets `m_eLastError` (+0x34) to 0x10. A
/// global entry (kind 0x4) found gives a held copy of its key handle to
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (in the argument's stack word). Then the
/// element is set to null (`SetAt` on `m_aspEntries` +0xC). Answers `m_eLastError`.
pub fn ni_d3d_shader_constant_map_remove_entry(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let t = this.addr();
    let index = e.vcall(t, 0x7c, &args![key]).u32();
    if index == 0xffff_ffff {
        e.mem.set_u32(t.wrapping_add(0x34), 0x10);
        return e.mem.u32(t.wrapping_add(0x34));
    }
    let elements = e.mem.u32(t.wrapping_add(0x10));
    let entry = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        if entry != 0 && e.mem.u32(entry.wrapping_add(0x14)) & 0xf000_0000 == 0x4000_0000 {
            let handle = e.mem.u32(entry.wrapping_add(0xc));
            if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                interlocked_increment(e, handle.wrapping_sub(8));
            }
            let handle = e.mem.u32(entry.wrapping_add(0xc));
            e.mem.set_u32(slot, handle);
            e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![slot]);
            let handle = e.mem.u32(slot);
            if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
                interlocked_decrement(e, handle.wrapping_sub(8));
            }
        }
        e.mem.set_u32(slot, 0);
        ni_t_array_set_at(e, Ptr::new(t.wrapping_add(0xc)), index, Ptr::new(slot));
    });
    e.mem.u32(t.wrapping_add(0x34))
}

/// `MapConstantValue`'s scratch values: the byte widened to a word (`0126f7b0`), four words
/// (`0126f7b4`), four floats (`0126f7cc`) and sixteen floats (`0126f7e0`).
const CONSTANT_BYTE_0126F7B0: u32 = 0x0126_f7b0;
const CONSTANT_WORDS_0126F7B4: u32 = 0x0126_f7b4;
const CONSTANT_FLOATS_0126F7CC: u32 = 0x0126_f7cc;
const CONSTANT_MATRIX_0126F7E0: u32 = 0x0126_f7e0;
/// 0.0 and 1.0 as floats (what `FLDZ`/`FLD1` and a float store leave).
const ZERO: u32 = 0;

// Translated from 00e83a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::MapConstantValue` (engine map), `stdcall`, one word (the entry):
/// the data to give the program for a constant entry. With `count` = size (+0x28) / stride
/// (+0x2C) (an unsigned division) and the type the table at `011f5fc4` gives the flags' low
/// byte (1 a byte, 3 integer words, else floats), from the data (+0x30): 1 value: the byte
/// widened (`0126f7b0`), the word four times (`0126f7b4`) or the float four times
/// (`0126f7cc`); 2: the pair twice (words in the order 0, 1, 0, 1 as the code stores them);
/// 3: the three and a 1 (word 1 or float 1.0); 4 and 16: the data itself; 8: a 4x4 float
/// matrix of the eight floats and zeros (`0126f7e0`); 9: a 3x3 matrix as rows of four
/// (zeros in the fourth column and the last row); 12: the twelve floats and four zeros; any
/// other count: the data itself when the stride is 4 and the count a multiple of 4, else 0.
/// Floats are `FLD`/`FSTP` copies. The compiler's byte map and jump table over the count
/// are the `match`.
pub fn ni_d3d_shader_constant_map_map_constant_value(e: &mut Engine, entry: Ptr) -> u32 {
    let entry = entry.addr();
    let size = e.mem.u32(entry.wrapping_add(0x28));
    let stride = e.mem.u32(entry.wrapping_add(0x2c));
    // DIV: a zero stride faults in the exe; the Rust division panics.
    let count = size / stride;
    let flags = e.mem.u32(entry.wrapping_add(0x14)) & 0xff;
    let kind = e
        .mem
        .u32(CONSTANT_TYPE_TABLE_011F5FC4.wrapping_add(flags * 4));
    let data = e.mem.u32(entry.wrapping_add(0x30));
    let word = |e: &Engine, i: u32| e.mem.u32(data.wrapping_add(4 * i));
    let float = |e: &Engine, i: u32| copy_float(e, data.wrapping_add(4 * i));
    match count {
        1 => {
            if kind == 1 {
                let byte = u32::from(e.mem.u8(data));
                e.mem.set_u32(CONSTANT_BYTE_0126F7B0, byte);
                return CONSTANT_BYTE_0126F7B0;
            }
            if kind == 3 {
                let value = word(e, 0);
                store_vector(e, CONSTANT_WORDS_0126F7B4, [value; 4]);
                return CONSTANT_WORDS_0126F7B4;
            }
            let value = float(e, 0);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC, value);
            let value = copy_float(e, CONSTANT_FLOATS_0126F7CC);
            for i in 1..4u32 {
                e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 4 * i, value);
            }
            CONSTANT_FLOATS_0126F7CC
        }
        2 => {
            if kind == 3 {
                let first = word(e, 0);
                e.mem.set_u32(CONSTANT_WORDS_0126F7B4, first);
                let second = word(e, 1);
                e.mem.set_u32(CONSTANT_WORDS_0126F7B4 + 8, first);
                e.mem.set_u32(CONSTANT_WORDS_0126F7B4 + 4, second);
                e.mem.set_u32(CONSTANT_WORDS_0126F7B4 + 0xc, second);
                return CONSTANT_WORDS_0126F7B4;
            }
            let first = float(e, 0);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC, first);
            let second = float(e, 1);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 4, second);
            let first = copy_float(e, CONSTANT_FLOATS_0126F7CC);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 8, first);
            let second = copy_float(e, CONSTANT_FLOATS_0126F7CC + 4);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 0xc, second);
            CONSTANT_FLOATS_0126F7CC
        }
        3 => {
            if kind == 3 {
                let values = [word(e, 0), word(e, 1), word(e, 2), 1];
                store_vector(e, CONSTANT_WORDS_0126F7B4, values);
                return CONSTANT_WORDS_0126F7B4;
            }
            let values = [float(e, 0), float(e, 1), float(e, 2), ONE];
            store_vector(e, CONSTANT_FLOATS_0126F7CC, values);
            CONSTANT_FLOATS_0126F7CC
        }
        4 | 16 => data,
        8 => {
            for i in 0..8u32 {
                let value = float(e, i);
                e.mem.set_u32(CONSTANT_MATRIX_0126F7E0 + 4 * i, value);
            }
            for i in 8..16u32 {
                e.mem.set_u32(CONSTANT_MATRIX_0126F7E0 + 4 * i, ZERO);
            }
            CONSTANT_MATRIX_0126F7E0
        }
        9 => {
            for row in 0..3u32 {
                for column in 0..3u32 {
                    let value = float(e, row * 3 + column);
                    e.mem
                        .set_u32(CONSTANT_MATRIX_0126F7E0 + 0x10 * row + 4 * column, value);
                }
                if row < 2 {
                    e.mem
                        .set_u32(CONSTANT_MATRIX_0126F7E0 + 0x10 * row + 0xc, ZERO);
                }
            }
            for i in 11..16u32 {
                e.mem.set_u32(CONSTANT_MATRIX_0126F7E0 + 4 * i, ZERO);
            }
            CONSTANT_MATRIX_0126F7E0
        }
        12 => {
            for i in 0..12u32 {
                let value = float(e, i);
                e.mem.set_u32(CONSTANT_MATRIX_0126F7E0 + 4 * i, value);
            }
            for i in 12..16u32 {
                e.mem.set_u32(CONSTANT_MATRIX_0126F7E0 + 4 * i, ZERO);
            }
            CONSTANT_MATRIX_0126F7E0
        }
        _ => {
            if stride == 4 && count & 3 == 0 {
                data
            } else {
                0
            }
        }
    }
}

/// `NiShaderFactory::RegisterGlobalShaderConstant` (`00aa0400`, engine map; `cdecl`: the
/// address of the key handle, the type, the size and the data).
const REGISTER_GLOBAL_SHADER_CONSTANT: u32 = 0x00aa_0400;
/// The 16-bit object flags `AddObjectEntry` takes by object type (`011f5f7c`).
const OBJECT_FLAGS_011F5F7C: u32 = 0x011f_5f7c;

/// The common end of the entry adders: `InsertEntry` (vtable +0x80) of the entry into
/// `m_eLastError` (+0x34); an error deletes the entry (its slot 0 with 1) and is answered
/// (read again), else 0.
fn insert_or_delete(e: &mut Engine, this: u32, entry: u32) -> u32 {
    let error = e.vcall(this, 0x80, &args![entry]).u32();
    e.mem.set_u32(this.wrapping_add(0x34), error);
    if error == 0 {
        return 0;
    }
    e.vcall(entry, 0, &args![1u32]);
    e.mem.u32(this.wrapping_add(0x34))
}

// Translated from 00e82090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddAttributeEntry` (engine map), `thiscall`, ten stack words (the
/// key, the flags, the extra word, the register, the register count, the variable name, the
/// data size, stride and source, and whether to copy). A new entry (`00aa13e0` and the
/// constructor; a failed allocation is used as the null entry, as in the exe) gets the key,
/// the flags' low 28 bits with 0x30000000, the extra word (+0x18), register (+0x1C), count
/// (+0x20) and variable name, and, only with a data source, its data (`00e81ca0`). Then
/// `InsertEntry`: an error deletes the entry and is answered, else 0.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_add_attribute_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: u8,
) -> u32 {
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    e.mem.set_u32(
        entry.wrapping_add(0x14),
        (flags & 0x0fff_ffff) | 0x3000_0000,
    );
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    e.mem.set_u32(entry.wrapping_add(0x20), register_count);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    if data_source != 0 {
        fn_00e81ca0(
            e,
            Ptr::new(entry),
            data_size,
            data_stride,
            data_source,
            copy_data,
        );
    }
    insert_or_delete(e, this.addr(), entry)
}

// Translated from 00e821e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddGlobalEntry` (engine map), `thiscall`, ten stack words (as
/// `AddAttributeEntry`). The entry gets the key, the flags' low 28 bits with 0x40000000, the
/// extra word, register, count, variable name, size, stride and data (copied or pointed at).
/// `InsertEntry`: an error deletes the entry and is answered; otherwise the key (as a fixed
/// string kept in the copy flag's stack word) is registered with the factory
/// (`RegisterGlobalShaderConstant` with the type the table at `011f5fc4` gives the flags'
/// low byte, the size and the data source), the handle released, and 0 answered.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_add_global_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
    data_size: u32,
    data_stride: u32,
    data_source: u32,
    copy_data: u8,
) -> u32 {
    let t = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    let entry_flags = (flags & 0x0fff_ffff) | 0x4000_0000;
    e.mem.set_u32(entry.wrapping_add(0x20), register_count);
    e.mem.set_u32(entry.wrapping_add(0x14), entry_flags);
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x28), data_size);
    e.mem.set_u32(entry.wrapping_add(0x2c), data_stride);
    set_data_source(e, entry, data_size, data_source, copy_data != 0);
    let error = e.vcall(t, 0x80, &args![entry]).u32();
    e.mem.set_u32(t.wrapping_add(0x34), error);
    if error != 0 {
        e.vcall(entry, 0, &args![1u32]);
        return e.mem.u32(t.wrapping_add(0x34));
    }
    let kind = e
        .mem
        .u32(CONSTANT_TYPE_TABLE_011F5FC4.wrapping_add((entry_flags & 0xff) * 4));
    let handle = if key == 0 {
        e.global::<u32>(FIXED_STRING_NULL_0109B220)
    } else {
        e.call(MAKE_FIXED_STRING_00A5B690, &args![key]).u32()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), handle);
        e.call(
            REGISTER_GLOBAL_SHADER_CONSTANT,
            &args![slot, kind, data_size, data_source],
        );
        let handle = e.mem.u32(slot.addr());
        if handle != e.global::<u32>(FIXED_STRING_NULL_0109B220) {
            interlocked_decrement(e, handle.wrapping_sub(8));
        }
    });
    e.mem.u32(t.wrapping_add(0x34))
}

// Translated from 00e823d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddOperatorEntry` (engine map), `thiscall`, six stack words (the
/// key, the flags, the extra word, the register, the register count and the variable name):
/// a new entry with them (flags: the low 28 bits with 0x50000000), then `InsertEntry` (an
/// error deletes the entry and is answered, else 0).
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_add_operator_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    flags: u32,
    extra: u32,
    register: u32,
    register_count: u32,
    variable: u32,
) -> u32 {
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    e.mem.set_u32(
        entry.wrapping_add(0x14),
        (flags & 0x0fff_ffff) | 0x5000_0000,
    );
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    e.mem.set_u32(entry.wrapping_add(0x20), register_count);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    insert_or_delete(e, this.addr(), entry)
}

// Translated from 00e82500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::AddObjectEntry` (engine map), `thiscall`, five stack words (the
/// key, the register, the variable name, the extra word and the object type): a new entry
/// with the key, register, variable name, extra word and the flags 0x60000000 with the
/// 16-bit word the table at `011f5f7c` gives the object type. `SetupObjectEntry` (vtable
/// +0xA0) into `m_eLastError`: 0 inserts the entry (`InsertEntry`, its result stored and
/// answered), else the entry is deleted and the error answered.
pub fn ni_d3d_shader_constant_map_add_object_entry(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    register: u32,
    variable: u32,
    extra: u32,
    object_type: u32,
) -> u32 {
    let t = this.addr();
    let entry = new_entry(e);
    set_fixed_string(e, entry.wrapping_add(0xc), key);
    e.mem.set_u32(entry.wrapping_add(0x1c), register);
    set_fixed_string(e, entry.wrapping_add(0x24), variable);
    e.mem.set_u32(entry.wrapping_add(0x18), extra);
    let object_flags = u32::from(
        e.mem
            .u16(OBJECT_FLAGS_011F5F7C.wrapping_add(object_type.wrapping_mul(2))),
    );
    e.mem
        .set_u32(entry.wrapping_add(0x14), object_flags | 0x6000_0000);
    let error = e.vcall(t, 0xa0, &args![entry]).u32();
    e.mem.set_u32(t.wrapping_add(0x34), error);
    if error == 0 {
        let error = e.vcall(t, 0x80, &args![entry]).u32();
        e.mem.set_u32(t.wrapping_add(0x34), error);
        return error;
    }
    e.vcall(entry, 0, &args![1u32]);
    e.mem.u32(t.wrapping_add(0x34))
}

/// The operator scratch values `SetOperatorConstant` fills and the `PerformOperator*`
/// functions read and write: the first operand as a matrix (`0126f820`, 16 floats), the
/// second as a matrix (`0126f860`), the first as a vector or scalar (`0126f8a0`, 4 floats),
/// the second as one (`0126f8b0`), the matrix result (`0126f8c0`) and the vector result
/// (`0126f900`).
const FIRST_MATRIX_0126F820: u32 = 0x0126_f820;
const SECOND_MATRIX_0126F860: u32 = 0x0126_f860;
const FIRST_VECTOR_0126F8A0: u32 = 0x0126_f8a0;
const SECOND_VECTOR_0126F8B0: u32 = 0x0126_f8b0;
const MATRIX_RESULT_0126F8C0: u32 = 0x0126_f8c0;
const VECTOR_RESULT_0126F900: u32 = 0x0126_f900;
/// The `D3DXVec4Transform` thunk (`00ee6e34`, slot `00fdf3ec`; `stdcall`: out, vector,
/// matrix).
const VEC4_TRANSFORM_00EE6E34: u32 = 0x00ee_6e34;
/// The operand kinds the operators handle (the entry types `SetOperatorConstant` gives):
/// 3 and 4 scalars, 7 and 10 vectors, 9 a matrix.
const OPERAND_MATRIX: u32 = 9;

/// The program's `SetShaderConstant` (vtable +0x74: the entry, the data, 0): 0 when it
/// accepts, 0x50 when not.
fn set_shader_constant(e: &mut Engine, program: u32, entry: u32, data: u32) -> u32 {
    if e.vcall(program, 0x74, &args![entry, data, 0u32]).bool() {
        0
    } else {
        0x50
    }
}

/// The float at `at` as an x87 `FLD dword` leaves it.
fn load_at(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// The float at `at` as an x87 arithmetic memory operand.
fn operand_at(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// The first matrix times the factor `factor` (each entry loaded and `FMUL`ed, the last as
/// the memory operand), into the matrix result.
fn scale_first_matrix(e: &mut Engine, factor: f64) {
    for i in 0..16u32 {
        let entry = if i == 15 {
            operand_at(e, FIRST_MATRIX_0126F820 + 4 * i)
        } else {
            load_at(e, FIRST_MATRIX_0126F820 + 4 * i)
        };
        let product = x87_store32(x87_mul(entry, factor));
        e.mem.set_u32(MATRIX_RESULT_0126F8C0 + 4 * i, product);
    }
}

/// The first vector times the factor `factor` (the last component as the memory operand),
/// into the vector result.
fn scale_first_vector(e: &mut Engine, factor: f64) {
    for i in 0..4u32 {
        let component = if i == 3 {
            operand_at(e, FIRST_VECTOR_0126F8A0 + 4 * i)
        } else {
            load_at(e, FIRST_VECTOR_0126F8A0 + 4 * i)
        };
        let product = x87_store32(x87_mul(component, factor));
        e.mem.set_u32(VECTOR_RESULT_0126F900 + 4 * i, product);
    }
}

/// The matrix result inverted (`D3DXMatrixInverse`) and transposed in place as the flags
/// say, then given to the program.
fn finish_matrix(e: &mut Engine, program: u32, entry: u32, inverse: u8, transpose: u8) -> u32 {
    invert_transpose(e, MATRIX_RESULT_0126F8C0, inverse != 0, transpose != 0);
    set_shader_constant(e, program, entry, MATRIX_RESULT_0126F8C0)
}

// Translated from 00e82b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::PerformOperatorMultiply` (engine map), `stdcall`, six words: the
/// program, the entry, the two operand kinds and the inverse and transpose flags (low
/// bytes). A matrix (9) times a matrix: `D3DXMatrixMultiply` (first, second); times a scalar
/// (3, 4): every entry times the scalar's first float (x87); each then inverted and
/// transposed as asked and given to the program. A matrix times a vector (7, 10), a vector
/// times a matrix: `D3DXVec4Transform` (the vector with the matrix); a vector times a
/// scalar: every component times it; a vector times a vector: component by component. Other
/// kinds answer 1. The program's answer gives 0 or 0x50. The compiler's jump tables over the
/// second kind are the `match` arms.
pub fn ni_d3d_shader_constant_map_perform_operator_multiply(
    e: &mut Engine,
    program: u32,
    entry: u32,
    first: u32,
    second: u32,
    inverse: u8,
    transpose: u8,
) -> u32 {
    match first {
        OPERAND_MATRIX => match second {
            3 | 4 => {
                let scalar = load_at(e, SECOND_VECTOR_0126F8B0);
                scale_first_matrix(e, scalar);
                finish_matrix(e, program, entry, inverse, transpose)
            }
            7 | 10 => {
                e.call(
                    VEC4_TRANSFORM_00EE6E34,
                    &args![
                        VECTOR_RESULT_0126F900,
                        SECOND_VECTOR_0126F8B0,
                        FIRST_MATRIX_0126F820
                    ],
                );
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            OPERAND_MATRIX => {
                e.call(
                    MATRIX_MULTIPLY_00EE6DE0,
                    &args![
                        MATRIX_RESULT_0126F8C0,
                        FIRST_MATRIX_0126F820,
                        SECOND_MATRIX_0126F860
                    ],
                );
                finish_matrix(e, program, entry, inverse, transpose)
            }
            _ => 1,
        },
        7 | 10 => match second {
            3 | 4 => {
                let scalar = load_at(e, SECOND_VECTOR_0126F8B0);
                scale_first_vector(e, scalar);
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            7 | 10 => {
                // The first component is the first operand's times the second's (memory
                // operand), the others the second's times the first's.
                for i in 0..4u32 {
                    let (loaded, operand) = if i == 0 {
                        (FIRST_VECTOR_0126F8A0, SECOND_VECTOR_0126F8B0)
                    } else {
                        (
                            SECOND_VECTOR_0126F8B0 + 4 * i,
                            FIRST_VECTOR_0126F8A0 + 4 * i,
                        )
                    };
                    let product = x87_store32(x87_mul(load_at(e, loaded), operand_at(e, operand)));
                    e.mem.set_u32(VECTOR_RESULT_0126F900 + 4 * i, product);
                }
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            OPERAND_MATRIX => {
                e.call(
                    VEC4_TRANSFORM_00EE6E34,
                    &args![
                        VECTOR_RESULT_0126F900,
                        FIRST_VECTOR_0126F8A0,
                        SECOND_MATRIX_0126F860
                    ],
                );
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            _ => 1,
        },
        _ => 1,
    }
}

/// One over the second operand's first float (x87 `FLD1`, `FDIVRP`), stored as a float.
fn inverse_scalar(e: &Engine) -> f64 {
    let scalar = load_at(e, SECOND_VECTOR_0126F8B0);
    x87_load32(x87_store32(x87_div(1.0, scalar)))
}

// Translated from 00e82e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::PerformOperatorDivide` (engine map), `stdcall`, six words (as
/// `PerformOperatorMultiply`). A matrix divided by a scalar (kind 3 or 4, signed range
/// checks): every entry times one over the scalar (stored as a float first), inverted and
/// transposed as asked; a vector divided by a scalar: every component times it; a vector by a
/// vector: component by component (`FDIV`). Other kinds answer 1. The program's answer
/// gives 0 or 0x50.
pub fn ni_d3d_shader_constant_map_perform_operator_divide(
    e: &mut Engine,
    program: u32,
    entry: u32,
    first: u32,
    second: u32,
    inverse: u8,
    transpose: u8,
) -> u32 {
    match first {
        OPERAND_MATRIX => {
            if (second as i32) < 3 || (second as i32) > 4 {
                return 1;
            }
            let factor = inverse_scalar(e);
            scale_first_matrix(e, factor);
            finish_matrix(e, program, entry, inverse, transpose)
        }
        7 | 10 => match second {
            3 | 4 => {
                let factor = inverse_scalar(e);
                scale_first_vector(e, factor);
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            7 | 10 => {
                for i in 0..4u32 {
                    let a = load_at(e, FIRST_VECTOR_0126F8A0 + 4 * i);
                    let b = operand_at(e, SECOND_VECTOR_0126F8B0 + 4 * i);
                    let quotient = x87_store32(x87_div(a, b));
                    e.mem.set_u32(VECTOR_RESULT_0126F900 + 4 * i, quotient);
                }
                set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
            }
            _ => 1,
        },
        _ => 1,
    }
}

/// Which side of an add or subtract a kind is: a scalar (3, 4), a vector (7, 10), or
/// neither.
#[derive(Clone, Copy, PartialEq)]
enum Side {
    Scalar,
    Vector,
}

fn side(kind: u32) -> Option<Side> {
    match kind {
        3 | 4 => Some(Side::Scalar),
        7 | 10 => Some(Side::Vector),
        _ => None,
    }
}

// Translated from 00e830f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the operator add `SetOperatorConstant` calls, `stdcall`, six
/// words (as `PerformOperatorMultiply`; the two flags are not read). Scalars (3, 4) and
/// vectors (7, 10) only, else 1: two scalars or two vectors add component by component; a
/// scalar plus a vector adds the scalar's first float to each component of the vector, and
/// a vector plus a scalar the scalar's first float to each of the vector's (all x87 `FADD`,
/// stored as floats in the vector result). The program's answer gives 0 or 0x50. The
/// compiler's byte maps and jump tables over the kinds are the `match`.
pub fn fn_00e830f0(
    e: &mut Engine,
    program: u32,
    entry: u32,
    first: u32,
    second: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let (Some(first), Some(second)) = (side(first), side(second)) else {
        return 1;
    };
    let a = |i: u32| FIRST_VECTOR_0126F8A0 + 4 * i;
    let b = |i: u32| SECOND_VECTOR_0126F8B0 + 4 * i;
    for i in 0..4u32 {
        let sum = match (first, second) {
            (Side::Scalar, Side::Vector) => {
                // FLD a0; FLD ST0; FADD b0, then FLD b1..b2; FADD ST0,ST1; the last FADD b3.
                let scalar = load_at(e, a(0));
                let component = if i == 0 || i == 3 {
                    operand_at(e, b(i))
                } else {
                    load_at(e, b(i))
                };
                x87_add(scalar, component)
            }
            (Side::Vector, Side::Scalar) => {
                // FLD a0; FLD b0; FADDP, then FLD a1..a2; FADD ST0,ST1; the last FADD a3.
                let scalar = load_at(e, b(0));
                let component = if i == 3 {
                    operand_at(e, a(i))
                } else {
                    load_at(e, a(i))
                };
                x87_add(component, scalar)
            }
            _ => x87_add(load_at(e, a(i)), operand_at(e, b(i))),
        };
        e.mem
            .set_u32(VECTOR_RESULT_0126F900 + 4 * i, x87_store32(sum));
    }
    set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
}

// Translated from 00e83320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the operator subtract `SetOperatorConstant` calls, as the add
/// `00e830f0` with `FSUB`: two scalars or two vectors component by component; a scalar minus
/// a vector the scalar's first float minus each component; a vector minus a scalar each
/// component minus the scalar's first float.
pub fn fn_00e83320(
    e: &mut Engine,
    program: u32,
    entry: u32,
    first: u32,
    second: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let (Some(first), Some(second)) = (side(first), side(second)) else {
        return 1;
    };
    let a = |i: u32| FIRST_VECTOR_0126F8A0 + 4 * i;
    let b = |i: u32| SECOND_VECTOR_0126F8B0 + 4 * i;
    for i in 0..4u32 {
        let difference = match (first, second) {
            (Side::Scalar, Side::Vector) => {
                // FLD a0, then FLD ST0; FSUB bi for each component.
                let scalar = load_at(e, a(0));
                x87_sub(scalar, operand_at(e, b(i)))
            }
            (Side::Vector, Side::Scalar) => {
                // FLD a0; FLD b0; FSUBP ST2; FLD a1..a2; FSUB ST0,ST1; the last FSUBR a3.
                let scalar = load_at(e, b(0));
                let component = if i == 3 {
                    operand_at(e, a(i))
                } else {
                    load_at(e, a(i))
                };
                x87_sub(component, scalar)
            }
            _ => x87_sub(load_at(e, a(i)), operand_at(e, b(i))),
        };
        e.mem
            .set_u32(VECTOR_RESULT_0126F900 + 4 * i, x87_store32(difference));
    }
    set_shader_constant(e, program, entry, VECTOR_RESULT_0126F900)
}

/// `00045bad0` (`cdecl`: an `NiRTTI` and an object): whether the object (not null) is of
/// that class (`0045baf0`). `MapAttributeValue` asks it about `011f5b60` and `011f5b68`.
const IS_KIND_OF_0045BAD0: u32 = 0x0045_bad0;
const ATTRIBUTE_CLASS_011F5B60: u32 = 0x011f_5b60;
const ATTRIBUTE_CLASS_011F5B68: u32 = 0x011f_5b68;
/// `NiSCMExtraData::AddEntry` (`00e80640`, engine map; `thiscall`, five stack words) and
/// `00a62660` (`thiscall`, no stack words; the engine map's `GetDegree3Basis` is folded
/// code): the extra data's name handle by address.
const SCM_ADD_ENTRY_00E80640: u32 = 0x00e8_0640;
const NAME_OF_00A62660: u32 = 0x00a6_2660;
/// `NiShaderConstantMap::GetDynamicEffectForObject` (`00a9c130`, engine map; `cdecl`: the
/// effect state, the object kind and the index).
const GET_DYNAMIC_EFFECT_FOR_OBJECT: u32 = 0x00a9_c130;
/// `NiShaderConstantMap::LookUpPredefinedMappingType_ov2` (`00a9c0a0`, engine map; `cdecl`,
/// the key) and `NiShaderFactory::ReportError` (`00aa0020`, engine map; `cdecl`: 1, 0 and a
/// message).
const LOOK_UP_PREDEFINED_MAPPING_TYPE_BY_KEY: u32 = 0x00a9_c0a0;
const REPORT_ERROR_00AA0020: u32 = 0x00aa_0020;
/// `SetOperatorConstant`'s messages (`"Invalid entries in OperatorConstant"`, `010ef4e8`;
/// `"Invalid operands in OperatorConstant"`, `010ef510`).
const INVALID_ENTRIES_010EF4E8: u32 = 0x010e_f4e8;
const INVALID_OPERANDS_010EF510: u32 = 0x010e_f510;
/// The dynamic effect data `SetObjectConstant` fills (`0126f770`).
const EFFECT_DATA_0126F770: u32 = 0x0126_f770;

/// The type of an entry's constant: the word the table at `011f5fc4` gives the low byte of
/// its flags (+0x14).
fn entry_type(e: &Engine, entry: u32) -> u32 {
    let flags = e.mem.u32(entry.wrapping_add(0x14)) & 0xff;
    e.mem
        .u32(CONSTANT_TYPE_TABLE_011F5FC4.wrapping_add(flags * 4))
}

/// `count` floats copied from `from` to `to` (`FLD`/`FSTP`).
fn copy_floats_to(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let value = copy_float(e, from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), value);
    }
}

/// A 3x3 float matrix at `from` written at `to` as rows of four with zeros in the fourth
/// column (the third row's too); the last row is left alone.
fn widen_rows(e: &mut Engine, to: u32, from: u32) {
    for row in 0..3u32 {
        copy_floats_to(
            e,
            to.wrapping_add(0x10 * row),
            from.wrapping_add(0xc * row),
            3,
        );
        e.mem.set_u32(to.wrapping_add(0x10 * row + 0xc), ZERO);
    }
}

// Translated from 00e83550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupTextureTransformMatrix` (engine map), `cdecl`: the output
/// matrix, a 3x3 texture transform (or null), the inverse and transpose flags (low bytes)
/// and the dimension. Without a transform the output is the identity. Otherwise its first
/// two rows' first two entries go to the output's first two columns (transposed: entry
/// (row, column) to (column, row)); a dimension of 3 or 4 puts the third column's two
/// entries at +0x30 and +0x34 (0 at +0x20 and +0x24), a dimension of 2 at +0x20 and +0x24
/// (0 at +0x30 and +0x34); every other word, the diagonal's last two included, becomes 0.
/// Then inverted and transposed in place as asked.
pub fn ni_d3d_shader_constant_map_setup_texture_transform_matrix(
    e: &mut Engine,
    out: Ptr,
    matrix: Ptr,
    inverse: u8,
    transpose: u8,
    dimension: u32,
) {
    let o = out.addr();
    let m = matrix.addr();
    if m == 0 {
        for offset in [
            0x38u32, 0x34, 0x30, 0x2c, 0x24, 0x20, 0x1c, 0x18, 0x10, 0xc, 0x8, 0x4,
        ] {
            e.mem.set_u32(o.wrapping_add(offset), ZERO);
        }
        for offset in [0x3cu32, 0x28, 0x14, 0] {
            e.mem.set_u32(o.wrapping_add(offset), ONE);
        }
        return;
    }
    for (to, from) in [(0u32, 0u32), (0x10, 4), (4, 0xc), (0x14, 0x10)] {
        let value = copy_float(e, m.wrapping_add(from));
        e.mem.set_u32(o.wrapping_add(to), value);
    }
    if dimension == 4 || dimension == 3 {
        e.mem.set_u32(o.wrapping_add(0x20), ZERO);
        let value = copy_float(e, m.wrapping_add(8));
        e.mem.set_u32(o.wrapping_add(0x30), value);
        e.mem.set_u32(o.wrapping_add(0x24), ZERO);
        let value = copy_float(e, m.wrapping_add(0x14));
        e.mem.set_u32(o.wrapping_add(0x34), value);
    } else if dimension == 2 {
        let value = copy_float(e, m.wrapping_add(8));
        e.mem.set_u32(o.wrapping_add(0x20), value);
        e.mem.set_u32(o.wrapping_add(0x30), ZERO);
        let value = copy_float(e, m.wrapping_add(0x14));
        e.mem.set_u32(o.wrapping_add(0x24), value);
        e.mem.set_u32(o.wrapping_add(0x34), ZERO);
    }
    for offset in [0x38u32, 0x28, 0x18, 0x8, 0x3c, 0x2c, 0x1c, 0xc] {
        e.mem.set_u32(o.wrapping_add(offset), ZERO);
    }
    invert_transpose(e, o, inverse != 0, transpose != 0);
}

// Translated from 00e83840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::MapAttributeValue` (engine map), `stdcall`, two words (the entry
/// and an extra data object): the data to give the program, by the entry's type: 1 the
/// object's byte at +0xC widened (`0126f7b0`); 2 none (0); 3 its word at +0xC four times
/// (`0126f7b4`); 4 its float at +0xC four times (`0126f7cc`); 5 the two floats its +0x10
/// points at, twice; 6 the three and 1.0; 7 four floats: from its +0x10 when it is of the
/// class `011f5b60`, from its +0xC when of `011f5b68`, else whatever `0126f7cc` holds; 8 the
/// nine floats at its +0x10 as rows of four (`0126f7e0`; the last row not written); 9 its
/// +0x10 itself; 10 the four floats at its +0xC; others none.
pub fn ni_d3d_shader_constant_map_map_attribute_value(
    e: &mut Engine,
    entry: Ptr,
    attribute: Ptr,
) -> u32 {
    let a = attribute.addr();
    match entry_type(e, entry.addr()) {
        1 => {
            let byte = u32::from(e.mem.u8(a.wrapping_add(0xc)));
            e.mem.set_u32(CONSTANT_BYTE_0126F7B0, byte);
            CONSTANT_BYTE_0126F7B0
        }
        2 => 0,
        3 => {
            let value = e.mem.u32(a.wrapping_add(0xc));
            store_vector(e, CONSTANT_WORDS_0126F7B4, [value; 4]);
            CONSTANT_WORDS_0126F7B4
        }
        4 => {
            let value = copy_float(e, a.wrapping_add(0xc));
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC, value);
            let value = copy_float(e, CONSTANT_FLOATS_0126F7CC);
            for i in 1..4u32 {
                e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 4 * i, value);
            }
            CONSTANT_FLOATS_0126F7CC
        }
        5 => {
            let p = e.mem.u32(a.wrapping_add(0x10));
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, p, 2);
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC + 8, p, 2);
            CONSTANT_FLOATS_0126F7CC
        }
        6 => {
            let p = e.mem.u32(a.wrapping_add(0x10));
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, p, 3);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 0xc, ONE);
            CONSTANT_FLOATS_0126F7CC
        }
        7 => {
            if e.call(IS_KIND_OF_0045BAD0, &args![ATTRIBUTE_CLASS_011F5B60, a])
                .bool()
            {
                let p = e.mem.u32(a.wrapping_add(0x10));
                copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, p, 4);
            } else if e
                .call(IS_KIND_OF_0045BAD0, &args![ATTRIBUTE_CLASS_011F5B68, a])
                .bool()
            {
                copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, a.wrapping_add(0xc), 4);
            }
            CONSTANT_FLOATS_0126F7CC
        }
        8 => {
            let p = e.mem.u32(a.wrapping_add(0x10));
            widen_rows(e, CONSTANT_MATRIX_0126F7E0, p);
            CONSTANT_MATRIX_0126F7E0
        }
        9 => e.mem.u32(a.wrapping_add(0x10)),
        10 => {
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, a.wrapping_add(0xc), 4);
            CONSTANT_FLOATS_0126F7CC
        }
        _ => 0,
    }
}

/// The program's `SetShaderConstant` with the entry, the data and a word: 0 when it accepts,
/// 0x50 when not.
fn set_constant_with(e: &mut Engine, program: u32, entry: u32, data: u32, word: u32) -> u32 {
    if e.vcall(program, 0x74, &args![entry, data, word]).bool() {
        0
    } else {
        0x50
    }
}

// Translated from 00e84ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetAttributeConstant` (engine map), `thiscall`, thirteen stack
/// words: the program, the entry, the object, seven words not read, two words (the
/// eleventh and twelfth) and the shader's `NiSCMExtraData` (the thirteenth). No object
/// answers 1. With the
/// extra data its next entry for the entry's register (`GetNextEntry` with the eleventh word,
/// the map's program type +8 and the twelfth) is used when its name (`00a62660`) is the
/// entry's key; otherwise the object's extra data named by the key (`GetExtraData`) is
/// looked up and, when found, recorded (`NiSCMExtraData::AddEntry`). Without extra data the
/// object's extra data is looked up directly. Found: `MapAttributeValue` (none answers
/// 0x40); not found: `MapConstantValue` (none answers 0x10). The program's
/// `SetShaderConstant` takes the data with 4 for a type-9 constant, 3 for type 8, 0 for
/// types 1, 3 to 7 and 10 (any other type sets nothing); 0 or 0x50.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_set_attribute_constant(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    entry: u32,
    object: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
    eleventh: u32,
    twelfth: u32,
    extra: u32,
) -> u32 {
    let t = this.addr();
    if object == 0 {
        return 1;
    }
    let key = entry.wrapping_add(0xc);
    let found = if extra != 0 {
        let program_type = e.mem.u32(t.wrapping_add(8));
        let register = e.mem.u32(entry.wrapping_add(0x1c));
        let next = ni_scm_extra_data_get_next_entry(
            e,
            Ptr::new(extra),
            register,
            eleventh,
            program_type,
            twelfth,
        );
        let matches = next != 0 && {
            let name = e.call(NAME_OF_00A62660, &args![next]).u32();
            e.mem.u32(name) == e.mem.u32(key)
        };
        if matches {
            next
        } else {
            let data = e.call(GET_EXTRA_DATA_00A5BDD0, &args![object, key]).u32();
            if data != 0 {
                let program_type = e.mem.u32(t.wrapping_add(8));
                let register = e.mem.u32(entry.wrapping_add(0x1c));
                e.call(
                    SCM_ADD_ENTRY_00E80640,
                    &args![extra, register, eleventh, program_type, data, twelfth],
                );
            }
            data
        }
    } else {
        e.call(GET_EXTRA_DATA_00A5BDD0, &args![object, key]).u32()
    };
    let data = if found == 0 {
        let data = ni_d3d_shader_constant_map_map_constant_value(e, Ptr::new(entry));
        if data == 0 {
            return 0x10;
        }
        data
    } else {
        let data =
            ni_d3d_shader_constant_map_map_attribute_value(e, Ptr::new(entry), Ptr::new(found));
        if data == 0 {
            return 0x40;
        }
        data
    };
    match entry_type(e, entry) {
        9 => set_constant_with(e, program, entry, data, 4),
        1 | 3 | 4 | 5 | 6 | 7 | 10 => set_constant_with(e, program, entry, data, 0),
        8 => set_constant_with(e, program, entry, data, 3),
        _ => 0,
    }
}

// Translated from 00e84d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetGlobalConstant` (engine map), `thiscall` (the map is not
/// read), three stack words (the program, the entry, one not read): the entry's data
/// (+0x30) as `MapConstantValue` would widen it by its type (1 the byte, 3 the word four
/// times, 4 the float four times, 5 the pair twice, 6 three floats and 1.0, 7 and 10 four
/// floats, 8 the nine floats as rows of four, the last row not written, 9 the data itself)
/// given to the program's `SetShaderConstant` with 0; 0, or 0x50 when it refuses. Other
/// types answer 0.
pub fn ni_d3d_shader_constant_map_set_global_constant(
    e: &mut Engine,
    _this: Ptr,
    program: u32,
    entry: u32,
    _unused_3: u32,
) -> u32 {
    let data = e.mem.u32(entry.wrapping_add(0x30));
    let values = match entry_type(e, entry) {
        1 => {
            let byte = u32::from(e.mem.u8(data));
            e.mem.set_u32(CONSTANT_BYTE_0126F7B0, byte);
            CONSTANT_BYTE_0126F7B0
        }
        3 => {
            let value = e.mem.u32(data);
            store_vector(e, CONSTANT_WORDS_0126F7B4, [value; 4]);
            CONSTANT_WORDS_0126F7B4
        }
        4 => {
            let value = copy_float(e, data);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC, value);
            let value = copy_float(e, CONSTANT_FLOATS_0126F7CC);
            for i in 1..4u32 {
                e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 4 * i, value);
            }
            CONSTANT_FLOATS_0126F7CC
        }
        5 => {
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, data, 2);
            let first = copy_float(e, CONSTANT_FLOATS_0126F7CC);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 8, first);
            let second = copy_float(e, CONSTANT_FLOATS_0126F7CC + 4);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 0xc, second);
            CONSTANT_FLOATS_0126F7CC
        }
        6 => {
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, data, 3);
            e.mem.set_u32(CONSTANT_FLOATS_0126F7CC + 0xc, ONE);
            CONSTANT_FLOATS_0126F7CC
        }
        7 | 10 => {
            copy_floats_to(e, CONSTANT_FLOATS_0126F7CC, data, 4);
            CONSTANT_FLOATS_0126F7CC
        }
        8 => {
            widen_rows(e, CONSTANT_MATRIX_0126F7E0, data);
            CONSTANT_MATRIX_0126F7E0
        }
        9 => data,
        _ => return 0,
    };
    set_constant_with(e, program, entry, values, 0)
}

// Translated from 00e85c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetObjectConstant` (engine map), `thiscall` (the map is not
/// read), eleven stack words: the program, the entry and nine words passed on (the sixth is
/// the dynamic effect state). The effect (`GetDynamicEffectForObject` with the state, the
/// object kind the table at `011f5f90` gives the flags' bits 8 to 15, and the entry's
/// +0x18) and the mapping's type (`LookUpObjectMappingType` of the entry's +0x10, its two
/// out words in the entry's and the effect state's own stack words); none answers 0x10.
/// `GetDynamicEffectData` fills `0126f770` (the first out word times 16 bytes; a false
/// answer makes the result 0x60) and the program's `SetShaderConstant` takes it (a refusal
/// makes it 0x50). Answers the result (0 otherwise).
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_set_object_constant(
    e: &mut Engine,
    _this: Ptr,
    program: u32,
    entry: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    effects: u32,
    word_9: u32,
    word_10: u32,
    word_11: u32,
) -> u32 {
    let kind = u32::from(e.mem.u8(entry.wrapping_add(0x15)));
    let object_kind = e.mem.u32(OBJECT_TABLE_011F5F90.wrapping_add(kind * 4));
    let index = e.mem.u32(entry.wrapping_add(0x18));
    let effect = e
        .call(
            GET_DYNAMIC_EFFECT_FOR_OBJECT,
            &args![effects, object_kind, index],
        )
        .u32();
    let mapping = e.mem.u32(entry.wrapping_add(0x10));
    e.with_stack(8, |e, outs| {
        let first = outs.addr();
        let second = first + 4;
        e.mem.set_u32(first, entry);
        e.mem.set_u32(second, effects);
        let found = e
            .call(LOOK_UP_OBJECT_MAPPING_TYPE, &args![mapping, first, second])
            .u32();
        if found == 0 {
            return 0x10;
        }
        let size = e.mem.u32(first).wrapping_mul(16);
        let mut result = 0u32;
        let filled = ni_d3d_shader_constant_map_get_dynamic_effect_data(
            e,
            Ptr::new(EFFECT_DATA_0126F770),
            size,
            mapping,
            Ptr::new(effect),
            word_3,
            word_4,
            word_5,
            word_6,
            word_7,
            effects,
            Ptr::new(word_9),
            Ptr::new(word_10),
            word_11,
        );
        if !filled {
            result = 0x60;
        }
        if !e
            .vcall(program, 0x74, &args![entry, EFFECT_DATA_0126F770, 0u32])
            .bool()
        {
            result = 0x50;
        }
        result
    })
}

// Translated from 00e86a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupConstantConstantValue` (engine map), `stdcall`, three words
/// (the operand, 0 or 1; the entry; one not read): the entry's data (+0x30) into the
/// operator scratch for that operand, by type: 3 to 7 and 10 the first 1, 1, 2, 3, 4 and 4
/// floats of the operand's vector (`0126f8a0` + 16 bytes an operand); 8 the nine floats as
/// a 4x4 matrix with zeros and a final 1.0 (`0126f820` + 64 bytes an operand); 9 the
/// sixteen floats. Answers the type, or 0 for any other.
pub fn ni_d3d_shader_constant_map_setup_constant_constant_value(
    e: &mut Engine,
    operand: u32,
    entry: Ptr,
    _unused_3: u32,
) -> u32 {
    let entry = entry.addr();
    let data = e.mem.u32(entry.wrapping_add(0x30));
    let kind = entry_type(e, entry);
    fill_operand(e, operand, kind, data)
}

/// The operand's scratch filled from `data` for a constant of type `kind` (as
/// `SetupConstantConstantValue` and `SetupAttributeConstantValue` do it, except type 3 that
/// each handles itself): answers the type, or 0.
fn fill_operand(e: &mut Engine, operand: u32, kind: u32, data: u32) -> u32 {
    let vector = FIRST_VECTOR_0126F8A0.wrapping_add(operand << 4);
    let matrix = FIRST_MATRIX_0126F820.wrapping_add(operand << 6);
    match kind {
        3 | 4 => copy_floats_to(e, vector, data, 1),
        5 => copy_floats_to(e, vector, data, 2),
        6 => copy_floats_to(e, vector, data, 3),
        7 | 10 => copy_floats_to(e, vector, data, 4),
        8 => {
            widen_rows(e, matrix, data);
            for i in 12..15u32 {
                e.mem.set_u32(matrix.wrapping_add(4 * i), ZERO);
            }
            e.mem.set_u32(matrix.wrapping_add(0x3c), ONE);
        }
        9 => copy_floats_to(e, matrix, data, 16),
        _ => return 0,
    }
    kind
}

// Translated from 00e86ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupAttributeConstantValue` (engine map), `thiscall`, eight
/// stack words (the operand, the entry, the object and five not read). No object answers
/// 1; the object's extra data named by the entry's key (`GetExtraData`; none answers 0x10)
/// goes through `MapAttributeValue` into the operand's scratch as
/// `SetupConstantConstantValue` puts it, except that a type-3 value is an unsigned integer
/// turned into a float (`FILD`, plus 2^32 when negative). Answers the type, or 0.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_setup_attribute_constant_value(
    e: &mut Engine,
    _this: Ptr,
    operand: u32,
    entry: Ptr,
    object: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
) -> u32 {
    if object == 0 {
        return 1;
    }
    let entry = entry.addr();
    let extra = e
        .call(
            GET_EXTRA_DATA_00A5BDD0,
            &args![object, entry.wrapping_add(0xc)],
        )
        .u32();
    if extra == 0 {
        return 0x10;
    }
    let data = ni_d3d_shader_constant_map_map_attribute_value(e, Ptr::new(entry), Ptr::new(extra));
    let kind = entry_type(e, entry);
    if kind == 3 {
        let value = e.mem.u32(data);
        let converted = unsigned_to_float(e, value);
        let vector = FIRST_VECTOR_0126F8A0.wrapping_add(operand << 4);
        e.mem.set_u32(vector, converted);
        return 3;
    }
    fill_operand(e, operand, kind, data)
}

/// One operand of `SetOperatorConstant` put into the operator scratch by its kind: answers
/// the type it was set up as (0 when its kind is none of these).
#[allow(clippy::too_many_arguments)]
fn set_up_operand(
    e: &mut Engine,
    this: Ptr,
    operand: u32,
    entry: u32,
    words: [u32; 7],
    second: bool,
) -> Option<u32> {
    let [word_3, word_4, word_5, word_6, word_7, word_8, word_9] = words;
    let kind = e.mem.u32(entry.wrapping_add(0x14)) & 0xf000_0000;
    let attribute = |e: &mut Engine| {
        ni_d3d_shader_constant_map_setup_attribute_constant_value(
            e,
            this,
            operand,
            Ptr::new(entry),
            word_3,
            word_4,
            word_5,
            word_7,
            word_8,
            word_9,
        );
    };
    match kind {
        0x2000_0000 => {
            let key = e.mem.u32(entry.wrapping_add(0xc));
            let mapping = e
                .call(LOOK_UP_PREDEFINED_MAPPING_TYPE_BY_KEY, &args![key])
                .u32();
            ni_d3d_shader_constant_map_setup_defined_constant_value(
                e, this, operand, entry, word_3, word_4, word_5, word_6, word_7, word_8,
            );
            return Some(mapping);
        }
        0x4000_0000 => {
            ni_d3d_shader_constant_map_setup_constant_constant_value(
                e,
                operand,
                Ptr::new(entry),
                word_9,
            );
        }
        0x3000_0000 => attribute(e),
        0x1000_0000 => {
            if second {
                ni_d3d_shader_constant_map_setup_constant_constant_value(
                    e,
                    operand,
                    Ptr::new(entry),
                    word_9,
                );
            } else {
                attribute(e);
            }
        }
        _ => {}
    }
    None
}

// Translated from 00e87220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetOperatorConstant` (engine map), `thiscall`, nine stack words
/// (the program, the entry and seven words passed on). The entry's +0x18 holds the operator
/// (top four bits: 1 multiply, 2 divide, 3 add, 4 subtract), the inverse (bit 24) and
/// transpose (bit 25) flags and the two operand entries' indices (bits 0 to 11 and 12 to
/// 23, through `GetEntryAtIndex`, vtable +0x74). Missing operands report
/// `"Invalid entries in OperatorConstant"` (`NiShaderFactory::ReportError`) and answer 1.
/// Each operand's type is its constant type, and it is put into the operator scratch: a
/// predefined one through `SetupDefinedConstantValue` (`00e85d10`, its type then
/// `LookUpPredefinedMappingType` of its key), a global one through
/// `SetupConstantConstantValue`, an attribute one through `SetupAttributeConstantValue`,
/// a constant one through `SetupAttributeConstantValue` as the first operand and
/// `SetupConstantConstantValue` as the second. A zero type reports
/// `"Invalid operands in OperatorConstant"` and answers 1. Then the operator runs with the
/// program, the entry, both types and the flags (the code pushes each flag in a stack word
/// whose upper bytes it never set; the operators read only the low byte); another operator
/// answers 1.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_set_operator_constant(
    e: &mut Engine,
    this: Ptr,
    program: u32,
    entry: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
    word_8: u32,
    word_9: u32,
) -> u32 {
    let t = this.addr();
    let descriptor = e.mem.u32(entry.wrapping_add(0x18));
    let transpose = ((descriptor >> 25) & 1) as u8;
    let operator = descriptor & 0xf000_0000;
    let inverse = ((descriptor >> 24) & 1) as u8;
    let first = e.vcall(t, 0x74, &args![descriptor & 0xfff]).u32();
    let second = e.vcall(t, 0x74, &args![(descriptor >> 12) & 0xfff]).u32();
    if first == 0 || second == 0 {
        e.call(
            REPORT_ERROR_00AA0020,
            &args![1u32, 0u32, INVALID_ENTRIES_010EF4E8],
        );
        return 1;
    }
    let mut first_type = entry_type(e, first);
    let mut second_type = entry_type(e, second);
    let words = [word_3, word_4, word_5, word_6, word_7, word_8, word_9];
    if let Some(mapping) = set_up_operand(e, this, 0, first, words, false) {
        first_type = mapping;
    }
    if let Some(mapping) = set_up_operand(e, this, 1, second, words, true) {
        second_type = mapping;
    }
    if first_type == 0 || second_type == 0 {
        e.call(
            REPORT_ERROR_00AA0020,
            &args![1u32, 0u32, INVALID_OPERANDS_010EF510],
        );
        return 1;
    }
    let operator_fn: fn(&mut Engine, u32, u32, u32, u32, u8, u8) -> u32 = match operator {
        0x1000_0000 => ni_d3d_shader_constant_map_perform_operator_multiply,
        0x2000_0000 => ni_d3d_shader_constant_map_perform_operator_divide,
        0x3000_0000 => {
            |e, p, n, a, b, inv, tr| fn_00e830f0(e, p, n, a, b, u32::from(inv), u32::from(tr))
        }
        0x4000_0000 => {
            |e, p, n, a, b, inv, tr| fn_00e83320(e, p, n, a, b, u32::from(inv), u32::from(tr))
        }
        _ => return 1,
    };
    operator_fn(
        e,
        program,
        entry,
        first_type,
        second_type,
        inverse,
        transpose,
    )
}

/// CRT `memset` (`00ec61c0`, `cdecl`: destination, value, size).
const MEMSET_00EC61C0: u32 = 0x00ec_61c0;

/// Writes the identity matrix at `at` (as the code does it: zeros first, then the ones).
fn store_identity(e: &mut Engine, at: u32) {
    for offset in [
        0x38u32, 0x34, 0x30, 0x2c, 0x24, 0x20, 0x1c, 0x18, 0x10, 0xc, 0x8, 0x4,
    ] {
        e.mem.set_u32(at.wrapping_add(offset), ZERO);
    }
    for offset in [0x3cu32, 0x28, 0x14, 0] {
        e.mem.set_u32(at.wrapping_add(offset), ONE);
    }
}

// Translated from 00e85d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::SetupDefinedConstantValue` (engine map), `thiscall`, eight stack
/// words: the operand (0 or 1), the entry, the object, a word that must be set for some
/// constants, the property state, the dynamic effect state and two not read. Puts the
/// entry's predefined constant (+0x10) into the operator scratch for the operand (a matrix
/// at `0126f820` + 64 bytes an operand, a vector at `0126f8a0` + 16 bytes) and answers its
/// type (9 a matrix, 10 or 7 a vector, 0 none). As `SetDefinedConstant`, with these
/// differences (the renderer is the map's +0x3C; "inverted and transposed" as there):
/// - 1/2/13/14, 3/4/15/16, 5/6/17/18: the renderer's +0x9C0, +0x980, +0x940 matrix;
///   7/8/19/20 the product of +0x940 and +0x980; 9/10/21/22 of +0x980 and +0x9C0; 11/12/23/24
///   the product of +0x940 and +0x980 times +0x9C0.
/// - 25 to 28 and 29 to 32 (needing the fourth word): the identity times +0x980, or times
///   +0x980 and then +0x9C0.
/// - 37 to 60: the texture transforms of maps 0, 5, 2, 3, 4 and 5 (ids 41 to 44 take map 5,
///   as the code reads +0x14 there); 61 to 64 the map at the entry's index; 65 to 68 the one
///   `00a463e0` answers (`SetupTextureTransformMatrix` with 4).
/// - 73 to 77: the property state's +8 floats (answering 10); 78 and 79: the last and third
///   row of the renderer's +0xA40 matrix (7).
/// - 82 to 86: the extra data's time (none answers 0x10) four times, through the CRT helpers,
///   or all four (7).
/// - 87: (0, 0, 0, 1) plus `00633b40`'s three floats for each light of the effect state's list
///   that `00653290` answers an object for whose byte at +0x9C is set (7).
/// - 96 to 99 (needing the fourth word): the identity, inverted and transposed.
/// - 33 to 36, 69 to 72, 80, 81, 88 to 95 and any other: nothing, answering 0.
///
/// The compiler's byte map and jump table over the constant are the `match`.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_setup_defined_constant_value(
    e: &mut Engine,
    this: Ptr,
    operand: u32,
    entry: u32,
    object: u32,
    fourth: u32,
    properties: u32,
    effects: u32,
    _unused_7: u32,
    _unused_8: u32,
) -> u32 {
    let base = this.addr();
    let id = e.mem.u32(entry.wrapping_add(0x10));
    let matrix = FIRST_MATRIX_0126F820.wrapping_add(operand << 6);
    let vector = FIRST_VECTOR_0126F8A0.wrapping_add(operand << 4);
    // The inverted and transposed members of the group of four from `first`.
    let flags = |first: u32| {
        (
            id == first + 1 || id == first + 3,
            id == first + 2 || id == first + 3,
        )
    };
    let renderer = |e: &Engine| e.mem.u32(base.wrapping_add(0x3c));
    e.with_stack(0x1c0, |e, frame| {
        let frame = frame.addr();
        let name = frame;
        let time = frame + 4;
        let helper = frame + 8;
        let offset_out = frame + 0xc;
        let scratch_30 = frame + 0x20;
        let scratch_70 = frame + 0x60;
        let scratch_b0 = frame + 0xa0;
        let scratch_f0 = frame + 0xe0;
        let scratch_130 = frame + 0x120;
        let scratch_170 = frame + 0x160;
        // The finished matrix: inverted and transposed in place as asked; a matrix (9).
        let finish = |e: &mut Engine, inverse: bool, transpose: bool| {
            invert_transpose(e, matrix, inverse, transpose);
            9u32
        };
        let texture = |e: &mut Engine, map: u32, first: u32| {
            let mut source = 0u32;
            let (mut inverse, mut transpose) = (false, false);
            if map != 0 {
                let transform = e.mem.u32(map.wrapping_add(0xc));
                if transform != 0 {
                    source = e.call(UPDATE_MATRIX_00BC5C20, &args![transform]).u32();
                    (inverse, transpose) = flags(first);
                }
            }
            // The flags go as whole stack words whose upper three bytes the code never
            // wrote; the callee reads only their low bytes.
            ni_d3d_shader_constant_map_setup_texture_transform_matrix(
                e,
                Ptr::new(matrix),
                Ptr::new(source),
                u8::from(inverse),
                u8::from(transpose),
                4,
            );
            9u32
        };
        let texturing = |e: &Engine| e.mem.u32(properties.wrapping_add(0x14));
        let slot_map = |e: &Engine, offset: u32| {
            let texturing = texturing(e);
            if texturing == 0 {
                return 0;
            }
            let maps = e.mem.u32(texturing.wrapping_add(0x20));
            e.mem.u32(maps.wrapping_add(offset))
        };
        match id {
            1 | 2 | 13 | 14 => {
                let from = renderer(e).wrapping_add(0x9c0);
                copy_matrix(e, from, scratch_30);
                copy_matrix(e, scratch_30, matrix);
                finish(e, id == 2 || id == 14, id == 13 || id == 14)
            }
            3 | 4 | 15 | 16 => {
                let from = renderer(e).wrapping_add(0x980);
                copy_matrix(e, from, scratch_30);
                copy_matrix(e, scratch_30, matrix);
                finish(e, id == 4 || id == 16, id == 15 || id == 16)
            }
            5 | 6 | 17 | 18 => {
                let from = renderer(e).wrapping_add(0x940);
                copy_matrix(e, from, matrix);
                finish(e, id == 6 || id == 18, id == 17 || id == 18)
            }
            7 | 8 | 19 | 20 => {
                let r = renderer(e);
                copy_matrix(e, r.wrapping_add(0x940), scratch_70);
                copy_matrix(e, r.wrapping_add(0x980), scratch_30);
                e.call(
                    MATRIX_MULTIPLY_00EE6DE0,
                    &args![scratch_170, scratch_70, scratch_30],
                );
                copy_matrix(e, scratch_170, matrix);
                finish(e, id == 8 || id == 20, id == 19 || id == 20)
            }
            9 | 10 | 21 | 22 => {
                let r = renderer(e);
                copy_matrix(e, r.wrapping_add(0x980), scratch_30);
                copy_matrix(e, r.wrapping_add(0x9c0), scratch_70);
                e.call(
                    MATRIX_MULTIPLY_00EE6DE0,
                    &args![scratch_130, scratch_30, scratch_70],
                );
                copy_matrix(e, scratch_130, matrix);
                finish(e, id == 10 || id == 22, id == 21 || id == 22)
            }
            11 | 12 | 23 | 24 => {
                let r = renderer(e);
                copy_matrix(e, r.wrapping_add(0x940), scratch_30);
                copy_matrix(e, r.wrapping_add(0x980), scratch_70);
                copy_matrix(e, r.wrapping_add(0x9c0), scratch_f0);
                e.call(
                    MATRIX_MULTIPLY_00EE6DE0,
                    &args![scratch_b0, scratch_30, scratch_70],
                );
                let scratch_1b0 = scratch_170 + 0x40;
                e.call(
                    MATRIX_MULTIPLY_00EE6DE0,
                    &args![scratch_1b0, scratch_b0, scratch_f0],
                );
                copy_matrix(e, scratch_1b0, matrix);
                finish(e, id == 12 || id == 24, id == 23 || id == 24)
            }
            25..=28 => {
                if fourth == 0 {
                    return 0;
                }
                store_identity(e, matrix);
                let (inverse, transpose) = flags(0x19);
                let from = renderer(e).wrapping_add(0x980);
                copy_matrix(e, from, scratch_b0);
                e.call(MATRIX_MULTIPLY_00EE6DE0, &args![matrix, matrix, scratch_b0]);
                finish(e, inverse, transpose)
            }
            29..=32 => {
                if fourth == 0 {
                    return 0;
                }
                store_identity(e, matrix);
                let (inverse, transpose) = flags(0x1d);
                let r = renderer(e);
                copy_matrix(e, r.wrapping_add(0x980), scratch_b0);
                copy_matrix(e, r.wrapping_add(0x9c0), scratch_f0);
                e.call(MATRIX_MULTIPLY_00EE6DE0, &args![matrix, matrix, scratch_b0]);
                e.call(MATRIX_MULTIPLY_00EE6DE0, &args![matrix, matrix, scratch_f0]);
                finish(e, inverse, transpose)
            }
            37..=40 => {
                let map = slot_map(e, 0);
                texture(e, map, 0x25)
            }
            41..=44 => {
                let map = slot_map(e, 0x14);
                texture(e, map, 0x29)
            }
            45..=48 => {
                let map = slot_map(e, 8);
                texture(e, map, 0x2d)
            }
            49..=52 => {
                let map = slot_map(e, 0xc);
                texture(e, map, 0x31)
            }
            53..=56 => {
                let map = slot_map(e, 0x10);
                texture(e, map, 0x35)
            }
            57..=60 => {
                let map = slot_map(e, 0x14);
                texture(e, map, 0x39)
            }
            61..=64 => {
                let index = e.mem.u32(entry.wrapping_add(0x18));
                let map = slot_map(e, index.wrapping_mul(4).wrapping_add(0x20));
                texture(e, map, 0x3d)
            }
            65..=68 => {
                let texturing = texturing(e);
                let map = if texturing != 0 {
                    let index = e.mem.u32(entry.wrapping_add(0x18));
                    e.call(MAP_AT_00A463E0, &args![texturing, index]).u32()
                } else {
                    0
                };
                texture(e, map, 0x41)
            }
            73..=77 => {
                let source = e.mem.u32(properties.wrapping_add(8));
                if source == 0 {
                    return 0;
                }
                let at = |offset: u32| source.wrapping_add(offset);
                match id {
                    73 | 74 => {
                        copy_floats_to(e, vector, VECTOR_011F4998, 3);
                        let w = copy_float(e, at(0x3c));
                        e.mem.set_u32(vector + 0xc, w);
                    }
                    75 => {
                        copy_floats_to(e, vector, at(0x1c), 3);
                        let w = copy_float(e, at(0x3c));
                        e.mem.set_u32(vector + 0xc, w);
                    }
                    76 => {
                        for i in 0..3u32 {
                            let held = e.mem.u32(at(0x34));
                            let from = if held != 0 { held } else { at(0x28) };
                            let value = copy_float(e, from.wrapping_add(4 * i));
                            e.mem.set_u32(vector + 4 * i, value);
                        }
                        let w = copy_float(e, at(0x3c));
                        e.mem.set_u32(vector + 0xc, w);
                    }
                    _ => {
                        for i in 0..4u32 {
                            let value = copy_float(e, at(0x38));
                            e.mem.set_u32(vector + 4 * i, value);
                        }
                    }
                }
                0xa
            }
            78 | 79 => {
                let from = renderer(e).wrapping_add(0xa40);
                copy_matrix(e, from, scratch_30);
                let row = scratch_30 + if id == 78 { 0x30 } else { 0x20 };
                copy_floats_to(e, vector, row, 4);
                7
            }
            82..=86 => {
                let handle = e.mem.u32(EXTRA_DATA_NAME_011BF65C);
                let held = e
                    .call(FIXED_STRING_CONSTRUCT_00438170, &args![name, handle])
                    .u32();
                let extra = e.call(GET_EXTRA_DATA_00A5BDD0, &args![object, held]).u32();
                e.call(FIXED_STRING_DESTRUCT_004381B0, &args![name]);
                if extra == 0 {
                    return 0x10;
                }
                let value = copy_float(e, extra.wrapping_add(0xc));
                e.mem.set_u32(time, value);
                let values = match id - 0x52 {
                    0 => [value; 4],
                    1 => [crt_on(e, CRT_HELPER_00ECA0A0, value); 4],
                    2 => [crt_on(e, CRT_HELPER_00EC9F70, value); 4],
                    3 => [crt_on(e, CRT_HELPER_00ECA5D0, value); 4],
                    // 86 (the inner jump table's last arm).
                    _ => {
                        e.mem.set_u32(vector, value);
                        let a = crt_on(e, CRT_HELPER_00ECA0A0, value);
                        e.mem.set_u32(helper, a);
                        e.mem.set_u32(vector + 4, a);
                        let b = crt_on(e, CRT_HELPER_00EC9F70, value);
                        e.mem.set_u32(helper, b);
                        e.mem.set_u32(vector + 8, b);
                        let c = crt_on(e, CRT_HELPER_00ECA5D0, value);
                        e.mem.set_u32(helper, c);
                        [value, a, b, c]
                    }
                };
                store_vector(e, vector, values);
                7
            }
            87 => {
                store_vector(e, vector, [ZERO, ZERO, ZERO, ONE]);
                let mut node = if effects != 0 {
                    e.mem.u32(effects.wrapping_add(0xc))
                } else {
                    0
                };
                while node != 0 {
                    let light = e.mem.u32(node.wrapping_add(4));
                    node = e.mem.u32(node);
                    if light == 0 {
                        continue;
                    }
                    let found = e
                        .call(LIGHT_CAST_00653290, &args![light, LIGHT_RECORD_011F5F6C])
                        .u32();
                    if found == 0 || e.mem.u8(found.wrapping_add(0x9c)) == 0 {
                        continue;
                    }
                    let strength = copy_float(e, found.wrapping_add(0xc4));
                    e.call(
                        LIGHT_OFFSET_00633B40,
                        &args![found.wrapping_add(0xc8), offset_out, strength],
                    );
                    let first = x87_add(
                        x87_load32(e.mem.u32(offset_out)),
                        x87_m32(e.mem.u32(vector)),
                    );
                    e.mem.set_u32(vector, x87_store32(first));
                    let second = x87_add(
                        x87_load32(e.mem.u32(vector + 4)),
                        x87_m32(e.mem.u32(offset_out + 4)),
                    );
                    e.mem.set_u32(vector + 4, x87_store32(second));
                    let third = x87_add(
                        x87_load32(e.mem.u32(offset_out + 8)),
                        x87_m32(e.mem.u32(vector + 8)),
                    );
                    e.mem.set_u32(vector + 8, x87_store32(third));
                }
                7
            }
            96..=99 => {
                if fourth == 0 {
                    return 0;
                }
                store_identity(e, matrix);
                let (inverse, transpose) = flags(0x60);
                finish(e, inverse, transpose)
            }
            _ => 0,
        }
    })
}

/// What `GetDynamicEffectData` calls (unnamed unless the engine map names them; `thiscall`
/// unless noted): `00439180` (a point, a float: scales it), `NiTransform::InvertNonUniform`
/// (`004b4880`: the transform, the result), `00524c40` (a transform, the result, a point:
/// the point transformed), `00439ef0` (an object, the result, a point), `004a0c10` (a
/// point: normalizes it), `004b4500` (a matrix, the result, a point: the point rotated),
/// `NiMatrix3::Transpose` (`004768c0`: the matrix, the result), `NiMatrix3::operator*`
/// (`0043f8d0`: the matrix, the result, a point), `0062c250` (a transform, the result, a
/// transform: their product), `NiPoint3::Dot` (`004b6190`: a point, another; the result in
/// `ST0`); `00b71a40` (`cdecl`: the result, a point or transform, a point, a float) and
/// `NiD3DUtility::GetD3DFromNi` (`00b799a0`, `cdecl`: the result, a transform), and
/// `0043b300` (`cdecl`: an `NiRTTI`, an object; whether it is of that class) with the
/// record `011f4a98`.
const POINT_SCALE_00439180: u32 = 0x0043_9180;
const INVERT_NON_UNIFORM_004B4880: u32 = 0x004b_4880;
const TRANSFORM_POINT_00524C40: u32 = 0x0052_4c40;
const POINT_FROM_00439EF0: u32 = 0x0043_9ef0;
const NORMALIZE_004A0C10: u32 = 0x004a_0c10;
const ROTATE_POINT_004B4500: u32 = 0x004b_4500;
const MATRIX_TRANSPOSE_004768C0: u32 = 0x0047_68c0;
const MATRIX_TIMES_POINT_0043F8D0: u32 = 0x0043_f8d0;
const TRANSFORM_PRODUCT_0062C250: u32 = 0x0062_c250;
const POINT_DOT_004B6190: u32 = 0x004b_6190;
const LIGHT_MATRIX_00B71A40: u32 = 0x00b7_1a40;
const D3D_FROM_NI_00B799A0: u32 = 0x00b7_99a0;
const IS_KIND_OF_0043B300: u32 = 0x0043_b300;
const SPOT_CLASS_011F4A98: u32 = 0x011f_4a98;
/// The constants it reads: `-1048576.0` (`double`, `010ef4e0`), the point `011f426c`, pi as
/// a `double` (`0101ff40`), `180.0` (`double`, `0104ed58`), and the floats `-1.0`
/// (`01012054`), `2.0` (`010162c0`) and `3.0` (`01017718`).
const FAR_SCALE_010EF4E0: u32 = 0x010e_f4e0;
const ORIGIN_011F426C: u32 = 0x011f_426c;
const PI_0101FF40: u32 = 0x0101_ff40;
const HALF_TURN_DEGREES_0104ED58: u32 = 0x0104_ed58;
const MINUS_ONE_01012054: u32 = 0x0101_2054;
const TWO_010162C0: u32 = 0x0101_62c0;
const THREE_01017718: u32 = 0x0101_7718;

/// Three words copied from `from` to `to` (`MOV`s).
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The light's far point the code makes for a light of kind 3: its three floats from +0xF0
/// times `-1048576.0` (the first two through `FMUL ST(1)` forms, the third with the float as
/// the memory operand), stored at `to`.
fn far_point(e: &mut Engine, light: u32, to: u32) {
    let scale = x87_load64(e.mem.u64(FAR_SCALE_010EF4E0));
    let x = x87_mul(load_at(e, light.wrapping_add(0xf0)), scale);
    e.mem.set_u32(to, x87_store32(x));
    let y = x87_mul(load_at(e, light.wrapping_add(0xf4)), scale);
    e.mem.set_u32(to + 4, x87_store32(y));
    let z = x87_mul(scale, operand_at(e, light.wrapping_add(0xf8)));
    e.mem.set_u32(to + 8, x87_store32(z));
}

/// Writes four float bit patterns at `out`.
fn put_four(e: &mut Engine, out: u32, values: [u32; 4]) {
    store_vector(e, out, values);
}

// Translated from 00e84ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantMap::GetDynamicEffectData` (engine map), `cdecl`, thirteen words: the
/// output, its size, the mapping (1 to 24), the dynamic effect (a light; may be null), six
/// words not read, two transforms (the eleventh, inverted for the eye-space values, and the
/// twelfth, given to `00439ef0`) and one not read. A local 4x4 identity (from `memset` and
/// four 1.0 stores) is the result for matrices without a light. By mapping, with `L` the
/// light (each answering true, or false when there is no light):
/// - 1: (L+0xC4, 0, 0, 0), else zeros; 2, 3, 4: the light's +0xC8, +0xD4 or +0xE0 three
///   floats and 1.0, else (0, 0, 0, 1); 5, 6, 7: the same points scaled by L+0xC4
///   (`00439180`) and 1.0; 18: the floats at +0x118 and 1.0.
/// - 8: the far point of a kind-3 light (+0x9D; `+0xF0` times -1048576) or +0x8C, and 1.0;
///   9: that point (or +0x8C) through the inverted eleventh transform; 19: +0x118 through it.
/// - 10, 11: the light's direction: for kind 2 `00439ef0` (the twelfth, +0x8C) normalized
///   (`004a0c10`), for kind 3 +0xF0, else +0xFC; for 11 rotated by the eleventh transform's
///   transposed rotation (`004768c0`, `004b4500`); with 1.0; no light gives (1, 0, 0, 1).
/// - 12: the light's world transform (+0x68) as a Direct3D matrix (`GetD3DFromNi`), for a
///   kind-3 light through `00b71a40` with the far point and L+0x98; 13: the same with the
///   inverted eleventh transform first (`0062c250`; a kind-3 light's transform copy getting
///   the far point as its translation), through a second local matrix; 16, 17, 23, 24:
///   `00b71a40` of the second local matrix with L+0xF4 (17, 24: rotated by the transposed
///   eleventh transform's rotation, `0043f8d0`) and `011f426c` or the point +0x118 (24:
///   through the inverted eleventh transform) and 1.0; each copied out by `00e81b00`.
/// - 14: for a kind-4 light the cosines (CRT `00ec9f70`) of L+0x10C and +0x108 in radians
///   (`* pi / 180`), L+0x110 and 0; else (-1, -1, 0, 0).
/// - 15: L+0xF0 three floats and 0 for a light of the class `011f4a98`, else (1, 0, 0, 0);
///   no light (1, 0, 0, 0) answering false.
/// - 20: with L+0x138 set its four floats at +0x14C, else zeros; 21: that plane's normal
///   rotated by the inverted eleventh transform (`004b4500`) and, as the fourth float, its
///   dot with the plane's point (normal times +0x158) transformed (`00524c40`, `Dot`).
/// - 22: by L+0x134: 0 and 1 give 2.0, 2 and 3 give 3.0, 4 gives 1.0, others 0; then three
///   zeros.
/// - Any other mapping: `memset` of the output with its size, answering false.
///
/// The compiler's jump tables over the mapping and over L+0x134 are the `match` arms; the
/// 1.0 the code keeps on the x87 stack across the switch is the literal 1.0 below.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_shader_constant_map_get_dynamic_effect_data(
    e: &mut Engine,
    out: Ptr,
    size: u32,
    mapping: u32,
    light: Ptr,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
    eye: Ptr,
    other: Ptr,
    _unused_13: u32,
) -> bool {
    let o = out.addr();
    let l = light.addr();
    let eye = eye.addr();
    let other = other.addr();
    e.with_stack(0x260, |e, frame| {
        let f = frame.addr();
        let point = f + 8;
        let l14 = f + 0x14;
        let l18 = f + 0x18;
        let second = f + 0x28;
        let l68 = f + 0x68;
        let identity = f + 0x74;
        let inverse = f + 0xb4;
        let copy = f + 0xe8;
        e.call(MEMSET_00EC61C0, &args![identity, 0u32, 0x40u32]);
        for offset in [0x3cu32, 0x28, 0x14, 0] {
            e.mem.set_u32(identity + offset, ONE);
        }
        let kind = |e: &Engine| e.mem.u8(l.wrapping_add(0x9d));
        // The common ends: the point with 1.0; the identity copied out (false).
        let point_and_one = |e: &mut Engine| {
            let values = [
                copy_float(e, point),
                copy_float(e, point + 4),
                copy_float(e, point + 8),
                ONE,
            ];
            put_four(e, o, values);
            true
        };
        let identity_out = |e: &mut Engine| {
            fn_00e81b00(e, out, Ptr::new(identity));
            false
        };
        let zero_one = |e: &mut Engine| {
            put_four(e, o, [ZERO, ZERO, ZERO, ONE]);
            false
        };
        // The second local matrix: its translation column cleared and its last word 1.0
        // (the rest is what the callee writes).
        let second_matrix = |e: &mut Engine| {
            for offset in [0x2cu32, 0x1c, 0xc] {
                e.mem.set_u32(second + offset, ZERO);
            }
            e.mem.set_u32(second + 0x3c, ONE);
        };
        let light_matrix = |e: &mut Engine, from: u32, with: u32| {
            e.call(LIGHT_MATRIX_00B71A40, &args![second, from, with, ONE]);
            fn_00e81b00(e, out, Ptr::new(second));
            true
        };
        let inverted_eye = |e: &mut Engine| {
            e.call(INVERT_NON_UNIFORM_004B4880, &args![eye, inverse]);
        };
        let transformed_point = |e: &mut Engine, result: u32, from: u32| {
            let at = e
                .call(TRANSFORM_POINT_00524C40, &args![inverse, result, from])
                .u32();
            copy_words(e, point, at, 3);
        };
        let direction = |e: &mut Engine, result: u32| match kind(e) {
            2 => {
                let at = e
                    .call(
                        POINT_FROM_00439EF0,
                        &args![other, result, l.wrapping_add(0x8c)],
                    )
                    .u32();
                copy_words(e, point, at, 3);
                e.call(NORMALIZE_004A0C10, &args![point]);
            }
            3 => copy_words(e, point, l.wrapping_add(0xf0), 3),
            _ => copy_words(e, point, l.wrapping_add(0xfc), 3),
        };
        match mapping {
            1 => {
                if l == 0 {
                    put_four(e, o, [ZERO; 4]);
                    return false;
                }
                let value = copy_float(e, l.wrapping_add(0xc4));
                put_four(e, o, [value, ZERO, ZERO, ZERO]);
                true
            }
            2 | 3 | 4 | 18 => {
                if l == 0 {
                    return zero_one(e);
                }
                let at = l.wrapping_add(match mapping {
                    2 => 0xc8,
                    3 => 0xd4,
                    4 => 0xe0,
                    _ => 0x118,
                });
                let values = [
                    copy_float(e, at),
                    copy_float(e, at + 4),
                    copy_float(e, at + 8),
                    ONE,
                ];
                put_four(e, o, values);
                true
            }
            5..=7 => {
                if l == 0 {
                    return zero_one(e);
                }
                let from = l.wrapping_add(match mapping {
                    5 => 0xc8,
                    6 => 0xd4,
                    _ => 0xe0,
                });
                copy_words(e, point, from, 3);
                let scale = copy_float(e, l.wrapping_add(0xc4));
                e.call(POINT_SCALE_00439180, &args![point, scale]);
                point_and_one(e)
            }
            8 => {
                if l == 0 {
                    return zero_one(e);
                }
                if kind(e) == 3 {
                    far_point(e, l, point);
                    return point_and_one(e);
                }
                let at = l.wrapping_add(0x8c);
                let values = [
                    copy_float(e, at),
                    copy_float(e, at + 4),
                    copy_float(e, at + 8),
                    ONE,
                ];
                put_four(e, o, values);
                true
            }
            9 => {
                if l == 0 {
                    return zero_one(e);
                }
                inverted_eye(e);
                if kind(e) == 3 {
                    far_point(e, l, point);
                    transformed_point(e, f + 0x140, point);
                } else {
                    transformed_point(e, f + 0x134, l.wrapping_add(0x8c));
                }
                point_and_one(e)
            }
            10 | 11 => {
                if l == 0 {
                    put_four(e, o, [ONE, ZERO, ZERO, ONE]);
                    return false;
                }
                direction(e, f + if mapping == 10 { 0x14c } else { 0x164 });
                if mapping == 11 {
                    let transposed = e
                        .call(MATRIX_TRANSPOSE_004768C0, &args![eye, f + 0x200])
                        .u32();
                    let at = e
                        .call(ROTATE_POINT_004B4500, &args![transposed, f + 0x158, point])
                        .u32();
                    copy_words(e, point, at, 3);
                }
                point_and_one(e)
            }
            12 => {
                if l == 0 {
                    return identity_out(e);
                }
                for offset in [0x2cu32, 0x1c, 0xc] {
                    e.mem.set_u32(identity + offset, ZERO);
                }
                e.mem.set_u32(identity + 0x3c, ONE);
                if kind(e) == 3 {
                    far_point(e, l, point);
                    let range = copy_float(e, l.wrapping_add(0x98));
                    e.call(
                        LIGHT_MATRIX_00B71A40,
                        &args![identity, l.wrapping_add(0x68), point, range],
                    );
                } else {
                    e.call(D3D_FROM_NI_00B799A0, &args![identity, l.wrapping_add(0x68)]);
                }
                fn_00e81b00(e, out, Ptr::new(identity));
                true
            }
            13 => {
                if l == 0 {
                    return identity_out(e);
                }
                inverted_eye(e);
                second_matrix(e);
                let product = if kind(e) == 3 {
                    copy_words(e, copy, l.wrapping_add(0x68), 13);
                    far_point(e, l, point);
                    copy_words(e, copy + 0x24, point, 3);
                    e.call(TRANSFORM_PRODUCT_0062C250, &args![inverse, f + 0x224, copy])
                        .u32()
                } else {
                    e.call(
                        TRANSFORM_PRODUCT_0062C250,
                        &args![inverse, identity, l.wrapping_add(0x68)],
                    )
                    .u32()
                };
                e.call(D3D_FROM_NI_00B799A0, &args![second, product]);
                fn_00e81b00(e, out, Ptr::new(second));
                true
            }
            14 => {
                let minus_one = copy_float(e, MINUS_ONE_01012054);
                if l == 0 || kind(e) != 4 {
                    put_four(e, o, [minus_one, minus_one, ZERO, ZERO]);
                    return l != 0;
                }
                let pi = x87_load64(e.mem.u64(PI_0101FF40));
                let degrees = x87_load64(e.mem.u64(HALF_TURN_DEGREES_0104ED58));
                let cosine = |e: &mut Engine, at: u32| {
                    let radians = x87_store32(x87_div(x87_mul(load_at(e, at), pi), degrees));
                    crt_on(e, CRT_HELPER_00EC9F70, radians)
                };
                let first = cosine(e, l.wrapping_add(0x10c));
                e.mem.set_u32(o, first);
                let second_value = cosine(e, l.wrapping_add(0x108));
                e.mem.set_u32(o + 4, second_value);
                let third = copy_float(e, l.wrapping_add(0x110));
                e.mem.set_u32(o + 8, third);
                e.mem.set_u32(o + 0xc, ZERO);
                true
            }
            15 => {
                if l == 0 {
                    put_four(e, o, [ONE, ZERO, ZERO, ZERO]);
                    return false;
                }
                if e.call(IS_KIND_OF_0043B300, &args![SPOT_CLASS_011F4A98, l])
                    .bool()
                {
                    let at = l.wrapping_add(0xf0);
                    let values = [
                        copy_float(e, at),
                        copy_float(e, at + 4),
                        copy_float(e, at + 8),
                        ZERO,
                    ];
                    put_four(e, o, values);
                } else {
                    put_four(e, o, [ONE, ZERO, ZERO, ZERO]);
                }
                true
            }
            16 | 23 => {
                if l == 0 {
                    return identity_out(e);
                }
                second_matrix(e);
                let with = if mapping == 16 {
                    ORIGIN_011F426C
                } else {
                    l.wrapping_add(0x118)
                };
                light_matrix(e, l.wrapping_add(0xf4), with)
            }
            17 => {
                if l == 0 {
                    return identity_out(e);
                }
                second_matrix(e);
                let transposed = e
                    .call(MATRIX_TRANSPOSE_004768C0, &args![eye, f + 0x194])
                    .u32();
                let rotated = e
                    .call(
                        MATRIX_TIMES_POINT_0043F8D0,
                        &args![transposed, f + 0x1b8, l.wrapping_add(0xf4)],
                    )
                    .u32();
                light_matrix(e, rotated, ORIGIN_011F426C)
            }
            19 => {
                if l == 0 {
                    return zero_one(e);
                }
                inverted_eye(e);
                copy_words(e, point, l.wrapping_add(0x118), 3);
                transformed_point(e, f + 0x128, point);
                point_and_one(e)
            }
            20 | 21 => {
                if l == 0 {
                    put_four(e, o, [ONE, ZERO, ZERO, ZERO]);
                    return false;
                }
                if e.mem.u8(l.wrapping_add(0x138)) == 0 {
                    put_four(e, o, [ZERO; 4]);
                    return true;
                }
                if mapping == 20 {
                    let at = l.wrapping_add(0x14c);
                    let values = [
                        copy_float(e, at),
                        copy_float(e, at + 4),
                        copy_float(e, at + 8),
                        copy_float(e, at + 0xc),
                    ];
                    put_four(e, o, values);
                    return true;
                }
                inverted_eye(e);
                copy_words(e, l18, l.wrapping_add(0x14c), 4);
                e.call(ROTATE_POINT_004B4500, &args![inverse, point, l18]);
                let w = load_at(e, l18 + 0xc);
                let x = x87_store32(x87_mul(w, operand_at(e, l18)));
                e.mem.set_u32(l68, x);
                let y = x87_store32(x87_mul(load_at(e, l18 + 4), w));
                e.mem.set_u32(l68 + 4, y);
                let z = x87_store32(x87_mul(w, operand_at(e, l18 + 8)));
                e.mem.set_u32(l68 + 8, z);
                e.call(TRANSFORM_POINT_00524C40, &args![inverse, l18, l68]);
                let values = [
                    copy_float(e, point),
                    copy_float(e, point + 4),
                    copy_float(e, point + 8),
                ];
                for (i, value) in values.iter().enumerate() {
                    e.mem.set_u32(o + 4 * i as u32, *value);
                }
                let dot = e.call(POINT_DOT_004B6190, &args![point, l18]).f64();
                e.mem.set_u32(o + 0xc, x87_store32(dot));
                true
            }
            22 => {
                if l == 0 {
                    put_four(e, o, [ZERO; 4]);
                    return false;
                }
                e.mem.set_u32(l14, ZERO);
                let first = match e.mem.u32(l.wrapping_add(0x134)) {
                    0 | 1 => copy_float(e, TWO_010162C0),
                    2 | 3 => copy_float(e, THREE_01017718),
                    4 => ONE,
                    _ => ZERO,
                };
                e.mem.set_u32(l14, first);
                put_four(e, o, [first, ZERO, ZERO, ZERO]);
                true
            }
            24 => {
                if l == 0 {
                    return identity_out(e);
                }
                second_matrix(e);
                inverted_eye(e);
                copy_words(e, point, l.wrapping_add(0x118), 3);
                let at = e
                    .call(TRANSFORM_POINT_00524C40, &args![inverse, f + 0x11c, point])
                    .u32();
                copy_words(e, point, at, 3);
                let transposed = e
                    .call(MATRIX_TRANSPOSE_004768C0, &args![eye, f + 0x1dc])
                    .u32();
                let rotated = e
                    .call(
                        MATRIX_TIMES_POINT_0043F8D0,
                        &args![transposed, f + 0x170, l.wrapping_add(0xf4)],
                    )
                    .u32();
                light_matrix(e, rotated, point)
            }
            _ => {
                e.call(MEMSET_00EC61C0, &args![o, 0u32, size]);
                false
            }
        }
    })
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slot `s` holds the address `tag + s`.
    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    const PROGRAM_TAG: u32 = 0x7100_0000;
    const MAP_TAG: u32 = 0x7200_0000;
    const ENTRY_TAG: u32 = 0x7300_0000;

    /// An engine with the scratch globals mapped, the type table set (low byte 1: type 1,
    /// 3: 3, 4: 4, 5: 5, 6: 6, 7: 7, 8: 8, 9: 9, 10: 10, 2: 2) and a program accepting
    /// every constant.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(CONSTANT_TYPE_TABLE_011F5FC4, 0x400);
        for i in 0..0x100u32 {
            e.mem.set_u32(CONSTANT_TYPE_TABLE_011F5FC4 + 4 * i, i);
        }
        e.map(CONSTANT_BYTE_0126F7B0, 0x200);
        e.map(FIXED_STRING_NULL_0109B220, 4);
        e.mem.set_u32(FIXED_STRING_NULL_0109B220, 0x5550);
        let program = object(&mut e, PROGRAM_TAG, 0x10);
        e.register(PROGRAM_TAG + 0x74, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        (e, program)
    }

    /// An entry of type `kind` (the flags' low byte), its data `size` bytes of stride
    /// `stride` from `data`.
    fn entry(e: &mut Engine, kind: u32, size: u32, stride: u32, data: u32) -> u32 {
        let entry = object(e, ENTRY_TAG, 0x38);
        e.mem.set_u32(entry + 0x14, kind);
        e.mem.set_u32(entry + 0x28, size);
        e.mem.set_u32(entry + 0x2c, stride);
        e.mem.set_u32(entry + 0x30, data);
        entry
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
        at
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    #[test]
    fn fn_00a9beb0_and_fn_00e81b00() {
        let (mut e, _) = engine();
        e.map(NI_REF_OBJECT_COUNT_011F4400, 4);
        quiet(
            &mut e,
            &[IMPORT_INTERLOCKED_DECREMENT, NI_FREE_SIZED_00AA1460],
        );
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x00a9_beb0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0xc]]);
        e.call(0x00a9_beb0, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460).len(), 1);
        let values: Vec<f32> = (0..16).map(|i| i as f32).collect();
        let source = floats(&mut e, &values);
        let destination = e.mem.alloc(0x40);
        e.call(0x00e8_1b00, &args![destination, source]);
        assert_eq!(read(&e, destination, 16), values);
    }

    #[test]
    fn ni_scm_extra_data_get_next_entry_moves_on_a_match() {
        let (mut e, _) = engine();
        e.map(ENTRY_KEY_SHIFT_01101294, 4);
        e.mem.set_u32(ENTRY_KEY_SHIFT_01101294, 16);
        let this = e.mem.alloc(0x30);
        let pairs = e.mem.alloc(0x10);
        e.mem.set_u32(pairs, (2 << 16) | 5);
        e.mem.set_u32(pairs + 4, 0xaa);
        e.mem.set_u32(this + 0x24 + 4, pairs);
        assert_eq!(
            e.call(0x00e8_1b60, &args![this, 5u32, 2u32, 1u32, 0u32])
                .u32(),
            0xaa
        );
        assert_eq!(e.mem.u32(this + 0x18 + 4), 1);
        assert_eq!(
            e.call(0x00e8_1b60, &args![this, 5u32, 2u32, 1u32, 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn entry_setters() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[
                IMPORT_INTERLOCKED_INCREMENT,
                IMPORT_INTERLOCKED_DECREMENT,
                NI_FREE_DATA_00AA10F0,
                MEMCPY,
            ],
        );
        e.register(NI_ALLOCATE_DATA_00AA1070, |_, _| 0x8800u32.into_ret());
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x24, 0x5550);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x6008);
        e.call(0x00e8_1c60, &args![entry, name]);
        assert_eq!(e.mem.u32(entry + 0x24), 0x6008);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT), vec![vec![0x6000]]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        e.call(0x00e8_1c60, &args![entry, name]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).len(), 1);
        e.call(0x00e8_1ca0, &args![entry, 8u32, 4u32, 0x9000u32, 1u32]);
        assert_eq!(e.mem.u32(entry + 0x28), 8);
        assert_eq!(e.mem.u32(entry + 0x2c), 4);
        assert_eq!(e.mem.u32(entry + 0x30), 0x8800);
        assert_eq!(e.mem.u8(entry + 0x34), 1);
        assert_eq!(calls(&e, MEMCPY), vec![vec![0x8800, 0x9000, 8]]);
        e.call(0x00e8_1ca0, &args![entry, 8u32, 4u32, 0x9000u32, 0u32]);
        assert_eq!(e.mem.u32(entry + 0x30), 0x9000);
        assert_eq!(e.mem.u8(entry + 0x34), 0);
    }

    #[test]
    fn entries_by_index_and_key() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0xc);
        let a = e.mem.alloc(0x38);
        let b = e.mem.alloc(0x38);
        e.mem.set_u32(a + 0xc, 0x100);
        e.mem.set_u32(b + 0xc, 0x200);
        e.mem.set_u32(elements, a);
        e.mem.set_u32(elements + 8, b);
        e.mem.set_u32(this + 0x10, elements);
        e.mem.set_u16(this + 0x16, 3);
        assert_eq!(e.call(0x00e8_26c0, &args![this, 2u32]).u32(), b);
        e.register(NI_STRICMP, |_, a| u32::from(a[0] != a[1]).into_ret());
        assert_eq!(e.call(0x00e8_2960, &args![this, 0x200u32]).u32(), 2);
        assert_eq!(
            e.call(0x00e8_2960, &args![this, 0x300u32]).u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn setup_object_entry_looks_up_the_mapping() {
        let (mut e, _) = engine();
        e.map(PREDEFINED_TYPE_TABLE_011F5FB4, 0x10);
        e.mem.set_u8(PREDEFINED_TYPE_TABLE_011F5FB4 + 2, 7);
        e.register(LOOK_UP_OBJECT_MAPPING, |e, a| {
            e.mem.set_u32(a[1], 0x33);
            1u32.into_ret()
        });
        e.register(LOOK_UP_OBJECT_MAPPING_TYPE, |e, a| {
            e.mem.set_u32(a[1], 4);
            2u32.into_ret()
        });
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x14, 0x6000_0011);
        assert_eq!(e.call(0x00e8_2ac0, &args![0u32, entry]).u32(), 0);
        assert_eq!(e.mem.u32(entry + 0x14), 0x6000_0007);
        assert_eq!(e.mem.u32(entry + 0x10), 0x33);
        assert_eq!(e.mem.u32(entry + 0x20), 4);
        e.register(LOOK_UP_OBJECT_MAPPING, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e8_2ac0, &args![0u32, entry]).u32(), 0x40);
    }

    #[test]
    fn map_constant_value_by_count_and_type() {
        let (mut e, _) = engine();
        let data = floats(
            &mut e,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        );
        let one = entry(&mut e, 4, 4, 4, data);
        assert_eq!(
            e.call(0x00e8_3a90, &args![one]).u32(),
            CONSTANT_FLOATS_0126F7CC
        );
        assert_eq!(read(&e, CONSTANT_FLOATS_0126F7CC, 4), vec![1.0; 4]);
        let byte = entry(&mut e, 1, 4, 4, data);
        e.mem.set_u8(data, 0x81);
        assert_eq!(
            e.call(0x00e8_3a90, &args![byte]).u32(),
            CONSTANT_BYTE_0126F7B0
        );
        assert_eq!(e.mem.u32(CONSTANT_BYTE_0126F7B0), 0x81);
        e.mem.set_u32(data, 1.0f32.to_bits());
        let pair = entry(&mut e, 3, 8, 4, data);
        e.call(0x00e8_3a90, &args![pair]);
        let words: Vec<u32> = (0..4)
            .map(|i| e.mem.u32(CONSTANT_WORDS_0126F7B4 + 4 * i))
            .collect();
        let (x, y) = (1.0f32.to_bits(), 2.0f32.to_bits());
        assert_eq!(words, vec![x, y, x, y]);
        let three = entry(&mut e, 5, 12, 4, data);
        e.call(0x00e8_3a90, &args![three]);
        assert_eq!(
            read(&e, CONSTANT_FLOATS_0126F7CC, 4),
            vec![1.0, 2.0, 3.0, 1.0]
        );
        let four = entry(&mut e, 5, 16, 4, data);
        assert_eq!(e.call(0x00e8_3a90, &args![four]).u32(), data);
        let nine = entry(&mut e, 5, 36, 4, data);
        e.call(0x00e8_3a90, &args![nine]);
        assert_eq!(
            read(&e, CONSTANT_MATRIX_0126F7E0, 16),
            vec![1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0, 7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
        let twelve = entry(&mut e, 5, 48, 4, data);
        e.call(0x00e8_3a90, &args![twelve]);
        assert_eq!(
            read(&e, CONSTANT_MATRIX_0126F7E0 + 0x2c, 5),
            vec![12.0, 0.0, 0.0, 0.0, 0.0]
        );
        let eight = entry(&mut e, 5, 32, 4, data);
        e.call(0x00e8_3a90, &args![eight]);
        assert_eq!(read(&e, CONSTANT_MATRIX_0126F7E0 + 0x1c, 2), vec![8.0, 0.0]);
        let twenty = entry(&mut e, 5, 80, 4, data);
        assert_eq!(e.call(0x00e8_3a90, &args![twenty]).u32(), data);
        let odd = entry(&mut e, 5, 20, 4, data);
        assert_eq!(e.call(0x00e8_3a90, &args![odd]).u32(), 0);
    }

    #[test]
    fn set_constant_constant_maps_and_sets() {
        let (mut e, program) = engine();
        let data = floats(&mut e, &[2.0]);
        let one = entry(&mut e, 4, 4, 4, data);
        assert_eq!(
            e.call(0x00e8_4b60, &args![0u32, program, one, 0u32]).u32(),
            0
        );
        assert_eq!(
            calls(&e, PROGRAM_TAG + 0x74),
            vec![vec![program, one, CONSTANT_FLOATS_0126F7CC, 0]]
        );
        e.register(PROGRAM_TAG + 0x74, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00e8_4b60, &args![0u32, program, one, 0u32]).u32(),
            0x50
        );
        let odd = entry(&mut e, 4, 20, 4, data);
        assert_eq!(
            e.call(0x00e8_4b60, &args![0u32, program, odd, 0u32]).u32(),
            1
        );
    }

    #[test]
    fn remove_entry_releases_a_global_key() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[
                IMPORT_INTERLOCKED_INCREMENT,
                IMPORT_INTERLOCKED_DECREMENT,
                RELEASE_GLOBAL_SHADER_CONSTANT,
            ],
        );
        e.map(NULL_ELEMENT_GUARD_0126F914, 4);
        e.mem.set_u32(NULL_ELEMENT_GUARD_0126F914, 1);
        e.map(NULL_ELEMENT_0126F910, 4);
        let this = object(&mut e, MAP_TAG, 0x40);
        let elements = e.mem.alloc(8);
        let global = e.mem.alloc(0x38);
        e.mem.set_u32(global + 0x14, 0x4000_0000);
        e.mem.set_u32(global + 0xc, 0x6008);
        e.mem.set_u32(elements + 4, global);
        e.mem.set_u32(this + 0x10, elements);
        e.mem.set_u16(this + 0x14, 2);
        e.mem.set_u16(this + 0x16, 2);
        e.mem.set_u32(this + 0x34, 0x22);
        e.register(MAP_TAG + 0x7c, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00e8_7010, &args![this, 0x100u32]).u32(), 0x10);
        e.register(MAP_TAG + 0x7c, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0x34, 0);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(global, table);
        e.mem.set_u32(table + 4, ENTRY_TAG + 4);
        quiet(&mut e, &[ENTRY_TAG + 4]);
        assert_eq!(e.call(0x00e8_7010, &args![this, 0x100u32]).u32(), 0);
        assert_eq!(calls(&e, RELEASE_GLOBAL_SHADER_CONSTANT).len(), 1);
        assert_eq!(e.mem.u32(elements + 4), 0);
    }
}

#[cfg(test)]
mod tests_n084_values {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn object(e: &mut Engine, tag: u32, size: u32) -> u32 {
        let o = e.mem.alloc(size.max(8));
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200u32).step_by(4) {
            e.mem.set_u32(table + slot, tag + slot);
        }
        e.mem.set_u32(o, table);
        o
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    const PROGRAM_TAG: u32 = 0x7100_0000;
    const MAP_TAG: u32 = 0x7200_0000;
    const ENTRY_TAG: u32 = 0x7300_0000;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(CONSTANT_TYPE_TABLE_011F5FC4, 0x400);
        for i in 0..0x100u32 {
            e.mem.set_u32(CONSTANT_TYPE_TABLE_011F5FC4 + 4 * i, i);
        }
        e.map(CONSTANT_BYTE_0126F7B0, 0x200);
        e.map(FIRST_MATRIX_0126F820, 0x100);
        e.map(FIXED_STRING_NULL_0109B220, 4);
        e.mem.set_u32(FIXED_STRING_NULL_0109B220, 0x5550);
        e.map(NI_REF_OBJECT_COUNT_011F4400, 4);
        e.put_vtable(ENTRY_VTABLE_010EF4D8, &[ENTRY_TAG]);
        e.register(ENTRY_TAG, |_, _| Ret::default());
        let program = object(&mut e, PROGRAM_TAG, 0x10);
        e.register(PROGRAM_TAG + 0x74, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                IMPORT_INTERLOCKED_INCREMENT,
                IMPORT_INTERLOCKED_DECREMENT,
                NI_FREE_DATA_00AA10F0,
                MEMCPY,
                MATRIX_INVERSE_00EE6DDA,
                MATRIX_TRANSPOSE_00EE6DD4,
                MATRIX_MULTIPLY_00EE6DE0,
                VEC4_TRANSFORM_00EE6E34,
            ],
        );
        e.register(NI_ALLOCATE_DATA_00AA1070, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MAKE_FIXED_STRING_00A5B690, |_, a| (a[0] + 8).into_ret());
        e.call_log = Some(vec![]);
        (e, program)
    }

    fn entry(e: &mut Engine, kind: u32, size: u32, stride: u32, data: u32) -> u32 {
        let entry = object(e, ENTRY_TAG, 0x38);
        e.mem.set_u32(entry + 0x14, kind);
        e.mem.set_u32(entry + 0x28, size);
        e.mem.set_u32(entry + 0x2c, stride);
        e.mem.set_u32(entry + 0x30, data);
        entry
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len().max(1) as u32);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
        at
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
    }

    #[test]
    fn the_entry_adders() {
        let (mut e, _) = engine();
        let this = object(&mut e, MAP_TAG, 0x40);
        e.register(MAP_TAG + 0x80, |_, _| 0u32.into_ret());
        e.register(MAP_TAG + 0xa0, |_, _| 0u32.into_ret());
        quiet(&mut e, &[REGISTER_GLOBAL_SHADER_CONSTANT]);
        let r = e
            .call(
                0x00e8_2090,
                &args![
                    this,
                    0x100u32,
                    0xf123_4567u32,
                    1u32,
                    2u32,
                    3u32,
                    0x200u32,
                    8u32,
                    4u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        assert_eq!(r, 0);
        let attribute = calls(&e, MAP_TAG + 0x80)[0][1];
        assert_eq!(e.mem.u32(attribute + 0x14), 0x3123_4567);
        assert_eq!(e.mem.u32(attribute + 0x18), 1);
        assert_eq!(e.mem.u32(attribute + 0x1c), 2);
        assert_eq!(e.mem.u32(attribute + 0x20), 3);
        assert_eq!(e.mem.u32(attribute + 0xc), 0x108);
        assert_eq!(e.mem.u32(attribute + 0x24), 0x208);
        // No data source: size and stride stay unset.
        assert_eq!(e.mem.u32(attribute + 0x28), 0);
        let source = floats(&mut e, &[1.0, 2.0]);
        e.call(
            0x00e8_21e0,
            &args![this, 0x100u32, 0x5u32, 1u32, 2u32, 3u32, 0u32, 8u32, 4u32, source, 0u32],
        );
        let global = calls(&e, MAP_TAG + 0x80)[1][1];
        assert_eq!(e.mem.u32(global + 0x14), 0x4000_0005);
        assert_eq!(e.mem.u32(global + 0x30), source);
        let registered = &calls(&e, REGISTER_GLOBAL_SHADER_CONSTANT)[0];
        assert_eq!(registered[1..], [5, 8, source]);
        e.call(
            0x00e8_23d0,
            &args![this, 0u32, 0x7u32, 1u32, 2u32, 3u32, 0u32],
        );
        let operator = calls(&e, MAP_TAG + 0x80)[2][1];
        assert_eq!(e.mem.u32(operator + 0x14), 0x5000_0007);
        e.map(OBJECT_FLAGS_011F5F7C, 0x10);
        e.mem.set_u16(OBJECT_FLAGS_011F5F7C + 4, 0x0302);
        e.call(0x00e8_2500, &args![this, 0u32, 9u32, 0u32, 0x44u32, 2u32]);
        let object_entry = calls(&e, MAP_TAG + 0xa0)[0][1];
        assert_eq!(e.mem.u32(object_entry + 0x14), 0x6000_0302);
        assert_eq!(e.mem.u32(object_entry + 0x18), 0x44);
        assert_eq!(e.mem.u32(object_entry + 0x1c), 9);
        // An insertion error deletes the entry and is answered.
        e.register(MAP_TAG + 0x80, |_, _| 0x30u32.into_ret());
        quiet(&mut e, &[ENTRY_TAG]);
        let r = e
            .call(
                0x00e8_23d0,
                &args![this, 0u32, 0x7u32, 1u32, 2u32, 3u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x30);
        assert_eq!(calls(&e, ENTRY_TAG).len(), 1);
        // A set-up error of an object entry deletes it too.
        e.register(MAP_TAG + 0xa0, |_, _| 0x40u32.into_ret());
        let r = e
            .call(0x00e8_2500, &args![this, 0u32, 9u32, 0u32, 0x44u32, 2u32])
            .u32();
        assert_eq!(r, 0x40);
        assert_eq!(calls(&e, ENTRY_TAG).len(), 2);
    }

    #[test]
    fn perform_operator_multiply() {
        let (mut e, program) = engine();
        put(&mut e, FIRST_MATRIX_0126F820, &[2.0; 16]);
        put(&mut e, SECOND_VECTOR_0126F8B0, &[3.0, 9.0, 9.0, 9.0]);
        assert_eq!(
            e.call(0x00e8_2b40, &args![program, 1u32, 9u32, 3u32, 1u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(read(&e, MATRIX_RESULT_0126F8C0, 16), vec![6.0; 16]);
        assert_eq!(
            calls(&e, MATRIX_INVERSE_00EE6DDA),
            vec![vec![MATRIX_RESULT_0126F8C0, 0, MATRIX_RESULT_0126F8C0]]
        );
        assert!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
        e.call(0x00e8_2b40, &args![program, 1u32, 9u32, 9u32, 0u32, 1u32]);
        assert_eq!(
            calls(&e, MATRIX_MULTIPLY_00EE6DE0),
            vec![vec![
                MATRIX_RESULT_0126F8C0,
                FIRST_MATRIX_0126F820,
                SECOND_MATRIX_0126F860
            ]]
        );
        assert_eq!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        e.call(0x00e8_2b40, &args![program, 1u32, 9u32, 7u32, 0u32, 0u32]);
        e.call(0x00e8_2b40, &args![program, 1u32, 10u32, 9u32, 0u32, 0u32]);
        assert_eq!(
            calls(&e, VEC4_TRANSFORM_00EE6E34),
            vec![
                vec![
                    VECTOR_RESULT_0126F900,
                    SECOND_VECTOR_0126F8B0,
                    FIRST_MATRIX_0126F820
                ],
                vec![
                    VECTOR_RESULT_0126F900,
                    FIRST_VECTOR_0126F8A0,
                    SECOND_MATRIX_0126F860
                ]
            ]
        );
        put(&mut e, FIRST_VECTOR_0126F8A0, &[1.0, 2.0, 3.0, 4.0]);
        e.call(0x00e8_2b40, &args![program, 1u32, 7u32, 4u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![3.0, 6.0, 9.0, 12.0]
        );
        e.call(0x00e8_2b40, &args![program, 1u32, 7u32, 10u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![3.0, 18.0, 27.0, 36.0]
        );
        assert_eq!(
            e.call(0x00e8_2b40, &args![program, 1u32, 9u32, 5u32, 0u32, 0u32])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00e8_2b40, &args![program, 1u32, 3u32, 3u32, 0u32, 0u32])
                .u32(),
            1
        );
        e.register(PROGRAM_TAG + 0x74, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00e8_2b40, &args![program, 1u32, 7u32, 4u32, 0u32, 0u32])
                .u32(),
            0x50
        );
    }

    #[test]
    fn perform_operator_divide() {
        let (mut e, program) = engine();
        put(&mut e, FIRST_MATRIX_0126F820, &[6.0; 16]);
        put(&mut e, SECOND_VECTOR_0126F8B0, &[2.0, 4.0, 8.0, 16.0]);
        assert_eq!(
            e.call(0x00e8_2e70, &args![program, 1u32, 9u32, 4u32, 0u32, 1u32])
                .u32(),
            0
        );
        assert_eq!(read(&e, MATRIX_RESULT_0126F8C0, 16), vec![3.0; 16]);
        assert_eq!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        assert_eq!(
            e.call(0x00e8_2e70, &args![program, 1u32, 9u32, 7u32, 0u32, 0u32])
                .u32(),
            1
        );
        put(&mut e, FIRST_VECTOR_0126F8A0, &[8.0, 8.0, 8.0, 8.0]);
        e.call(0x00e8_2e70, &args![program, 1u32, 10u32, 3u32, 0u32, 0u32]);
        assert_eq!(read(&e, VECTOR_RESULT_0126F900, 4), vec![4.0; 4]);
        e.call(0x00e8_2e70, &args![program, 1u32, 7u32, 7u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![4.0, 2.0, 1.0, 0.5]
        );
        assert_eq!(
            e.call(0x00e8_2e70, &args![program, 1u32, 7u32, 9u32, 0u32, 0u32])
                .u32(),
            1
        );
    }

    #[test]
    fn the_operator_add_and_subtract() {
        let (mut e, program) = engine();
        put(&mut e, FIRST_VECTOR_0126F8A0, &[1.0, 2.0, 3.0, 4.0]);
        put(&mut e, SECOND_VECTOR_0126F8B0, &[10.0, 20.0, 30.0, 40.0]);
        e.call(0x00e8_30f0, &args![program, 1u32, 7u32, 10u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![11.0, 22.0, 33.0, 44.0]
        );
        e.call(0x00e8_30f0, &args![program, 1u32, 3u32, 7u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![11.0, 21.0, 31.0, 41.0]
        );
        e.call(0x00e8_30f0, &args![program, 1u32, 7u32, 4u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![11.0, 12.0, 13.0, 14.0]
        );
        assert_eq!(
            e.call(0x00e8_30f0, &args![program, 1u32, 9u32, 7u32, 0u32, 0u32])
                .u32(),
            1
        );
        e.call(0x00e8_3320, &args![program, 1u32, 3u32, 7u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![-9.0, -19.0, -29.0, -39.0]
        );
        e.call(0x00e8_3320, &args![program, 1u32, 7u32, 3u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![-9.0, -8.0, -7.0, -6.0]
        );
        e.call(0x00e8_3320, &args![program, 1u32, 4u32, 4u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, VECTOR_RESULT_0126F900, 4),
            vec![-9.0, -18.0, -27.0, -36.0]
        );
        assert_eq!(
            e.call(0x00e8_3320, &args![program, 1u32, 7u32, 8u32, 0u32, 0u32])
                .u32(),
            1
        );
    }

    #[test]
    fn setup_texture_transform_matrix_by_dimension() {
        let (mut e, _) = engine();
        let out = e.mem.alloc(0x40);
        e.call(0x00e8_3550, &args![out, 0u32, 0u32, 0u32, 4u32]);
        let identity: Vec<f32> = (0..16)
            .map(|i| if i % 5 == 0 { 1.0 } else { 0.0 })
            .collect();
        assert_eq!(read(&e, out, 16), identity);
        let m = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        e.call(0x00e8_3550, &args![out, m, 0u32, 0u32, 3u32]);
        assert_eq!(
            read(&e, out, 16),
            vec![1.0, 4.0, 0.0, 0.0, 2.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 6.0, 0.0, 0.0]
        );
        e.call(0x00e8_3550, &args![out, m, 1u32, 1u32, 2u32]);
        assert_eq!(read(&e, out + 0x20, 2), vec![3.0, 6.0]);
        assert_eq!(read(&e, out + 0x30, 2), vec![0.0, 0.0]);
        assert_eq!(calls(&e, MATRIX_INVERSE_00EE6DDA), vec![vec![out, 0, out]]);
        assert_eq!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4), vec![vec![out, out]]);
    }

    #[test]
    fn map_attribute_value_by_type() {
        let (mut e, _) = engine();
        let attribute = e.mem.alloc(0x20);
        let pointed = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        e.mem.set_u32(attribute + 0x10, pointed);
        put(&mut e, attribute + 0xc, &[0.5]);
        let typed = |e: &mut Engine, kind: u32| entry(e, kind, 4, 4, 0);
        let five = typed(&mut e, 5);
        e.call(0x00e8_3840, &args![five, attribute]);
        assert_eq!(
            read(&e, CONSTANT_FLOATS_0126F7CC, 4),
            vec![1.0, 2.0, 1.0, 2.0]
        );
        let four = typed(&mut e, 4);
        e.call(0x00e8_3840, &args![four, attribute]);
        assert_eq!(read(&e, CONSTANT_FLOATS_0126F7CC, 4), vec![0.5; 4]);
        let six = typed(&mut e, 6);
        e.call(0x00e8_3840, &args![six, attribute]);
        assert_eq!(
            read(&e, CONSTANT_FLOATS_0126F7CC, 4),
            vec![1.0, 2.0, 3.0, 1.0]
        );
        let nine = typed(&mut e, 9);
        assert_eq!(e.call(0x00e8_3840, &args![nine, attribute]).u32(), pointed);
        let two = typed(&mut e, 2);
        assert_eq!(e.call(0x00e8_3840, &args![two, attribute]).u32(), 0);
        let eight = typed(&mut e, 8);
        put(&mut e, CONSTANT_MATRIX_0126F7E0 + 0x30, &[7.5]);
        e.call(0x00e8_3840, &args![eight, attribute]);
        assert_eq!(
            read(&e, CONSTANT_MATRIX_0126F7E0 + 0x20, 5),
            vec![7.0, 8.0, 9.0, 0.0, 7.5]
        );
        e.register(IS_KIND_OF_0045BAD0, |_, a| {
            u32::from(a[0] == ATTRIBUTE_CLASS_011F5B68).into_ret()
        });
        let seven = typed(&mut e, 7);
        e.call(0x00e8_3840, &args![seven, attribute]);
        assert_eq!(read(&e, CONSTANT_FLOATS_0126F7CC, 1), vec![0.5]);
    }

    #[test]
    fn set_attribute_constant_finds_extra_data() {
        let (mut e, program) = engine();
        let this = e.mem.alloc(0x40);
        let object_with = e.mem.alloc(0x10);
        let data = floats(&mut e, &[4.0]);
        let constant = entry(&mut e, 4, 4, 4, data);
        e.register(GET_EXTRA_DATA_00A5BDD0, |_, _| 0u32.into_ret());
        let words = [0u32; 7];
        let call = |e: &mut Engine, object: u32, extra: u32| {
            e.call(
                0x00e8_4ba0,
                &args![
                    this, program, constant, object, words[0], words[1], words[2], words[3],
                    words[4], words[5], words[6], 0x11u32, 0x22u32, extra
                ],
            )
            .u32()
        };
        assert_eq!(call(&mut e, 0, 0), 1);
        // No extra data: the constant's own value.
        assert_eq!(call(&mut e, object_with, 0), 0);
        assert_eq!(
            calls(&e, PROGRAM_TAG + 0x74)[0][2],
            CONSTANT_FLOATS_0126F7CC
        );
        // Extra data found and recorded with the shader's extra data.
        let attribute = e.mem.alloc(0x20);
        put(&mut e, attribute + 0xc, &[2.0]);
        e.register_double(GET_EXTRA_DATA_00A5BDD0, move |_, _| attribute.into_ret());
        quiet(&mut e, &[SCM_ADD_ENTRY_00E80640]);
        e.map(ENTRY_KEY_SHIFT_01101294, 4);
        let extra = e.mem.alloc(0x30);
        let pairs = e.mem.alloc(8);
        e.mem.set_u32(pairs, 0xdead);
        for i in 0..3u32 {
            e.mem.set_u32(extra + 0x24 + 4 * i, pairs);
        }
        e.mem.set_u32(constant + 0x1c, 7);
        assert_eq!(call(&mut e, object_with, extra), 0);
        assert_eq!(
            calls(&e, SCM_ADD_ENTRY_00E80640),
            vec![vec![extra, 7, 0x11, 0, attribute, 0x22]]
        );
        assert_eq!(read(&e, CONSTANT_FLOATS_0126F7CC, 1), vec![2.0]);
        e.register(PROGRAM_TAG + 0x74, |_, _| 0u32.into_ret());
        assert_eq!(call(&mut e, object_with, 0), 0x50);
    }

    #[test]
    fn set_global_constant_widens_the_data() {
        let (mut e, program) = engine();
        let data = floats(&mut e, &[1.0, 2.0, 3.0]);
        let three = entry(&mut e, 6, 12, 4, data);
        assert_eq!(
            e.call(0x00e8_4d00, &args![0u32, program, three, 0u32])
                .u32(),
            0
        );
        assert_eq!(
            read(&e, CONSTANT_FLOATS_0126F7CC, 4),
            vec![1.0, 2.0, 3.0, 1.0]
        );
        let matrix = entry(&mut e, 9, 64, 4, data);
        e.call(0x00e8_4d00, &args![0u32, program, matrix, 0u32]);
        assert_eq!(calls(&e, PROGRAM_TAG + 0x74)[1][2], data);
        let other = entry(&mut e, 2, 4, 4, data);
        assert_eq!(
            e.call(0x00e8_4d00, &args![0u32, program, other, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, PROGRAM_TAG + 0x74).len(), 2);
        e.register(PROGRAM_TAG + 0x74, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00e8_4d00, &args![0u32, program, three, 0u32])
                .u32(),
            0x50
        );
    }

    #[test]
    fn setup_constant_and_attribute_values() {
        let (mut e, _) = engine();
        let data = floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let vector = entry(&mut e, 7, 16, 4, data);
        assert_eq!(e.call(0x00e8_6a50, &args![1u32, vector, 0u32]).u32(), 7);
        assert_eq!(
            read(&e, FIRST_VECTOR_0126F8A0 + 0x10, 4),
            vec![1.0, 2.0, 3.0, 4.0]
        );
        let matrix = entry(&mut e, 8, 36, 4, data);
        assert_eq!(e.call(0x00e8_6a50, &args![0u32, matrix, 0u32]).u32(), 8);
        assert_eq!(
            read(&e, FIRST_MATRIX_0126F820 + 0x20, 8),
            vec![7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        let other = entry(&mut e, 2, 4, 4, data);
        assert_eq!(e.call(0x00e8_6a50, &args![0u32, other, 0u32]).u32(), 0);
        // The attribute form: no object answers 1, no extra data 0x10, an integer a float.
        let this = e.mem.alloc(0x40);
        let integer = entry(&mut e, 3, 4, 4, 0);
        let call = |e: &mut Engine, object: u32| {
            e.call(
                0x00e8_6ca0,
                &args![this, 0u32, integer, object, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32()
        };
        assert_eq!(call(&mut e, 0), 1);
        e.register(GET_EXTRA_DATA_00A5BDD0, |_, _| 0u32.into_ret());
        let holder = e.mem.alloc(8);
        assert_eq!(call(&mut e, holder), 0x10);
        let attribute = e.mem.alloc(0x20);
        e.mem.set_u32(attribute + 0xc, 5);
        e.register_double(GET_EXTRA_DATA_00A5BDD0, move |_, _| attribute.into_ret());
        e.map(TWO_TO_32_01099378, 4);
        assert_eq!(call(&mut e, holder), 3);
        assert_eq!(read(&e, FIRST_VECTOR_0126F8A0, 1), vec![5.0]);
    }

    #[test]
    fn set_operator_constant_runs_the_operator() {
        let (mut e, program) = engine();
        let this = object(&mut e, MAP_TAG, 0x40);
        quiet(&mut e, &[REPORT_ERROR_00AA0020]);
        let a = floats(&mut e, &[2.0, 0.0, 0.0, 0.0]);
        let b = floats(&mut e, &[3.0, 0.0, 0.0, 0.0]);
        let first = entry(&mut e, 7, 16, 4, a);
        e.mem.set_u32(first + 0x14, 0x4000_0007);
        let second = entry(&mut e, 4, 4, 4, b);
        e.mem.set_u32(second + 0x14, 0x4000_0004);
        let elements = [first, second];
        e.register_double(MAP_TAG + 0x74, move |_, a| {
            elements[a[1] as usize].into_ret()
        });
        let operator = entry(&mut e, 0, 0, 4, 0);
        // Multiply operand 0 by operand 1.
        e.mem.set_u32(operator + 0x18, 0x1000_0000 | (1 << 12));
        let words = [0u32; 7];
        let call = |e: &mut Engine| {
            e.call(
                0x00e8_7220,
                &args![
                    this, program, operator, words[0], words[1], words[2], words[3], words[4],
                    words[5], words[6]
                ],
            )
            .u32()
        };
        assert_eq!(call(&mut e), 0);
        assert_eq!(read(&e, VECTOR_RESULT_0126F900, 1), vec![6.0]);
        // A missing operand is reported.
        e.register(MAP_TAG + 0x74, |_, _| 0u32.into_ret());
        assert_eq!(call(&mut e), 1);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, INVALID_ENTRIES_010EF4E8]]
        );
    }
}

#[cfg(test)]
mod tests_n084_dynamic {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, callee: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &f in callees {
            e.register(f, |_, _| Ret::default());
        }
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| f32::from_bits(e.mem.u32(at + 4 * i)))
            .collect()
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, v.to_bits());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FIRST_MATRIX_0126F820, 0x100);
        quiet(
            &mut e,
            &[
                MATRIX_INVERSE_00EE6DDA,
                MATRIX_TRANSPOSE_00EE6DD4,
                MATRIX_MULTIPLY_00EE6DE0,
                MEMSET_00EC61C0,
                INVERT_NON_UNIFORM_004B4880,
                LIGHT_MATRIX_00B71A40,
                D3D_FROM_NI_00B799A0,
                POINT_SCALE_00439180,
            ],
        );
        e.call_log = Some(vec![]);
        e
    }

    /// The map (its renderer at +0x3C, whose matrices at +0x940, +0x980, +0x9C0 hold 1, 2
    /// and 3) and an entry for the predefined constant `id`.
    fn defined_rig(id: u32) -> (Engine, u32, u32) {
        let mut e = engine();
        let renderer = e.mem.alloc(0xb80);
        for (offset, value) in [(0x940u32, 1.0f32), (0x980, 2.0), (0x9c0, 3.0)] {
            put(&mut e, renderer + offset, &[value; 16]);
        }
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + 0x3c, renderer);
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x10, id);
        (e, this, entry)
    }

    fn defined(e: &mut Engine, this: u32, operand: u32, entry: u32, words: [u32; 6]) -> u32 {
        e.call(
            0x00e8_5d10,
            &args![
                this, operand, entry, words[0], words[1], words[2], words[3], words[4], words[5]
            ],
        )
        .u32()
    }

    #[test]
    fn setup_defined_constant_value_copies_renderer_matrices() {
        let (mut e, this, entry) = defined_rig(14);
        assert_eq!(defined(&mut e, this, 1, entry, [0; 6]), 9);
        let matrix = FIRST_MATRIX_0126F820 + 0x40;
        assert_eq!(read(&e, matrix, 16), vec![3.0; 16]);
        assert_eq!(
            calls(&e, MATRIX_INVERSE_00EE6DDA),
            vec![vec![matrix, 0, matrix]]
        );
        assert_eq!(
            calls(&e, MATRIX_TRANSPOSE_00EE6DD4),
            vec![vec![matrix, matrix]]
        );
        let (mut e, this, entry) = defined_rig(5);
        assert_eq!(defined(&mut e, this, 0, entry, [0; 6]), 9);
        assert_eq!(read(&e, FIRST_MATRIX_0126F820, 1), vec![1.0]);
        assert!(calls(&e, MATRIX_INVERSE_00EE6DDA).is_empty());
        let (mut e, this, entry) = defined_rig(11);
        e.call(
            0x00e8_5d10,
            &args![this, 0u32, entry, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, MATRIX_MULTIPLY_00EE6DE0).len(), 2);
    }

    #[test]
    fn setup_defined_constant_value_identity_needs_the_fourth_word() {
        let (mut e, this, entry) = defined_rig(26);
        assert_eq!(defined(&mut e, this, 0, entry, [0; 6]), 0);
        assert_eq!(defined(&mut e, this, 0, entry, [0, 1, 0, 0, 0, 0]), 9);
        let identity: Vec<f32> = (0..16)
            .map(|i| if i % 5 == 0 { 1.0 } else { 0.0 })
            .collect();
        assert_eq!(read(&e, FIRST_MATRIX_0126F820, 16), identity);
        let multiplied = &calls(&e, MATRIX_MULTIPLY_00EE6DE0)[0];
        assert_eq!(
            multiplied[..2],
            [FIRST_MATRIX_0126F820, FIRST_MATRIX_0126F820]
        );
        assert_eq!(calls(&e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        let (mut e, this, entry) = defined_rig(99);
        assert_eq!(defined(&mut e, this, 0, entry, [0, 1, 0, 0, 0, 0]), 9);
        assert_eq!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).len(), 1);
        let (mut e, this, entry) = defined_rig(90);
        assert_eq!(defined(&mut e, this, 0, entry, [0, 1, 0, 0, 0, 0]), 0);
    }

    #[test]
    fn setup_defined_constant_value_vectors() {
        let (mut e, this, entry) = defined_rig(75);
        let properties = e.mem.alloc(0x20);
        assert_eq!(
            defined(&mut e, this, 0, entry, [0, 0, properties, 0, 0, 0]),
            0
        );
        let source = e.mem.alloc(0x40);
        put(&mut e, source + 0x1c, &[1.0, 2.0, 3.0]);
        put(&mut e, source + 0x3c, &[4.0]);
        e.mem.set_u32(properties + 8, source);
        assert_eq!(
            defined(&mut e, this, 1, entry, [0, 0, properties, 0, 0, 0]),
            0xa
        );
        assert_eq!(
            read(&e, FIRST_VECTOR_0126F8A0 + 0x10, 4),
            vec![1.0, 2.0, 3.0, 4.0]
        );
        // 87 with no lights: (0, 0, 0, 1).
        e.mem.set_u32(entry + 0x10, 87);
        assert_eq!(defined(&mut e, this, 0, entry, [0; 6]), 7);
        assert_eq!(read(&e, FIRST_VECTOR_0126F8A0, 4), vec![0.0, 0.0, 0.0, 1.0]);
        // 82 to 86 without the extra data answer 0x10.
        e.mem.set_u32(entry + 0x10, 82);
        e.map(EXTRA_DATA_NAME_011BF65C, 4);
        quiet(
            &mut e,
            &[
                FIXED_STRING_CONSTRUCT_00438170,
                FIXED_STRING_DESTRUCT_004381B0,
            ],
        );
        e.register(GET_EXTRA_DATA_00A5BDD0, |_, _| 0u32.into_ret());
        assert_eq!(defined(&mut e, this, 0, entry, [0; 6]), 0x10);
        let extra = e.mem.alloc(0x10);
        put(&mut e, extra + 0xc, &[0.25]);
        e.register_double(GET_EXTRA_DATA_00A5BDD0, move |_, _| extra.into_ret());
        assert_eq!(defined(&mut e, this, 0, entry, [0; 6]), 7);
        assert_eq!(read(&e, FIRST_VECTOR_0126F8A0, 4), vec![0.25; 4]);
    }

    #[test]
    fn setup_defined_constant_value_texture_transforms() {
        let (mut e, this, entry) = defined_rig(42);
        e.register(UPDATE_MATRIX_00BC5C20, |_, _| 0x4400u32.into_ret());
        let properties = e.mem.alloc(0x20);
        let texturing = e.mem.alloc(0x30);
        let maps = e.mem.alloc(0x40);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 0x77);
        // Ids 41 to 44 take the map at +0x14, as the code reads it.
        e.mem.set_u32(maps + 0x14, map);
        e.mem.set_u32(texturing + 0x20, maps);
        e.mem.set_u32(properties + 0x14, texturing);
        e.map(0x4400, 0x24);
        assert_eq!(
            defined(&mut e, this, 0, entry, [0, 0, properties, 0, 0, 0]),
            9
        );
        assert_eq!(calls(&e, UPDATE_MATRIX_00BC5C20), vec![vec![0x77]]);
        // 42: inverted, not transposed.
        assert_eq!(calls(&e, MATRIX_INVERSE_00EE6DDA).len(), 1);
        assert!(calls(&e, MATRIX_TRANSPOSE_00EE6DD4).is_empty());
    }

    /// A light object (0x160 bytes) with recognisable floats.
    fn light(e: &mut Engine) -> u32 {
        let l = e.mem.alloc(0x160);
        put(e, l + 0xc4, &[0.5]);
        put(e, l + 0xc8, &[1.0, 2.0, 3.0]);
        put(e, l + 0x8c, &[4.0, 5.0, 6.0]);
        put(e, l + 0xf0, &[1.0, 2.0, 4.0]);
        put(e, l + 0x118, &[7.0, 8.0, 9.0]);
        put(e, l + 0x14c, &[1.0, 0.0, 0.0, 2.0]);
        l
    }

    fn effect(e: &mut Engine, out: u32, mapping: u32, light: u32) -> bool {
        e.call(
            0x00e8_4ed0,
            &args![
                out, 0x40u32, mapping, light, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0x6000u32,
                0x6100u32, 0u32
            ],
        )
        .bool()
    }

    #[test]
    fn get_dynamic_effect_data_points_and_defaults() {
        let mut e = engine();
        let out = e.mem.alloc(0x40);
        let l = light(&mut e);
        assert!(effect(&mut e, out, 2, l));
        assert_eq!(read(&e, out, 4), vec![1.0, 2.0, 3.0, 1.0]);
        assert!(!effect(&mut e, out, 2, 0));
        assert_eq!(read(&e, out, 4), vec![0.0, 0.0, 0.0, 1.0]);
        assert!(effect(&mut e, out, 1, l));
        assert_eq!(read(&e, out, 4), vec![0.5, 0.0, 0.0, 0.0]);
        assert!(!effect(&mut e, out, 10, 0));
        assert_eq!(read(&e, out, 4), vec![1.0, 0.0, 0.0, 1.0]);
        assert!(!effect(&mut e, out, 15, 0));
        assert_eq!(read(&e, out, 4), vec![1.0, 0.0, 0.0, 0.0]);
        assert!(effect(&mut e, out, 18, l));
        assert_eq!(read(&e, out, 4), vec![7.0, 8.0, 9.0, 1.0]);
        // A kind-3 light's far point.
        e.mem.set_u8(l + 0x9d, 3);
        e.map(FAR_SCALE_010EF4E0, 8);
        e.mem
            .set_u64(FAR_SCALE_010EF4E0, (-1_048_576.0f64).to_bits());
        assert!(effect(&mut e, out, 8, l));
        assert_eq!(
            read(&e, out, 4),
            vec![-1_048_576.0, -2_097_152.0, -4_194_304.0, 1.0]
        );
        // 5: the point scaled by `00439180`.
        assert!(effect(&mut e, out, 5, l));
        assert_eq!(calls(&e, POINT_SCALE_00439180)[0][1], 0.5f32.to_bits());
        // 20: the plane, or zeros without one.
        assert!(effect(&mut e, out, 20, l));
        assert_eq!(read(&e, out, 4), vec![0.0; 4]);
        e.mem.set_u8(l + 0x138, 1);
        assert!(effect(&mut e, out, 20, l));
        assert_eq!(read(&e, out, 4), vec![1.0, 0.0, 0.0, 2.0]);
        // 22: by the word at +0x134.
        e.mem.set_u32(l + 0x134, 4);
        assert!(effect(&mut e, out, 22, l));
        assert_eq!(read(&e, out, 4), vec![1.0, 0.0, 0.0, 0.0]);
        e.map(THREE_01017718, 4);
        put(&mut e, THREE_01017718, &[3.0]);
        e.mem.set_u32(l + 0x134, 2);
        assert!(effect(&mut e, out, 22, l));
        assert_eq!(read(&e, out, 1), vec![3.0]);
        // Any other mapping clears the output.
        assert!(!effect(&mut e, out, 30, l));
        assert_eq!(
            calls(&e, MEMSET_00EC61C0).last().unwrap(),
            &vec![out, 0, 0x40]
        );
    }

    #[test]
    fn get_dynamic_effect_data_matrices() {
        let mut e = engine();
        let out = e.mem.alloc(0x40);
        // No light: the identity.
        assert!(!effect(&mut e, out, 12, 0));
        let identity: Vec<f32> = (0..16)
            .map(|i| if i % 5 == 0 { 1.0 } else { 0.0 })
            .collect();
        assert_eq!(read(&e, out, 16), identity);
        let l = light(&mut e);
        assert!(effect(&mut e, out, 12, l));
        assert_eq!(calls(&e, D3D_FROM_NI_00B799A0)[0][1], l + 0x68);
        assert!(effect(&mut e, out, 16, l));
        let made = calls(&e, LIGHT_MATRIX_00B71A40);
        assert_eq!(made[0][1..], [l + 0xf4, ORIGIN_011F426C, 1.0f32.to_bits()]);
        e.register(TRANSFORM_PRODUCT_0062C250, |_, _| 0x5000u32.into_ret());
        assert!(effect(&mut e, out, 13, l));
        assert_eq!(calls(&e, INVERT_NON_UNIFORM_004B4880)[0][0], 0x6000);
        assert_eq!(calls(&e, D3D_FROM_NI_00B799A0)[1][1], 0x5000);
        // 14: the cosines of a kind-4 light.
        e.mem.set_u8(l + 0x9d, 4);
        e.map(PI_0101FF40, 8);
        e.mem.set_u64(PI_0101FF40, std::f64::consts::PI.to_bits());
        e.map(HALF_TURN_DEGREES_0104ED58, 8);
        e.mem
            .set_u64(HALF_TURN_DEGREES_0104ED58, 180.0f64.to_bits());
        e.map(MINUS_ONE_01012054, 4);
        put(&mut e, l + 0x10c, &[0.0]);
        put(&mut e, l + 0x110, &[5.0]);
        e.register(CRT_HELPER_00EC9F70, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) + 1.0,
            ..Ret::default()
        });
        assert!(effect(&mut e, out, 14, l));
        assert_eq!(read(&e, out, 4), vec![1.0, 1.0, 5.0, 0.0]);
    }

    #[test]
    fn set_object_constant_fills_the_effect_data() {
        let mut e = engine();
        e.map(OBJECT_TABLE_011F5F90, 0x400);
        e.map(EFFECT_DATA_0126F770, 0x40);
        e.register(GET_DYNAMIC_EFFECT_FOR_OBJECT, |_, _| 0u32.into_ret());
        e.register(LOOK_UP_OBJECT_MAPPING_TYPE, |e, a| {
            e.mem.set_u32(a[1], 1);
            1u32.into_ret()
        });
        let program = e.mem.alloc(8);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(program, table);
        e.mem.set_u32(table + 0x74, 0x7100_0074);
        e.register(0x7100_0074, |_, _| 1u32.into_ret());
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x10, 2);
        let r = e
            .call(
                0x00e8_5c40,
                &args![0u32, program, entry, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        // Mapping 2 without a light: (0, 0, 0, 1) and false, so 0x60.
        assert_eq!(r, 0x60);
        assert_eq!(read(&e, EFFECT_DATA_0126F770, 4), vec![0.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            calls(&e, 0x7100_0074),
            vec![vec![program, entry, EFFECT_DATA_0126F770, 0]]
        );
        e.register(LOOK_UP_OBJECT_MAPPING_TYPE, |_, _| 0u32.into_ret());
        let r = e
            .call(
                0x00e8_5c40,
                &args![0u32, program, entry, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(r, 0x10);
    }
}
