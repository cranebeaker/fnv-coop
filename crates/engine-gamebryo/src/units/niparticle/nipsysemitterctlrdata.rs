//! `NiParticle/nipsysemitterctlrdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a89af0`: the jump target of `00a724d0`.
const JUMP_TARGET_00A724D0: u32 = 0x00a8_9af0;

// Translated from 00a724d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::LinkObject` (Xbox PDB): a 5-byte jump to `00a89af0`, passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysEmitterCtlrData@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_emitter_ctlr_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A724D0, &args![this, stream]);
}

// Translated from 00c2baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2760.
pub fn ni_p_sys_emitter_ctlr_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2760
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};
use crate::units::niparticle::nipsysmodifier::{stream_read, STREAM_VERSION};

layout! {
    /// `NiPSysEmitterCtlrData` (Xbox PDB), 0x28 bytes; the PC code uses the
    /// same offsets.
    pub struct NiPSysEmitterCtlrData: 0x28 {
        /// `m_uiNumBirthRateKeys` (Xbox PDB).
        0x08 m_uiNumBirthRateKeys: u32,
        /// `m_pkBirthRateKeys` (Xbox PDB): `NiFloatKey*`.
        0x0C m_pkBirthRateKeys: u32,
        /// `m_eBirthRateKeyType` (Xbox PDB).
        0x10 m_eBirthRateKeyType: u32,
        /// `m_ucBirthRateSize` (Xbox PDB): the size of one key.
        0x14 m_ucBirthRateSize: u8,
        /// `m_uiNumEmitterActiveKeys` (Xbox PDB).
        0x18 m_uiNumEmitterActiveKeys: u32,
        /// `m_pkEmitterActiveKeys` (Xbox PDB): `NiBoolKey*`.
        0x1C m_pkEmitterActiveKeys: u32,
        /// `m_eEmitterActiveKeyType` (Xbox PDB).
        0x20 m_eEmitterActiveKeyType: u32,
        /// `m_ucEmitterActiveSize` (Xbox PDB).
        0x24 m_ucEmitterActiveSize: u8,
    }
}

/// The key-function tables the code indexes by key type (words of
/// `cdecl` functions in the exe's data): the equality test the comparison
/// uses for both key arrays (`011f3ae0`: two keys), the deletes (`011f38a0`
/// for the birth-rate keys and, in `ReplaceEmitterActiveKeys`, the active
/// ones; `011f3918` in the destructor for the active ones: the keys), the
/// loads (`011f3db0` and `011f3e28`: stream, count; the keys), the
/// functions run on the loaded keys (`011f3780` and `011f37f8`: keys,
/// count, a word whose low byte is the key size) and the saves (`011f3a50`
/// and `011f3ac8`: stream, keys, count); and the key-size bytes by type
/// (`011f3758` for the birth-rate keys, `011f3776` for the active ones).
const KEY_EQUAL_TABLE: u32 = 0x011f_3ae0;
const BIRTH_KEY_DELETE_TABLE: u32 = 0x011f_38a0;
const ACTIVE_KEY_DELETE_TABLE: u32 = 0x011f_3918;
const BIRTH_KEY_LOAD_TABLE: u32 = 0x011f_3db0;
const ACTIVE_KEY_LOAD_TABLE: u32 = 0x011f_3e28;
const BIRTH_KEY_FILL_TABLE: u32 = 0x011f_3780;
const ACTIVE_KEY_FILL_TABLE: u32 = 0x011f_37f8;
const BIRTH_KEY_SAVE_TABLE: u32 = 0x011f_3a50;
const ACTIVE_KEY_SAVE_TABLE: u32 = 0x011f_3ac8;
const BIRTH_KEY_SIZES: u32 = 0x011f_3758;
const ACTIVE_KEY_SIZES: u32 = 0x011f_3776;
/// `NiObject`'s `IsEqual` (`00a5d470`), `LoadBinary` (`00a5d610`) and
/// `GetViewerStrings` (`00a5d6e0`) (Xbox PDB names), `00a89af0` (unnamed;
/// what `SaveBinary` runs first), and `NiObject`'s destructor body
/// (`00a5d3d0`); all `thiscall`.
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// The vtable the destructor stores (`010bfd54`).
const VTABLE: u32 = 0x010b_fd54;
/// Streams older than this version have no emitter-active key type (5 is
/// used).
const VERSION_ACTIVE_KEY_TYPE: u32 = 0x0a01_0068;
const DEFAULT_ACTIVE_KEY_TYPE: u32 = 5;
/// `NiGetViewerString` (`00a7d330`, the text), `_ov8` (`00a7d510`: a label
/// and a word) and `_ov11` (`00a7d5e0`: a label and a text) (Xbox PDB
/// names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_WORD: u32 = 0x00a7_d510;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `NiPSysEmitterCtlrData::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2760;
/// The labels "Num Birth Rate Keys" (`010bfe24`), "Birth Rate Key Type"
/// (`010bfdf8`) and "Num Emitter Active Keys" (`010bfde0`), and the key
/// type names "LINKEY" (`010bfe0c`), "BEZKEY" (`010bfe14`), "TCBKEY"
/// (`010bfe1c`) and "Unknown" (`0106f148`).
const LABEL_NUM_BIRTH_KEYS: u32 = 0x010b_fe24;
const LABEL_BIRTH_KEY_TYPE: u32 = 0x010b_fdf8;
const LABEL_NUM_ACTIVE_KEYS: u32 = 0x010b_fde0;
const TEXT_LINKEY: u32 = 0x010b_fe0c;
const TEXT_BEZKEY: u32 = 0x010b_fe14;
const TEXT_TCBKEY: u32 = 0x010b_fe1c;
const TEXT_UNKNOWN: u32 = 0x0106_f148;

