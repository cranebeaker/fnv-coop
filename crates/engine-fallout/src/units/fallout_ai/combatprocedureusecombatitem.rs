//! `fallout/ai/combat/combatprocedureusecombatitem.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009daa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0109214c` (RTTI `.?AVCombatProcedureUseCombatItem@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009daa90(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_214c);
    e.call(0x009c_a6c0, &args![this]);
}

// ---- Batch n025 ---------------------------------------------------------------------------

/// `CombatProcedureUseCombatItem` (Xbox PDB), 0x18 bytes: the `CombatProcedure` base
/// (vtable, `pCombatController` +0x4, `eStatus` +0x8, `pFailureReason` +0xc), then `pItem`
/// +0x10 and `bAnimating` +0x14; the PC offsets are the PDB's (constructor `009daa10`).
const USE_ITEM: u32 = 0x10;
const USE_ANIMATING: u32 = 0x14;
/// The `CombatProcedure` base constructor (`009ca680`, `thiscall`).
const PROCEDURE_CONSTRUCT: u32 = 0x009c_a680;
/// Ends the procedure (`009caae0`, `thiscall`).
const PROCEDURE_DONE: u32 = 0x009c_aae0;
/// `CombatProcedure::GetAttacker` (Xbox PDB, `00996de0`).
const GET_ATTACKER: u32 = 0x0099_6de0;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map name, `008d8520`) on the actor.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `CombatProcedure::SaveGame` / `LoadGame` (Xbox PDB, `00996ed0` / `00996ef0`).
const PROCEDURE_SAVE_GAME: u32 = 0x0099_6ed0;
const PROCEDURE_LOAD_GAME: u32 = 0x0099_6ef0;
/// `TESForm`'s and `TESBoundObject`'s RTTI type descriptors.
const TES_FORM_TYPE: u32 = 0x0118_3028;
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
/// `"ANIMATING"` (`01020fc8`).
const ANIMATING_TEXT: u32 = 0x0102_0fc8;
/// `"Item: '%s' (%08X)"` (`010921dc`).
const ITEM_FORMAT: u32 = 0x0109_21dc;
/// `"COMBAT: Unhandled form type %s in CombatProcedureUseCombatItem::Update()"`
/// (`01092190`).
const UNHANDLED_FORM_TYPE_FORMAT: u32 = 0x0109_2190;

// Translated from 009da970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the
/// procedure's own save (`00996ed0`), the item's form id (`00865df0`) and the animating byte
/// (`00865e50`).
pub fn combat_procedure_use_combat_item_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(PROCEDURE_SAVE_GAME, &args![this, buffer]);
    let item = e.mem.u32(me + USE_ITEM);
    e.call(0x0086_5df0, &args![buffer, item, 0u32]);
    e.call(0x0086_5e50, &args![buffer, me + USE_ANIMATING, 1u32, 0u32]);
}

// Translated from 009daa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::CombatProcedureUseCombatItem` (Xbox PDB; `thiscall`,
/// `RET 4`): the base constructor (`009ca680`), the vtable `0109214c`, `pItem` = `item` and
/// not animating. Returns `this`.
pub fn combat_procedure_use_combat_item_construct(e: &mut Engine, this: Ptr, item: u32) -> Ptr {
    e.call(PROCEDURE_CONSTRUCT, &args![this]);
    let me = this.addr();
    e.mem.set_u32(me, 0x0109_214c);
    e.mem.set_u32(me + USE_ITEM, item);
    e.mem.set_u8(me + USE_ANIMATING, 0);
    this
}

// Translated from 009daa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::GetType` (Xbox PDB; `thiscall`, `this` unread): 11.
pub fn combat_procedure_use_combat_item_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xb
}

