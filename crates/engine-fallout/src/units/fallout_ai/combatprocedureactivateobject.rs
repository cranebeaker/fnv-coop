//! `fallout/ai/combat/combatprocedureactivateobject.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009ca740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0109082c` (RTTI `.?AVCombatProcedureActivateObject@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009ca740(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_082c);
    e.call(0x009c_a6c0, &args![this]);
}

// ---- Batch n023 ----------------------------------------------------------------------------

/// The vtables of `CombatProcedure` (`01090874`) and `CombatProcedureActivateObject`
/// (`0109082c`), RTTI names.
const PROCEDURE_VTABLE: u32 = 0x0109_0874;
const ACTIVATE_OBJECT_VTABLE: u32 = 0x0109_082c;
/// `TESObjectREFR::SetTargeted` (engine-map name of `00564db0`; `thiscall`, `RET 4`: a byte).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `"Targeted reference is NULL for some reason"` and `"Targeted reference pointer was
/// cleared"`.
const NULL_REFERENCE_TEXT: u32 = 0x0109_08b8;
const CLEARED_REFERENCE_TEXT: u32 = 0x0109_08e4;
/// `"ANIM PLAYING"`, `"ANIM GRABBING"`, `"ANIM FINISHING PLAYING"`, `"NO ANIM"` and
/// `"Target: '%s' (%08X) - Position: (%.02f,%.02f,%.02f)"`.
const ANIM_PLAYING_TEXT: u32 = 0x0109_0968;
const ANIM_GRABBING_TEXT: u32 = 0x0109_0958;
const ANIM_FINISHING_TEXT: u32 = 0x0109_0940;
const NO_ANIM_TEXT: u32 = 0x0109_0978;
const TARGET_FORMAT: u32 = 0x0109_090c;
/// The procedure's save (`00996ed0`) and load (`00996ef0`), in `combatprocedure.cpp`.
const PROCEDURE_SAVE_GAME: u32 = 0x0099_6ed0;
const PROCEDURE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `CombatProcedure::GetAttacker` (`00996de0`).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// `007af430` (`thiscall` on a reference; engine map `BGSSaveFormBuffer::GetForm`): its form.
const REFERENCE_FORM: u32 = 0x007a_f430;

// Translated from 009ca680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedure` base constructor (by its vtable `01090874`; `ECX` only): the vtable,
/// then the words at `+4`, `+8` and `+0xc` cleared. Returns `this`.
pub fn fn_009ca680(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, PROCEDURE_VTABLE);
    e.mem.set_u32(me.wrapping_add(4), 0);
    e.mem.set_u32(me.wrapping_add(8), 0);
    e.mem.set_u32(me.wrapping_add(0xc), 0);
    this
}

// Translated from 009ca6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedure` base destructor (by its vtable `01090874`; `ECX` only): stores the
/// vtable.
pub fn fn_009ca6c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PROCEDURE_VTABLE);
}

// Translated from 009ca6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedure` scalar deleting destructor (by its body; `thiscall`, `RET 4`):
/// `009ca6c0`, then `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009ca6e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009ca6c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009ca710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureActivateObject` scalar deleting destructor (by its body; `thiscall`,
/// `RET 4`): `009ca740`, then `00401030` frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009ca710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009ca740(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009ca5f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ca5f0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatProcedureActivateObject::CombatProcedureActivateObject` (Xbox PDB; `thiscall`,
/// `RET 8`): the base constructor (`009ca680`), the vtable `0109082c`, the reference at
/// `+0x10`, `form` at `+0x14`, the animation state byte at `+0x18` cleared, and a non-null
/// reference marked targeted (`SetTargeted(1)`). Returns `this`.
pub fn combat_procedure_activate_object_construct(
    e: &mut Engine,
    this: Ptr,
    reference: Ptr,
    form: u32,
) -> Ptr {
    let me = this.addr();
    fn_009ca680(e, this);
    e.mem.set_u32(me, ACTIVATE_OBJECT_VTABLE);
    e.mem.set_u32(me.wrapping_add(0x10), reference.addr());
    e.mem.set_u32(me.wrapping_add(0x14), form);
    e.mem.set_u8(me.wrapping_add(0x18), 0);
    let reference = e.mem.u32(me.wrapping_add(0x10));
    if reference != 0 {
        e.call(SET_TARGETED, &args![reference, 1u32]);
    }
    this
}

// Translated from 009caac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the procedure fails with `text`: `+0xc` =
/// `text`, `+8` = 1.
pub fn fn_009caac0(e: &mut Engine, this: Ptr, text: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0xc), text);
    e.mem.set_u32(this.addr().wrapping_add(8), 1);
}