/// The function in `table` for key type `key_type`.
fn key_function(e: &Engine, table: u32, key_type: u32) -> u32 {
    e.global::<u32>(table.wrapping_add(key_type.wrapping_mul(4)))
}

// Translated from 00c2bab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::IsEqual` (Xbox PDB): `NiObject`'s, then equal
/// birth-rate key counts and types, emitter-active key counts and
/// birth-rate key sizes; then every birth-rate key and every
/// emitter-active key (at this one's key sizes) through the equality test
/// of this one's birth-rate key type (other's key first).
pub fn ni_p_sys_emitter_ctlr_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    other: Ptr<NiPSysEmitterCtlrData>,
) -> bool {
    type D = NiPSysEmitterCtlrData;
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(other, D::m_uiNumBirthRateKeys) != e.get(this, D::m_uiNumBirthRateKeys)
        || e.get(other, D::m_eBirthRateKeyType) != e.get(this, D::m_eBirthRateKeyType)
        || e.get(other, D::m_uiNumEmitterActiveKeys) != e.get(this, D::m_uiNumEmitterActiveKeys)
        || e.get(other, D::m_ucBirthRateSize) != e.get(this, D::m_ucBirthRateSize)
    {
        return false;
    }
    let equal = key_function(e, KEY_EQUAL_TABLE, e.get(this, D::m_eBirthRateKeyType));
    let mut index = 0u32;
    while index < e.get(this, D::m_uiNumBirthRateKeys) {
        let offset = u32::from(e.get(this, D::m_ucBirthRateSize)).wrapping_mul(index);
        let ours = e.get(this, D::m_pkBirthRateKeys).wrapping_add(offset);
        let theirs = e.get(other, D::m_pkBirthRateKeys).wrapping_add(offset);
        if !e.call(equal, &args![theirs, ours]).bool() {
            return false;
        }
        index += 1;
    }
    let mut index = 0u32;
    while index < e.get(this, D::m_uiNumEmitterActiveKeys) {
        let offset = u32::from(e.get(this, D::m_ucEmitterActiveSize)).wrapping_mul(index);
        let ours = e.get(this, D::m_pkEmitterActiveKeys).wrapping_add(offset);
        let theirs = e.get(other, D::m_pkEmitterActiveKeys).wrapping_add(offset);
        if !e.call(equal, &args![theirs, ours]).bool() {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00c2bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces the birth-rate keys. Old keys are
/// deleted (the delete of their type, `011f38a0`); with keys and a count the
/// keys, count, type and the type's key size (`011f3758`) are stored,
/// otherwise count, keys, type and size are cleared.
pub fn fn_00c2bb70(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    keys: u32,
    count: u32,
    key_type: u32,
) {
    type D = NiPSysEmitterCtlrData;
    let old = e.get(this, D::m_pkBirthRateKeys);
    if old != 0 {
        let delete = key_function(
            e,
            BIRTH_KEY_DELETE_TABLE,
            e.get(this, D::m_eBirthRateKeyType),
        );
        e.call(delete, &args![old]);
    }
    if keys != 0 && count != 0 {
        let size = e.global::<u8>(BIRTH_KEY_SIZES.wrapping_add(key_type));
        e.set(this, D::m_pkBirthRateKeys, keys);
        e.set(this, D::m_ucBirthRateSize, size);
        e.set(this, D::m_uiNumBirthRateKeys, count);
        e.set(this, D::m_eBirthRateKeyType, key_type);
        return;
    }
    e.set(this, D::m_uiNumBirthRateKeys, 0);
    e.set(this, D::m_pkBirthRateKeys, 0);
    e.set(this, D::m_eBirthRateKeyType, 0);
    e.set(this, D::m_ucBirthRateSize, 0);
}

// Translated from 00c2bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces the emitter-active keys. Old keys
/// are deleted through the birth-rate delete table (`011f38a0`) at the
/// active key type, as the exe does; with keys and a count the keys, count,
/// type and the type's key size (`011f3776`) are stored, otherwise count,
/// keys and size are cleared (the type is kept).
pub fn fn_00c2bbd0(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    keys: u32,
    count: u32,
    key_type: u32,
) {
    type D = NiPSysEmitterCtlrData;
    let old = e.get(this, D::m_pkEmitterActiveKeys);
    if old != 0 {
        let old_type = e.get(this, D::m_eEmitterActiveKeyType);
        let delete = key_function(e, BIRTH_KEY_DELETE_TABLE, old_type);
        e.call(delete, &args![old]);
    }
    if keys != 0 && count != 0 {
        let size = e.global::<u8>(ACTIVE_KEY_SIZES.wrapping_add(key_type));
        e.set(this, D::m_pkEmitterActiveKeys, keys);
        e.set(this, D::m_ucEmitterActiveSize, size);
        e.set(this, D::m_uiNumEmitterActiveKeys, count);
        e.set(this, D::m_eEmitterActiveKeyType, key_type);
        return;
    }
    e.set(this, D::m_uiNumEmitterActiveKeys, 0);
    e.set(this, D::m_pkEmitterActiveKeys, 0);
    e.set(this, D::m_ucEmitterActiveSize, 0);
}

// Translated from 00c2bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::_scalar_deleting_destructor_` (Xbox PDB): the
/// vtable back to this class's, the birth-rate keys deleted (`011f38a0` by
/// type) and the emitter-active keys (`011f3918` by type), `NiObject`'s
/// destructor body, then the sized delete of 0x28 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_p_sys_emitter_ctlr_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    flags: u32,
) -> Ptr<NiPSysEmitterCtlrData> {
    type D = NiPSysEmitterCtlrData;
    e.mem.set_u32(this.addr(), VTABLE);
    let keys = e.get(this, D::m_pkBirthRateKeys);
    if keys != 0 {
        let delete = key_function(
            e,
            BIRTH_KEY_DELETE_TABLE,
            e.get(this, D::m_eBirthRateKeyType),
        );
        e.call(delete, &args![keys]);
    }
    let keys = e.get(this, D::m_pkEmitterActiveKeys);
    if keys != 0 {
        let key_type = e.get(this, D::m_eEmitterActiveKeyType);
        let delete = key_function(e, ACTIVE_KEY_DELETE_TABLE, key_type);
        e.call(delete, &args![keys]);
    }
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x28u32]);
    }
    this
}

