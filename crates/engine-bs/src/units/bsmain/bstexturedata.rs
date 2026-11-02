//! `BSMain/bstexturedata.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c5f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c5ed60` and frees 0x1c bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_texture_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c5_ed60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00c5f560 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01204378 without reading the receiver. The map names this body `BSTextureData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5f560(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_4378
}

/// The global holding `BSTextureData`'s extra-data name (an allocated `NiFixedString`).
const TEXTURE_DATA_NAME: u32 = 0x0120_4374;
/// `"TXD"`.
const TEXTURE_DATA_NAME_TEXT: u32 = 0x010c_3194;
/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c5f240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5f240 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ff15, state variable) is not translated
/// The start-up initializer of the texture-data name: a 4-byte `NiFixedString` from the
/// memory manager set to `"TXD"` (`00a5b690`); the compiler kept a test of the constant
/// text's address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) never runs (translated anyway). The string (or null when the allocation
/// failed) goes to `01204374`.
pub fn fn_00c5f240(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(TEXTURE_DATA_NAME_TEXT);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![TEXTURE_DATA_NAME_TEXT])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(TEXTURE_DATA_NAME, stored);
}

// Translated from 00c5f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the name exists, releases its handle (unless it is the
/// empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the text),
/// frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always true),
/// and clears the global.
pub fn fn_00c5f2e0(e: &mut Engine) {
    let string = e.global::<u32>(TEXTURE_DATA_NAME);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(TEXTURE_DATA_NAME, 0);
}

/// `BSTextureData`: 0x1C bytes, vtable `010c30f4`; an `NiExtraData`, then an array of
/// object pointers at +0xC (its vtable +0xC, elements +0x10, size +0x14, capacity +0x18).
const BS_TEXTURE_DATA_VTABLE: u32 = 0x010c_30f4;
const BS_TEXTURE_DATA_SIZE: u32 = 0x1c;
const ARRAY: u32 = 0xc;
const ELEMENTS: u32 = 0x10;
const COUNT: u32 = 0x14;
/// The array's own fields (relative to it): elements +4, size +8, capacity +0xC; its
/// vtable's slot 4 allocates storage for a count.
const ARRAY_ELEMENTS: u32 = 0x4;
const ARRAY_SIZE: u32 = 0x8;
const ARRAY_CAPACITY: u32 = 0xc;
/// The array's constructor (`00753070`) and destructor (`007530a0`), its clear (`008454f0`,
/// `thiscall`, `(shrink)`) and its storage change (`0042fc00`, `thiscall`, `(capacity,
/// kept)`).
const ARRAY_CONSTRUCT: u32 = 0x0075_3070;
const ARRAY_DESTRUCT: u32 = 0x0075_30a0;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const ARRAY_REALLOCATE: u32 = 0x0042_fc00;
/// `NiExtraData`'s constructor with a name (`00a7b440`, `thiscall`, `(name)`), destructor
/// body (`00a7b300`), member copy (`00a7b330`, `(destination, cloning)`), `LoadBinary`
/// (`00a7b4a0`), `LinkObject` (`00a7b3b0`), `RegisterStreamables` (`00a7b290`),
/// `SaveBinary` (`00a7b2c0`) and `GetViewerStrings` (`00a7b580`).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b440;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_LINK_OBJECT: u32 = 0x00a7_b3b0;
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `00aa13e0` (`cdecl`, one word): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The stream word read (`00a26df0`, `cdecl`, `(stream, &word)`), `NiStream::ReadLinkID`
/// (`00a645b0`), `NiStream::GetObjectFromLinkID` (`00a63430`) and
/// `NiStreamSaveBinary<unsigned int>` (`0044adf0`, `cdecl`, `(stream, &word)`).
const STREAM_READ_WORD: u32 = 0x00a2_6df0;
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_SAVE_WORD: u32 = 0x0044_adf0;
/// `NiGetViewerString` (`00a7d330`, `(name)`), its `(label, text)` form `00a7d5e0` and the
/// viewer-string array's add (`0096a610`, `thiscall`, `(&string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `BSTextureData::ms_RTTI` (its first word is the class name) and `"ObjectName"`.
const RTTI: u32 = 0x0120_4378;
const OBJECT_NAME_LABEL: u32 = 0x010c_3188;

