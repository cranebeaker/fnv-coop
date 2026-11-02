//! `fallout shared/bgsaddonnode.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESForm@@`, the cast's source type.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor `.?AVTESSound@@`, the cast's target type.
const TYPE_SOUND: u32 = 0x0118_32fc;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Unable to find sound (%08X) on addon node '%s'."`.
const MISSING_SOUND_FORMAT: u32 = 0x0102_4394;
/// `BGSAddonNode::iMasterParticleSystemCount` (Xbox PDB): the next master
/// particle system index the record reader hands out.
const MASTER_PARTICLE_SYSTEM_COUNT: u32 = 0x011c_8054;

/// `TESFile` chunk reading: the record's form type byte (`00472660`), the
/// current chunk's type (`004726b0`, 0 at the end), the step to the next
/// chunk (`004726f0`, false at the end), the chunk's size (`00401660`), the
/// chunk's data into a buffer (`004727f0`, one word) or with a size
/// (`00472890`, two words), whether the file needs its words swapped
/// (`00401680`), and the `u16` the texture-swap chunk passes on
/// (`00403570`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
const FILE_TEXTURE_SWAP_WORD: u32 = 0x0040_3570;

/// The chunk types the record reader handles (four characters,
/// little-endian).
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_SNAM: u32 = 0x4d41_4e53;

layout! {
    /// `BGSAddonNode` (Xbox PDB), 0x70 bytes on the Xbox; the PC `TESForm`
    /// is 0x10 smaller, so every field below sits 0x10 lower.
    pub struct BgsAddonNode: 0x60 {
        /// `iIndex` (Xbox PDB).
        0x50 iIndex: u32,
        /// `pSound` (Xbox PDB): `TESSound*`; a form id until `InitItem`.
        0x54 pSound: u32,
        /// `Data` (Xbox PDB): `ADDON_DATA`, four bytes.
        0x58 Data: u32,
        /// `iMasterParticleSystemIndex` (Xbox PDB).
        0x5C iMasterParticleSystemIndex: u32,
    }
}

/// The `TESModelTextureSwap` base inside the node (Xbox PDB +0x40, PC +0x30).
const MODEL_TEXTURE_SWAP: u32 = 0x30;

