//! `fallout shared/tesmodeltextureswap.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048a470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00489040` (Ghidra name
/// `TESModel::InitializeDataComponent`), which tests `00559450` and clears the byte at
/// `+0x14`. The Xbox PDB name `TESModelTextureSwap::InitializeDataComponent` is the class's
/// vslot +0x000 (pdbdump `type ... TESModelTextureSwap`; the engine map row is `vt
/// 822daa78`), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00489040`'s disassembly ends on
/// the byte store `MOV byte ptr [EAX+0x14],0x0` with no result computed (Ghidra types it
/// void).
pub fn tes_model_texture_swap_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_9040, &args![this]);
}

// Translated from 0048aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::InitItem` (Xbox PDB): resolves the texture-set form ids of the swap list.
/// `owner` is the form that owns the model (it names the form in the error message).
///
/// It walks the list that `00500940` returns for `this` node by node (`00726070` is the next
/// node, `008256d0` tells whether a node ends the walk, `006815c0` gives the address of the
/// node's data pointer). Each entry holds an id; the entry gets the owner file's compile index
/// (`00484e60` with -1, `00485d50`), is looked up (`004839c0`) and cast with `__RTDynamicCast`
/// from the type at `0x01183028` to the type at `0x01187788`. An entry that is not found prints
/// `MASTERFILE: Failed to find texture set ...` (`005b5e40`), is taken out of the list (after a
/// previous good node: `00905330` on it, then the walk goes on with that node's next; at the
/// head: `0063f7b0` on the node, which stays the current node) and is freed (`00401030`).
pub fn tesmodeltextureswap_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut node = e.call(0x0050_0940, &args![this]).u32();
    let mut previous = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let data = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(data);
        let id_slot = e.mem.alloc(4);
        let entry_id = e.mem.u32(entry);
        e.mem.set_u32(id_slot, entry_id);
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![id_slot, file]);
        let id = e.mem.u32(id_slot);
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let found = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_7788u32, 0u32],
            )
            .u32();
        e.mem.set_u32(entry, found);
        if found == 0 {
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            let owner_name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(
                0x005b_5e40,
                &args![0x0101_d148u32, id, owner_name, owner_id],
            );
            if previous != 0 {
                let list_entry = e.mem.alloc(4);
                e.mem.set_u32(list_entry, entry);
                e.call(0x0090_5330, &args![previous, list_entry]);
                e.mem.free(list_entry);
                node = e.call(0x0072_6070, &args![previous]).u32();
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
            e.call(0x0040_1030, &args![entry]);
        } else {
            previous = node;
            node = e.call(0x0072_6070, &args![node]).u32();
        }
        e.mem.free(id_slot);
    }
}

// Translated from 0048a440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap` scalar deleting destructor (engine map name): runs the destructor body `0048a490` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_model_texture_swap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_a490, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0048b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<int_TEX_SWAP_P>` scalar deleting destructor (engine map name): runs the destructor body `0048b460` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_int_tex_swap_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_b460, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0048b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<int_TEX_SWAP_P>` scalar deleting destructor (engine map name): runs the destructor body `0048b4c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_int_tex_swap_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_b4c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The game's allocator (`00401000`, cdecl size) and its free (`00401030`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The swap list's first node (`00500940`, `__thiscall` on the model), a
/// node's data address, its next node and whether it ends the walk.
const SWAP_LIST: u32 = 0x0050_0940;
const NODE_DATA: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const NODE_IS_END: u32 = 0x0082_56d0;
/// Adds the word at the pointer argument to a list (`__thiscall`, `RET 4`).
const LIST_ADD: u32 = 0x005a_e3d0;
/// Removes the head entry of a list (`__thiscall`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The error printer (`005b5e40`, cdecl, format and arguments).
const PRINT_ERROR: u32 = 0x005b_5e40;
/// The word at `+0xc` (`0084e3a0`): a form's id, a map's item count.
const WORD_0C: u32 = 0x0084_e3a0;
/// The vtables of `BSMap<int,TEX_SWAP*>` and of its base
/// `BSMapBase<int,TEX_SWAP*>` (engine map names of their scalar deleting
/// destructors, `0048b3b0` and `0048b530`).
const MAP_VTABLE: u32 = 0x0101_d254;
const MAP_BASE_VTABLE: u32 = 0x0101_d274;
/// The texture-swap entry: the texture set (+0), the shape index (+4)
/// and the shape name (+8, 0x80 bytes).
const SWAP_ENTRY_SIZE: u32 = 0x88;

// Translated from 0048af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a texture-swap entry: no texture set, index -1, an empty
/// name. Returns `this` (`ECX`, no stack words). The engine map gives it no
/// name.
pub fn fn_0048af10(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0xffff_ffff);
    e.mem.set_u8(this.addr() + 8, 0);
    this
}

// Translated from 0048af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the word at `source` into `this` after `0043ba60(source)`
/// (modelloader.cpp, cdecl). Returns `this`. `__thiscall`, `RET 4`. The
/// engine map gives it no name.
pub fn fn_0048af40(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(0x0043_ba60, &args![source]);
    let value = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 0048af70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this` differs from the global at `0109b220` (`ECX`,
/// no stack words; the decompiler folds the global to 0). The engine map
/// gives it no name.
pub fn fn_0048af70(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) != e.global::<u32>(0x0109_b220)
}

// Translated from 0048b2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `texture_set` to the holder at `this + 0xa4` (`0066b0d0`; the
/// callers pass a lighting shader property). `__thiscall`, `RET 4`. The
/// engine map gives it no name.
pub fn fn_0048b2f0(e: &mut Engine, this: Ptr, texture_set: u32) {
    e.call(0x0066_b0d0, &args![this.addr() + 0xa4, texture_set]);
}

// Translated from 0048b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `item + 8` and frees `item` (`00401030`).
/// `__thiscall`, `RET 4` (`this` is not read). The engine map gives it no
/// name.
pub fn fn_0048b500(e: &mut Engine, _this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr() + 8, 0);
    e.call(FREE, &args![item]);
}

// Translated from 0048b3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase<int,TEX_SWAP*>` constructor: stores its vtable
/// (`0101d274`), the bucket count `size` at +4 and 0 items at +0xc, and for
/// a non-zero size allocates `4 * size` bytes of buckets (+8) and clears
/// them (`_memset`). Returns `this`. `__thiscall`, `RET 4`. The engine map
/// gives it no name.
pub fn fn_0048b3e0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let map = this.addr();
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.mem.set_u32(map + 4, size);
    e.mem.set_u32(map + 0xc, 0);
    if size != 0 {
        let bytes = e.mem.u32(map + 4) << 2;
        let buckets = e.call(ALLOCATE, &args![bytes]).u32();
        e.mem.set_u32(map + 8, buckets);
        let bytes = e.mem.u32(map + 4) << 2;
        let buckets = e.mem.u32(map + 8);
        e.call(0x00ec_61c0, &args![buckets, 0u32, bytes]);
    }
    this
}

// Translated from 0048b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMap<int,TEX_SWAP*>` constructor: the base constructor
/// (`fn_0048b3e0`) with `size`, then its own vtable (`0101d254`). Returns
/// `this`. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_0048b380(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_0048b3e0(e, this, size);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 0048b4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase<int,TEX_SWAP*>` destructor body: stores the base vtable
/// back and frees the buckets (+8) when there are any. `ECX`, no stack
/// words. The engine map gives it no name.
pub fn fn_0048b4c0(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    if e.mem.u32(map + 8) != 0 {
        let buckets = e.mem.u32(map + 8);
        e.call(FREE, &args![buckets]);
    }
}

// Translated from 0048b460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048b460 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `BSMap<int,TEX_SWAP*>` destructor body: stores its vtable back,
/// empties the map (`00438af0`) and runs the base destructor
/// (`fn_0048b4c0`). `ECX`, no stack words. The exception-unwinding frame is
/// not translated.
pub fn fn_0048b460(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(0x0043_8af0, &args![this]);
    fn_0048b4c0(e, this);
}