// Translated from 00c5ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5ecf0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fe78, state variable) is not translated
/// Unnamed in the engine map: the `BSTextureData` constructor. `NiExtraData`'s constructor
/// with the name at `01204374`, the vtable and the array's constructor. Returns `this`.
pub fn fn_00c5ecf0(e: &mut Engine, this: Ptr) -> Ptr {
    let name = e.global::<u32>(TEXTURE_DATA_NAME);
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, name]);
    e.mem.set_u32(this.addr(), BS_TEXTURE_DATA_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this.addr().wrapping_add(ARRAY)]);
    this
}

// Translated from 00c5ed60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5ed60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fe9b, state variable) is not translated
/// Unnamed in the engine map: the `BSTextureData` destructor body. The vtable, the array's
/// destructor, then `NiExtraData`'s.
pub fn fn_00c5ed60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_TEXTURE_DATA_VTABLE);
    e.call(ARRAY_DESTRUCT, &args![this.addr().wrapping_add(ARRAY)]);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00c5ee00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5ee00 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2fec5, state variable) is not translated
/// `BSTextureData::CreateClone` (Xbox PDB, engine map): allocates 0x1C bytes, constructs
/// the data in it when that succeeds, copies the members into it (`00c5ee90`; into
/// address 0 when the allocation failed) and returns it.
pub fn bs_texture_data_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_TEXTURE_DATA_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c5ecf0(e, Ptr::new(block))
    };
    fn_00c5ee90(e, this, clone, cloning);
    clone
}

// Translated from 00c5ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(destination, cloning)`, the member copy):
/// `NiExtraData`'s member copy, then the array copied into the destination's
/// (`00c5f500`).
pub fn fn_00c5ee90(e: &mut Engine, this: Ptr, destination: Ptr, cloning: u32) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, destination, cloning]);
    fn_00c5f500(
        e,
        this.addr().wrapping_add(ARRAY),
        destination.addr().wrapping_add(ARRAY),
    );
}

// Translated from 00c5eed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5eed0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2feef, state variable) is not translated
/// Unnamed in the engine map (the `BSTextureData` factory, no arguments): allocates 0x1C
/// bytes and constructs the data in it when that succeeds. Returns it, or 0.
pub fn fn_00c5eed0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![BS_TEXTURE_DATA_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c5ecf0(e, Ptr::new(block))
}

// Translated from 00c5ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::LoadBinary` (Xbox PDB, engine map): `NiExtraData::LoadBinary`, the count
/// (4 bytes through `00a26df0`), that many link ids read (`NiStream::ReadLinkID`, a signed
/// count), and the array sized to it (`00c5f390` with shrinking allowed).
pub fn bs_texture_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let count = e.with_stack(4, |e, local| {
        e.call(STREAM_READ_WORD, &args![stream, local]);
        e.mem.i32(local.addr())
    });
    let mut index = 0i32;
    while index < count {
        e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
        index = index.wrapping_add(1);
    }
    fn_00c5f390(e, this.addr().wrapping_add(ARRAY), count as u32, 1);
}

// Translated from 00c5efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::LinkObject` (Xbox PDB, engine map): `NiExtraData::LinkObject`, then each
/// element (up to the size, read each time) set to the next linked object
/// (`NiStream::GetObjectFromLinkID`).
pub fn bs_texture_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LINK_OBJECT, &args![this, stream]);
    let me = this.addr();
    let mut index = 0u32;
    while index < e.mem.u32(me.wrapping_add(COUNT)) {
        let elements = e.mem.u32(me.wrapping_add(ELEMENTS));
        let slot = elements.wrapping_add(index.wrapping_mul(4));
        let object = e
            .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
            .u32();
        e.mem.set_u32(slot, object);
        index = index.wrapping_add(1);
    }
}

