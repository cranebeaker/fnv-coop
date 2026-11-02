//! `fallout/magic/scripteffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ScriptEffect` (Xbox PDB, 0x50 bytes) is the active effect that runs a
//! script: the `ActiveEffect` base (0x48 bytes; `pSpell` +8, `pEffect` +0xC,
//! `pTarget` +0x24, `pCaster` +0x28) plus `pScript` (+0x48) and
//! `pEffectLocals` (+0x4C, the script's `ScriptLocals`). The base fields
//! are read at their offsets. The compiler's exception frames (`FS:[0]`)
//! are not translated; the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ScriptEffect`'s vtable (RTTI `.?AVScriptEffect@@`).
const SCRIPT_EFFECT_VTABLE: u32 = 0x0107_c164;
/// `pScript` (Xbox PDB).
const SCRIPT: u32 = 0x48;
/// `pEffectLocals` (Xbox PDB).
const EFFECT_LOCALS: u32 = 0x4c;

/// the allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// the allocator's free, cdecl (block).
const FREE: u32 = 0x0040_1030;
/// thiscall on the `EffectItem`: the script the effect runs.
const EFFECT_ITEM_SCRIPT: u32 = 0x0040_45e0;
/// The `ActiveEffect` constructor (thiscall, three words).
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// The `ActiveEffect` destructor (thiscall).
const ACTIVE_EFFECT_DESTROY: u32 = 0x0080_3ed0;
/// `ActiveEffect::CopyData` (engine map), thiscall (this, other).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
/// `ActiveEffect::SaveGame_ov2` (engine map), thiscall (this, buffer).
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_6230;
/// The `ScriptLocals` deleting destructor (`0041af70`, thiscall (locals, flags)).
const LOCALS_DELETING_DESTRUCTOR: u32 = 0x0041_af70;
/// `__RTDynamicCast` (engine map), cdecl (object, offset, source type, target type, is reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `ActiveEffect` and `ScriptEffect`.
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
const SCRIPT_EFFECT_TYPE: u32 = 0x011a_1490;
/// thiscall on a `Script`: makes a new `ScriptLocals` for it.
const SCRIPT_CREATE_LOCALS: u32 = 0x005a_bf60;
/// thiscall (locals, other locals): copies the other locals in.
const LOCALS_COPY: u32 = 0x005a_8f40;
/// thiscall on `ScriptLocals`: the first node of its variable list (+0xC).
const LOCALS_FIRST_VARIABLE: u32 = 0x0084_e3a0;
/// thiscall on a list node: whether it is empty (item and next both 0).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// thiscall on a list node: answers the node itself.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// thiscall on a list node: the next node (+4).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `ScriptLocals::SetVariable` (engine map), thiscall (locals, id, double).
const LOCALS_SET_VARIABLE: u32 = 0x005a_9290;
/// `ScriptLocals::SaveGame` (engine map), thiscall (locals, buffer).
const LOCALS_SAVE_GAME: u32 = 0x005a_9db0;
/// thiscall on an `ActiveEffect`: `pTarget` (+0x24) (`D3DTexture_LockRect` in the engine map; folded code).
const ACTIVE_EFFECT_TARGET: u32 = 0x0059_bb30;
/// `Script::RunScriptEffectStart` (engine map), thiscall (script, reference, locals).
const SCRIPT_RUN_EFFECT_START: u32 = 0x005a_c340;
/// `Script::RunScriptEffectUpdate` (engine map), thiscall (script, reference, locals, seconds).
const SCRIPT_RUN_EFFECT_UPDATE: u32 = 0x005a_c3c0;
/// thiscall (buffer, data, size, 0): writes bytes to the save buffer.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;

/// The `MagicTarget` vtable slot called for the reference the script runs
/// on (+4, `GetTargetStatsObject` in the Xbox PDB; no stack words).
const TARGET_SLOT_REFERENCE: u32 = 0x4;
/// The `ActiveEffect` vtable slot `Copy` calls on the copy's source (`CopyData`).
const SLOT_COPY_DATA: u32 = 0x44;

// Translated from 00827740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827740 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when the script lookup throws is not translated
/// `ScriptEffect::ScriptEffect` (unnamed in the engine map; the body is
/// the constructor), thiscall `(caster, spell, effect)`: constructs the
/// `ActiveEffect` base with the three words, sets the vtable, takes the
/// effect item's script as `pScript` and clears `pEffectLocals`. Answers
/// `this`.
pub fn fn_00827740(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![base, caster, spell, effect]);
    e.mem.set_u32(base, SCRIPT_EFFECT_VTABLE);
    let script = e.call(EFFECT_ITEM_SCRIPT, &args![effect]).u32();
    e.mem.set_u32(base + SCRIPT, script);
    e.mem.set_u32(base + EFFECT_LOCALS, 0);
    this
}