// Translated from 0048ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::ClearTexSwapList` (engine map): until the list at
/// `this + 0x18` is empty, removes its head entry (`0063f7b0`) and frees
/// it (`00401030`). `ECX`, no stack words.
pub fn tes_model_texture_swap_clear_tex_swap_list(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 0x18;
    while !e.call(NODE_IS_END, &args![list]).bool() {
        let data = e.call(NODE_DATA, &args![list]).u32();
        let entry = e.mem.u32(data);
        e.call(LIST_REMOVE_HEAD, &args![list]);
        e.call(FREE, &args![entry]);
    }
}

// Translated from 0048a500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::ClearDataComponent` (engine map): clears the swap
/// list (`ClearTexSwapList`), then `TESModel::ClearDataComponent`
/// (`004890e0`). `ECX`, no stack words.
pub fn tes_model_texture_swap_clear_data_component(e: &mut Engine, this: Ptr) {
    tes_model_texture_swap_clear_tex_swap_list(e, this);
    e.call(0x0048_90e0, &args![this]);
}

// Translated from 0048a490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a490 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The `TESModelTextureSwap` destructor body (the scalar deleting
/// destructor `0048a440` runs it): stores the vtable (`0101d124`) back,
/// clears the data (`ClearDataComponent`), destroys the list at +0x18
/// (`0046ffb0`) and runs the `TESModel` destructor (`00489070`). `ECX`, no
/// stack words. The exception-unwinding frame is not translated.
pub fn fn_0048a490(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_d124);
    tes_model_texture_swap_clear_data_component(e, this);
    e.call(0x0046_ffb0, &args![this.addr() + 0x18]);
    e.call(0x0048_9070, &args![this]);
}

// Translated from 0048ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::GetTexSwap` (engine map): the swap entry whose
/// shape index (+4) is `index`, or 0 (also for index -1). `__thiscall`,
/// `RET 4`.
pub fn tes_model_texture_swap_get_tex_swap(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    if index == -1 {
        return 0;
    }
    let mut node = e.call(SWAP_LIST, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let entry = e.mem.u32(data);
        if index == e.mem.i32(entry + 4) {
            let data = e.call(NODE_DATA, &args![node]).u32();
            return e.mem.u32(data);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 0048ae30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048ae30 - the exception-unwinding frame (FS:[0]) around the entry's construction and the security-cookie word are not translated.
/// `TESModelTextureSwap::AddTexSwap` (engine map): with a name and a
/// texture set, finds the entry of `index` (`GetTexSwap`) or makes one (a
/// 0x88-byte allocation, `fn_0048af10`, the name copied with `00406d30`
/// into its 0x80 bytes, the index stored, added to the list at +0x18), and
/// sets its texture set. `__thiscall`, `RET 0xc`. The exception-unwinding
/// frame is not translated.
pub fn tes_model_texture_swap_add_tex_swap(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    index: i32,
    texture_set: u32,
) {
    if name == 0 || texture_set == 0 {
        return;
    }
    let mut entry = tes_model_texture_swap_get_tex_swap(e, this, index);
    if entry == 0 {
        let block = e.call(ALLOCATE, &args![SWAP_ENTRY_SIZE]).u32();
        entry = if block != 0 {
            fn_0048af10(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.call(0x0040_6d30, &args![entry.wrapping_add(8), 0x80u32, name]);
        e.mem.set_i32(entry + 4, index);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), entry);
            e.call(LIST_ADD, &args![this.addr() + 0x18, slot]);
        });
    }
    e.mem.set_u32(entry, texture_set);
}

/// Takes `n` bytes from the chunk cursor (`position`, `remaining`), or
/// `None` when fewer remain.
fn take(position: &mut u32, remaining: &mut u32, n: u32) -> Option<u32> {
    if n > *remaining {
        return None;
    }
    let at = *position;
    *position = position.wrapping_add(n);
    *remaining -= n;
    Some(at)
}

// Translated from 0048a7b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a7b0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (engine map): reads a
/// count and then for each swap a name length, the name (copied with
/// `00401460` into a 0x104-byte buffer, at most 0x103 bytes), a texture
/// set id and, for `version` 0xf and later, a shape index (-1 otherwise);
/// a swap with a name and a texture set is added (`AddTexSwap`). Answers
/// false when the data runs out or a name is too long, true otherwise
/// (also for no data or 4 bytes or less). `__thiscall`, `RET 0xc`. The
/// stack cookie check is not translated.
pub fn tes_model_texture_swap_unpack_texture_swap_chunk_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    size: u32,
    version: u32,
) -> bool {
    if data == 0 || size <= 4 {
        return true;
    }
    let mut count = e.mem.u32(data);
    let mut position = data + 4;
    let mut remaining = size - 4;
    e.with_stack(0x10c, |e, name| {
        while count != 0 {
            count -= 1;
            let mut index = 0xffff_ffffu32;
            e.mem.set_u8(name.addr(), 0);
            let Some(at) = take(&mut position, &mut remaining, 4) else {
                return false;
            };
            let length = e.mem.u32(at);
            if length > remaining || length >= 0x104 {
                return false;
            }
            e.call(0x0040_1460, &args![name, position, length]);
            e.mem.set_u8(name.addr() + length, 0);
            position = position.wrapping_add(length);
            remaining -= length;
            let Some(at) = take(&mut position, &mut remaining, 4) else {
                return false;
            };
            let texture_set = e.mem.u32(at);
            if version >= 0xf {
                let Some(at) = take(&mut position, &mut remaining, 4) else {
                    return false;
                };
                index = e.mem.u32(at);
            }
            if e.mem.i8(name.addr()) != 0 && texture_set != 0 {
                tes_model_texture_swap_add_tex_swap(
                    e,
                    this,
                    name.addr(),
                    index as i32,
                    texture_set,
                );
            }
        }
        true
    })
}

// Translated from 0048a990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a990 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESModelTextureSwap::LoadModelTextureSwapChunk` (engine map), cdecl:
/// with a model and a file, a `MODS`, `MO2S`, `MO3S` or `MO4S` chunk is
/// read into a stack buffer of its size and unpacked
/// (`UnpackTextureSwapChunkData` with `00403570`'s word); any other model
/// chunk (`TESModel::IsModelChunk`) goes to `TESModel::LoadModelChunk`.
/// The stack cookie check is not translated.
pub fn tes_model_texture_swap_load_model_texture_swap_chunk(e: &mut Engine, model: Ptr, file: Ptr) {
    if file.addr() == 0 || model.addr() == 0 {
        return;
    }
    let chunk = e.call(0x0047_26b0, &args![file]).u32();
    match chunk {
        0x5334_4f4d | 0x5332_4f4d | 0x5333_4f4d | 0x5344_4f4d => {
            let size = e.call(0x0040_1660, &args![file]).u32();
            // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
            let reserved = size.wrapping_add(15) & !15;
            e.with_stack(reserved, |e, buffer| {
                e.call(0x0047_2890, &args![file, buffer, size]);
                let word = e.call(0x0040_3570, &args![file]).u16();
                tes_model_texture_swap_unpack_texture_swap_chunk_data(
                    e,
                    model,
                    buffer.addr(),
                    size,
                    u32::from(word),
                );
            });
        }
        _ => {
            if e.call(0x0048_9280, &args![chunk]).bool() {
                e.call(0x0048_92d0, &args![model, file]);
            }
        }
    }
}

// Translated from 0048b310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::HasTextureSwap` (engine map), cdecl: the model of
/// `form` (`TESBipedModelForm::GetWorldTESModel` with `which`, else the
/// form cast to the type at `011831c4`) has a non-empty swap list.
pub fn tes_model_texture_swap_has_texture_swap(e: &mut Engine, form: u32, which: u32) -> bool {
    let mut model = e.call(0x0048_1010, &args![form, which]).u32();
    if model == 0 {
        model = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_31c4u32, 0u32],
            )
            .u32();
    }
    if model == 0 {
        return false;
    }
    let list = e.call(SWAP_LIST, &args![model]).u32();
    !e.call(NODE_IS_END, &args![list]).bool()
}

