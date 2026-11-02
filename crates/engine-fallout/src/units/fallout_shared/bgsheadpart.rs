//! `fallout shared/bgsheadpart.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSHeadPart` is form type 9; its vtable (`010464b4`, the RTTI names
//! `.?AVBGSHeadPart@@`) has `fn_005e7530` in the `Load` slot (+0x20). The
//! texture-swap model is at +0x24, the extra-part list at +0x48.

#[allow(unused_imports)]
use crate::prelude::*;

const GET_TES_FORM: u32 = 0x0047_2660;
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (the word at `file + 0x25c`).
const CHUNK_SIZE: u32 = 0x0040_1660;
const GET_CHUNK_DATA: u32 = 0x0047_2890;
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// A 16-bit value of the file (`00403570`), passed to the texture-swap
/// unpacker.
const FILE_WORD: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (engine map), cdecl: model, file.
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (engine map):
/// `__thiscall`, data, size, word.
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// The full-name reader (`00487050`, cdecl: name, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// Adds the word at the pointer argument to a list (`__thiscall`, `RET 4`).
const LIST_ADD: u32 = 0x005a_e3d0;
/// The form type of a head part record.
const FORM_TYPE_HEAD_PART: u32 = 9;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_HNAM: u32 = 0x4d41_4e48;

// Translated from 005e7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `part` to the extra-part list at `this + 0x48` (`005ae3d0` with
/// the address of the stack word). `__thiscall`, `RET 4`. The engine map
/// gives it no name.
pub fn fn_005e7790(e: &mut Engine, this: Ptr, part: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), part);
        e.call(LIST_ADD, &args![this.addr() + 0x48, slot]);
    });
}

