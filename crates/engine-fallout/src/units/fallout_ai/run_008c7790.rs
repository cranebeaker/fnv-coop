//! `fallout/ai/run_008c7790` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object whose value word `0043d4d0` locates (its `+4`, or a static
/// zero word for null); `*value > 1` enables the whole check.
const GATE_OBJECT: u32 = 0x011c_3ea4;
/// The global that holds the object `0044edb0` is called on (its `+0x10` is
/// the current procedure index, `BaseProcess::GetCurrentProcedureIndex`
/// (Xbox PDB name of `0044edb0`)).
const PROCESS_REFERENCE: u32 = 0x011d_ea0c;

/// `0043d4d0`: address of the word at `+4` of the object (a static zero word
/// for a null object).
const VALUE_WORD_ADDRESS: u32 = 0x0043_d4d0;
/// `00713d90`: a global byte.
const ENABLED_FLAG: u32 = 0x0071_3d90;
/// `004537b0`: a global pointer.
const CURRENT_OBJECT: u32 = 0x0045_37b0;
/// `0087a730` (`__thiscall` on the object): a bool.
const OBJECT_QUALIFIES: u32 = 0x0087_a730;
/// `0040fc90`: `GetCurrentThreadId` wrapper.
const THREAD_ID: u32 = 0x0040_fc90;
/// `0044edb0` (`__thiscall`): the word at `+0x10`.
const PROCEDURE_INDEX: u32 = 0x0044_edb0;
/// `008c7bc0`: a global byte.
const SECOND_FLAG: u32 = 0x008c_7bc0;
/// `00713d80`: the address of a static table.
const TABLE: u32 = 0x0071_3d80;
/// `008c7a30` (`__thiscall` on the table, `RET 4`): the table entry for an
/// index below 2, else 0.
const TABLE_LOOKUP: u32 = 0x008c_7a30;
/// `0084e3a0`: the word at `+0xc` of the object.
const OBJECT_VALUE: u32 = 0x0084_e3a0;

/// Entry `index` of the static table.
fn table_entry(e: &mut Engine, index: u32) -> u32 {
    let table = e.call(TABLE, &args![]).u32();
    e.call(TABLE_LOOKUP, &args![table, index]).u32()
}

// ---------------------------------------------------------------------------
// The object the functions at `008c7790`..`008c8110` work on. Its class is
// not named in the Xbox PDB map; only its layout is known from the code:
//
//   +0x00  two words: non-null when slot `i` has a worker (`008c9fb0` and
//          `008c7490` are called on the word)
//   +0x0c  two words: the current stage of slot `i`
//   +0x18  three 12-byte members (`008c8110` destroys them as an array;
//          `008c7a30` reads the first word of member `i`)
//   +0x30, +0x3c  the members `008c77c0` closes (`0055a2d0`)
//   +0x3c  also 12 event handles, reset by `008c78c0`
//   +0x50  a member `0052d240` destroys
//   +0x6c  8 more event handles, reset by `008c78c0`
//   +0x8c  two pointers: slot `i`'s array of event handles (one per stage)
//
// The single instance sits at the address `00713d80` returns.
// ---------------------------------------------------------------------------

/// Import slot of `ResetEvent` (named by Ghidra): handle.
const RESET_EVENT: u32 = 0x00fd_f1cc;
/// Import slot of `SetEvent` (named by Ghidra): handle.
const SET_EVENT: u32 = 0x00fd_f0d0;
/// Import slot of `WaitForSingleObject` (named by Ghidra): handle, timeout.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0055a2d0` (`__thiscall`): its body closes the handle at `+4` of the
/// member it is called on.
const CLOSE_HANDLE_MEMBER: u32 = 0x0055_a2d0;
/// `0052d240` (`__thiscall`): destroys a member (its body calls `00483710`).
const DESTROY_MEMBER: u32 = 0x0052_d240;
/// `00aa63c0` (`__thiscall`): the destructor of the object's base part
/// (sets a vtable word and deletes the critical section at `+4`).
const DESTROY_BASE: u32 = 0x00aa_63c0;
/// `008c7490` (`__thiscall` on a slot's worker): its body calls the
/// wait wrapper `004424e0`.
const WORKER_WAIT: u32 = 0x008c_7490;
/// `008c9fb0` (`__thiscall` on a slot's worker): its body calls the
/// semaphore release wrapper `00442550`.
const WORKER_RELEASE: u32 = 0x008c_9fb0;
/// `00559450` (`__thiscall`): the first word of the member.
const MEMBER_FIRST_WORD: u32 = 0x0055_9450;
/// The byte `00713d90` returns (set by `008c78c0`, cleared by `008c7990`).
const WORKERS_RUNNING: u32 = 0x011d_fa19;
/// The byte `008c7bc0` returns and `008c80e0` sets.
const SECOND_FLAG_BYTE: u32 = 0x011d_fa18;
/// The object `004424e0` (wait) and `00442550` (release) are called on.
const SEMAPHORE_OBJECT: u32 = 0x011d_fafc;
/// `004424e0` (`__fastcall`; Xbox PDB name `WaitForSingleObject`): waits on
/// the handle at `+4` of the object.
const SEMAPHORE_WAIT: u32 = 0x0044_24e0;
/// `00442550` (`__fastcall`; Xbox PDB name `ReleaseSemaphore`): releases
/// the semaphore whose handle is at `+4` of the object.
const SEMAPHORE_RELEASE: u32 = 0x0044_2550;
/// `__ehvec_dtor` (`00ec5fce`, `RET 0x10`): array, element size, count,
/// destructor.
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The destructor of one 12-byte member (`008c7270`).
const MEMBER_DESTRUCTOR: u32 = 0x008c_7270;

// The callees of the stage sequences (`008c7bd0`, `008c7da0`, `008c7f50`).
// None of them is named in the Xbox PDB map except `008d0600`; what their
// bodies show is said where it is known. `this` is the global they are
// called on, when they take one.