// Translated from 008277c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`008277f0`) and, when bit 0 of the flags is set, frees the
/// object. Answers `this`.
pub fn script_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008277f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 008277f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008277f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when the locals' destructor throws is not translated
/// `ScriptEffect::~ScriptEffect` (unnamed in the engine map; the body is
/// the destructor): sets the vtable, deletes `pEffectLocals` when there is
/// one (its deleting destructor with 1) and destroys the `ActiveEffect`
/// base.
pub fn fn_008277f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SCRIPT_EFFECT_VTABLE);
    let locals = e.mem.u32(base + EFFECT_LOCALS);
    if locals != 0 {
        e.call(LOCALS_DELETING_DESTRUCTOR, &args![locals, 1u32]);
    }
    e.call(ACTIVE_EFFECT_DESTROY, &args![base]);
}

// Translated from 00827880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827880 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
/// `ScriptEffect::Copy` (Xbox PDB): allocates a `ScriptEffect` (0x50
/// bytes), constructs it with this effect's `pCaster` (+0x28), `pSpell`
/// (+8) and `pEffect` (+0xC) (0 when the allocation failed), lets this
/// effect copy its data into it (vtable +0x44 on `this` with the copy)
/// and answers the copy.
pub fn script_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    let copy = if block != 0 {
        let caster = e.mem.u32(base + 0x28);
        let spell = e.mem.u32(base + 0x8);
        let effect = e.mem.u32(base + 0xc);
        fn_00827740(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(base, SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 00827930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::CopyData` (Xbox PDB), thiscall `(target)`: copies the
/// `ActiveEffect` data, then, when the target is a `ScriptEffect` (a
/// dynamic cast from `ActiveEffect`), gives it this effect's script and,
/// when this effect has a script and locals, new locals for the script
/// (`005abf60`) filled from this effect's (`005a8f40`), then each variable
/// of this effect's list (until a node that is empty) set again by id with
/// its `double` value (+8 of the variable; `FLD`/`FSTP` of a `double`: a
/// signalling NaN comes out quiet), and, when this effect's locals have a
/// block at +0x10, a new 8-byte block at +0x10 of the target's locals.
pub fn script_effect_copy_data(e: &mut Engine, this: Ptr, target: u32) {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![base, target]);
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![target, 0u32, ACTIVE_EFFECT_TYPE, SCRIPT_EFFECT_TYPE, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    let script = e.mem.u32(base + SCRIPT);
    e.mem.set_u32(cast + SCRIPT, script);
    if e.mem.u32(base + SCRIPT) == 0 || e.mem.u32(base + EFFECT_LOCALS) == 0 {
        return;
    }
    let script = e.mem.u32(base + SCRIPT);
    let locals = e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32();
    e.mem.set_u32(cast + EFFECT_LOCALS, locals);
    let source = e.mem.u32(base + EFFECT_LOCALS);
    let locals = e.mem.u32(cast + EFFECT_LOCALS);
    e.call(LOCALS_COPY, &args![locals, source]);
    let source = e.mem.u32(base + EFFECT_LOCALS);
    let mut node = e.call(LOCALS_FIRST_VARIABLE, &args![source]).u32();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let variable = e.mem.u32(item);
        let value = x87_load64(e.mem.u64(variable + 8));
        let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let variable = e.mem.u32(item);
        let id = e.mem.u32(variable);
        let locals = e.mem.u32(cast + EFFECT_LOCALS);
        e.call(LOCALS_SET_VARIABLE, &args![locals, id, value]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let source = e.mem.u32(base + EFFECT_LOCALS);
    if e.mem.u32(source + 0x10) != 0 {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let locals = e.mem.u32(cast + EFFECT_LOCALS);
        e.mem.set_u32(locals + 0x10, block);
    }
}

// Translated from 00827a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827a40 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
/// A `cdecl (caster, spell, effect)` factory (unnamed in the engine map):
/// allocates a `ScriptEffect` (0x50 bytes) and constructs it with the
/// three words; answers it, or 0 when the allocation failed.
pub fn fn_00827a40(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00827740(e, Ptr::new(block), caster, spell, effect).addr()
}

// Translated from 00827ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::Start` (Xbox PDB): when there is a script, makes its
/// locals (`pEffectLocals`) and runs its effect-start block on the
/// target's reference (`pTarget`'s vtable +4) with them.
pub fn script_effect_start(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + SCRIPT) == 0 {
        return;
    }
    let script = e.mem.u32(base + SCRIPT);
    let locals = e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32();
    e.mem.set_u32(base + EFFECT_LOCALS, locals);
    let target = e.call(ACTIVE_EFFECT_TARGET, &args![base]).u32();
    let locals = e.mem.u32(base + EFFECT_LOCALS);
    let reference = e.vcall(target, TARGET_SLOT_REFERENCE, &args![]).u32();
    let script = e.mem.u32(base + SCRIPT);
    e.call(SCRIPT_RUN_EFFECT_START, &args![script, reference, locals]);
}

// Translated from 00827bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::Update` (Xbox PDB), thiscall `(seconds)`: when there is
/// a script, runs its effect-update block on the target's reference
/// (`pTarget`'s vtable +4) with `pEffectLocals` and the seconds (passed on
/// through `FLD`/`FSTP`: a signalling NaN comes out quiet).
pub fn script_effect_update(e: &mut Engine, this: Ptr, seconds: u32) {
    let base = this.addr();
    if e.mem.u32(base + SCRIPT) == 0 {
        return;
    }
    let target = e.call(ACTIVE_EFFECT_TARGET, &args![base]).u32();
    let seconds = x87_quiet_bits(seconds);
    let locals = e.mem.u32(base + EFFECT_LOCALS);
    let reference = e.vcall(target, TARGET_SLOT_REFERENCE, &args![]).u32();
    let script = e.mem.u32(base + SCRIPT);
    e.call(
        SCRIPT_RUN_EFFECT_UPDATE,
        &args![script, reference, locals, seconds],
    );
}

// Translated from 00827e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::SaveGame_ov2` (Xbox PDB), thiscall `(buffer)`: saves the
/// `ActiveEffect` data, writes one byte saying whether there are locals
/// and, when the byte (read back after the write) is set, saves the
/// locals.
pub fn script_effect_save_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![base, buffer]);
    let has_locals = e.mem.u32(base + EFFECT_LOCALS) != 0;
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), u8::from(has_locals));
        e.call(SAVE_BUFFER_WRITE, &args![buffer, flag, 1u32, 0u32]);
        if e.mem.u8(flag.addr()) != 0 {
            let locals = e.mem.u32(base + EFFECT_LOCALS);
            e.call(LOCALS_SAVE_GAME, &args![locals, buffer]);
        }
    });
}

