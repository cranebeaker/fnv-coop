//! `fallout shared/tempeffects/bstempeffect.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSTempEffect` (Xbox PDB), 0x18 bytes on both builds (the scalar deleting
    /// destructor frees 0x18): an `NiObject` and the effect's timing.
    pub struct BSTempEffect: 0x18 {
        /// `fLifetime` (Xbox PDB).
        0x08 fLifetime: u32,
        /// `pCell` (Xbox PDB): `TESObjectCELL*`.
        0x0C pCell: u32,
        /// `fAge` (Xbox PDB).
        0x10 fAge: u32,
        /// `bInitialized` (Xbox PDB).
        0x14 bInitialized: u8,
    }
}

/// `BSTempEffect`'s vtable (`01069754`), stored by the destructor.
const BS_TEMP_EFFECT_VTABLE: u32 = 0x0106_9754;
/// `NiObject`'s destructor body (`00a5d3d0`, `thiscall`).
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;

// Translated from 00682860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTempEffect` destructor body (unnamed in the engine map; the scalar deleting
/// destructor `00682830` and the derived destructors call it; `thiscall`): stores the
/// class's vtable, clears `pCell`, `fLifetime`, `fAge` (0.0) and `bInitialized`, then
/// runs `NiObject`'s destructor (`00a5d3d0`).
pub fn bs_temp_effect_destructor(e: &mut Engine, this: Ptr<BSTempEffect>) {
    e.mem.set_u32(this.addr(), BS_TEMP_EFFECT_VTABLE);
    e.set(this, BSTempEffect::pCell, 0);
    e.set(this, BSTempEffect::fLifetime, 0);
    e.set(this, BSTempEffect::fAge, 0);
    e.set(this, BSTempEffect::bInitialized, 0);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 006828a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed time): adds the
/// time to `fAge` and answers whether the effect is still alive: true unless
/// `fLifetime < fAge` (`FCOMPP`, `TEST AH,5`, `JP`: equal ages and NaN answer true).
pub fn bs_temp_effect_update(e: &mut Engine, this: Ptr<BSTempEffect>, seconds: u32) -> bool {
    let age = x87_load32(e.get(this, BSTempEffect::fAge));
    let age = x87_store32(x87_add(age, x87_m32(seconds)));
    e.set(this, BSTempEffect::fAge, age);
    let age = x87_load32(e.get(this, BSTempEffect::fAge));
    let lifetime = x87_load32(e.get(this, BSTempEffect::fLifetime));
    !(lifetime < age)
}

// Translated from 00682830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00682860` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0068_2860, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00682800 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6a44 without reading the receiver. The map names this body `BSTempEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682800(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6a44
}

/// The `TESSaveLoadGame` singleton pointer (its `SaveNumericID` and
/// `LoadNumericID` run on it).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008579b0` (unnamed in the engine map; `thiscall`, `RET 8`: data, size):
/// writes bytes to the save.
const SAVE_DATA_008579B0: u32 = 0x0085_79b0;
/// `008579e0` (unnamed in the engine map; `thiscall`, `RET 8`: data, size):
/// reads bytes from the save.
const LOAD_DATA_008579E0: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::SaveNumericID` (`00857a10`, engine map; `RET 8`).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `TESSaveLoadGame::LoadNumericID` (`00857aa0`, engine map; `RET 8`).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `0084e3a0` (unnamed in the engine map; no stack word): the word at +0xC
/// of a form (`TESForm`'s form id on PC).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `004839c0` (unnamed in the engine map; cdecl, the id): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The cast's source type descriptor (`.?AVTESForm@@`).
const CAST_SOURCE_TESFORM: u32 = 0x0118_3028;
/// The cast's target type descriptor (`.?AVTESObjectCELL@@`).
const CAST_TARGET_TESOBJECTCELL: u32 = 0x0118_3fb4;
/// `TESObjectCELL::Get3D` (`00545cb0`, engine map; no stack word).
const CELL_GET_3D: u32 = 0x0054_5cb0;
/// `00865e50` (unnamed in the engine map; `thiscall`, `RET 0xC`: data,
/// size, a word): writes bytes to the stream.
const STREAM_WRITE_00865E50: u32 = 0x0086_5e50;
/// `00864980` (unnamed in the engine map; `thiscall`, `RET 8`: data, size):
/// reads bytes from the stream.
const STREAM_READ_00864980: u32 = 0x0086_4980;

// Translated from 00682810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::Initialize` (engine map): sets `bInitialized`.
pub fn bs_temp_effect_initialize(e: &mut Engine, this: Ptr<BSTempEffect>) {
    e.set(this, BSTempEffect::bInitialized, 1);
}