// Translated from 00c2bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::LoadBinary` (Xbox PDB): `NiObject`'s, then the
/// birth-rate key count; with keys, their type, the keys from the load
/// function of the type (`011f3db0`), the function of `011f3780` on them
/// (keys, count, the type's word with its low byte replaced by the key
/// size), and `00c2bb70` with keys, count and type. Then the emitter-active
/// key count; with keys, their type (5 before stream version 10.1.0.104;
/// the size word is then just the size), the same steps through
/// `011f3e28`, `011f37f8` and `00c2bbd0`. Every read goes through the
/// stream's read function (4 bytes, component size 4).
pub fn ni_p_sys_emitter_ctlr_data_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    stream: Ptr,
) {
    let stream = stream.addr();
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0x10, |e, locals| {
        // The exe keeps the birth-rate count in the stream argument's slot
        // and the rest in three locals; the component sizes go through the
        // active-type word and the type-and-size word.
        let birth_count = locals.addr();
        let active_count = locals.addr() + 4;
        let active_type = locals.addr() + 8;
        let word = locals.addr() + 0xc;
        stream_read(e, stream, birth_count, 4, 4, active_type);
        if e.mem.u32(birth_count) != 0 {
            stream_read(e, stream, word, 4, 4, active_type);
            let key_type = e.mem.u32(word);
            let size = e.global::<u8>(BIRTH_KEY_SIZES.wrapping_add(key_type));
            let count = e.mem.u32(birth_count);
            e.mem.set_u8(word, size);
            let load = key_function(e, BIRTH_KEY_LOAD_TABLE, key_type);
            let keys = e.call(load, &args![stream, count]).u32();
            let size_word = e.mem.u32(word);
            let fill = key_function(e, BIRTH_KEY_FILL_TABLE, key_type);
            let count = e.mem.u32(birth_count);
            e.call(fill, &args![keys, count, size_word]);
            let count = e.mem.u32(birth_count);
            fn_00c2bb70(e, this, keys, count, key_type);
        }
        stream_read(e, stream, active_count, 4, 4, word);
        if e.mem.u32(active_count) != 0 {
            let mut key_type = DEFAULT_ACTIVE_KEY_TYPE;
            if e.mem.u32(stream + STREAM_VERSION) >= VERSION_ACTIVE_KEY_TYPE {
                stream_read(e, stream, active_type, 4, 4, word);
                key_type = e.mem.u32(active_type);
            }
            let size = e.global::<u8>(ACTIVE_KEY_SIZES.wrapping_add(key_type));
            let count = e.mem.u32(active_count);
            e.mem.set_u8(word, size);
            let load = key_function(e, ACTIVE_KEY_LOAD_TABLE, key_type);
            let keys = e.call(load, &args![stream, count]).u32();
            let size_word = e.mem.u32(word);
            let fill = key_function(e, ACTIVE_KEY_FILL_TABLE, key_type);
            let count = e.mem.u32(active_count);
            e.call(fill, &args![keys, count, size_word]);
            let count = e.mem.u32(active_count);
            fn_00c2bbd0(e, this, keys, count, key_type);
        }
    });
}