// Translated from 009caae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the procedure is done: `+8` = 2.
pub fn fn_009caae0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(8), 2);
}

// Translated from 009cab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::ClearReferencePointers` (Xbox PDB; `thiscall`, `RET 4`):
/// when `reference` is the targeted one (`+0x10`), clears it and fails the procedure with
/// `"Targeted reference pointer was cleared"` (`009caac0`).
pub fn combat_procedure_activate_object_clear_reference_pointers(
    e: &mut Engine,
    this: Ptr,
    reference: Ptr,
) {
    if e.mem.u32(this.addr().wrapping_add(0x10)) == reference.addr() {
        e.mem.set_u32(this.addr().wrapping_add(0x10), 0);
        fn_009caac0(e, this, CLEARED_REFERENCE_TEXT);
    }
}

// Translated from 009ca760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::Update` (Xbox PDB; `ECX` only). Fails with `"Targeted
/// reference is NULL for some reason"` without a reference (`+0x10`). With the attacker
/// (`GetAttacker`), its animation (slot `+0x1e4`) and `0045cd60` of its `008d8520` answering
/// 0, the state byte (`+0x18`) moves on: 0 asks for a special idle (`008ff0b0` on the
/// attacker's `008d8520` with the attacker, the reference's form `007af430` and the
/// reference) once the animation is not busy (`004985f0`, `00498d30(0)`), becoming 1 or, when
/// none plays, -1 (and activating now); 1 and 3 wait while `004985f0` passes and `00498f80`
/// fails; in state 1 the playing sequence (`008da3d0`, its name through slot `+0x14` of
/// `+0x18`) gives the grab time after its last `_` (`strrchr`, `atof`; 0 without one), and
/// state 2 comes once there is no controller (`00921e30`) or the scaled time (`004eec60` of
/// `00453700`) is past it; other states activate. Activation (all but state 3): with the
/// controller's state (`00586150` of `+4`), a reference whose form is not `+0x14` gives up to
/// 5 of the state's entries (`0097cfc0`) for it, and each of their items with a non-zero
/// count in it (`00575610`, made positive) is activated (`00573170`) after
/// `PreActivateReference` (`0099a780`); otherwise the reference itself is activated
/// (`00573170(attacker, 1, 0, 1)`). It then stops unless `00493bb0` passes the attacker, and
/// runs `009993c0` on the state. Finally state 2 becomes 3 and any other state completes the
/// procedure (`009caae0`).
pub fn combat_procedure_activate_object_update(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let state_of = |e: &Engine| e.mem.u8(me.wrapping_add(0x18)) as i8;
    if e.mem.u32(me.wrapping_add(0x10)) == 0 {
        fn_009caac0(e, this, NULL_REFERENCE_TEXT);
        return;
    }
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let animation = e.vcall(attacker, 0x1e4, &[]).u32();
    let process = e.call(0x008d_8520, &args![attacker]).u32();
    'activate: {
        if e.call(0x0045_cd60, &args![process]).u32() != 0 || animation == 0 {
            break 'activate;
        }
        if state_of(e) == 0 {
            if animation != 0 {
                if !e.call(0x0049_85f0, &args![animation]).bool() {
                    return;
                }
                if e.call(0x0049_8d30, &args![animation, 0u32]).bool() {
                    return;
                }
            }
            let process = e.call(0x008d_8520, &args![attacker]).u32();
            let reference = e.mem.u32(me.wrapping_add(0x10));
            let form = e.call(REFERENCE_FORM, &args![reference]).u32();
            let reference = e.mem.u32(me.wrapping_add(0x10));
            if e.call(0x008f_f0b0, &args![process, attacker, form, reference])
                .bool()
            {
                e.mem.set_u8(me.wrapping_add(0x18), 1);
                return;
            }
            e.mem.set_u8(me.wrapping_add(0x18), 0xff);
            break 'activate;
        }
        let state = state_of(e);
        if (state != 1 && state != 3) || animation == 0 {
            break 'activate;
        }
        if e.call(0x0049_85f0, &args![animation]).bool()
            && !e.call(0x0049_8f80, &args![animation]).bool()
        {
            break 'activate;
        }
        if state_of(e) == 1 {
            let sequence = e.call(0x008d_a3d0, &args![animation]).u32();
            if sequence != 0 {
                let holder = sequence.wrapping_add(0x18);
                let name = e.vcall(holder, 0x14, &[]).u32();
                if name != 0 {
                    let mut grab = x87_store32(0.0);
                    let underscore = e.call(0x00ec_6e30, &args![name, 0x5fu32]).u32();
                    if underscore != 0 {
                        let value = e
                            .call(0x00ec_a573, &args![underscore.wrapping_add(1)])
                            .f64();
                        grab = x87_store32(value);
                    }
                    let controller = e.call(0x0092_1e30, &args![animation]).u32();
                    let reached = if controller != 0 {
                        let time = x87_store32(e.call(0x0045_3700, &args![animation]).f64());
                        let scaled = e.call(0x004e_ec60, &args![controller, time]).f64();
                        x87_load32(grab) < scaled
                    } else {
                        true
                    };
                    if reached {
                        e.mem.set_u8(me.wrapping_add(0x18), 2);
                    }
                }
            }
        }
        if state_of(e) != 2 {
            return;
        }
    }
    if state_of(e) != 3 {
        let controller = e.mem.u32(me.wrapping_add(4));
        let combat_state = e.call(0x0058_6150, &args![controller]).u32();
        let reference = e.mem.u32(me.wrapping_add(0x10));
        let form = e.call(REFERENCE_FORM, &args![reference]).u32();
        if form != e.mem.u32(me.wrapping_add(0x14)) {
            // The frame: up to 5 items at +0.
            e.with_stack(0x14, |e, items| {
                let list = items.addr();
                e.call(0x0040_3d30, &args![list, 0u32, 0x14u32]);
                let mut count = 0u32;
                for i in 0..5u32 {
                    let entry = e.call(0x0097_cfc0, &args![combat_state, i]).u32();
                    if entry != 0 && e.mem.u32(entry) == e.mem.u32(me.wrapping_add(0x10)) {
                        let item = e.mem.u32(entry.wrapping_add(4));
                        e.mem
                            .set_u32(list.wrapping_add(count.wrapping_mul(4)), item);
                        count = count.wrapping_add(1);
                    }
                }
                let reference = e.mem.u32(me.wrapping_add(0x10));
                e.call(0x0099_a780, &args![combat_state, reference]);
                let mut j = 0u32;
                while j < count {
                    let item = e.mem.u32(list.wrapping_add(j.wrapping_mul(4)));
                    let reference = e.mem.u32(me.wrapping_add(0x10));
                    let mut held = e.call(0x0057_5610, &args![reference, item]).i32();
                    if held != 0 {
                        if held < 0 {
                            held = held.wrapping_neg();
                        }
                        let item = e.mem.u32(list.wrapping_add(j.wrapping_mul(4)));
                        let reference = e.mem.u32(me.wrapping_add(0x10));
                        e.call(
                            0x0057_3170,
                            &args![reference, attacker, 1u32, item, held as u32],
                        );
                    }
                    j = j.wrapping_add(1);
                }
            });
        } else {
            let reference = e.mem.u32(me.wrapping_add(0x10));
            e.call(0x0099_a780, &args![combat_state, reference]);
            let reference = e.mem.u32(me.wrapping_add(0x10));
            e.call(0x0057_3170, &args![reference, attacker, 1u32, 0u32, 1u32]);
        }
        if !e.call(0x0049_3bb0, &args![attacker]).bool() {
            return;
        }
        e.call(0x0099_93c0, &args![combat_state]);
    }
    if state_of(e) == 2 {
        e.mem.set_u8(me.wrapping_add(0x18), 3);
    } else {
        fn_009caae0(e, this);
    }
}

