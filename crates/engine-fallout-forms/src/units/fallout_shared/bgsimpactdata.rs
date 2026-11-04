//! `fallout shared/bgsimpactdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `.?AVTESForm@@`, `.?AVBGSTextureSet@@` and `.?AVTESSound@@`: the type descriptors of the
/// casts.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TEXTURE_SET_TYPE: u32 = 0x0118_7788;
const SOUND_TYPE: u32 = 0x0118_32fc;
/// "MASTERFILE: Unable to find texture set (%08X) on impact data %s (%08X)."
const MISSING_TEXTURE_SET: u32 = 0x0103_30f0;
/// "MASTERFILE: Unable to find sound (%08X) on impact data %s (%08X)."
const MISSING_SOUND: u32 = 0x0103_30a8;

/// Resolves the form ID at `offset`: when not 0, fixed up in a stack slot through
/// `00485d50` (cdecl, the slot and the form's file from `00484e60` with -1), looked up
/// with `004839c0`, cast to `target_type` and stored back at `offset`; a null result
/// prints `warning` with the ID, the form's name (vtable slot +0x130) and the word
/// `0084e3a0` gives for it.
fn resolve_member(e: &mut Engine, this: Ptr, offset: u32, target_type: u32, warning: u32) {
    let base = this.addr();
    let loaded = e.mem.u32(base + offset);
    if loaded == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), loaded);
        let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, target_type, 0u32],
        )
        .u32();
    e.mem.set_u32(base + offset, cast);
    if e.mem.u32(base + offset) != 0 {
        return;
    }
    let form_id = e.call(0x0084_e3a0, &args![this]).u32();
    let name = e.vcall(base, 0x130, &args![]).u32();
    e.call(0x005b_5e40, &args![warning, id, name, form_id]);
}

