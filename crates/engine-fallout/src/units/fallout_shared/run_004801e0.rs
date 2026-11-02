//! `fallout shared/run_004801e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two groups of functions:
//!
//! * The save-game functions of a 7-byte block at +4 of a component that
//!   `TESActorBase`'s save functions (`005f16f0` ... `005f1fd0`) call at
//!   +0xB8 of the actor base. Their change-flag masks were folded to zero by
//!   the compiler in three of them, so those blocks never run.
//! * `TESBipedModelForm`'s constructor and destructor and `TESModelRDT`'s
//!   constructor (classes from their vtables' RTTI). `TESBipedModelForm`
//!   (Xbox PDB, 0xDC bytes on both builds): the `BaseFormComponent` vtable,
//!   `bipedModelData` +0x04, `bipedModel` +0x0C and `worldModel` +0x4C
//!   (two `TESModelTextureSwap` of 0x20 bytes each), `inventoryIcon` +0x8C
//!   (two `TESIcon` of 0x0C), `messageIcon` +0xA4 (two `BGSMessageIcon` of
//!   0x10) and `ConstraintTemplate` +0xC4 (`TESModelRDT`).

#[allow(unused_imports)]
use crate::prelude::*;

/// Copies the word at +0x17 of a save/load buffer into `*out` and returns
/// `out` (`(buffer, out)`).
const BUFFER_CHANGE_FLAGS: u32 = 0x0042_8110;
/// Whether `*flags & mask` is non-zero (`(flags, mask)`, `bool`).
const FLAGS_TEST: u32 = 0x0042_80f0;
/// Writes bytes to a save buffer (`(buffer, data, size, 0)`).
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
/// Reads bytes from a load buffer (`(buffer, data, size)`).
const LOAD_BUFFER_READ: u32 = 0x0086_4980;
/// The change flag the two buffer functions test.
const CHANGE_FLAG: u32 = 4;
/// Size of the block at +4 that is saved and loaded.
const BLOCK_SIZE: u32 = 7;

/// `BaseFormComponent`'s constructor (`(this)`).
const BASE_FORM_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// `TESBipedModelForm`'s vtable (RTTI `.?AVTESBipedModelForm@@`).
const BIPED_MODEL_FORM_VTABLE: u32 = 0x0101_bfd8;
/// `TESModelRDT`'s vtable (RTTI `.?AVTESModelRDT@@`).
const MODEL_RDT_VTABLE: u32 = 0x0101_bfec;
/// `` `eh vector constructor iterator' `` (Ghidra's library match)
/// (`(array, size, count, ctor, dtor)`, stdcall).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `` `eh vector destructor iterator' `` (the engine map's library name)
/// (`(array, size, count, dtor)`, stdcall).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The element constructor and destructor of the two
/// `TESModelTextureSwap` arrays.
const TEXTURE_SWAP_CONSTRUCT: u32 = 0x0048_a3d0;
const TEXTURE_SWAP_DESTRUCT: u32 = 0x0048_a490;
/// The element constructor and destructor of the `TESIcon` array.
const ICON_CONSTRUCT: u32 = 0x0040_2c20;
const ICON_DESTRUCT: u32 = 0x0040_2c40;
/// The element constructor and destructor of the `BGSMessageIcon` array.
const MESSAGE_ICON_CONSTRUCT: u32 = 0x0047_a130;
const MESSAGE_ICON_DESTRUCT: u32 = 0x0047_a1a0;
/// `TESBipedModelForm::InitializeDataComponent` (Xbox PDB).
const INITIALIZE_DATA_COMPONENT: u32 = 0x0048_04d0;
/// `TESBipedModelForm::ClearDataComponent` (Xbox PDB).
const CLEAR_DATA_COMPONENT: u32 = 0x0048_0570;
/// `TESModelRDT`'s base constructor (`(this)`, called first by
/// [`fn_004803e0`]).
const MODEL_RDT_BASE_CONSTRUCT: u32 = 0x0048_8f50;
/// The destructor `00480400` runs on `ConstraintTemplate` (`(this)`).
const MODEL_RDT_DESTRUCT: u32 = 0x0051_b950;

// Translated from 004801e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of the save-game data for `flags`: always 0. The `flags & 0` block
/// (that would add 7) never runs. Returns a 16-bit value.
pub fn fn_004801e0(_e: &mut Engine, _this: Ptr, _flags: u32) -> u16 {
    0
}

// Translated from 00480210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves nothing: the `flags & 0` block that would write the 7 bytes at
/// +4 through the save/load game object (`011de45c`, `008579b0`) never
/// runs.
pub fn fn_00480210(_e: &mut Engine, _this: Ptr, _flags: u32) {}

// Translated from 00480240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads nothing: the `flags & 0` block that would read the 7 bytes at +4
/// through the save/load game object (`011de45c`, `008579e0`) never runs.
/// The second word is never read.
pub fn fn_00480240(_e: &mut Engine, _this: Ptr, _flags: u32, _unused_2: u32) {}

