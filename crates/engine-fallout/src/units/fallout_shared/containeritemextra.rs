//! `fallout shared/containeritemextra.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ContainerItemExtra` (Xbox PDB), 0xC bytes (the same on PC).
    pub struct ContainerItemExtra: 0x0C {
        /// `pOwnerForm` (Xbox PDB): `TESForm*` (a form id until `InitItem`).
        0x00 pOwnerForm: u32,
        /// `pOwnerGlobal` / `iOwnerRank` (Xbox PDB, a union).
        0x04 iUnion: u32,
        /// `fHealthMult` (Xbox PDB), as stored bits.
        0x08 fHealthMult: u32,
    }
}

/// The `COED` chunk signature, as the word the exe compares.
const SIG_COED: u32 = 0x4445_4f43;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVTESGlobal@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_GLOBAL: u32 = 0x0118_3ad4;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The form type byte (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Failed to find owner form (%08X) in init for
/// ContainerItemExtra for form '%s' (%08X)."
const MISSING_OWNER_MESSAGE: u32 = 0x0101_4168;
/// "MASTERFILE: Failed to find owner global (%08X) in init for
/// ContainerItemExtra for form '%s' (%08X)."
const MISSING_GLOBAL_MESSAGE: u32 = 0x0101_4100;

// Translated from 0040e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerItemExtra::ContainerItemExtra` (Xbox PDB): no owner, no
/// global or rank, a health multiplier of 1.0 (`FLD1`). Returns `this`.
pub fn container_item_extra_container_item_extra(
    e: &mut Engine,
    this: Ptr<ContainerItemExtra>,
) -> Ptr<ContainerItemExtra> {
    e.set(this, ContainerItemExtra::pOwnerForm, 0);
    e.set(this, ContainerItemExtra::iUnion, 0);
    e.set(this, ContainerItemExtra::fHealthMult, 1.0f32.to_bits());
    this
}

// Translated from 0040e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the extra from `file` (`RET 4`): with a file whose next chunk
/// (`TESFile::GetTESChunk`, `004726b0`) is `COED`, 0xC bytes are read into a
/// stack copy (`00472890`), swapped (`005a6b10`) when the file says so
/// (`00401680`), and copied in (the multiplier through `FLD`/`FSTP`, a
/// signalling NaN coming out quiet); true then, false otherwise.
pub fn fn_0040e780(e: &mut Engine, this: Ptr<ContainerItemExtra>, file: Ptr) -> bool {
    if file.is_null() || e.call(0x0047_26b0, &args![file]).u32() != SIG_COED {
        return false;
    }
    e.with_stack(0xc, |e, copy| {
        e.call(0x0047_2890, &args![file, copy, 0xcu32]);
        if e.call(0x0040_1680, &args![file]).bool() {
            e.call(0x005a_6b10, &args![copy]);
        }
        let owner = e.mem.u32(copy.addr());
        let union = e.mem.u32(copy.addr() + 4);
        let health = x87_quiet_bits(e.mem.u32(copy.addr() + 8));
        e.set(this, ContainerItemExtra::pOwnerForm, owner);
        e.set(this, ContainerItemExtra::iUnion, union);
        e.set(this, ContainerItemExtra::fHealthMult, health);
    });
    true
}

// Translated from 0040e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerItemExtra::InitItem` (Xbox PDB), `form` the form holding it
/// (`RET 4`): an owner id gets the compile index of `form`'s file
/// (`00485d50` with `TESForm::GetFile(-1)`) and is looked up (`004839c0`);
/// missing, it is reported (with the id, `form`'s vtable slot 0x130 answer
/// and form id). Without an owner the union word is cleared; with an owner
/// of form type 0x2A (`00401170`) a global id in it that is not zero is
/// resolved the same way and cast to `TESGlobal`, a missing one reported.
pub fn container_item_extra_init_item(e: &mut Engine, this: Ptr<ContainerItemExtra>, form: Ptr) {
    let resolve = |e: &mut Engine, id_at: Ptr| {
        let file = e.call(GET_FILE, &args![form, u32::MAX]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
        let id = e.mem.u32(id_at.addr());
        e.call(LOOKUP_FORM, &args![id]).u32()
    };
    let report = |e: &mut Engine, message: u32, id: u32| {
        let form_id = e.call(FORM_ID, &args![form]).u32();
        let name = e.vcall(form.addr(), 0x130, &args![]).u32();
        e.call(WARNING, &args![message, id, name, form_id]);
    };
    e.with_stack(8, |e, locals| {
        let owner_at = locals;
        let global_at = locals.byte_add(4);
        if e.get(this, ContainerItemExtra::pOwnerForm) != 0 {
            let owner = e.get(this, ContainerItemExtra::pOwnerForm);
            e.mem.set_u32(owner_at.addr(), owner);
            let found = resolve(e, owner_at);
            e.set(this, ContainerItemExtra::pOwnerForm, found);
            if e.get(this, ContainerItemExtra::pOwnerForm) == 0 {
                let id = e.mem.u32(owner_at.addr());
                report(e, MISSING_OWNER_MESSAGE, id);
            }
        }
        let owner = e.get(this, ContainerItemExtra::pOwnerForm);
        if owner == 0 {
            e.set(this, ContainerItemExtra::iUnion, 0);
        } else if e.call(FORM_TYPE, &args![owner]).u32() == 0x2a
            && e.get(this, ContainerItemExtra::iUnion) != 0
        {
            let global = e.get(this, ContainerItemExtra::iUnion);
            e.mem.set_u32(global_at.addr(), global);
            let found = resolve(e, global_at);
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0u32],
                )
                .u32();
            e.set(this, ContainerItemExtra::iUnion, cast);
            if e.get(this, ContainerItemExtra::iUnion) == 0 {
                let id = e.mem.u32(global_at.addr());
                report(e, MISSING_GLOBAL_MESSAGE, id);
            }
        }
    });
}