// Translated from 005e7530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e7530 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSHeadPart`'s `Load` (vtable slot +0x20): `__thiscall`, `RET 4`,
/// `file`. Answers false when the record's form type is not 9; otherwise
/// reads the form header and every chunk and answers true:
/// - `EDID`: a stack buffer of the chunk's size (at most 0x200 read) for
///   virtual slot 0x134; `OBND`: virtual slot 0xe0 with the file;
/// - `FULL`: `00487050` on +0x18; `MODL`, `MODD`, `MOSD`, `MODT`:
///   `TESModel::LoadModelChunk` on +0x24 (each null when `this` is null);
/// - `MODS`: the chunk read into a stack buffer of its size and unpacked as
///   a texture swap on +0x24 with `00403570`'s word;
/// - `DATA`: one byte into +0x44; `HNAM`: an extra part id for
///   `fn_005e7790`.
///
/// The stack cookie check is not translated.
pub fn fn_005e7530(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let part = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != FORM_TYPE_HEAD_PART {
        return false;
    }
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(part, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(part, 0xe0, &args![file]);
            }
            CHUNK_FULL => {
                let name = if part == 0 { 0 } else { part + 0x18 };
                e.call(LOAD_FULL_NAME, &args![name, file]);
            }
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = if part == 0 { 0 } else { part + 0x24 };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    let word = e.call(FILE_WORD, &args![file]).u16();
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![part + 0x24, buffer, size, u32::from(word)],
                    );
                });
            }
            CHUNK_DATA => {
                e.call(GET_CHUNK_DATA, &args![file, part + 0x44, 1u32]);
            }
            CHUNK_HNAM => {
                let id = e.with_stack(4, |e, id| {
                    e.call(GET_CHUNK_WORD, &args![file, id]);
                    e.mem.u32(id.addr())
                });
                fn_005e7790(e, this, id);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005e77b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHeadPart::InitItem` (engine map): unless `004013e0` answers true,
/// resolves the texture swaps (`TESModelTextureSwap::InitItem` on +0x24)
/// and each extra-part id of the list (`00624700`): the id gets the file's
/// compile index (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), is
/// looked up (`004839c0`) and cast to `BGSHeadPart`; a found part replaces
/// the id in its node (`00726c60`), a missing one prints `Unable to find
/// extra part ...` and is taken out (after a good node: `00905330` on it
/// with the id, the walk going on with that node's next; at the head:
/// `0063f7b0` on the node, which stays the current node). Then
/// `00484ab0(1)`. `ECX`, no stack words.
pub fn bgs_head_part_init_item(e: &mut Engine, this: Ptr) {
    let part = this.addr();
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_aa80, &args![part + 0x24, this]);
    let mut node = e.call(0x0062_4700, &args![this]).u32();
    let mut previous = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let data = e.call(0x0068_15c0, &args![node]).u32();
        let raw = e.mem.u32(data);
        let (id, found) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), raw);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            let form = e.call(0x0048_39c0, &args![id]).u32();
            let found = e
                .call(
                    0x00ec_43fb,
                    &args![form, 0u32, 0x0118_3028u32, 0x0118_63b8u32, 0u32],
                )
                .u32();
            (id, found)
        });
        if found == 0 {
            let form_id = e.call(0x0084_e3a0, &args![this]).u32();
            let name = e.vcall(part, 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0104_65ecu32, id, name, form_id]);
            if previous != 0 {
                e.with_stack(4, |e, entry| {
                    e.mem.set_u32(entry.addr(), id);
                    e.call(0x0090_5330, &args![previous, entry]);
                });
                node = e.call(0x0072_6070, &args![previous]).u32();
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
        } else {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), found);
                e.call(0x0072_6c60, &args![node, slot]);
            });
            previous = node;
            node = e.call(0x0072_6070, &args![node]).u32();
        }
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 005e7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSHeadPart::Save` (engine map): starts the form (`004855a0`), saves
/// the full name (`00487010`, +0x18) and the texture-swap model
/// (`0048a520`, +0x24, with `MODL`, `MODT`, `MODS`), the byte +0x44 as
/// `DATA` (`004858f0`), each extra part (list `00624700`, nodes until
/// empty, the next node taken before) as its form id (`0084e3a0`) in an
/// `HNAM` chunk (`00485910`), then closes the form (`00485680`).
pub fn bgs_head_part_save(e: &mut Engine, this: Ptr) {
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![this.addr() + 0x18]);
    e.call(
        0x0048_a520,
        &args![this.addr() + 0x24, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    let flags = u32::from(e.mem.u8(this.addr() + 0x44));
    e.call(0x0048_58f0, &args![CHUNK_DATA, flags]);
    let mut node = e.call(0x0062_4700, &args![this]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let extra_part = e.mem.u32(item);
        node = e.call(0x0072_6070, &args![node]).u32();
        let id = e.call(0x0084_e3a0, &args![extra_part]).u32();
        e.call(0x0048_5910, &args![CHUNK_HNAM, id]);
    }
    e.call(0x0048_5680, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_7790, fn_005e7790(Ptr, u32)),
        entry!(0x005e_7530, fn_005e7530(Ptr, Ptr) -> bool),
        entry!(0x005e_77b0, bgs_head_part_init_item(Ptr)),
        entry!(0x005e_7480, bgs_head_part_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0200_0000;
    const SET_EDITOR_ID: u32 = 0x0300_0000;
    const LOAD_BOUNDS: u32 = 0x0300_0010;
    const NAME: u32 = 0x0300_0020;

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

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

    fn part(e: &mut Engine) -> u32 {
        e.map(VTABLE, 0x200);
        e.mem.set_u32(VTABLE + 0x134, SET_EDITOR_ID);
        e.mem.set_u32(VTABLE + 0xe0, LOAD_BOUNDS);
        e.mem.set_u32(VTABLE + 0x130, NAME);
        e.register(SET_EDITOR_ID, |_, _| Ret::default());
        e.register(LOAD_BOUNDS, |_, _| Ret::default());
        e.register(NAME, |_, _| ret(0x0aae));
        let part = e.mem.alloc(0x50);
        e.mem.set_u32(part, VTABLE);
        part
    }

    #[test]
    fn extra_parts_go_into_the_list_at_0x48() {
        let mut e = Engine::new();
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x005e_7790, &args![0x1000u32, 0x77u32]);
        assert_eq!(*added.borrow(), vec![(0x1048, 0x77)]);
    }

    fn load_engine(form_type: u32, chunks: &[(u32, &[u8])]) -> (Engine, u32, Ptr) {
        let mut e = Engine::new();
        let part = part(&mut e);
        let file: Ptr = Ptr::new(e.mem.alloc(0x300));
        let queue: Chunks = Rc::new(RefCell::new(
            chunks.iter().map(|(s, d)| (*s, d.to_vec())).collect(),
        ));
        e.register_double(GET_TES_FORM, move |_, _| ret(form_type));
        let q = queue.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.0))
        });
        let q = queue.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            ret(q.borrow().first().map_or(0, |c| c.1.len() as u32))
        });
        let q = queue.clone();
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = q.borrow().first().map(|c| c.1.clone()).unwrap_or_default();
            let n = (a[2] as usize).min(data.len());
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let q = queue.clone();
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            let data = q.borrow().first().map(|c| c.1.clone()).unwrap_or_default();
            e.mem.write(a[1], &data[..4.min(data.len())]);
            Ret::default()
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut q = queue.borrow_mut();
            q.remove(0);
            ret(u32::from(!q.is_empty()))
        });
        e.register(FILE_WORD, |_, _| ret(0x1_0005));
        for callee in [
            TES_FORM_LOAD_FORM,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
            LOAD_FULL_NAME,
            LIST_ADD,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, part, file)
    }

    #[test]
    fn load_needs_a_head_part_record() {
        let (mut e, part, file) = load_engine(8, &[(CHUNK_DATA, &[1])]);
        assert!(!e.call(0x005e_7530, &args![part, file]).bool());
        assert!(calls_to(&e, TES_FORM_LOAD_FORM).is_empty());
    }

    #[test]
    fn load_reads_every_chunk() {
        let chunks: [(u32, &[u8]); 10] = [
            (CHUNK_EDID, b"HairA\0"),
            (CHUNK_OBND, &[0; 12]),
            (CHUNK_FULL, b"x\0"),
            (CHUNK_MODL, b"m\0"),
            (CHUNK_MODD, &[0; 4]),
            (CHUNK_MOSD, &[0; 4]),
            (CHUNK_MODT, &[0; 4]),
            (CHUNK_MODS, &[1, 2, 3]),
            (CHUNK_DATA, &[9]),
            (CHUNK_HNAM, &[0x34, 0x12, 0, 0]),
        ];
        let (mut e, part, file) = load_engine(9, &chunks);
        let text = Rc::new(RefCell::new(vec![]));
        let seen = text.clone();
        e.register_double(SET_EDITOR_ID, move |e, a| {
            *seen.borrow_mut() = e.mem.cstr(a[1]);
            Ret::default()
        });
        let swapped = Rc::new(RefCell::new(vec![]));
        let seen = swapped.clone();
        e.register_double(UNPACK_TEXTURE_SWAP, move |e, a| {
            *seen.borrow_mut() = e.mem.bytes(a[1], a[2]);
            Ret::default()
        });
        assert!(e.call(0x005e_7530, &args![part, file]).bool());
        let f = file.addr();
        assert_eq!(calls_to(&e, TES_FORM_LOAD_FORM), vec![vec![part, f]]);
        assert_eq!(*text.borrow(), b"HairA");
        assert_eq!(*swapped.borrow(), vec![1, 2, 3]);
        assert_eq!(calls_to(&e, LOAD_BOUNDS), vec![vec![part, f]]);
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![part + 0x18, f]]);
        assert_eq!(
            calls_to(&e, LOAD_MODEL_CHUNK),
            vec![vec![part + 0x24, f]; 4]
        );
        let unpacks = calls_to(&e, UNPACK_TEXTURE_SWAP);
        assert_eq!(unpacks.len(), 1);
        assert_eq!(unpacks[0][0], part + 0x24);
        assert_eq!(&unpacks[0][2..], &[3, 5]);
        assert_eq!(e.mem.u8(part + 0x44), 9);
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds[0][0], part + 0x48);
    }

    #[test]
    fn a_null_part_reads_components_into_null() {
        let (mut e, _, file) = load_engine(9, &[(CHUNK_FULL, &[0]), (CHUNK_MODL, &[0])]);
        assert!(e.call(0x005e_7530, &args![0u32, file]).bool());
        assert_eq!(calls_to(&e, LOAD_FULL_NAME), vec![vec![0, file.addr()]]);
        assert_eq!(calls_to(&e, LOAD_MODEL_CHUNK), vec![vec![0, file.addr()]]);
    }

    fn init_engine(ids: &[u32]) -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        let part = part(&mut e);
        // Nodes `{data, next}` holding the ids.
        let mut nodes = vec![];
        let mut next = 0;
        for &id in ids.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, id);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.insert(0, node);
        }
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register_double(0x0062_4700, move |_, _| ret(next));
        e.register(0x0082_56d0, |_, _| ret(0));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0048_4e60, |_, _| ret(0x0f11e));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| ret(a[0]));
        // Ids with bit 0x10 set are missing.
        e.register(0x00ec_43fb, |_, a| {
            ret(if a[0] & 0x10 != 0 { 0 } else { a[0] + 1 })
        });
        e.register(0x0084_e3a0, |_, _| ret(0x0abc));
        // The removal after a good node unlinks the next node.
        e.register(0x0090_5330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(0x0072_6c60, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        for callee in [0x0048_aa80, 0x005b_5e40, 0x0063_f7b0, 0x0048_4ab0] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, part, nodes)
    }

    #[test]
    fn init_item_resolves_and_drops_extra_parts() {
        let (mut e, part, nodes) = init_engine(&[0x1, 0x12, 0x3]);
        e.call(0x005e_77b0, &args![part]);
        assert_eq!(calls_to(&e, 0x0048_aa80), vec![vec![part + 0x24, part]]);
        assert_eq!(e.mem.u32(nodes[0]), 0x0100_0002);
        assert_eq!(e.mem.u32(nodes[2]), 0x0100_0004);
        assert_eq!(
            calls_to(&e, 0x005b_5e40),
            vec![vec![0x0104_65ec, 0x0100_0012, 0x0aae, 0x0abc]]
        );
        let removed = calls_to(&e, 0x0090_5330);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], nodes[0]);
        assert!(calls_to(&e, 0x0063_f7b0).is_empty());
        let casts = calls_to(&e, 0x00ec_43fb);
        assert_eq!(casts.len(), 3);
        assert_eq!(casts[0], vec![0x0100_0001, 0, 0x0118_3028, 0x0118_63b8, 0]);
        assert_eq!(calls_to(&e, 0x0048_4ab0), vec![vec![part, 1]]);
    }

    #[test]
    fn a_missing_first_part_is_removed_at_the_head() {
        let (mut e, part, nodes) = init_engine(&[0x10]);
        // `0063f7b0` empties the head node, which then ends the walk.
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let ends = Rc::new(RefCell::new(0));
        let count = ends.clone();
        e.register_double(0x0082_56d0, move |_, _| {
            *count.borrow_mut() += 1;
            ret(u32::from(*count.borrow() > 1))
        });
        e.call(0x005e_77b0, &args![part]);
        assert_eq!(calls_to(&e, 0x0063_f7b0), vec![vec![nodes[0]]]);
        assert!(calls_to(&e, 0x0090_5330).is_empty());
    }

    #[test]
    fn init_item_does_nothing_when_told_so() {
        let (mut e, part, _) = init_engine(&[0x1]);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call(0x005e_77b0, &args![part]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    #[test]
    fn save_writes_model_data_and_each_extra_part() {
        let mut e = Engine::new();
        for callee in [
            0x0048_55a0,
            0x0048_7010,
            0x0048_a520,
            0x0048_58f0,
            0x0048_5910,
            0x0048_5680,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(0x0062_4700, |_, a| (a[0] + 0x48).into_ret());
        e.register(0x0082_56d0, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0084_e3a0, |_, a| (a[0] | 0x0100_0000).into_ret());
        let this: Ptr = Ptr::new(e.mem.alloc(0x60));
        e.mem.set_u8(this.addr() + 0x44, 0x83);
        let next = e.mem.alloc(8);
        e.mem.set_u32(next, 0x22);
        e.mem.set_u32(this.addr() + 0x48, 0x11);
        e.mem.set_u32(this.addr() + 0x4c, next);
        e.call_log = Some(vec![]);
        e.call(0x005e_7480, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| {
                ![
                    0x005e_7480,
                    0x0062_4700,
                    0x0082_56d0,
                    0x0068_15c0,
                    0x0072_6070,
                    0x0084_e3a0,
                ]
                .contains(&c.0)
            })
            .collect();
        let a = this.addr();
        assert_eq!(
            calls,
            vec![
                (0x0048_55a0, vec![a]),
                (0x0048_7010, vec![a + 0x18]),
                (
                    0x0048_a520,
                    vec![a + 0x24, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (0x0048_58f0, vec![CHUNK_DATA, 0x83]),
                (0x0048_5910, vec![CHUNK_HNAM, 0x0100_0011]),
                (0x0048_5910, vec![CHUNK_HNAM, 0x0100_0022]),
                (0x0048_5680, vec![a]),
            ]
        );
    }
}