// Translated from 009cab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::DisplayDebugText` (Xbox PDB; `thiscall`, `RET 0xc`): prints
/// through the printer's slot `+0x14` (`cdecl`: printer, the two words, text, ...) the
/// animation state (`"NO ANIM"` for -1, nothing for 0, `"ANIM PLAYING"`, `"ANIM GRABBING"`,
/// `"ANIM FINISHING PLAYING"` for 1..=3; the jump table at `009cac9c`, as arms), then for a
/// reference `"Target: '%s' (%08X) - Position: (%.02f,%.02f,%.02f)"` with its name (slot
/// `+0x130` when `00474cb0` answers non-zero, else `0055d520`), `0084e3a0` and its position
/// (slot `+0x1f4`, read before the name) as doubles.
pub fn combat_procedure_activate_object_display_debug_text(
    e: &mut Engine,
    this: Ptr,
    printer: Ptr,
    first: u32,
    second: u32,
) {
    let me = this.addr();
    let p = printer.addr();
    let state = e.mem.u8(me.wrapping_add(0x18)) as i8;
    let text = match (state as i32).wrapping_add(1) {
        0 => Some(NO_ANIM_TEXT),
        2 => Some(ANIM_PLAYING_TEXT),
        3 => Some(ANIM_GRABBING_TEXT),
        4 => Some(ANIM_FINISHING_TEXT),
        _ => None,
    };
    if let Some(text) = text {
        e.vcall(p, 0x14, &args![first, second, text]);
    }
    let reference = e.mem.u32(me.wrapping_add(0x10));
    if reference == 0 {
        return;
    }
    let position = e.vcall(reference, 0x1f4, &[]).u32();
    let x = e.mem.u32(position);
    let y = e.mem.u32(position.wrapping_add(4));
    let z = e.mem.u32(position.wrapping_add(8));
    let reference = e.mem.u32(me.wrapping_add(0x10));
    let name = if e.call(0x0047_4cb0, &args![reference]).u32() != 0 {
        let reference = e.mem.u32(me.wrapping_add(0x10));
        e.vcall(reference, 0x130, &[]).u32()
    } else {
        let reference = e.mem.u32(me.wrapping_add(0x10));
        e.call(0x0055_d520, &args![reference]).u32()
    };
    let reference = e.mem.u32(me.wrapping_add(0x10));
    let id = e.call(0x0084_e3a0, &args![reference]).u32();
    e.vcall(
        p,
        0x14,
        &args![
            first,
            second,
            TARGET_FORMAT,
            name,
            id,
            x87_load32(x),
            x87_load32(y),
            x87_load32(z)
        ],
    );
}

