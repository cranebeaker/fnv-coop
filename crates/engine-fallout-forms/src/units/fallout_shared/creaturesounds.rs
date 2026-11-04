//! `fallout shared/creaturesounds.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CreatureSounds` (Xbox PDB), 0x58 bytes: 22 list pointers, one per
    /// sound type. The PC code indexes the same 22 words from +0.
    pub struct CreatureSounds: 0x58 {
        /// `pSoundListArray[0]` (Xbox PDB): `BSSimpleList<CREATURE_SOUND *> *`;
        /// type `t` is at `+4 * t`.
        0x00 pSoundListArray: u32,
    }

    /// `CREATURE_SOUND` (Xbox PDB), 8 bytes.
    pub struct CreatureSound: 0x08 {
        /// `pSound` (Xbox PDB): `TESSound *`.
        0x00 pSound: u32,
        /// `iChance` (Xbox PDB).
        0x04 iChance: u8,
    }
}

/// `TESFile::GetTESChunk` (`004726b0`, `thiscall`): the current chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (`004727f0`, `thiscall`, `RET 4`): reads the chunk
/// into the given address.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` overload (`00472890`, `thiscall`, `RET 8`): reads
/// at most the given count of bytes into the given address.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `004726f0` (`thiscall` on the file): moves to the next chunk.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESForm::AddCompileIndex` (`00485d50`, `cdecl`: the form id's address,
/// the file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors the cast passes (source and target type).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_32fc;
/// `0084e3a0` (`thiscall`, no stack words): the word at +0xc of the creature
/// (`TESForm::iFormID`), printed as the second `%08X` of the warning.
const FORM_WORD_FOR_WARNING: u32 = 0x0084_e3a0;
/// Byte offset of the creature's vtable slot whose result the warning prints
/// as `%s` (no stack words).
const NAME_FOR_WARNING_SLOT: u32 = 0x130;
/// `005b5e40` (`cdecl`, variable): prints a warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// The warning's format string, "AUDIO: Unable to find CreatureSound ID
/// (%08X) for creature '%s' (%08X)." (`010475a0`).
const UNKNOWN_SOUND_FORMAT: u32 = 0x0104_75a0;
/// `005e3570` (`cdecl`, one word): the sound for a file name.
const SOUND_FOR_FILE_NAME: u32 = 0x005e_3570;
/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `005f0900` (`thiscall`, `RET 8`): adds a `CREATURE_SOUND` to a type's list.
const ADD_SOUND: u32 = 0x005f_0900;
/// The chunk tags `CSDI`, `CSDF` and `CSDC`.
const CHUNK_CSDI: u32 = 0x4944_5343;
const CHUNK_CSDF: u32 = 0x4644_5343;
const CHUNK_CSDC: u32 = 0x4344_5343;

