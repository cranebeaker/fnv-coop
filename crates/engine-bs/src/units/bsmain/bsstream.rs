//! `BSMain/bsstream.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The two functions are the `NiTLargeArray<NiPointer<NiObject>>` instance the
//! stream's object array uses (`NiStream::m_kObjects`): elements are counted
//! references to objects whose reference count is at +4 and whose virtual
//! slot at +4 deletes them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nistream::NiTLargeArray;

/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00c1d790`: `cdecl(count)`, allocates the storage of `count` pointers (the
/// count in front of the elements, which are null).
const ARRAY_ALLOCATE: u32 = 0x00c1_d790;
/// `00a47600`: `cdecl(storage)`, releases every reference in the storage
/// `ARRAY_ALLOCATE` made and frees it.
const ARRAY_FREE_STORAGE: u32 = 0x00a4_7600;
/// The virtual slot of an object that deletes it when its count reaches zero.
const SLOT_DELETE_THIS: u32 = 0x04;

/// `InterlockedDecrement` of the count at +4; the delete slot runs at zero.
/// (The code tests for null first where the element can be null.)
fn release(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// `InterlockedIncrement` of the count at +4.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

// Translated from 00a64fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetSize`
/// (engine map; `thiscall`, one stack word): gives the array room for `new_max`
/// elements. Does nothing when that is the current capacity. When it is below
/// the used size, the elements from `new_max` on are released and cleared (the
/// element count `m_uiESize` goes down for each non-null one) and the used size
/// becomes `new_max`. Then a new storage block (`00c1d790`) takes the elements,
/// each one a new counted reference, the old storage is released and freed
/// (`00a47600`) and the capacity is `new_max` (the pointer is null for 0).
pub fn ni_t_large_array_set_size(e: &mut Engine, this: Ptr<NiTLargeArray>, new_max: u32) {
    if new_max == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    if new_max < e.get(this, NiTLargeArray::m_uiSize) {
        let mut index = new_max;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
                let present = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.set(this, NiTLargeArray::m_uiSize, new_max);
    }
    let old = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_max);
    if new_max == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let storage = e.call(ARRAY_ALLOCATE, &args![new_max]).u32();
        e.set(this, NiTLargeArray::m_pBase, storage);
        let mut index = 0u32;
        while index < e.get(this, NiTLargeArray::m_uiSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let current = e.mem.u32(slot);
            let carried = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            if current != carried {
                if current != 0 {
                    release(e, current);
                }
                e.mem.set_u32(slot, carried);
                if carried != 0 {
                    add_ref(e, carried);
                }
            }
            index = index.wrapping_add(1);
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        while index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(4));
            let entry = e.mem.u32(slot);
            if entry != 0 {
                release(e, entry);
                e.mem.set_u32(slot, 0);
            }
            index = index.wrapping_add(1);
        }
    }
    e.call(ARRAY_FREE_STORAGE, &args![old]);
}