/// The save game object (`011de45c`).
const SAVE_GAME: u32 = 0x011d_e45c;
/// `0047c850` (`thiscall`, here on the save game object): a byte (its body answers 0).
const SAVE_GAME_FLAG_0047C850: u32 = 0x0047_c850;
/// `008df040` (`thiscall`): the save game object's version byte (+0x80).
const SAVE_VERSION: u32 = 0x008d_f040;
/// The version from which the locals' size is saved as a 16-bit word.
const VERSION_SIZED_LOCALS: i32 = 0x2a;
/// `Script::RunScriptEffectFinish` (engine map, `005ac380`; `thiscall`, two words:
/// reference, locals).
const SCRIPT_RUN_EFFECT_FINISH: u32 = 0x005a_c380;
/// `ActiveEffect::GetSaveSize` (engine map, `00804f80`), `ActiveEffect::SaveGame`
/// (`008050d0`), `ActiveEffect::LoadGame` (`00805330`), `ActiveEffect::LoadGame_ov2`
/// (`00806420`) (`thiscall`, one word each).
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
const ACTIVE_EFFECT_SAVE: u32 = 0x0080_50d0;
const ACTIVE_EFFECT_LOAD: u32 = 0x0080_5330;
const ACTIVE_EFFECT_LOAD_OV2: u32 = 0x0080_6420;
/// `004534f0` (`thiscall`, one word): an empty body.
const EMPTY_004534F0: u32 = 0x0045_34f0;
/// `ScriptLocals` helpers (`tesscript.cpp`; `thiscall`): the save size (`005a94f0`, a
/// 16-bit answer), save (`005a9670`), load (`005a9950`), revert (`005a9d00`),
/// `ScriptLocals::LoadGame` (`005a9f20`, one word: the buffer), and a stack copy's
/// constructor (`005a8b80`) and destructor (`005a8bc0`; the engine map's name,
/// `ScriptLocals::ScriptLocals`, is its caller's view).
const LOCALS_SAVE_SIZE: u32 = 0x005a_94f0;
const LOCALS_SAVE: u32 = 0x005a_9670;
const LOCALS_LOAD: u32 = 0x005a_9950;
const LOCALS_REVERT: u32 = 0x005a_9d00;
const LOCALS_LOAD_GAME: u32 = 0x005a_9f20;
const LOCALS_CONSTRUCT: u32 = 0x005a_8b80;
const LOCALS_DESTROY: u32 = 0x005a_8bc0;
/// The save game object's write (`008579b0`) and read (`008579e0`) of raw bytes
/// (`thiscall`, two words: address, size), and its skip (`00857bd0`, one word: a byte
/// count added to its position at +0x14).
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
const SAVE_SKIP: u32 = 0x0085_7bd0;
/// `00864980` (`thiscall` on a load buffer, two words: address, size): reads raw bytes.
const LOAD_BUFFER_READ: u32 = 0x0086_4980;

/// The save game object's version byte, as the exe compares it (`MOVZX`, signed `CMP`).
fn save_version(e: &mut Engine) -> i32 {
    let save_game: u32 = e.global(SAVE_GAME);
    i32::from(e.call(SAVE_VERSION, &args![save_game]).u8())
}