// Translated from 009daa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatProcedureUseCombatItem` scalar deleting destructor (`thiscall`, `RET 4`): the
/// destructor `009daa90`, then `operator delete` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009daa60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009daa90(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 009daab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::Update` (Xbox PDB; `thiscall`). Unless the process
/// object's `0045cd60` is set, the item is used only after an animation: while the
/// attacker's current animation (slot `+0x1e4`) has not finished its special idle
/// (`004985f0`) or still works it (`00498d30` with 0) nothing happens; before animating, a
/// special idle for the item is started (`HighProcess::FindSpecialIdletoPlay` `008ff0b0`
/// with the attacker, the item and 0) and the procedure is marked animating. Using the item:
/// the combat state (`00586150` of the controller) is told (`0099a840`); an item of form
/// type 0x2f (`00401170`) is drunk (`Actor::DrinkPotion` `008c1f80` with 0, 1) and the
/// state updated (`0097f450` with 1); any other type is reported (`005b5e40` with its
/// `00440e30` name). Then the procedure ends (`009caae0`).
pub fn combat_procedure_use_combat_item_update(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let attacker = e.call(GET_ATTACKER, &args![this]).u32();
    let controller = e.mem.u32(me + 4);
    let state = e.call(0x0058_6150, &args![controller]).u32();
    let process = e.call(ACTOR_PROCESS, &args![attacker]).u32();
    if e.call(0x0045_cd60, &args![process]).u32() == 0 {
        let animation = e.vcall(attacker, 0x1e4, &args![]).u32();
        if animation != 0 {
            if !e.call(0x0049_85f0, &args![animation]).bool() {
                return;
            }
            if e.call(0x0049_8d30, &args![animation, 0u32]).bool() {
                return;
            }
        }
        if e.mem.u8(me + USE_ANIMATING) == 0 {
            let process = e.call(ACTOR_PROCESS, &args![attacker]).u32();
            let item = e.mem.u32(me + USE_ITEM);
            e.call(0x008f_f0b0, &args![process, attacker, item, 0u32]);
            e.mem.set_u8(me + USE_ANIMATING, 1);
            return;
        }
    }
    let item = e.mem.u32(me + USE_ITEM);
    e.call(0x0099_a840, &args![state, item]);
    let item = e.mem.u32(me + USE_ITEM);
    if e.call(0x0040_1170, &args![item]).u32() == 0x2f {
        let item = e.mem.u32(me + USE_ITEM);
        e.call(0x008c_1f80, &args![attacker, item, 0u32, 1u32]);
        e.call(0x0097_f450, &args![state, 1u32]);
    } else {
        let item = e.mem.u32(me + USE_ITEM);
        let name = e.call(0x0044_0e30, &args![item]).u32();
        e.call(0x005b_5e40, &args![UNHANDLED_FORM_TYPE_FORMAT, name]);
    }
    e.call(PROCEDURE_DONE, &args![this]);
}

// Translated from 009dabd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::DisplayDebugText` (Xbox PDB; `thiscall`, `RET 0xc`):
/// prints through `printer`'s slot `+0x14` (`cdecl`, with `first` and `second`):
/// `"ANIMATING"` while animating, and for an item `"Item: '%s' (%08X)"` with its name
/// (its slot `+0x130` when `00474cb0` holds, else `TESFullName::GetFullName` `00482720`)
/// and form id (`0084e3a0`).
pub fn combat_procedure_use_combat_item_display_debug_text(
    e: &mut Engine,
    this: Ptr,
    printer: Ptr,
    first: u32,
    second: u32,
) {
    let me = this.addr();
    let p = printer.addr();
    if e.mem.u8(me + USE_ANIMATING) != 0 {
        e.vcall(p, 0x14, &args![first, second, ANIMATING_TEXT]);
    }
    let item = e.mem.u32(me + USE_ITEM);
    if item == 0 {
        return;
    }
    let name = if e.call(0x0047_4cb0, &args![item]).u32() != 0 {
        let item = e.mem.u32(me + USE_ITEM);
        e.vcall(item, 0x130, &args![]).u32()
    } else {
        let item = e.mem.u32(me + USE_ITEM);
        e.call(0x0048_2720, &args![item]).u32()
    };
    let item = e.mem.u32(me + USE_ITEM);
    let id = e.call(0x0084_e3a0, &args![item]).u32();
    e.vcall(p, 0x14, &args![first, second, ITEM_FORMAT, name, id]);
}

// Translated from 009dac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the
/// procedure's own load (`00996ef0`), the item (`008648a0`'s form id through `004839c0` and
/// a cast to `TESBoundObject`, `00ec43fb`) and the animating byte (`00864980`).
pub fn combat_procedure_use_combat_item_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    let me = this.addr();
    e.call(PROCEDURE_LOAD_GAME, &args![this, buffer]);
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let item = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, TES_FORM_TYPE, TES_BOUND_OBJECT_TYPE, 0u32],
        )
        .u32();
    e.mem.set_u32(me + USE_ITEM, item);
    e.call(0x0086_4980, &args![buffer, me + USE_ANIMATING, 1u32]);
}