// Translated from 00cc1a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<NiPointer<NiObject>,NiTNewInterface<NiPointer<NiObject>>>::SetAt`
/// (engine map; `thiscall`, two stack words): stores the reference that
/// `value` points at into element `index`. Keeps the used size (`index + 1`
/// when the index is at or beyond it) and the non-null count (`m_uiESize`) right,
/// and, when the element changes, releases the old object and counts a new
/// reference to the new one. The capacity is not checked.
pub fn ni_t_large_array_set_at(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: Ptr) {
    let present = e.get(this, NiTLargeArray::m_uiESize);
    let wanted = e.mem.u32(value.addr());
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    if index < e.get(this, NiTLargeArray::m_uiSize) {
        let held = e.mem.u32(slot);
        if wanted == 0 {
            if held != 0 {
                e.set(this, NiTLargeArray::m_uiESize, present.wrapping_sub(1));
            }
        } else if held == 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    } else {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if wanted != 0 {
            e.set(this, NiTLargeArray::m_uiESize, present.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(4));
    let held = e.mem.u32(slot);
    if held != e.mem.u32(value.addr()) {
        if held != 0 {
            release(e, held);
        }
        let new = e.mem.u32(value.addr());
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

// Translated from 00c3b9e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043dc without reading the receiver. The map names this body `BSReference::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3b9e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43dc
}

// Translated from 00c3ba70 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043e4 without reading the receiver. The map names this body `BSNodeReferences::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3ba70(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43e4
}

/// `NiStream::RegisterLoader` (engine map, `00a64900`, cdecl `(name,
/// loader)`).
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// The class names and loader functions `BSStream::InitLoaders` registers,
/// in its order (the code's immediate operands: name string, loader).
const LOADERS: [(u32, u32); 25] = [
    (0x010c_1344, 0x00c6_4810),
    (0x010c_1330, 0x00c6_49f0),
    (0x010c_1318, 0x00c5_37f0),
    (0x010c_1310, 0x00c5_5010),
    (0x010c_1304, 0x00c4_3190),
    (0x010c_12e4, 0x00c6_4530),
    (0x010c_12d4, 0x00c4_6740),
    (0x010c_12b8, 0x00c6_3880),
    (0x010c_12a4, 0x00c5_4210),
    (0x010c_1290, 0x00c6_1e70),
    (0x010c_1284, 0x00c6_1510),
    (0x010c_1274, 0x00c4_b0c0),
    (0x010c_1268, 0x00c4_ac00),
    (0x010c_125c, 0x00c4_3a30),
    (0x010c_123c, 0x00c5_cbb0),
    (0x010c_1224, 0x00c5_e5a0),
    (0x010c_1010, 0x00c3_61a0),
    (0x010c_1210, 0x00c4_6df0),
    (0x010c_1048, 0x00c3_81a0),
    (0x010c_1020, 0x00c3_6500),
    (0x010c_11fc, 0x00c6_10a0),
    (0x010c_11e8, 0x00c4_fdb0),
    (0x010c_11cc, 0x00c6_07d0),
    (0x010c_11bc, 0x00c4_ae60),
    (0x010c_11a0, 0x00c5_fee0),
];
/// `NiFile::GetFile` (engine map, `00aa1a20`, cdecl `(path, mode, size)`).
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;
/// `strcpy_s` (`00ec65a6`, cdecl) and the path normalizer `00aa1380`
/// (cdecl, one word).
const STRCPY_S: u32 = 0x00ec_65a6;
const NORMALIZE_PATH: u32 = 0x00aa_1380;
/// `NiStream::Load` on a binary stream (`00a63330`, engine map `Load_ov2`).
const NI_STREAM_LOAD_FROM_STREAM: u32 = 0x00a6_3330;
/// `NiStream`'s file name buffer (`+0xE0`, 0x104 bytes).
const FILE_NAME_OFFSET: u32 = 0xe0;
/// `NiStream` fields `LoadStream` uses: the file version (`+0xD8`), the
/// object array and its count (`+0x1F0`, `+0x1F4`), the load state
/// (`+0x28C`, 2 is failure), the three link indices (`+0x294`, `+0x298`,
/// `+0x29C`) and the reference map of `BSStream` (`+0x5C4`).
const VERSION_OFFSET: u32 = 0xd8;
const OBJECTS_OFFSET: u32 = 0x1f0;
const OBJECT_COUNT_OFFSET: u32 = 0x1f4;
const LOAD_STATE_OFFSET: u32 = 0x28c;
const LOAD_INDEX_OFFSET: u32 = 0x294;
const LINK_INDEX_OFFSET: u32 = 0x298;
const POST_LINK_INDEX_OFFSET: u32 = 0x29c;
const REFERENCE_MAP_OFFSET: u32 = 0x5c4;
/// `NiObjectNET::ms_RTTI` and the version strings `"5.0.0.1"` and
/// `"5.0.0.6"`.
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
const VERSION_5_0_0_1: u32 = 0x010c_1358;
const VERSION_5_0_0_6: u32 = 0x010c_1350;

// Translated from 00c3a6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::InitLoaders` (Xbox PDB, engine map): registers the 25 class
/// loaders of [`LOADERS`] with `NiStream::RegisterLoader`, in order.
pub fn bs_stream_init_loaders(e: &mut Engine) {
    for (name, loader) in LOADERS {
        e.call(REGISTER_LOADER, &args![name, loader]);
    }
}

// Translated from 00c3a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::Load` (Xbox PDB, engine map; vtable): opens `path` as a file
/// (`NiFile::GetFile` with mode 0 and 0x2800 bytes) when no `file` is given,
/// keeps the path (`strcpy_s` into `+0xE0`, then `00aa1380` on it), loads
/// from the file (`NiStream::Load`) and deletes the file (its vtable +0
/// with 1). Returns the load's result, false without a file.
pub fn bs_stream_load(e: &mut Engine, this: Ptr, path: u32, file: u32) -> u8 {
    let mut file = file;
    if file == 0 {
        file = e
            .call(NI_FILE_GET_FILE, &args![path, 0u32, 0x2800u32])
            .u32();
    }
    let mut loaded = 0u8;
    if file != 0 {
        let name = this.addr().wrapping_add(FILE_NAME_OFFSET);
        e.call(STRCPY_S, &args![name, 0x104u32, path]);
        e.call(NORMALIZE_PATH, &args![name]);
        loaded = e.call(NI_STREAM_LOAD_FROM_STREAM, &args![this, file]).u8();
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
    }
    loaded
}

/// The object at `index` of the stream's object array when it is an
/// `NiObjectNET` (its RTTI chain from vtable +8 reaches
/// `NiObjectNET::ms_RTTI`), else null; and the object itself.
fn object_net(e: &mut Engine, this: Ptr, index: u32) -> (u32, u32) {
    let objects = e.mem.u32(this.addr().wrapping_add(OBJECTS_OFFSET));
    let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
    if object == 0 {
        return (object, 0);
    }
    let mut rtti = e.vcall(object, 8, &[]).u32();
    let is_net = loop {
        if rtti == 0 {
            break false;
        }
        if rtti == NI_OBJECT_NET_RTTI {
            break true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    };
    (object, if is_net { object } else { 0 })
}

/// Whether the reference map (`+0x5C4`, `00853130`) already resolved the
/// named object (an `NiObjectNET` whose name is at `+8`).
fn resolved_by_map(e: &mut Engine, this: Ptr, net: u32) -> bool {
    if net == 0 {
        return false;
    }
    let name = e.mem.u32(net.wrapping_add(8));
    e.with_stack(4, |e, found| {
        let map = e.mem.u32(this.addr().wrapping_add(REFERENCE_MAP_OFFSET));
        if map != 0 && name != 0 {
            e.call(0x0085_3130, &args![map, name, found]);
        }
        e.mem.u32(found.addr()) != 0
    })
}

fn stream_field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_stream_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

// Translated from 00c3ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::LoadStream` (Xbox PDB, engine map; vtable): without a
/// reference map (`+0x5C4`) it is `NiStream::LoadStream` (`00a65c60`).
/// Otherwise: the header (vtable +0x34) must load; the objects are removed
/// (`NiStream::RemoveAllObjects`); from version 5.0.0.1 on the RTTI table
/// must load (`NiStream::LoadRTTI`), and from 5.0.0.6 on `00a647f0` runs.
/// Each object is then loaded (its vtable +0x4C with the stream from
/// 5.0.0.1 on, else the stream's +0x50 must succeed), the top objects read
/// (vtable +0x48), and each object linked (+0x50) and post-linked (+0x6C)
/// unless it is a named `NiObjectNET` the reference map resolves. A load
/// state of 2 stops with false. Ends with `00a63d10` and
/// `NiStream::FreeLoadData`; returns true.
pub fn bs_stream_load_stream(e: &mut Engine, this: Ptr) -> u8 {
    if stream_field(e, this, REFERENCE_MAP_OFFSET) == 0 {
        return e.call(0x00a6_5c60, &args![this]).u8();
    }
    if e.vcall(this.addr(), 0x34, &[]).u8() == 0 {
        return 0;
    }
    set_stream_field(e, this, POST_LINK_INDEX_OFFSET, 0);
    set_stream_field(e, this, LINK_INDEX_OFFSET, 0);
    set_stream_field(e, this, LOAD_INDEX_OFFSET, 0);
    e.call(0x00a6_44f0, &args![this]);
    let version = stream_field(e, this, VERSION_OFFSET);
    let rtti_version = e.call(0x00a6_3630, &args![VERSION_5_0_0_1]).u32();
    let newer = version >= rtti_version;
    if newer && e.call(0x00a6_5800, &args![this]).u8() == 0 {
        return 0;
    }
    let version = stream_field(e, this, VERSION_OFFSET);
    if version >= e.call(0x00a6_3630, &args![VERSION_5_0_0_6]).u32() {
        e.call(0x00a6_47f0, &args![this]);
    }
    let count = stream_field(e, this, OBJECT_COUNT_OFFSET);
    while stream_field(e, this, LOAD_INDEX_OFFSET) < count {
        if stream_field(e, this, LOAD_STATE_OFFSET) == 2 {
            return 0;
        }
        if newer {
            let index = stream_field(e, this, LOAD_INDEX_OFFSET);
            let objects = stream_field(e, this, OBJECTS_OFFSET);
            let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
            e.vcall(object, 0x4c, &args![this]);
        } else if e.vcall(this.addr(), 0x50, &[]).u8() == 0 {
            return 0;
        }
        let index = stream_field(e, this, LOAD_INDEX_OFFSET);
        set_stream_field(e, this, LOAD_INDEX_OFFSET, index.wrapping_add(1));
    }
    e.vcall(this.addr(), 0x48, &[]);
    for (index_offset, slot) in [(LINK_INDEX_OFFSET, 0x50), (POST_LINK_INDEX_OFFSET, 0x6c)] {
        while stream_field(e, this, index_offset) < count {
            if stream_field(e, this, LOAD_STATE_OFFSET) == 2 {
                return 0;
            }
            let index = stream_field(e, this, index_offset);
            let (object, net) = object_net(e, this, index);
            if !resolved_by_map(e, this, net) {
                e.vcall(object, slot, &args![this]);
            }
            let index = stream_field(e, this, index_offset);
            set_stream_field(e, this, index_offset, index.wrapping_add(1));
        }
    }
    e.call(0x00a6_3d10, &args![this]);
    e.call(0x00a6_4ca0, &args![this]);
    1
}

/// `NiStream::Save` (`00a63380`, `thiscall`, `(stream)`).
const NI_STREAM_SAVE: u32 = 0x00a6_3380;
/// `00006532c0` (`thiscall`, `(rtti)`): whether the object's RTTI chain (vtable +8, then
/// each parent at +4) reaches `rtti`.
const IS_KIND_OF: u32 = 0x0065_32c0;
/// `NiAVObject::ms_RTTI` and `NiTexturingProperty::ms_RTTI`.
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
const NI_TEXTURING_PROPERTY_RTTI: u32 = 0x011f_49a4;
/// The reference map's lookup (`00853130`, `thiscall`, `(name, &found)`, answers a byte),
/// insert (`00ec2f00`, `thiscall`, `(name, object)`) and removal (`00405430`, `thiscall`,
/// `(name)`).
const REFERENCE_MAP_LOOKUP: u32 = 0x0085_3130;
const REFERENCE_MAP_INSERT: u32 = 0x00ec_2f00;
const REFERENCE_MAP_REMOVE: u32 = 0x0040_5430;
/// `NiStream::RegisterSaveObject` (`00a663e0`, `thiscall`, `(object)`).
const NI_STREAM_REGISTER_SAVE_OBJECT: u32 = 0x00a6_63e0;
/// The stream's object-to-index map (`+0x274`): its lookup (`00a53c30`, `thiscall`,
/// `(key, &value)`, answers a byte) and its insert (`00b57ea0`, `thiscall`, `(key,
/// value)`).
const SAVE_OBJECT_MAP_OFFSET: u32 = 0x274;
const OBJECT_MAP_LOOKUP: u32 = 0x00a5_3c30;
const OBJECT_MAP_INSERT: u32 = 0x00b5_7ea0;
/// The stream's object array (`NiTLargeArray`, `+0x1EC`) and `BSStream`'s node references
/// (`+0x5C8`, a counted `BSNodeReferences` pointer).
const OBJECT_ARRAY_OFFSET: u32 = 0x1ec;
const NODE_REFERENCES_OFFSET: u32 = 0x5c8;
/// `00c64c40` (`thiscall` on the node references, `(stream, object)`).
const NODE_REFERENCES_ADD: u32 = 0x00c6_4c40;
/// `NiObject`'s constructor (`00a5d3a0`) and destructor body (`00a5d3d0`), `thiscall`.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `BSReference` (0xC bytes, vtable `010c1364`, name at +8) and `BSNodeReferences`
/// (0x18 bytes, vtable `010c13f4`, an `NiTArray` at +8 whose vtable is `01069d84` once
/// constructed and `01069d7c` in its destructor).
const BS_REFERENCE_VTABLE: u32 = 0x010c_1364;
const BS_REFERENCE_SIZE: u32 = 0xc;
const BS_NODE_REFERENCES_VTABLE: u32 = 0x010c_13f4;
const BS_NODE_REFERENCES_SIZE: u32 = 0x18;
const NODE_REFERENCES_ARRAY_VTABLE: u32 = 0x0106_9d84;
const NODE_REFERENCES_ARRAY_BASE_VTABLE: u32 = 0x0106_9d7c;
/// `BSReference::SetName` (`00c64780`, `thiscall`, `(name)`; unit `bsreference.obj`).
const BS_REFERENCE_SET_NAME: u32 = 0x00c6_4780;
/// The `NiTArray` constructor `0068ac40` (`thiscall`, `(max, grow_by)`).
const NI_T_ARRAY_CONSTRUCT: u32 = 0x0068_ac40;
/// `NiStream::InsertObject` (`00a66370`, `thiscall`, `(object)`) and `NiStream::SaveStream`
/// (`00a66850`, `thiscall`).
const NI_STREAM_INSERT_OBJECT: u32 = 0x00a6_6370;
const NI_STREAM_SAVE_STREAM: u32 = 0x00a6_6850;
/// `NiStream::RemoveAllObjects` (`00a644f0`) and `NiStream::FreeLoadData` (`00a64ca0`).
const NI_STREAM_REMOVE_ALL_OBJECTS: u32 = 0x00a6_44f0;
const NI_STREAM_FREE_LOAD_DATA: u32 = 0x00a6_4ca0;
/// `00aa13e0` (`cdecl`, one word): the Ni allocator; `00aa1460` (`cdecl`, `(block, size)`):
/// the sized delete; `00aa10f0` (`cdecl`, one word): the Ni free.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const NI_FREE: u32 = 0x00aa_10f0;
/// The global loader map `00c3b3c0` reads (`011f4784`, a pointer).
const LOADER_MAP: u32 = 0x011f_4784;
/// `strlen`, `strcat_s`, `strrchr` and `_strnicmp` (CRT, `cdecl`).
const STRLEN: u32 = 0x00ec_6130;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRRCHR: u32 = 0x00ec_6e30;
const STRNICMP: u32 = 0x00ec_7ec0;
/// `GetPrivateProfileStringA` (import slot).
const GET_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f164;
/// The strings `00c3b3f0` uses: `"\\"`, `"Loaders.ini"`, the section `"Loaders"`, and the
/// two defaults (`" "` and `""`, as the decompiler shows them).
const BACKSLASH_STRING: u32 = 0x0101_3444;
const LOADERS_INI: u32 = 0x010c_1488;
const LOADERS_SECTION: u32 = 0x010c_1480;
const KEYS_DEFAULT: u32 = 0x0102_0770;
const VALUE_DEFAULT: u32 = 0x0101_1584;
/// The global memory manager (`011f6238`) and its `Allocate` (`00aa3e40`, `thiscall`,
/// `(size)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00c3b5e0`'s table of name suffixes (`011af484`, null-terminated string pointers), the
/// palette lookup and store (`BSTexturePalette::GetTexture_ov2` `00a61b90`, `cdecl`,
/// `(path, &texture)`; `BSTexturePalette::SetTexture` `00a61c50`, `cdecl`, `(texture,
/// 0)`), `NiFile::Access` (`00aa1a30`, `cdecl`, `(path, mode)`), the fixed-string
/// constructor (`00a5b690`, `cdecl`, `(text)`) and its null value (`0109b220`),
/// `NiSourceTexture::Create` (`00a5fd70`, `cdecl`, four words; its format preferences at
/// `011a9598`) and the counted-pointer assignment `0066b0d0` (`thiscall`, `(object)`).
const TEXTURE_SUFFIXES: u32 = 0x011a_f484;
const PALETTE_GET_TEXTURE: u32 = 0x00a6_1b90;
const PALETTE_SET_TEXTURE: u32 = 0x00a6_1c50;
const NI_FILE_ACCESS: u32 = 0x00aa_1a30;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const FIXED_STRING_NULL: u32 = 0x0109_b220;
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_fd70;
const SOURCE_TEXTURE_PREFERENCES: u32 = 0x011a_9598;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The path prefix `00c3b7e0` puts in front of a relative name (`010724e8`) and the
/// four characters it compares the name's start with (`010a9470`).
const RELATIVE_PREFIX: u32 = 0x0107_24e8;
const PREFIX_TEST: u32 = 0x010a_9470;

/// Releases a counted reference held in a local (`InterlockedDecrement` of +4, the delete
/// slot at zero), when it is not null.
fn release_if_held(e: &mut Engine, object: u32) {
    if object != 0 {
        release(e, object);
    }
}

// Translated from 00c3a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::Save` (Xbox PDB, engine map; vtable): opens `path` for writing
/// (`NiFile::GetFile` with mode 1 and 0x2800 bytes); with a file, saves into it
/// (`NiStream::Save`) and deletes the file (its vtable +0 with 1). Returns the save's
/// result, false without a file.
pub fn bs_stream_save(e: &mut Engine, this: Ptr, path: u32) -> u8 {
    let file = e
        .call(NI_FILE_GET_FILE, &args![path, 1u32, 0x2800u32])
        .u32();
    let mut saved = 0u8;
    if file != 0 {
        saved = e.call(NI_STREAM_SAVE, &args![this, file]).u8();
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
    }
    saved
}

// Translated from 00c3a9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `(object, flags)`): with a reference map
/// (`+0x5C4`), walks `object` and records named objects in it. The object's name (+8, read
/// before any null test) is, with flag 0x80, added to the map (`00ec2f00`) unless the map
/// already has it (`00853130`), and without it removed (`00405430`). With any of flags 0x1F:
/// an `NiAVObject` gets, with flag 4, each non-null entry of its property list (+0x24, next
/// at +0, item at +8) walked, and when it is a node (vtable +0xC) and flag 1 is set, its
/// children (+0xA0, count +0xA6) walked from the last, each one only when flags 1 and 2 are
/// clear or the child is itself a node. Any other object, with flag 0x10, when it is an
/// `NiTexturingProperty`, gets the texture (+8) of its first map (+0x20, entry 0) walked.
pub fn fn_00c3a9c0(e: &mut Engine, this: Ptr, object: u32, flags: u32) {
    if stream_field(e, this, REFERENCE_MAP_OFFSET) == 0 {
        return;
    }
    let name = e.mem.u32(object.wrapping_add(8));
    if name != 0 {
        let map = stream_field(e, this, REFERENCE_MAP_OFFSET);
        if flags & 0x80 != 0 {
            let known = e.with_stack(4, |e, found| {
                e.call(REFERENCE_MAP_LOOKUP, &args![map, name, found]).u8()
            });
            if known == 0 {
                let map = stream_field(e, this, REFERENCE_MAP_OFFSET);
                e.call(REFERENCE_MAP_INSERT, &args![map, name, object]);
            }
        } else {
            e.call(REFERENCE_MAP_REMOVE, &args![map, name]);
        }
    }
    if flags & 0x1f == 0 {
        return;
    }
    let av_object = if object == 0 {
        0
    } else if e.call(IS_KIND_OF, &args![object, NI_AV_OBJECT_RTTI]).u8() != 0 {
        object
    } else {
        0
    };
    if av_object != 0 {
        if flags & 4 != 0 {
            let mut link = e.mem.u32(av_object.wrapping_add(0x24));
            while link != 0 {
                let item = e.mem.u32(link.wrapping_add(8));
                link = e.mem.u32(link);
                if item != 0 {
                    fn_00c3a9c0(e, this, item, flags);
                }
            }
        }
        let node = e.vcall(object, 0xc, &[]).u32();
        if node != 0 && flags & 1 != 0 {
            let mut count = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
            loop {
                let current = count;
                count = count.wrapping_sub(1);
                if current == 0 {
                    break;
                }
                let children = e.mem.u32(node.wrapping_add(0xa0));
                let child = e.mem.u32(children.wrapping_add(count.wrapping_mul(4)));
                if child != 0 && (flags & 3 == 0 || e.vcall(child, 0xc, &[]).u32() != 0) {
                    fn_00c3a9c0(e, this, child, flags);
                }
            }
        }
        return;
    }
    if flags & 0x10 == 0 {
        return;
    }
    let property = if object == 0 {
        0
    } else {
        let mut rtti = e.vcall(object, 8, &[]).u32();
        let is_texturing = loop {
            if rtti == 0 {
                break false;
            }
            if rtti == NI_TEXTURING_PROPERTY_RTTI {
                break true;
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        };
        if is_texturing {
            object
        } else {
            0
        }
    };
    if property == 0 {
        return;
    }
    let maps = e.mem.u32(property.wrapping_add(0x20));
    let map = e.mem.u32(maps);
    let texture = if map != 0 {
        e.mem.u32(map.wrapping_add(8))
    } else {
        0
    };
    if texture != 0 {
        fn_00c3a9c0(e, this, texture, flags);
    }
}

// Translated from 00c3afc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3afc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2db47, state variable) is not translated
/// `BSStream::RegisterSaveObject` (Xbox PDB, engine map; vtable): without a reference map
/// (`+0x5C4`) it is `NiStream::RegisterSaveObject`. Otherwise an object already in the
/// object map (`+0x274`, `00a53c30`) answers false. A named `NiObjectNET` whose name the
/// reference map holds is saved as a new `BSReference` with that name (the counted pointer
/// `00c3b970` keeps it), anything else is given to the node references (`+0x5C8`,
/// `00c64c40`). The object then maps to the next index (`+0x1F8`, `00b57ea0`) and the saved
/// object is stored at that index of the object array (grown by its grow-by when full).
/// Answers true.
pub fn bs_stream_register_save_object(e: &mut Engine, this: Ptr, object: u32) -> u8 {
    if stream_field(e, this, REFERENCE_MAP_OFFSET) == 0 {
        return e
            .call(NI_STREAM_REGISTER_SAVE_OBJECT, &args![this, object])
            .u8();
    }
    let saved_map = this.addr().wrapping_add(SAVE_OBJECT_MAP_OFFSET);
    let known = e.with_stack(4, |e, value| {
        e.call(OBJECT_MAP_LOOKUP, &args![saved_map, object, value])
            .u8()
    });
    if known != 0 {
        return 0;
    }
    e.with_stack(8, |e, locals| {
        // The `BSReference` pointer (first word) and the array's temporary (second word).
        let reference = locals.addr();
        let temporary = reference.wrapping_add(4);
        let held = e.mem.u32(reference);
        if held != 0 {
            add_ref(e, held);
        }
        let mut saved = object;
        let net = if object == 0 {
            0
        } else if e.call(IS_KIND_OF, &args![object, NI_OBJECT_NET_RTTI]).u8() != 0 {
            object
        } else {
            0
        };
        let mut found = 0;
        if net != 0 {
            let name = e.mem.u32(net.wrapping_add(8));
            found = e.with_stack(4, |e, out| {
                let map = stream_field(e, this, REFERENCE_MAP_OFFSET);
                if map != 0 && name != 0 {
                    e.call(REFERENCE_MAP_LOOKUP, &args![map, name, out]);
                }
                e.mem.u32(out.addr())
            });
        }
        if found != 0 {
            let block = e.call(NI_ALLOCATE, &args![BS_REFERENCE_SIZE]).u32();
            let created = if block == 0 {
                0
            } else {
                let name = e.mem.u32(net.wrapping_add(8));
                e.call(NI_OBJECT_CONSTRUCT, &args![block]);
                e.mem.set_u32(block, BS_REFERENCE_VTABLE);
                e.mem.set_u32(block.wrapping_add(8), 0);
                e.call(BS_REFERENCE_SET_NAME, &args![block, name]);
                block
            };
            ni_pointer_bs_reference_assign(e, Ptr::new(reference), created);
            saved = e.mem.u32(reference);
        } else {
            let references = stream_field(e, this, NODE_REFERENCES_OFFSET);
            e.call(NODE_REFERENCES_ADD, &args![references, this, object]);
        }
        let array = Ptr::<NiTLargeArray>::new(this.addr().wrapping_add(OBJECT_ARRAY_OFFSET));
        let index = e.get(array, NiTLargeArray::m_uiSize);
        e.call(OBJECT_MAP_INSERT, &args![saved_map, object, index]);
        e.mem.set_u32(temporary, saved);
        if saved != 0 {
            add_ref(e, saved);
        }
        let size = e.get(array, NiTLargeArray::m_uiSize);
        if size >= e.get(array, NiTLargeArray::m_uiMaxSize) {
            let grow_by = e.get(array, NiTLargeArray::m_uiGrowBy);
            ni_t_large_array_set_size(e, array, size.wrapping_add(grow_by));
        }
        ni_t_large_array_set_at(e, array, size, Ptr::new(temporary));
        let temporary_object = e.mem.u32(temporary);
        release_if_held(e, temporary_object);
        let held = e.mem.u32(reference);
        release_if_held(e, held);
    });
    1
}

// Translated from 00c3b270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3b270 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2db79, state variable) is not translated
/// `BSStream::SaveStream` (Xbox PDB, engine map; vtable): with a reference map (`+0x5C4`), a
/// new `BSNodeReferences` (0x18 bytes: `NiObject`, its vtable, an `NiTArray` at +8 made with
/// `0068ac40(0, 1)`) becomes the node references (`+0x5C8`) and is inserted as an object
/// (`NiStream::InsertObject`). Then `NiStream::SaveStream` runs, the node references are
/// cleared, and its result is returned.
pub fn bs_stream_save_stream(e: &mut Engine, this: Ptr) -> u8 {
    let references = Ptr::new(this.addr().wrapping_add(NODE_REFERENCES_OFFSET));
    if stream_field(e, this, REFERENCE_MAP_OFFSET) != 0 {
        let block = e.call(NI_ALLOCATE, &args![BS_NODE_REFERENCES_SIZE]).u32();
        let created = if block == 0 {
            0
        } else {
            e.call(NI_OBJECT_CONSTRUCT, &args![block]);
            e.mem.set_u32(block, BS_NODE_REFERENCES_VTABLE);
            let array = block.wrapping_add(8);
            e.call(NI_T_ARRAY_CONSTRUCT, &args![array, 0u32, 1u32]);
            e.mem.set_u32(array, NODE_REFERENCES_ARRAY_VTABLE);
            block
        };
        ni_pointer_bs_reference_assign(e, references, created);
        let current = stream_field(e, this, NODE_REFERENCES_OFFSET);
        e.call(NI_STREAM_INSERT_OBJECT, &args![this, current]);
    }
    let saved = e.call(NI_STREAM_SAVE_STREAM, &args![this]).u8();
    ni_pointer_bs_reference_assign(e, references, 0);
    saved
}

// Translated from 00c3b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStream::FreeAllObjects` (Xbox PDB, engine map): `NiStream::RemoveAllObjects`, the
/// words at +0x25C, +0x26C, +0x260 and +0x270 cleared (in that order), then
/// `NiStream::FreeLoadData`.
pub fn bs_stream_free_all_objects(e: &mut Engine, this: Ptr) {
    e.call(NI_STREAM_REMOVE_ALL_OBJECTS, &args![this]);
    for offset in [0x25c, 0x26c, 0x260, 0x270] {
        set_stream_field(e, this, offset, 0);
    }
    e.call(NI_STREAM_FREE_LOAD_DATA, &args![this]);
}

// Translated from 00c3b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the loader registered under `name` in
/// the global loader map (`011f4784`, its lookup `00a53c30`), or 0.
pub fn fn_00c3b3c0(e: &mut Engine, name: u32) -> u32 {
    e.with_stack(4, |e, value| {
        let map = e.global::<u32>(LOADER_MAP);
        e.call(OBJECT_MAP_LOOKUP, &args![map, name, value]);
        e.mem.u32(value.addr())
    })
}

// Translated from 00c3b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3b3f0 - the /GS stack-cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`cdecl`, one word): reads class loaders from an INI file.
/// The path is copied (0x104 bytes); unless its fourth-last character is `.`, a `\` (when
/// it does not end in one) and `Loaders.ini` are appended. Every key of the `Loaders`
/// section whose value names a known loader (`00c3b3c0`) is copied into a new block of the
/// memory manager and registered for that loader (`NiStream::RegisterLoader`).
pub fn fn_00c3b3f0(e: &mut Engine, path: u32) {
    // The frame from EBP-0x11A0: the value (0x80 bytes), the key list (0x1004 bytes from
    // -0x111C), the copied key's pointer (-0x114) and the path (0x104 bytes from -0x110).
    e.with_stack(0x11a0, |e, frame| {
        let value = frame.addr();
        let keys = value.wrapping_add(0x88);
        let copy = value.wrapping_add(0x108c);
        let file = value.wrapping_add(0x1090);
        e.call(STRCPY_S, &args![file, 0x104u32, path]);
        let length = e.call(STRLEN, &args![file]).u32();
        let has_extension =
            length >= 4 && e.mem.u8(file.wrapping_add(length).wrapping_sub(4)) == b'.';
        if !has_extension {
            if e.mem.u8(file.wrapping_add(length).wrapping_sub(1)) != b'\\' {
                e.call(STRCAT_S, &args![file, 0x104u32, BACKSLASH_STRING]);
            }
            e.call(STRCAT_S, &args![file, 0x104u32, LOADERS_INI]);
        }
        let read = e
            .call(
                GET_PRIVATE_PROFILE_STRING,
                &args![LOADERS_SECTION, 0u32, KEYS_DEFAULT, keys, 0x400u32, file],
            )
            .u32();
        e.mem.set_u32(copy, 0);
        if read == 0 {
            return;
        }
        let mut key = keys;
        while e.mem.u8(key) != 0 {
            let found = e
                .call(
                    GET_PRIVATE_PROFILE_STRING,
                    &args![LOADERS_SECTION, key, VALUE_DEFAULT, value, 0x80u32, file],
                )
                .u32();
            if found != 0 {
                let loader = fn_00c3b3c0(e, value);
                if loader != 0 {
                    let length = e.call(STRLEN, &args![key]).u32();
                    let block = e
                        .call(
                            MEMORY_MANAGER_ALLOCATE,
                            &args![MEMORY_MANAGER, length.wrapping_add(1)],
                        )
                        .u32();
                    e.mem.set_u32(copy, block);
                    e.call(STRCPY_S, &args![block, length.wrapping_add(1), key]);
                    e.call(REGISTER_LOADER, &args![block, loader]);
                }
            }
            let length = e.call(STRLEN, &args![key]).u32();
            key = key.wrapping_add(length).wrapping_add(1);
        }
    });
}

// Translated from 00c3b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3b5e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2dbaa, state variable) and the /GS stack-cookie check (00ec408c) are not translated
/// Unnamed in the engine map, no callers (`thiscall`, one stack word; `this` is not read):
/// for each suffix of the null-terminated table at `011af484`, builds the variant of `path`
/// with it (`00c3b7e0`); when the texture palette has no texture for it
/// (`BSTexturePalette::GetTexture_ov2`) and the file exists (`NiFile::Access` with 0), an
/// `NiSourceTexture` is created for it (`NiSourceTexture::Create` with the fixed-string name,
/// `011a9598`, 1, 0; kept through `0066b0d0`), the name's string released and the texture,
/// when made, stored in the palette (`BSTexturePalette::SetTexture` with 0). The texture
/// reference is released at the end of each suffix.
pub fn fn_00c3b5e0(e: &mut Engine, _this: Ptr, path: u32) {
    // The frame: the texture pointer (+0), the fixed string (+4), the path (+8, 0x104
    // bytes).
    e.with_stack(0x10c, |e, frame| {
        let texture = frame.addr();
        let name = texture.wrapping_add(4);
        let buffer = texture.wrapping_add(8);
        let mut entry = TEXTURE_SUFFIXES;
        while e.mem.u32(entry) != 0 {
            let suffix = e.mem.u32(entry);
            fn_00c3b7e0(e, buffer, path, suffix);
            e.mem.set_u32(texture, 0);
            let held = e.mem.u32(texture);
            if held != 0 {
                add_ref(e, held);
            }
            e.call(PALETTE_GET_TEXTURE, &args![buffer, texture]);
            if e.mem.u32(texture) == 0 && e.call(NI_FILE_ACCESS, &args![buffer, 0u32]).u8() != 0 {
                let fixed = if buffer == 0 {
                    e.global::<u32>(FIXED_STRING_NULL)
                } else {
                    e.call(FIXED_STRING_CREATE, &args![buffer]).u32()
                };
                e.mem.set_u32(name, fixed);
                let created = e
                    .call(
                        SOURCE_TEXTURE_CREATE,
                        &args![name, SOURCE_TEXTURE_PREFERENCES, 1u32, 0u32],
                    )
                    .u32();
                e.call(NI_POINTER_ASSIGN, &args![texture, created]);
                let fixed = e.mem.u32(name);
                if fixed != e.global::<u32>(FIXED_STRING_NULL) {
                    e.call(INTERLOCKED_DECREMENT, &args![fixed.wrapping_sub(8)]);
                }
                let made = e.mem.u32(texture);
                if made != 0 {
                    e.call(PALETTE_SET_TEXTURE, &args![made, 0u32]);
                }
            }
            let held = e.mem.u32(texture);
            release_if_held(e, held);
            entry = entry.wrapping_add(4);
        }
    });
}

// Translated from 00c3b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3b7e0 - the /GS stack-cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`cdecl`, `(out, path, suffix)`): `out` (0x104 bytes) is
/// emptied; when the copied `path` has a `.`, it is rebuilt with `suffix` before the
/// extension: the part from the last `.` (at most 10 bytes) is kept, the name is cut at the
/// last `_` (unless that comes before the last `\`) and at the `.`, and `out` becomes
/// [prefix `010724e8` when the name starts with neither `\` nor `?:` and not with the four
/// characters at `010a9470` (`_strnicmp`)] + name + suffix + extension.
pub fn fn_00c3b7e0(e: &mut Engine, out: u32, path: u32, suffix: u32) {
    // The frame: the copy (0x104 bytes) and the extension (10 bytes, at +0x118).
    e.with_stack(0x124, |e, frame| {
        let copy = frame.addr();
        let extension = copy.wrapping_add(0x118);
        e.call(STRCPY_S, &args![copy, 0x104u32, path]);
        e.mem.set_u8(out, 0);
        let dot = e.call(STRRCHR, &args![copy, u32::from(b'.')]).u32();
        let slash = e.call(STRRCHR, &args![copy, u32::from(b'\\')]).u32();
        let mut underscore = e.call(STRRCHR, &args![copy, u32::from(b'_')]).u32();
        if slash != 0 && underscore != 0 && underscore < slash {
            underscore = 0;
        }
        if dot == 0 {
            return;
        }
        e.call(STRCPY_S, &args![extension, 0xau32, dot]);
        if underscore != 0 {
            e.mem.set_u8(underscore, 0);
        }
        e.mem.set_u8(dot, 0);
        let relative = e.mem.u8(copy) != b'\\'
            && e.mem.u8(copy.wrapping_add(1)) != b':'
            && e.call(STRNICMP, &args![copy, PREFIX_TEST, 4u32]).u32() != 0;
        if relative {
            e.call(STRCPY_S, &args![out, 0x104u32, RELATIVE_PREFIX]);
            e.call(STRCAT_S, &args![out, 0x104u32, copy]);
        } else {
            e.call(STRCPY_S, &args![out, 0x104u32, copy]);
        }
        e.call(STRCAT_S, &args![out, 0x104u32, suffix]);
        e.call(STRCAT_S, &args![out, 0x104u32, extension]);
    });
}

// Translated from 00c3b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<BSReference>::operator=` (Xbox PDB, engine map; `thiscall`, `(object)`):
/// when the held object differs, releases it (the delete slot at a count of zero), holds
/// `object` and counts a reference to it. Returns `this`.
pub fn ni_pointer_bs_reference_assign(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    let held = e.mem.u32(this.addr());
    if held != object {
        if held != 0 {
            release(e, held);
        }
        e.mem.set_u32(this.addr(), object);
        if object != 0 {
            add_ref(e, object);
        }
    }
    this
}

// Translated from 00c3b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3b9f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2dbe8, state variable) is not translated
/// `BSReference::scalar deleting destructor` (Xbox PDB, engine map; vtable): the vtable,
/// the name cleared (`BSReference::SetName` with 0), `NiObject`'s destructor body, and the
/// sized delete of 0xC bytes with bit 0 of `flags`. Returns `this`.
pub fn bs_reference_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BS_REFERENCE_VTABLE);
    e.call(BS_REFERENCE_SET_NAME, &args![this, 0u32]);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, BS_REFERENCE_SIZE]);
    }
    this
}