/// Whether the save/load buffer's change flags (`00428110` into a stack
/// word) have [`CHANGE_FLAG`] (`004280f0`).
fn buffer_has_change_flag(e: &mut Engine, buffer: Ptr) -> bool {
    e.with_stack(4, |e, flags| {
        let flags = e.call(BUFFER_CHANGE_FLAGS, &args![buffer, flags]).u32();
        e.call(FLAGS_TEST, &args![flags, CHANGE_FLAG]).bool()
    })
}

// Translated from 00480270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the 7 bytes at +4 to the save buffer `buffer` (`00865e50`) when
/// the buffer's change flags have bit 2.
pub fn fn_00480270(e: &mut Engine, this: Ptr, buffer: Ptr) {
    if buffer_has_change_flag(e, buffer) {
        e.call(
            SAVE_BUFFER_WRITE,
            &args![buffer, this.addr() + 4, BLOCK_SIZE, 0u32],
        );
    }
}

// Translated from 004802b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the 7 bytes at +4 from the load buffer `buffer` (`00864980`) when
/// the buffer's change flags have bit 2.
pub fn fn_004802b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    if buffer_has_change_flag(e, buffer) {
        e.call(
            LOAD_BUFFER_READ,
            &args![buffer, this.addr() + 4, BLOCK_SIZE],
        );
    }
}

// Translated from 004802f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm` constructor: the `BaseFormComponent` base, its
/// vtable, the two `TESModelTextureSwap` arrays, the `TESIcon` and
/// `BGSMessageIcon` arrays (two elements each), `ConstraintTemplate`
/// ([`fn_004803e0`]), then `InitializeDataComponent`. Returns `this`.
// Translation incomplete: 004802f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the members built so far when a constructor throws is not translated.
pub fn fn_004802f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_FORM_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, BIPED_MODEL_FORM_VTABLE);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0xc,
            0x20u32,
            2u32,
            TEXTURE_SWAP_CONSTRUCT,
            TEXTURE_SWAP_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x4c,
            0x20u32,
            2u32,
            TEXTURE_SWAP_CONSTRUCT,
            TEXTURE_SWAP_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![base + 0x8c, 0xcu32, 2u32, ICON_CONSTRUCT, ICON_DESTRUCT],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0xa4,
            0x10u32,
            2u32,
            MESSAGE_ICON_CONSTRUCT,
            MESSAGE_ICON_DESTRUCT
        ],
    );
    fn_004803e0(e, Ptr::new(base + 0xc4));
    e.call(INITIALIZE_DATA_COMPONENT, &args![this]);
    this
}

// Translated from 004803e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelRDT` constructor: its base (`00488f50`), then its vtable.
/// Returns `this`.
pub fn fn_004803e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MODEL_RDT_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), MODEL_RDT_VTABLE);
    this
}