// Translated from 005f0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreatureSounds::LoadCreatureSound` (Xbox PDB), `thiscall`, `RET 0xc`: the
/// file, the sound type, the creature (used for the warning). Without a file
/// it answers false. A `CSDI` chunk (a sound form id) gives a new 8-byte
/// `CREATURE_SOUND` (`00401000`): its sound is the id's form (compile index
/// added, looked up and dynamically cast) or null for id 0; a failed
/// lookup prints the warning with the id, the creature's name (vtable
/// `+0x130`) and its form id (`0084e3a0`); then the next chunk. A
/// `CSDF` chunk (a file name of up to 0x104 bytes) gives one whose sound is
/// `005e3570` of the name, then the next chunk. With a `CREATURE_SOUND`
/// made, a following `CSDC` chunk reads its one-byte chance (+4), adds it to
/// the type's list (`005f0900`) and answers true; otherwise it is freed
/// (`00401030`) and the answer is false.
// Translation incomplete: 005f0480 - the /GS stack cookie check (00ec408c) is not translated.
pub fn creature_sounds_load_creature_sound(
    e: &mut Engine,
    this: Ptr<CreatureSounds>,
    file: Ptr,
    sound_type: u32,
    creature: Ptr,
) -> bool {
    let mut loaded = false;
    if file.is_null() {
        return loaded;
    }
    let mut sound: u32 = 0;
    if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_CSDI {
        e.with_stack(4, |e, form_id| {
            e.call(GET_CHUNK_DATA, &args![file, form_id]);
            sound = e.call(ALLOCATE, &args![8u32]).u32();
            if e.mem.u32(form_id.addr()) == 0 {
                e.mem.set_u32(sound, 0);
            } else {
                e.call(ADD_COMPILE_INDEX, &args![form_id, file]);
                let id = e.mem.u32(form_id.addr());
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                let cast = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                e.mem.set_u32(sound, cast);
                if e.mem.u32(sound) == 0 {
                    let word = e.call(FORM_WORD_FOR_WARNING, &args![creature]).u32();
                    let name = e
                        .vcall(creature.addr(), NAME_FOR_WARNING_SLOT, &args![])
                        .u32();
                    let id = e.mem.u32(form_id.addr());
                    e.call(PRINT_WARNING, &args![UNKNOWN_SOUND_FORMAT, id, name, word]);
                }
            }
        });
        e.call(NEXT_CHUNK, &args![file]);
    } else if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_CSDF {
        e.with_stack(0x104, |e, name| {
            e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x104u32]);
            sound = e.call(ALLOCATE, &args![8u32]).u32();
            let found = e.call(SOUND_FOR_FILE_NAME, &args![name]).u32();
            e.mem.set_u32(sound, found);
        });
        e.call(NEXT_CHUNK, &args![file]);
    }
    if sound != 0 {
        if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_CSDC {
            e.call(
                GET_CHUNK_DATA_SIZED,
                &args![file, sound.wrapping_add(4), 1u32],
            );
            e.call(ADD_SOUND, &args![this, sound, sound_type]);
            loaded = true;
        } else {
            e.call(FREE, &args![sound]);
        }
    }
    loaded
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x005f_0480,
        creature_sounds_load_creature_sound(Ptr<CreatureSounds>, Ptr, u32, Ptr) -> bool
    )]
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CREATURE_VTABLE: u32 = 0x00ab_0000;
    const NAME_SLOT_FN: u32 = 0x00ab_1000;

    /// An engine whose file hands out the given chunk tags in order (the
    /// current tag moves on at each `004726f0`), a form id of `form_id`, a
    /// form map answering `form` and a cast answering its first word.
    fn engine(tags: Vec<u32>, form_id: u32, form: u32) -> Engine {
        let mut e = Engine::new();
        let position = Rc::new(RefCell::new(0usize));
        let p = position.clone();
        let t = tags.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            t.get(*p.borrow()).copied().unwrap_or(0).into_ret()
        });
        e.register_double(NEXT_CHUNK, move |_, _| {
            *position.borrow_mut() += 1;
            Ret::default()
        });
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            e.mem.set_u32(a[1], form_id);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 40);
            } else {
                e.mem.set_cstr(a[1], b"x.wav");
            }
            Ret::default()
        });
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v | 0x0100_0000);
            Ret::default()
        });
        e.register_double(LOOKUP_FORM, move |_, _| form.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_WORD_FOR_WARNING, |e, a| {
            e.mem.u32(a[0] + 0xc).into_ret()
        });
        e.register(NAME_SLOT_FN, |_, _| 0x0055_0000u32.into_ret());
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e.register(SOUND_FOR_FILE_NAME, |_, _| 0x0066_0000u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(ADD_SOUND, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = NAME_SLOT_FN;
        e.put_vtable(CREATURE_VTABLE, &slots);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn creature(e: &mut Engine) -> u32 {
        let c = e.mem.alloc(0x20);
        e.mem.set_u32(c, CREATURE_VTABLE);
        e.mem.set_u32(c + 0xc, 0x0001_2345);
        c
    }

    #[test]
    fn load_creature_sound_reads_an_id_then_the_chance() {
        let mut e = engine(vec![CHUNK_CSDI, CHUNK_CSDC], 0x77, 0x0042_0000);
        let this = e.new_object::<CreatureSounds>();
        let c = creature(&mut e);
        // No file: false, nothing read.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_0480, &args![this, 0u32, 3u32, c]).bool());
        assert!(calls_to(&e, GET_TES_CHUNK).is_empty());
        assert!(e.call(0x005f_0480, &args![this, 0x9000u32, 3u32, c]).bool());
        let add = calls_to(&e, ADD_SOUND);
        assert_eq!(add.len(), 1);
        let sound = add[0][1];
        assert_eq!(add[0], vec![this.addr(), sound, 3]);
        assert_eq!(e.mem.u32(sound), 0x0042_0000);
        assert_eq!(e.mem.u8(sound + 4), 40);
        assert_eq!(calls_to(&e, LOOKUP_FORM), vec![vec![0x0100_0077]]);
        assert!(calls_to(&e, PRINT_WARNING).is_empty());
    }

    #[test]
    fn load_creature_sound_warns_on_an_unknown_id_and_frees_without_chance() {
        let mut e = engine(vec![CHUNK_CSDI, 0], 0x77, 0);
        let this = e.new_object::<CreatureSounds>();
        let c = creature(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_0480, &args![this, 0x9000u32, 3u32, c]).bool());
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![
                UNKNOWN_SOUND_FORMAT,
                0x0100_0077,
                0x0055_0000,
                0x0001_2345
            ]]
        );
        assert_eq!(calls_to(&e, FREE).len(), 1);
        assert!(calls_to(&e, ADD_SOUND).is_empty());
    }

    #[test]
    fn load_creature_sound_reads_a_file_name_and_id_zero() {
        let mut e = engine(vec![CHUNK_CSDF, CHUNK_CSDC], 0, 0);
        let this = e.new_object::<CreatureSounds>();
        let c = creature(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_0480, &args![this, 0x9000u32, 1u32, c]).bool());
        let sound = calls_to(&e, ADD_SOUND)[0][1];
        assert_eq!(e.mem.u32(sound), 0x0066_0000);
        // A CSDI with id 0: a null sound, no lookup.
        let mut e = engine(vec![CHUNK_CSDI, CHUNK_CSDC], 0, 0x0042_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_0480, &args![this, 0x9000u32, 1u32, c]).bool());
        assert!(calls_to(&e, LOOKUP_FORM).is_empty());
        let sound = calls_to(&e, ADD_SOUND)[0][1];
        assert_eq!(e.mem.u32(sound), 0);
        // Another chunk: nothing made, false.
        let mut e = engine(vec![0x1234_5678], 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_0480, &args![this, 0x9000u32, 1u32, c]).bool());
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(calls_to(&e, NEXT_CHUNK).is_empty());
    }
}