// Translated from 009cacb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's
/// own save (`00996ed0`), the reference's and the form's ids (`00865df0`), then the state
/// byte (`00865e50`, 1 byte).
pub fn combat_procedure_activate_object_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(PROCEDURE_SAVE_GAME, &args![this, buffer]);
    let reference = e.mem.u32(me.wrapping_add(0x10));
    e.call(0x0086_5df0, &args![buffer, reference, 0u32]);
    let form = e.mem.u32(me.wrapping_add(0x14));
    e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    e.call(
        0x0086_5e50,
        &args![buffer, me.wrapping_add(0x18), 1u32, 0u32],
    );
}

// Translated from 009cad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the procedure's
/// own load (`00996ef0`), the reference's id into `+0x10` (`008648e0`), the form
/// (`008648a0`, `004839c0`, `__RTDynamicCast(.., 0, 01183028, 01183108, 0)`) into `+0x14`,
/// then the state byte (`00864980`, 1 byte).
pub fn combat_procedure_activate_object_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(PROCEDURE_LOAD_GAME, &args![this, buffer]);
    e.call(0x0086_48e0, &args![buffer, me.wrapping_add(0x10)]);
    let loaded = e.call(0x0086_48a0, &args![buffer]).u32();
    let object = e.call(0x0048_39c0, &args![loaded]).u32();
    let form = e
        .call(
            0x00ec_43fb,
            &args![object, 0u32, 0x0118_3028u32, 0x0118_3108u32, 0u32],
        )
        .u32();
    e.mem.set_u32(me.wrapping_add(0x14), form);
    e.call(0x0086_4980, &args![buffer, me.wrapping_add(0x18), 1u32]);
}