/// `0086fd70`, called on the word the global `PROCESS_REFERENCE` holds.
const STEP_0086FD70: u32 = 0x0086_fd70;
/// `0087a6b0`, called on `CURRENT_OBJECT`'s result.
const STEP_0087A6B0: u32 = 0x0087_a6b0;
/// `0087a6d0`, called on `CURRENT_OBJECT`'s result.
const STEP_0087A6D0: u32 = 0x0087_a6d0;
/// `005c42d0`: returns the global byte `011dea2d`.
const STEP_005C42D0: u32 = 0x005c_42d0;
/// `00991500(this, byte)`, called on the word at `OBJECT_A`.
const STEP_00991500: u32 = 0x0099_1500;
/// `00991dc0`, called on the word at `OBJECT_A`.
const STEP_00991DC0: u32 = 0x0099_1dc0;
/// The global word `011f1958`.
const OBJECT_A: u32 = 0x011f_1958;
/// `0040fbf0(this, 0)`, called on the object at `OBJECT_B`.
const STEP_0040FBF0: u32 = 0x0040_fbf0;
/// `0040fba0`, called on the object at `OBJECT_B`.
const STEP_0040FBA0: u32 = 0x0040_fba0;
/// The object at `011f11a0`.
const OBJECT_B: u32 = 0x011f_11a0;
/// The object at `011e0e80`; `ProcessLists::PrintLists` (Xbox PDB name of
/// `008d0600`) is called on it, as are the `STEP_0096..` and `009784c0`.
const OBJECT_C: u32 = 0x011e_0e80;
const STEP_0096CCA0: u32 = 0x0096_cca0;
const STEP_0096CDA0: u32 = 0x0096_cda0;
const STEP_009784C0: u32 = 0x0097_84c0;
/// `0096bcd0(this, 0.0f, 0)`.
const STEP_0096BCD0: u32 = 0x0096_bcd0;
const STEP_0096C330: u32 = 0x0096_c330;
/// `0096db30(this, 0.0f, 0, 0)`.
const STEP_0096DB30: u32 = 0x0096_db30;
/// `ProcessLists::PrintLists` (Xbox PDB name of `008d0600`):
/// `(this, 0.0f, 0)`; its PC body is empty.
const PRINT_LISTS: u32 = 0x008d_0600;
const STEP_0096C7C0: u32 = 0x0096_c7c0;
const STEP_0096CB50: u32 = 0x0096_cb50;
/// `0096d520` takes no arguments (it reads a global).
const STEP_0096D520: u32 = 0x0096_d520;
/// `00455640`, called on the word at `OBJECT_D`.
const STEP_00455640: u32 = 0x0045_5640;
/// `00453550(this, float)`, called on the word at `OBJECT_D`.
const STEP_00453550: u32 = 0x0045_3550;
/// The global word `011dea10`.
const OBJECT_D: u32 = 0x011d_ea10;
/// `00453850`: returns the global float at `011dea30` in `ST0`.
const STEP_00453850: u32 = 0x0045_3850;
/// `004772f0`: no arguments.
const STEP_004772F0: u32 = 0x0047_72f0;
/// `005ae270`: no arguments (clears three words of the thread block).
const STEP_005AE270: u32 = 0x005a_e270;
/// `005a9d60`: no arguments (writes three words of the thread block).
const STEP_005A9D60: u32 = 0x005a_9d60;
/// `00408d60`, called on the object at `OBJECT_E`: the address of a byte
/// (`+4` of the object, a static zero byte when it is null).
const STEP_00408D60: u32 = 0x0040_8d60;
/// The object at `011d73e4`.
const OBJECT_E: u32 = 0x011d_73e4;
/// `006c0720` (no arguments) returns an object that `006c3640` is called on.
const STEP_006C0720: u32 = 0x006c_0720;
/// `006c3640`, called on `006c0720`'s result.
const STEP_006C3640: u32 = 0x006c_3640;

/// `base + index * 4 + offset` with `index` read as a signed word, as the
/// code indexes.
fn indexed(base: u32, index: i32, offset: u32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(4))
        .wrapping_add(offset)
}

/// The object `00713d80` returns.
fn table(e: &mut Engine) -> u32 {
    e.call(TABLE, &args![]).u32()
}

/// `008c79e0(table, slot, stage)`.
fn signal(e: &mut Engine, slot: i32, stage: i32) {
    let this = table(e);
    fn_008c79e0(e, this, slot, stage);
}

/// `008c7a70(table, slot, stage)`.
fn wait(e: &mut Engine, slot: i32, stage: i32) {
    let this = table(e);
    fn_008c7a70(e, this, slot, stage);
}

/// `008c7d80(table, slot, stage)`.
fn set_stage(e: &mut Engine, slot: i32, stage: i32) {
    let this = table(e);
    fn_008c7d80(e, this, slot, stage);
}

/// `00408d60` on its object, then `006c0720` / `006c3640` when the byte it
/// locates is non-zero.
fn optional_step(e: &mut Engine) {
    let byte = e.call(STEP_00408D60, &args![OBJECT_E]).u32();
    if e.mem.u8(byte) != 0 {
        let object = e.call(STEP_006C0720, &args![]).u32();
        e.call(STEP_006C3640, &args![object]);
    }
}

/// `0086fd70` on the word the process reference global holds.
fn step_process_reference(e: &mut Engine) {
    let reference = e.mem.u32(PROCESS_REFERENCE);
    e.call(STEP_0086FD70, &args![reference]);
}

/// `step` (`0087a6b0` or `0087a6d0`) on the current object.
fn step_current_object(e: &mut Engine, step: u32) {
    let object = e.call(CURRENT_OBJECT, &args![]).u32();
    e.call(step, &args![object]);
}

/// `00453850` (a float in `ST0`) passed to `00453550` on `OBJECT_D`'s word.
fn step_float(e: &mut Engine) {
    let value = e.call(STEP_00453850, &args![]).f32();
    let object = e.mem.u32(OBJECT_D);
    e.call(STEP_00453550, &args![object, value]);
}

// Translated from 008c7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the object: `008c77c0`, then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_008c7790(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_008c77c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c77c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c77c0 - the compiler's FS:[0] exception frame (unwinding of the first member if the second destructor throws) is not translated
/// Destructor body: closes the member at `+0x3c`, then the one at `+0x30`,
/// then destroys the base part (`00aa63c0`).
pub fn fn_008c77c0(e: &mut Engine, this: u32) {
    e.call(CLOSE_HANDLE_MEMBER, &args![this.wrapping_add(0x3c)]);
    e.call(CLOSE_HANDLE_MEMBER, &args![this.wrapping_add(0x30)]);
    e.call(DESTROY_BASE, &args![this]);
}