// Translated from 0048b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::SwapTexturesRecurse` (engine map), cdecl: walks
/// the scene graph under `node`, numbering the shapes in `*shape_index`.
/// A node (virtual slot 0xc) recurses into its children (`0043b480` the
/// count, `0043b4a0` the child). A shape (slot 0x18) whose index is in
/// `swaps` (`00853130`) gets the swap: `00450f90(1)` when the swap's
/// texture set is `00593220`'s, otherwise, for a lighting shader property
/// (`NiAVObject::GetProperty(3)` with a type 8..=0xc, `00441110`), the
/// set's shader textures (`BGSTextureSet::CreateShaderTextureSet`,
/// `fn_0048b2f0`), `BSShaderPPLightingProperty::LoadTextureSet(0, 0)` and
/// virtual slot 0x108; the swap is removed (`00405430`) and true answered
/// when none are left. Every shape counts one index.
pub fn tes_model_texture_swap_swap_textures_recurse(
    e: &mut Engine,
    node: u32,
    swaps: u32,
    shape_index: u32,
) -> bool {
    if node == 0 || swaps == 0 {
        return false;
    }
    let children = e.vcall(node, 0xc, &args![]).u32();
    if children != 0 {
        let count = e.call(0x0043_b480, &args![children]).u32();
        for i in 0..count {
            let child = e.call(0x0043_b4a0, &args![children, i]).u32();
            if child != 0
                && tes_model_texture_swap_swap_textures_recurse(e, child, swaps, shape_index)
            {
                return true;
            }
        }
        return false;
    }
    if e.vcall(node, 0x18, &args![]).u32() == 0 {
        return false;
    }
    let found = e.with_stack(4, |e, slot| {
        let index = e.mem.u32(shape_index);
        let ok = e.call(0x0085_3130, &args![swaps, index, slot]).bool();
        (ok, e.mem.u32(slot.addr()))
    });
    if found.0 && found.1 != 0 {
        let swap = found.1;
        if e.mem.u32(swap) == e.call(0x0059_3220, &args![]).u32() {
            e.call(0x0045_0f90, &args![node, 1u32]);
        } else {
            let property = e.call(0x00a5_9d30, &args![node, 3u32]).u32();
            let lighting = property != 0 && {
                let kind = e.call(0x0044_1110, &args![property]).i32();
                kind >= 8 && e.call(0x0044_1110, &args![property]).i32() <= 0xc
            };
            // `NEG`/`SBB`/`AND`: the property when it is a lighting one.
            let shader = if lighting { property } else { 0 };
            if shader != 0 {
                let texture_set = e.mem.u32(swap);
                let shader_set = e.call(0x0059_2d20, &args![texture_set]).u32();
                fn_0048b2f0(e, Ptr::new(shader), shader_set);
                e.call(0x00b6_6ad0, &args![shader, 0u32, 0u32]);
                e.vcall(shader, 0x108, &args![]);
            }
        }
        let index = e.mem.u32(shape_index);
        e.call(0x0040_5430, &args![swaps, index]);
        if e.call(WORD_0C, &args![swaps]).u32() == 0 {
            return true;
        }
    }
    let index = e.mem.u32(shape_index);
    e.mem.set_u32(shape_index, index.wrapping_add(1));
    false
}

// Translated from 0048afe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048afe0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESModelTextureSwap::SwapTextures` (engine map): applies the model's
/// texture swaps to the scene graph `node`. With warnings disabled
/// (`MessageHandler::IncDisableWarningCount(1)`) it fills a 0x25-bucket
/// `BSMap` (`fn_0048b380`) with the swaps by shape index (`006f9a80`;
/// index -1 prints "Found invalid swap index."), swaps them
/// (`SwapTexturesRecurse` from shape 0; any left print "Not all texture
/// swaps were performed."), enables warnings again and, when the warning
/// count (`0046e8a0`) changed, prints the model's path (virtual slot 0x14)
/// in "Warnings were encountered ...". The map is destroyed
/// (`fn_0048b460`). `__thiscall`, `RET 4`. The exception-unwinding frame is
/// not translated.
pub fn tes_model_texture_swap_swap_textures(e: &mut Engine, this: Ptr, node: u32) {
    if node == 0 {
        return;
    }
    let warnings = e.call(0x0046_e8a0, &args![]).u32();
    e.call(0x0043_b2b0, &args![1u32]);
    e.with_stack(0x10, |e, map| {
        fn_0048b380(e, map, 0x25);
        let mut list = e.call(SWAP_LIST, &args![this]).u32();
        while list != 0 && !e.call(NODE_IS_END, &args![list]).bool() {
            let data = e.call(NODE_DATA, &args![list]).u32();
            let entry = e.mem.u32(data);
            list = e.call(NODE_NEXT, &args![list]).u32();
            if e.mem.i32(entry + 4) == -1 {
                e.call(PRINT_ERROR, &args![0x0101_d234u32]);
            } else {
                let index = e.mem.u32(entry + 4);
                e.call(0x006f_9a80, &args![map, index, entry]);
            }
        }
        if e.call(WORD_0C, &args![map]).u32() != 0 {
            e.with_stack(4, |e, shape_index| {
                tes_model_texture_swap_swap_textures_recurse(
                    e,
                    node,
                    map.addr(),
                    shape_index.addr(),
                );
            });
            if e.call(WORD_0C, &args![map]).u32() != 0 {
                e.call(PRINT_ERROR, &args![0x0101_d20cu32]);
            }
        }
        e.call(0x0043_b2b0, &args![0u32]);
        if warnings != e.call(0x0046_e8a0, &args![]).u32() {
            let path = e.vcall(this.addr(), 0x14, &args![]).u32();
            e.call(PRINT_ERROR, &args![0x0101_d1a8u32, path]);
        }
        fn_0048b460(e, map);
    });
}

/// `strlen` (wrapper `0044a670`, cdecl).
const STRLEN: u32 = 0x0044_a670;
/// `memcpy(dest, source, count)` (wrapper `00401460`, cdecl).
const MEMCPY: u32 = 0x0040_1460;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped).
const SWAP_BYTES: u32 = 0x0040_1500;
/// `00401080` (cdecl(word pointer, byte)): byte-swaps the word in place.
const SWAP_WORD: u32 = 0x0040_1080;
/// `TESForm::__AddChunkData` (`00485990`, cdecl(chunk, data, size)).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESModel::Save` (`004891d0`, thiscall, `RET 8`), `TESModel::CopyComponent`
/// (`00489430`, `RET 4`) and `TESModel::CompareComponent` (`00489480`,
/// `RET 4`).
const MODEL_SAVE: u32 = 0x0048_91d0;
const MODEL_COPY_COMPONENT: u32 = 0x0048_9430;
const MODEL_COMPARE_COMPONENT: u32 = 0x0048_9480;
/// `005ae380` (thiscall on a list node): the number of nodes with an item.
const LIST_COUNT: u32 = 0x005a_e380;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type descriptors
/// `BaseFormComponent` (`01183040`) and `TESModelTextureSwap` (`011831c4`,
/// name `.?AVTESModelTextureSwap@@` read at +8).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_MODEL_TEXTURE_SWAP: u32 = 0x0118_31c4;
/// `_stricmp` (wrapper `00404dc0`, cdecl) and `NiStricmp` (`00aa1360`, cdecl).
const STRICMP: u32 = 0x0040_4dc0;
const NI_STRICMP: u32 = 0x00aa_1360;

/// `other` as a `TESModelTextureSwap` (`__RTDynamicCast` from
/// `BaseFormComponent`), or 0.
fn as_texture_swap(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_MODEL_TEXTURE_SWAP,
            0u32
        ],
    )
    .u32()
}

// Translated from 0048a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::Save` (engine map, `RET 0xc`): `TESModel::Save`
/// with `model_chunk` and `texture_chunk`, then, when the swap list packs
/// into a block (`BuildTextureSwapChunkData`), adds the block as
/// `swap_chunk` (`TESForm::__AddChunkData(swap_chunk, block, size)`) and
/// frees it. The two out words the game keeps on its stack are a block
/// here.
pub fn tes_model_texture_swap_save(
    e: &mut Engine,
    this: Ptr,
    model_chunk: u32,
    texture_chunk: u32,
    swap_chunk: u32,
) {
    e.call(MODEL_SAVE, &args![this, model_chunk, texture_chunk]);
    e.with_stack(8, |e, out| {
        let block = out.addr();
        let size = out.addr() + 4;
        e.mem.set_u32(block, 0);
        e.mem.set_u32(size, 0);
        if tes_model_texture_swap_build_texture_swap_chunk_data(e, this, block, size) {
            let data = e.mem.u32(block);
            let length = e.mem.u32(size);
            e.call(ADD_CHUNK_DATA, &args![swap_chunk, data, length]);
            let data = e.mem.u32(block);
            e.call(FREE, &args![data]);
        }
    });
}