// Translated from 00827b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::Finish` (Xbox PDB): when there is a script and the save game object's
/// byte `0047c850` is clear (asked twice: run when the first answer is clear or the second
/// set), runs the script's effect-finish block (`005ac380`) on the target's reference
/// (`pTarget`'s vtable +4) with `pEffectLocals`; then, when there are locals, deletes them
/// (their deleting destructor with 1) and clears `pEffectLocals`.
pub fn script_effect_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + SCRIPT) != 0 {
        let save_game: u32 = e.global(SAVE_GAME);
        let run = !e.call(SAVE_GAME_FLAG_0047C850, &args![save_game]).bool() || {
            let save_game: u32 = e.global(SAVE_GAME);
            e.call(SAVE_GAME_FLAG_0047C850, &args![save_game]).bool()
        };
        if run {
            let target = e.call(ACTIVE_EFFECT_TARGET, &args![base]).u32();
            let locals = e.mem.u32(base + EFFECT_LOCALS);
            let reference = e.vcall(target, TARGET_SLOT_REFERENCE, &args![]).u32();
            let script = e.mem.u32(base + SCRIPT);
            e.call(SCRIPT_RUN_EFFECT_FINISH, &args![script, reference, locals]);
        }
    }
    if e.mem.u32(base + EFFECT_LOCALS) != 0 {
        let locals = e.mem.u32(base + EFFECT_LOCALS);
        if locals != 0 {
            e.call(LOCALS_DELETING_DESTRUCTOR, &args![locals, 1u32]);
        }
        e.mem.set_u32(base + EFFECT_LOCALS, 0);
    }
}

// Translated from 00827c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::GetSaveSize` (Xbox PDB; `thiscall`, `RET 4`; 16-bit sums): the
/// `ActiveEffect` size (`00804f80` with the same word), plus 2 from save version 0x2A on,
/// plus the locals' size (`005a94f0`) when there are locals.
pub fn script_effect_get_save_size(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let base = this.addr();
    let mut size = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    if save_version(e) >= VERSION_SIZED_LOCALS {
        size = size.wrapping_add(2);
    }
    if e.mem.u32(base + EFFECT_LOCALS) != 0 {
        let locals = e.mem.u32(base + EFFECT_LOCALS);
        let locals_size = e.call(LOCALS_SAVE_SIZE, &args![locals]).u16();
        size = size.wrapping_add(locals_size);
    }
    size
}

// Translated from 00827ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): saves the `ActiveEffect`
/// data (`008050d0`); from save version 0x2A on, writes the locals' size as a 16-bit word
/// (0 without locals; `008579b0`); before that version the size counts as 1. When the
/// size is not 0, saves the locals (`005a9670`).
pub fn script_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_SAVE, &args![this, argument]);
    e.with_stack(4, |e, size| {
        e.mem.set_u16(size.addr(), 0);
        if save_version(e) >= VERSION_SIZED_LOCALS {
            if e.mem.u32(base + EFFECT_LOCALS) != 0 {
                let locals = e.mem.u32(base + EFFECT_LOCALS);
                let locals_size = e.call(LOCALS_SAVE_SIZE, &args![locals]).u16();
                e.mem.set_u16(size.addr(), locals_size);
            }
            let save_game: u32 = e.global(SAVE_GAME);
            e.call(SAVE_WRITE, &args![save_game, size, 2u32]);
        }
        if save_version(e) < VERSION_SIZED_LOCALS {
            e.mem.set_u16(size.addr(), 1);
        }
        if e.mem.u16(size.addr()) != 0 {
            let locals = e.mem.u32(base + EFFECT_LOCALS);
            e.call(LOCALS_SAVE, &args![locals]);
        }
    });
}

// Translated from 00827d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): loads the `ActiveEffect`
/// data (`00805330`); from save version 0x2A on reads the locals' 16-bit size
/// (`008579e0`), before it the size counts as 1. With a size, makes the script's locals
/// when there are none (`005abf60`) and loads them (`005a9950`); when there still are none
/// it skips the size's bytes instead (`00857bd0`).
pub fn script_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_LOAD, &args![this, argument]);
    e.with_stack(4, |e, size| {
        e.mem.set_u16(size.addr(), 0);
        if save_version(e) >= VERSION_SIZED_LOCALS {
            let save_game: u32 = e.global(SAVE_GAME);
            e.call(SAVE_READ, &args![save_game, size, 2u32]);
        }
        if save_version(e) < VERSION_SIZED_LOCALS {
            e.mem.set_u16(size.addr(), 1);
        }
        let bytes = e.mem.u16(size.addr());
        if bytes == 0 {
            return;
        }
        if e.mem.u32(base + EFFECT_LOCALS) == 0 {
            let script = e.mem.u32(base + SCRIPT);
            let locals = e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32();
            e.mem.set_u32(base + EFFECT_LOCALS, locals);
        }
        if e.mem.u32(base + EFFECT_LOCALS) != 0 {
            let locals = e.mem.u32(base + EFFECT_LOCALS);
            e.call(LOCALS_LOAD, &args![locals]);
        } else {
            let save_game: u32 = e.global(SAVE_GAME);
            e.call(SAVE_SKIP, &args![save_game, u32::from(bytes)]);
        }
    });
}

// Translated from 00827de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptEffect::Revert` (Xbox PDB; `thiscall`, `RET 4`): the base's `004534f0` (an empty
/// body) with the same word, then, when there are locals, their revert (`005a9d00`).
pub fn script_effect_revert(e: &mut Engine, this: Ptr, argument: u32) {
    let base = this.addr();
    e.call(EMPTY_004534F0, &args![this, argument]);
    if e.mem.u32(base + EFFECT_LOCALS) != 0 {
        let locals = e.mem.u32(base + EFFECT_LOCALS);
        e.call(LOCALS_REVERT, &args![locals]);
    }
}

