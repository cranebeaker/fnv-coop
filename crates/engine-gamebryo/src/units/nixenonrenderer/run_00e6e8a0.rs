//! `NiXenonRenderer/run_00e6e8a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTPointerMap};

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00e6e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e6e8a0`): a pointer list's item release, a virtual
/// `thiscall` with one stack word (the item, `RET 4`); `this` (the list) is not read. Clears
/// the item's element (+8), enters the item pool's lock (`0040fbf0` on `011c5f80` with 0),
/// clears the item's +4, pushes the item on the free list at `011c5f58` (its +0 takes the old
/// head) and leaves the lock: the count at `011c5f84` drops by one, and at zero the owner word
/// `011c5f80` is cleared.
pub fn fn_00e6e8a0(e: &mut Engine, _unused_this: Ptr, item: Ptr) {
    let item = item.addr();
    e.mem.set_u32(item.wrapping_add(8), 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(item.wrapping_add(4), 0);
    let head = e.mem.u32(NODE_POOL_FREE_LIST);
    e.mem.set_u32(item, head);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
}

/// `NiTMapBase::RemoveAll` (`00b601a0`, `thiscall`, no stack words): empties a map's buckets
/// through its vtable and zeroes the item count.
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// `NiFree` (`00aa10f0`, `cdecl`, one word).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, one word: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;

/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiVBBlock*, NiDX9Renderer::PrePackObject*>`'s
/// vtable (`010eda24`, RTTI) and `NiTPointerMap<NiVBBlock*, ...>`'s (`010edafc`).
const VB_BLOCK_MAP_BASE_VTABLE: u32 = 0x010e_da24;
const VB_BLOCK_MAP_VTABLE: u32 = 0x010e_dafc;
/// `NiTMapBase<..., HWND__*, NiPointer<NiRenderTargetGroup>>` (`010eda44`) and
/// `NiTPointerMap<HWND__*, ...>` (`010edb1c`), RTTI.
const WINDOW_MAP_BASE_VTABLE: u32 = 0x010e_da44;
const WINDOW_MAP_VTABLE: u32 = 0x010e_db1c;
/// `NiTMapBase<..., NiRenderedTexture*, NiDX9RenderedTextureData*>` (`010eda64`) and
/// `NiTPointerMap<NiRenderedTexture*, ...>` (`010edb3c`), RTTI.
const RENDERED_TEXTURE_MAP_BASE_VTABLE: u32 = 0x010e_da64;
const RENDERED_TEXTURE_MAP_VTABLE: u32 = 0x010e_db3c;
/// `NiTMapBase<..., NiRenderedCubeMap*, NiDX9RenderedCubeMapData*>` (`010eda84`) and
/// `NiTPointerMap<NiRenderedCubeMap*, ...>` (`010edb5c`), RTTI.
const RENDERED_CUBE_MAP_MAP_BASE_VTABLE: u32 = 0x010e_da84;
const RENDERED_CUBE_MAP_MAP_VTABLE: u32 = 0x010e_db5c;
/// `NiTMapBase<..., NiDynamicTexture*, NiDX9DynamicTextureData*>` (`010edaa4`) and
/// `NiTPointerMap<NiDynamicTexture*, ...>` (`010edb7c`), RTTI.
const DYNAMIC_TEXTURE_MAP_BASE_VTABLE: u32 = 0x010e_daa4;
const DYNAMIC_TEXTURE_MAP_VTABLE: u32 = 0x010e_db7c;
/// `NiTArray<void*, NiTMallocInterface<void*>>`'s vtable (`010edae4`, RTTI).
const MALLOC_POINTER_ARRAY_VTABLE: u32 = 0x010e_dae4;
/// `NiTMapBase<DFALL<NiTMapItem<_D3DFORMAT, NiPixelFormat*>>, _D3DFORMAT, NiPixelFormat*>`
/// (`010edac4`) and `NiTMap<_D3DFORMAT, NiPixelFormat*>` (`010edb9c`), RTTI.
const PIXEL_FORMAT_MAP_BASE_VTABLE: u32 = 0x010e_dac4;
const PIXEL_FORMAT_MAP_VTABLE: u32 = 0x010e_db9c;