// Translated from 006828e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no stack word): the size of the saved data,
/// 0 + 4 + 4 + 4 = 12, summed in a 16-bit local (`AX`).
pub fn fn_006828e0(_e: &mut Engine, _this: Ptr) -> u16 {
    let mut size: u16 = 0;
    size += 4;
    size += 4;
    size += 4;
    size
}

// Translated from 00682920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::SaveGame` (engine map): writes `fLifetime` and `fAge` (4
/// bytes each, `008579b0` on the save/load game object), then the form id
/// of `pCell` (0 without a cell) through `SaveNumericID`.
pub fn bs_temp_effect_save_game(e: &mut Engine, this: Ptr<BSTempEffect>) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_DATA_008579B0, &args![game, this.addr() + 8, 4u32]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_DATA_008579B0, &args![game, this.addr() + 0x10, 4u32]);
    e.with_stack(4, |e, id| {
        let cell = e.get(this, BSTempEffect::pCell);
        if cell != 0 {
            let cell_id = e.call(FORM_ID_0084E3A0, &args![cell]).u32();
            e.mem.set_u32(id.addr(), cell_id);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![game, id, 4u32]);
    });
}

// Translated from 00682990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::LoadGame` (engine map): reads `fLifetime` and `fAge`
/// (`008579e0`), then a form id (`LoadNumericID`) whose form, cast to
/// `TESObjectCELL`, becomes `pCell`. False when there is no such cell or
/// the cell has no 3D (`TESObjectCELL::Get3D`), else true.
pub fn bs_temp_effect_load_game(e: &mut Engine, this: Ptr<BSTempEffect>) -> bool {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_DATA_008579E0, &args![game, this.addr() + 8, 4u32]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_DATA_008579E0, &args![game, this.addr() + 0x10, 4u32]);
    let id = e.with_stack(4, |e, slot| {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![game, slot, 4u32]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
    let cell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                form,
                0u32,
                CAST_SOURCE_TESFORM,
                CAST_TARGET_TESOBJECTCELL,
                0u32
            ],
        )
        .u32();
    e.set(this, BSTempEffect::pCell, cell);
    let cell = e.get(this, BSTempEffect::pCell);
    cell != 0 && e.call(CELL_GET_3D, &args![cell]).u32() != 0
}

// Translated from 00682a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::SaveGame_ov2` (engine map; `RET 4`): writes `fLifetime`,
/// `fAge` (4 bytes each) and `bInitialized` (1 byte) to `stream`
/// (`00865e50`, each with 0).
pub fn bs_temp_effect_save_game_ov2(e: &mut Engine, this: Ptr<BSTempEffect>, stream: u32) {
    e.call(
        STREAM_WRITE_00865E50,
        &args![stream, this.addr() + 8, 4u32, 0u32],
    );
    e.call(
        STREAM_WRITE_00865E50,
        &args![stream, this.addr() + 0x10, 4u32, 0u32],
    );
    e.call(
        STREAM_WRITE_00865E50,
        &args![stream, this.addr() + 0x14, 1u32, 0u32],
    );
}

