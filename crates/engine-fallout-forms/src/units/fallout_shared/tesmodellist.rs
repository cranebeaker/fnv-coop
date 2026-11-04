//! `fallout shared/tesmodellist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008256d0` (thiscall on a list node): whether the list is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (thiscall on a list node): the address of the node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (thiscall on a list node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00905820` (thiscall on a list, `RET 4`: item pointer): appends the item.
const LIST_APPEND: u32 = 0x0090_5820;
/// `00401000` (cdecl: size): allocates.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (cdecl: block): frees.
const FREE: u32 = 0x0040_1030;
/// `0044a670` (cdecl): `strlen`.
const STRLEN: u32 = 0x0044_a670;
/// `0048a010` (thiscall): empties the texture-list array (+0x0C of the model list).
const TEXTURE_ARRAY_CLEAR: u32 = 0x0048_a010;
/// `00488fd0` (thiscall): the constructor of an 8-byte texture-list entry.
const TEXTURE_ENTRY_CONSTRUCT: u32 = 0x0048_8fd0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `TESForm` (`01183028`), `BaseFormComponent`
/// (`01183040`), `TESModelList` (`01186b14`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_MODEL_LIST: u32 = 0x0118_6b14;

const CHUNK_NIFZ: u32 = 0x5a46_494e;
const CHUNK_NIFT: u32 = 0x5446_494e;