// Translated from 008c7830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a second class (`008c7860`, then
/// `operator delete` when bit 0 of `flags` is set). Returns `this`.
pub fn fn_008c7830(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_008c7860(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008c7860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008c7860 - the compiler's FS:[0] exception frame (unwinding if the member destructor throws) is not translated
/// Destructor body: destroys the member at `+0x50` (`0052d240`), then runs
/// `008c77c0`. Ghidra matches the body to a library destructor
/// (`_CancellationTokenCallback`); that identity is unconfirmed here.
pub fn fn_008c7860(e: &mut Engine, this: u32) {
    e.call(DESTROY_MEMBER, &args![this.wrapping_add(0x50)]);
    fn_008c77c0(e, this);
}

// Translated from 008c78c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets every event (12 at `+0x3c`, 8 at `+0x6c`), zeroes both slots'
/// stage words (`+0x0c`), sets the running byte (`011dfa19`) and calls
/// `008c9fb0` on every slot that has a worker (`+0x00`, `+0x04`).
pub fn fn_008c78c0(e: &mut Engine, this: u32) {
    for i in 0..0xc {
        let handle = e.mem.u32(this + 0x3c + i * 4);
        e.call(RESET_EVENT, &args![handle]);
    }
    for i in 0..8 {
        let handle = e.mem.u32(this + 0x6c + i * 4);
        e.call(RESET_EVENT, &args![handle]);
    }
    for i in 0..2 {
        e.mem.set_u32(this + 0x0c + i * 4, 0);
    }
    e.set_global::<u8>(WORKERS_RUNNING, 1);
    for i in 0..2 {
        let worker = e.mem.u32(this + i * 4);
        if worker != 0 {
            e.call(WORKER_RELEASE, &args![worker]);
        }
    }
}

// Translated from 008c7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `008c7490` on every slot that has a worker, then clears the
/// running byte (`011dfa19`).
pub fn fn_008c7990(e: &mut Engine, this: u32) {
    for i in 0..2 {
        let worker = e.mem.u32(this + i * 4);
        if worker != 0 {
            e.call(WORKER_WAIT, &args![worker]);
        }
    }
    e.set_global::<u8>(WORKERS_RUNNING, 0);
}

// Translated from 008c79e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `slot` is below 2 (signed) and the slot has a worker: sets the
/// event of `stage` in the slot's event array (`+0x8c`) and records `stage`
/// as the slot's current stage (`+0x0c`).
pub fn fn_008c79e0(e: &mut Engine, this: u32, slot: i32, stage: i32) {
    if slot < 2 && e.mem.u32(indexed(this, slot, 0)) != 0 {
        let events = e.mem.u32(indexed(this, slot, 0x8c));
        let handle = e.mem.u32(indexed(events, stage, 0));
        e.call(SET_EVENT, &args![handle]);
        e.mem.set_i32(indexed(this, slot, 0x0c), stage);
    }
}

// Translated from 008c7a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// For `slot` below 2 (signed): the slot's stage word (`+0x0c`) plus the
/// first word of the member at `+0x18 + slot * 12` (`00559450`); else 0.
pub fn fn_008c7a30(e: &mut Engine, this: u32, slot: i32) -> u32 {
    if slot >= 2 {
        return 0;
    }
    let member = this
        .wrapping_add(0x18)
        .wrapping_add((slot as u32).wrapping_mul(0xc));
    let first = e.call(MEMBER_FIRST_WORD, &args![member]).u32();
    e.mem.u32(indexed(this, slot, 0x0c)).wrapping_add(first)
}

// Translated from 008c7a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits without timeout on the event of `stage` in `slot`'s event array.
pub fn fn_008c7a70(e: &mut Engine, this: u32, slot: i32, stage: i32) {
    let events = e.mem.u32(indexed(this, slot, 0x8c));
    let handle = e.mem.u32(indexed(events, stage, 0));
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
}

// Translated from 008c7aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the current thread has to be treated specially (the result is an
/// `OR` of two conditions). Nothing happens unless the gate value is above 1
/// and the enabled flag is set. Then:
///
/// - `b` is whether the current object qualifies (`0087a730`);
/// - `a` is decided by comparing the thread id with the current procedure
///   index. Equal: unless the second flag is set, `a` holds when table entry
///   0 is below 11 or table entry 1 is below 7. Different: `a` holds when the
///   second flag is set or table entry 1 is below 6.
/// - if `a` is still false, it holds when the current object's value word
///   (`0084e3a0`) is non-zero and differs from the thread id.
///
/// Returns `a || b`. The names of the flags and table entries are not known.
pub fn fn_008c7aa0(e: &mut Engine) -> bool {
    let mut a = false;
    let mut b = false;
    let gate = e.call(VALUE_WORD_ADDRESS, &args![GATE_OBJECT]).u32();
    if e.mem.i32(gate) > 1 && e.call(ENABLED_FLAG, &args![]).bool() {
        let object = e.call(CURRENT_OBJECT, &args![]).u32();
        if e.call(OBJECT_QUALIFIES, &args![object]).bool() {
            b = true;
        }
        let thread = e.call(THREAD_ID, &args![]).u32();
        let process = e.mem.u32(PROCESS_REFERENCE);
        let procedure = e.call(PROCEDURE_INDEX, &args![process]).u32();
        if thread == procedure {
            if !e.call(SECOND_FLAG, &args![]).bool() {
                a = table_entry(e, 0) < 0xb || table_entry(e, 1) < 7;
            }
        } else if e.call(SECOND_FLAG, &args![]).bool() || table_entry(e, 1) < 6 {
            a = true;
        }
        if !a {
            let object = e.call(CURRENT_OBJECT, &args![]).u32();
            let value = e.call(OBJECT_VALUE, &args![object]).u32();
            if value != 0 && thread != value {
                a = true;
            }
        }
    }
    a || b
}

// Translated from 008c7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second flag byte (`011dfa18`).
pub fn fn_008c7bc0(e: &mut Engine) -> u8 {
    e.global::<u8>(SECOND_FLAG_BYTE)
}

// Translated from 008c7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// One fixed sequence of steps: signals slots and stages of the table
/// between calls into the game's subsystems, then records stages 6, 0xb and
/// 7 (`008c7d80`) and runs the semaphore wait (`008c80d0`). The names of the
/// steps are not known.
pub fn fn_008c7bd0(e: &mut Engine) {
    signal(e, 0, 0);
    step_process_reference(e);
    signal(e, 0, 1);
    step_current_object(e, STEP_0087A6B0);
    let byte = e.call(STEP_005C42D0, &args![]).u8();
    let object = e.mem.u32(OBJECT_A);
    e.call(STEP_00991500, &args![object, byte as u32]);
    e.call(STEP_0040FBF0, &args![OBJECT_B, 0u32]);
    e.call(STEP_0096CCA0, &args![OBJECT_C]);
    e.call(STEP_0096CDA0, &args![OBJECT_C]);
    e.call(STEP_0040FBA0, &args![OBJECT_B]);
    let object = e.mem.u32(OBJECT_D);
    e.call(STEP_00455640, &args![object]);
    e.call(STEP_0040FBF0, &args![OBJECT_B, 0u32]);
    e.call(STEP_009784C0, &args![OBJECT_C]);
    e.call(STEP_0040FBA0, &args![OBJECT_B]);
    let object = e.mem.u32(OBJECT_A);
    e.call(STEP_00991DC0, &args![object]);
    e.call(STEP_0040FBF0, &args![OBJECT_B, 0u32]);
    e.call(STEP_0096BCD0, &args![OBJECT_C, 0.0f32, 0u32]);
    e.call(STEP_0096D520, &args![]);
    e.call(STEP_0096C330, &args![OBJECT_C]);
    e.call(STEP_0096DB30, &args![OBJECT_C, 0.0f32, 0u32, 0u32]);
    e.call(PRINT_LISTS, &args![OBJECT_C, 0.0f32, 0u32]);
    e.call(STEP_0096C7C0, &args![OBJECT_C]);
    e.call(STEP_0096CB50, &args![OBJECT_C]);
    e.call(STEP_004772F0, &args![]);
    e.call(STEP_0040FBA0, &args![OBJECT_B]);
    step_float(e);
    set_stage(e, 1, 6);
    fn_008c80d0(e);
    optional_step(e);
    step_current_object(e, STEP_0087A6D0);
    e.call(STEP_005AE270, &args![]);
    e.call(STEP_005A9D60, &args![]);
    set_stage(e, 0, 0xb);
    set_stage(e, 1, 7);
}

// Translated from 008c7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records `stage` as the slot's current stage (`+0x0c`), without any
/// check.
pub fn fn_008c7d80(e: &mut Engine, this: u32, slot: i32, stage: i32) {
    e.mem.set_i32(indexed(this, slot, 0x0c), stage);
}

// Translated from 008c7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first of the two companion sequences: the same steps as `008c7bd0`
/// in another order, signalling slot 0 after each stage and waiting on slot
/// 1's events (stages 1, 3 and 7) instead of recording them. The names of
/// the steps are not known.
pub fn fn_008c7da0(e: &mut Engine) {
    signal(e, 0, 0);
    step_process_reference(e);
    signal(e, 0, 1);
    step_current_object(e, STEP_0087A6B0);
    let byte = e.call(STEP_005C42D0, &args![]).u8();
    let object = e.mem.u32(OBJECT_A);
    e.call(STEP_00991500, &args![object, byte as u32]);
    e.call(STEP_0096CCA0, &args![OBJECT_C]);
    e.call(STEP_0096CDA0, &args![OBJECT_C]);
    signal(e, 0, 2);
    let object = e.mem.u32(OBJECT_D);
    e.call(STEP_00455640, &args![object]);
    signal(e, 0, 3);
    let object = e.mem.u32(OBJECT_A);
    e.call(STEP_00991DC0, &args![object]);
    signal(e, 0, 4);
    e.call(STEP_009784C0, &args![OBJECT_C]);
    signal(e, 0, 5);
    e.call(STEP_0096BCD0, &args![OBJECT_C, 0.0f32, 0u32]);
    signal(e, 0, 6);
    e.call(STEP_0096D520, &args![]);
    signal(e, 0, 0xa);
    wait(e, 1, 1);
    wait(e, 1, 3);
    e.call(STEP_0096C7C0, &args![OBJECT_C]);
    signal(e, 0, 7);
    step_current_object(e, STEP_0087A6D0);
    signal(e, 0, 8);
    wait(e, 1, 7);
    fn_008c80d0(e);
    signal(e, 0, 9);
    optional_step(e);
    e.call(STEP_005AE270, &args![]);
    e.call(STEP_005A9D60, &args![]);
    signal(e, 0, 0xb);
}

// Translated from 008c7f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second companion sequence: signals slot 1 and waits on slot 0's
/// events between steps. The names of the steps are not known.
pub fn fn_008c7f50(e: &mut Engine) {
    signal(e, 1, 0);
    wait(e, 0, 1);
    step_current_object(e, STEP_0087A6B0);
    e.call(STEP_0096C330, &args![OBJECT_C]);
    signal(e, 1, 1);
    signal(e, 1, 2);
    wait(e, 0, 2);
    e.call(STEP_0096DB30, &args![OBJECT_C, 0.0f32, 0u32, 0u32]);
    signal(e, 1, 3);
    e.call(PRINT_LISTS, &args![OBJECT_C, 0.0f32, 0u32]);
    e.call(STEP_0096CB50, &args![OBJECT_C]);
    signal(e, 1, 4);
    e.call(STEP_004772F0, &args![]);
    signal(e, 1, 5);
    wait(e, 0, 4);
    step_float(e);
    signal(e, 1, 6);
    step_current_object(e, STEP_0087A6D0);
    wait(e, 0, 8);
    signal(e, 1, 7);
    e.call(STEP_005AE270, &args![]);
    e.call(STEP_005A9D60, &args![]);
    wait(e, 0, 9);
    wait(e, 0, 0xb);
}

// Translated from 008c80b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the semaphore object (`004424e0`).
pub fn fn_008c80b0(e: &mut Engine) {
    e.call(SEMAPHORE_WAIT, &args![SEMAPHORE_OBJECT]);
}

// Translated from 008c80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the semaphore object (`00442550`).
pub fn fn_008c80c0(e: &mut Engine) {
    e.call(SEMAPHORE_RELEASE, &args![SEMAPHORE_OBJECT]);
}

// Translated from 008c80d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `008c80b0` (the semaphore wait).
pub fn fn_008c80d0(e: &mut Engine) {
    fn_008c80b0(e);
}

// Translated from 008c80e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the second flag byte (`011dfa18`); when it was set and `value` is
/// zero, the byte is cleared first and the semaphore released
/// (`008c80c0`).
pub fn fn_008c80e0(e: &mut Engine, value: u8) {
    if e.global::<u8>(SECOND_FLAG_BYTE) != 0 && value == 0 {
        e.set_global::<u8>(SECOND_FLAG_BYTE, value);
        fn_008c80c0(e);
    }
    e.set_global::<u8>(SECOND_FLAG_BYTE, value);
}

// Translated from 008c8110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the three 12-byte members at `+0x18` with `008c7270`
/// (`__ehvec_dtor`).
pub fn fn_008c8110(e: &mut Engine, this: u32) {
    e.call(
        VECTOR_DESTRUCT,
        &args![this.wrapping_add(0x18), 0xcu32, 3u32, MEMBER_DESTRUCTOR],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c7790, fn_008c7790(u32, u32) -> u32),
        entry!(0x008c77c0, fn_008c77c0(u32)),
        entry!(0x008c7830, fn_008c7830(u32, u32) -> u32),
        entry!(0x008c7860, fn_008c7860(u32)),
        entry!(0x008c78c0, fn_008c78c0(u32)),
        entry!(0x008c7990, fn_008c7990(u32)),
        entry!(0x008c79e0, fn_008c79e0(u32, i32, i32)),
        entry!(0x008c7a30, fn_008c7a30(u32, i32) -> u32),
        entry!(0x008c7a70, fn_008c7a70(u32, i32, i32)),
        entry!(0x008c7aa0, fn_008c7aa0() -> bool),
        entry!(0x008c7bc0, fn_008c7bc0() -> u8),
        entry!(0x008c7bd0, fn_008c7bd0()),
        entry!(0x008c7d80, fn_008c7d80(u32, i32, i32)),
        entry!(0x008c7da0, fn_008c7da0()),
        entry!(0x008c7f50, fn_008c7f50()),
        entry!(0x008c80b0, fn_008c80b0()),
        entry!(0x008c80c0, fn_008c80c0()),
        entry!(0x008c80d0, fn_008c80d0()),
        entry!(0x008c80e0, fn_008c80e0(u8)),
        entry!(0x008c8110, fn_008c8110(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the callees answer.
    #[derive(Clone, Copy)]
    struct Setup {
        gate: i32,
        enabled: bool,
        qualifies: bool,
        thread: u32,
        procedure: u32,
        second_flag: bool,
        entries: [u32; 2],
        object_value: u32,
    }

    /// Everything off: the result is false and the thread (5) differs from
    /// the procedure (6), with large table entries.
    const QUIET: Setup = Setup {
        gate: 2,
        enabled: true,
        qualifies: false,
        thread: 5,
        procedure: 6,
        second_flag: false,
        entries: [100, 100],
        object_value: 0,
    };

    fn run(s: Setup) -> bool {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let gate = e.mem.alloc(4);
        e.mem.set_i32(gate, s.gate);
        e.register_double(VALUE_WORD_ADDRESS, move |_, a| {
            assert_eq!(a[0], GATE_OBJECT);
            Ret {
                eax: gate,
                ..Ret::default()
            }
        });
        e.register_double(ENABLED_FLAG, move |_, _| (s.enabled as u32).into_ret_u32());
        e.register_double(CURRENT_OBJECT, |_, _| 0x7777u32.into_ret_u32());
        e.register_double(OBJECT_QUALIFIES, move |_, a| {
            assert_eq!(a[0], 0x7777);
            (s.qualifies as u32).into_ret_u32()
        });
        e.register_double(THREAD_ID, move |_, _| s.thread.into_ret_u32());
        e.mem.set_u32(PROCESS_REFERENCE, 0x8888);
        e.register_double(PROCEDURE_INDEX, move |_, a| {
            assert_eq!(a[0], 0x8888);
            s.procedure.into_ret_u32()
        });
        e.register_double(SECOND_FLAG, move |_, _| {
            (s.second_flag as u32).into_ret_u32()
        });
        e.register_double(TABLE, |_, _| 0x9999u32.into_ret_u32());
        e.register_double(TABLE_LOOKUP, move |_, a| {
            assert_eq!(a[0], 0x9999);
            s.entries[a[1] as usize].into_ret_u32()
        });
        e.register_double(OBJECT_VALUE, move |_, a| {
            assert_eq!(a[0], 0x7777);
            s.object_value.into_ret_u32()
        });
        e.call(0x008c_7aa0, &args![]).bool()
    }

    trait IntoRetU32 {
        fn into_ret_u32(self) -> Ret;
    }
    impl IntoRetU32 for u32 {
        fn into_ret_u32(self) -> Ret {
            Ret {
                eax: self,
                ..Ret::default()
            }
        }
    }

    #[test]
    fn nothing_happens_without_the_gate() {
        assert!(!run(Setup {
            gate: 1,
            qualifies: true,
            ..QUIET
        }));
        assert!(!run(Setup {
            enabled: false,
            qualifies: true,
            ..QUIET
        }));
    }

    #[test]
    fn a_qualifying_object_is_enough() {
        assert!(!run(QUIET));
        assert!(run(Setup {
            qualifies: true,
            ..QUIET
        }));
    }

    #[test]
    fn same_thread_and_procedure_checks_both_table_entries() {
        let same = Setup {
            procedure: 5,
            ..QUIET
        };
        assert!(!run(same));
        assert!(run(Setup {
            entries: [10, 100],
            ..same
        }));
        assert!(!run(Setup {
            entries: [11, 7],
            ..same
        }));
        assert!(run(Setup {
            entries: [11, 6],
            ..same
        }));
        // The second flag skips the table check altogether.
        assert!(!run(Setup {
            second_flag: true,
            entries: [0, 0],
            ..same
        }));
    }

    #[test]
    fn different_procedure_checks_the_flag_or_the_second_entry() {
        assert!(run(Setup {
            second_flag: true,
            ..QUIET
        }));
        assert!(run(Setup {
            entries: [100, 5],
            ..QUIET
        }));
        assert!(!run(Setup {
            entries: [0, 6],
            ..QUIET
        }));
    }

    #[test]
    fn a_foreign_owner_of_the_object_value_counts() {
        assert!(run(Setup {
            object_value: 9,
            ..QUIET
        }));
        // Owned by this very thread: no.
        assert!(!run(Setup {
            object_value: 5,
            ..QUIET
        }));
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Registers do-nothing doubles over `addrs`.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn log(e: &Engine) -> Log {
        e.call_log.clone().unwrap()
    }

    /// An object with workers in both slots, `12 + 8` event handles
    /// (`0x100 + i`, `0x200 + i`) and per-slot event arrays whose handle
    /// for stage `k` of slot `s` is `0x1000 + s * 0x100 + k`.
    fn object(e: &mut Engine) -> u32 {
        e.map(0x011d_e000, 0x3000);
        e.map(0x011f_1000, 0x1000);
        let this = e.mem.alloc(0xb0);
        for i in 0..12 {
            e.mem.set_u32(this + 0x3c + i * 4, 0x100 + i);
        }
        for i in 0..8 {
            e.mem.set_u32(this + 0x6c + i * 4, 0x200 + i);
        }
        e.mem.set_u32(this, 0xaaa0);
        e.mem.set_u32(this + 4, 0xaaa4);
        for slot in 0..2 {
            let events = e.mem.alloc(0x40);
            for stage in 0..16 {
                e.mem
                    .set_u32(events + stage * 4, 0x1000 + slot * 0x100 + stage);
            }
            e.mem.set_u32(this + 0x8c + slot * 4, events);
        }
        this
    }

    #[test]
    fn deleting_destructor_closes_members_then_deletes_on_bit_zero() {
        for flags in [0u32, 1, 2, 3] {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[CLOSE_HANDLE_MEMBER, DESTROY_BASE, OPERATOR_DELETE],
            );
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x008c_7790, &args![0x1000u32, flags]).u32(), 0x1000);
            let mut expected: Log = vec![
                (0x008c_7790, vec![0x1000, flags]),
                (CLOSE_HANDLE_MEMBER, vec![0x103c]),
                (CLOSE_HANDLE_MEMBER, vec![0x1030]),
                (DESTROY_BASE, vec![0x1000]),
            ];
            if flags & 1 != 0 {
                expected.push((OPERATOR_DELETE, vec![0x1000]));
            }
            assert_eq!(log(&e), expected);
        }
    }

    #[test]
    fn destructor_body_closes_the_two_members_and_the_base() {
        let mut e = Engine::new();
        quiet(&mut e, &[CLOSE_HANDLE_MEMBER, DESTROY_BASE]);
        e.call_log = Some(vec![]);
        e.call(0x008c_77c0, &args![0x2000u32]);
        assert_eq!(
            log(&e),
            vec![
                (0x008c_77c0, vec![0x2000]),
                (CLOSE_HANDLE_MEMBER, vec![0x203c]),
                (CLOSE_HANDLE_MEMBER, vec![0x2030]),
                (DESTROY_BASE, vec![0x2000]),
            ]
        );
    }

    #[test]
    fn second_deleting_destructor_destroys_its_member_first() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[
                    DESTROY_MEMBER,
                    CLOSE_HANDLE_MEMBER,
                    DESTROY_BASE,
                    OPERATOR_DELETE,
                ],
            );
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x008c_7830, &args![0x1000u32, flags]).u32(), 0x1000);
            let mut expected: Log = vec![
                (0x008c_7830, vec![0x1000, flags]),
                (DESTROY_MEMBER, vec![0x1050]),
                (CLOSE_HANDLE_MEMBER, vec![0x103c]),
                (CLOSE_HANDLE_MEMBER, vec![0x1030]),
                (DESTROY_BASE, vec![0x1000]),
            ];
            if flags & 1 != 0 {
                expected.push((OPERATOR_DELETE, vec![0x1000]));
            }
            assert_eq!(log(&e), expected);
        }
    }

    #[test]
    fn second_destructor_body_destroys_the_member_then_runs_the_first() {
        let mut e = Engine::new();
        quiet(&mut e, &[DESTROY_MEMBER, CLOSE_HANDLE_MEMBER, DESTROY_BASE]);
        e.call_log = Some(vec![]);
        e.call(0x008c_7860, &args![0x3000u32]);
        let calls = log(&e);
        assert_eq!(calls[1], (DESTROY_MEMBER, vec![0x3050]));
        assert_eq!(calls[4], (DESTROY_BASE, vec![0x3000]));
        assert_eq!(calls.len(), 5);
    }

    #[test]
    fn reset_clears_events_and_stages_and_releases_the_workers() {
        let mut e = Engine::new();
        let this = object(&mut e);
        quiet(&mut e, &[RESET_EVENT, WORKER_RELEASE]);
        e.mem.set_u32(this + 0x0c, 5);
        e.mem.set_u32(this + 0x10, 6);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x008c_78c0, &args![this]);
        let calls = log(&e);
        let resets: Vec<u32> = calls
            .iter()
            .filter(|c| c.0 == RESET_EVENT)
            .map(|c| c.1[0])
            .collect();
        let expected: Vec<u32> = (0..12)
            .map(|i| 0x100 + i)
            .chain((0..8).map(|i| 0x200 + i))
            .collect();
        assert_eq!(resets, expected);
        assert_eq!((e.mem.u32(this + 0x0c), e.mem.u32(this + 0x10)), (0, 0));
        assert_eq!(e.global::<u8>(WORKERS_RUNNING), 1);
        // Only slot 0 has a worker.
        assert_eq!(calls.last().unwrap(), &(WORKER_RELEASE, vec![0xaaa0]));
        assert_eq!(calls.iter().filter(|c| c.0 == WORKER_RELEASE).count(), 1);
    }

    #[test]
    fn stop_waits_for_the_workers_and_clears_the_running_byte() {
        let mut e = Engine::new();
        let this = object(&mut e);
        quiet(&mut e, &[WORKER_WAIT]);
        e.set_global::<u8>(WORKERS_RUNNING, 1);
        e.call_log = Some(vec![]);
        e.call(0x008c_7990, &args![this]);
        assert_eq!(
            log(&e),
            vec![
                (0x008c_7990, vec![this]),
                (WORKER_WAIT, vec![0xaaa0]),
                (WORKER_WAIT, vec![0xaaa4]),
            ]
        );
        assert_eq!(e.global::<u8>(WORKERS_RUNNING), 0);
    }

    #[test]
    fn signal_sets_the_stage_event_of_a_slot_with_a_worker() {
        let mut e = Engine::new();
        let this = object(&mut e);
        quiet(&mut e, &[SET_EVENT]);
        e.call_log = Some(vec![]);
        e.call(0x008c_79e0, &args![this, 1u32, 5u32]);
        assert_eq!(log(&e)[1], (SET_EVENT, vec![0x1105]));
        assert_eq!(e.mem.u32(this + 0x10), 5);
        // No worker in the slot: nothing happens.
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x008c_79e0, &args![this, 0u32, 3u32]);
        assert_eq!(log(&e).len(), 1);
        assert_eq!(e.mem.u32(this + 0x0c), 0);
        // Slot 2 and above: nothing happens either.
        e.call_log = Some(vec![]);
        e.call(0x008c_79e0, &args![this, 2u32, 3u32]);
        assert_eq!(log(&e).len(), 1);
    }

    #[test]
    fn stage_value_adds_the_member_word_for_slots_below_two() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.register(MEMBER_FIRST_WORD, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x10, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_7a30, &args![this, 1u32]).u32(), 12);
        assert_eq!(log(&e)[1], (MEMBER_FIRST_WORD, vec![this + 0x18 + 12]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_7a30, &args![this, 2u32]).u32(), 0);
        assert_eq!(log(&e).len(), 1);
    }

    #[test]
    fn wait_has_no_timeout() {
        let mut e = Engine::new();
        let this = object(&mut e);
        quiet(&mut e, &[WAIT_FOR_SINGLE_OBJECT]);
        e.call_log = Some(vec![]);
        e.call(0x008c_7a70, &args![this, 1u32, 7u32]);
        assert_eq!(
            log(&e)[1],
            (WAIT_FOR_SINGLE_OBJECT, vec![0x1107, 0xffff_ffff])
        );
    }

    #[test]
    fn second_flag_and_stage_setter() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.set_global::<u8>(SECOND_FLAG_BYTE, 3);
        assert_eq!(e.call(0x008c_7bc0, &args![]).u8(), 3);
        e.call(0x008c_7d80, &args![this, 1u32, 9u32]);
        assert_eq!(e.mem.u32(this + 0x10), 9);
    }

    #[test]
    fn semaphore_wrappers_use_the_semaphore_object() {
        let mut e = Engine::new();
        quiet(&mut e, &[SEMAPHORE_WAIT, SEMAPHORE_RELEASE]);
        e.call_log = Some(vec![]);
        e.call(0x008c_80b0, &args![]);
        e.call(0x008c_80c0, &args![]);
        e.call(0x008c_80d0, &args![]);
        assert_eq!(
            log(&e),
            vec![
                (0x008c_80b0, vec![]),
                (SEMAPHORE_WAIT, vec![SEMAPHORE_OBJECT]),
                (0x008c_80c0, vec![]),
                (SEMAPHORE_RELEASE, vec![SEMAPHORE_OBJECT]),
                (0x008c_80d0, vec![]),
                (SEMAPHORE_WAIT, vec![SEMAPHORE_OBJECT]),
            ]
        );
    }

    /// Runs `008c80e0(value)` with the flag at `before`; returns the flag
    /// afterwards and whether the semaphore was released.
    fn set_flag(before: u8, value: u8) -> (u8, bool) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x3000);
        quiet(&mut e, &[SEMAPHORE_RELEASE]);
        e.set_global::<u8>(SECOND_FLAG_BYTE, before);
        e.call_log = Some(vec![]);
        e.call(0x008c_80e0, &args![value]);
        let released = log(&e).iter().any(|c| c.0 == SEMAPHORE_RELEASE);
        (e.global::<u8>(SECOND_FLAG_BYTE), released)
    }

    #[test]
    fn clearing_a_set_flag_releases_the_semaphore() {
        assert_eq!(set_flag(1, 0), (0, true));
        assert_eq!(set_flag(1, 4), (4, false));
        assert_eq!(set_flag(0, 0), (0, false));
        assert_eq!(set_flag(0, 5), (5, false));
    }

    #[test]
    fn vector_destructor_covers_the_three_members() {
        let mut e = Engine::new();
        quiet(&mut e, &[VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x008c_8110, &args![0x4000u32]);
        assert_eq!(
            log(&e)[1],
            (VECTOR_DESTRUCT, vec![0x4018, 0xc, 3, 0x008c_7270])
        );
    }

    // The three stage sequences. The expected call order below was read
    // from the disassembly's `CALL` instructions: `SIG` is `008c79e0`
    // (which sets an event), `WAIT` is `008c7a70` (a wait), `SET` is
    // `008c7d80` (no call), `SEMWAIT` is `008c80d0` (the semaphore wait).
    const SEQUENCE_7BD0: &str = "00713d80 SIG 0086fd70 00713d80 SIG 004537b0 0087a6b0 005c42d0 00991500 0040fbf0 0096cca0 0096cda0 0040fba0 00455640 0040fbf0 009784c0 0040fba0 00991dc0 0040fbf0 0096bcd0 0096d520 0096c330 0096db30 008d0600 0096c7c0 0096cb50 004772f0 0040fba0 00453850 00453550 00713d80 SET SEMWAIT 00408d60 006c0720 006c3640 004537b0 0087a6d0 005ae270 005a9d60 00713d80 SET 00713d80 SET";
    const SEQUENCE_7DA0: &str = "00713d80 SIG 0086fd70 00713d80 SIG 004537b0 0087a6b0 005c42d0 00991500 0096cca0 0096cda0 00713d80 SIG 00455640 00713d80 SIG 00991dc0 00713d80 SIG 009784c0 00713d80 SIG 0096bcd0 00713d80 SIG 0096d520 00713d80 SIG 00713d80 WAIT 00713d80 WAIT 0096c7c0 00713d80 SIG 004537b0 0087a6d0 00713d80 SIG 00713d80 WAIT SEMWAIT 00713d80 SIG 00408d60 006c0720 006c3640 005ae270 005a9d60 00713d80 SIG";
    const SEQUENCE_7F50: &str = "00713d80 SIG 00713d80 WAIT 004537b0 0087a6b0 0096c330 00713d80 SIG 00713d80 SIG 00713d80 WAIT 0096db30 00713d80 SIG 008d0600 0096cb50 00713d80 SIG 004772f0 00713d80 SIG 00713d80 WAIT 00453850 00453550 00713d80 SIG 004537b0 0087a6d0 00713d80 WAIT 00713d80 SIG 005ae270 005a9d60 00713d80 WAIT 00713d80 WAIT";

    /// The addresses a sequence calls, in order, with the flag byte of
    /// `00408d60` deciding whether `006c0720` and `006c3640` run.
    fn expected_addresses(sequence: &str, optional: bool) -> Vec<u32> {
        let mut out = Vec::new();
        for token in sequence.split(' ') {
            match token {
                "SIG" => out.push(SET_EVENT),
                "WAIT" => out.push(WAIT_FOR_SINGLE_OBJECT),
                "SET" => {}
                "SEMWAIT" => out.push(SEMAPHORE_WAIT),
                hex => {
                    let addr = u32::from_str_radix(hex, 16).unwrap();
                    if optional || (addr != STEP_006C0720 && addr != STEP_006C3640) {
                        out.push(addr);
                    }
                }
            }
        }
        out
    }

    /// Runs the sequence at `entry` and returns the engine, the object and
    /// the log.
    fn run_sequence(entry: u32, sequence: &str, optional: bool) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        let this = object(&mut e);
        let mut addrs = expected_addresses(sequence, true);
        addrs.sort_unstable();
        addrs.dedup();
        quiet(&mut e, &addrs);
        e.register_double(TABLE, move |_, _| Ret {
            eax: this,
            ..Ret::default()
        });
        e.register(CURRENT_OBJECT, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register(STEP_005C42D0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(STEP_00453850, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let flag = e.mem.alloc(1);
        e.mem.set_u8(flag, optional as u8);
        e.register_double(STEP_00408D60, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        e.register(STEP_006C0720, |_, _| Ret {
            eax: 0x6c00,
            ..Ret::default()
        });
        e.mem.set_u32(PROCESS_REFERENCE, 0x8888);
        e.mem.set_u32(OBJECT_A, 0xa000);
        e.mem.set_u32(OBJECT_D, 0xd000);
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let calls = log(&e);
        (e, this, calls)
    }

    fn addresses(calls: &Log) -> Vec<u32> {
        calls.iter().skip(1).map(|c| c.0).collect()
    }

    fn find(calls: &Log, addr: u32) -> Vec<u32> {
        calls.iter().find(|c| c.0 == addr).unwrap().1.clone()
    }

    #[test]
    fn first_sequence_calls_in_the_disassembly_order() {
        for optional in [true, false] {
            let (e, this, calls) = run_sequence(0x008c_7bd0, SEQUENCE_7BD0, optional);
            assert_eq!(
                addresses(&calls),
                expected_addresses(SEQUENCE_7BD0, optional)
            );
            assert_eq!(find(&calls, STEP_0086FD70), vec![0x8888]);
            assert_eq!(find(&calls, STEP_0087A6B0), vec![0x7777]);
            assert_eq!(find(&calls, STEP_00991500), vec![0xa000, 1]);
            assert_eq!(find(&calls, STEP_0040FBF0), vec![OBJECT_B, 0]);
            assert_eq!(find(&calls, STEP_0096BCD0), vec![OBJECT_C, 0, 0]);
            assert_eq!(find(&calls, STEP_0096DB30), vec![OBJECT_C, 0, 0, 0]);
            assert_eq!(find(&calls, PRINT_LISTS), vec![OBJECT_C, 0, 0]);
            assert_eq!(find(&calls, STEP_00453550), vec![0xd000, 2.5f32.to_bits()]);
            assert_eq!(find(&calls, SEMAPHORE_WAIT), vec![SEMAPHORE_OBJECT]);
            if optional {
                assert_eq!(find(&calls, STEP_006C3640), vec![0x6c00]);
            }
            // Slot 0 was signalled for stages 0 and 1; the stage words are
            // then set to 0xb and 7.
            assert_eq!(e.mem.u32(this + 0x0c), 0xb);
            assert_eq!(e.mem.u32(this + 0x10), 7);
            let events: Vec<u32> = calls
                .iter()
                .filter(|c| c.0 == SET_EVENT)
                .map(|c| c.1[0])
                .collect();
            assert_eq!(events, vec![0x1000, 0x1001]);
        }
    }

    #[test]
    fn second_sequence_signals_slot_zero_and_waits_on_slot_one() {
        for optional in [true, false] {
            let (e, this, calls) = run_sequence(0x008c_7da0, SEQUENCE_7DA0, optional);
            assert_eq!(
                addresses(&calls),
                expected_addresses(SEQUENCE_7DA0, optional)
            );
            let events: Vec<u32> = calls
                .iter()
                .filter(|c| c.0 == SET_EVENT)
                .map(|c| c.1[0])
                .collect();
            assert_eq!(
                events,
                vec![0x1000, 0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x100a]
                    .into_iter()
                    .chain([0x1007, 0x1008, 0x1009, 0x100b])
                    .collect::<Vec<u32>>()
            );
            let waits: Vec<u32> = calls
                .iter()
                .filter(|c| c.0 == WAIT_FOR_SINGLE_OBJECT)
                .map(|c| c.1[0])
                .collect();
            assert_eq!(waits, vec![0x1101, 0x1103, 0x1107]);
            assert_eq!(find(&calls, STEP_0096BCD0), vec![OBJECT_C, 0, 0]);
            assert_eq!(e.mem.u32(this + 0x0c), 0xb);
        }
    }

    #[test]
    fn third_sequence_signals_slot_one_and_waits_on_slot_zero() {
        for optional in [true, false] {
            let (e, this, calls) = run_sequence(0x008c_7f50, SEQUENCE_7F50, optional);
            assert_eq!(
                addresses(&calls),
                expected_addresses(SEQUENCE_7F50, optional)
            );
            let events: Vec<u32> = calls
                .iter()
                .filter(|c| c.0 == SET_EVENT)
                .map(|c| c.1[0])
                .collect();
            assert_eq!(
                events,
                vec![0x1100, 0x1101, 0x1102, 0x1103, 0x1104, 0x1105, 0x1106, 0x1107]
            );
            let waits: Vec<u32> = calls
                .iter()
                .filter(|c| c.0 == WAIT_FOR_SINGLE_OBJECT)
                .map(|c| c.1[0])
                .collect();
            assert_eq!(waits, vec![0x1001, 0x1002, 0x1004, 0x1008, 0x1009, 0x100b]);
            assert_eq!(find(&calls, PRINT_LISTS), vec![OBJECT_C, 0, 0]);
            assert_eq!(find(&calls, STEP_00453550), vec![0xd000, 2.5f32.to_bits()]);
            assert_eq!(e.mem.u32(this + 0x10), 7);
        }
    }
}