// Translated from 00480400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm` destructor: its vtable, `ClearDataComponent`, then
/// the members in reverse order (`ConstraintTemplate` through `0051b950`,
/// the message icons, icons and both model arrays).
// Translation incomplete: 00480400 - the C++ exception-unwinding frame (FS:[0]) that keeps destroying the remaining members when a destructor throws is not translated.
pub fn fn_00480400(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BIPED_MODEL_FORM_VTABLE);
    e.call(CLEAR_DATA_COMPONENT, &args![this]);
    e.call(MODEL_RDT_DESTRUCT, &args![base + 0xc4]);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0xa4, 0x10u32, 2u32, MESSAGE_ICON_DESTRUCT],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x8c, 0xcu32, 2u32, ICON_DESTRUCT],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x4c, 0x20u32, 2u32, TEXTURE_SWAP_DESTRUCT],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0xc, 0x20u32, 2u32, TEXTURE_SWAP_DESTRUCT],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004801e0, fn_004801e0(Ptr, u32) -> u16),
        entry!(0x00480210, fn_00480210(Ptr, u32)),
        entry!(0x00480240, fn_00480240(Ptr, u32, u32)),
        entry!(0x00480270, fn_00480270(Ptr, Ptr)),
        entry!(0x004802b0, fn_004802b0(Ptr, Ptr)),
        entry!(0x004802f0, fn_004802f0(Ptr) -> Ptr),
        entry!(0x004803e0, fn_004803e0(Ptr) -> Ptr),
        entry!(0x00480400, fn_00480400(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        log(e)
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args)
            .collect()
    }

    #[test]
    fn folded_save_functions_do_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_01e0, &[this, 0xffff_ffff]).u32(), 0);
        e.call(0x0048_0210, &[this, 0xffff_ffff]);
        e.call(0x0048_0240, &[this, 0xffff_ffff, 0]);
        assert_eq!(log(&e).len(), 3);
    }

    /// Doubles the change-flag pair the way the exe's bodies work: the
    /// buffer's word at +0x17 is copied out and tested against the mask.
    fn double_flag_helpers(e: &mut Engine) {
        e.register(BUFFER_CHANGE_FLAGS, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| (e.mem.u32(a[0]) & a[1] != 0).into_ret());
        e.register(SAVE_BUFFER_WRITE, |_, _| Ret::default());
        e.register(LOAD_BUFFER_READ, |_, _| Ret::default());
    }

    #[test]
    fn save_and_load_need_change_flag_four() {
        let mut e = Engine::new();
        double_flag_helpers(&mut e);
        let this = e.mem.alloc(0xc);
        let buffer = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.mem.set_u32(buffer + 0x17, 3);
        e.call(0x0048_0270, &[this, buffer]);
        e.call(0x0048_02b0, &[this, buffer]);
        assert!(calls_to(&e, SAVE_BUFFER_WRITE).is_empty());
        assert!(calls_to(&e, LOAD_BUFFER_READ).is_empty());
        assert_eq!(calls_to(&e, FLAGS_TEST).len(), 2);
        assert_eq!(calls_to(&e, FLAGS_TEST)[0][1], 4);

        e.mem.set_u32(buffer + 0x17, 4);
        e.call(0x0048_0270, &[this, buffer]);
        e.call(0x0048_02b0, &[this, buffer]);
        assert_eq!(
            calls_to(&e, SAVE_BUFFER_WRITE),
            vec![vec![buffer, this + 4, 7, 0]]
        );
        assert_eq!(
            calls_to(&e, LOAD_BUFFER_READ),
            vec![vec![buffer, this + 4, 7]]
        );
    }

    #[test]
    fn biped_model_form_constructor_builds_members_in_order() {
        let mut e = Engine::new();
        for addr in [
            BASE_FORM_COMPONENT_CONSTRUCT,
            VECTOR_CONSTRUCT,
            MODEL_RDT_BASE_CONSTRUCT,
            INITIALIZE_DATA_COMPONENT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_02f0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), BIPED_MODEL_FORM_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc4), MODEL_RDT_VTABLE);
        let calls: Vec<_> = log(&e).into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (BASE_FORM_COMPONENT_CONSTRUCT, vec![this]),
                (
                    VECTOR_CONSTRUCT,
                    vec![
                        this + 0xc,
                        0x20,
                        2,
                        TEXTURE_SWAP_CONSTRUCT,
                        TEXTURE_SWAP_DESTRUCT
                    ]
                ),
                (
                    VECTOR_CONSTRUCT,
                    vec![
                        this + 0x4c,
                        0x20,
                        2,
                        TEXTURE_SWAP_CONSTRUCT,
                        TEXTURE_SWAP_DESTRUCT
                    ]
                ),
                (
                    VECTOR_CONSTRUCT,
                    vec![this + 0x8c, 0xc, 2, ICON_CONSTRUCT, ICON_DESTRUCT]
                ),
                (
                    VECTOR_CONSTRUCT,
                    vec![
                        this + 0xa4,
                        0x10,
                        2,
                        MESSAGE_ICON_CONSTRUCT,
                        MESSAGE_ICON_DESTRUCT
                    ]
                ),
                (MODEL_RDT_BASE_CONSTRUCT, vec![this + 0xc4]),
                (INITIALIZE_DATA_COMPONENT, vec![this]),
            ]
        );
    }

    #[test]
    fn model_rdt_constructor_sets_vtable_after_base() {
        let mut e = Engine::new();
        e.register(MODEL_RDT_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x0048_03e0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), MODEL_RDT_VTABLE);
    }

    #[test]
    fn biped_model_form_destructor_tears_down_in_reverse() {
        let mut e = Engine::new();
        for addr in [CLEAR_DATA_COMPONENT, MODEL_RDT_DESTRUCT, VECTOR_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        e.call(0x0048_0400, &[this]);
        assert_eq!(e.mem.u32(this), BIPED_MODEL_FORM_VTABLE);
        let calls: Vec<_> = log(&e).into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (CLEAR_DATA_COMPONENT, vec![this]),
                (MODEL_RDT_DESTRUCT, vec![this + 0xc4]),
                (
                    VECTOR_DESTRUCT,
                    vec![this + 0xa4, 0x10, 2, MESSAGE_ICON_DESTRUCT]
                ),
                (VECTOR_DESTRUCT, vec![this + 0x8c, 0xc, 2, ICON_DESTRUCT]),
                (
                    VECTOR_DESTRUCT,
                    vec![this + 0x4c, 0x20, 2, TEXTURE_SWAP_DESTRUCT]
                ),
                (
                    VECTOR_DESTRUCT,
                    vec![this + 0xc, 0x20, 2, TEXTURE_SWAP_DESTRUCT]
                ),
            ]
        );
    }
}