// Translated from 009cad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureActivateObject::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): `004534f0`
/// on the procedure, then the reference id at `+0x10` resolved (`004839c0`,
/// `__RTDynamicCast(.., 0, 01183028, 011841cc, 0)`; null stays null) and, when non-null,
/// marked targeted (`SetTargeted(1)`).
pub fn combat_procedure_activate_object_init_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(0x0045_34f0, &args![this, buffer]);
    let id = e.mem.u32(me.wrapping_add(0x10));
    let reference = if id != 0 {
        let object = e.call(0x0048_39c0, &args![id]).u32();
        e.call(
            0x00ec_43fb,
            &args![object, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(me.wrapping_add(0x10), reference);
    let reference = e.mem.u32(me.wrapping_add(0x10));
    if reference != 0 {
        e.call(SET_TARGETED, &args![reference, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_a740, fn_009ca740(Ptr)),
        entry!(0x009c_a680, fn_009ca680(Ptr) -> Ptr),
        entry!(0x009c_a6c0, fn_009ca6c0(Ptr)),
        entry!(0x009c_a6e0, fn_009ca6e0(Ptr, u32) -> Ptr),
        entry!(0x009c_a710, fn_009ca710(Ptr, u32) -> Ptr),
        entry!(
            0x009c_a5f0,
            combat_procedure_activate_object_construct(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(0x009c_aac0, fn_009caac0(Ptr, u32)),
        entry!(0x009c_aae0, fn_009caae0(Ptr)),
        entry!(
            0x009c_ab00,
            combat_procedure_activate_object_clear_reference_pointers(Ptr, Ptr)
        ),
        entry!(0x009c_a760, combat_procedure_activate_object_update(Ptr)),
        entry!(
            0x009c_ab30,
            combat_procedure_activate_object_display_debug_text(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x009c_acb0,
            combat_procedure_activate_object_save_game(Ptr, Ptr)
        ),
        entry!(
            0x009c_ad00,
            combat_procedure_activate_object_load_game(Ptr, Ptr)
        ),
        entry!(
            0x009c_ad70,
            combat_procedure_activate_object_init_load_game(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009ca740_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009c_a740, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_082c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_082c,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n023_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    const VTABLE: u32 = 0x0e00_0000;

    const fn slot(offset: u32) -> u32 {
        0x0f00_0000 + offset
    }

    fn object(e: &mut Engine, words: &[u32]) -> u32 {
        if !e.mem.is_mapped(VTABLE) {
            let slots: Vec<u32> = (0..0x80).map(|i| slot(4 * i)).collect();
            e.put_vtable(VTABLE, &slots);
        }
        let o = e.mem.alloc(4 + 4 * words.len() as u32);
        e.mem.set_u32(o, VTABLE);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(o + 4 + 4 * i as u32, *w);
        }
        o
    }

    #[test]
    fn the_base_constructor_clears_its_words() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x10);
        e.mem.write(p, &[0xcd; 0x10]);
        assert_eq!(e.call(0x009c_a680, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), PROCEDURE_VTABLE);
        assert_eq!(e.mem.bytes(p + 4, 12), vec![0; 12]);
    }

    #[test]
    fn the_base_destructor_stores_its_vtable() {
        let mut e = Engine::new();
        let p = e.mem.alloc(4);
        e.call(0x009c_a6c0, &args![p]);
        assert_eq!(e.mem.u32(p), PROCEDURE_VTABLE);
    }

    #[test]
    fn the_base_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[FREE]);
        let p = e.mem.alloc(4);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009c_a6e0, &args![p, flags]).u32(), p);
            assert_eq!(e.mem.u32(p), PROCEDURE_VTABLE);
            assert_eq!(calls(&e, FREE).len(), frees);
        }
    }

    #[test]
    fn the_deleting_destructor_runs_the_destructor_then_frees() {
        let mut e = Engine::new();
        noop(&mut e, &[FREE]);
        let p = e.mem.alloc(0x1c);
        for (flags, frees) in [(2u32, 0usize), (3, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009c_a710, &args![p, flags]).u32(), p);
            assert_eq!(calls(&e, 0x009c_a6c0), vec![vec![p]]);
            assert_eq!(e.mem.u32(p), PROCEDURE_VTABLE);
            assert_eq!(calls(&e, FREE).len(), frees);
        }
    }

    #[test]
    fn the_constructor_targets_its_reference() {
        let mut e = Engine::new();
        noop(&mut e, &[SET_TARGETED]);
        let p = e.mem.alloc(0x1c);
        e.mem.write(p, &[0xcd; 0x1c]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009c_a5f0, &args![p, 0x7000u32, 0x7100u32]).u32(),
            p
        );
        assert_eq!(e.mem.u32(p), ACTIVATE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(p + 4), 0);
        assert_eq!((e.mem.u32(p + 0x10), e.mem.u32(p + 0x14)), (0x7000, 0x7100));
        assert_eq!(e.mem.u8(p + 0x18), 0);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x7000, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x009c_a5f0, &args![p, 0u32, 0x7100u32]);
        assert!(calls(&e, SET_TARGETED).is_empty());
    }

    #[test]
    fn failing_stores_the_text_and_state_one() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x1c);
        e.call(0x009c_aac0, &args![p, 0x1234u32]);
        assert_eq!((e.mem.u32(p + 8), e.mem.u32(p + 0xc)), (1, 0x1234));
    }

    #[test]
    fn completing_stores_state_two() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x1c);
        e.call(0x009c_aae0, &args![p]);
        assert_eq!(e.mem.u32(p + 8), 2);
    }

    #[test]
    fn clearing_the_targeted_reference_fails_the_procedure() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x1c);
        e.mem.set_u32(p + 0x10, 0x7000);
        e.call(0x009c_ab00, &args![p, 0x7100u32]);
        assert_eq!((e.mem.u32(p + 0x10), e.mem.u32(p + 8)), (0x7000, 0));
        e.call(0x009c_ab00, &args![p, 0x7000u32]);
        assert_eq!(e.mem.u32(p + 0x10), 0);
        assert_eq!(
            (e.mem.u32(p + 8), e.mem.u32(p + 0xc)),
            (1, CLEARED_REFERENCE_TEXT)
        );
    }

    /// The `Update` fixture: the procedure (controller +4 = 0x6100), its attacker (an object
    /// whose slot `+0x1e4` answers the animation, word +4; word +8 its process), the animation
    /// (words: done +0, working +4, still +8, sequence +0xc, controller +0x10, time +0x14), the
    /// process (busy word +0), the reference (its form at +0).
    struct Scene {
        procedure: u32,
        attacker: u32,
        animation: u32,
        process: u32,
        reference: u32,
    }

    fn scene(e: &mut Engine, state: u8) -> Scene {
        let animation = e.mem.alloc(0x18);
        let process = e.mem.alloc(4);
        let attacker = object(e, &[animation, process]);
        let reference = e.mem.alloc(4);
        e.mem.set_u32(reference, 0x5000);
        let procedure = e.mem.alloc(0x1c);
        e.mem.set_u32(procedure + 4, 0x6100);
        e.mem.set_u32(procedure + 0x10, reference);
        e.mem.set_u32(procedure + 0x14, 0x5000);
        e.mem.set_u8(procedure + 0x18, state);
        e.register_double(GET_ATTACKER, move |_, _| attacker.into_ret());
        e.register(slot(0x1e4), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x008d_8520, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0045_cd60, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0049_85f0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0049_8d30, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(0x0049_8f80, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x008d_a3d0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(0x0092_1e30, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0045_3700, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x14)),
            ..Ret::default()
        });
        e.register(0x004e_ec60, |_, a| Ret {
            st0: 2.0 * f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.register(REFERENCE_FORM, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CONTROLLER_STATE_OF, |_, a| (a[0] + 1).into_ret());
        noop(e, &[0x0099_a780, 0x0057_3170, 0x0099_93c0, 0x0040_3d30]);
        e.register(0x0049_3bb0, |_, _| 1u32.into_ret());
        Scene {
            procedure,
            attacker,
            animation,
            process,
            reference,
        }
    }

    const CONTROLLER_STATE_OF: u32 = 0x0058_6150;

    fn state(e: &Engine, s: &Scene) -> u8 {
        e.mem.u8(s.procedure + 0x18)
    }

    fn progress(e: &Engine, s: &Scene) -> u32 {
        e.mem.u32(s.procedure + 8)
    }

    #[test]
    fn update_without_a_reference_fails() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0);
        e.mem.set_u32(s.procedure + 0x10, 0);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(progress(&e, &s), 1);
        assert_eq!(e.mem.u32(s.procedure + 0xc), NULL_REFERENCE_TEXT);
    }

    #[test]
    fn update_activates_the_reference_itself_when_busy() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0);
        e.mem.set_u32(s.process, 1);
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(calls(&e, 0x0099_a780), vec![vec![0x6101, s.reference]]);
        assert_eq!(
            calls(&e, 0x0057_3170),
            vec![vec![s.reference, s.attacker, 1, 0, 1]]
        );
        assert_eq!(calls(&e, 0x0099_93c0), vec![vec![0x6101]]);
        assert_eq!(progress(&e, &s), 2);
        // Without `00493bb0` the procedure stays.
        let s = scene(&mut e, 0);
        e.mem.set_u32(s.process, 1);
        e.register(0x0049_3bb0, |_, _| Ret::default());
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(progress(&e, &s), 0);
    }

    #[test]
    fn update_starts_a_special_idle_once_the_animation_is_free() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0);
        e.register(0x008f_f0b0, |_, _| 1u32.into_ret());
        // Not done playing: nothing yet.
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert!(calls(&e, 0x008f_f0b0).is_empty());
        e.mem.set_u32(s.animation, 1);
        // Still working: nothing.
        e.mem.set_u32(s.animation + 4, 1);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(state(&e, &s), 0);
        e.mem.set_u32(s.animation + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(
            calls(&e, 0x008f_f0b0),
            vec![vec![s.process, s.attacker, 0x5000, s.reference]]
        );
        assert_eq!(state(&e, &s), 1);
        assert!(calls(&e, 0x0057_3170).is_empty());
        // No idle to play: -1 and the items of the reference are taken now.
        let s = scene(&mut e, 0);
        e.mem.set_u32(s.animation, 1);
        e.register(0x008f_f0b0, |_, _| Ret::default());
        e.mem.set_u32(s.procedure + 0x14, 0x5100);
        let reference = s.reference;
        let entries: Vec<u32> = [(reference, 0x10u32), (0x9999, 0x11), (reference, 0x12)]
            .iter()
            .map(|(r, item)| {
                let entry = e.mem.alloc(8);
                e.mem.set_u32(entry, *r);
                e.mem.set_u32(entry + 4, *item);
                entry
            })
            .collect();
        e.register_double(0x0097_cfc0, move |_, a| {
            entries.get(a[1] as usize).copied().unwrap_or(0).into_ret()
        });
        e.register(0x0057_5610, |_, a| match a[1] {
            0x10 => (-3i32 as u32).into_ret(),
            _ => Ret::default(),
        });
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(state(&e, &s), 0xff);
        assert_eq!(calls(&e, 0x0097_cfc0).len(), 5);
        assert_eq!(
            calls(&e, 0x0057_5610),
            vec![vec![reference, 0x10], vec![reference, 0x12]]
        );
        assert_eq!(
            calls(&e, 0x0057_3170),
            vec![vec![reference, s.attacker, 1, 0x10, 3]]
        );
        assert_eq!(calls(&e, 0x0040_3d30)[0][1..], [0, 0x14]);
        assert_eq!(progress(&e, &s), 2);
    }

    #[test]
    fn update_grabs_once_the_animation_passes_its_named_time() {
        let mut e = Engine::new();
        let s = scene(&mut e, 1);
        // The sequence: its name through slot +0x14 of +0x18.
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Use_0.5");
        let sequence = e.mem.alloc(0x20);
        e.mem.set_u32(sequence + 0x18, VTABLE);
        e.mem.set_u32(sequence + 0x1c, name);
        e.register(slot(0x14), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x00ec_6e30, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|c| *c as u32 == a[1]);
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(0x00ec_a573, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            Ret {
                st0: text.parse::<f64>().unwrap(),
                ..Ret::default()
            }
        });
        e.mem.set_u32(s.animation + 0xc, sequence);
        e.mem.set_u32(s.animation + 0x10, 0x7700);
        e.mem.set_f32(s.animation + 0x14, 0.2);
        // 2 * 0.2 is not past 0.5: still playing.
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(state(&e, &s), 1);
        // A busy animation that still works waits too.
        e.mem.set_u32(s.animation, 1);
        e.mem.set_u32(s.animation + 8, 1);
        e.mem.set_f32(s.animation + 0x14, 0.3);
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(state(&e, &s), 3);
        assert_eq!(calls(&e, 0x004e_ec60), vec![vec![0x7700, 0.3f32.to_bits()]]);
        assert_eq!(calls(&e, 0x0057_3170).len(), 1);
        assert_eq!(progress(&e, &s), 0);
        // In state 3 with the animation done: complete without activating again.
        e.mem.set_u32(s.animation + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert!(calls(&e, 0x0057_3170).is_empty());
        assert_eq!(progress(&e, &s), 2);
        // Without a controller the grab comes at once; without an underscore the time is 0.
        let s = scene(&mut e, 1);
        e.mem.set_u32(s.animation + 0xc, sequence);
        e.mem.set_cstr(name, b"Use");
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(state(&e, &s), 3);
    }

    #[test]
    fn update_in_state_three_waits_for_the_animation() {
        let mut e = Engine::new();
        let s = scene(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x009c_a760, &args![s.procedure]);
        assert!(calls(&e, 0x008d_a3d0).is_empty());
        assert_eq!((state(&e, &s), progress(&e, &s)), (3, 0));
        // Any other state activates right away and completes.
        let s = scene(&mut e, 0xff);
        e.call(0x009c_a760, &args![s.procedure]);
        assert_eq!(progress(&e, &s), 2);
    }

    #[test]
    fn debug_text_names_the_state_and_the_target() {
        let mut e = Engine::new();
        let printed = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let p = printed.clone();
        e.register_double(slot(0x14), move |_, a| {
            p.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let position = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        let reference = object(&mut e, &[position]);
        e.register(slot(0x1f4), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(slot(0x130), |_, _| 0x4e4du32.into_ret());
        e.register(0x0047_4cb0, |_, _| 1u32.into_ret());
        e.register(0x0084_e3a0, |_, _| 0x99u32.into_ret());
        let printer = object(&mut e, &[]);
        let p = e.mem.alloc(0x1c);
        for (state, text) in [
            (0xffu8, Some(NO_ANIM_TEXT)),
            (0, None),
            (1, Some(ANIM_PLAYING_TEXT)),
            (2, Some(ANIM_GRABBING_TEXT)),
            (3, Some(ANIM_FINISHING_TEXT)),
            (4, None),
        ] {
            printed.borrow_mut().clear();
            e.mem.set_u8(p + 0x18, state);
            e.call(0x009c_ab30, &args![p, printer, 7u32, 8u32]);
            let lines = printed.borrow().clone();
            assert_eq!(lines.len(), usize::from(text.is_some()));
            if let Some(text) = text {
                assert_eq!(lines[0], vec![printer, 7, 8, text]);
            }
        }
        e.mem.set_u32(p + 0x10, reference);
        printed.borrow_mut().clear();
        e.call(0x009c_ab30, &args![p, printer, 7u32, 8u32]);
        let words = |v: f64| [v.to_bits() as u32, (v.to_bits() >> 32) as u32];
        let mut expected = vec![printer, 7, 8, TARGET_FORMAT, 0x4e4d, 0x99];
        for v in [1.0, 2.0, 3.0] {
            expected.extend(words(v));
        }
        assert_eq!(printed.borrow().clone(), vec![expected]);
    }

    #[test]
    fn save_game_writes_the_ids_and_the_state() {
        let mut e = Engine::new();
        noop(&mut e, &[PROCEDURE_SAVE_GAME, 0x0086_5df0, 0x0086_5e50]);
        let p = e.mem.alloc(0x1c);
        e.mem.set_u32(p + 0x10, 0x7000);
        e.mem.set_u32(p + 0x14, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x009c_acb0, &args![p, 0x9000u32]);
        assert_eq!(calls(&e, PROCEDURE_SAVE_GAME), vec![vec![p, 0x9000]]);
        assert_eq!(
            calls(&e, 0x0086_5df0),
            vec![vec![0x9000, 0x7000, 0], vec![0x9000, 0x7100, 0]]
        );
        assert_eq!(calls(&e, 0x0086_5e50), vec![vec![0x9000, p + 0x18, 1, 0]]);
    }

    #[test]
    fn load_game_reads_the_ids_and_resolves_the_form() {
        let mut e = Engine::new();
        noop(&mut e, &[PROCEDURE_LOAD_GAME, 0x0086_48e0, 0x0086_4980]);
        e.register(0x0086_48a0, |_, _| 0x55u32.into_ret());
        e.register(0x0048_39c0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00ec_43fb, |_, a| (a[0] + a[3]).into_ret());
        let p = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x009c_ad00, &args![p, 0x9000u32]);
        assert_eq!(calls(&e, PROCEDURE_LOAD_GAME), vec![vec![p, 0x9000]]);
        assert_eq!(calls(&e, 0x0086_48e0), vec![vec![0x9000, p + 0x10]]);
        assert_eq!(calls(&e, 0x0086_48a0), vec![vec![0x9000]]);
        assert_eq!(
            calls(&e, 0x00ec_43fb),
            vec![vec![0x56, 0, 0x0118_3028, 0x0118_3108, 0]]
        );
        assert_eq!(e.mem.u32(p + 0x14), 0x56 + 0x0118_3108);
        assert_eq!(calls(&e, 0x0086_4980), vec![vec![0x9000, p + 0x18, 1]]);
    }

    #[test]
    fn init_load_game_resolves_and_targets_the_reference() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0045_34f0, SET_TARGETED]);
        e.register(0x0048_39c0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00ec_43fb, |_, a| {
            assert_eq!(a[1..], [0, 0x0118_3028, 0x0118_41cc, 0]);
            (a[0] * 2).into_ret()
        });
        let p = e.mem.alloc(0x1c);
        e.mem.set_u32(p + 0x10, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x009c_ad70, &args![p, 0x9000u32]);
        assert_eq!(calls(&e, 0x0045_34f0), vec![vec![p, 0x9000]]);
        assert_eq!(e.mem.u32(p + 0x10), 0x202);
        assert_eq!(calls(&e, SET_TARGETED), vec![vec![0x202, 1]]);
        e.mem.set_u32(p + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x009c_ad70, &args![p, 0x9000u32]);
        assert!(calls(&e, 0x0048_39c0).is_empty());
        assert!(calls(&e, SET_TARGETED).is_empty());
    }
}