// Translated from 0058e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): when `0058de80` answers true, fills
/// the 0x24-byte record at +0x54 through `005932b0` from the record `0056a880` gives for
/// the object `006733e0` returns, or, when that is null, from the one `0056a8c0` gives
/// (called without arguments).
pub fn fn_0058e730(e: &mut Engine, this: Ptr) {
    if !e.call(0x0058_de80, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    if e.call(0x0067_33e0, &args![this]).u32() != 0 {
        let owner = e.call(0x0067_33e0, &args![this]).u32();
        let record = e.call(0x0056_a880, &args![owner]).u32();
        e.call(0x0059_32b0, &args![base + 0x54, record]);
    } else {
        let record = e.call(0x0056_a8c0, &args![]).u32();
        e.call(0x0059_32b0, &args![base + 0x54, record]);
    }
}

// Translated from 0058e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, resolves the texture set at +0x48 and the sounds at +0x4c and +0x50 (see
/// `resolve_member`), runs `0058e730`, then `00484ab0` with 1.
pub fn bgs_impact_data_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    resolve_member(e, this, 0x48, TEXTURE_SET_TYPE, MISSING_TEXTURE_SET);
    resolve_member(e, this, 0x4c, SOUND_TYPE, MISSING_SOUND);
    resolve_member(e, this, 0x50, SOUND_TYPE, MISSING_SOUND);
    fn_0058e730(e, this);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// `TESForm::StartForm` and `TESForm::CloseForm` (`004855a0`, `00485680`, `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESModel::Save` (`004891d0`, `RET 8`: the model and texture chunk signatures).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// `00401500` (`cdecl`, no arguments, `AL`): whether saved data is byte-swapped.
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `0058e3a0` (`ECX` = the impact data block at +0x30): swaps its byte order.
const SWAP_IMPACT_DATA: u32 = 0x0058_e3a0;
/// `TESForm::__SaveData` (`00485e20`, `RET 8`: address, size).
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
/// `0058de80` (`ECX` = impact data, `AL`): whether bit 0 of the byte at +0x44 is set; the
/// decal data is saved only when it is clear.
const NO_DECAL_DATA: u32 = 0x0058_de80;
/// `DecalData::Save` (`00593320`, `RET 4`: the chunk signature).
const DECAL_DATA_SAVE: u32 = 0x0059_3320;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// Chunk signatures.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_DODT: u32 = 0x5444_4f44;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_NAM1: u32 = 0x314d_414e;
/// The 0x18-byte impact data block (+0x30), saved as the form data.
const DATA_OFFSET: u32 = 0x30;
const DATA_SIZE: u32 = 0x18;

// Translated from 0058e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSImpactData::Save` (Xbox PDB; `thiscall`, no stack word): starts the form record,
/// saves the model (+0x18: `MODL`, `MODT`), the 0x18-byte data block (+0x30,
/// `TESForm::__SaveData`, byte swapped by `0058e3a0` around it when `00401500` says so),
/// the decal data (+0x54, `DODT`) unless `0058de80` answers true, then the form ids of
/// the forms at +0x48 (`DNAM`), +0x4C (`SNAM`) and +0x50 (`NAM1`) when set, and closes
/// the record.
pub fn bgs_impact_data_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START_FORM, &args![this]);
    e.call(MODEL_SAVE, &args![base + 0x18, CHUNK_MODL, CHUNK_MODT]);
    let data = base + DATA_OFFSET;
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_IMPACT_DATA, &args![data]);
    }
    e.call(FORM_SAVE_DATA, &args![this, data, DATA_SIZE]);
    if e.call(SWAP_ON_SAVE, &[]).u8() != 0 {
        e.call(SWAP_IMPACT_DATA, &args![data]);
    }
    if e.call(NO_DECAL_DATA, &args![this]).u8() == 0 {
        e.call(DECAL_DATA_SAVE, &args![base + 0x54, CHUNK_DODT]);
    }
    for (offset, chunk) in [(0x48, CHUNK_DNAM), (0x4c, CHUNK_SNAM), (0x50, CHUNK_NAM1)] {
        let form = e.mem.u32(base + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![chunk, id]);
        }
    }
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_e730, fn_0058e730(Ptr)),
        entry!(0x0058_e560, bgs_impact_data_init_item(Ptr)),
        entry!(0x0058_e450, bgs_impact_data_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// Doubles for `0058e730`'s callees: the check answers `active`, the owner is `owner`.
    fn record_case(e: &mut Engine, active: bool, owner: u32) {
        e.register_double(0x0058_de80, move |_, _| Ret {
            eax: u32::from(active),
            ..Ret::default()
        });
        e.register_double(0x0067_33e0, move |_, _| Ret {
            eax: owner,
            ..Ret::default()
        });
        e.register(0x0056_a880, |_, a| Ret {
            eax: a[0] + 0x24,
            ..Ret::default()
        });
        e.register(0x0056_a8c0, |_, _| Ret {
            eax: 0x0abc_0000,
            ..Ret::default()
        });
        e.register(0x0059_32b0, |_, _| Ret::default());
    }

    #[test]
    fn fn_0058e730_copies_the_owner_or_default_record() {
        for (active, owner, expected) in [
            (false, 0x7000u32, None),
            (true, 0x7000, Some(0x7024u32)),
            (true, 0, Some(0x0abc_0000)),
        ] {
            let mut e = Engine::new();
            record_case(&mut e, active, owner);
            let data = e.mem.alloc(0x80);
            e.call_log = Some(vec![]);
            e.call(0x0058_e730, &args![data]);
            let log = e.call_log.take().unwrap();
            match expected {
                None => assert_eq!(log.len(), 2),
                Some(record) => {
                    assert_eq!(calls_to(&log, 0x0059_32b0), vec![vec![data + 0x54, record]]);
                }
            }
        }
    }

    #[test]
    fn impact_data_init_item_resolves_the_members() {
        // IDs below 0x2000 are found (cast gives the form + 1), others are not.
        let mut e = Engine::new();
        record_case(&mut e, false, 0);
        let data = e.mem.alloc(0x80);
        e.put_vtable(0x0300_0000, &[0; 0x50]);
        e.mem.set_u32(0x0300_0000 + 0x130, 0x0300_1130);
        e.mem.set_u32(data, 0x0300_0000);
        e.mem.set_u32(data + 0x48, 0x10);
        e.mem.set_u32(data + 0x4c, 0);
        e.mem.set_u32(data + 0x50, 0x3000);
        e.register(0x0300_1130, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 1);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: if a[0] < 0x2000 { a[0] + 0x100 } else { 0 },
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.register(0x0048_4ab0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_e560, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(data + 0x48), 0x111);
        assert_eq!(e.mem.u32(data + 0x4c), 0);
        assert_eq!(e.mem.u32(data + 0x50), 0);
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![
                vec![0x11, 0, TES_FORM_TYPE, TEXTURE_SET_TYPE, 0],
                vec![0x3001, 0, TES_FORM_TYPE, SOUND_TYPE, 0],
            ]
        );
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![MISSING_SOUND, 0x3001, 0x7777, 0xbeef]]
        );
        assert_eq!(calls_to(&log, 0x0058_de80), vec![vec![data]]);
        assert_eq!(log.last().unwrap(), &(0x0048_4ab0, vec![data, 1]));
    }

    #[test]
    fn impact_data_init_item_stops_when_checked() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x80);
        e.register(0x0040_13e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0058_e560, &args![data]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn run(no_decal: bool, swap: bool, forms: [u32; 3]) -> (u32, CallLog) {
        let mut e = Engine::new();
        for addr in [
            FORM_START_FORM,
            FORM_CLOSE_FORM,
            MODEL_SAVE,
            SWAP_IMPACT_DATA,
            FORM_SAVE_DATA,
            DECAL_DATA_SAVE,
            ADD_CHUNK_WORD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register_double(SWAP_ON_SAVE, move |_, _| u32::from(swap).into_ret());
        e.register_double(NO_DECAL_DATA, move |_, _| u32::from(no_decal).into_ret());
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x48, forms[0]);
        e.mem.set_u32(this + 0x4c, forms[1]);
        e.mem.set_u32(this + 0x50, forms[2]);
        e.call_log = Some(vec![]);
        e.call(0x0058_e450, &args![this]);
        let log = e.call_log.take().unwrap();
        let log = log
            .into_iter()
            .filter(|c| c.0 != SWAP_ON_SAVE && c.0 != NO_DECAL_DATA)
            .collect();
        (this, log)
    }

    #[test]
    fn save_writes_every_part_in_order() {
        let (this, log) = run(false, true, [0x100, 0x200, 0x300]);
        assert_eq!(
            log[1..],
            [
                (FORM_START_FORM, vec![this]),
                (MODEL_SAVE, vec![this + 0x18, CHUNK_MODL, CHUNK_MODT]),
                (SWAP_IMPACT_DATA, vec![this + 0x30]),
                (FORM_SAVE_DATA, vec![this, this + 0x30, 0x18]),
                (SWAP_IMPACT_DATA, vec![this + 0x30]),
                (DECAL_DATA_SAVE, vec![this + 0x54, CHUNK_DODT]),
                (FORM_ID, vec![0x100]),
                (ADD_CHUNK_WORD, vec![CHUNK_DNAM, 0x101]),
                (FORM_ID, vec![0x200]),
                (ADD_CHUNK_WORD, vec![CHUNK_SNAM, 0x201]),
                (FORM_ID, vec![0x300]),
                (ADD_CHUNK_WORD, vec![CHUNK_NAM1, 0x301]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }

    #[test]
    fn save_skips_the_decal_the_swaps_and_unset_forms() {
        let (this, log) = run(true, false, [0, 0x200, 0]);
        assert_eq!(log[3], (FORM_SAVE_DATA, vec![this, this + 0x30, 0x18]));
        assert_eq!(
            log[4..],
            [
                (FORM_ID, vec![0x200]),
                (ADD_CHUNK_WORD, vec![CHUNK_SNAM, 0x201]),
                (FORM_CLOSE_FORM, vec![this]),
            ]
        );
    }
}