// Translated from 00c2bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::SaveBinary` (Xbox PDB): `00a89af0` first, then
/// the birth-rate key count and, with keys, their type and the keys (the
/// save of the type, `011f3a50`); then the emitter-active key count and,
/// once per emitter-active key (the exe's loop), their type and all the
/// keys (`011f3ac8`).
pub fn ni_p_sys_emitter_ctlr_data_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    stream: Ptr,
) {
    type D = NiPSysEmitterCtlrData;
    let stream = stream.addr();
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let field = |f: Field<D, u32>| this.addr() + f.off;
        stream_write(e, stream, field(D::m_uiNumBirthRateKeys), 4, 4, slot);
        if e.get(this, D::m_uiNumBirthRateKeys) != 0 {
            stream_write(e, stream, field(D::m_eBirthRateKeyType), 4, 4, slot);
            let count = e.get(this, D::m_uiNumBirthRateKeys);
            let keys = e.get(this, D::m_pkBirthRateKeys);
            let save = key_function(e, BIRTH_KEY_SAVE_TABLE, e.get(this, D::m_eBirthRateKeyType));
            e.call(save, &args![stream, keys, count]);
        }
        stream_write(e, stream, field(D::m_uiNumEmitterActiveKeys), 4, 4, slot);
        let mut count = e.get(this, D::m_uiNumEmitterActiveKeys);
        let mut index = 0u32;
        if count != 0 {
            loop {
                if count != 0 {
                    stream_write(e, stream, field(D::m_eEmitterActiveKeyType), 4, 4, slot);
                    let count = e.get(this, D::m_uiNumEmitterActiveKeys);
                    let keys = e.get(this, D::m_pkEmitterActiveKeys);
                    let key_type = e.get(this, D::m_eEmitterActiveKeyType);
                    let save = key_function(e, ACTIVE_KEY_SAVE_TABLE, key_type);
                    e.call(save, &args![stream, keys, count]);
                }
                index += 1;
                count = e.get(this, D::m_uiNumEmitterActiveKeys);
                if index >= count {
                    break;
                }
            }
        }
    });
}