// Translated from 0048a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::BuildTextureSwapChunkData` (engine map, `RET 8`):
/// clears `*block` and `*size`; unless the swap list (`00500940`) ends at
/// once (`008256d0`, asked even of a null node, as the exe does), sizes the
/// chunk (4 bytes of count, then per entry 4 + name length + 4 + 4),
/// allocates it into `*block` and writes the count, then per entry the
/// name length, the name's bytes (`memcpy`, no terminator), the texture
/// set's form id (`0084e3a0`, 0 without a texture set) and the shape index
/// (+4). Before the count, and again before each name length, the count
/// word on the stack is byte-swapped when `00401500` says so (`00401080`);
/// the exe swaps the count there, never the length. Answers whether
/// `*block` is set.
pub fn tes_model_texture_swap_build_texture_swap_chunk_data(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    size: u32,
) -> bool {
    e.mem.set_u32(block, 0);
    e.mem.set_u32(size, 0);
    let mut node = e.call(SWAP_LIST, &args![this]).u32();
    if !e.call(NODE_IS_END, &args![node]).bool() {
        let mut count = 0u32;
        e.mem.set_u32(size, 4);
        while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
            let data = e.call(NODE_DATA, &args![node]).u32();
            let entry = e.mem.u32(data);
            let total = e.mem.u32(size).wrapping_add(4);
            e.mem.set_u32(size, total);
            let length = e.call(STRLEN, &args![entry + 8]).u32();
            let total = length.wrapping_add(e.mem.u32(size));
            e.mem.set_u32(size, total);
            let total = e.mem.u32(size).wrapping_add(4);
            e.mem.set_u32(size, total);
            let total = e.mem.u32(size).wrapping_add(4);
            e.mem.set_u32(size, total);
            count = count.wrapping_add(1);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let bytes = e.mem.u32(size);
        let buffer = e.call(ALLOCATE, &args![bytes]).u32();
        e.mem.set_u32(block, buffer);
        let mut out = e.mem.u32(block);
        e.with_stack(4, |e, count_word| {
            e.mem.set_u32(count_word.addr(), count);
            if e.call(SWAP_BYTES, &[]).u32() as u8 != 0 {
                e.call(SWAP_WORD, &args![count_word, 0u32]);
            }
            let value = e.mem.u32(count_word.addr());
            e.mem.set_u32(out, value);
            out = out.wrapping_add(4);
            node = e.call(SWAP_LIST, &args![this]).u32();
            while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
                let data = e.call(NODE_DATA, &args![node]).u32();
                let entry = e.mem.u32(data);
                let length = e.call(STRLEN, &args![entry + 8]).u32();
                if e.call(SWAP_BYTES, &[]).u32() as u8 != 0 {
                    e.call(SWAP_WORD, &args![count_word, 0u32]);
                }
                e.mem.set_u32(out, length);
                out = out.wrapping_add(4);
                e.call(MEMCPY, &args![out, entry + 8, length]);
                out = out.wrapping_add(length);
                let mut id = 0u32;
                let texture_set = e.mem.u32(entry);
                if texture_set != 0 {
                    id = e.call(WORD_0C, &args![texture_set]).u32();
                }
                e.mem.set_u32(out, id);
                out = out.wrapping_add(4);
                let index = e.mem.u32(entry + 4);
                e.mem.set_u32(out, index);
                out = out.wrapping_add(4);
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        });
    }
    e.mem.u32(block) != 0
}