/// `TESForm::__AddChunkData` (Xbox PDB, `00485990`, cdecl: signature, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Whether the data written needs byte swapping (`00401500`, no arguments,
/// `bool` in `AL`).
const SAVE_SWAPS: u32 = 0x0040_1500;
/// Byte-swaps the 0xC-byte `COED` record in place (`005a6b10`, `this` only).
const SWAP_COED: u32 = 0x005a_6b10;
/// Byte-swaps the 8-byte `CNTO` record in place (`00462230`, `this` only).
const SWAP_CNTO: u32 = 0x0046_2230;
/// Whether the extra holds nothing but its defaults (`0040e9c0`, `this`
/// only, `bool` in `AL`).
const IS_DEFAULT: u32 = 0x0040_e9c0;
/// The `CNTO` chunk signature, as the word the exe pushes.
const SIG_CNTO: u32 = 0x4f54_4e43;

// Translated from 0040e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the extra as a `COED` chunk unless it holds only its defaults
/// (`0040e9c0`): a stack record of the owner's form id (0 without an
/// owner), the global's form id when there is one (`fn_0040e9f0`), else
/// the union word as it is (0 without an owner), and the health multiplier
/// (`FLD`/`FSTP`, a signalling NaN coming out quiet). The record is swapped
/// (`005a6b10`) before and after `TESForm::__AddChunkData` when `00401500`
/// says so.
pub fn fn_0040e6c0(e: &mut Engine, this: Ptr<ContainerItemExtra>) {
    if e.call(IS_DEFAULT, &args![this]).bool() {
        return;
    }
    e.with_stack(0xc, |e, record| {
        let owner = e.get(this, ContainerItemExtra::pOwnerForm);
        let (owner_id, union) = if owner == 0 {
            (0, 0)
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            let global = fn_0040e9f0(e, this);
            let union = if global == 0 {
                e.get(this, ContainerItemExtra::iUnion)
            } else {
                e.call(FORM_ID, &args![global]).u32()
            };
            (owner_id, union)
        };
        e.mem.set_u32(record.addr(), owner_id);
        e.mem.set_u32(record.addr() + 4, union);
        let health = x87_quiet_bits(e.get(this, ContainerItemExtra::fHealthMult));
        e.mem.set_u32(record.addr() + 8, health);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_COED, &args![record]);
        }
        e.call(ADD_CHUNK_DATA, &args![SIG_COED, record, 0xcu32]);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_COED, &args![record]);
        }
    });
}

// Translated from 0040e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The owner's global: the union word when there is an owner of form type
/// 0x2A (`00401170`), else 0.
pub fn fn_0040e9f0(e: &mut Engine, this: Ptr<ContainerItemExtra>) -> u32 {
    let owner = e.get(this, ContainerItemExtra::pOwnerForm);
    if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 0x2a {
        return e.get(this, ContainerItemExtra::iUnion);
    }
    0
}