// Translated from 009dace0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatProcedureUseCombatItem::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): only the
/// base's `004534f0(this, buffer)`.
pub fn combat_procedure_use_combat_item_init_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0045_34f0, &args![this, buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009d_a970,
            combat_procedure_use_combat_item_save_game(Ptr, u32)
        ),
        entry!(
            0x009d_aa10,
            combat_procedure_use_combat_item_construct(Ptr, u32) -> Ptr
        ),
        entry!(0x009d_aa50, combat_procedure_use_combat_item_get_type(Ptr) -> u32),
        entry!(0x009d_aa60, fn_009daa60(Ptr, u32) -> Ptr),
        entry!(0x009d_aab0, combat_procedure_use_combat_item_update(Ptr)),
        entry!(
            0x009d_abd0,
            combat_procedure_use_combat_item_display_debug_text(Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x009d_ac80,
            combat_procedure_use_combat_item_load_game(Ptr, u32)
        ),
        entry!(
            0x009d_ace0,
            combat_procedure_use_combat_item_init_load_game(Ptr, u32)
        ),
        entry!(0x009d_aa90, fn_009daa90(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009daa90_stores_the_vtable_then_forwards_this() {
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
        e.call(0x009d_aa90, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_214c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_214c,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n025_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the batch's translations call, and that of the
    /// destructor `009daa90` (`009ca6c0`).
    const EXTERNAL: &[u32] = &[
        0x0040_1030,
        0x0040_1170,
        0x0044_0e30,
        0x0045_34f0,
        0x0045_cd60,
        0x0047_4cb0,
        0x0048_2720,
        0x0048_39c0,
        0x0049_85f0,
        0x0049_8d30,
        0x0058_6150,
        0x005b_5e40,
        0x0084_e3a0,
        0x0086_48a0,
        0x0086_4980,
        0x0086_5df0,
        0x0086_5e50,
        0x008c_1f80,
        0x008d_8520,
        0x008f_f0b0,
        0x0097_f450,
        0x0099_6de0,
        0x0099_6ed0,
        0x0099_6ef0,
        0x0099_a840,
        0x009c_a680,
        0x009c_aae0,
        0x00ec_43fb,
        0x009c_a6c0,
    ];

    const VTABLE: u32 = 0x0200_0000;
    const SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let slots: Vec<u32> = (0..0x200).map(|s| SLOTS + 4 * s).collect();
        for &slot in &slots {
            stub(&mut e, slot, 0);
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    #[test]
    fn construct_type_and_delete() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcd; 0x18]);
        assert_eq!(e.call(0x009d_aa10, &args![this, 0x66u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0109_214c);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u8(this + 0x14)), (0x66, 0));
        assert_eq!(e.call(0x009d_aa50, &args![this]).u32(), 0xb);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009d_aa60, &args![this, 1u32]).u32(), this);
            e.call(0x009d_aa60, &args![this, 0u32]);
        });
        assert_eq!(calls(&log, 0x009c_a6c0).len(), 2);
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![this]]);
    }

    #[test]
    fn save_load_and_init_load() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x10, 0x66);
        stub(&mut e, 0x0086_48a0, 0x200);
        e.register(0x0048_39c0, |_, a| ret(a[0] + 1));
        e.register(0x00ec_43fb, |_, a| ret(a[0] + a[3]));
        let log = logged(&mut e, |e| {
            e.call(0x009d_a970, &args![this, 0x8000u32]);
            e.call(0x009d_ac80, &args![this, 0x8000u32]);
            e.call(0x009d_ace0, &args![this, 0x8000u32]);
        });
        assert_eq!(calls(&log, PROCEDURE_SAVE_GAME), vec![vec![this, 0x8000]]);
        assert_eq!(calls(&log, 0x0086_5df0), vec![vec![0x8000, 0x66, 0]]);
        assert_eq!(
            calls(&log, 0x0086_5e50),
            vec![vec![0x8000, this + 0x14, 1, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x201 + TES_BOUND_OBJECT_TYPE);
        assert_eq!(calls(&log, 0x0086_4980), vec![vec![0x8000, this + 0x14, 1]]);
        assert_eq!(calls(&log, 0x0045_34f0), vec![vec![this, 0x8000]]);
    }

    #[test]
    fn update_animates_then_uses_the_item() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 4, 0x700);
        e.mem.set_u32(this + 0x10, 0x66);
        let attacker = e.mem.alloc(0x10);
        e.mem.set_u32(attacker, VTABLE);
        stub(&mut e, GET_ATTACKER, attacker);
        stub(&mut e, 0x0058_6150, 0x900);
        stub(&mut e, ACTOR_PROCESS, 0xa00);
        // An animation still working: nothing.
        stub(&mut e, SLOTS + 0x1e4, 0xb00);
        let log = logged(&mut e, |e| {
            e.call(0x009d_aab0, &args![this]);
        });
        assert_eq!(calls(&log, 0x0049_85f0), vec![vec![0xb00]]);
        assert!(calls(&log, PROCEDURE_DONE).is_empty());
        // Finished: start the special idle and mark animating.
        stub(&mut e, 0x0049_85f0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_aab0, &args![this]);
        });
        assert_eq!(calls(&log, 0x0049_8d30), vec![vec![0xb00, 0]]);
        assert_eq!(
            calls(&log, 0x008f_f0b0),
            vec![vec![0xa00, attacker, 0x66, 0]]
        );
        assert_eq!(e.mem.u8(this + 0x14), 1);
        // Animating: use it; a potion (form type 0x2f) is drunk.
        stub(&mut e, 0x0040_1170, 0x2f);
        let log = logged(&mut e, |e| {
            e.call(0x009d_aab0, &args![this]);
        });
        assert_eq!(calls(&log, 0x0099_a840), vec![vec![0x900, 0x66]]);
        assert_eq!(calls(&log, 0x008c_1f80), vec![vec![attacker, 0x66, 0, 1]]);
        assert_eq!(calls(&log, 0x0097_f450), vec![vec![0x900, 1]]);
        assert_eq!(calls(&log, PROCEDURE_DONE), vec![vec![this]]);
        // Anything else is reported.
        stub(&mut e, 0x0040_1170, 0x28);
        stub(&mut e, 0x0044_0e30, 0x1234);
        let log = logged(&mut e, |e| {
            e.call(0x009d_aab0, &args![this]);
        });
        assert_eq!(
            calls(&log, 0x005b_5e40),
            vec![vec![UNHANDLED_FORM_TYPE_FORMAT, 0x1234]]
        );
        assert!(calls(&log, 0x008c_1f80).is_empty());
        // `0045cd60` set: used at once even when not animating.
        e.mem.set_u8(this + 0x14, 0);
        stub(&mut e, 0x0045_cd60, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009d_aab0, &args![this]);
        });
        assert!(calls(&log, SLOTS + 0x1e4).is_empty());
        assert_eq!(calls(&log, PROCEDURE_DONE).len(), 1);
    }

    #[test]
    fn display_debug_text_prints_the_item() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let printer = e.mem.alloc(8);
        e.mem.set_u32(printer, VTABLE);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, VTABLE);
        e.mem.set_u32(this + 0x10, item);
        e.mem.set_u8(this + 0x14, 1);
        stub(&mut e, 0x0048_2720, 0x3000);
        stub(&mut e, 0x0084_e3a0, 0x0001_2345);
        let log = logged(&mut e, |e| {
            e.call(0x009d_abd0, &args![this, printer, 1u32, 2u32]);
        });
        assert_eq!(
            calls(&log, SLOTS + 0x14),
            vec![
                vec![printer, 1, 2, ANIMATING_TEXT],
                vec![printer, 1, 2, ITEM_FORMAT, 0x3000, 0x0001_2345]
            ]
        );
        // `00474cb0` set: the item's own name (slot 0x130).
        stub(&mut e, 0x0047_4cb0, 1);
        stub(&mut e, SLOTS + 0x130, 0x4000);
        e.mem.set_u8(this + 0x14, 0);
        let log = logged(&mut e, |e| {
            e.call(0x009d_abd0, &args![this, printer, 1u32, 2u32]);
        });
        assert_eq!(calls(&log, SLOTS + 0x14)[0][4], 0x4000);
        assert!(calls(&log, 0x0048_2720).is_empty());
    }
}