// Translated from 00827e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827e60 - the C++ exception-unwinding frame (FS:[0]) that destroys the stack ScriptLocals when its load throws is not translated
/// `ScriptEffect::LoadGame_ov2` (Xbox PDB; `thiscall`, `RET 4`): loads the `ActiveEffect`
/// data (`00806420`) and reads one byte from the buffer (`00864980`). When the byte is
/// set, makes the script's locals when there are none (`005abf60`) and loads them from the
/// buffer (`005a9f20`); when there still are none, loads into a 0x14-byte `ScriptLocals`
/// on the stack (`005a8b80`, `005a9f20`, `005a8bc0`) to skip the data.
pub fn script_effect_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(ACTIVE_EFFECT_LOAD_OV2, &args![this, buffer]);
    e.with_stack(0x20, |e, frame| {
        let flag = frame.addr() + 0x18;
        let scratch_locals = frame.addr();
        e.mem.set_u8(flag, 0);
        e.call(LOAD_BUFFER_READ, &args![buffer, flag, 1u32]);
        if e.mem.u8(flag) == 0 {
            return;
        }
        if e.mem.u32(base + EFFECT_LOCALS) == 0 {
            let script = e.mem.u32(base + SCRIPT);
            let locals = e.call(SCRIPT_CREATE_LOCALS, &args![script]).u32();
            e.mem.set_u32(base + EFFECT_LOCALS, locals);
        }
        if e.mem.u32(base + EFFECT_LOCALS) != 0 {
            let locals = e.mem.u32(base + EFFECT_LOCALS);
            e.call(LOCALS_LOAD_GAME, &args![locals, buffer]);
        } else {
            e.call(LOCALS_CONSTRUCT, &args![scratch_locals]);
            e.call(LOCALS_LOAD_GAME, &args![scratch_locals, buffer]);
            e.call(LOCALS_DESTROY, &args![scratch_locals]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_7b20, script_effect_finish(Ptr)),
        entry!(0x0082_7c20, script_effect_get_save_size(Ptr, u32) -> u16),
        entry!(0x0082_7ca0, script_effect_save_game(Ptr, u32)),
        entry!(0x0082_7d30, script_effect_load_game(Ptr, u32)),
        entry!(0x0082_7de0, script_effect_revert(Ptr, u32)),
        entry!(0x0082_7e60, script_effect_load_game_ov2(Ptr, u32)),
        entry!(0x00827740, fn_00827740(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x008277c0,
            script_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x008277f0, fn_008277f0(Ptr)),
        entry!(0x00827880, script_effect_copy(Ptr) -> u32),
        entry!(0x00827930, script_effect_copy_data(Ptr, u32)),
        entry!(0x00827a40, fn_00827a40(u32, u32, u32) -> u32),
        entry!(0x00827ac0, script_effect_start(Ptr)),
        entry!(0x00827bd0, script_effect_update(Ptr, u32)),
        entry!(0x00827e10, script_effect_save_game_ov2(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_DOUBLE: u32 = 0xdead_1000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// An object whose vtable has `SLOT_DOUBLE` at every slot.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x50);
        for slot in 0..0x14 {
            e.mem.set_u32(vtable + 4 * slot, SLOT_DOUBLE);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn constructor_and_factory_build_the_effect() {
        let mut e = Engine::new();
        quiet(&mut e, &[ACTIVE_EFFECT_CONSTRUCT]);
        e.register(EFFECT_ITEM_SCRIPT, |_, a| ret(a[0] + 1));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + EFFECT_LOCALS, 9);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_7740, &args![this, 1u32, 2u32, 0x30u32]).u32(),
            this
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![this, 1, 2, 0x30]]
        );
        assert_eq!(e.mem.u32(this), SCRIPT_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + SCRIPT), 0x31);
        assert_eq!(e.mem.u32(this + EFFECT_LOCALS), 0);
        let made = e.call(0x0082_7a40, &args![1u32, 2u32, 0x40u32]).u32();
        assert_eq!(e.mem.u32(made + SCRIPT), 0x41);
        e.register(ALLOCATE, |_, _| ret(0));
        assert_eq!(e.call(0x0082_7a40, &args![1u32, 2u32, 0x40u32]).u32(), 0);
    }

    #[test]
    fn destructor_deletes_the_locals_and_frees_on_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[LOCALS_DELETING_DESTRUCTOR, ACTIVE_EFFECT_DESTROY, FREE],
        );
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + EFFECT_LOCALS, 0x700);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_77c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SCRIPT_EFFECT_VTABLE);
        assert_eq!(calls(&e, LOCALS_DELETING_DESTRUCTOR), vec![vec![0x700, 1]]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DESTROY), vec![vec![this]]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
        e.mem.set_u32(this + EFFECT_LOCALS, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_77c0, &args![this, 0u32]);
        assert!(calls(&e, LOCALS_DELETING_DESTRUCTOR).is_empty());
        assert!(calls(&e, FREE).is_empty());
    }

    #[test]
    fn copy_constructs_a_copy_and_lets_it_take_the_data() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[ACTIVE_EFFECT_CONSTRUCT, EFFECT_ITEM_SCRIPT, SLOT_DOUBLE],
        );
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let this = object(&mut e, 0x50);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u32(this + 0x8, 0x8);
        e.mem.set_u32(this + 0xc, 0xc);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0082_7880, &args![this]).u32();
        assert_ne!(copy, 0);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![copy, 0x28, 0x8, 0xc]]
        );
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![this, copy]]);
        e.register(ALLOCATE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_7880, &args![this]).u32(), 0);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![this, 0]]);
    }

    #[test]
    fn copy_data_copies_the_script_and_its_variables() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[ACTIVE_EFFECT_COPY_DATA, LOCALS_COPY, LOCALS_SET_VARIABLE],
        );
        let target = e.mem.alloc(0x50);
        let new_locals = e.mem.alloc(0x20);
        e.register_double(DYNAMIC_CAST, move |_, a| ret(a[0]));
        e.register_double(SCRIPT_CREATE_LOCALS, move |_, _| ret(new_locals));
        e.register(ALLOCATE, |_, _| ret(0x8888));
        e.register(LOCALS_FIRST_VARIABLE, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        let variable = e.mem.alloc(0x10);
        e.mem.set_u32(variable, 7);
        e.mem.set_u64(variable + 8, 0x7ff0_0000_0000_0001);
        let empty = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, variable);
        e.mem.set_u32(node + 4, empty);
        let locals = e.mem.alloc(0x20);
        e.mem.set_u32(locals + 0xc, node);
        e.mem.set_u32(locals + 0x10, 1);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + SCRIPT, 0x500);
        e.mem.set_u32(this + EFFECT_LOCALS, locals);
        e.call_log = Some(vec![]);
        e.call(0x0082_7930, &args![this, target]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![target, 0, ACTIVE_EFFECT_TYPE, SCRIPT_EFFECT_TYPE, 0]]
        );
        assert_eq!(e.mem.u32(target + SCRIPT), 0x500);
        assert_eq!(e.mem.u32(target + EFFECT_LOCALS), new_locals);
        assert_eq!(calls(&e, LOCALS_COPY), vec![vec![new_locals, locals]]);
        assert_eq!(
            calls(&e, LOCALS_SET_VARIABLE),
            vec![vec![new_locals, 7, 1, 0x7ff8_0000]]
        );
        assert_eq!(e.mem.u32(new_locals + 0x10), 0x8888);
        // No locals: only the script is copied.
        e.mem.set_u32(this + EFFECT_LOCALS, 0);
        e.mem.set_u32(target + SCRIPT, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_7930, &args![this, target]);
        assert_eq!(e.mem.u32(target + SCRIPT), 0x500);
        assert!(calls(&e, SCRIPT_CREATE_LOCALS).is_empty());
        // Not a script effect: nothing after the base copy.
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.mem.set_u32(target + SCRIPT, 0);
        e.call(0x0082_7930, &args![this, target]);
        assert_eq!(e.mem.u32(target + SCRIPT), 0);
    }

    #[test]
    fn start_makes_the_locals_and_runs_the_start_block() {
        let mut e = Engine::new();
        quiet(&mut e, &[SCRIPT_RUN_EFFECT_START]);
        let target = object(&mut e, 8);
        e.register(SLOT_DOUBLE, |_, _| ret(0x1234));
        e.register(SCRIPT_CREATE_LOCALS, |_, _| ret(0x600));
        e.register(ACTIVE_EFFECT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + SCRIPT, 0x500);
        e.call_log = Some(vec![]);
        e.call(0x0082_7ac0, &args![this]);
        assert_eq!(e.mem.u32(this + EFFECT_LOCALS), 0x600);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![target]]);
        assert_eq!(
            calls(&e, SCRIPT_RUN_EFFECT_START),
            vec![vec![0x500, 0x1234, 0x600]]
        );
        e.mem.set_u32(this + SCRIPT, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_7ac0, &args![this]);
        assert!(calls(&e, SCRIPT_CREATE_LOCALS).is_empty());
    }

    #[test]
    fn update_runs_the_update_block_with_the_seconds() {
        let mut e = Engine::new();
        quiet(&mut e, &[SCRIPT_RUN_EFFECT_UPDATE]);
        let target = object(&mut e, 8);
        e.register(SLOT_DOUBLE, |_, _| ret(0x1234));
        e.register(ACTIVE_EFFECT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + SCRIPT, 0x500);
        e.mem.set_u32(this + EFFECT_LOCALS, 0x600);
        e.call_log = Some(vec![]);
        e.call(0x0082_7bd0, &args![this, 0x7f80_0002u32]);
        assert_eq!(
            calls(&e, SCRIPT_RUN_EFFECT_UPDATE),
            vec![vec![0x500, 0x1234, 0x600, 0x7fc0_0002]]
        );
        e.mem.set_u32(this + SCRIPT, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_7bd0, &args![this, 0u32]);
        assert!(calls(&e, ACTIVE_EFFECT_TARGET).is_empty());
    }

    #[test]
    fn save_game_writes_the_flag_and_the_locals() {
        let mut e = Engine::new();
        quiet(&mut e, &[ACTIVE_EFFECT_SAVE_GAME, LOCALS_SAVE_GAME]);
        let written = std::rc::Rc::new(std::cell::Cell::new(0xffu8));
        let seen = written.clone();
        e.register_double(SAVE_BUFFER_WRITE, move |e, a| {
            seen.set(e.mem.u8(a[1]));
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + EFFECT_LOCALS, 0x600);
        e.call_log = Some(vec![]);
        e.call(0x0082_7e10, &args![this, 0x900u32]);
        assert_eq!(written.get(), 1);
        assert_eq!(calls(&e, ACTIVE_EFFECT_SAVE_GAME), vec![vec![this, 0x900]]);
        assert_eq!(calls(&e, SAVE_BUFFER_WRITE)[0][2..], [1, 0]);
        assert_eq!(calls(&e, LOCALS_SAVE_GAME), vec![vec![0x600, 0x900]]);
        e.mem.set_u32(this + EFFECT_LOCALS, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_7e10, &args![this, 0x900u32]);
        assert_eq!(written.get(), 0);
        assert!(calls(&e, LOCALS_SAVE_GAME).is_empty());
    }
}