// Translated from 00481310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a container entry (the engine map names it
/// `ContainerItemExtra::Save`, but `this` is the entry: the count at +0, the
/// form at +4, the `ContainerItemExtra*` at +8): a `CNTO` chunk of the form's
/// id and the count (swapped by `00462230` before and after
/// `TESForm::__AddChunkData` when `00401500` says so), then the extra's
/// `COED` chunk (`fn_0040e6c0`) when there is one.
pub fn fn_00481310(e: &mut Engine, this: Ptr) {
    // The entry's count (+0), form (+4) and extra (+8).
    let count = e.mem.u32(this.addr());
    let form = e.mem.u32(this.addr() + 4);
    e.with_stack(8, |e, record| {
        e.mem.set_u32(record.addr() + 4, count);
        let form_id = e.call(FORM_ID, &args![form]).u32();
        e.mem.set_u32(record.addr(), form_id);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_CNTO, &args![record]);
        }
        e.call(ADD_CHUNK_DATA, &args![SIG_CNTO, record, 8u32]);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_CNTO, &args![record]);
        }
    });
    let extra = e.mem.u32(this.addr() + 8);
    if extra != 0 {
        fn_0040e6c0(e, Ptr::new(extra));
    }
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040_e690,
            container_item_extra_container_item_extra(
                Ptr<ContainerItemExtra>,
            ) -> Ptr<ContainerItemExtra>
        ),
        entry!(
            0x0040_e780,
            fn_0040e780(Ptr<ContainerItemExtra>, Ptr) -> bool
        ),
        entry!(
            0x0040_e7f0,
            container_item_extra_init_item(Ptr<ContainerItemExtra>, Ptr)
        ),
        entry!(0x0040_e6c0, fn_0040e6c0(Ptr<ContainerItemExtra>)),
        entry!(0x0040_e9f0, fn_0040e9f0(Ptr<ContainerItemExtra>) -> u32),
        entry!(0x0048_1310, fn_00481310(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_values() {
        let mut e = Engine::new();
        let this: Ptr<ContainerItemExtra> = e.new_object();
        e.mem.write(this.addr(), &[0xcd; 0xc]);
        assert_eq!(e.call(0x0040_e690, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.f32(this.addr() + 8), 1.0);
    }

    #[test]
    fn load_reads_a_coed_chunk() {
        let mut e = Engine::new();
        let this: Ptr<ContainerItemExtra> = e.new_object();
        assert!(!e.call(0x0040_e780, &args![this, 0u32]).bool());
        e.register(0x0047_26b0, |_, _| ret(0x1234));
        assert!(!e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        e.register(0x0047_26b0, |_, _| ret(SIG_COED));
        e.register(0x0047_2890, |e, a| {
            assert_eq!(a[2], 0xc);
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u32(a[1] + 4, 0x22);
            e.mem.set_u32(a[1] + 8, 0x7f80_0003);
            Ret::default()
        });
        e.register(0x0040_1680, |_, _| ret(0));
        e.register(0x005a_6b10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        assert!(calls_to(e.call_log.as_ref().unwrap(), 0x005a_6b10).is_empty());
        assert_eq!(e.mem.u32(this.addr()), 0x11);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x22);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x7fc0_0003);
        // Swapped data: the swap runs on the stack copy before the copy in.
        e.register(0x0040_1680, |_, _| ret(1));
        e.register(0x005a_6b10, |e, a| {
            e.mem.set_u32(a[0], 0x33);
            Ret::default()
        });
        assert!(e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        assert_eq!(e.mem.u32(this.addr()), 0x33);
    }

    #[test]
    fn init_item_resolves_owner_and_global() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x18);
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(form, 0x0600_0000);
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(GET_FILE, |_, _| ret(0x0400_0000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            assert_eq!(a[1], 0x0400_0000);
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        // Ids ending in 1 resolve to forms of type 0x2a, ending in 2 to
        // type 0x1c.
        e.register(LOOKUP_FORM, |_, a| {
            ret(match a[0] & 0xf {
                1 | 2 => a[0] + 0x1000,
                _ => 0,
            })
        });
        e.register(FORM_TYPE, |_, a| {
            ret(if a[0] & 0xf == 1 { 0x2a } else { 0x1c })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(WARNING, |_, _| Ret::default());
        let this: Ptr<ContainerItemExtra> = e.new_object();
        let t = this.addr();
        // An NPC owner and a global that resolves.
        e.mem.set_u32(t, 1);
        e.mem.set_u32(t + 4, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t), 0x1101);
        assert_eq!(e.mem.u32(t + 4), 0x1111);
        assert!(calls_to(&log, WARNING).is_empty());
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x1111, 0, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0]]
        );
        // An NPC owner with a missing global: reported, null.
        e.mem.set_u32(t, 1);
        e.mem.set_u32(t + 4, 0x13);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t + 4), 0);
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![
                MISSING_GLOBAL_MESSAGE,
                0x113,
                0x0102_0304,
                0x00ab_cdef
            ]]
        );
        // Another owner type keeps the rank word.
        e.mem.set_u32(t, 2);
        e.mem.set_u32(t + 4, 5);
        e.call(0x0040_e7f0, &[t, form]);
        assert_eq!(e.mem.u32(t + 4), 5);
        // A missing owner: reported, and the union cleared.
        e.mem.set_u32(t, 3);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![MISSING_OWNER_MESSAGE, 0x103, 0x0102_0304, 0x00ab_cdef]]
        );
        assert_eq!(e.mem.u32(t + 4), 0);
        // No owner at all: the union is cleared without a lookup.
        e.mem.set_u32(t + 4, 9);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        assert!(calls_to(e.call_log.as_ref().unwrap(), LOOKUP_FORM).is_empty());
        assert_eq!(e.mem.u32(t + 4), 0);
    }

    type Chunks = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the save path: form ids are the form address plus 0x100,
    /// forms ending in 1 are of type 0x2A, a swap adds 0x1000 to each word of
    /// the record; returns the chunks written (signature and words).
    fn save_doubles(e: &mut Engine, swaps: bool) -> Chunks {
        e.register(FORM_ID, |_, a| ret(a[0] + 0x100));
        e.register(FORM_TYPE, |_, a| {
            ret(if a[0] & 0xf == 1 { 0x2a } else { 0x1c })
        });
        e.register(IS_DEFAULT, |_, _| ret(0x100));
        if swaps {
            e.register(SAVE_SWAPS, |_, _| ret(0x101));
        } else {
            e.register(SAVE_SWAPS, |_, _| ret(0x100));
        }
        e.register(SWAP_COED, |e, a| {
            for i in 0..3 {
                let w = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, w + 0x1000);
            }
            Ret::default()
        });
        e.register(SWAP_CNTO, |e, a| {
            for i in 0..2 {
                let w = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, w + 0x1000);
            }
            Ret::default()
        });
        let chunks = Chunks::default();
        let seen = chunks.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            let words = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push((a[0], words));
            Ret::default()
        });
        chunks
    }

    #[test]
    fn global_of_owner_needs_an_owner_of_type_0x2a() {
        let mut e = Engine::new();
        save_doubles(&mut e, false);
        let this: Ptr<ContainerItemExtra> = e.new_object();
        e.mem.set_u32(this.addr() + 4, 0x77);
        assert_eq!(e.call(0x0040_e9f0, &args![this]).u32(), 0);
        e.mem.set_u32(this.addr(), 0x1002);
        assert_eq!(e.call(0x0040_e9f0, &args![this]).u32(), 0);
        e.mem.set_u32(this.addr(), 0x1001);
        assert_eq!(e.call(0x0040_e9f0, &args![this]).u32(), 0x77);
    }

    #[test]
    fn coed_save_writes_ids_and_the_multiplier() {
        let mut e = Engine::new();
        let chunks = save_doubles(&mut e, false);
        let this: Ptr<ContainerItemExtra> = e.new_object();
        let t = this.addr();
        // An owner of type 0x2A with a global: both as form ids.
        e.mem.set_u32(t, 0x1001);
        e.mem.set_u32(t + 4, 0x2000);
        e.mem.set_u32(t + 8, 0x7f80_0001);
        e.call(0x0040_e6c0, &args![this]);
        // Another owner: the rank word as it is.
        e.mem.set_u32(t, 0x1002);
        e.mem.set_u32(t + 4, 3);
        e.call(0x0040_e6c0, &args![this]);
        // No owner: zeros.
        e.mem.set_u32(t, 0);
        e.call(0x0040_e6c0, &args![this]);
        assert_eq!(
            *chunks.borrow(),
            vec![
                (SIG_COED, vec![0x1101, 0x2100, 0x7fc0_0001]),
                (SIG_COED, vec![0x1102, 3, 0x7fc0_0001]),
                (SIG_COED, vec![0, 0, 0x7fc0_0001]),
            ]
        );
        // Swapped before the write; a default extra writes nothing.
        let chunks = save_doubles(&mut e, true);
        e.call(0x0040_e6c0, &args![this]);
        e.register(IS_DEFAULT, |_, _| ret(1));
        e.call(0x0040_e6c0, &args![this]);
        assert_eq!(
            *chunks.borrow(),
            vec![(SIG_COED, vec![0x1000, 0x1000, 0x7fc0_1001])]
        );
    }

    #[test]
    fn container_entry_save_writes_cnto_then_coed() {
        let mut e = Engine::new();
        let chunks = save_doubles(&mut e, true);
        let extra: Ptr<ContainerItemExtra> = e.new_object();
        e.mem.set_u32(extra.addr(), 0x1002);
        e.mem.set_u32(extra.addr() + 4, 4);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry, 5);
        e.mem.set_u32(entry + 4, 0x3000);
        e.call(0x0048_1310, &[entry]);
        e.mem.set_u32(entry + 8, extra.addr());
        e.call(0x0048_1310, &[entry]);
        let cnto = (SIG_CNTO, vec![0x4100, 0x1005]);
        assert_eq!(
            *chunks.borrow(),
            vec![cnto.clone(), cnto, (SIG_COED, vec![0x2102, 0x1004, 0x1000]),]
        );
    }
}
