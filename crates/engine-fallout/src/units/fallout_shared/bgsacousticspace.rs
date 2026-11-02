//! `fallout shared/bgsacousticspace.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSAcousticSpace` (Xbox PDB; fields 0x10 lower on PC, where
    /// `TESForm` is 0x18 bytes): the fields used so far.
    pub struct BGSAcousticSpace: 0x54 {
        /// `pDawnSound` (Xbox PDB): `TESSound*`, a form id until `InitItem`.
        0x34 pDawnSound: u32,
        /// `pNoonSound` (Xbox PDB).
        0x38 pNoonSound: u32,
        /// `pDuskSound` (Xbox PDB).
        0x3C pDuskSound: u32,
        /// `pNightSound` (Xbox PDB).
        0x40 pNightSound: u32,
        /// `pWallaSound` (Xbox PDB).
        0x44 pWallaSound: u32,
        /// `pSoundRegion` (Xbox PDB): `TESRegion*`.
        0x48 pSoundRegion: u32,
    }
}

/// `004013e0` (`thiscall`): whether bit 3 (0x8) of the form's flag word at
/// +8 is set.
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`thiscall`, one byte): sets (non-zero) or clears bit 3 of the
/// form's flag word at +8.
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (`00484e60`, engine map; `thiscall`, one word).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map; `cdecl`: `&form_id`,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by form id (`004839c0`, `cdecl`).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor `.?AVTESSound@@`.
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
/// RTTI type descriptor `.?AVTESRegion@@`.
const TYPE_TES_REGION: u32 = 0x0118_99b4;
/// `0084e3a0` (`thiscall`): the form's word at +0xC (its form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// The form's name: its vtable slot +0x130.
const NAME_SLOT: u32 = 0x130;
/// The warning printer (`005b5e40`, `cdecl`, format and arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;

/// The resolved fields in the exe's order: offset, target type, the
/// "MASTERFILE: Could not find ... (%08X) for activator '%s' (%08X)."
/// format.
const SOUNDS: [(u32, u32, u32); 6] = [
    (0x34, TYPE_TES_SOUND, 0x0103_2400),
    (0x38, TYPE_TES_SOUND, 0x0103_23b8),
    (0x3c, TYPE_TES_SOUND, 0x0103_2370),
    (0x40, TYPE_TES_SOUND, 0x0103_2320),
    (0x48, TYPE_TES_REGION, 0x0103_22d0),
    (0x44, TYPE_TES_SOUND, 0x0103_2280),
];

// Translated from 005899f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAcousticSpace::InitItem` (Xbox PDB), `thiscall`: unless the form's
/// flag bit 0x8 is set, resolves the form ids of the dawn, noon, dusk and
/// night sounds, the sound region and the walla sound (in that order): each
/// non-zero id gets the form's file compile index, is looked up and cast
/// (`TESSound`, `TESRegion` for the region); a missing one (null) is
/// reported with the form's name and form id. Then the flag bit is set.
pub fn bgs_acoustic_space_init_item(e: &mut Engine, this: Ptr<BGSAcousticSpace>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, form_id| {
        for (offset, target, format) in SOUNDS {
            let field = this.addr() + offset;
            let id = e.mem.u32(field);
            e.mem.set_u32(form_id.addr(), id);
            if id == 0 {
                continue;
            }
            let file = e.call(FORM_GET_FILE, &args![this, -1i32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
            let id = e.mem.u32(form_id.addr());
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
                )
                .u32();
            e.mem.set_u32(field, cast);
            if e.mem.u32(field) == 0 {
                let own_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(this.addr(), NAME_SLOT, &args![]).u32();
                e.call(PRINT_WARNING, &args![format, id, name, own_id]);
            }
        }
    });
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0058_99f0,
        bgs_acoustic_space_init_item(Ptr<BGSAcousticSpace>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: u32 = 0x0001_0010;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine(initialized: bool) -> (Engine, Ptr<BGSAcousticSpace>) {
        let mut e = Engine::new();
        let this = e.new_object::<BGSAcousticSpace>();
        let mut slots = vec![0u32; 0x130 / 4 + 1];
        slots[0x130 / 4] = NAME;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0002_0000);
        e.mem.set_u32(this.addr() + 0xc, 0x0000_abcd);
        e.register_double(FORM_FLAG_8_SET, move |_, _| word(u32::from(initialized)));
        e.register(FORM_SET_FLAG_8, |_, _| Ret::default());
        e.register(FORM_GET_FILE, |_, _| word(0x7000));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids with bit 23 set are not found.
        e.register(LOOKUP_FORM_BY_ID, |_, a| {
            word(if a[0] & 0x0080_0000 != 0 {
                0
            } else {
                a[0] + 0x10
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        e.register(FORM_ID, |e, a| word(e.mem.u32(a[0] + 0xc)));
        e.register(NAME, |_, _| word(0x9000));
        e.register(PRINT_WARNING, |_, _| Ret::default());
        (e, this)
    }

    #[test]
    fn init_item_resolves_each_sound_and_the_region() {
        let (mut e, this) = engine(false);
        for (i, (offset, _, _)) in SOUNDS.iter().enumerate() {
            e.mem.set_u32(this.addr() + offset, 0x100 * (i as u32 + 1));
        }
        // The noon sound stays unset; the walla sound is missing.
        e.set(this, BGSAcousticSpace::pNoonSound, 0);
        e.set(this, BGSAcousticSpace::pWallaSound, 0x0080_0001);
        e.call_log = Some(vec![]);
        e.call(0x0058_99f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, BGSAcousticSpace::pDawnSound), 0x0100_0110);
        assert_eq!(e.get(this, BGSAcousticSpace::pNoonSound), 0);
        assert_eq!(e.get(this, BGSAcousticSpace::pDuskSound), 0x0100_0310);
        assert_eq!(e.get(this, BGSAcousticSpace::pNightSound), 0x0100_0410);
        assert_eq!(e.get(this, BGSAcousticSpace::pSoundRegion), 0x0100_0510);
        assert_eq!(e.get(this, BGSAcousticSpace::pWallaSound), 0);
        let casts: Vec<_> = log
            .iter()
            .filter(|c| c.0 == RT_DYNAMIC_CAST)
            .map(|c| c.1[3])
            .collect();
        assert_eq!(
            casts,
            vec![
                TYPE_TES_SOUND,
                TYPE_TES_SOUND,
                TYPE_TES_SOUND,
                TYPE_TES_REGION,
                TYPE_TES_SOUND
            ]
        );
        let warnings: Vec<_> = log
            .iter()
            .filter(|c| c.0 == PRINT_WARNING)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            warnings,
            vec![vec![0x0103_2280, 0x0180_0001, 0x9000, 0xabcd]]
        );
        assert_eq!(
            log.last().unwrap(),
            &(FORM_SET_FLAG_8, vec![this.addr(), 1])
        );
    }

    #[test]
    fn init_item_runs_once() {
        let (mut e, this) = engine(true);
        e.set(this, BGSAcousticSpace::pDawnSound, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0058_99f0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        assert_eq!(e.get(this, BGSAcousticSpace::pDawnSound), 0x100);
    }
}