// Translated from 00c5f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::RegisterStreamables` (Xbox PDB, engine map): false when
/// `NiExtraData::RegisterStreamables` fails; otherwise each element's
/// `RegisterStreamables` (its vtable slot 0x54, no null test) and true.
pub fn bs_texture_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if e.call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        == 0
    {
        return false;
    }
    let me = this.addr();
    let mut index = 0u32;
    while index < e.mem.u32(me.wrapping_add(COUNT)) {
        let elements = e.mem.u32(me.wrapping_add(ELEMENTS));
        let object = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        e.vcall(object, 0x54, &args![stream]);
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c5f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::SaveBinary` (Xbox PDB, engine map): `NiExtraData::SaveBinary`, the count
/// (`NiStreamSaveBinary<unsigned int>` from a local copy), then each element's link (the
/// stream's vtable slot 0x30).
pub fn bs_texture_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let me = this.addr();
    let count = e.mem.u32(me.wrapping_add(COUNT));
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), count);
        e.call(STREAM_SAVE_WORD, &args![stream, local]);
    });
    let mut index = 0u32;
    while index < e.mem.u32(me.wrapping_add(COUNT)) {
        let elements = e.mem.u32(me.wrapping_add(ELEMENTS));
        let object = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        e.vcall(stream.addr(), 0x30, &args![object]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00c5f100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::IsEqual` (Xbox PDB, engine map): false for no `other` or another count;
/// otherwise whether every element pointer is the same (the base `IsEqual` is not called).
pub fn bs_texture_data_is_equal(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if other == 0 {
        return false;
    }
    let me = this.addr();
    if e.mem.u32(me.wrapping_add(COUNT)) != e.mem.u32(other.wrapping_add(COUNT)) {
        return false;
    }
    let mut index = 0u32;
    while index < e.mem.u32(me.wrapping_add(COUNT)) {
        let mine = e.mem.u32(me.wrapping_add(ELEMENTS));
        let theirs = e.mem.u32(other.wrapping_add(ELEMENTS));
        let offset = index.wrapping_mul(4);
        if e.mem.u32(mine.wrapping_add(offset)) != e.mem.u32(theirs.wrapping_add(offset)) {
            return false;
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00c5f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTextureData::GetViewerStrings` (Xbox PDB, engine map): `NiExtraData::GetViewerStrings`,
/// the class name, then `"ObjectName"` with each element's name (its +8), each added to the
/// list.
pub fn bs_texture_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let me = this.addr();
    e.with_stack(8, |e, slots| {
        let name = e.global::<u32>(RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        let slot = slots.addr().wrapping_add(4);
        e.mem.set_u32(slot, text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
        let mut index = 0u32;
        while index < e.mem.u32(me.wrapping_add(COUNT)) {
            let elements = e.mem.u32(me.wrapping_add(ELEMENTS));
            let object = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
            let object_name = e.mem.u32(object.wrapping_add(8));
            let text = e
                .call(
                    NI_GET_VIEWER_STRING_TEXT,
                    &args![OBJECT_NAME_LABEL, object_name],
                )
                .u32();
            e.mem.set_u32(slots.addr(), text);
            e.call(VIEWER_STRINGS_ADD, &args![strings, slots]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c5f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the pointer array, `(count, shrink)`): sets the
/// array's size. A count of 0 clears it (`008454f0` with `shrink`). Above the capacity the
/// storage is made (vtable slot 4 with the count, when there is none) or grown (`0042fc00`
/// with the count and the size) to the count. Below the size, with `shrink`, the storage is
/// cut to the count (`0042fc00`) when the count is at most a quarter of the capacity. The
/// element loops of the code (construct and destroy of pointer elements) only compute
/// addresses.
pub fn fn_00c5f390(e: &mut Engine, this: u32, count: u32, shrink: u8) {
    if count == 0 {
        e.call(ARRAY_CLEAR, &args![this, u32::from(shrink)]);
        return;
    }
    let capacity = e.mem.u32(this.wrapping_add(ARRAY_CAPACITY));
    if count > capacity {
        if capacity == 0 {
            let storage = e.vcall(this, 4, &args![count]).u32();
            e.mem.set_u32(this.wrapping_add(ARRAY_ELEMENTS), storage);
            e.mem.set_u32(this.wrapping_add(ARRAY_CAPACITY), count);
        } else {
            let size = e.mem.u32(this.wrapping_add(ARRAY_SIZE));
            e.call(ARRAY_REALLOCATE, &args![this, count, size]);
            e.mem.set_u32(this.wrapping_add(ARRAY_CAPACITY), count);
        }
        let mut index = 0u32;
        while index < count.wrapping_sub(e.mem.u32(this.wrapping_add(ARRAY_SIZE))) {
            index = index.wrapping_add(1);
        }
        e.mem.set_u32(this.wrapping_add(ARRAY_SIZE), count);
    } else if count < e.mem.u32(this.wrapping_add(ARRAY_SIZE)) {
        let mut index = 0u32;
        while index < e.mem.u32(this.wrapping_add(ARRAY_SIZE)).wrapping_sub(count) {
            index = index.wrapping_add(1);
        }
        e.mem.set_u32(this.wrapping_add(ARRAY_SIZE), count);
        if shrink != 0 && count <= e.mem.u32(this.wrapping_add(ARRAY_CAPACITY)) >> 2 {
            e.call(ARRAY_REALLOCATE, &args![this, count, count]);
            e.mem.set_u32(this.wrapping_add(ARRAY_CAPACITY), count);
        }
    } else {
        let mut index = 0u32;
        while index < count.wrapping_sub(e.mem.u32(this.wrapping_add(ARRAY_SIZE))) {
            index = index.wrapping_add(1);
        }
        e.mem.set_u32(this.wrapping_add(ARRAY_SIZE), count);
    }
}

// Translated from 00c5f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the pointer array, `(destination)`): sizes the
/// destination to this array's size (`00c5f390` with shrinking allowed) and copies the
/// element pointers.
pub fn fn_00c5f500(e: &mut Engine, this: u32, destination: u32) {
    let size = e.mem.u32(this.wrapping_add(ARRAY_SIZE));
    fn_00c5f390(e, destination, size, 1);
    let mut index = 0u32;
    while index < e.mem.u32(this.wrapping_add(ARRAY_SIZE)) {
        let source = e.mem.u32(this.wrapping_add(ARRAY_ELEMENTS));
        let target = e.mem.u32(destination.wrapping_add(ARRAY_ELEMENTS));
        let offset = index.wrapping_mul(4);
        let value = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(target.wrapping_add(offset), value);
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_ecf0, fn_00c5ecf0(Ptr) -> Ptr),
        entry!(0x00c5_ed60, fn_00c5ed60(Ptr)),
        entry!(0x00c5_ee00, bs_texture_data_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00c5_ee90, fn_00c5ee90(Ptr, Ptr, u32)),
        entry!(0x00c5_eed0, fn_00c5eed0() -> Ptr),
        entry!(0x00c5_ef50, bs_texture_data_load_binary(Ptr, Ptr)),
        entry!(0x00c5_efb0, bs_texture_data_link_object(Ptr, Ptr)),
        entry!(0x00c5_f010, bs_texture_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c5_f080, bs_texture_data_save_binary(Ptr, Ptr)),
        entry!(0x00c5_f100, bs_texture_data_is_equal(Ptr, u32) -> bool),
        entry!(0x00c5_f190, bs_texture_data_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c5_f390, fn_00c5f390(u32, u32, u8)),
        entry!(0x00c5_f500, fn_00c5f500(u32, u32)),
        entry!(0x00c5_f560, fn_00c5f560(Ptr) -> u32),
        entry!(0x00c5_f360, bs_texture_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c5_f240, fn_00c5f240()),
        entry!(0x00c5_f2e0, fn_00c5f2e0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_texture_data_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c5_f360, 0x00c5_ed60, 0x1c);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5f560_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_f560, &args![0x1000u32]).u32(), 0x0120_4378);
        assert_eq!(e.call(0x00c5_f560, &args![0u32]).u32(), 0x0120_4378);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn name_initializer_and_destructor() {
        let mut e = Engine::new();
        e.map(TEXTURE_DATA_NAME, 4);
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
        let block = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_f240, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f240),
            vec![
                (ALLOCATE, vec![MEMORY_MANAGER, 4]),
                (FIXED_STRING_CREATE, vec![TEXTURE_DATA_NAME_TEXT]),
            ]
        );
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), block);
        assert_eq!(e.mem.u32(block), 0x8008);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f2e0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f2e0),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00c5_f240, &args![]);
        assert_eq!(e.mem.u32(TEXTURE_DATA_NAME), 0);
        e.mem.set_u32(block, 0x4444);
        e.mem.set_u32(TEXTURE_DATA_NAME, block);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f2e0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c5_f2e0),
            vec![(DEALLOCATE, vec![MEMORY_MANAGER, block])]
        );
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

    /// Texture data whose array holds `objects` (capacity 8).
    fn data(e: &mut Engine, objects: &[u32]) -> u32 {
        let this = e.mem.alloc(0x1c);
        let elements = e.mem.alloc(0x20);
        for (i, object) in (0u32..).zip(objects) {
            e.mem.set_u32(elements + 4 * i, *object);
        }
        e.mem.set_u32(this + ELEMENTS, elements);
        e.mem.set_u32(this + COUNT, objects.len() as u32);
        e.mem.set_u32(this + 0x18, 8);
        this
    }

    #[test]
    fn constructor_destructor_and_factory() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_CONSTRUCT, 0);
        logged(&mut e, &log, ARRAY_CONSTRUCT, 0);
        logged(&mut e, &log, ARRAY_DESTRUCT, 0);
        logged(&mut e, &log, EXTRA_DATA_DESTRUCT, 0);
        e.map(TEXTURE_DATA_NAME, 4);
        e.set_global::<u32>(TEXTURE_DATA_NAME, 0x4444);
        let block = e.mem.alloc(0x1c);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00c5_eed0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), BS_TEXTURE_DATA_VTABLE);
        assert_eq!(calls(&log, EXTRA_DATA_CONSTRUCT), vec![vec![block, 0x4444]]);
        assert_eq!(calls(&log, ARRAY_CONSTRUCT), vec![vec![block + 0xc]]);
        e.mem.set_u32(block, 0);
        e.call(0x00c5_ed60, &args![block]);
        assert_eq!(e.mem.u32(block), BS_TEXTURE_DATA_VTABLE);
        assert_eq!(calls(&log, ARRAY_DESTRUCT), vec![vec![block + 0xc]]);
        assert_eq!(calls(&log, EXTRA_DATA_DESTRUCT), vec![vec![block]]);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c5_eed0, &args![]).u32(), 0);
        assert_eq!(calls(&log, NI_ALLOCATE), vec![vec![0x1c], vec![0x1c]]);
    }

    #[test]
    fn create_clone_copies_the_element_pointers() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_CONSTRUCT, 0);
        logged(&mut e, &log, ARRAY_CONSTRUCT, 0);
        logged(&mut e, &log, EXTRA_DATA_COPY_MEMBERS, 0);
        e.map(TEXTURE_DATA_NAME, 4);
        let this = data(&mut e, &[0x11, 0x22, 0x33]);
        let block = data(&mut e, &[0x99]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00c5_ee00, &args![this, 0x5555u32]).u32(), block);
        assert_eq!(
            calls(&log, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, block, 0x5555]]
        );
        let elements = e.mem.u32(block + ELEMENTS);
        assert_eq!(e.mem.u32(block + COUNT), 3);
        assert_eq!(
            [
                e.mem.u32(elements),
                e.mem.u32(elements + 4),
                e.mem.u32(elements + 8)
            ],
            [0x11, 0x22, 0x33]
        );
    }

    #[test]
    fn load_binary_reads_the_count_and_sizes_the_array() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_LOAD_BINARY, 0);
        logged(&mut e, &log, NI_STREAM_READ_LINK_ID, 0);
        e.register(STREAM_READ_WORD, |e, a| {
            e.mem.set_u32(a[1], 3);
            Ret::default()
        });
        let this = data(&mut e, &[]);
        e.mem.set_u32(this + 0x18, 0);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0040_6600);
        e.mem.set_u32(this + ARRAY, vtable);
        let storage = e.mem.alloc(0xc);
        logged(&mut e, &log, 0x0040_6600, storage);
        e.call(0x00c5_ef50, &args![this, 0x6000u32]);
        assert_eq!(calls(&log, NI_STREAM_READ_LINK_ID).len(), 3);
        assert_eq!(calls(&log, 0x0040_6600), vec![vec![this + 0xc, 3]]);
        assert_eq!(e.mem.u32(this + ELEMENTS), storage);
        assert_eq!((e.mem.u32(this + COUNT), e.mem.u32(this + 0x18)), (3, 3));
    }

    #[test]
    fn resize_grows_shrinks_and_clears() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, ARRAY_CLEAR, 0);
        logged(&mut e, &log, ARRAY_REALLOCATE, 0);
        let array = data(&mut e, &[1, 2, 3, 4]) + ARRAY;
        // Growing within the capacity, then beyond it.
        e.call(0x00c5_f390, &args![array, 6u32, 0u32]);
        assert_eq!(e.mem.u32(array + ARRAY_SIZE), 6);
        assert!(calls(&log, ARRAY_REALLOCATE).is_empty());
        e.call(0x00c5_f390, &args![array, 10u32, 0u32]);
        assert_eq!(calls(&log, ARRAY_REALLOCATE), vec![vec![array, 10, 6]]);
        assert_eq!(
            (
                e.mem.u32(array + ARRAY_SIZE),
                e.mem.u32(array + ARRAY_CAPACITY)
            ),
            (10, 10)
        );
        // Shrinking: without the flag, or above a quarter of the capacity, the storage stays.
        e.call(0x00c5_f390, &args![array, 2u32, 0u32]);
        e.call(0x00c5_f390, &args![array, 1u32, 0u32]);
        e.mem.set_u32(array + ARRAY_SIZE, 4);
        e.call(0x00c5_f390, &args![array, 3u32, 1u32]);
        assert_eq!(calls(&log, ARRAY_REALLOCATE).len(), 1);
        e.call(0x00c5_f390, &args![array, 2u32, 1u32]);
        assert_eq!(calls(&log, ARRAY_REALLOCATE)[1], vec![array, 2, 2]);
        assert_eq!(
            (
                e.mem.u32(array + ARRAY_SIZE),
                e.mem.u32(array + ARRAY_CAPACITY)
            ),
            (2, 2)
        );
        e.call(0x00c5_f390, &args![array, 0u32, 1u32]);
        assert_eq!(calls(&log, ARRAY_CLEAR), vec![vec![array, 1]]);
    }

    #[test]
    fn link_object_fills_the_elements_in_order() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_LINK_OBJECT, 0);
        let mut next = 0x100u32;
        e.register_double(NI_STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next += 1;
            next.into_ret()
        });
        let this = data(&mut e, &[0, 0]);
        e.call(0x00c5_efb0, &args![this, 0x6000u32]);
        let elements = e.mem.u32(this + ELEMENTS);
        assert_eq!(
            [e.mem.u32(elements), e.mem.u32(elements + 4)],
            [0x101, 0x102]
        );
        assert_eq!(
            calls(&log, EXTRA_DATA_LINK_OBJECT),
            vec![vec![this, 0x6000]]
        );
    }

    fn object(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn register_streamables_registers_each_element_after_the_base() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_REGISTER_STREAMABLES, 0);
        logged(&mut e, &log, 0x0040_6700, 0);
        let first = object(&mut e, 0x54, 0x0040_6700);
        let second = object(&mut e, 0x54, 0x0040_6700);
        let this = data(&mut e, &[first, second]);
        assert!(!e.call(0x00c5_f010, &args![this, 0x6000u32]).bool());
        assert!(calls(&log, 0x0040_6700).is_empty());
        logged(&mut e, &log, EXTRA_DATA_REGISTER_STREAMABLES, 1);
        assert!(e.call(0x00c5_f010, &args![this, 0x6000u32]).bool());
        assert_eq!(
            calls(&log, 0x0040_6700),
            vec![vec![first, 0x6000], vec![second, 0x6000]]
        );
    }

    #[test]
    fn save_binary_writes_the_count_and_the_links() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_SAVE_BINARY, 0);
        e.register(STREAM_SAVE_WORD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 2);
            Ret::default()
        });
        logged(&mut e, &log, 0x0040_6800, 0);
        let stream = object(&mut e, 0x30, 0x0040_6800);
        let this = data(&mut e, &[0x11, 0x22]);
        e.call(0x00c5_f080, &args![this, stream]);
        assert_eq!(
            calls(&log, 0x0040_6800),
            vec![vec![stream, 0x11], vec![stream, 0x22]]
        );
    }

    #[test]
    fn is_equal_compares_the_element_pointers() {
        let mut e = Engine::new();
        let this = data(&mut e, &[1, 2]);
        let same = data(&mut e, &[1, 2]);
        let shorter = data(&mut e, &[1]);
        let different = data(&mut e, &[1, 3]);
        assert!(e.call(0x00c5_f100, &args![this, same]).bool());
        assert!(!e.call(0x00c5_f100, &args![this, shorter]).bool());
        assert!(!e.call(0x00c5_f100, &args![this, different]).bool());
        assert!(!e.call(0x00c5_f100, &args![this, 0u32]).bool());
    }

    #[test]
    fn viewer_strings_add_the_name_and_each_object_name() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, EXTRA_DATA_GET_VIEWER_STRINGS, 0);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0101_0000);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, a| (a[0] + a[1]).into_ret());
        let l = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            l.borrow_mut().push((0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, 0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x20);
        let this = data(&mut e, &[first, second]);
        e.call(0x00c5_f190, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&log, 0),
            vec![
                vec![0x6000, 0x0101_0001],
                vec![0x6000, OBJECT_NAME_LABEL + 0x10],
                vec![0x6000, OBJECT_NAME_LABEL + 0x20],
            ]
        );
    }
}