// Translated from 00682a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::LoadGame_ov2` (engine map; `RET 4`): reads `fLifetime`,
/// `fAge` (4 bytes each) and `bInitialized` (1 byte) from `stream`
/// (`00864980`).
pub fn bs_temp_effect_load_game_ov2(e: &mut Engine, this: Ptr<BSTempEffect>, stream: u32) {
    e.call(STREAM_READ_00864980, &args![stream, this.addr() + 8, 4u32]);
    e.call(
        STREAM_READ_00864980,
        &args![stream, this.addr() + 0x10, 4u32],
    );
    e.call(
        STREAM_READ_00864980,
        &args![stream, this.addr() + 0x14, 1u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_2810, bs_temp_effect_initialize(Ptr<BSTempEffect>)),
        entry!(0x0068_28e0, fn_006828e0(Ptr) -> u16),
        entry!(0x0068_2920, bs_temp_effect_save_game(Ptr<BSTempEffect>)),
        entry!(
            0x0068_2990,
            bs_temp_effect_load_game(Ptr<BSTempEffect>) -> bool
        ),
        entry!(
            0x0068_2a30,
            bs_temp_effect_save_game_ov2(Ptr<BSTempEffect>, u32)
        ),
        entry!(
            0x0068_2a80,
            bs_temp_effect_load_game_ov2(Ptr<BSTempEffect>, u32)
        ),
        entry!(0x0068_2800, fn_00682800(Ptr) -> u32),
        entry!(0x0068_2830, bs_temp_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_2860, bs_temp_effect_destructor(Ptr<BSTempEffect>)),
        entry!(
            0x0068_28a0,
            bs_temp_effect_update(Ptr<BSTempEffect>, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod temp_effect_tests {
    use super::*;

    #[test]
    fn destructor_clears_the_timing_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xee; 0x18]);
        e.call_log = Some(vec![]);
        e.call(0x0068_2860, &[this]);
        assert_eq!(e.mem.u32(this), BS_TEMP_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_eeee);
        for offset in [8u32, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x15), 0xee);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_OBJECT_DESTRUCTOR, vec![this]));
    }

    #[test]
    fn update_ages_and_answers_alive_until_past_the_lifetime() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_f32(this + 0x10, 0.25);
        assert!(e.call(0x0068_28a0, &[this, 0.5f32.to_bits()]).bool());
        assert_eq!(e.mem.f32(this + 0x10), 0.75);
        // Reaching the lifetime exactly is still alive.
        assert!(e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
        assert!(!e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
        assert_eq!(e.mem.f32(this + 0x10), 1.25);
        // A NaN lifetime answers alive.
        e.mem.set_f32(this + 8, f32::NAN);
        assert!(e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_temp_effect_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_2830, 0x0068_2860, 0x18);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682800_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2800, &args![0x1000u32]).u32(), 0x011d_6a44);
        assert_eq!(e.call(0x0068_2800, &args![0u32]).u32(), 0x011d_6a44);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn initialize_sets_the_flag_and_the_saved_size_is_twelve() {
        let mut e = Engine::new();
        let this: Ptr<BSTempEffect> = e.new_object();
        e.call(0x0068_2810, &args![this]);
        assert_eq!(e.get(this, BSTempEffect::bInitialized), 1);
        assert_eq!(e.call(0x0068_28e0, &args![this]).u16(), 12);
    }

    #[test]
    fn save_game_writes_times_and_the_cell_id() {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GAME & !0xfff, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5a5au32);
        e.register(SAVE_DATA_008579B0, |_, _| Ret::default());
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let ids = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2]));
            Ret::default()
        });
        let this: Ptr<BSTempEffect> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0068_2920, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_DATA_008579B0),
            vec![
                vec![0x5a5a, this.addr() + 8, 4],
                vec![0x5a5a, this.addr() + 0x10, 4]
            ]
        );
        let cell = e.mem.alloc(0x10);
        e.mem.set_u32(cell + 0xc, 0x1234);
        e.set(this, BSTempEffect::pCell, cell);
        e.call(0x0068_2920, &args![this]);
        assert_eq!(*ids.borrow(), vec![(0x5a5a, 0, 4), (0x5a5a, 0x1234, 4)]);
    }

    #[test]
    fn load_game_reads_times_and_resolves_the_cell() {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GAME & !0xfff, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5a5au32);
        e.register(LOAD_DATA_008579E0, |_, _| Ret::default());
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.register(LOOKUP_FORM_004839C0, |_, a| ret(a[0] + 0x1000));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(CELL_GET_3D, |_, _| ret(1));
        let this: Ptr<BSTempEffect> = e.new_object();
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_2990, &args![this]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, BSTempEffect::pCell), 0x1077);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![
                0x1077,
                0,
                CAST_SOURCE_TESFORM,
                CAST_TARGET_TESOBJECTCELL,
                0
            ]]
        );
        assert_eq!(calls_to(&log, LOAD_DATA_008579E0).len(), 2);
        e.register(CELL_GET_3D, |_, _| ret(0));
        assert!(!e.call(0x0068_2990, &args![this]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_2990, &args![this]).bool());
        assert!(calls_to(&e.call_log.take().unwrap(), CELL_GET_3D).is_empty());
    }

    #[test]
    fn stream_save_and_load_move_the_three_fields() {
        let mut e = Engine::new();
        e.register(STREAM_WRITE_00865E50, |_, _| Ret::default());
        e.register(STREAM_READ_00864980, |_, _| Ret::default());
        let this: Ptr<BSTempEffect> = e.new_object();
        let t = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x0068_2a30, &args![this, 0x900u32]);
        e.call(0x0068_2a80, &args![this, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, STREAM_WRITE_00865E50),
            vec![
                vec![0x900, t + 8, 4, 0],
                vec![0x900, t + 0x10, 4, 0],
                vec![0x900, t + 0x14, 1, 0]
            ]
        );
        assert_eq!(
            calls_to(&log, STREAM_READ_00864980),
            vec![
                vec![0x900, t + 8, 4],
                vec![0x900, t + 0x10, 4],
                vec![0x900, t + 0x14, 1]
            ]
        );
    }
}