#[cfg(test)]
mod n049_tests {
    use super::*;

    const REFERENCE_DOUBLE: u32 = 0x0e70_0000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An effect with `script` and `locals`, the save game object at 0x3000 with
    /// `version`, and every callee doubled.
    fn engine(script: u32, locals: u32, version: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(SAVE_GAME, 0x3000);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + SCRIPT, script);
        e.mem.set_u32(this + EFFECT_LOCALS, locals);
        let vtable = e.mem.alloc(0x10);
        e.mem
            .set_u32(vtable + TARGET_SLOT_REFERENCE, REFERENCE_DOUBLE);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, vtable);
        e.register_double(ACTIVE_EFFECT_TARGET, move |_, _| target.into_ret());
        e.register(REFERENCE_DOUBLE, |_, _| 0x5150u32.into_ret());
        e.register_double(SAVE_VERSION, move |_, _| version.into_ret());
        e.register(SAVE_GAME_FLAG_0047C850, |_, _| 0u32.into_ret());
        e.register(LOCALS_SAVE_SIZE, |_, _| 0x1_0020u32.into_ret());
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| 0x100u32.into_ret());
        e.register(SCRIPT_CREATE_LOCALS, |_, _| 0x6600u32.into_ret());
        e.register(SAVE_READ, |e, a| {
            e.mem.set_u16(a[1], 0x33);
            Ret::default()
        });
        e.register(LOAD_BUFFER_READ, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        for callee in [
            SCRIPT_RUN_EFFECT_FINISH,
            LOCALS_DELETING_DESTRUCTOR,
            ACTIVE_EFFECT_SAVE,
            ACTIVE_EFFECT_LOAD,
            ACTIVE_EFFECT_LOAD_OV2,
            EMPTY_004534F0,
            LOCALS_SAVE,
            LOCALS_LOAD,
            LOCALS_REVERT,
            LOCALS_LOAD_GAME,
            LOCALS_CONSTRUCT,
            LOCALS_DESTROY,
            SAVE_WRITE,
            SAVE_SKIP,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn finish_runs_the_script_and_deletes_the_locals() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x50);
        e.call(0x0082_7b20, &[this]);
        assert_eq!(
            calls(&e, SCRIPT_RUN_EFFECT_FINISH),
            vec![vec![0x4400, 0x5150, 0x5500]]
        );
        assert_eq!(calls(&e, SAVE_GAME_FLAG_0047C850), vec![vec![0x3000]]);
        assert_eq!(calls(&e, LOCALS_DELETING_DESTRUCTOR), vec![vec![0x5500, 1]]);
        assert_eq!(e.mem.u32(this + EFFECT_LOCALS), 0);
    }

    #[test]
    fn finish_asks_twice_and_skips_without_script() {
        let (mut e, this) = engine(0x4400, 0, 0x50);
        e.register(SAVE_GAME_FLAG_0047C850, |_, _| 1u32.into_ret());
        e.call(0x0082_7b20, &[this]);
        assert_eq!(calls(&e, SAVE_GAME_FLAG_0047C850).len(), 2);
        assert_eq!(calls(&e, SCRIPT_RUN_EFFECT_FINISH).len(), 1);
        assert!(calls(&e, LOCALS_DELETING_DESTRUCTOR).is_empty());

        let (mut e, this) = engine(0, 0x5500, 0x50);
        e.call(0x0082_7b20, &[this]);
        assert!(calls(&e, SCRIPT_RUN_EFFECT_FINISH).is_empty());
        assert!(calls(&e, SAVE_GAME_FLAG_0047C850).is_empty());
        assert_eq!(calls(&e, LOCALS_DELETING_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn get_save_size_by_version_and_locals() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x2a);
        assert_eq!(e.call(0x0082_7c20, &[this, 9]).u32(), 0x122);
        assert_eq!(calls(&e, ACTIVE_EFFECT_GET_SAVE_SIZE), vec![vec![this, 9]]);
        let (mut e, this) = engine(0x4400, 0, 0x29);
        assert_eq!(e.call(0x0082_7c20, &[this, 9]).u32(), 0x100);
    }

    #[test]
    fn save_game_writes_the_size_then_the_locals() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x2a);
        e.call(0x0082_7ca0, &[this, 9]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_SAVE), vec![vec![this, 9]]);
        let writes = calls(&e, SAVE_WRITE);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][2], 2);
        assert_eq!(calls(&e, LOCALS_SAVE), vec![vec![0x5500]]);

        // Without locals the size 0 is written and nothing more.
        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.call(0x0082_7ca0, &[this, 9]);
        assert_eq!(calls(&e, SAVE_WRITE).len(), 1);
        assert!(calls(&e, LOCALS_SAVE).is_empty());

        // An old version writes nothing and saves the locals.
        let (mut e, this) = engine(0x4400, 0x5500, 0x10);
        e.call(0x0082_7ca0, &[this, 9]);
        assert!(calls(&e, SAVE_WRITE).is_empty());
        assert_eq!(calls(&e, LOCALS_SAVE), vec![vec![0x5500]]);
    }

    #[test]
    fn save_game_writes_the_locals_size_word() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x2a);
        let probe = e.mem.alloc(4);
        e.register_double(SAVE_WRITE, move |e, a| {
            let word = e.mem.u16(a[1]);
            e.mem.set_u16(probe, word);
            Ret::default()
        });
        e.call(0x0082_7ca0, &[this, 9]);
        assert_eq!(e.mem.u16(probe), 0x20);
    }

    #[test]
    fn load_game_reads_and_loads_or_skips() {
        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.call(0x0082_7d30, &[this, 9]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_LOAD), vec![vec![this, 9]]);
        assert_eq!(calls(&e, SAVE_READ).len(), 1);
        assert_eq!(calls(&e, SCRIPT_CREATE_LOCALS), vec![vec![0x4400]]);
        assert_eq!(e.mem.u32(this + EFFECT_LOCALS), 0x6600);
        assert_eq!(calls(&e, LOCALS_LOAD), vec![vec![0x6600]]);

        // No locals can be made: the bytes are skipped.
        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.register(SCRIPT_CREATE_LOCALS, |_, _| 0u32.into_ret());
        e.call(0x0082_7d30, &[this, 9]);
        assert_eq!(calls(&e, SAVE_SKIP), vec![vec![0x3000, 0x33]]);

        // A zero size does nothing more.
        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.register(SAVE_READ, |_, _| Ret::default());
        e.call(0x0082_7d30, &[this, 9]);
        assert!(calls(&e, SCRIPT_CREATE_LOCALS).is_empty());

        // An old version reads nothing and counts 1.
        let (mut e, this) = engine(0x4400, 0, 0x10);
        e.register(SCRIPT_CREATE_LOCALS, |_, _| 0u32.into_ret());
        e.call(0x0082_7d30, &[this, 9]);
        assert!(calls(&e, SAVE_READ).is_empty());
        assert_eq!(calls(&e, SAVE_SKIP), vec![vec![0x3000, 1]]);
    }

    #[test]
    fn revert_reverts_the_locals() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x2a);
        e.call(0x0082_7de0, &[this, 9]);
        assert_eq!(calls(&e, EMPTY_004534F0), vec![vec![this, 9]]);
        assert_eq!(calls(&e, LOCALS_REVERT), vec![vec![0x5500]]);
        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.call(0x0082_7de0, &[this, 9]);
        assert!(calls(&e, LOCALS_REVERT).is_empty());
    }

    #[test]
    fn load_game_ov2_loads_into_the_locals_or_a_scratch_copy() {
        let (mut e, this) = engine(0x4400, 0x5500, 0x2a);
        e.call(0x0082_7e60, &[this, 0x900]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_LOAD_OV2), vec![vec![this, 0x900]]);
        assert_eq!(calls(&e, LOAD_BUFFER_READ)[0][2], 1);
        assert_eq!(calls(&e, LOCALS_LOAD_GAME), vec![vec![0x5500, 0x900]]);

        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.register(SCRIPT_CREATE_LOCALS, |_, _| 0u32.into_ret());
        e.call(0x0082_7e60, &[this, 0x900]);
        let scratch = calls(&e, LOCALS_CONSTRUCT);
        assert_eq!(scratch.len(), 1);
        assert_eq!(
            calls(&e, LOCALS_LOAD_GAME),
            vec![vec![scratch[0][0], 0x900]]
        );
        assert_eq!(calls(&e, LOCALS_DESTROY), vec![vec![scratch[0][0]]]);

        let (mut e, this) = engine(0x4400, 0, 0x2a);
        e.register(LOAD_BUFFER_READ, |_, _| Ret::default());
        e.call(0x0082_7e60, &[this, 0x900]);
        assert!(calls(&e, LOCALS_LOAD_GAME).is_empty());
        assert!(calls(&e, SCRIPT_CREATE_LOCALS).is_empty());
    }
}