// Translated from 004f8c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// initializes the model (`TESModelTextureSwap::InitItem`, `0048aa80`),
/// resolves the sound form id at `pSound` (the id adjusted to the node's
/// file, then looked up and cast; a warning with the node's name, vtable
/// +0x130, when nothing is found) and calls `00484ab0(1)`.
pub fn bgs_addon_node_init_item(e: &mut Engine, this: Ptr<BgsAddonNode>) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_aa80, &args![this.addr() + MODEL_TEXTURE_SWAP, this]);
    let sound = e.get(this, BgsAddonNode::pSound);
    if sound != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), sound);
            let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_SOUND, 0u32],
            )
            .u32();
        e.set(this, BgsAddonNode::pSound, cast);
        if e.get(this, BgsAddonNode::pSound) == 0 {
            let name = e.vcall(this.addr(), 0x130, &args![]).u32();
            e.call(WARNING, &args![MISSING_SOUND_FORMAT, id, name]);
        }
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 004f8d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8d50 - the /GS stack cookie check (00ec408c) is not translated.
/// The node's record reader (`RET 4`: the `TESFile`): false unless the
/// record's form type byte is `'X'`; otherwise `TESForm::LoadForm`
/// (`00485110`) and each chunk in turn: `EDID` (vtable +0x134 with the
/// text), `OBND` (vtable +0xE0 with the file), `MODL`/`MODT`/`MODD`/`MOSD`
/// (`TESModel::LoadModelChunk`, `004892d0`), `MODS` (the texture swaps,
/// `0048a7b0`), `DATA` (`iIndex`), `SNAM` (the sound's form id) and `DNAM`
/// (`Data`, word-swapped by `00512810` when the file asks; then, if
/// `00448bf0` answers true, the next master particle system index).
/// Returns true.
pub fn fn_004f8d50(e: &mut Engine, this: Ptr<BgsAddonNode>, file: Ptr) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x58 {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODT | CHUNK_MODD | CHUNK_MOSD => {
                let model = if this.is_null() {
                    0
                } else {
                    this.addr() + MODEL_TEXTURE_SWAP
                };
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(this.addr(), 0x134, &args![text]);
                });
            }
            CHUNK_DATA => {
                e.call(FILE_CHUNK_DATA, &args![file, this.addr() + 0x50]);
            }
            CHUNK_OBND => {
                e.vcall(this.addr(), 0xe0, &args![file]);
            }
            CHUNK_MODS => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, data| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, data, size]);
                    let word = e.call(FILE_TEXTURE_SWAP_WORD, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![
                            this.addr() + MODEL_TEXTURE_SWAP,
                            data,
                            size,
                            u32::from(word)
                        ],
                    );
                });
            }
            CHUNK_DNAM => {
                e.call(
                    FILE_CHUNK_DATA_SIZED,
                    &args![file, this.addr() + 0x58, 4u32],
                );
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x0051_2810, &args![this.addr() + 0x58]);
                }
                if e.call(0x0044_8bf0, &args![this]).bool() {
                    let index = e.global::<u32>(MASTER_PARTICLE_SYSTEM_COUNT);
                    e.set(this, BgsAddonNode::iMasterParticleSystemIndex, index);
                    let count = e.global::<u32>(MASTER_PARTICLE_SYSTEM_COUNT);
                    e.set_global(MASTER_PARTICLE_SYSTEM_COUNT, count.wrapping_add(1));
                }
            }
            CHUNK_SNAM => {
                let id = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.set(this, BgsAddonNode::pSound, id);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB; `004855a0`, `00485680`,
/// `thiscall` on the form).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESModelTextureSwap::Save` (Xbox PDB name of `0048a520`; `RET 0xc`: the model,
/// texture and texture-swap chunk signatures).
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::__AddChunkData` (Xbox PDB name of `00485990`; `cdecl`: chunk signature,
/// address, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `00401500` (`cdecl`, no arguments, `AL`): whether saved data is byte-swapped.
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `00512810` (`ECX` = the four `ADDON_DATA` bytes): swaps their byte order.
const SWAP_ADDON_DATA: u32 = 0x0051_2810;
/// The size of the `ADDON_DATA` bytes saved in the `DNAM` chunk.
const ADDON_DATA_SIZE: u32 = 4;

// Translated from 004f8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode::Save` (Xbox PDB; `thiscall`, no stack word): starts the form record,
/// saves the model (`MODL`, `MODT`, `MODS`), the index (`DATA`), the sound's form id
/// (`SNAM`, only when a sound is set), then the four `ADDON_DATA` bytes (`DNAM`), byte
/// swapped before and swapped back after when `00401500` says so, and closes the record.
pub fn bgs_addon_node_save(e: &mut Engine, this: Ptr<BgsAddonNode>) {
    e.call(FORM_START_FORM, &args![this]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![
            this.addr() + MODEL_TEXTURE_SWAP,
            CHUNK_MODL,
            CHUNK_MODT,
            CHUNK_MODS
        ],
    );
    let index = e.get(this, BgsAddonNode::iIndex);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_DATA, index]);
    let sound = e.get(this, BgsAddonNode::pSound);
    if sound != 0 {
        let id = e.call(FORM_ID, &args![sound]).u32();
        e.call(ADD_CHUNK_WORD, &args![CHUNK_SNAM, id]);
    }
    let data = this.addr() + 0x58;
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_ADDON_DATA, &args![data]);
    }
    e.call(ADD_CHUNK_DATA, &args![CHUNK_DNAM, data, ADDON_DATA_SIZE]);
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_ADDON_DATA, &args![data]);
    }
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_8c90, bgs_addon_node_init_item(Ptr<BgsAddonNode>)),
        entry!(0x004f_8d50, fn_004f8d50(Ptr<BgsAddonNode>, Ptr) -> bool),
        entry!(0x004f_8fe0, bgs_addon_node_save(Ptr<BgsAddonNode>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A node with a vtable whose +0x130 (name), +0x134 (editor id) and
    /// +0xE0 (bounds) slots are doubles.
    fn node(e: &mut Engine) -> Ptr<BgsAddonNode> {
        let this: Ptr<BgsAddonNode> = e.new_object();
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0x130, 0x7000_0130);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(this.addr(), vtable);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    fn init_doubles(e: &mut Engine, found: bool) {
        quiet(e, &[0x0048_aa80, 0x0048_4ab0, WARNING]);
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        if found {
            e.register(LOOKUP_FORM, |_, a| ret(a[0] & 0xffff));
        } else {
            e.register(LOOKUP_FORM, |_, _| ret(0));
        }
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
    }

    #[test]
    fn init_item_resolves_the_sound_and_warns_when_missing() {
        let mut e = Engine::new();
        let this = node(&mut e);
        init_doubles(&mut e, true);
        e.set(this, BgsAddonNode::pSound, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x004f_8c90, &args![this]);
        assert_eq!(e.get(this, BgsAddonNode::pSound), 0x1234);
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![0x0100_1234]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x1234, 0, TYPE_TES_FORM, TYPE_SOUND, 0]]
        );
        assert_eq!(
            calls(&e, 0x0048_aa80),
            vec![vec![this.addr() + 0x30, this.addr()]]
        );
        assert!(calls(&e, WARNING).is_empty());
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this.addr(), 1]]);

        // Not found: null and a warning with the adjusted id and the name.
        let mut e = Engine::new();
        let this = node(&mut e);
        init_doubles(&mut e, false);
        e.set(this, BgsAddonNode::pSound, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x004f_8c90, &args![this]);
        assert_eq!(e.get(this, BgsAddonNode::pSound), 0);
        assert_eq!(
            calls(&e, WARNING),
            vec![vec![MISSING_SOUND_FORMAT, 0x0100_1234, 0x5555]]
        );

        // No sound: no lookup.
        let mut e = Engine::new();
        let this = node(&mut e);
        init_doubles(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x004f_8c90, &args![this]);
        assert!(calls(&e, LOOKUP_FORM).is_empty());
        assert_eq!(calls(&e, 0x0048_4ab0).len(), 1);

        // `004013e0` true: nothing else.
        let mut e = Engine::new();
        let this = node(&mut e);
        init_doubles(&mut e, true);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x004f_8c90, &args![this]);
        assert!(calls(&e, 0x0048_aa80).is_empty());
        assert!(calls(&e, 0x0048_4ab0).is_empty());
    }

    /// A file whose chunks are `chunks` (type, first data word), with form
    /// type byte `form_type` and endian swapping `swap`.
    fn file(e: &mut Engine, form_type: u32, chunks: Vec<(u32, u32)>, swap: bool) -> Ptr {
        let file = Ptr::new(e.mem.alloc(0x10));
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register(FILE_TEXTURE_SWAP_WORD, |_, _| ret(0x1_0007));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(e, &[0x0048_5110, 0x0048_92d0, 0x0048_a7b0, 0x0051_2810]);
        file
    }

    #[test]
    fn load_reads_each_chunk() {
        let mut e = Engine::new();
        let this = node(&mut e);
        e.map(0x011c_8000, 0x1000);
        e.set_global(MASTER_PARTICLE_SYSTEM_COUNT, 7u32);
        e.register(0x0044_8bf0, |_, _| ret(1));
        let f = file(
            &mut e,
            0x58,
            vec![
                (CHUNK_EDID, 0x6f6c),
                (CHUNK_OBND, 0),
                (CHUNK_MODL, 0),
                (CHUNK_MOSD, 0),
                (CHUNK_MODS, 0),
                (CHUNK_DATA, 3),
                (CHUNK_SNAM, 0x1234),
                (CHUNK_DNAM, 0x0102_0304),
                (0x5858_5858, 0),
            ],
            true,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8d50, &args![this, f]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this.addr(), f.addr()]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this.addr(), f.addr()]]);
        assert_eq!(
            calls(&e, 0x0048_92d0),
            vec![vec![this.addr() + 0x30, f.addr()]; 2]
        );
        let swaps = calls(&e, 0x0048_a7b0);
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0][0], this.addr() + 0x30);
        assert_eq!(&swaps[0][2..], &[8, 7]);
        assert_eq!(e.get(this, BgsAddonNode::iIndex), 3);
        assert_eq!(e.get(this, BgsAddonNode::pSound), 0x1234);
        assert_eq!(e.get(this, BgsAddonNode::Data), 0x0102_0304);
        assert_eq!(calls(&e, 0x0051_2810), vec![vec![this.addr() + 0x58]]);
        assert_eq!(e.get(this, BgsAddonNode::iMasterParticleSystemIndex), 7);
        assert_eq!(e.global::<u32>(MASTER_PARTICLE_SYSTEM_COUNT), 8);
    }

    #[test]
    fn load_refuses_other_records_and_keeps_the_count_without_a_master() {
        let mut e = Engine::new();
        let this = node(&mut e);
        let f = file(&mut e, 0x57, vec![(CHUNK_DATA, 3)], false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8d50, &args![this, f]).bool());
        assert!(calls(&e, 0x0048_5110).is_empty());

        let mut e = Engine::new();
        let this = node(&mut e);
        e.map(0x011c_8000, 0x1000);
        e.set_global(MASTER_PARTICLE_SYSTEM_COUNT, 7u32);
        e.register(0x0044_8bf0, |_, _| ret(0));
        let f = file(&mut e, 0x58, vec![(CHUNK_DNAM, 5)], false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8d50, &args![this, f]).bool());
        assert!(calls(&e, 0x0051_2810).is_empty());
        assert_eq!(e.get(this, BgsAddonNode::Data), 5);
        assert_eq!(e.get(this, BgsAddonNode::iMasterParticleSystemIndex), 0);
        assert_eq!(e.global::<u32>(MASTER_PARTICLE_SYSTEM_COUNT), 7);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn run(sound: u32, swap: bool) -> (u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            MODEL_TEXTURE_SWAP_SAVE,
            ADD_CHUNK_WORD,
            ADD_CHUNK_DATA,
            SWAP_ADDON_DATA,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register_double(SWAP_ON_SAVE, move |_, _| u32::from(swap).into_ret());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x50, 7);
        e.mem.set_u32(this + 0x54, sound);
        e.call_log = Some(vec![]);
        e.call(0x004f_8fe0, &args![this]);
        let log = e.call_log.take().unwrap();
        (
            this,
            log.into_iter().filter(|c| c.0 != SWAP_ON_SAVE).collect(),
        )
    }

    #[test]
    fn save_writes_the_chunks_in_order() {
        let (this, log) = run(0x9000, true);
        assert_eq!(
            log[1..],
            [
                (FORM_START_FORM, vec![this]),
                (
                    MODEL_TEXTURE_SWAP_SAVE,
                    vec![this + 0x30, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (ADD_CHUNK_WORD, vec![CHUNK_DATA, 7]),
                (FORM_ID, vec![0x9000]),
                (ADD_CHUNK_WORD, vec![CHUNK_SNAM, 0x9001]),
                (SWAP_ADDON_DATA, vec![this + 0x58]),
                (ADD_CHUNK_DATA, vec![CHUNK_DNAM, this + 0x58, 4]),
                (SWAP_ADDON_DATA, vec![this + 0x58]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }

    #[test]
    fn save_skips_the_sound_and_the_swaps() {
        let (this, log) = run(0, false);
        assert_eq!(
            log[4..],
            [
                (ADD_CHUNK_DATA, vec![CHUNK_DNAM, this + 0x58, 4]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
        assert!(!log.iter().any(|c| c.0 == FORM_ID));
    }
}