// Translated from 00c2bf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlrData::GetViewerStrings` (Xbox PDB): `NiObject`'s
/// strings, the class name, the birth-rate key count, its key type
/// ("LINKEY" for 1, "BEZKEY" for 2, "TCBKEY" for 3, else "Unknown") and
/// the emitter-active key count.
pub fn ni_p_sys_emitter_ctlr_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysEmitterCtlrData>,
    strings: Ptr,
) {
    type D = NiPSysEmitterCtlrData;
    let strings = strings.addr();
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let count = e.get(this, D::m_uiNumBirthRateKeys);
    let string = e
        .call(VIEWER_STRING_WORD, &args![LABEL_NUM_BIRTH_KEYS, count])
        .u32();
    add_viewer_string(e, strings, string);
    let text = match e.get(this, D::m_eBirthRateKeyType) {
        1 => TEXT_LINKEY,
        2 => TEXT_BEZKEY,
        3 => TEXT_TCBKEY,
        _ => TEXT_UNKNOWN,
    };
    let string = e
        .call(VIEWER_STRING_TEXT, &args![LABEL_BIRTH_KEY_TYPE, text])
        .u32();
    add_viewer_string(e, strings, string);
    let count = e.get(this, D::m_uiNumEmitterActiveKeys);
    let string = e
        .call(VIEWER_STRING_WORD, &args![LABEL_NUM_ACTIVE_KEYS, count])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2baa0, ni_p_sys_emitter_ctlr_data_get_r_t_t_i() -> u32),
        entry!(
            0x00a7_24d0,
            ni_p_sys_emitter_ctlr_data_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00c2_bab0,
            ni_p_sys_emitter_ctlr_data_is_equal(
                Ptr<NiPSysEmitterCtlrData>,
                Ptr<NiPSysEmitterCtlrData>,
            ) -> bool
        ),
        entry!(
            0x00c2_bb70,
            fn_00c2bb70(Ptr<NiPSysEmitterCtlrData>, u32, u32, u32)
        ),
        entry!(
            0x00c2_bbd0,
            fn_00c2bbd0(Ptr<NiPSysEmitterCtlrData>, u32, u32, u32)
        ),
        entry!(
            0x00c2_bc30,
            ni_p_sys_emitter_ctlr_data_scalar_deleting_destructor(
                Ptr<NiPSysEmitterCtlrData>,
                u32,
            )
                -> Ptr<NiPSysEmitterCtlrData>
        ),
        entry!(
            0x00c2_bc90,
            ni_p_sys_emitter_ctlr_data_load_binary(Ptr<NiPSysEmitterCtlrData>, Ptr)
        ),
        entry!(
            0x00c2_bdf0,
            ni_p_sys_emitter_ctlr_data_save_binary(Ptr<NiPSysEmitterCtlrData>, Ptr)
        ),
        entry!(
            0x00c2_bf00,
            ni_p_sys_emitter_ctlr_data_get_viewer_strings(Ptr<NiPSysEmitterCtlrData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a724d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_24d0;
        const TARGET: u32 = 0x00a8_9af0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_emitter_ctlr_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_baa0, &args![]).u32(), 0x0120_2760);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};
    use crate::units::niparticle::nipsysmodifier::test_stream;

    type D = NiPSysEmitterCtlrData;

    /// Test functions the tables point at: table `t`, type `k` is
    /// `0x0e00_0000 + (t << 8) + k`.
    fn table_function(table: u32, key_type: u32) -> u32 {
        0x0e00_0000 + ((table & 0xfff) << 8) + key_type
    }

    /// Maps the key tables, fills types 0..6 of each function table with
    /// quiet test functions and the size tables with `0x10 + type` (birth)
    /// and `0x20 + type` (active).
    fn tables(e: &mut Engine) {
        e.map(0x011f_3700, 0x800);
        for table in [
            KEY_EQUAL_TABLE,
            BIRTH_KEY_DELETE_TABLE,
            ACTIVE_KEY_DELETE_TABLE,
            BIRTH_KEY_LOAD_TABLE,
            ACTIVE_KEY_LOAD_TABLE,
            BIRTH_KEY_FILL_TABLE,
            ACTIVE_KEY_FILL_TABLE,
            BIRTH_KEY_SAVE_TABLE,
            ACTIVE_KEY_SAVE_TABLE,
        ] {
            for key_type in 0..6 {
                let function = table_function(table, key_type);
                e.mem.set_u32(table + 4 * key_type, function);
                e.register(function, |_, _| Ret::default());
            }
        }
        for key_type in 0..6u8 {
            e.mem
                .set_u8(BIRTH_KEY_SIZES + u32::from(key_type), 0x10 + key_type);
            e.mem
                .set_u8(ACTIVE_KEY_SIZES + u32::from(key_type), 0x20 + key_type);
        }
    }

    fn data(e: &mut Engine) -> Ptr<D> {
        e.new_object::<D>()
    }

    #[test]
    fn is_equal_compares_counts_types_and_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_IS_EQUAL, |_, _| answer(1));
        let equal = table_function(KEY_EQUAL_TABLE, 2);
        // Keys are equal when their first bytes are.
        e.register(equal, |e, a| {
            answer(u32::from(e.mem.u8(a[0]) == e.mem.u8(a[1])))
        });
        let this = data(&mut e);
        let other = data(&mut e);
        for p in [this, other] {
            e.set(p, D::m_uiNumBirthRateKeys, 2);
            e.set(p, D::m_eBirthRateKeyType, 2);
            e.set(p, D::m_ucBirthRateSize, 8);
            e.set(p, D::m_uiNumEmitterActiveKeys, 2);
            e.set(p, D::m_ucEmitterActiveSize, 4);
            e.set(p, D::m_eEmitterActiveKeyType, 9);
            let keys = e.mem.alloc(0x10);
            e.set(p, D::m_pkBirthRateKeys, keys);
            let keys = e.mem.alloc(8);
            e.set(p, D::m_pkEmitterActiveKeys, keys);
        }
        // The active key types are not compared.
        e.set(other, D::m_eEmitterActiveKeyType, 7);
        let check = |e: &mut Engine| e.call(0x00c2_bab0, &args![this, other]).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        let ours = e.get(this, D::m_pkBirthRateKeys);
        let theirs = e.get(other, D::m_pkBirthRateKeys);
        let active_ours = e.get(this, D::m_pkEmitterActiveKeys);
        let active_theirs = e.get(other, D::m_pkEmitterActiveKeys);
        assert_eq!(
            calls_to(&e, equal),
            vec![
                vec![theirs, ours],
                vec![theirs + 8, ours + 8],
                vec![active_theirs, active_ours],
                vec![active_theirs + 4, active_ours + 4]
            ]
        );
        e.mem.set_u8(active_theirs + 4, 1);
        assert!(!check(&mut e));
        e.mem.set_u8(active_theirs + 4, 0);
        e.mem.set_u8(theirs, 1);
        e.call_log = Some(vec![]);
        assert!(!check(&mut e));
        assert_eq!(calls_to(&e, equal).len(), 1);
        e.mem.set_u8(theirs, 0);
        for (field, value) in [
            (D::m_uiNumBirthRateKeys, 3),
            (D::m_eBirthRateKeyType, 3),
            (D::m_uiNumEmitterActiveKeys, 3),
        ] {
            let kept = e.get(other, field);
            e.set(other, field, value);
            assert!(!check(&mut e));
            e.set(other, field, kept);
        }
        e.set(other, D::m_ucBirthRateSize, 4);
        assert!(!check(&mut e));
        e.set(other, D::m_ucBirthRateSize, 8);
        assert!(check(&mut e));
        e.register(OBJECT_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e));
    }

    #[test]
    fn birth_rate_keys_are_replaced() {
        let mut e = Engine::new();
        tables(&mut e);
        let this = data(&mut e);
        e.set(this, D::m_pkBirthRateKeys, 0x700);
        e.set(this, D::m_eBirthRateKeyType, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c2_bb70, &args![this, 0x800u32, 5u32, 2u32]);
        let delete = table_function(BIRTH_KEY_DELETE_TABLE, 3);
        assert_eq!(calls_to(&e, delete), vec![vec![0x700]]);
        assert_eq!(e.get(this, D::m_pkBirthRateKeys), 0x800);
        assert_eq!(e.get(this, D::m_uiNumBirthRateKeys), 5);
        assert_eq!(e.get(this, D::m_eBirthRateKeyType), 2);
        assert_eq!(e.get(this, D::m_ucBirthRateSize), 0x12);
        // No count: everything cleared.
        e.call(0x00c2_bb70, &args![this, 0x900u32, 0u32, 1u32]);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_DELETE_TABLE, 2)),
            vec![vec![0x800]]
        );
        assert_eq!(e.get(this, D::m_pkBirthRateKeys), 0);
        assert_eq!(e.get(this, D::m_uiNumBirthRateKeys), 0);
        assert_eq!(e.get(this, D::m_eBirthRateKeyType), 0);
        assert_eq!(e.get(this, D::m_ucBirthRateSize), 0);
        // No keys and none before: nothing deleted, cleared again.
        e.call(0x00c2_bb70, &args![this, 0u32, 4u32, 1u32]);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_DELETE_TABLE, 0)),
            Vec::<Vec<u32>>::new()
        );
    }

    #[test]
    fn emitter_active_keys_are_replaced() {
        let mut e = Engine::new();
        tables(&mut e);
        let this = data(&mut e);
        e.set(this, D::m_pkEmitterActiveKeys, 0x700);
        e.set(this, D::m_eEmitterActiveKeyType, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c2_bbd0, &args![this, 0x800u32, 5u32, 1u32]);
        // The birth-rate delete table at the active type.
        let delete = table_function(BIRTH_KEY_DELETE_TABLE, 4);
        assert_eq!(calls_to(&e, delete), vec![vec![0x700]]);
        assert_eq!(e.get(this, D::m_pkEmitterActiveKeys), 0x800);
        assert_eq!(e.get(this, D::m_uiNumEmitterActiveKeys), 5);
        assert_eq!(e.get(this, D::m_eEmitterActiveKeyType), 1);
        assert_eq!(e.get(this, D::m_ucEmitterActiveSize), 0x21);
        e.call(0x00c2_bbd0, &args![this, 0u32, 5u32, 3u32]);
        assert_eq!(e.get(this, D::m_pkEmitterActiveKeys), 0);
        assert_eq!(e.get(this, D::m_uiNumEmitterActiveKeys), 0);
        assert_eq!(e.get(this, D::m_ucEmitterActiveSize), 0);
        // The type is kept.
        assert_eq!(e.get(this, D::m_eEmitterActiveKeyType), 1);
    }

    #[test]
    fn destructor_deletes_both_key_arrays() {
        let mut e = Engine::new();
        tables(&mut e);
        quiet(&mut e, &[OBJECT_DESTRUCTOR, NI_DELETE]);
        let this = data(&mut e);
        e.set(this, D::m_pkBirthRateKeys, 0x700);
        e.set(this, D::m_eBirthRateKeyType, 1);
        e.set(this, D::m_pkEmitterActiveKeys, 0x800);
        e.set(this, D::m_eEmitterActiveKeyType, 2);
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_bc30, &args![this, 1u32]).u32();
        assert_eq!(made, this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_DELETE_TABLE, 1)),
            vec![vec![0x700]]
        );
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_DELETE_TABLE, 2)),
            vec![vec![0x800]]
        );
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x28]]);
        e.set(this, D::m_pkBirthRateKeys, 0);
        e.set(this, D::m_pkEmitterActiveKeys, 0);
        e.call(0x00c2_bc30, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_DELETE).len(), 1);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_DELETE_TABLE, 1)).len(),
            1
        );
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    #[test]
    fn load_binary_reads_both_key_arrays() {
        let mut e = Engine::new();
        tables(&mut e);
        quiet(&mut e, &[OBJECT_LOAD_BINARY]);
        let load = table_function(BIRTH_KEY_LOAD_TABLE, 2);
        e.register(load, |_, _| answer(0x7100));
        let load = table_function(ACTIVE_KEY_LOAD_TABLE, 3);
        e.register(load, |_, _| answer(0x7200));
        // Counts 4 and 6, types 2 and 3.
        let bytes = words(&[4, 2, 6, 3]);
        let stream = test_stream::new(&mut e, 0x0a01_0068, &bytes);
        let this = data(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_bc90, &args![this, stream]);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_LOAD_TABLE, 2)),
            vec![vec![stream, 4]]
        );
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_FILL_TABLE, 2)),
            vec![vec![0x7100, 4, 0x12]]
        );
        assert_eq!(e.get(this, D::m_pkBirthRateKeys), 0x7100);
        assert_eq!(e.get(this, D::m_uiNumBirthRateKeys), 4);
        assert_eq!(e.get(this, D::m_ucBirthRateSize), 0x12);
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_LOAD_TABLE, 3)),
            vec![vec![stream, 6]]
        );
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_FILL_TABLE, 3)),
            vec![vec![0x7200, 6, 0x23]]
        );
        assert_eq!(e.get(this, D::m_pkEmitterActiveKeys), 0x7200);
        assert_eq!(e.get(this, D::m_eEmitterActiveKeyType), 3);
        assert_eq!(e.get(this, D::m_ucEmitterActiveSize), 0x23);
        assert_eq!(test_stream::position(&e, stream), 16);
    }

    #[test]
    fn load_binary_keeps_the_type_word_and_uses_type_5_for_old_streams() {
        let mut e = Engine::new();
        tables(&mut e);
        quiet(&mut e, &[OBJECT_LOAD_BINARY]);
        // An old stream: the active type is not read.
        let bytes = words(&[1, 1, 2]);
        let stream = test_stream::new(&mut e, 0x0a01_0067, &bytes);
        let this = data(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_bc90, &args![this, stream]);
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_FILL_TABLE, 1)),
            vec![vec![0, 1, 0x11]]
        );
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_LOAD_TABLE, 5)),
            vec![vec![stream, 2]]
        );
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_FILL_TABLE, 5)),
            vec![vec![0, 2, 0x25]]
        );
        assert_eq!(test_stream::position(&e, stream), 12);
        // No keys at all: only the two counts are read.
        let stream = test_stream::new(&mut e, 0x0a01_0068, &words(&[0, 0]));
        e.call_log = Some(vec![]);
        e.call(0x00c2_bc90, &args![this, stream]);
        assert_eq!(test_stream::position(&e, stream), 8);
        assert!(calls_to(&e, table_function(BIRTH_KEY_LOAD_TABLE, 0)).is_empty());
    }

    #[test]
    fn save_binary_writes_counts_types_and_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        quiet(&mut e, &[BASE_SAVE_BINARY]);
        let stream = viewer_test::out_stream(&mut e);
        let this = data(&mut e);
        e.set(this, D::m_uiNumBirthRateKeys, 3);
        e.set(this, D::m_pkBirthRateKeys, 0x700);
        e.set(this, D::m_eBirthRateKeyType, 1);
        e.set(this, D::m_uiNumEmitterActiveKeys, 2);
        e.set(this, D::m_pkEmitterActiveKeys, 0x800);
        e.set(this, D::m_eEmitterActiveKeyType, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c2_bdf0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, BASE_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, table_function(BIRTH_KEY_SAVE_TABLE, 1)),
            vec![vec![stream, 0x700, 3]]
        );
        // The active type and keys go out once per active key.
        assert_eq!(
            calls_to(&e, table_function(ACTIVE_KEY_SAVE_TABLE, 4)),
            vec![vec![stream, 0x800, 2], vec![stream, 0x800, 2]]
        );
        assert_eq!(viewer_test::written(&e, stream).0, words(&[3, 1, 2, 4, 4]));
        // No keys: just the two counts.
        let stream = viewer_test::out_stream(&mut e);
        let empty = data(&mut e);
        e.call(0x00c2_bdf0, &args![empty, stream]);
        assert_eq!(viewer_test::written(&e, stream), (words(&[0, 0]), 8));
    }

    #[test]
    fn viewer_strings_name_the_counts_and_key_type() {
        let mut e = Engine::new();
        quiet(&mut e, &[OBJECT_GET_VIEWER_STRINGS]);
        viewer_test::counting(
            &mut e,
            &[VIEWER_STRING, VIEWER_STRING_WORD, VIEWER_STRING_TEXT],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0090);
        let this = data(&mut e);
        e.set(this, D::m_uiNumBirthRateKeys, 3);
        e.set(this, D::m_uiNumEmitterActiveKeys, 2);
        for (key_type, text) in [
            (1, TEXT_LINKEY),
            (2, TEXT_BEZKEY),
            (3, TEXT_TCBKEY),
            (4, TEXT_UNKNOWN),
            (0, TEXT_UNKNOWN),
        ] {
            e.set(this, D::m_eBirthRateKeyType, key_type);
            let strings = viewer_test::strings(&mut e, 8);
            e.call_log = Some(vec![]);
            e.call(0x00c2_bf00, &args![this, strings]);
            assert_eq!(viewer_test::collected(&e, strings).len(), 4);
            assert_eq!(
                calls_to(&e, VIEWER_STRING_TEXT),
                vec![vec![LABEL_BIRTH_KEY_TYPE, text]]
            );
        }
        assert_eq!(
            calls_to(&e, VIEWER_STRING_WORD),
            vec![
                vec![LABEL_NUM_BIRTH_KEYS, 3],
                vec![LABEL_NUM_ACTIVE_KEYS, 2]
            ]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0090]]);
    }
}