/// The shared body of the map destructors below: each vtable of `vtables` in turn (most
/// derived first) is stored and the map emptied (`00b601a0`), then the bucket array (+8) goes
/// to `NiFree` and, when `flags & 1`, the map to the memory manager's free. Returns `this`.
fn destroy_map(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    vtables: &[u32],
    flags: u32,
) -> Ptr<NiTPointerMap> {
    for vtable in vtables {
        e.mem.set_u32(this.addr(), *vtable);
        e.call(MAP_REMOVE_ALL, &args![this]);
    }
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(NI_FREE, &args![table]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e6e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiVBBlock*, NiDX9Renderer::PrePackObject*>`
/// (its vtable `010eda24`, RTTI): empties the map, frees its buckets and, when `flags & 1`,
/// the map. Returns `this`.
pub fn fn_00e6e8f0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[VB_BLOCK_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTPointerMap<NiVBBlock*, NiDX9Renderer::PrePackObject*>` (vtable `010edafc`, RTTI):
/// empties the map as itself and again as its `NiTMapBase` (`010eda24`), frees the buckets
/// and, when `flags & 1`, the map. Returns `this`.
pub fn fn_00e6e930(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(
        e,
        this,
        &[VB_BLOCK_MAP_VTABLE, VB_BLOCK_MAP_BASE_VTABLE],
        flags,
    )
}

// Translated from 00e6e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, HWND__*, NiPointer<NiRenderTargetGroup>>`
/// (vtable `010eda44`, RTTI), as [`fn_00e6e8f0`].
pub fn fn_00e6e970(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[WINDOW_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTPointerMap<HWND__*, NiPointer<NiRenderTargetGroup>>` (vtable `010edb1c`, RTTI), as
/// [`fn_00e6e930`] with the base vtable `010eda44`.
pub fn fn_00e6e9b0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[WINDOW_MAP_VTABLE, WINDOW_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiRenderedTexture*, NiDX9RenderedTextureData*>`
/// (vtable `010eda64`, RTTI), as [`fn_00e6e8f0`].
pub fn fn_00e6e9f0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[RENDERED_TEXTURE_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTPointerMap<NiRenderedTexture*, NiDX9RenderedTextureData*>` (vtable `010edb3c`, RTTI), as
/// [`fn_00e6e930`] with the base vtable `010eda64`.
pub fn fn_00e6ea30(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(
        e,
        this,
        &[
            RENDERED_TEXTURE_MAP_VTABLE,
            RENDERED_TEXTURE_MAP_BASE_VTABLE,
        ],
        flags,
    )
}

// Translated from 00e6ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiRenderedCubeMap*, NiDX9RenderedCubeMapData*>`
/// (vtable `010eda84`, RTTI), as [`fn_00e6e8f0`].
pub fn fn_00e6ea70(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[RENDERED_CUBE_MAP_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6eab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTPointerMap<NiRenderedCubeMap*, NiDX9RenderedCubeMapData*>` (vtable `010edb5c`, RTTI), as
/// [`fn_00e6e930`] with the base vtable `010eda84`.
pub fn fn_00e6eab0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(
        e,
        this,
        &[
            RENDERED_CUBE_MAP_MAP_VTABLE,
            RENDERED_CUBE_MAP_MAP_BASE_VTABLE,
        ],
        flags,
    )
}

// Translated from 00e6eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiDynamicTexture*, NiDX9DynamicTextureData*>`
/// (vtable `010edaa4`, RTTI), as [`fn_00e6e8f0`].
pub fn fn_00e6eaf0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[DYNAMIC_TEXTURE_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTPointerMap<NiDynamicTexture*, NiDX9DynamicTextureData*>` (vtable `010edb7c`, RTTI), as
/// [`fn_00e6e930`] with the base vtable `010edaa4`.
pub fn fn_00e6eb30(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(
        e,
        this,
        &[DYNAMIC_TEXTURE_MAP_VTABLE, DYNAMIC_TEXTURE_MAP_BASE_VTABLE],
        flags,
    )
}

// Translated from 00e6eb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTArray<void*, NiTMallocInterface<void*>>` (vtable `010edae4`, RTTI): stores the vtable,
/// frees the elements (`m_pBase`, read before the store) with `NiFree` and, when `flags & 1`,
/// the array through the memory manager. Returns `this`.
pub fn fn_00e6eb70(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    let base = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), MALLOC_POINTER_ARRAY_VTABLE);
    e.call(NI_FREE, &args![base]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e6eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMapBase<DFALL<NiTMapItem<_D3DFORMAT, NiPixelFormat*>>, _D3DFORMAT, NiPixelFormat*>`
/// (vtable `010edac4`, RTTI), as [`fn_00e6e8f0`].
pub fn fn_00e6eba0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(e, this, &[PIXEL_FORMAT_MAP_BASE_VTABLE], flags)
}

// Translated from 00e6ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTMap<_D3DFORMAT, NiPixelFormat*>` (vtable `010edb9c`, RTTI), as [`fn_00e6e930`] with the
/// base vtable `010edac4`.
pub fn fn_00e6ebe0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    destroy_map(
        e,
        this,
        &[PIXEL_FORMAT_MAP_VTABLE, PIXEL_FORMAT_MAP_BASE_VTABLE],
        flags,
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_e8a0, fn_00e6e8a0(Ptr, Ptr)),
        entry!(
            0x00e6_e8f0,
            fn_00e6e8f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_e930,
            fn_00e6e930(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_e970,
            fn_00e6e970(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_e9b0,
            fn_00e6e9b0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_e9f0,
            fn_00e6e9f0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_ea30,
            fn_00e6ea30(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_ea70,
            fn_00e6ea70(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_eab0,
            fn_00e6eab0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_eaf0,
            fn_00e6eaf0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_eb30,
            fn_00e6eb30(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_eb70,
            fn_00e6eb70(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x00e6_eba0,
            fn_00e6eba0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e6_ebe0,
            fn_00e6ebe0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00e6_ec20, fn_00e6ec20()),
    ]
}

/// The COM object at `0126f0d8`, released through its vtable slot +8 (the object pushed).
const COM_OBJECT_0126F0D8: u32 = 0x0126_f0d8;
/// The 0x14-byte object at `0126f0e8`: destructor body `00e7bcd0` (`thiscall`, no stack
/// words), then the sized Ni free `00aa1460` (`cdecl`: block, size).
const OBJECT_0126F0E8: u32 = 0x0126_f0e8;
const OBJECT_DESTRUCT: u32 = 0x00e7_bcd0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The module handle at `0126f0d4`, given to the imported `FreeLibrary` (import slot
/// `00fdf0a8`) when not null; it is not cleared.
const MODULE_0126F0D4: u32 = 0x0126_f0d4;
const IMPORT_FREE_LIBRARY: u32 = 0x00fd_f0a8;
/// The word `0126f0dc`, cleared.
const CLEARED_0126F0DC: u32 = 0x0126_f0dc;
/// The pointer list at `0126f0f8` and its `RemoveAll` (engine map
/// `NiTPointerListBase<..., Script*>::RemoveAll`, `004ed900`, `thiscall`, no stack words),
/// reached by a jump.
const LIST_0126F0F8: u32 = 0x0126_f0f8;
const LIST_REMOVE_ALL: u32 = 0x004e_d900;

// Translated from 00e6ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; the last step of the shut-down function
/// `00e91c80`, reached by its jump): releases the COM object at `0126f0d8` (slot +8) and
/// clears the word, destroys and frees the 0x14-byte object at `0126f0e8` and clears the
/// word (both when not null), frees the library at `0126f0d4` when it is loaded, clears
/// `0126f0dc` and empties the list at `0126f0f8` (a jump to its `RemoveAll`).
pub fn fn_00e6ec20(e: &mut Engine) {
    let com = e.mem.u32(COM_OBJECT_0126F0D8);
    if com != 0 {
        e.vcall(com, 8, &args![]);
        e.mem.set_u32(COM_OBJECT_0126F0D8, 0);
    }
    let object = e.mem.u32(OBJECT_0126F0E8);
    if object != 0 {
        e.call(OBJECT_DESTRUCT, &args![object]);
        e.call(NI_DELETE, &args![object, 0x14u32]);
    }
    let module = e.mem.u32(MODULE_0126F0D4);
    e.mem.set_u32(OBJECT_0126F0E8, 0);
    if module != 0 {
        e.call(IMPORT_FREE_LIBRARY, &args![module]);
    }
    e.mem.set_u32(CLEARED_0126F0DC, 0);
    e.call(LIST_REMOVE_ALL, &args![LIST_0126F0F8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(count: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0e10_0000);
        e.mem.set_u32(NODE_POOL_LOCK, 7);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        // The lock enter double counts one more entry.
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e6e8a0_pushes_the_item_on_the_free_list() {
        let mut e = engine(0);
        let item = e.mem.alloc(0xc);
        e.mem.write(item, &[0xcc; 0xc]);
        e.call(0x00e6_e8a0, &args![0x0e20_0000u32, item]);
        assert_eq!(e.mem.u32(item), 0x0e10_0000);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), item);
        // The entry this call made is left again: count back to 0, owner cleared.
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        let log = e.call_log.take().unwrap();
        let locks: Vec<_> = log.iter().filter(|c| c.0 == LOCK_ENTER).collect();
        assert_eq!(locks, vec![&(LOCK_ENTER, vec![NODE_POOL_LOCK, 0])]);
    }

    #[test]
    fn fn_00e6e8a0_keeps_the_owner_of_a_lock_still_held() {
        let mut e = engine(1);
        let item = e.mem.alloc(0xc);
        e.call(0x00e6_e8a0, &args![0u32, item]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}

#[cfg(test)]
mod shut_down_tests {
    use super::*;

    const RELEASE: u32 = 0x0e50_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for callee in [
            RELEASE,
            OBJECT_DESTRUCT,
            NI_DELETE,
            IMPORT_FREE_LIBRARY,
            LIST_REMOVE_ALL,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.mem.set_u32(CLEARED_0126F0DC, 7);
        e
    }

    #[test]
    fn fn_00e6ec20_releases_what_is_held_then_empties_the_list() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, RELEASE);
        let com = e.mem.alloc(4);
        e.mem.set_u32(com, vtable);
        e.mem.set_u32(COM_OBJECT_0126F0D8, com);
        e.mem.set_u32(OBJECT_0126F0E8, 0x0e60_0000);
        e.mem.set_u32(MODULE_0126F0D4, 0x0e70_0000);
        e.call_log = Some(vec![]);
        e.call(0x00e6_ec20, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (RELEASE, vec![com]),
                (OBJECT_DESTRUCT, vec![0x0e60_0000]),
                (NI_DELETE, vec![0x0e60_0000, 0x14]),
                (IMPORT_FREE_LIBRARY, vec![0x0e70_0000]),
                (LIST_REMOVE_ALL, vec![LIST_0126F0F8]),
            ]
        );
        assert_eq!(e.mem.u32(COM_OBJECT_0126F0D8), 0);
        assert_eq!(e.mem.u32(OBJECT_0126F0E8), 0);
        assert_eq!(e.mem.u32(MODULE_0126F0D4), 0x0e70_0000);
        assert_eq!(e.mem.u32(CLEARED_0126F0DC), 0);
    }

    #[test]
    fn fn_00e6ec20_with_nothing_held_only_empties_the_list() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00e6_ec20, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(LIST_REMOVE_ALL, vec![LIST_0126F0F8])]
        );
        assert_eq!(e.mem.u32(CLEARED_0126F0DC), 0);
    }
}

#[cfg(test)]
mod map_destructor_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Runs the destructor at `address` on a map with buckets at `0x0e30_0000` and checks the
    /// vtable seen by each `RemoveAll`, the bucket free and the map free for both flag values.
    fn check(address: u32, vtables: &[u32]) {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let record = seen.clone();
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            e.register(NI_FREE, |_, _| Ret::default());
            e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
            let map = e.new_object::<NiTPointerMap>();
            e.set(map, NiTPointerMap::m_ppkHashTable, 0x0e30_0000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, flags]).u32(), map.addr());
            assert_eq!(seen.borrow().as_slice(), vtables);
            assert_eq!(e.mem.u32(map.addr()), *vtables.last().unwrap());
            let log = e.call_log.take().unwrap();
            let frees: Vec<_> = log.iter().filter(|c| c.0 == NI_FREE).collect();
            assert_eq!(frees, vec![&(NI_FREE, vec![0x0e30_0000])]);
            let deletes: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_MANAGER_FREE).collect();
            if flags == 1 {
                assert_eq!(
                    deletes,
                    vec![&(MEMORY_MANAGER_FREE, vec![MEMORY_MANAGER, map.addr()])]
                );
            } else {
                assert!(deletes.is_empty());
            }
        }
    }

    #[test]
    fn fn_00e6e8f0_destroys_the_vb_block_map_base() {
        check(0x00e6_e8f0, &[VB_BLOCK_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6e930_destroys_the_vb_block_map() {
        check(
            0x00e6_e930,
            &[VB_BLOCK_MAP_VTABLE, VB_BLOCK_MAP_BASE_VTABLE],
        );
    }

    #[test]
    fn fn_00e6e970_destroys_the_window_map_base() {
        check(0x00e6_e970, &[WINDOW_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6e9b0_destroys_the_window_map() {
        check(0x00e6_e9b0, &[WINDOW_MAP_VTABLE, WINDOW_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6e9f0_destroys_the_rendered_texture_map_base() {
        check(0x00e6_e9f0, &[RENDERED_TEXTURE_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6ea30_destroys_the_rendered_texture_map() {
        check(
            0x00e6_ea30,
            &[
                RENDERED_TEXTURE_MAP_VTABLE,
                RENDERED_TEXTURE_MAP_BASE_VTABLE,
            ],
        );
    }

    #[test]
    fn fn_00e6ea70_destroys_the_rendered_cube_map_map_base() {
        check(0x00e6_ea70, &[RENDERED_CUBE_MAP_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6eab0_destroys_the_rendered_cube_map_map() {
        check(
            0x00e6_eab0,
            &[
                RENDERED_CUBE_MAP_MAP_VTABLE,
                RENDERED_CUBE_MAP_MAP_BASE_VTABLE,
            ],
        );
    }

    #[test]
    fn fn_00e6eaf0_destroys_the_dynamic_texture_map_base() {
        check(0x00e6_eaf0, &[DYNAMIC_TEXTURE_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6eb30_destroys_the_dynamic_texture_map() {
        check(
            0x00e6_eb30,
            &[DYNAMIC_TEXTURE_MAP_VTABLE, DYNAMIC_TEXTURE_MAP_BASE_VTABLE],
        );
    }

    #[test]
    fn fn_00e6eba0_destroys_the_pixel_format_map_base() {
        check(0x00e6_eba0, &[PIXEL_FORMAT_MAP_BASE_VTABLE]);
    }

    #[test]
    fn fn_00e6ebe0_destroys_the_pixel_format_map() {
        check(
            0x00e6_ebe0,
            &[PIXEL_FORMAT_MAP_VTABLE, PIXEL_FORMAT_MAP_BASE_VTABLE],
        );
    }

    #[test]
    fn fn_00e6eb70_frees_the_elements_then_the_array_with_bit_zero() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(NI_FREE, |_, _| Ret::default());
            e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
            let array = e.new_object::<NiTArray>();
            e.set(array, NiTArray::m_pBase, 0x0e40_0000);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00e6_eb70, &args![array, flags]).u32(),
                array.addr()
            );
            assert_eq!(e.mem.u32(array.addr()), MALLOC_POINTER_ARRAY_VTABLE);
            let log = e.call_log.take().unwrap();
            let frees: Vec<_> = log.iter().filter(|c| c.0 == NI_FREE).collect();
            assert_eq!(frees, vec![&(NI_FREE, vec![0x0e40_0000])]);
            let deletes = log.iter().filter(|c| c.0 == MEMORY_MANAGER_FREE).count();
            assert_eq!(deletes, flags as usize);
        }
    }
}