// Translated from 0048aba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::CopyComponent` (engine map, `RET 4`):
/// `TESModel::CopyComponent`, then, when `other` is a
/// `TESModelTextureSwap`, empties this swap list (`ClearTexSwapList`) and
/// adds each of the other's entries (`AddTexSwap` with its name, shape index
/// and texture set).
pub fn tes_model_texture_swap_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    e.call(MODEL_COPY_COMPONENT, &args![this, other]);
    let source = as_texture_swap(e, other);
    if source == 0 {
        return;
    }
    tes_model_texture_swap_clear_tex_swap_list(e, this);
    let mut node = e.call(SWAP_LIST, &args![source]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let entry = e.mem.u32(data);
        let texture_set = e.mem.u32(entry);
        let index = e.mem.i32(entry + 4);
        tes_model_texture_swap_add_tex_swap(e, this, entry + 8, index, texture_set);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0048ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::CompareComponent` (engine map, `RET 4`): true when
/// the two differ: when `TESModel::CompareComponent` says so; when `other`
/// is no `TESModelTextureSwap`; when the swap lists hold different numbers
/// of entries (`005ae380`, the other's counted first); when an entry of the
/// other has no entry of its shape index here (`GetTexSwap`), another
/// texture set, or a name that differs ignoring case (`_stricmp`, only when
/// both names are not empty).
pub fn tes_model_texture_swap_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(MODEL_COMPARE_COMPONENT, &args![this, other]).u32() as u8 != 0 {
        return true;
    }
    let source = as_texture_swap(e, other);
    if source == 0 {
        return true;
    }
    let list = e.call(SWAP_LIST, &args![source]).u32();
    let theirs = e.call(LIST_COUNT, &args![list]).u32();
    let list = e.call(SWAP_LIST, &args![this]).u32();
    let mine = e.call(LIST_COUNT, &args![list]).u32();
    if theirs != mine {
        return true;
    }
    let mut node = e.call(SWAP_LIST, &args![source]).u32();
    while node != 0 && !e.call(NODE_IS_END, &args![node]).bool() {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let entry = e.mem.u32(data);
        let index = e.mem.i32(entry + 4);
        let own = tes_model_texture_swap_get_tex_swap(e, this, index);
        if own == 0 {
            return true;
        }
        if e.mem.u32(own) != e.mem.u32(entry) {
            return true;
        }
        if e.mem.i8(own + 8) != 0
            && e.mem.i8(entry + 8) != 0
            && e.call(STRICMP, &args![own + 8, entry + 8]).u32() != 0
        {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0048af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFixedString::EqualsNoCase` (engine map, `RET 4`): true when the
/// string word at `this` is `other`; false when either is null; otherwise
/// whether `NiStricmp(*this, other)` gives 0.
pub fn ni_fixed_string_equals_no_case(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let mine = e.mem.u32(this.addr());
    if mine == other {
        return true;
    }
    if other == 0 || e.mem.u32(this.addr()) == 0 {
        return false;
    }
    let mine = e.mem.u32(this.addr());
    e.call(NI_STRICMP, &args![mine, other]).u32() == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_af10, fn_0048af10(Ptr) -> Ptr),
        entry!(0x0048_af40, fn_0048af40(Ptr, Ptr) -> Ptr),
        entry!(0x0048_af70, fn_0048af70(Ptr) -> bool),
        entry!(0x0048_b2f0, fn_0048b2f0(Ptr, u32)),
        entry!(0x0048_b500, fn_0048b500(Ptr, Ptr)),
        entry!(0x0048_b3e0, fn_0048b3e0(Ptr, u32) -> Ptr),
        entry!(0x0048_b380, fn_0048b380(Ptr, u32) -> Ptr),
        entry!(0x0048_b4c0, fn_0048b4c0(Ptr)),
        entry!(0x0048_b460, fn_0048b460(Ptr)),
        entry!(0x0048_ade0, tes_model_texture_swap_clear_tex_swap_list(Ptr)),
        entry!(
            0x0048_a500,
            tes_model_texture_swap_clear_data_component(Ptr)
        ),
        entry!(0x0048_a490, fn_0048a490(Ptr)),
        entry!(0x0048_ad70, tes_model_texture_swap_get_tex_swap(Ptr, i32) -> u32),
        entry!(
            0x0048_ae30,
            tes_model_texture_swap_add_tex_swap(Ptr, u32, i32, u32)
        ),
        entry!(0x0048_a7b0, tes_model_texture_swap_unpack_texture_swap_chunk_data(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x0048_a990,
            tes_model_texture_swap_load_model_texture_swap_chunk(Ptr, Ptr)
        ),
        entry!(0x0048_b310, tes_model_texture_swap_has_texture_swap(u32, u32) -> bool),
        entry!(0x0048_b130, tes_model_texture_swap_swap_textures_recurse(u32, u32, u32) -> bool),
        entry!(0x0048_afe0, tes_model_texture_swap_swap_textures(Ptr, u32)),
        entry!(
            0x0048_a470,
            tes_model_texture_swap_initialize_data_component(Ptr)
        ),
        entry!(0x0048_aa80, tesmodeltextureswap_init_item(Ptr, Ptr)),
        entry!(0x0048_a440, tes_model_texture_swap_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0048_b3b0, bs_map_int_tex_swap_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0048_b530, bs_map_base_int_tex_swap_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0048_a520, tes_model_texture_swap_save(Ptr, u32, u32, u32)),
        entry!(
            0x0048_a590,
            tes_model_texture_swap_build_texture_swap_chunk_data(Ptr, u32, u32) -> bool
        ),
        entry!(0x0048_aba0, tes_model_texture_swap_copy_component(Ptr, u32)),
        entry!(
            0x0048_ac40,
            tes_model_texture_swap_compare_component(Ptr, u32) -> bool
        ),
        entry!(0x0048_af90, ni_fixed_string_equals_no_case(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0048a470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_9040, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_a470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_9040), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{calls_to, run, stub};
    use std::cell::Cell;
    use std::rc::Rc;

    const OWNER: u32 = 0x0800_0000;
    const FIRST_NODE: u32 = 0x0800_3000;
    const SECOND_NODE: u32 = 0x0800_3100;
    const OWNER_NAME: u32 = 0xaaaa_0001;
    /// An id the lookup (`004839c0`) does not find.
    const MISSING_ID: u32 = 0xbad;

    struct Setup {
        e: Engine,
        entries: [u32; 2],
    }

    /// A list of two nodes whose entries hold `ids`. `ends_after_first_visit` makes
    /// `008256d0` answer true from its second call on (the walk at a head that stays).
    fn setup(ids: [u32; 2], ends_after_first_visit: bool) -> Setup {
        let mut e = Engine::new();
        let entries = ids.map(|id| {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, id);
            entry
        });
        let slots = entries.map(|entry| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, entry);
            slot
        });
        e.map(OWNER, 0x40);
        e.map(0x0900_0000, 0x200);
        e.mem.set_u32(OWNER, 0x0900_0000);
        e.mem.set_u32(0x0900_0130, 0x0090_0300);
        e.register_double(0x0090_0300, |_, _| Ret {
            eax: OWNER_NAME,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[0x005b_5e40, 0x0063_f7b0, 0x0040_1030, 0x0090_5330],
        );
        e.register_double(0x0050_0940, move |_, _| Ret {
            eax: FIRST_NODE,
            ..Ret::default()
        });
        let visits = Rc::new(Cell::new(0));
        e.register_double(0x0082_56d0, move |_, _| {
            visits.set(visits.get() + 1);
            Ret {
                eax: u32::from(ends_after_first_visit && visits.get() > 1),
                ..Ret::default()
            }
        });
        e.register_double(0x0068_15c0, move |_, a| Ret {
            eax: if a[0] == FIRST_NODE {
                slots[0]
            } else {
                slots[1]
            },
            ..Ret::default()
        });
        // `00905330` takes the second node out of the list: the first node then has no next.
        let removed = Rc::new(Cell::new(false));
        let removal = removed.clone();
        e.register_double(0x0090_5330, move |_, _| {
            removal.set(true);
            Ret::default()
        });
        e.register_double(0x0072_6070, move |_, a| Ret {
            eax: match (a[0], removed.get()) {
                (FIRST_NODE, false) => SECOND_NODE,
                _ => 0,
            },
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x0a11_e000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: if a[0] & 0xffff == MISSING_ID {
                0
            } else {
                a[0] ^ 0x1000_0000
            },
            ..Ret::default()
        });
        // The cast keeps what the lookup found.
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 0x77,
            ..Ret::default()
        });
        Setup { e, entries }
    }

    #[test]
    fn found_entries_are_replaced_by_their_forms() {
        let mut s = setup([0x11, 0x22], false);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(s.e.mem.u32(s.entries[0]), 0x1100_0011);
        assert_eq!(s.e.mem.u32(s.entries[1]), 0x1100_0022);
        assert_eq!(
            calls_to(&calls, 0x0072_6070),
            [vec![FIRST_NODE], vec![SECOND_NODE]]
        );
        assert!(calls_to(&calls, 0x005b_5e40).is_empty());
        assert_eq!(calls_to(&calls, 0x0050_0940), [vec![0x1000]]);
    }

    #[test]
    fn a_missing_entry_after_a_good_one_is_taken_out_of_the_list() {
        let mut s = setup([0x11, MISSING_ID], false);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_d148, 0x0100_0bad, OWNER_NAME, OWNER + 0x77]]
        );
        // The list entry holds the entry's address; the walk continues after the good node.
        let removal = calls_to(&calls, 0x0090_5330);
        assert_eq!(removal.len(), 1);
        assert_eq!(removal[0][0], FIRST_NODE);
        assert_eq!(
            calls_to(&calls, 0x0072_6070),
            [vec![FIRST_NODE], vec![FIRST_NODE]]
        );
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![s.entries[1]]]);
        assert!(calls_to(&calls, 0x0063_f7b0).is_empty());
        // The entry was cleared before it was freed.
        assert_eq!(s.e.mem.u32(s.entries[1]), 0);
    }

    #[test]
    fn a_missing_head_entry_is_removed_in_place() {
        let mut s = setup([MISSING_ID, 0x22], true);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(calls_to(&calls, 0x0063_f7b0), [vec![FIRST_NODE]]);
        assert!(calls_to(&calls, 0x0090_5330).is_empty());
        assert!(calls_to(&calls, 0x0072_6070).is_empty());
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![s.entries[0]]]);
        // The head node is asked again and ends the walk.
        assert_eq!(
            calls_to(&calls, 0x0082_56d0),
            [vec![FIRST_NODE], vec![FIRST_NODE]]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0048a440_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_a490, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_a440, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_a440)
                .collect();
            let mut expected = vec![(0x0048_a490, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0048b3b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_b460, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_b3b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_b3b0)
                .collect();
            let mut expected = vec![(0x0048_b460, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0048b530_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_b4c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_b530, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_b530)
                .collect();
            let mut expected = vec![(0x0048_b4c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with a shared vtable (slot `n` calls `SLOTS + n`, no-ops)
    /// and no-op doubles for `callees`.
    fn engine(callees: &[u32]) -> Engine {
        let mut e = Engine::new();
        e.map(VTABLE, 0x200);
        for i in 0..0x80 {
            e.mem.set_u32(VTABLE + 4 * i, SLOTS + 4 * i);
            e.register(SLOTS + 4 * i, |_, _| Ret::default());
        }
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// List doubles over `{data, next}` nodes (the node's data address is
    /// the node itself; a null node ends the walk).
    fn lists(e: &mut Engine) {
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_END, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
    }

    fn node(e: &mut Engine, data: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, data);
        e.mem.set_u32(node + 4, next);
        node
    }

    fn swap_entry(e: &mut Engine, texture_set: u32, index: i32) -> u32 {
        let entry = e.mem.alloc(SWAP_ENTRY_SIZE);
        e.mem.set_u32(entry, texture_set);
        e.mem.set_i32(entry + 4, index);
        entry
    }

    #[test]
    fn entries_and_small_helpers() {
        let mut e = engine(&[0x0043_ba60, 0x0066_b0d0, FREE]);
        let entry = e.mem.alloc(0x10);
        e.mem.write(entry, &[0xff; 0x10]);
        assert_eq!(e.call(0x0048_af10, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0);
        assert_eq!(e.mem.u32(entry + 4), 0xffff_ffff);
        assert_eq!(e.mem.u8(entry + 8), 0);
        assert_eq!(e.mem.u8(entry + 9), 0xff);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x5050);
        let target = e.mem.alloc(4);
        assert_eq!(e.call(0x0048_af40, &args![target, source]).u32(), target);
        assert_eq!(e.mem.u32(target), 0x5050);
        assert_eq!(calls_to(&e, 0x0043_ba60), vec![vec![source]]);
        e.map(0x0109_b220, 4);
        e.mem.set_u32(0x0109_b220, 0x5050);
        assert!(!e.call(0x0048_af70, &args![target]).bool());
        e.mem.set_u32(target, 0);
        assert!(e.call(0x0048_af70, &args![target]).bool());
        e.call(0x0048_b2f0, &args![0x1000u32, 0x77u32]);
        assert_eq!(calls_to(&e, 0x0066_b0d0), vec![vec![0x10a4, 0x77]]);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 8, 9);
        e.call(0x0048_b500, &args![0u32, item]);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&e, FREE), vec![vec![item]]);
    }

    #[test]
    fn the_map_is_built_and_destroyed() {
        let mut e = engine(&[0x00ec_61c0, FREE, 0x0043_8af0]);
        e.register(ALLOCATE, |_, a| ret(0x0700_0000 + a[0]));
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0xdead);
        assert_eq!(e.call(0x0048_b380, &args![map, 0x25u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 8), 0x0700_0094);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(calls_to(&e, 0x00ec_61c0), vec![vec![0x0700_0094, 0, 0x94]]);
        e.call(0x0048_b460, &args![map]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![map]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x0700_0094]]);
        // A size of 0 allocates nothing (the bucket word stays as it was);
        // without buckets the destructor frees nothing.
        let empty = e.mem.alloc(0x10);
        e.call(0x0048_b3e0, &args![empty, 0u32]);
        assert_eq!(e.mem.u32(empty), MAP_BASE_VTABLE);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
        e.call(0x0048_b4c0, &args![empty]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn clearing_frees_every_entry() {
        let mut e = engine(&[FREE, 0x0048_90e0, 0x0046_ffb0, 0x0048_9070]);
        let model = e.mem.alloc(0x20);
        // The list head at +0x18 holds the head entry; removing it moves
        // the next entry in.
        let second = node(&mut e, 0x0bbb, 0);
        e.mem.set_u32(model + 0x18, 0x0aaa);
        e.mem.set_u32(model + 0x1c, second);
        lists(&mut e);
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (data, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], data);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.call(0x0048_a490, &args![model]);
        assert_eq!(e.mem.u32(model), 0x0101_d124);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x0aaa], vec![0x0bbb]]);
        let log: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [0x0048_90e0, 0x0046_ffb0, 0x0048_9070].contains(a))
            .collect();
        assert_eq!(log, vec![0x0048_90e0, 0x0046_ffb0, 0x0048_9070]);
        assert_eq!(calls_to(&e, 0x0046_ffb0), vec![vec![model + 0x18]]);
    }

    #[test]
    fn swaps_are_found_by_index_and_added() {
        let mut e = engine(&[0x0040_6d30]);
        lists(&mut e);
        let a = swap_entry(&mut e, 0x51, 2);
        let b = swap_entry(&mut e, 0x52, 5);
        let n2 = node(&mut e, b, 0);
        let n1 = node(&mut e, a, n2);
        e.register_double(SWAP_LIST, move |_, _| ret(n1));
        let model = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0048_ad70, &args![model, 5i32]).u32(), b);
        assert_eq!(e.call(0x0048_ad70, &args![model, 7i32]).u32(), 0);
        assert_eq!(e.call(0x0048_ad70, &args![model, -1i32]).u32(), 0);
        // Adding to an existing index replaces the texture set.
        e.call(0x0048_ae30, &args![model, 0x0aaau32, 2i32, 0x99u32]);
        assert_eq!(e.mem.u32(a), 0x99);
        // A new index makes an entry.
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0048_ae30, &args![model, 0x0aaau32, 9i32, 0x98u32]);
        let made = added.borrow()[0].1;
        assert_eq!(added.borrow()[0].0, model + 0x18);
        assert_eq!(e.mem.u32(made), 0x98);
        assert_eq!(e.mem.i32(made + 4), 9);
        assert_eq!(
            calls_to(&e, 0x0040_6d30),
            vec![vec![made + 8, 0x80, 0x0aaa]]
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x88]]);
        // No name or no texture set: nothing.
        e.call(0x0048_ae30, &args![model, 0u32, 11i32, 0x98u32]);
        e.call(0x0048_ae30, &args![model, 0x0aaau32, 11i32, 0u32]);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
    }

    /// Swap chunk bytes: count, then (length, name, texture set[, index]).
    fn chunk(swaps: &[(&[u8], u32, u32)], with_index: bool) -> Vec<u8> {
        let mut out = (swaps.len() as u32).to_le_bytes().to_vec();
        for (name, set, index) in swaps {
            out.extend((name.len() as u32).to_le_bytes());
            out.extend(*name);
            out.extend(set.to_le_bytes());
            if with_index {
                out.extend(index.to_le_bytes());
            }
        }
        out
    }

    fn unpack(bytes: &[u8], version: u32) -> (bool, Vec<(Vec<u8>, i32, u32)>) {
        let mut e = engine(&[]);
        e.register(0x0040_1460, |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            Ret::default()
        });
        // Every add finds no entry and makes one; the list double records
        // its name and index.
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register(SWAP_LIST, |_, _| ret(0));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register_double(LIST_ADD, move |e, a| {
            let entry = e.mem.u32(a[1]);
            seen.borrow_mut()
                .push((e.mem.cstr(entry + 8), e.mem.i32(entry + 4), 0));
            Ret::default()
        });
        let data = e.mem.alloc(bytes.len() as u32 + 4);
        e.mem.write(data, bytes);
        let model = e.mem.alloc(0x20);
        let result = e
            .call(
                0x0048_a7b0,
                &args![model, data, bytes.len() as u32, version],
            )
            .bool();
        let list = added.borrow().clone();
        (result, list)
    }

    #[test]
    fn unpacking_adds_named_swaps() {
        let bytes = chunk(
            &[(b"Shape", 0x10, 3), (b"", 0x11, 4), (b"Other", 0, 5)],
            true,
        );
        let (result, added) = unpack(&bytes, 0xf);
        assert!(result);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].0, b"Shape");
        assert_eq!(added[0].1, 3);
        // Before version 0xf there is no index (-1).
        let bytes = chunk(&[(b"Shape", 0x10, 0)], false);
        let (result, added) = unpack(&bytes, 0xe);
        assert!(result);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].1, -1);
    }

    #[test]
    fn unpacking_stops_at_short_or_bad_data() {
        assert!(unpack(&[1, 0, 0, 0], 0xf).0);
        let mut bytes = chunk(&[(b"Shape", 0x10, 3)], true);
        bytes.truncate(bytes.len() - 2);
        assert!(!unpack(&bytes, 0xf).0);
        // A name of 0x104 bytes is refused.
        let mut long = 1u32.to_le_bytes().to_vec();
        long.extend(0x104u32.to_le_bytes());
        long.extend([b'a'; 0x110]);
        assert!(!unpack(&long, 0xf).0);
        // A length past the end is refused.
        let mut past = 1u32.to_le_bytes().to_vec();
        past.extend(9u32.to_le_bytes());
        past.extend(b"abc");
        assert!(!unpack(&past, 0xf).0);
    }

    #[test]
    fn swap_chunks_are_unpacked_and_model_chunks_loaded() {
        for (signature, unpacked) in [
            (0x5344_4f4du32, true),
            (0x5332_4f4d, true),
            (0x5333_4f4d, true),
            (0x5334_4f4d, true),
            (0x4c44_4f4d, false),
        ] {
            let mut e = engine(&[0x0048_92d0, 0x0047_2890]);
            e.register_double(0x0047_26b0, move |_, _| ret(signature));
            e.register(0x0040_1660, |_, _| ret(4));
            e.register(0x0040_3570, |_, _| ret(0x1_000f));
            e.register(0x0048_9280, |_, a| ret(u32::from(a[0] == 0x4c44_4f4d)));
            e.call(0x0048_a990, &args![0x1000u32, 0x2000u32]);
            assert_eq!(calls_to(&e, 0x0047_2890).len(), usize::from(unpacked));
            if unpacked {
                assert_eq!(calls_to(&e, 0x0047_2890)[0][2], 4);
                assert!(calls_to(&e, 0x0048_9280).is_empty());
            } else {
                assert_eq!(calls_to(&e, 0x0048_92d0), vec![vec![0x1000, 0x2000]]);
            }
        }
        // No model or no file: nothing.
        let mut e = engine(&[]);
        e.call(0x0048_a990, &args![0u32, 0x2000u32]);
        e.call(0x0048_a990, &args![0x1000u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn has_texture_swap_checks_the_list() {
        let mut e = engine(&[]);
        e.register(0x0048_1010, |_, a| ret(if a[0] == 1 { 0x0b1 } else { 0 }));
        e.register(0x00ec_43fb, |_, a| ret(if a[0] == 2 { 0x0c2 } else { 0 }));
        e.register(SWAP_LIST, |_, a| ret(a[0] + 1));
        e.register(NODE_IS_END, |_, a| ret(u32::from(a[0] == 0x0c3)));
        assert!(e.call(0x0048_b310, &args![1u32, 0u32]).bool());
        assert!(!e.call(0x0048_b310, &args![2u32, 0u32]).bool());
        assert!(!e.call(0x0048_b310, &args![3u32, 0u32]).bool());
        assert_eq!(
            calls_to(&e, 0x00ec_43fb)[0],
            vec![2, 0, 0x0118_3028, 0x0118_31c4, 0]
        );
    }

    /// A scene: a node with two shapes and a null child. The swap map holds
    /// the swaps by shape index.
    fn scene(e: &mut Engine) -> (u32, u32, u32) {
        let shape_a = e.mem.alloc(0x10);
        e.mem.set_u32(shape_a, VTABLE);
        let shape_b = e.mem.alloc(0x10);
        e.mem.set_u32(shape_b, VTABLE);
        let root = e.mem.alloc(0x10);
        e.mem.set_u32(root, VTABLE);
        (root, shape_a, shape_b)
    }

    #[test]
    fn swapping_recurses_and_applies_shader_textures() {
        let mut e = engine(&[
            0x0045_0f90,
            0x0048_1010,
            0x00b6_6ad0,
            0x0040_5430,
            0x0066_b0d0,
        ]);
        let (root, shape_a, shape_b) = scene(&mut e);
        let children = [shape_a, 0, shape_b];
        e.register_double(SLOTS + 0xc, move |_, a| {
            ret(if a[0] == root { 0x0c0 } else { 0 })
        });
        e.register(SLOTS + 0x18, |_, _| ret(1));
        e.register(0x0043_b480, |_, _| ret(3));
        e.register_double(0x0043_b4a0, move |_, a| ret(children[a[1] as usize]));
        // Swaps for shape indices 0 and 1, held in a test map of two words.
        let swap_0 = e.mem.alloc(8);
        e.mem.set_u32(swap_0, 0x0dd);
        let swap_1 = e.mem.alloc(8);
        e.mem.set_u32(swap_1, 0x7e5);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 2);
        let swaps = [swap_0, swap_1];
        e.register_double(0x0085_3130, move |e, a| {
            if a[1] < 2 {
                e.mem.set_u32(a[2], swaps[a[1] as usize]);
            }
            ret(u32::from(a[1] < 2))
        });
        e.register(0x0040_5430, |e, a| {
            let count = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0xc, count - 1);
            Ret::default()
        });
        e.register(WORD_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(0x0059_3220, |_, _| ret(0x0dd));
        let property = e.mem.alloc(0x10);
        e.mem.set_u32(property, VTABLE);
        e.register_double(0x00a5_9d30, move |_, _| ret(property));
        e.register(0x0044_1110, |_, _| ret(0xc));
        e.register(0x0059_2d20, |_, a| ret(a[0] + 0x1000));
        let counter = e.mem.alloc(4);
        let done = e.call(0x0048_b130, &args![root, map, counter]).bool();
        assert!(done);
        // Shape 0's set is `00593220`'s: `00450f90(1)`.
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![shape_a, 1]]);
        // Shape 1 gets the shader textures.
        assert_eq!(
            calls_to(&e, 0x0066_b0d0),
            vec![vec![property + 0xa4, 0x17e5]]
        );
        assert_eq!(calls_to(&e, 0x00b6_6ad0), vec![vec![property, 0, 0]]);
        assert_eq!(calls_to(&e, SLOTS + 0x108), vec![vec![property]]);
        assert_eq!(calls_to(&e, 0x0040_5430), vec![vec![map, 0], vec![map, 1]]);
        // The first shape counted; the last returned before counting.
        assert_eq!(e.mem.u32(counter), 1);
    }

    #[test]
    fn swapping_skips_non_lighting_properties_and_counts_shapes() {
        let mut e = engine(&[0x0045_0f90, 0x00b6_6ad0, 0x0040_5430, 0x0066_b0d0]);
        let (_, shape, _) = scene(&mut e);
        e.register(SLOTS + 0xc, |_, _| ret(0));
        e.register(SLOTS + 0x18, |_, _| ret(1));
        let swap = e.mem.alloc(8);
        e.mem.set_u32(swap, 0x7e5);
        e.register_double(0x0085_3130, move |e, a| {
            e.mem.set_u32(a[2], swap);
            ret(1)
        });
        e.register(WORD_0C, |_, _| ret(3));
        e.register(0x0059_3220, |_, _| ret(0x0dd));
        e.register(0x00a5_9d30, |_, _| ret(0x0bad));
        e.register(0x0044_1110, |_, _| ret(7));
        let counter = e.mem.alloc(4);
        e.mem.set_u32(counter, 4);
        assert!(!e
            .call(0x0048_b130, &args![shape, 0x0aaau32, counter])
            .bool());
        assert!(calls_to(&e, 0x00b6_6ad0).is_empty());
        assert_eq!(calls_to(&e, 0x0040_5430), vec![vec![0x0aaa, 4]]);
        assert_eq!(e.mem.u32(counter), 5);
        // No geometry: nothing counted. Null node or map: false at once.
        e.register(SLOTS + 0x18, |_, _| ret(0));
        assert!(!e
            .call(0x0048_b130, &args![shape, 0x0aaau32, counter])
            .bool());
        assert_eq!(e.mem.u32(counter), 5);
        assert!(!e.call(0x0048_b130, &args![0u32, 0x0aaau32, counter]).bool());
        assert!(!e.call(0x0048_b130, &args![shape, 0u32, counter]).bool());
    }

    #[test]
    fn swap_textures_fills_the_map_and_reports() {
        let callees = [0x0043_b2b0, PRINT_ERROR, 0x00ec_61c0, FREE, 0x0043_8af0];
        let mut e = engine(&callees);
        lists(&mut e);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let good = swap_entry(&mut e, 0x51, 2);
        let bad = swap_entry(&mut e, 0x52, -1);
        let n2 = node(&mut e, bad, 0);
        let n1 = node(&mut e, good, n2);
        e.register_double(SWAP_LIST, move |_, _| ret(n1));
        // The map's count is its word at +0xc; SetAt counts one.
        e.register(0x006f_9a80, |e, a| {
            let count = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0xc, count + 1);
            Ret::default()
        });
        e.register(WORD_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        // The recursion double does nothing: the swap stays.
        e.register(SLOTS + 0xc, |_, _| ret(0));
        e.register(SLOTS + 0x18, |_, _| ret(0));
        let warnings = Rc::new(RefCell::new(0));
        let w = warnings.clone();
        e.register_double(0x0046_e8a0, move |_, _| {
            *w.borrow_mut() += 1;
            ret(*w.borrow())
        });
        e.register(SLOTS + 0x14, |_, _| ret(0x0a7a));
        let model = e.mem.alloc(0x20);
        e.mem.set_u32(model, VTABLE);
        let root = e.mem.alloc(0x10);
        e.mem.set_u32(root, VTABLE);
        e.call(0x0048_afe0, &args![model, root]);
        assert_eq!(calls_to(&e, 0x0043_b2b0), vec![vec![1], vec![0]]);
        let set_at = calls_to(&e, 0x006f_9a80);
        assert_eq!(set_at.len(), 1);
        assert_eq!(&set_at[0][1..], &[2, good]);
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![
                vec![0x0101_d234],
                vec![0x0101_d20c],
                vec![0x0101_d1a8, 0x0a7a]
            ]
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, 0x0043_8af0).len(), 1);
        // A null node does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_afe0, &args![model, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod save_copy_compare_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn c_string(e: &Engine, mut p: u32) -> String {
        let mut s = String::new();
        while e.mem.u8(p) != 0 {
            s.push(e.mem.u8(p) as char);
            p += 1;
        }
        s
    }

    /// List doubles: a node is two words (item, next); the swap list of a
    /// model is its `+0x18` node.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SWAP_LIST, |_, a| ret(a[0] + 0x18));
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_END, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_COUNT, |e, a| {
            let (mut node, mut n) = (a[0], 0);
            while node != 0 {
                n += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE, |_, _| Ret::default());
        e.register(STRLEN, |e, a| ret(c_string(e, a[0]).len() as u32));
        e.register(0x0040_6d30, |e, a| {
            let text = c_string(e, a[2]);
            for (i, b) in text.bytes().chain([0]).enumerate() {
                e.mem.set_u8(a[0] + i as u32, b);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(STRICMP, |e, a| {
            let (x, y) = (c_string(e, a[0]), c_string(e, a[1]));
            ret(u32::from(!x.eq_ignore_ascii_case(&y)))
        });
        e.register(WORD_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SWAP_BYTES, |_, _| ret(0));
        e.register(SWAP_WORD, |e, a| {
            let v = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e
    }

    /// A swap entry (texture set, index, name).
    fn entry(e: &mut Engine, texture_set: u32, index: u32, name: &str) -> u32 {
        let p = e.mem.alloc(0x88);
        e.mem.set_u32(p, texture_set);
        e.mem.set_u32(p + 4, index);
        for (i, b) in name.bytes().enumerate() {
            e.mem.set_u8(p + 8 + i as u32, b);
        }
        p
    }

    /// A model (0x20 bytes) whose swap list (+0x18) holds `entries`.
    fn model(e: &mut Engine, entries: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let mut node = this + 0x18;
        for (i, item) in entries.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    fn texture_set(e: &mut Engine, id: u32) -> u32 {
        let p = e.mem.alloc(0x18);
        e.mem.set_u32(p + 0xc, id);
        p
    }

    #[test]
    fn build_packs_count_names_ids_and_indices() {
        let mut e = engine();
        let set = texture_set(&mut e, 0x1234);
        let a = entry(&mut e, set, 3, "ab");
        let b = entry(&mut e, 0, 0xffff_ffff, "xyz");
        let this = model(&mut e, &[a, b]);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0x77);
        let built = e.call(0x0048_a590, &args![this, out, out + 4]).bool();
        assert!(built);
        let (block, size) = (e.mem.u32(out), e.mem.u32(out + 4));
        assert_eq!(size, 4 + (4 + 2 + 8) + (4 + 3 + 8));
        let words = |e: &Engine, at: u32| e.mem.u32(block + at);
        assert_eq!(words(&e, 0), 2);
        assert_eq!(words(&e, 4), 2);
        assert_eq!((e.mem.u8(block + 8), e.mem.u8(block + 9)), (b'a', b'b'));
        assert_eq!(words(&e, 10), 0x1234);
        assert_eq!(words(&e, 14), 3);
        assert_eq!(words(&e, 18), 3);
        assert_eq!(words(&e, 25), 0);
        assert_eq!(words(&e, 29), 0xffff_ffff);
    }

    #[test]
    fn build_swaps_the_count_word_only() {
        let mut e = engine();
        e.register(SWAP_BYTES, |_, _| ret(1));
        let a = entry(&mut e, 0, 1, "ab");
        let this = model(&mut e, &[a]);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_a590, &args![this, out, out + 4]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, SWAP_WORD).len(), 2);
        let block = e.mem.u32(out);
        assert_eq!(e.mem.u32(block), 0x0100_0000);
        assert_eq!(e.mem.u32(block + 4), 2);
    }

    #[test]
    fn build_of_an_empty_list_answers_false() {
        let mut e = engine();
        let this = model(&mut e, &[0]);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 5);
        e.mem.set_u32(out + 4, 5);
        assert!(!e.call(0x0048_a590, &args![this, out, out + 4]).bool());
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
    }

    #[test]
    fn save_adds_the_swap_chunk_after_the_model() {
        let mut e = engine();
        e.register(MODEL_SAVE, |_, _| Ret::default());
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        let a = entry(&mut e, 0, 1, "ab");
        let this = model(&mut e, &[a]);
        e.call_log = Some(vec![]);
        e.call(0x0048_a520, &args![this, 1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, MODEL_SAVE), vec![vec![this, 1, 2]]);
        let added = only(&log, ADD_CHUNK_DATA);
        assert_eq!(added.len(), 1);
        assert_eq!((added[0][0], added[0][2]), (3, 4 + 4 + 2 + 8));
        assert_eq!(only(&log, FREE), vec![vec![added[0][1]]]);
        let empty = model(&mut e, &[0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_a520, &args![empty, 1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, ADD_CHUNK_DATA).is_empty());
        assert!(only(&log, FREE).is_empty());
    }

    #[test]
    fn copy_replaces_the_list_with_the_others_entries() {
        let mut e = engine();
        e.register(MODEL_COPY_COMPONENT, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let old = entry(&mut e, 9, 9, "old");
        let this = model(&mut e, &[old]);
        let a = entry(&mut e, 0x500, 2, "two");
        let b = entry(&mut e, 0x600, 4, "four");
        let other = model(&mut e, &[a, b]);
        e.call(0x0048_aba0, &args![this, other]);
        let first = e.mem.u32(this + 0x18);
        assert_ne!(first, old);
        assert_eq!((e.mem.u32(first), e.mem.u32(first + 4)), (0x500, 2));
        assert_eq!(c_string(&e, first + 8), "two");
        let second = e.mem.u32(e.mem.u32(this + 0x1c));
        assert_eq!((e.mem.u32(second), e.mem.u32(second + 4)), (0x600, 4));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let keep = model(&mut e, &[old]);
        e.call(0x0048_aba0, &args![keep, other]);
        assert_eq!(e.mem.u32(keep + 0x18), old);
    }

    #[test]
    fn compare_reports_each_difference() {
        let mut e = engine();
        e.register(MODEL_COMPARE_COMPONENT, |_, _| ret(0));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let a = entry(&mut e, 0x500, 2, "Two");
        let same = entry(&mut e, 0x500, 2, "two");
        let other_set = entry(&mut e, 0x501, 2, "two");
        let other_name = entry(&mut e, 0x500, 2, "zwei");
        let no_name = entry(&mut e, 0x500, 2, "");
        let other_index = entry(&mut e, 0x500, 3, "two");
        let differs = |e: &mut Engine, mine: &[u32], theirs: &[u32]| {
            let this = model(e, mine);
            let other = model(e, theirs);
            e.call(0x0048_ac40, &args![this, other]).bool()
        };
        assert!(!differs(&mut e, &[a], &[same]));
        assert!(!differs(&mut e, &[a], &[no_name]));
        assert!(differs(&mut e, &[a], &[other_set]));
        assert!(differs(&mut e, &[a], &[other_name]));
        assert!(differs(&mut e, &[a], &[other_index]));
        assert!(differs(&mut e, &[a], &[same, other_index]));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(differs(&mut e, &[a], &[same]));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(MODEL_COMPARE_COMPONENT, |_, _| ret(0x100));
        assert!(!differs(&mut e, &[a], &[same]));
        e.register(MODEL_COMPARE_COMPONENT, |_, _| ret(1));
        assert!(differs(&mut e, &[a], &[same]));
    }

    #[test]
    fn fixed_strings_compare_ignoring_case() {
        let mut e = engine();
        e.register(NI_STRICMP, |e, a| {
            let (x, y) = (c_string(e, a[0]), c_string(e, a[1]));
            ret(u32::from(!x.eq_ignore_ascii_case(&y)))
        });
        let this = e.mem.alloc(4);
        let text = entry(&mut e, 0, 0, "Abc") + 8;
        let upper = entry(&mut e, 0, 0, "ABC") + 8;
        let other = entry(&mut e, 0, 0, "abd") + 8;
        let equal = |e: &mut Engine, other: u32| e.call(0x0048_af90, &args![this, other]).bool();
        assert!(equal(&mut e, 0));
        e.mem.set_u32(this, text);
        assert!(equal(&mut e, text));
        assert!(equal(&mut e, upper));
        assert!(!equal(&mut e, other));
        assert!(!equal(&mut e, 0));
    }
}