// Translated from 00489f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the model list's name list (+0x04) holds `name` (case-insensitive
/// compare `00404dc0` gives 0): false for an empty list or a null name.
pub fn fn_00489f20(e: &mut Engine, this: Ptr, name: u32) -> bool {
    let list = this.addr() + 4;
    if e.call(LIST_IS_EMPTY, &args![list]).bool() || name == 0 {
        return false;
    }
    let mut node = list;
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let text = e.mem.u32(item);
        if text != 0 && e.call(0x0040_4dc0, &args![text, name]).u32() == 0 {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 00489fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::AddNIFFile` (Xbox PDB, `RET 4`: the file name): unless the
/// list holds it already (`00489f20`), a copy of the name (`strlen + 1` bytes,
/// `00401000`, copied by `00406d30` with the size) is appended to the name
/// list at +0x04 (`00905820`).
pub fn tes_model_list_add_nif_file(e: &mut Engine, this: Ptr, name: u32) {
    if fn_00489f20(e, this, name) {
        return;
    }
    let size = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
    let copy = e.call(ALLOCATE, &args![size]).u32();
    e.call(0x0040_6d30, &args![copy, size, name]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), copy);
        e.call(LIST_APPEND, &args![this.addr() + 4, slot]);
    });
}

// Translated from 00489b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::ClearDataComponent` (Xbox PDB): removes and frees each name
/// of the list at +0x04, then empties the texture-list array at +0x0C
/// (`0048a010`).
pub fn tes_model_list_clear_data_component(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 4;
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let item = e.call(NODE_ITEM, &args![list]).u32();
        let name = e.mem.u32(item);
        e.call(NODE_REMOVE, &args![list]);
        e.call(FREE, &args![name]);
    }
    e.call(TEXTURE_ARRAY_CLEAR, &args![this.addr() + 0xc]);
}

// Translated from 00489ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::LoadModelListChunk` (Xbox PDB, `RET 8`: the model list
/// the texture lists belong to, the file): nothing unless both are given. `NIFZ`:
/// the chunk (its size from `00401660`, a block from `00401000`, read with
/// size 0) holds names one after the other up to an empty one, each added
/// with `AddNIFFile`; the block is then freed. `NIFT`: the texture lists
/// (`0048a0d0` on +0x0C with that model list and the file).
pub fn tes_model_list_load_model_list_chunk(e: &mut Engine, this: Ptr, owner: u32, file: Ptr) {
    if file.addr() == 0 || owner == 0 {
        return;
    }
    match e.call(0x0047_26b0, &args![file]).u32() {
        CHUNK_NIFT => {
            fn_0048a0d0(e, Ptr::new(this.addr() + 0xc), owner, file);
        }
        CHUNK_NIFZ => {
            let size = e.call(0x0040_1660, &args![file]).u32();
            let block = e.call(ALLOCATE, &args![size]).u32();
            e.call(0x0047_2890, &args![file, block, 0u32]);
            let mut name = block;
            while e.mem.u8(name) as i8 != 0 {
                tes_model_list_add_nif_file(e, this, name);
                let length = e.call(STRLEN, &args![name]).u32();
                name = name.wrapping_add(length).wrapping_add(1);
            }
            e.call(FREE, &args![block]);
        }
        _ => {}
    }
}

// Translated from 00489db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::CopyComponent` (Xbox PDB, `RET 4`: the other component):
/// when the other component is a `TESModelList`, clears this one (vtable
/// slot 0x04), adds the other's names in order (`AddNIFFile`, skipping null
/// items) and copies its texture lists (`0048a2c0` on +0x0C with the other's
/// +0x0C from `0048d150`).
pub fn tes_model_list_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_MODEL_LIST,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    e.vcall(this.addr(), 0x4, &[]);
    let mut node = source + 4;
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let name = e.mem.u32(item);
        if name != 0 {
            tes_model_list_add_nif_file(e, this, name);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let textures = e.call(0x0048_d150, &args![source]).u32();
    fn_0048a2c0(e, Ptr::new(this.addr() + 0xc), textures);
}

/// A new 8-byte texture-list entry (`00401000`, `00488fd0`), or 0 when the
/// allocation fails.
fn new_texture_entry(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block != 0 {
        e.call(TEXTURE_ENTRY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

/// A new array of `count` words (`00401000`; the size is all ones when
/// `count * 4` overflows, `MUL`/`SETO`/`NEG`/`OR`).
fn new_word_array(e: &mut Engine, count: u32) -> u32 {
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    e.call(ALLOCATE, &args![size]).u32()
}

// Translated from 0048a0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a0d0 - the exception-unwinding frame (FS:[0], frees an entry whose constructor throws) and the security cookie check (`__security_check_cookie`, 00ec408c) are not translated; the alloca chunk buffer is a heap block.
/// Loads the texture-list array (`this`: count at +0, array at +4; `RET 8`:
/// the owning model list, the file) from the `NIFT` chunk: empties it
/// (`0048a010`), reads the chunk into a buffer its size, takes the count from
/// its first 4 bytes (`00401460`; byte-swapped by `00401080` with 0 when the
/// file swaps), allocates the array and, for each entry, a new 8-byte entry
/// (`00488fd0`; 0 if the allocation fails) loaded by
/// `TESTextureList::LoadFromBuffer` (`00489670`) from the bytes after a one-byte
/// entry count, with that count, the owner cast to `TESForm` and the matching
/// name of the owner's name list (`00717e50`; 0 past its end); each entry
/// takes 1 + 0x18 * count bytes.
pub fn fn_0048a0d0(e: &mut Engine, this: Ptr, owner: u32, file: Ptr) {
    let base = this.addr();
    e.call(TEXTURE_ARRAY_CLEAR, &args![this]);
    let size = e.call(0x0040_1660, &args![file]).u32();
    let buffer = e.mem.alloc(size.max(1));
    e.call(0x0047_2890, &args![file, buffer, size]);
    e.call(0x0040_1460, &args![this, buffer, 4u32]);
    if e.call(0x0040_1680, &args![file]).bool() {
        e.call(0x0040_1080, &args![this, 0u32]);
    }
    let mut offset = 4u32;
    let count = e.mem.u32(base);
    let array = new_word_array(e, count);
    e.mem.set_u32(base + 4, array);
    let mut node = e.call(0x0071_7e50, &args![owner]).u32();
    let mut index = 0u32;
    while index < e.mem.u32(base) {
        let mut name = 0u32;
        if node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            name = e.mem.u32(item);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
        let entry = new_texture_entry(e);
        let array = e.mem.u32(base + 4);
        e.mem
            .set_u32(array.wrapping_add(index.wrapping_mul(4)), entry);
        let textures = e.mem.u8(buffer.wrapping_add(offset));
        offset += 1;
        let form = e
            .call(
                RT_DYNAMIC_CAST,
                &args![owner, 0u32, TYPE_TES_MODEL_LIST, TYPE_TES_FORM, 0u32],
            )
            .u32();
        let array = e.mem.u32(base + 4);
        let entry = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
        e.call(
            0x0048_9670,
            &args![
                entry,
                buffer.wrapping_add(offset),
                u32::from(textures),
                form,
                name
            ],
        );
        offset = (u32::from(textures) * 0x18).wrapping_add(offset);
        index += 1;
    }
    e.mem.free(buffer);
}

// Translated from 0048a2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048a2c0 - the exception-unwinding frame (FS:[0], frees an entry whose constructor throws) is not translated.
/// Copies the texture-list array `other` (count at +0, array at +4; `RET 4`)
/// into `this`: empties this one (`0048a010`); for a nonzero count, takes it,
/// allocates the array and for each entry a new 8-byte entry (`00488fd0`; 0
/// if the allocation fails) copied from the other's entry (`004899f0`).
pub fn fn_0048a2c0(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    e.call(TEXTURE_ARRAY_CLEAR, &args![this]);
    if e.mem.u32(other) == 0 {
        return;
    }
    let count = e.mem.u32(other);
    e.mem.set_u32(base, count);
    let array = new_word_array(e, count);
    e.mem.set_u32(base + 4, array);
    let mut index = 0u32;
    while index < e.mem.u32(base) {
        let entry = new_texture_entry(e);
        let array = e.mem.u32(base + 4);
        e.mem
            .set_u32(array.wrapping_add(index.wrapping_mul(4)), entry);
        let source = e
            .mem
            .u32(e.mem.u32(other + 4).wrapping_add(index.wrapping_mul(4)));
        let entry = e
            .mem
            .u32(e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(4)));
        e.call(0x0048_99f0, &args![entry, source]);
        index += 1;
    }
}

/// `TESForm::AddChunkArray` (`004856f0`, cdecl(chunk, data, size)).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `strcpy_s(dest, size, source)` (wrapper `00406d30`, cdecl).
const STRCPY_S: u32 = 0x0040_6d30;
/// `004534f0` (thiscall, `RET 4`): the texture lists' save step; its body
/// does nothing in this build.
const TEXTURE_ARRAY_SAVE: u32 = 0x0045_34f0;
/// `_stricmp` (wrapper `00404dc0`, cdecl).
const STRICMP: u32 = 0x0040_4dc0;

// Translated from 00489ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::Save` (Xbox PDB, `RET 8`): unless the name list (+0x04) is
/// empty, packs its names (null items skipped) one after another, each with
/// its terminator, plus one final zero byte, into a block of exactly that
/// size (`00401000`; `strcpy_s` with the room left), adds it as `chunk`
/// (`TESForm::AddChunkArray`) and frees it. Then always runs `004534f0` on
/// the texture lists (+0x0C) with `texture_chunk`.
pub fn tes_model_list_save(e: &mut Engine, this: Ptr, chunk: u32, texture_chunk: u32) {
    let list = this.addr() + 4;
    if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let mut size = 1u32;
        let mut node = list;
        while node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let name = e.mem.u32(item);
            if name != 0 {
                let length = e.call(STRLEN, &args![name]).u32();
                size = size.wrapping_add(length).wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let buffer = e.call(ALLOCATE, &args![size]).u32();
        let mut out = buffer;
        let mut room = size;
        node = list;
        while node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let name = e.mem.u32(item);
            if name != 0 {
                e.call(STRCPY_S, &args![out, room, name]);
                let step = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
                out = out.wrapping_add(step);
                room = room.wrapping_sub(step);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.mem.set_u8(out, 0);
        e.call(ADD_CHUNK_ARRAY, &args![chunk, buffer, size]);
        e.call(FREE, &args![buffer]);
    }
    e.call(TEXTURE_ARRAY_SAVE, &args![this.addr() + 0xc, texture_chunk]);
}

// Translated from 00489e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelList::CompareComponent` (Xbox PDB, `RET 4`): true when the two
/// differ: when `other` is no `TESModelList`; when, walking both name lists
/// (+0x04) side by side, one item has a name and the other not, or both
/// names differ ignoring case (`_stricmp`); when one list is longer.
pub fn tes_model_list_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_MODEL_LIST,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    let mut mine = this.addr() + 4;
    let mut theirs = source + 4;
    while mine != 0 && theirs != 0 {
        let item = e.call(NODE_ITEM, &args![mine]).u32();
        let a = e.mem.u32(item);
        let item = e.call(NODE_ITEM, &args![theirs]).u32();
        let b = e.mem.u32(item);
        if (a == 0 && b != 0) || (a != 0 && b == 0) {
            return true;
        }
        if a != 0 && b != 0 && e.call(STRICMP, &args![a, b]).u32() != 0 {
            return true;
        }
        mine = e.call(NODE_NEXT, &args![mine]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
    mine != 0 || theirs != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_9f20, fn_00489f20(Ptr, u32) -> bool),
        entry!(0x0048_9fa0, tes_model_list_add_nif_file(Ptr, u32)),
        entry!(0x0048_9b40, tes_model_list_clear_data_component(Ptr)),
        entry!(
            0x0048_9ce0,
            tes_model_list_load_model_list_chunk(Ptr, u32, Ptr)
        ),
        entry!(0x0048_9db0, tes_model_list_copy_component(Ptr, u32)),
        entry!(0x0048_a0d0, fn_0048a0d0(Ptr, u32, Ptr)),
        entry!(0x0048_a2c0, fn_0048a2c0(Ptr, u32)),
        entry!(0x0048_9ba0, tes_model_list_save(Ptr, u32, u32)),
        entry!(
            0x0048_9e40,
            tes_model_list_compare_component(Ptr, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

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

    fn put_string(e: &mut Engine, text: &str) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(p + i as u32, b);
        }
        p
    }

    /// List, string and allocator doubles (a node is two words: item, next).
    fn model_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0082_56d0, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(0x0063_f7b0, |e, a| {
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
        e.register(0x0090_5820, |e, a| {
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
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0040_1030, ok);
        e.register(0x0044_a670, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            ret(n)
        });
        e.register(0x0040_6d30, |e, a| {
            for i in 0..a[1] {
                let b = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(0x0040_4dc0, |e, a| {
            let (x, y) = (c_string(e, a[0]), c_string(e, a[1]));
            ret(u32::from(!x.eq_ignore_ascii_case(&y)))
        });
        e.register(0x0048_a010, ok);
        e
    }

    fn names(e: &Engine, head: u32) -> Vec<String> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(c_string(e, e.mem.u32(node)));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn at_00489f20_finds_a_name_ignoring_case() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x14);
        let name = put_string(&mut e, "Meshes\\A.nif");
        assert!(!e.call(0x0048_9f20, &[this, name]).bool());
        let a = put_string(&mut e, "x.nif");
        let b = put_string(&mut e, "meshes\\a.NIF");
        e.mem.set_u32(this + 4, a);
        let second = e.mem.alloc(8);
        e.mem.set_u32(this + 8, second);
        e.mem.set_u32(second, b);
        assert!(e.call(0x0048_9f20, &[this, name]).bool());
        assert!(!e.call(0x0048_9f20, &[this, 0]).bool());
        let other = put_string(&mut e, "y.nif");
        assert!(!e.call(0x0048_9f20, &[this, other]).bool());
    }

    #[test]
    fn at_00489fa0_adds_a_copy_once() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x14);
        let a = put_string(&mut e, "a.nif");
        let b = put_string(&mut e, "B.nif");
        e.call(0x0048_9fa0, &[this, a]);
        e.call(0x0048_9fa0, &[this, b]);
        e.call_log = Some(vec![]);
        let again = put_string(&mut e, "b.NIF");
        e.call(0x0048_9fa0, &[this, again]);
        assert!(only(&e.call_log.take().unwrap(), 0x0040_1000).is_empty());
        assert_eq!(names(&e, this + 4), vec!["a.nif", "B.nif"]);
        // A copy, not the caller's string.
        assert_ne!(e.mem.u32(this + 4), a);
    }

    #[test]
    fn at_00489b40_frees_each_name_and_clears_the_textures() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x14);
        for name in ["a", "b", "c"] {
            let p = put_string(&mut e, name);
            e.call(0x0048_9fa0, &[this, p]);
        }
        let first = e.mem.u32(this + 4);
        e.call_log = Some(vec![]);
        e.call(0x0048_9b40, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(names(&e, this + 4).is_empty());
        let freed = only(&log, 0x0040_1030);
        assert_eq!(freed.len(), 3);
        assert_eq!(freed[0], vec![first]);
        assert_eq!(only(&log, 0x0048_a010), vec![vec![this + 0xc]]);
    }

    #[test]
    fn at_00489ce0_loads_names_or_texture_lists() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x14);
        let file = e.mem.alloc(0x300);
        // Without a file or an owner: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_9ce0, &[this, 0, file]);
        e.call(0x0048_9ce0, &[this, 0x40, 0]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        // NIFZ: "a\0b\0\0".
        e.register(0x0047_26b0, |_, _| ret(0x5a46_494e));
        e.register(0x0040_1660, |_, _| ret(5));
        e.register(0x0047_2890, |e, a| {
            for (i, b) in b"a\0b\0\0".iter().enumerate() {
                e.mem.set_u8(a[1] + i as u32, *b);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_9ce0, &[this, 0x40, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(names(&e, this + 4), vec!["a", "b"]);
        let block = only(&log, 0x0040_1000)[0].clone();
        assert_eq!(block, vec![5]);
        let read = only(&log, 0x0047_2890);
        assert_eq!(read[0][2], 0);
        assert_eq!(only(&log, 0x0040_1030), vec![vec![read[0][1]]]);

        // NIFT: the texture lists on +0x0C (0048a0d0 runs; its count is 0).
        e.register(0x0047_26b0, |_, _| ret(0x5446_494e));
        e.register(0x0040_1460, ok);
        e.register(0x0040_1680, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0048_9ce0, &[this, 0x40, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0048_a010), vec![vec![this + 0xc]]);
        assert_eq!(only(&log, 0x0071_7e50), vec![vec![0x40]]);

        // Another chunk: nothing.
        e.register(0x0047_26b0, |_, _| ret(0x4c44_4f4d));
        e.call_log = Some(vec![]);
        e.call(0x0048_9ce0, &[this, 0x40, file]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn at_00489db0_copies_names_and_textures() {
        let mut e = model_engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        for name in ["x", "y"] {
            let p = put_string(&mut e, name);
            e.call(0x0048_9fa0, &[other, p]);
        }
        e.put_vtable(0x0090_0000, &[0x00a0_0004; 4]);
        e.mem.set_u32(this, 0x0090_0000);
        e.register(0x00a0_0004, ok);
        e.register_double(0x00ec_43fb, move |_, a| {
            ret(if a[0] == other { other } else { 0 })
        });
        e.register(0x0048_d150, |_, a| ret(a[0] + 0xc));
        e.call_log = Some(vec![]);
        e.call(0x0048_9db0, &[this, 0x1234]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x0048_9db0, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![vec![other, 0, 0x0118_3040, 0x0118_6b14, 0]]
        );
        assert_eq!(only(&log, 0x00a0_0004), vec![vec![this]]);
        assert_eq!(names(&e, this + 4), vec!["x", "y"]);
        // 0048a2c0 runs with the other's +0x0C (count 0: only the clear).
        assert_eq!(only(&log, 0x0048_a010), vec![vec![this + 0xc]]);
    }

    #[test]
    fn at_0048a0d0_loads_each_texture_list() {
        let mut e = model_engine();
        let array = e.mem.alloc(8);
        let owner = e.mem.alloc(0x14);
        for name in ["one", "two"] {
            let p = put_string(&mut e, name);
            e.call(0x0048_9fa0, &[owner, p]);
        }
        let file = e.mem.alloc(0x300);
        // Count 3; entry counts 1, 0, 2 (0x18 bytes each).
        let mut chunk = vec![3u8, 0, 0, 0, 1];
        chunk.extend([0xaa; 0x18]);
        chunk.push(0);
        chunk.push(2);
        let size = chunk.len() as u32;
        e.register_double(0x0040_1660, move |_, _| ret(size));
        e.register_double(0x0047_2890, move |e, a| {
            for (i, b) in chunk.iter().enumerate() {
                e.mem.set_u8(a[1] + i as u32, *b);
            }
            Ret::default()
        });
        e.register(0x0040_1460, |e, a| {
            let w = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], w);
            Ret::default()
        });
        e.register(0x0040_1680, |_, _| ret(1));
        e.register(0x0040_1080, ok);
        e.register(0x0048_8fd0, |_, a| ret(a[0]));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + 0x100));
        // The exe steps past the last name node too (`00726070` on null); the
        // double answers null there.
        e.register(0x0072_6070, |e, a| {
            ret(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 4) })
        });
        e.register(0x0048_9670, ok);
        e.call_log = Some(vec![]);
        e.call(0x0048_a0d0, &[array, owner, file]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array), 3);
        assert_eq!(only(&log, 0x0040_1080), vec![vec![array, 0]]);
        assert_eq!(only(&log, 0x0040_1000)[0..1], [vec![12]]);
        let table = e.mem.u32(array + 4);
        let loads = only(&log, 0x0048_9670);
        assert_eq!(loads.len(), 3);
        let buffer = only(&log, 0x0047_2890)[0][1];
        let first_name = e.mem.u32(owner + 4);
        assert_eq!(
            loads[0],
            vec![e.mem.u32(table), buffer + 5, 1, owner + 0x100, first_name]
        );
        assert_eq!(loads[1][1..3], [buffer + 0x1e, 0]);
        assert_eq!(loads[2][1..3], [buffer + 0x1f, 2]);
        // Past the end of the names: 0.
        assert_eq!(loads[2][4], 0);
        assert_eq!(
            only(&log, 0x00ec_43fb)[0],
            vec![owner, 0, 0x0118_6b14, 0x0118_3028, 0]
        );
        for i in 0..3 {
            assert_ne!(e.mem.u32(table + 4 * i), 0);
        }
    }

    #[test]
    fn at_0048a2c0_copies_the_array() {
        let mut e = model_engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.register(0x0048_8fd0, |_, a| ret(a[0]));
        e.register(0x0048_99f0, ok);
        // An empty source: only the clear.
        e.call_log = Some(vec![]);
        e.call(0x0048_a2c0, &[this, other]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x51);
        e.mem.set_u32(source + 4, 0x52);
        e.mem.set_u32(other, 2);
        e.mem.set_u32(other + 4, source);
        // The second entry's allocation fails: null.
        let mut calls = 0;
        e.register_double(0x0040_1000, move |e, a| {
            calls += 1;
            ret(if calls == 3 { 0 } else { e.mem.alloc(a[0]) })
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_a2c0, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 2);
        let table = e.mem.u32(this + 4);
        assert_eq!(only(&log, 0x0040_1000)[0], vec![8]);
        let first = e.mem.u32(table);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(table + 4), 0);
        assert_eq!(
            only(&log, 0x0048_99f0),
            vec![vec![first, 0x51], vec![0, 0x52]]
        );
    }
}

#[cfg(test)]
mod save_compare_tests {
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

    fn put_string(e: &mut Engine, text: &str) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        for (i, b) in text.bytes().enumerate() {
            e.mem.set_u8(p + i as u32, b);
        }
        p
    }

    fn c_string(e: &Engine, mut p: u32) -> String {
        let mut s = String::new();
        while e.mem.u8(p) != 0 {
            s.push(e.mem.u8(p) as char);
            p += 1;
        }
        s
    }

    /// List, string and allocator doubles (a node is two words: item,
    /// next).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE, |_, _| Ret::default());
        e.register(STRLEN, |e, a| ret(c_string(e, a[0]).len() as u32));
        e.register(STRCPY_S, |e, a| {
            let text = c_string(e, a[2]);
            assert!(text.len() < a[1] as usize);
            for (i, b) in text.bytes().chain([0]).enumerate() {
                e.mem.set_u8(a[0] + i as u32, b);
            }
            Ret::default()
        });
        e.register(STRICMP, |e, a| {
            let (x, y) = (c_string(e, a[0]), c_string(e, a[1]));
            ret(u32::from(!x.eq_ignore_ascii_case(&y)))
        });
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.register(TEXTURE_ARRAY_SAVE, |_, _| Ret::default());
        e
    }

    /// A model list (0x14 bytes) whose name list holds `items` (0 = a null
    /// item).
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x14);
        let mut node = this + 4;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn save_packs_the_names_and_saves_the_textures() {
        let mut e = engine();
        let a = put_string(&mut e, "a.nif");
        let b = put_string(&mut e, "bc.nif");
        let this = list(&mut e, &[a, 0, b]);
        e.call_log = Some(vec![]);
        e.call(0x0048_9ba0, &args![this, 0x4c44_4f4du32, 0x5446_494eu32]);
        let log = e.call_log.take().unwrap();
        let added = only(&log, ADD_CHUNK_ARRAY);
        assert_eq!(added.len(), 1);
        let (chunk, buffer, size) = (added[0][0], added[0][1], added[0][2]);
        assert_eq!((chunk, size), (0x4c44_4f4d, 14));
        let bytes: Vec<u8> = (0..size).map(|i| e.mem.u8(buffer + i)).collect();
        assert_eq!(bytes, b"a.nif\0bc.nif\0\0");
        assert_eq!(only(&log, FREE), vec![vec![buffer]]);
        assert_eq!(
            only(&log, TEXTURE_ARRAY_SAVE),
            vec![vec![this + 0xc, 0x5446_494e]]
        );
    }

    #[test]
    fn save_of_an_empty_list_only_saves_the_textures() {
        let mut e = engine();
        let this = list(&mut e, &[0]);
        e.call_log = Some(vec![]);
        e.call(0x0048_9ba0, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, ADD_CHUNK_ARRAY).is_empty());
        assert_eq!(only(&log, TEXTURE_ARRAY_SAVE), vec![vec![this + 0xc, 2]]);
    }

    fn compare(e: &mut Engine, mine: &[u32], theirs: &[u32]) -> bool {
        let this = list(e, mine);
        let other = list(e, theirs);
        e.call(0x0048_9e40, &args![this, other]).bool()
    }

    #[test]
    fn compare_reports_each_difference() {
        let mut e = engine();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let a = put_string(&mut e, "a.nif");
        let upper = put_string(&mut e, "A.NIF");
        let b = put_string(&mut e, "b.nif");
        assert!(!compare(&mut e, &[a, 0], &[upper, 0]));
        assert!(compare(&mut e, &[a], &[b]));
        assert!(compare(&mut e, &[a], &[0]));
        assert!(compare(&mut e, &[0], &[a]));
        assert!(compare(&mut e, &[a, b], &[a]));
        assert!(compare(&mut e, &[a], &[a, b]));
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(compare(&mut e, &[a], &[a]));
    }
}