// Translated from 00c3ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNodeReferences::scalar deleting destructor` (Xbox PDB, engine map; vtable): the
/// destructor (`00c3bab0`) and the sized delete of 0x18 bytes with bit 0 of `flags`.
/// Returns `this`.
pub fn bs_node_references_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c3bab0(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, BS_NODE_REFERENCES_SIZE]);
    }
    this
}

// Translated from 00c3bab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3bab0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2dc18, state variable) is not translated
/// Unnamed in the engine map: the `BSNodeReferences` destructor body. The vtable; the
/// array at +8 has its slots up to its size cleared, its size and element count zeroed,
/// its base vtable stored and its storage freed (`00aa10f0`); then `NiObject`'s destructor
/// body.
pub fn fn_00c3bab0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_NODE_REFERENCES_VTABLE);
    let array = Ptr::<NiTArray>::new(this.addr().wrapping_add(8));
    let mut index = 0u16;
    while index < e.get(array, NiTArray::m_usSize) {
        let base = e.get(array, NiTArray::m_pBase);
        e.mem.set_u32(base.wrapping_add(u32::from(index) * 4), 0);
        index = index.wrapping_add(1);
    }
    e.set(array, NiTArray::m_usSize, 0);
    e.set(array, NiTArray::m_usESize, 0);
    e.mem
        .set_u32(array.addr(), NODE_REFERENCES_ARRAY_BASE_VTABLE);
    let base = e.get(array, NiTArray::m_pBase);
    e.call(NI_FREE, &args![base]);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_a950, bs_stream_save(Ptr, u32) -> u8),
        entry!(0x00c3_a9c0, fn_00c3a9c0(Ptr, u32, u32)),
        entry!(0x00c3_afc0, bs_stream_register_save_object(Ptr, u32) -> u8),
        entry!(0x00c3_b270, bs_stream_save_stream(Ptr) -> u8),
        entry!(0x00c3_b370, bs_stream_free_all_objects(Ptr)),
        entry!(0x00c3_b3c0, fn_00c3b3c0(u32) -> u32),
        entry!(0x00c3_b3f0, fn_00c3b3f0(u32)),
        entry!(0x00c3_b5e0, fn_00c3b5e0(Ptr, u32)),
        entry!(0x00c3_b7e0, fn_00c3b7e0(u32, u32, u32)),
        entry!(0x00c3_b970, ni_pointer_bs_reference_assign(Ptr, u32) -> Ptr),
        entry!(
            0x00c3_b9f0,
            bs_reference_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00c3_ba80,
            bs_node_references_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c3_bab0, fn_00c3bab0(Ptr)),
        entry!(0x00c3_a6d0, bs_stream_init_loaders()),
        entry!(0x00c3_a8a0, bs_stream_load(Ptr, u32, u32) -> u8),
        entry!(0x00c3_ac10, bs_stream_load_stream(Ptr) -> u8),
        entry!(0x00c3_b9e0, fn_00c3b9e0(Ptr) -> u32),
        entry!(0x00c3_ba70, fn_00c3ba70(Ptr) -> u32),
        entry!(
            0x00a6_4fe0,
            ni_t_large_array_set_size(Ptr<NiTLargeArray>, u32)
        ),
        entry!(
            0x00cc_1a10,
            ni_t_large_array_set_at(Ptr<NiTLargeArray>, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETE_FUNCTION: u32 = 0x0040_0010;
    type Deleted = Rc<RefCell<Vec<u32>>>;

    struct Fixture {
        e: Engine,
        array: Ptr<NiTLargeArray>,
        vtable: u32,
        deleted: Deleted,
        freed: Rc<RefCell<Vec<u32>>>,
    }

    impl Fixture {
        fn new() -> Fixture {
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
            e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0] * 4 + 4).into_ret());
            let deleted: Deleted = Rc::default();
            let log = deleted.clone();
            e.register_double(DELETE_FUNCTION, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let freed: Rc<RefCell<Vec<u32>>> = Rc::default();
            let log = freed.clone();
            e.register_double(ARRAY_FREE_STORAGE, move |_, a| {
                log.borrow_mut().push(a[0]);
                Ret::default()
            });
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, DELETE_FUNCTION);
            let array = e.new_object::<NiTLargeArray>();
            Fixture {
                e,
                array,
                vtable,
                deleted,
                freed,
            }
        }

        fn object(&mut self, references: u32) -> u32 {
            let object = self.e.mem.alloc(8);
            self.e.mem.set_u32(object, self.vtable);
            self.e.mem.set_u32(object + 4, references);
            object
        }

        /// Gives the array storage for `max` elements holding `items`.
        fn fill(&mut self, max: u32, items: &[u32]) -> u32 {
            let base = self.e.mem.alloc(max * 4 + 4);
            for (i, item) in items.iter().enumerate() {
                self.e.mem.set_u32(base + i as u32 * 4, *item);
            }
            let array = self.array;
            self.e.set(array, NiTLargeArray::m_pBase, base);
            self.e.set(array, NiTLargeArray::m_uiMaxSize, max);
            self.e
                .set(array, NiTLargeArray::m_uiSize, items.len() as u32);
            let present = items.iter().filter(|item| **item != 0).count() as u32;
            self.e.set(array, NiTLargeArray::m_uiESize, present);
            base
        }

        fn state(&self) -> (u32, u32, u32) {
            (
                self.e.get(self.array, NiTLargeArray::m_uiMaxSize),
                self.e.get(self.array, NiTLargeArray::m_uiSize),
                self.e.get(self.array, NiTLargeArray::m_uiESize),
            )
        }

        fn element(&self, index: u32) -> u32 {
            let base = self.e.get(self.array, NiTLargeArray::m_pBase);
            self.e.mem.u32(base + index * 4)
        }

        fn references(&self, object: u32) -> u32 {
            self.e.mem.u32(object + 4)
        }
    }

    #[test]
    fn set_size_to_the_current_capacity_does_nothing() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let base = fx.fill(3, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 3u32]);
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), base);
        assert!(fx.freed.borrow().is_empty());
        assert_eq!(fx.references(a), 1);
    }

    #[test]
    fn set_size_grows_into_new_storage_and_counts_a_reference_per_element() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        let old = fx.fill(4, &[a, 0, b]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 8u32]);
        assert_eq!(fx.state(), (8, 3, 2));
        assert_eq!(*fx.freed.borrow(), vec![old]);
        assert_eq!([fx.element(0), fx.element(1), fx.element(2)], [a, 0, b]);
        assert_eq!(fx.element(5), 0);
        assert_eq!((fx.references(a), fx.references(b)), (2, 2));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_size_below_the_used_size_releases_the_cut_elements() {
        let mut fx = Fixture::new();
        let (a, b, c) = (fx.object(1), fx.object(2), fx.object(1));
        let old = fx.fill(3, &[a, b, c]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 1u32]);
        assert_eq!(fx.state(), (1, 1, 1));
        // b only loses the array's reference, c is deleted.
        assert_eq!((fx.references(b), fx.references(c)), (1, 0));
        assert_eq!(*fx.deleted.borrow(), vec![c]);
        // The cut slots of the old storage were cleared before the copy.
        assert_eq!(fx.e.mem.u32(old + 4), 0);
        assert_eq!(fx.e.mem.u32(old + 8), 0);
        assert_eq!(fx.element(0), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    #[test]
    fn set_size_to_zero_clears_the_pointer() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        let old = fx.fill(2, &[a]);
        fx.e.call(0x00a6_4fe0, &args![fx.array, 0u32]);
        assert_eq!(fx.state(), (0, 0, 0));
        assert_eq!(fx.e.get(fx.array, NiTLargeArray::m_pBase), 0);
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(*fx.freed.borrow(), vec![old]);
    }

    fn holder(fx: &mut Fixture, object: u32) -> u32 {
        let holder = fx.e.mem.alloc(4);
        fx.e.mem.set_u32(holder, object);
        holder
    }

    #[test]
    fn set_at_stores_a_new_reference_in_an_empty_slot() {
        let mut fx = Fixture::new();
        fx.fill(4, &[0, 0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 1u32, value]);
        assert_eq!(fx.element(1), a);
        assert_eq!(fx.references(a), 2);
        assert_eq!(fx.state(), (4, 2, 1));
    }

    #[test]
    fn set_at_beyond_the_used_size_extends_it() {
        let mut fx = Fixture::new();
        fx.fill(8, &[0]);
        let a = fx.object(1);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 5u32, value]);
        assert_eq!(fx.element(5), a);
        assert_eq!(fx.state(), (8, 6, 1));
        let nothing = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 7u32, nothing]);
        assert_eq!(fx.state(), (8, 8, 1));
    }

    #[test]
    fn set_at_replaces_and_releases_the_old_object() {
        let mut fx = Fixture::new();
        let (a, b) = (fx.object(1), fx.object(1));
        fx.fill(4, &[a]);
        let value = holder(&mut fx, b);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), b);
        assert_eq!((fx.references(a), fx.references(b)), (0, 2));
        assert_eq!(*fx.deleted.borrow(), vec![a]);
        assert_eq!(fx.state(), (4, 1, 1));
    }

    #[test]
    fn set_at_with_null_clears_the_slot_and_the_count() {
        let mut fx = Fixture::new();
        let a = fx.object(2);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, 0);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.element(0), 0);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 0));
        assert!(fx.deleted.borrow().is_empty());
    }

    #[test]
    fn set_at_with_the_same_object_changes_no_count() {
        let mut fx = Fixture::new();
        let a = fx.object(1);
        fx.fill(4, &[a]);
        let value = holder(&mut fx, a);
        fx.e.call(0x00cc_1a10, &args![fx.array, 0u32, value]);
        assert_eq!(fx.references(a), 1);
        assert_eq!(fx.state(), (4, 1, 1));
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c3b9e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_b9e0, &args![0x1000u32]).u32(), 0x0120_43dc);
        assert_eq!(e.call(0x00c3_b9e0, &args![0u32]).u32(), 0x0120_43dc);
    }

    #[test]
    fn fn_00c3ba70_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_ba70, &args![0x1000u32]).u32(), 0x0120_43e4);
        assert_eq!(e.call(0x00c3_ba70, &args![0u32]).u32(), 0x0120_43e4);
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object whose vtable slot `slot` points at `base + slot` (doubles
    /// returning 0 unless replaced).
    fn object(e: &mut Engine, base: u32) -> u32 {
        let table = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x600);
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn init_loaders_registers_every_class() {
        let mut e = Engine::new();
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_a6d0, &[]);
        let registered = calls_to(&e, REGISTER_LOADER);
        assert_eq!(registered.len(), 25);
        assert_eq!(registered[0], vec![0x010c_1344, 0x00c6_4810]);
        assert_eq!(registered[24], vec![0x010c_11a0, 0x00c5_fee0]);
    }

    #[test]
    fn load_opens_a_file_when_none_is_given_and_deletes_it() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x200);
        let file = object(&mut e, 0x7100_0000);
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(NORMALIZE_PATH, |_, _| Ret::default());
        e.register(NI_STREAM_LOAD_FROM_STREAM, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_a8a0, &args![stream, 0x5000u32, 0u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, NI_FILE_GET_FILE),
            vec![vec![0x5000, 0, 0x2800]]
        );
        assert_eq!(
            calls_to(&e, STRCPY_S),
            vec![vec![stream + 0xe0, 0x104, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, NI_STREAM_LOAD_FROM_STREAM),
            vec![vec![stream, file]]
        );
        assert_eq!(calls_to(&e, 0x7100_0000), vec![vec![file, 1]]);
        // A given file is used (and deleted) as it is; no file: false.
        e.call_log = Some(vec![]);
        e.call(0x00c3_a8a0, &args![stream, 0x5000u32, file]);
        assert!(calls_to(&e, NI_FILE_GET_FILE).is_empty());
        e.register(NI_FILE_GET_FILE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c3_a8a0, &args![stream, 0x5000u32, 0u32]).u8(), 0);
    }

    #[test]
    fn load_stream_without_a_map_is_the_base_load() {
        let mut e = Engine::new();
        let stream = object(&mut e, 0x7000_0000);
        e.register(0x00a6_5c60, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 1);
        // With a map, a header that fails to load: false.
        e.mem.set_u32(stream + 0x5c4, 0x4444);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
    }

    #[test]
    fn load_stream_loads_links_and_skips_mapped_objects() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let stream = object(&mut e, 0x7000_0000);
        e.register(0x7000_0034, |_, _| 1u32.into_ret());
        e.mem.set_u32(stream + 0x5c4, 0x4444);
        e.mem.set_u32(stream + 0xd8, 20);
        // Versions: 5.0.0.1 is 10 and 5.0.0.6 is 30.
        e.register(0x00a6_3630, |_, a| {
            if a[0] == VERSION_5_0_0_1 { 10u32 } else { 30 }.into_ret()
        });
        for target in [0x00a6_44f0, 0x00a6_47f0, 0x00a6_3d10, 0x00a6_4ca0] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x00a6_5800, |_, _| 1u32.into_ret());
        // Two objects: a plain one and an `NiObjectNET` named 0x99 that the
        // map resolves.
        let plain = object(&mut e, 0x7200_0000);
        let named = object(&mut e, 0x7300_0000);
        e.mem.set_u32(named + 8, 0x99);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent + 4, NI_OBJECT_NET_RTTI);
        e.register_double(0x7300_0008, move |_, _| parent.into_ret());
        e.register(0x0085_3130, |e, a| {
            e.mem.set_u32(a[2], u32::from(a[1] == 0x99));
            Ret::default()
        });
        let objects = e.mem.alloc(8);
        e.mem.set_u32(objects, plain);
        e.mem.set_u32(objects + 4, named);
        e.mem.set_u32(stream + 0x1f0, objects);
        e.mem.set_u32(stream + 0x1f4, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 1);
        assert_eq!(calls_to(&e, 0x7200_004c), vec![vec![plain, stream]]);
        assert_eq!(calls_to(&e, 0x7300_004c), vec![vec![named, stream]]);
        assert_eq!(calls_to(&e, 0x7200_0050), vec![vec![plain, stream]]);
        assert_eq!(calls_to(&e, 0x7200_006c), vec![vec![plain, stream]]);
        assert!(calls_to(&e, 0x7300_0050).is_empty());
        assert!(calls_to(&e, 0x7300_006c).is_empty());
        assert_eq!(calls_to(&e, 0x7000_0048).len(), 1);
        assert!(calls_to(&e, 0x00a6_47f0).is_empty());
        assert_eq!(e.mem.u32(stream + 0x294), 2);
        assert_eq!(e.mem.u32(stream + 0x29c), 2);
        // An old file loads each object through the stream (+0x50), which
        // may fail; a failed load state stops too.
        e.mem.set_u32(stream + 0xd8, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
        assert_eq!(calls_to(&e, 0x7000_0050).len(), 1);
        e.mem.set_u32(stream + 0xd8, 20);
        e.mem.set_u32(stream + 0x28c, 2);
        assert_eq!(e.call(0x00c3_ac10, &args![stream]).u8(), 0);
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double that records its arguments and answers `eax`.
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

    /// The interlocked imports as plain memory operations.
    fn counting(e: &mut Engine) {
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
    }

    /// A counted object (vtable with a logged delete slot) holding `count` references.
    fn counted(e: &mut Engine, log: &Log, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        let delete = 0x0040_7000 + vtable;
        e.mem.set_u32(vtable + 4, delete);
        logged(e, log, delete, 0);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn string(e: &mut Engine, address: u32, text: &[u8]) {
        e.map(address, text.len() as u32 + 1);
        e.mem.set_cstr(address, text);
    }

    /// `strrchr` as a double: the last occurrence of the character, or 0.
    fn last_occurrence(e: &mut Engine) {
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = text.iter().rposition(|c| u32::from(*c) == a[1]);
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
    }

    #[test]
    fn save_writes_through_a_file_and_deletes_it() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_FILE_GET_FILE, 0);
        logged(&mut e, &log, NI_STREAM_SAVE, 0x101);
        assert_eq!(e.call(0x00c3_a950, &args![0x5000u32, 0x6000u32]).u8(), 0);
        assert!(calls(&log, NI_STREAM_SAVE).is_empty());
        let vtable = e.mem.alloc(4);
        let file = e.mem.alloc(8);
        e.mem.set_u32(file, vtable);
        e.mem.set_u32(vtable, 0x0040_6000);
        logged(&mut e, &log, 0x0040_6000, 0);
        logged(&mut e, &log, NI_FILE_GET_FILE, file);
        assert_eq!(e.call(0x00c3_a950, &args![0x5000u32, 0x6000u32]).u8(), 1);
        assert_eq!(
            calls(&log, NI_FILE_GET_FILE),
            vec![vec![0x6000, 1, 0x2800], vec![0x6000, 1, 0x2800]]
        );
        assert_eq!(calls(&log, NI_STREAM_SAVE), vec![vec![0x5000, file]]);
        assert_eq!(calls(&log, 0x0040_6000), vec![vec![file, 1]]);
    }

    struct Walk {
        e: Engine,
        log: Log,
        stream: u32,
        map: u32,
        plain_vtable: u32,
        node_vtable: u32,
    }

    impl Walk {
        fn new() -> Walk {
            let mut e = Engine::new();
            let log = Log::default();
            let stream = e.mem.alloc(0x600);
            let map = e.mem.alloc(0x20);
            e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, map);
            logged(&mut e, &log, REFERENCE_MAP_INSERT, 0);
            logged(&mut e, &log, REFERENCE_MAP_REMOVE, 0);
            e.register(IS_KIND_OF, |_, _| 0u32.into_ret());
            // Slot 8 answers the word at +0x10, slot 0xC the object for a node.
            e.register(0x0040_8008, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
            e.register(0x0040_800c, |_, a| a[0].into_ret());
            e.register(0x0040_900c, |_, _| 0u32.into_ret());
            let node_vtable = e.mem.alloc(0x10);
            e.mem.set_u32(node_vtable + 8, 0x0040_8008);
            e.mem.set_u32(node_vtable + 0xc, 0x0040_800c);
            let plain_vtable = e.mem.alloc(0x10);
            e.mem.set_u32(plain_vtable + 8, 0x0040_8008);
            e.mem.set_u32(plain_vtable + 0xc, 0x0040_900c);
            Walk {
                e,
                log,
                stream,
                map,
                plain_vtable,
                node_vtable,
            }
        }

        /// `IsKindOf(NiAVObject)` holds for exactly these objects.
        fn av_objects(&mut self, objects: &[u32]) {
            let av: HashSet<u32> = objects.iter().copied().collect();
            self.e.register_double(IS_KIND_OF, move |_, a| {
                assert_eq!(a[1], NI_AV_OBJECT_RTTI);
                u32::from(av.contains(&a[0])).into_ret()
            });
        }

        fn object(&mut self, node: bool, name: u32) -> u32 {
            let object = self.e.mem.alloc(0xb0);
            let vtable = if node {
                self.node_vtable
            } else {
                self.plain_vtable
            };
            self.e.mem.set_u32(object, vtable);
            self.e.mem.set_u32(object + 8, name);
            object
        }

        /// The lookup answers `known` and records the names it is asked for.
        fn lookup(&mut self, known: u32) {
            logged(&mut self.e, &self.log, REFERENCE_MAP_LOOKUP, known);
        }

        fn looked_up(&self) -> Vec<u32> {
            calls(&self.log, REFERENCE_MAP_LOOKUP)
                .iter()
                .map(|c| c[1])
                .collect()
        }
    }

    #[test]
    fn walk_without_a_map_does_nothing_and_names_follow_flag_0x80() {
        let mut w = Walk::new();
        let object = w.object(false, 0x77);
        w.lookup(0);
        w.e.mem.set_u32(w.stream + REFERENCE_MAP_OFFSET, 0);
        w.e.call(0x00c3_a9c0, &args![w.stream, object, 0x80u32]);
        assert!(w.log.borrow().is_empty());
        w.e.mem.set_u32(w.stream + REFERENCE_MAP_OFFSET, w.map);
        // Unknown name with 0x80: inserted.
        w.e.call(0x00c3_a9c0, &args![w.stream, object, 0x80u32]);
        assert_eq!(
            calls(&w.log, REFERENCE_MAP_INSERT),
            vec![vec![w.map, 0x77, object]]
        );
        assert_eq!(calls(&w.log, REFERENCE_MAP_LOOKUP)[0][..2], [w.map, 0x77]);
        // Known name: not inserted again.
        w.lookup(1);
        w.e.call(0x00c3_a9c0, &args![w.stream, object, 0x80u32]);
        assert_eq!(calls(&w.log, REFERENCE_MAP_INSERT).len(), 1);
        // Without 0x80: removed.
        w.e.call(0x00c3_a9c0, &args![w.stream, object, 0u32]);
        assert_eq!(calls(&w.log, REFERENCE_MAP_REMOVE), vec![vec![w.map, 0x77]]);
    }

    #[test]
    fn walk_visits_properties_and_node_children_from_the_last() {
        let mut w = Walk::new();
        let root = w.object(true, 0x100);
        let item = w.object(false, 0x101);
        let first = w.object(true, 0x102);
        let second = w.object(false, 0x103);
        let link = w.e.mem.alloc(0xc);
        w.e.mem.set_u32(link + 8, item);
        let empty = w.e.mem.alloc(0xc);
        w.e.mem.set_u32(link, empty);
        w.e.mem.set_u32(root + 0x24, link);
        let children = w.e.mem.alloc(8);
        w.e.mem.set_u32(children, first);
        w.e.mem.set_u32(children + 4, second);
        w.e.mem.set_u32(root + 0xa0, children);
        w.e.mem.set_u16(root + 0xa6, 2);
        w.av_objects(&[root, first, second]);
        w.lookup(1);
        w.e.call(0x00c3_a9c0, &args![w.stream, root, 0x85u32]);
        // The second child is no node, so with flag 1 it is skipped.
        assert_eq!(w.looked_up(), vec![0x100, 0x101, 0x102]);
        // With flag 8 instead of 1: no children at all.
        w.log.borrow_mut().clear();
        w.e.call(0x00c3_a9c0, &args![w.stream, root, 0x8cu32]);
        assert_eq!(w.looked_up(), vec![0x100, 0x101]);
        // Flags without 0x1F: only the object itself.
        w.log.borrow_mut().clear();
        w.e.call(0x00c3_a9c0, &args![w.stream, root, 0x80u32]);
        assert_eq!(w.looked_up(), vec![0x100]);
    }

    #[test]
    fn walk_takes_every_child_without_flags_1_and_2_only_through_flag_1() {
        // Flag 1 is needed for the children and also restricts them to nodes; so every
        // child is walked only when the flag test on 3 fails, which flag 1 prevents. A
        // node's children are therefore always node-filtered: checked with two nodes.
        let mut w = Walk::new();
        let root = w.object(true, 0x300);
        let first = w.object(true, 0x301);
        let second = w.object(true, 0x302);
        let children = w.e.mem.alloc(8);
        w.e.mem.set_u32(children, first);
        w.e.mem.set_u32(children + 4, second);
        w.e.mem.set_u32(root + 0xa0, children);
        w.e.mem.set_u16(root + 0xa6, 2);
        w.av_objects(&[root, first, second]);
        w.lookup(1);
        w.e.call(0x00c3_a9c0, &args![w.stream, root, 0x81u32]);
        assert_eq!(w.looked_up(), vec![0x300, 0x302, 0x301]);
    }

    #[test]
    fn walk_follows_a_texturing_property_to_its_first_texture() {
        let mut w = Walk::new();
        w.lookup(1);
        let property = w.object(false, 0x200);
        let texture = w.object(false, 0x201);
        let rtti = w.e.mem.alloc(8);
        w.e.mem.set_u32(rtti + 4, NI_TEXTURING_PROPERTY_RTTI);
        w.e.mem.set_u32(property + 0x10, rtti);
        let map = w.e.mem.alloc(0xc);
        w.e.mem.set_u32(map + 8, texture);
        let maps = w.e.mem.alloc(4);
        w.e.mem.set_u32(maps, map);
        w.e.mem.set_u32(property + 0x20, maps);
        w.e.call(0x00c3_a9c0, &args![w.stream, property, 0x90u32]);
        assert_eq!(w.looked_up(), vec![0x200, 0x201]);
        // Without flag 0x10 nothing below it.
        w.log.borrow_mut().clear();
        w.e.call(0x00c3_a9c0, &args![w.stream, property, 0x81u32]);
        assert_eq!(w.looked_up(), vec![0x200]);
        // An RTTI chain that ends without the class: not followed.
        w.log.borrow_mut().clear();
        w.e.mem.set_u32(rtti + 4, 0);
        w.e.call(0x00c3_a9c0, &args![w.stream, property, 0x90u32]);
        assert_eq!(w.looked_up(), vec![0x200]);
    }

    fn save_stream_engine(log: &Log) -> (Engine, u32) {
        let mut e = Engine::new();
        counting(&mut e);
        let stream = e.mem.alloc(0x600);
        logged(&mut e, log, NI_STREAM_REGISTER_SAVE_OBJECT, 0x7);
        logged(&mut e, log, OBJECT_MAP_INSERT, 0);
        logged(&mut e, log, NODE_REFERENCES_ADD, 0);
        logged(&mut e, log, NI_OBJECT_CONSTRUCT, 0);
        logged(&mut e, log, BS_REFERENCE_SET_NAME, 0);
        logged(&mut e, log, ARRAY_FREE_STORAGE, 0);
        e.register(ARRAY_ALLOCATE, |e, a| e.mem.alloc(a[0] * 4 + 4).into_ret());
        (e, stream)
    }

    #[test]
    fn register_save_object_without_a_map_is_the_base_and_known_objects_fail() {
        let log = Log::default();
        let (mut e, stream) = save_stream_engine(&log);
        assert_eq!(e.call(0x00c3_afc0, &args![stream, 0x4444u32]).u8(), 7);
        assert_eq!(
            calls(&log, NI_STREAM_REGISTER_SAVE_OBJECT),
            vec![vec![stream, 0x4444]]
        );
        e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, 0x9000);
        logged(&mut e, &log, OBJECT_MAP_LOOKUP, 1);
        assert_eq!(e.call(0x00c3_afc0, &args![stream, 0x4444u32]).u8(), 0);
        assert_eq!(
            calls(&log, OBJECT_MAP_LOOKUP)[0][..2],
            [stream + 0x274, 0x4444]
        );
        assert!(calls(&log, OBJECT_MAP_INSERT).is_empty());
    }

    #[test]
    fn register_save_object_stores_plain_objects_and_grows_the_array() {
        let log = Log::default();
        let (mut e, stream) = save_stream_engine(&log);
        e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, 0x9000);
        e.mem.set_u32(stream + NODE_REFERENCES_OFFSET, 0x9100);
        logged(&mut e, &log, OBJECT_MAP_LOOKUP, 0);
        e.register(IS_KIND_OF, |_, _| 0u32.into_ret());
        let object = counted(&mut e, &log, 1);
        let array = stream + OBJECT_ARRAY_OFFSET;
        let base = e.mem.alloc(8);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u32(array + 8, 1);
        e.mem.set_u32(array + 0xc, 1);
        e.mem.set_u32(array + 0x14, 3);
        assert_eq!(e.call(0x00c3_afc0, &args![stream, object]).u8(), 1);
        assert_eq!(
            calls(&log, NODE_REFERENCES_ADD),
            vec![vec![0x9100, stream, object]]
        );
        assert_eq!(
            calls(&log, OBJECT_MAP_INSERT),
            vec![vec![stream + 0x274, object, 1]]
        );
        assert_eq!(e.mem.u32(array + 8), 4);
        assert_eq!(e.mem.u32(array + 0xc), 2);
        let storage = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(storage + 4), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(calls(&log, ARRAY_FREE_STORAGE), vec![vec![base]]);
    }

    #[test]
    fn register_save_object_replaces_a_mapped_name_with_a_reference() {
        let log = Log::default();
        let (mut e, stream) = save_stream_engine(&log);
        e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, 0x9000);
        e.register(OBJECT_MAP_LOOKUP, |_, _| 0u32.into_ret());
        e.register(IS_KIND_OF, |_, a| {
            assert_eq!(a[1], NI_OBJECT_NET_RTTI);
            1u32.into_ret()
        });
        e.register(REFERENCE_MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0x55);
            1u32.into_ret()
        });
        let object = counted(&mut e, &log, 1);
        e.mem.set_u32(object + 8, 0x3210);
        let block = e.mem.alloc(0xc);
        logged(&mut e, &log, NI_ALLOCATE, block);
        let array = stream + OBJECT_ARRAY_OFFSET;
        let base = e.mem.alloc(16);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u32(array + 8, 4);
        assert_eq!(e.call(0x00c3_afc0, &args![stream, object]).u8(), 1);
        assert_eq!(calls(&log, NI_ALLOCATE), vec![vec![0xc]]);
        assert_eq!(calls(&log, NI_OBJECT_CONSTRUCT), vec![vec![block]]);
        assert_eq!(
            calls(&log, BS_REFERENCE_SET_NAME),
            vec![vec![block, 0x3210]]
        );
        assert_eq!(e.mem.u32(block), BS_REFERENCE_VTABLE);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert!(calls(&log, NODE_REFERENCES_ADD).is_empty());
        assert_eq!(e.mem.u32(base), block);
        // Only the array keeps a reference to the new object.
        assert_eq!(e.mem.u32(block + 4), 1);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(
            calls(&log, OBJECT_MAP_INSERT),
            vec![vec![stream + 0x274, object, 0]]
        );
    }

    #[test]
    fn save_stream_makes_node_references_only_with_a_map() {
        let log = Log::default();
        let (mut e, stream) = save_stream_engine(&log);
        logged(&mut e, &log, NI_STREAM_SAVE_STREAM, 0x201);
        logged(&mut e, &log, NI_STREAM_INSERT_OBJECT, 0);
        logged(&mut e, &log, NI_T_ARRAY_CONSTRUCT, 0);
        assert_eq!(e.call(0x00c3_b270, &args![stream]).u8(), 1);
        assert!(calls(&log, NI_STREAM_INSERT_OBJECT).is_empty());
        e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, 0x9000);
        let block = e.mem.alloc(0x18);
        logged(&mut e, &log, NI_ALLOCATE, block);
        e.put_vtable(BS_NODE_REFERENCES_VTABLE, &[0, 0x0040_6100]);
        logged(&mut e, &log, 0x0040_6100, 0);
        assert_eq!(e.call(0x00c3_b270, &args![stream]).u8(), 1);
        assert_eq!(calls(&log, NI_ALLOCATE), vec![vec![0x18]]);
        assert_eq!(
            calls(&log, NI_T_ARRAY_CONSTRUCT),
            vec![vec![block + 8, 0, 1]]
        );
        assert_eq!(e.mem.u32(block + 8), NODE_REFERENCES_ARRAY_VTABLE);
        assert_eq!(
            calls(&log, NI_STREAM_INSERT_OBJECT),
            vec![vec![stream, block]]
        );
        // Cleared after the save: its only reference goes and it is deleted.
        assert_eq!(e.mem.u32(stream + NODE_REFERENCES_OFFSET), 0);
        assert_eq!(calls(&log, 0x0040_6100), vec![vec![block]]);
    }

    #[test]
    fn free_all_objects_clears_four_words_between_the_base_calls() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_STREAM_REMOVE_ALL_OBJECTS, 0);
        logged(&mut e, &log, NI_STREAM_FREE_LOAD_DATA, 0);
        let stream = e.mem.alloc(0x300);
        e.mem.write(stream + 0x258, &[0xff; 0x20]);
        e.call(0x00c3_b370, &args![stream]);
        for offset in [0x25c, 0x26c, 0x260, 0x270] {
            assert_eq!(e.mem.u32(stream + offset), 0);
        }
        assert_eq!(e.mem.u32(stream + 0x264), 0xffff_ffff);
        assert_eq!(
            *log.borrow(),
            vec![
                (NI_STREAM_REMOVE_ALL_OBJECTS, vec![stream]),
                (NI_STREAM_FREE_LOAD_DATA, vec![stream])
            ]
        );
    }

    #[test]
    fn loader_lookup_answers_the_mapped_loader_or_zero() {
        let mut e = Engine::new();
        e.map(LOADER_MAP, 4);
        e.set_global::<u32>(LOADER_MAP, 0x7700);
        e.register(OBJECT_MAP_LOOKUP, |e, a| {
            assert_eq!(a[0], 0x7700);
            if a[1] == 0x1234 {
                e.mem.set_u32(a[2], 0x00c0_0001);
                return 1u32.into_ret();
            }
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00c3_b3c0, &args![0x1234u32]).u32(), 0x00c0_0001);
        assert_eq!(e.call(0x00c3_b3c0, &args![0x1235u32]).u32(), 0);
    }

    fn words(text: &[u8]) -> Vec<u32> {
        text.iter().map(|b| u32::from(*b)).collect()
    }

    fn loader_ini_engine(log: &Log) -> Engine {
        let mut e = Engine::new();
        string(&mut e, BACKSLASH_STRING, b"\\");
        string(&mut e, LOADERS_INI, b"Loaders.ini");
        e.map(LOADER_MAP, 4);
        e.register(OBJECT_MAP_LOOKUP, |e, a| {
            if e.mem.cstr(a[1]) == b"LoaderA" {
                e.mem.set_u32(a[2], 0x00c0_00a0);
            }
            0u32.into_ret()
        });
        let l = log.clone();
        e.register_double(GET_PRIVATE_PROFILE_STRING, move |e, a| {
            let file = words(&e.mem.cstr(a[5]));
            l.borrow_mut().push((GET_PRIVATE_PROFILE_STRING, file));
            assert_eq!(a[0], LOADERS_SECTION);
            if a[1] == 0 {
                assert_eq!((a[2], a[4]), (KEYS_DEFAULT, 0x400));
                e.mem.write(a[3], b"a\0bb\0\0");
                return 5u32.into_ret();
            }
            assert_eq!((a[2], a[4]), (VALUE_DEFAULT, 0x80));
            if e.mem.cstr(a[1]) == b"a" {
                e.mem.set_cstr(a[3], b"LoaderA");
                return 7u32.into_ret();
            }
            e.mem.set_cstr(a[3], b"Other");
            5u32.into_ret()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            assert_eq!(a[0], MEMORY_MANAGER);
            e.mem.alloc(a[1]).into_ret()
        });
        let l = log.clone();
        e.register_double(REGISTER_LOADER, move |e, a| {
            let name = words(&e.mem.cstr(a[0]));
            l.borrow_mut().push((REGISTER_LOADER, vec![a[1]]));
            l.borrow_mut().push((0, name));
            Ret::default()
        });
        e
    }

    #[test]
    fn loaders_ini_registers_the_keys_with_known_loaders() {
        let log = Log::default();
        let mut e = loader_ini_engine(&log);
        let path = e.mem.alloc(0x20);
        e.mem.set_cstr(path, b"C:\\dir");
        e.call(0x00c3_b3f0, &args![path]);
        let files = calls(&log, GET_PRIVATE_PROFILE_STRING);
        assert_eq!(files.len(), 3);
        assert_eq!(files[0], words(b"C:\\dir\\Loaders.ini"));
        assert_eq!(calls(&log, REGISTER_LOADER), vec![vec![0x00c0_00a0]]);
        assert_eq!(calls(&log, 0), vec![words(b"a")]);
    }

    #[test]
    fn loaders_ini_keeps_a_file_name_and_adds_no_second_backslash() {
        let log = Log::default();
        let mut e = loader_ini_engine(&log);
        let path = e.mem.alloc(0x20);
        e.mem.set_cstr(path, b"C:\\my.ini");
        e.call(0x00c3_b3f0, &args![path]);
        assert_eq!(
            calls(&log, GET_PRIVATE_PROFILE_STRING)[0],
            words(b"C:\\my.ini")
        );
        log.borrow_mut().clear();
        e.mem.set_cstr(path, b"C:\\d\\");
        e.call(0x00c3_b3f0, &args![path]);
        assert_eq!(
            calls(&log, GET_PRIVATE_PROFILE_STRING)[0],
            words(b"C:\\d\\Loaders.ini")
        );
    }

    fn rebuild(e: &mut Engine, path: &[u8], suffix: &[u8]) -> Vec<u8> {
        let out = e.mem.alloc(0x104);
        e.mem.set_cstr(out, b"junk");
        let input = e.mem.alloc(0x40);
        e.mem.set_cstr(input, path);
        let tail = e.mem.alloc(0x10);
        e.mem.set_cstr(tail, suffix);
        e.call(0x00c3_b7e0, &args![out, input, tail]);
        e.mem.cstr(out)
    }

    #[test]
    fn suffixed_path_goes_before_the_extension() {
        let mut e = Engine::new();
        last_occurrence(&mut e);
        string(&mut e, RELATIVE_PREFIX, b"data\\");
        string(&mut e, PREFIX_TEST, b"data");
        assert_eq!(
            rebuild(&mut e, b"textures\\foo_n.dds", b"_g"),
            b"data\\textures\\foo_g.dds"
        );
        // A name already under the prefix, or absolute, gets none.
        assert_eq!(rebuild(&mut e, b"Data\\a.dds", b"_g"), b"Data\\a_g.dds");
        assert_eq!(
            rebuild(&mut e, b"c:\\a_b\\x.dds", b"_g"),
            b"c:\\a_b\\x_g.dds"
        );
        assert_eq!(rebuild(&mut e, b"\\x_y.tga", b"1"), b"\\x1.tga");
        // Without a dot the result stays empty.
        assert_eq!(rebuild(&mut e, b"noext", b"_g"), b"");
    }

    #[test]
    fn suffix_table_creates_missing_textures_that_exist() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e);
        last_occurrence(&mut e);
        string(&mut e, RELATIVE_PREFIX, b"d\\");
        string(&mut e, PREFIX_TEST, b"text");
        let first = e.mem.alloc(0x20);
        e.mem.set_cstr(first, b"_a");
        let second = e.mem.alloc(0x20);
        e.mem.set_cstr(second, b"_b");
        e.map(TEXTURE_SUFFIXES, 0xc);
        e.mem.set_u32(TEXTURE_SUFFIXES, first);
        e.mem.set_u32(TEXTURE_SUFFIXES + 4, second);
        e.map(FIXED_STRING_NULL, 4);
        let existing = counted(&mut e, &log, 1);
        let made = counted(&mut e, &log, 0);
        e.register_double(PALETTE_GET_TEXTURE, move |e, a| {
            if e.mem.cstr(a[0]) == b"textures\\x_b.dds" {
                e.mem.set_u32(a[1], existing);
                let count = e.mem.u32(existing + 4) + 1;
                e.mem.set_u32(existing + 4, count);
            }
            Ret::default()
        });
        e.register(NI_FILE_ACCESS, |_, _| 1u32.into_ret());
        let string_block = e.mem.alloc(0x20);
        e.mem.set_u32(string_block, 1);
        e.register_double(FIXED_STRING_CREATE, move |_, _| {
            (string_block + 8).into_ret()
        });
        logged(&mut e, &log, SOURCE_TEXTURE_CREATE, made);
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            let count = e.mem.u32(a[1] + 4) + 1;
            e.mem.set_u32(a[1] + 4, count);
            Ret::default()
        });
        logged(&mut e, &log, PALETTE_SET_TEXTURE, 0);
        let path = e.mem.alloc(0x20);
        e.mem.set_cstr(path, b"textures\\x.dds");
        e.call(0x00c3_b5e0, &args![0x1000u32, path]);
        let created = calls(&log, SOURCE_TEXTURE_CREATE);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [SOURCE_TEXTURE_PREFERENCES, 1, 0]);
        assert_eq!(e.mem.u32(string_block), 0);
        assert_eq!(calls(&log, PALETTE_SET_TEXTURE), vec![vec![made, 0]]);
        // The made texture's only reference was the local: released and deleted.
        assert_eq!(e.mem.u32(made + 4), 0);
        assert_eq!(e.mem.u32(existing + 4), 1);
    }

    #[test]
    fn reference_pointer_assignment_counts_references() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e);
        let a = counted(&mut e, &log, 1);
        let b = counted(&mut e, &log, 1);
        let slot = e.mem.alloc(4);
        assert_eq!(e.call(0x00c3_b970, &args![slot, a]).u32(), slot);
        assert_eq!((e.mem.u32(slot), e.mem.u32(a + 4)), (a, 2));
        e.call(0x00c3_b970, &args![slot, a]);
        assert_eq!(e.mem.u32(a + 4), 2);
        e.call(0x00c3_b970, &args![slot, b]);
        assert_eq!((e.mem.u32(a + 4), e.mem.u32(b + 4)), (1, 2));
        e.mem.set_u32(b + 4, 1);
        e.call(0x00c3_b970, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        let deletes: Vec<_> = log.borrow().iter().map(|c| c.1.clone()).collect();
        assert_eq!(deletes, vec![vec![b]]);
    }

    #[test]
    fn reference_destructor_clears_the_name_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BS_REFERENCE_SET_NAME, 0);
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x00c3_b9f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_REFERENCE_VTABLE);
        e.call(0x00c3_b9f0, &args![this, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (BS_REFERENCE_SET_NAME, vec![this, 0]),
                (NI_OBJECT_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0xc]),
                (BS_REFERENCE_SET_NAME, vec![this, 0]),
                (NI_OBJECT_DESTRUCT, vec![this]),
            ]
        );
    }

    #[test]
    fn node_references_destructor_empties_the_array() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_FREE, 0);
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let this = e.mem.alloc(0x18);
        let base = e.mem.alloc(0x10);
        e.mem.write(base, &[0x11; 0x10]);
        e.mem.set_u32(this + 0xc, base);
        e.mem.set_u16(this + 0x10, 4);
        e.mem.set_u16(this + 0x12, 3);
        e.mem.set_u16(this + 0x14, 2);
        assert_eq!(e.call(0x00c3_ba80, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_NODE_REFERENCES_VTABLE);
        assert_eq!(e.mem.u32(this + 8), NODE_REFERENCES_ARRAY_BASE_VTABLE);
        assert_eq!(e.mem.bytes(base, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(base + 0xc), 0x1111_1111);
        assert_eq!((e.mem.u16(this + 0x12), e.mem.u16(this + 0x14)), (0, 0));
        assert_eq!(
            *log.borrow(),
            vec![
                (NI_FREE, vec![base]),
                (NI_OBJECT_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0x18]),
            ]
        );
        log.borrow_mut().clear();
        e.call(0x00c3_ba80, &args![this, 0u32]);
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        assert_eq!(calls(&log, NI_FREE), vec![vec![base]]);
    }
}
