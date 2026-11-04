//! `fallout/ai/combat/combatactions.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006815c0` of unit run_006815c0, which answers
/// `this` itself, and returns its result (a pointer). Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. The return
/// type (`Ptr`) is that of `006815c0`'s translated signature `fn_006815c0(Ptr) -> Ptr` (its
/// doc: `mov eax, ecx`).
pub fn fn_0097b940(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]).ptr()
}

// Translated from 0097c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00979c80`, a destructor with an exception frame
/// that stores the vtable `0108cab8` and runs four list-clearing loops, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00979c80`'s disassembly ends with its exception-frame unlink and
/// returns no value (Ghidra types it void).
pub fn fn_0097c470(e: &mut Engine, this: Ptr) {
    e.call(0x0097_9c80, &args![this]);
}

// The `CombatAction` subclasses' constructors below all have one shape (read
// from each one's disassembly): the base constructor with the word the
// factory passes (`00979bd0`, which keeps it at +0x24; or the constructor of
// the subclass they derive from), the class's own vtable (each named by its
// RTTI locator), then pairs of words added to the action's lists: `0097ade0`
// (list +0x04), `00984390` (+0x0C), `0097adc0` (+0x14) and, through
// `0097bc60`, the list at +0x1C (all through `009aa2b0`); `0097c260`,
// `0097c070` and `0097c240` remove a word from the lists at +0x04, +0x0C and
// +0x14 (`009aa340`). What the words mean is not confirmed from the exe.

/// Adds the pair `(a, b)` to the list at +0x1C (`009aa2b0`, `thiscall(a,
/// b)`).
const LIST_ADD: u32 = 0x009a_a2b0;
/// Removes the entry holding `value` from a list (`009aa340`,
/// `thiscall(value)`).
const LIST_REMOVE: u32 = 0x009a_a340;

// Translated from 0097ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at +0xBC of `this` (unnamed in the engine map; the
/// decompiler's library match names it `SchedulerBase::GetIScheduler`,
/// which is only the shared shape).
pub fn fn_0097ae90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xbc)
}

// Translated from 0097bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `(a, b)` to the list at +0x1C (`009aa2b0`; `thiscall`, `RET 8`).
pub fn fn_0097bc60(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(LIST_ADD, &args![this.addr() + 0x1c, a, b]);
}

// Translated from 0097c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x0C (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c070(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0xc, value]);
}

// Translated from 0097c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x14 (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c240(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0x14, value]);
}

// Translated from 0097c260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `value` from the list at +0x04 (`009aa340`; `thiscall`, `RET 4`).
pub fn fn_0097c260(e: &mut Engine, this: Ptr, value: u32) {
    e.call(LIST_REMOVE, &args![this.addr() + 0x4, value]);
}

// Translated from 0097aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097aeb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedFromCover` constructor (by the vtable it sets, `0108cb7c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097aeb0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cb7c);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b0b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedExplosive` constructor (by the vtable it sets, `0108cb94`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b0b0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cb94);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x5u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0x6u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b280 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRangedExplosiveFromCover` constructor (by the vtable it sets, `0108cbac`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b280(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbac);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b350 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackMelee` constructor (by the vtable it sets, `0108cbc4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b350(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbc4);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x12u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x2u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x2u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x1u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b4e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b4e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackHandToHand` constructor (by the vtable it sets, `0108cbdc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b4e0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbdc);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x12u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x6u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x6u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x1u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b620 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenade` constructor (by the vtable it sets, `0108cbf4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b620(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cbf4);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x5u32, 0x1u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b7a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFlushTarget` constructor (by the vtable it sets, `0108cc0c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b7a0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc0c);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x9u32, 0x1u32]);
    this
}

// Translated from 0097b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b9e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFromCover` constructor (by the vtable it sets, `0108cc24`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097b9e0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc24);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0x4u32, 0x1u32]);
    e.call(0x0097_ade0, &args![this, 0x8u32, 0x0u32]);
    e.call(0x0097_ade0, &args![this, 0x7u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0xbu32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0x3u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x2u32]);
    e.call(0x0097_adc0, &args![this, 0x0u32, 0x1u32]);
    this
}

// Translated from 0097bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bbd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMove` constructor (by the vtable it sets, `0108cc3c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097bbd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc3c);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bfe0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndSwitchToMelee` constructor (by the vtable it sets, `0108cc54`): the base
/// constructor `0097bbd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097bfe0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097bbd0(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc54);
    fn_0097c070(e, this, 0xau32);
    e.call(0x0097_ade0, &args![this, 0xcu32, 0x2u32]);
    fn_0097bc60(e, this, 0xbu32, 0x2u32);
    fn_0097bc60(e, this, 0xau32, 0x2u32);
    this
}

// Translated from 0097c170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c170 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackRanged` constructor (by the vtable it sets, `0108cc6c`): the base
/// constructor `0097ad10` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c170(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_ad10, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cc6c);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c260(e, this, 0x12u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c3a0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackRangedExplosive` constructor (by the vtable it sets, `0108cc84`): the base
/// constructor `0097b0b0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c3a0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097b0b0(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc84);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c260(e, this, 0x12u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c560 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackGrenade` constructor (by the vtable it sets, `0108cc9c`): the base
/// constructor `0097b620` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c560(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    fn_0097b620(e, this, kind);
    e.mem.set_u32(this.addr(), 0x0108_cc9c);
    fn_0097c260(e, this, 0x3u32);
    fn_0097c070(e, this, 0x1u32);
    fn_0097c070(e, this, 0x6u32);
    fn_0097c070(e, this, 0x5u32);
    fn_0097c240(e, this, 0x0u32);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0097_ade0, &args![this, 0xfu32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x1u32, u32::MAX]);
    fn_0097bc60(e, this, 0x6u32, 0x0u32);
    this
}

// Translated from 0097c700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c700 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionDrawWeapon` constructor (by the vtable it sets, `0108ccb4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c700(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_ccb4);
    e.call(0x0097_ade0, &args![this, 0xcu32, u32::MAX]);
    e.call(0x0097_ade0, &args![this, 0xau32, 0x7u32]);
    e.call(0x0097_adc0, &args![this, 0xbu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xau32, u32::MAX]);
    this
}

// Translated from 0097c850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c850 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionSwitchWeapon` constructor (by the vtable it sets, `0108cccc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c850(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cccc);
    e.call(0x0097_ade0, &args![this, 0xcu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xbu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0xau32, u32::MAX]);
    this
}

// Translated from 0097c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c9d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAvoidThreat` constructor (by the vtable it sets, `0108cce4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097c9d0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cce4);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xeu32]);
    e.call(0x0097_adc0, &args![this, 0xeu32, 0x0u32]);
    this
}

// Translated from 0097ca60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ca60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionSearch` constructor (by the vtable it sets, `0108ccfc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097ca60(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_ccfc);
    e.call(0x0097_ade0, &args![this, 0x12u32, 0x11u32]);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0097_adc0, &args![this, 0x3u32, 0x0u32]);
    this
}

// Translated from 0097cb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cb70 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionInvestigate` constructor (by the vtable it sets, `0108cd14`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cb70(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd14);
    e.call(0x0098_4390, &args![this, 0xau32, 0xffff_0018u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xfu32]);
    e.call(0x0097_adc0, &args![this, 0x3u32, 0x0u32]);
    this
}

// Translated from 0097cc00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cc00 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionDodge` constructor (by the vtable it sets, `0108cd2c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cc00(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd2c);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x3u32]);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    this
}

// Translated from 0097cc80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cc80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionIgnoreBlockedTarget` constructor (by the vtable it sets, `0108cd44`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cc80(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd44);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    this
}

// Translated from 0097cd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cd30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionFlee` constructor (by the vtable it sets, `0108cd5c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cd30(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd5c);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xcu32]);
    e.call(0x0097_adc0, &args![this, 0xfu32, 0x0u32]);
    this
}

// Translated from 0097cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cfe0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionUseCombatItem` constructor (by the vtable it sets, `0108cd8c`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097cfe0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cd8c);
    e.call(0x0098_4390, &args![this, 0xdu32, u32::MAX]);
    e.call(0x0097_adc0, &args![this, 0x10u32, u32::MAX]);
    this
}

// Translated from 0097d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d1c0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAcquireLineOfSight` constructor (by the vtable it sets, `0108cdbc`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d1c0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cdbc);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xdu32]);
    e.call(0x0097_adc0, &args![this, 0x5u32, 0x1u32]);
    this
}

// Translated from 0097d250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d250 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionHide` constructor (by the vtable it sets, `0108cdd4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d250(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cdd4);
    e.call(0x0098_4390, &args![this, 0x1u32, 0xau32]);
    e.call(0x0097_adc0, &args![this, 0x11u32, u32::MAX]);
    this
}

// Translated from 0097d130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d130 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionApproachTarget` constructor (by the vtable it sets, `0108cda4`): the base
/// constructor `00979bd0` with `kind`, the vtable, then the list entries below
/// (see the note above `LIST_ADD`). Returns `this`.
pub fn fn_0097d130(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(0x0097_9bd0, &args![this, kind]);
    e.mem.set_u32(this.addr(), 0x0108_cda4);
    e.call(0x0097_ade0, &args![this, 0x3u32, 0x0u32]);
    e.call(0x0098_4390, &args![this, 0x1u32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x6u32, 0x0u32]);
    e.call(0x0097_adc0, &args![this, 0x5u32, 0x1u32]);
    this
}

// The actions' `GetCost`, `CanExecute`, `Execute` and `GetDebugText`
// overrides, and the small accessors they use. `controller` is the word the
// callers pass first (its +0x9C object, `00586150`, is the one the getters
// below read; `CombatController`'s procedure setters take it as `this`).

/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `CombatController::SetActionProcedure`, `SetMovementProcedure` and
/// `AddBackgroundProcedure` (engine map; `00980110`, `009801b0`,
/// `00980270`, each `thiscall(procedure)`).
const SET_ACTION_PROCEDURE: u32 = 0x0098_0110;
const SET_MOVEMENT_PROCEDURE: u32 = 0x0098_01b0;
const ADD_BACKGROUND_PROCEDURE: u32 = 0x0098_0270;
/// The controller's +0x9C object (`00586150`, `thiscall`).
const CONTROLLER_STATE: u32 = 0x0058_6150;
/// `00778970` (`thiscall`) on that object: the object the move actions read.
const STATE_MOVE_TARGET: u32 = 0x0077_8970;
/// `CombatAction::GetCost`, `GetAttackCost`, `CanExecute` and `GetName`
/// (engine map; `00979e40`, `0097ac30`, `00979e90`, `00979e20`).
const BASE_GET_COST: u32 = 0x0097_9e40;
const GET_ATTACK_COST: u32 = 0x0097_ac30;
const BASE_CAN_EXECUTE: u32 = 0x0097_9e90;
const GET_NAME: u32 = 0x0097_9e20;
/// `CombatState::FindCombatLocation` (engine map; `0099ac20`,
/// `thiscall(word)`).
const FIND_COMBAT_LOCATION: u32 = 0x0099_ac20;
/// `FLT_MAX` (`0108cb10`): the cost of an action that cannot be taken.
const NO_COST: u32 = 0x0108_cb10;
/// `1.0` (`double`, `01012070`) and `"CombatStyleMelee"` (`0108cb64`), the
/// argument of `008c1740` (`thiscall(text)`, ST0) the ranged costs compare
/// with 1.
const ONE: u32 = 0x0101_2070;
const MELEE_STYLE_TEXT: u32 = 0x0108_cb64;
const STYLE_VALUE: u32 = 0x008c_1740;
/// `MiddleHighProcess::GetForceNextUpdate` (engine map; `00566950`) and
/// `Actor::GetCurrentWeapon` (engine map; `008a1710`), both `thiscall` on
/// the word at +0xBC of the controller ([`fn_0097ae90`]).
const FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
const CURRENT_WEAPON: u32 = 0x008a_1710;
/// `006450c0` (`thiscall`, `bool`) on a weapon of the controller's list.
const WEAPON_TEST: u32 = 0x0064_50c0;
/// `00406d00` (`cdecl(buffer, size, format, ...)`) and the formats
/// `"%s: %s"` (`01012600`) and `"%s: %i"` (`0108cde8`).
const PRINT_TEXT: u32 = 0x0040_6d00;
const NAME_AND_TEXT: u32 = 0x0101_2600;
const NAME_AND_NUMBER: u32 = 0x0108_cde8;
/// The string tables the debug texts index: `011a5700` (by the low 16 bits
/// of the word) and `0118a8a8` (by the word).
const MOVE_TEXTS: u32 = 0x011a_5700;
const WEAPON_TEXTS: u32 = 0x0118_a8a8;

/// The procedure constructors the `Execute`s call on a fresh block (each
/// `thiscall`, named by the engine map where it names them): `009d0890`
/// (0x48 bytes), `009d2440` (`CombatProcedureBeInCover`, 0x6C, one word),
/// `009d3a00` (`CombatProcedureEngageTarget`, 0x64), `009cc0c0` (0x88),
/// `009cadf0` (`CombatProcedureAttackGrenade`, 0x3C), `009caee0` (0x3C, two
/// words), `009d5ad0` (0x18, one word), `009d69f0` (0x7C, three words),
/// `009da720` (`CombatProcedureSwitchWeapon`, 0x18, one word), `009d8b60`
/// (`CombatProcedureSearch`, 0x40), `009d6030`
/// (`CombatProcedureHideFromTarget`, 0x40, four words), `009ca5f0`
/// (`CombatProcedureActivateObject`, 0x1C, two words), `009daa10`
/// (`CombatProcedureUseCombatItem`, 0x18, one word).
const PROCEDURE_009D0890: u32 = 0x009d_0890;
const BE_IN_COVER: u32 = 0x009d_2440;
const ENGAGE_TARGET: u32 = 0x009d_3a00;
const PROCEDURE_009CC0C0: u32 = 0x009c_c0c0;
const ATTACK_GRENADE: u32 = 0x009c_adf0;
const PROCEDURE_009CAEE0: u32 = 0x009c_aee0;
const PROCEDURE_009D5AD0: u32 = 0x009d_5ad0;
const PROCEDURE_009D69F0: u32 = 0x009d_69f0;
const SWITCH_WEAPON: u32 = 0x009d_a720;
const SEARCH: u32 = 0x009d_8b60;
const HIDE_FROM_TARGET: u32 = 0x009d_6030;
const ACTIVATE_OBJECT: u32 = 0x009c_a5f0;
const USE_COMBAT_ITEM: u32 = 0x009d_aa10;

/// A procedure: `size` bytes from `operator new`, then `construct` with the
/// block and `words` (null when the allocation fails, and the constructor
/// does not run).
fn new_procedure(e: &mut Engine, size: u32, construct: u32, words: &[u32]) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut call = vec![block];
    call.extend_from_slice(words);
    e.call(construct, &call).u32()
}

/// The controller's +0x9C object.
fn controller_state(e: &mut Engine, controller: u32) -> u32 {
    e.call(CONTROLLER_STATE, &args![controller]).u32()
}

/// `CombatAction::GetCost` of `this` with the two words.
fn base_cost(e: &mut Engine, this: Ptr, controller: u32, word: u32) -> f64 {
    e.call(BASE_GET_COST, &args![this, controller, word]).f64()
}

/// `CombatAction::GetAttackCost(controller, index)`.
fn attack_cost(e: &mut Engine, controller: u32, index: u32) -> f64 {
    e.call(GET_ATTACK_COST, &args![controller, index]).f64()
}

/// The base cost (stored as a `double`) plus the attack cost of `index`, as
/// a `float`.
fn cost_with_attack(e: &mut Engine, this: Ptr, controller: u32, word: u32, index: u32) -> f32 {
    let cost = base_cost(e, this, controller, word);
    let attack = attack_cost(e, controller, index);
    f32::from_bits(x87_store32(x87_add(attack, cost)))
}

/// `FLT_MAX` from `0108cb10`, as `FLD` loads it.
fn no_cost(e: &mut Engine) -> f32 {
    x87_quiet(e.global::<f32>(NO_COST))
}

/// Whether the controller's +0xBC object is set, `00566950` holds on it and
/// its `008c1740("CombatStyleMelee")` is exactly 1.
fn melee_style(e: &mut Engine, controller: u32) -> bool {
    let controller = Ptr::new(controller);
    if fn_0097ae90(e, controller) == 0 {
        return false;
    }
    let object = fn_0097ae90(e, controller);
    if !e.call(FORCE_NEXT_UPDATE, &args![object]).bool() {
        return false;
    }
    let object = fn_0097ae90(e, controller);
    let value = e.call(STYLE_VALUE, &args![object, MELEE_STYLE_TEXT]).f64();
    value == e.global::<f64>(ONE)
}

/// The base cost when the attack cost of `index` is below `FLT_MAX`
/// (compared unrounded; NaN counts as below), else `FLT_MAX`.
fn cost_unless_no_attack(e: &mut Engine, this: Ptr, controller: u32, word: u32, index: u32) -> f32 {
    let attack = attack_cost(e, controller, index);
    let limit = no_cost(e);
    if f64::from(limit) <= attack {
        return limit;
    }
    base_cost(e, this, controller, word) as f32
}

// Translated from 0097ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackRanged::GetCost` (Xbox PDB; `thiscall(controller,
/// word)`): `FLT_MAX` when the controller's +0xBC object's
/// `"CombatStyleMelee"` value is 1 (see `melee_style`), else the base cost
/// plus the attack cost of index 1.
pub fn combat_action_attack_ranged_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    if melee_style(e, controller) {
        return no_cost(e);
    }
    cost_with_attack(e, this, controller, word, 1)
}

// Translated from 0097af70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097af70 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(controller, unused)`; an `Execute`
/// by its body): the action procedure `009d0890` (0x48 bytes) and the
/// movement procedure `CombatProcedureBeInCover` (0x6C bytes) with the
/// controller object's cover word ([`fn_0097b050`]).
pub fn fn_0097af70(e: &mut Engine, _this: Ptr, controller: u32, _unused_2: u32) {
    let procedure = new_procedure(e, 0x48, PROCEDURE_009D0890, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    let block = e.call(OPERATOR_NEW, &args![0x6cu32]).u32();
    let movement = if block != 0 {
        let state = controller_state(e, controller);
        let cover = fn_0097b050(e, Ptr::new(state));
        e.call(BE_IN_COVER, &args![block, cover]).u32()
    } else {
        0
    };
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, movement]);
}

// Translated from 0097b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the word at +0xE4.
pub fn fn_0097b050(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xe4)
}

// Translated from 0097b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackRangedFromCover::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): the base cost plus the attack cost of
/// index 1.
pub fn combat_action_attack_ranged_from_cover_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_with_attack(e, this, controller, word, 1)
}

// Translated from 0097b170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b170 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(controller, unused)`; an `Execute`
/// by its body): the action procedure `009d0890` (0x48 bytes) and the
/// movement procedure `CombatProcedureEngageTarget` (0x64 bytes).
pub fn fn_0097b170(e: &mut Engine, _this: Ptr, controller: u32, _unused_2: u32) {
    let procedure = new_procedure(e, 0x48, PROCEDURE_009D0890, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    let movement = new_procedure(e, 0x64, ENGAGE_TARGET, &[]);
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, movement]);
}

// Translated from 0097b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackRangedExplosive::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): the base cost plus the attack cost of
/// index 0.
pub fn combat_action_attack_ranged_explosive_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_with_attack(e, this, controller, word, 0)
}

// Translated from 0097b410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b410 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(controller, unused)`; an `Execute`
/// by its body): the action procedure `009cc0c0` (0x88 bytes).
pub fn fn_0097b410(e: &mut Engine, _this: Ptr, controller: u32, _unused_2: u32) {
    let procedure = new_procedure(e, 0x88, PROCEDURE_009CC0C0, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackMelee::GetCost` (Xbox PDB; `thiscall(controller,
/// word)`): the base cost plus the attack cost of index 2.
pub fn combat_action_attack_melee_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_with_attack(e, this, controller, word, 2)
}

// Translated from 0097b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackHandToHand::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): `FLT_MAX` when the controller's +0xBC
/// object is set, `00566950` holds on it and it has a current weapon
/// (`Actor::GetCurrentWeapon`); else the base cost plus the attack cost of
/// index 6.
pub fn combat_action_attack_hand_to_hand_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    let owner = Ptr::new(controller);
    if fn_0097ae90(e, owner) != 0 {
        let object = fn_0097ae90(e, owner);
        if e.call(FORCE_NEXT_UPDATE, &args![object]).bool() {
            let object = fn_0097ae90(e, owner);
            if e.call(CURRENT_WEAPON, &args![object]).u32() != 0 {
                return no_cost(e);
            }
        }
    }
    cost_with_attack(e, this, controller, word, 6)
}

// Translated from 0097b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b6d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenade::Execute` (Xbox PDB; `thiscall(controller,
/// unused)`): the action procedure `CombatProcedureAttackGrenade` (0x3C
/// bytes) and the movement procedure `CombatProcedureEngageTarget` (0x64
/// bytes).
pub fn combat_action_attack_grenade_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    _unused_2: u32,
) {
    let procedure = new_procedure(e, 0x3c, ATTACK_GRENADE, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    let movement = new_procedure(e, 0x64, ENGAGE_TARGET, &[]);
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, movement]);
}

// Translated from 0097b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackGrenadeFlushTarget::CanExecute` (Xbox PDB;
/// `thiscall(controller, world_state, default_value)`): false when
/// `CombatAction::CanExecute` is; else `CombatState::FindCombatLocation(11)`
/// on the controller's object.
pub fn combat_action_attack_grenade_flush_target_can_execute(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    world_state: u32,
    default_value: u32,
) -> bool {
    if !e
        .call(
            BASE_CAN_EXECUTE,
            &args![this, controller, world_state, default_value],
        )
        .bool()
    {
        return false;
    }
    let state = controller_state(e, controller);
    e.call(FIND_COMBAT_LOCATION, &args![state, 0xbu32]).bool()
}

// Translated from 0097b880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097b880 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFlushTarget::Execute` (Xbox PDB;
/// `thiscall(controller, unused)`): the action procedure `009caee0` (0x3C
/// bytes) with the controller object's move target ([`fn_0097b940`] of
/// `00778970`) and its byte at +0x76 ([`fn_0097b960`]).
pub fn combat_action_attack_grenade_flush_target_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    _unused_2: u32,
) {
    let state = controller_state(e, controller);
    let state_again = controller_state(e, controller);
    let target = e.call(STATE_MOVE_TARGET, &args![state_again]).u32();
    let block = e.call(OPERATOR_NEW, &args![0x3cu32]).u32();
    let procedure = if block != 0 {
        let flag = u32::from(fn_0097b960(e, Ptr::new(state)));
        let location = fn_0097b940(e, Ptr::new(target)).addr();
        e.call(PROCEDURE_009CAEE0, &args![block, location, flag])
            .u32()
    } else {
        0
    };
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the byte at +0x76.
pub fn fn_0097b960(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x76)
}

// Translated from 0097b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackGrenadeFlushTarget::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): the attack cost of index 3 (stored as a
/// `float`); `FLT_MAX` when it is at least `FLT_MAX`, else the base cost
/// plus it. A NaN attack cost takes the sum.
pub fn combat_action_attack_grenade_flush_target_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    let attack = x87_store32(attack_cost(e, controller, 3));
    let limit = no_cost(e);
    if f64::from(limit) <= x87_load32(attack) {
        return no_cost(e);
    }
    let cost = base_cost(e, this, controller, word);
    f32::from_bits(x87_store32(x87_add(cost, x87_m32(attack))))
}

// Translated from 0097bab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bab0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackGrenadeFromCover::Execute` (Xbox PDB;
/// `thiscall(controller, unused)`): the action procedure
/// `CombatProcedureAttackGrenade` (0x3C bytes) and the movement procedure
/// `CombatProcedureBeInCover` (0x6C bytes) with the controller object's
/// cover word ([`fn_0097b050`]).
pub fn combat_action_attack_grenade_from_cover_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    _unused_2: u32,
) {
    let procedure = new_procedure(e, 0x3c, ATTACK_GRENADE, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    let block = e.call(OPERATOR_NEW, &args![0x6cu32]).u32();
    let movement = if block != 0 {
        let state = controller_state(e, controller);
        let cover = fn_0097b050(e, Ptr::new(state));
        e.call(BE_IN_COVER, &args![block, cover]).u32()
    } else {
        0
    };
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, movement]);
}

// Translated from 0097bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionAttackGrenade::GetCost` (Xbox PDB; `thiscall(controller,
/// word)`): the base cost plus the attack cost of index 3.
pub fn combat_action_attack_grenade_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_with_attack(e, this, controller, word, 3)
}

// Translated from 0097bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMove::CheckIdentical` (Xbox PDB; `thiscall(controller,
/// location)`): [`combat_action_move_check_location_identical`].
pub fn combat_action_move_check_identical(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    location: u32,
) -> bool {
    combat_action_move_check_location_identical(e, controller, location)
}

// Translated from 0097bca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMove::Execute` (Xbox PDB; `thiscall(controller, word)`):
/// [`combat_action_move_start_movement_procedure`].
pub fn combat_action_move_execute(e: &mut Engine, _this: Ptr, controller: u32, word: u32) {
    combat_action_move_start_movement_procedure(e, controller, word);
}

// Translated from 0097bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMove::GetCost` (Xbox PDB; `thiscall(controller, index)`):
/// the base cost (stored as a `float`) plus the value `index` of the
/// controller's `005e3fc0` object ([`fn_0097bd00`]), as a `float`.
pub fn combat_action_move_get_cost(e: &mut Engine, this: Ptr, controller: u32, index: u32) -> f32 {
    let cost = x87_store32(base_cost(e, this, controller, index));
    let planner = e.call(0x005e_3fc0, &args![controller]).u32();
    let value = fn_0097bd00(e, Ptr::new(planner), index);
    f32::from_bits(x87_store32(x87_add(f64::from(value), x87_m32(cost))))
}

// Translated from 0097bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the `float` at
/// `+0xB8 + 4 * index` (`FLD`).
pub fn fn_0097bd00(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0xb8);
    x87_quiet(e.mem.f32(address))
}

// Translated from 0097bd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMove::CheckLocationIdentical` (Xbox PDB;
/// `cdecl(controller, location)`): whether the controller's current
/// procedure (`004fd3e0`, null: false) already moves to `location`. The
/// controller object's move target (`00778970`) must be set and its
/// `00825c00` equal `location`. A procedure of kind 6 (slot 0x34) must have
/// the same `00825c00` as `00639b40` gives; with a `0044edb0` on the target
/// it must equal the procedure's `009e32d0`, without one the target's
/// location ([`fn_0097b940`]) must not differ from the procedure's point
/// ([`fn_0097be80`], `00439090` true means different); and the target's
/// flag 1 ([`fn_0097be60`]) must be clear. A procedure of kind 4 needs flag
/// 1 set and the same `00825c00` on both. Any other kind: false.
pub fn combat_action_move_check_location_identical(
    e: &mut Engine,
    controller: u32,
    location: u32,
) -> bool {
    let procedure = e.call(0x004f_d3e0, &args![controller]).u32();
    if procedure == 0 {
        return false;
    }
    let state = controller_state(e, controller);
    let target = e.call(STATE_MOVE_TARGET, &args![state]).u32();
    if target == 0 || e.call(0x0082_5c00, &args![target]).u32() != location {
        return false;
    }
    let kind = e.vcall(procedure, 0x34, &[]).u32();
    if kind == 6 {
        let target_location = e.call(0x0082_5c00, &args![target]).u32();
        if target_location != e.call(0x0063_9b40, &args![procedure]).u32() {
            return false;
        }
        if e.call(0x0044_edb0, &args![target]).u32() != 0 {
            let target_index = e.call(0x0044_edb0, &args![target]).u32();
            if target_index != e.call(0x009e_32d0, &args![procedure]).u32() {
                return false;
            }
        } else {
            let differs = e.with_stack(0xc, |e, point| {
                let point = fn_0097be80(e, Ptr::new(procedure), point.addr());
                let position = fn_0097b940(e, Ptr::new(target)).addr();
                e.call(0x0043_9090, &args![position, point]).bool()
            });
            if differs {
                return false;
            }
        }
        if fn_0097be60(e, Ptr::new(target), 1) {
            return false;
        }
        true
    } else if kind == 4 {
        if !fn_0097be60(e, Ptr::new(target), 1) {
            return false;
        }
        let target_location = e.call(0x0082_5c00, &args![target]).u32();
        if target_location != e.call(0x0082_5c00, &args![procedure]).u32() {
            return false;
        }
        true
    } else {
        false
    }
}

// Translated from 0097be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(mask)`): whether the word at +0x1C
/// has any bit of `mask`.
pub fn fn_0097be60(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x1c) & mask != 0
}

// Translated from 0097be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(out)`): copies the three words at
/// `006815c0(this + 0x14)` to `out` and returns `out`.
pub fn fn_0097be80(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let point = e.call(0x0068_15c0, &args![this.addr() + 0x14]).u32();
    for offset in [0, 4, 8] {
        let word = e.mem.u32(point + offset);
        e.mem.set_u32(out + offset, word);
    }
    out
}

// Translated from 0097beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl(controller, unused, word)`):
/// `CombatState::FindCombatLocation(word)` on the controller's object.
pub fn fn_0097beb0(e: &mut Engine, controller: u32, _unused_1: u32, word: u32) -> bool {
    let state = controller_state(e, controller);
    e.call(FIND_COMBAT_LOCATION, &args![state, word]).bool()
}

// Translated from 0097bed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097bed0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMove::StartMovementProcedure` (Xbox PDB;
/// `cdecl(controller, word)`): with flag 1 of the controller object's move
/// target set ([`fn_0097be60`]), the movement procedure `009d5ad0` (0x18
/// bytes, `word`); otherwise `009d69f0` (0x7C bytes) with the target, its
/// `004a7bd0` value (as a `float`) and `word`. Null when the allocation
/// fails.
pub fn combat_action_move_start_movement_procedure(e: &mut Engine, controller: u32, word: u32) {
    let state = controller_state(e, controller);
    let target = e.call(STATE_MOVE_TARGET, &args![state]).u32();
    let procedure = if fn_0097be60(e, Ptr::new(target), 1) {
        new_procedure(e, 0x18, PROCEDURE_009D5AD0, &[word])
    } else {
        let block = e.call(OPERATOR_NEW, &args![0x7cu32]).u32();
        if block != 0 {
            let value = x87_store32(e.call(0x004a_7bd0, &args![target]).f64());
            e.call(PROCEDURE_009D69F0, &args![block, target, value, word])
                .u32()
        } else {
            0
        }
    };
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097c090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c090 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndSwitchToMelee::Execute` (Xbox PDB;
/// `thiscall(controller, word)`): `CombatActionMove::Execute`, then the
/// background procedure `CombatProcedureSwitchWeapon` (0x18 bytes) with the
/// controller object's weapon 2 ([`fn_0097c150`]), which `00714d70` also
/// gets.
pub fn combat_action_move_and_switch_to_melee_execute(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) {
    combat_action_move_execute(e, this, controller, word);
    let state = controller_state(e, controller);
    let weapon = fn_0097c150(e, Ptr::new(state), 2);
    let procedure = new_procedure(e, 0x18, SWITCH_WEAPON, &[weapon]);
    e.call(ADD_BACKGROUND_PROCEDURE, &args![controller, procedure]);
    e.call(0x0071_4d70, &args![controller, weapon]);
}

// Translated from 0097c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the word at
/// `+0x0C + 4 * index`.
pub fn fn_0097c150(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0xc);
    e.mem.u32(address)
}

// Translated from 0097c280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c280 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(controller, word)`; an `Execute` by
/// its body): the action procedure `009d0890` (0x48 bytes), then
/// `CombatActionMove::StartMovementProcedure`.
pub fn fn_0097c280(e: &mut Engine, _this: Ptr, controller: u32, word: u32) {
    let procedure = new_procedure(e, 0x48, PROCEDURE_009D0890, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    combat_action_move_start_movement_procedure(e, controller, word);
}

// Translated from 0097c310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMoveAndAttackRanged::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): `FLT_MAX` when the attack cost of index 1
/// is at least `FLT_MAX` or the controller's `"CombatStyleMelee"` value is
/// 1, else the base cost.
pub fn combat_action_move_and_attack_ranged_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    let attack = attack_cost(e, controller, 1);
    if f64::from(no_cost(e)) <= attack {
        return no_cost(e);
    }
    if melee_style(e, controller) {
        return no_cost(e);
    }
    base_cost(e, this, controller, word) as f32
}

// Translated from 0097c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMoveAndAttackRanged::CanExecute` (Xbox PDB;
/// `thiscall(controller, world_state, default_value)`): false when
/// `CombatAction::CanExecute` is, else [`fn_0097beb0`] with the three words.
pub fn combat_action_move_and_attack_ranged_can_execute(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    world_state: u32,
    default_value: u32,
) -> bool {
    if !e
        .call(
            BASE_CAN_EXECUTE,
            &args![this, controller, world_state, default_value],
        )
        .bool()
    {
        return false;
    }
    fn_0097beb0(e, controller, world_state, default_value)
}

// Translated from 0097c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMoveAndAttackRangedExplosive::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): `FLT_MAX` when the attack cost of index 0
/// is at least `FLT_MAX`, else the base cost.
pub fn combat_action_move_and_attack_ranged_explosive_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_unless_no_attack(e, this, controller, word, 0)
}

// Translated from 0097c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMove::GetDebugText` (Xbox PDB; `thiscall(buffer, size,
/// word)`): `"%s: %s"` with the action's name and the string of the low 16
/// bits of `word` in the table at `011a5700` (`00406d00`).
pub fn combat_action_move_get_debug_text(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    word: u32,
) {
    let text = e.mem.u32(MOVE_TEXTS + (word & 0xffff) * 4);
    let name = e.call(GET_NAME, &args![this]).u32();
    e.call(PRINT_TEXT, &args![buffer, size, NAME_AND_TEXT, name, text]);
}

// Translated from 0097c620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c620 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionMoveAndAttackGrenade::Execute` (Xbox PDB;
/// `thiscall(controller, word)`): the action procedure
/// `CombatProcedureAttackGrenade` (0x3C bytes), then
/// `CombatActionMove::StartMovementProcedure`.
pub fn combat_action_move_and_attack_grenade_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    word: u32,
) {
    let procedure = new_procedure(e, 0x3c, ATTACK_GRENADE, &[]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
    combat_action_move_start_movement_procedure(e, controller, word);
}

// Translated from 0097c6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionMoveAndAttackGrenade::GetCost` (Xbox PDB;
/// `thiscall(controller, word)`): `FLT_MAX` when the attack cost of index 3
/// is at least `FLT_MAX`, else the base cost.
pub fn combat_action_move_and_attack_grenade_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    word: u32,
) -> f32 {
    cost_unless_no_attack(e, this, controller, word, 3)
}

// Translated from 0097c790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097c790 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(controller, index)`; an `Execute`
/// by its body): the background procedure `CombatProcedureSwitchWeapon`
/// (0x18 bytes) with the controller object's weapon `index`
/// ([`fn_0097c150`]; none for index 6), which `00714d70` also gets.
pub fn fn_0097c790(e: &mut Engine, _this: Ptr, controller: u32, index: u32) {
    let weapon = if index != 6 {
        let state = controller_state(e, controller);
        fn_0097c150(e, Ptr::new(state), index)
    } else {
        0
    };
    let procedure = new_procedure(e, 0x18, SWITCH_WEAPON, &[weapon]);
    e.call(ADD_BACKGROUND_PROCEDURE, &args![controller, procedure]);
    e.call(0x0071_4d70, &args![controller, weapon]);
}

// Translated from 0097c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionSwitchWeapon::GetCost` (Xbox PDB; `thiscall(controller,
/// index)`): `FLT_MAX` when the controller's `"CombatStyleMelee"` value is 1
/// and its weapon `index` (none for 6) is set and fails `006450c0`; else the
/// base cost.
pub fn combat_action_switch_weapon_get_cost(
    e: &mut Engine,
    this: Ptr,
    controller: u32,
    index: u32,
) -> f32 {
    if melee_style(e, controller) {
        let weapon = if index != 6 {
            let state = controller_state(e, controller);
            fn_0097c150(e, Ptr::new(state), index)
        } else {
            0
        };
        if weapon != 0 && !e.call(WEAPON_TEST, &args![weapon]).bool() {
            return no_cost(e);
        }
    }
    base_cost(e, this, controller, index) as f32
}

// Translated from 0097c990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionDrawWeapon::GetDebugText` (Xbox PDB; `thiscall(buffer,
/// size, index)`): `"%s: %s"` with the action's name and string `index` of
/// the table at `0118a8a8` (`00406d00`).
pub fn combat_action_draw_weapon_get_debug_text(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    index: u32,
) {
    let text = e.mem.u32(WEAPON_TEXTS.wrapping_add(index.wrapping_mul(4)));
    let name = e.call(GET_NAME, &args![this]).u32();
    e.call(PRINT_TEXT, &args![buffer, size, NAME_AND_TEXT, name, text]);
}

// Translated from 0097caf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097caf0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionSearch::Execute` (Xbox PDB; `thiscall(controller,
/// unused)`): the movement procedure `CombatProcedureSearch` (0x40 bytes).
pub fn combat_action_search_execute(e: &mut Engine, _this: Ptr, controller: u32, _unused_2: u32) {
    let procedure = new_procedure(e, 0x40, SEARCH, &[]);
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionIgnoreBlockedTarget::Execute` (Xbox PDB;
/// `thiscall(controller, unused)`): clears the byte at +0x72 of the
/// controller's object ([`fn_0097cd10`] with 0).
pub fn combat_action_ignore_blocked_target_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    _unused_2: u32,
) {
    let state = controller_state(e, controller);
    fn_0097cd10(e, Ptr::new(state), 0);
}

// Translated from 0097cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(byte)`): stores `value` at +0x72.
pub fn fn_0097cd10(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x72, value);
}

// Translated from 0097cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cdb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionFlee::Execute` (Xbox PDB; `thiscall(controller, unused)`):
/// `0097ce70(2)` on the controller's object, then the movement procedure
/// `CombatProcedureHideFromTarget` (0x40 bytes) with the object's cover
/// word ([`fn_0097b050`]), the `float` of the setting at `011ce6e4`
/// (`00403e20` gives its address) and 1, 1.
pub fn combat_action_flee_execute(e: &mut Engine, _this: Ptr, controller: u32, _unused_2: u32) {
    let state = controller_state(e, controller);
    e.call(0x0097_ce70, &args![state, 2u32]);
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    let procedure = if block != 0 {
        let value = e.call(0x0040_3e20, &args![0x011c_e6e4u32]).u32();
        let distance = x87_quiet_bits(e.mem.u32(value));
        let state = controller_state(e, controller);
        let cover = fn_0097b050(e, Ptr::new(state));
        e.call(HIDE_FROM_TARGET, &args![block, cover, distance, 1u32, 1u32])
            .u32()
    } else {
        0
    };
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097cf10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097cf10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionActivateCombatItem::Execute` (Xbox PDB;
/// `thiscall(controller, kind)`): with the controller object's item
/// `kind - 0x11` ([`fn_0097cfc0`]) set, the action procedure
/// `CombatProcedureActivateObject` (0x1C bytes) with the item's two words.
pub fn combat_action_activate_combat_item_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    kind: u32,
) {
    let state = controller_state(e, controller);
    let item = fn_0097cfc0(e, Ptr::new(state), kind.wrapping_sub(0x11));
    if item == 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    let procedure = if block != 0 {
        let first = e.mem.u32(item);
        let second = e.mem.u32(item + 4);
        e.call(ACTIVATE_OBJECT, &args![block, first, second]).u32()
    } else {
        0
    };
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the word at
/// `+0x190 + 4 * index`.
pub fn fn_0097cfc0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0x190);
    e.mem.u32(address)
}

// Translated from 0097d060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d060 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionUseCombatItem::Execute` (Xbox PDB; `thiscall(controller,
/// kind)`): with the controller object's item `kind - 0x11`
/// ([`fn_0097d110`]) set, the action procedure
/// `CombatProcedureUseCombatItem` (0x18 bytes) with it.
pub fn combat_action_use_combat_item_execute(
    e: &mut Engine,
    _this: Ptr,
    controller: u32,
    kind: u32,
) {
    let state = controller_state(e, controller);
    let item = fn_0097d110(e, Ptr::new(state), kind.wrapping_sub(0x11));
    if item == 0 {
        return;
    }
    let procedure = new_procedure(e, 0x18, USE_COMBAT_ITEM, &[item]);
    e.call(SET_ACTION_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(index)`): the word at
/// `+0x1A4 + 4 * index`.
pub fn fn_0097d110(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let address = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(0x1a4);
    e.mem.u32(address)
}

// Translated from 0097d2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097d2d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionHide::Execute` (Xbox PDB; `thiscall(controller, count)`):
/// the movement procedure `CombatProcedureHideFromTarget` (0x40 bytes) with
/// the controller object's cover word ([`fn_0097b050`]), `count` as a
/// `float` (`FILD`) and 0, 0.
pub fn combat_action_hide_execute(e: &mut Engine, _this: Ptr, controller: u32, count: i32) {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    let procedure = if block != 0 {
        let value = x87_store32(f64::from(count));
        let state = controller_state(e, controller);
        let cover = fn_0097b050(e, Ptr::new(state));
        e.call(HIDE_FROM_TARGET, &args![block, cover, value, 0u32, 0u32])
            .u32()
    } else {
        0
    };
    e.call(SET_MOVEMENT_PROCEDURE, &args![controller, procedure]);
}

// Translated from 0097d370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatActionHide::GetDebugText` (Xbox PDB; `thiscall(buffer, size,
/// number)`): `"%s: %i"` with the action's name and `number` (`00406d00`).
pub fn combat_action_hide_get_debug_text(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    number: u32,
) {
    let name = e.call(GET_NAME, &args![this]).u32();
    e.call(
        PRINT_TEXT,
        &args![buffer, size, NAME_AND_NUMBER, name, number],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_b940, fn_0097b940(Ptr) -> Ptr),
        entry!(0x0097_c470, fn_0097c470(Ptr)),
        entry!(0x0097_ae90, fn_0097ae90(Ptr) -> u32),
        entry!(0x0097_bc60, fn_0097bc60(Ptr, u32, u32)),
        entry!(0x0097_c070, fn_0097c070(Ptr, u32)),
        entry!(0x0097_c240, fn_0097c240(Ptr, u32)),
        entry!(0x0097_c260, fn_0097c260(Ptr, u32)),
        entry!(0x0097_aeb0, fn_0097aeb0(Ptr, u32) -> Ptr),
        entry!(0x0097_b0b0, fn_0097b0b0(Ptr, u32) -> Ptr),
        entry!(0x0097_b280, fn_0097b280(Ptr, u32) -> Ptr),
        entry!(0x0097_b350, fn_0097b350(Ptr, u32) -> Ptr),
        entry!(0x0097_b4e0, fn_0097b4e0(Ptr, u32) -> Ptr),
        entry!(0x0097_b620, fn_0097b620(Ptr, u32) -> Ptr),
        entry!(0x0097_b7a0, fn_0097b7a0(Ptr, u32) -> Ptr),
        entry!(0x0097_b9e0, fn_0097b9e0(Ptr, u32) -> Ptr),
        entry!(0x0097_bbd0, fn_0097bbd0(Ptr, u32) -> Ptr),
        entry!(0x0097_bfe0, fn_0097bfe0(Ptr, u32) -> Ptr),
        entry!(0x0097_c170, fn_0097c170(Ptr, u32) -> Ptr),
        entry!(0x0097_c3a0, fn_0097c3a0(Ptr, u32) -> Ptr),
        entry!(0x0097_c560, fn_0097c560(Ptr, u32) -> Ptr),
        entry!(0x0097_c700, fn_0097c700(Ptr, u32) -> Ptr),
        entry!(0x0097_c850, fn_0097c850(Ptr, u32) -> Ptr),
        entry!(0x0097_c9d0, fn_0097c9d0(Ptr, u32) -> Ptr),
        entry!(0x0097_ca60, fn_0097ca60(Ptr, u32) -> Ptr),
        entry!(0x0097_cb70, fn_0097cb70(Ptr, u32) -> Ptr),
        entry!(0x0097_cc00, fn_0097cc00(Ptr, u32) -> Ptr),
        entry!(0x0097_cc80, fn_0097cc80(Ptr, u32) -> Ptr),
        entry!(0x0097_cd30, fn_0097cd30(Ptr, u32) -> Ptr),
        entry!(0x0097_cfe0, fn_0097cfe0(Ptr, u32) -> Ptr),
        entry!(0x0097_d1c0, fn_0097d1c0(Ptr, u32) -> Ptr),
        entry!(0x0097_d250, fn_0097d250(Ptr, u32) -> Ptr),
        entry!(0x0097_d130, fn_0097d130(Ptr, u32) -> Ptr),
        entry!(0x0097_ae00, combat_action_attack_ranged_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_af70, fn_0097af70(Ptr, u32, u32)),
        entry!(0x0097_b050, fn_0097b050(Ptr) -> u32),
        entry!(0x0097_b070, combat_action_attack_ranged_from_cover_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_b170, fn_0097b170(Ptr, u32, u32)),
        entry!(0x0097_b240, combat_action_attack_ranged_explosive_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_b410, fn_0097b410(Ptr, u32, u32)),
        entry!(0x0097_b4a0, combat_action_attack_melee_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_b5a0, combat_action_attack_hand_to_hand_get_cost(Ptr, u32, u32) -> f32),
        entry!(
            0x0097_b6d0,
            combat_action_attack_grenade_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_b840, combat_action_attack_grenade_flush_target_can_execute(Ptr, u32, u32, u32) -> bool),
        entry!(
            0x0097_b880,
            combat_action_attack_grenade_flush_target_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_b960, fn_0097b960(Ptr) -> u8),
        entry!(0x0097_b980, combat_action_attack_grenade_flush_target_get_cost(Ptr, u32, u32) -> f32),
        entry!(
            0x0097_bab0,
            combat_action_attack_grenade_from_cover_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_bb90, combat_action_attack_grenade_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_bc80, combat_action_move_check_identical(Ptr, u32, u32) -> bool),
        entry!(0x0097_bca0, combat_action_move_execute(Ptr, u32, u32)),
        entry!(0x0097_bcc0, combat_action_move_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_bd00, fn_0097bd00(Ptr, u32) -> f32),
        entry!(0x0097_bd20, combat_action_move_check_location_identical(u32, u32) -> bool),
        entry!(0x0097_be60, fn_0097be60(Ptr, u32) -> bool),
        entry!(0x0097_be80, fn_0097be80(Ptr, u32) -> u32),
        entry!(0x0097_beb0, fn_0097beb0(u32, u32, u32) -> bool),
        entry!(
            0x0097_bed0,
            combat_action_move_start_movement_procedure(u32, u32)
        ),
        entry!(
            0x0097_c090,
            combat_action_move_and_switch_to_melee_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_c150, fn_0097c150(Ptr, u32) -> u32),
        entry!(0x0097_c280, fn_0097c280(Ptr, u32, u32)),
        entry!(0x0097_c310, combat_action_move_and_attack_ranged_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_c490, combat_action_move_and_attack_ranged_can_execute(Ptr, u32, u32, u32) -> bool),
        entry!(0x0097_c4d0, combat_action_move_and_attack_ranged_explosive_get_cost(Ptr, u32, u32) -> f32),
        entry!(
            0x0097_c520,
            combat_action_move_get_debug_text(Ptr, u32, u32, u32)
        ),
        entry!(
            0x0097_c620,
            combat_action_move_and_attack_grenade_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_c6b0, combat_action_move_and_attack_grenade_get_cost(Ptr, u32, u32) -> f32),
        entry!(0x0097_c790, fn_0097c790(Ptr, u32, u32)),
        entry!(0x0097_c8e0, combat_action_switch_weapon_get_cost(Ptr, u32, u32) -> f32),
        entry!(
            0x0097_c990,
            combat_action_draw_weapon_get_debug_text(Ptr, u32, u32, u32)
        ),
        entry!(0x0097_caf0, combat_action_search_execute(Ptr, u32, u32)),
        entry!(
            0x0097_ccf0,
            combat_action_ignore_blocked_target_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_cd10, fn_0097cd10(Ptr, u8)),
        entry!(0x0097_cdb0, combat_action_flee_execute(Ptr, u32, u32)),
        entry!(
            0x0097_cf10,
            combat_action_activate_combat_item_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_cfc0, fn_0097cfc0(Ptr, u32) -> u32),
        entry!(
            0x0097_d060,
            combat_action_use_combat_item_execute(Ptr, u32, u32)
        ),
        entry!(0x0097_d110, fn_0097d110(Ptr, u32) -> u32),
        entry!(0x0097_d2d0, combat_action_hide_execute(Ptr, u32, i32)),
        entry!(
            0x0097_d370,
            combat_action_hide_get_debug_text(Ptr, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0097b940_returns_the_callees_pointer() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_b940, &args![0x0000_5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, 0x0068_15c0), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_0097c470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0097_9c80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_c470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0097_9c80), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;
    type Case = (u32, u32, Calls);

    const DOUBLED: [u32; 6] = [
        0x0097_9bd0,
        0x0097_ad10,
        0x0097_ade0,
        0x0097_adc0,
        0x0098_4390,
        LIST_ADD,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in DOUBLED {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e
    }

    #[test]
    fn small_helpers() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xbc, 0x1234);
        assert_eq!(e.call(0x0097_ae90, &args![this]).u32(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0097_bc60, &args![this, 3u32, 4u32]);
        e.call(0x0097_c070, &args![this, 5u32]);
        e.call(0x0097_c240, &args![this, 6u32]);
        e.call(0x0097_c260, &args![this, 7u32]);
        let log: Calls = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == LIST_ADD || *a == LIST_REMOVE)
            .collect();
        assert_eq!(
            log,
            vec![
                (LIST_ADD, vec![this + 0x1c, 3, 4]),
                (LIST_REMOVE, vec![this + 0xc, 5]),
                (LIST_REMOVE, vec![this + 0x14, 6]),
                (LIST_REMOVE, vec![this + 0x4, 7]),
            ]
        );
    }

    /// Every constructor, its vtable and the calls it makes (helpers of this
    /// file show as the list calls they make; a base constructor of this
    /// file shows as its own calls).
    fn cases(t: u32, k: u32) -> Vec<Case> {
        vec![
            (0x0097_aeb0, 0x0108_cb7c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x1]),
                    (0x0098_4390, vec![t, 0xa, 0x1]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b0b0, 0x0108_cb94, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0x6, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b280, 0x0108_cbac, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b350, 0x0108_cbc4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x12]),
                    (0x0098_4390, vec![t, 0xb, 0x2]),
                    (0x0098_4390, vec![t, 0xa, 0x2]),
                    (0x0098_4390, vec![t, 0x1, 0x1]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b4e0, 0x0108_cbdc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x12]),
                    (0x0098_4390, vec![t, 0xb, 0x6]),
                    (0x0098_4390, vec![t, 0xa, 0x6]),
                    (0x0098_4390, vec![t, 0x1, 0x1]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b620, 0x0108_cbf4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_b7a0, 0x0108_cc0c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x9, 0x1]),
                ]
            }),
            (0x0097_b9e0, 0x0108_cc24, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0x4, 0x1]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0097_ade0, vec![t, 0x7, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0098_4390, vec![t, 0x1, 0x2]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                ]
            }),
            (0x0097_bbd0, 0x0108_cc3c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_bfe0, 0x0108_cc54, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                    (0x009a_a340, vec![t + 0xc, 0xa]),
                    (0x0097_ade0, vec![t, 0xc, 0x2]),
                    (0x009a_a2b0, vec![t + 0x1c, 0xb, 0x2]),
                    (0x009a_a2b0, vec![t + 0x1c, 0xa, 0x2]),
                ]
            }),
            (0x0097_c170, 0x0108_cc6c, {
                vec![
                    (0x0097_ad10, vec![t, k]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0x4, 0x12]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c3a0, 0x0108_cc84, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0x6, 0x0]),
                    (0x0098_4390, vec![t, 0xb, 0x0]),
                    (0x0098_4390, vec![t, 0xa, 0x0]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0x4, 0x12]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c560, 0x0108_cc9c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0097_ade0, vec![t, 0x8, 0x0]),
                    (0x0098_4390, vec![t, 0x5, 0x1]),
                    (0x0098_4390, vec![t, 0xb, 0x3]),
                    (0x0098_4390, vec![t, 0xa, 0x3]),
                    (0x0097_adc0, vec![t, 0x0, 0x1]),
                    (0x009a_a340, vec![t + 0x4, 0x3]),
                    (0x009a_a340, vec![t + 0xc, 0x1]),
                    (0x009a_a340, vec![t + 0xc, 0x6]),
                    (0x009a_a340, vec![t + 0xc, 0x5]),
                    (0x009a_a340, vec![t + 0x14, 0x0]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0097_ade0, vec![t, 0xf, 0x0]),
                    (0x0097_adc0, vec![t, 0x1, u32::MAX]),
                    (0x009a_a2b0, vec![t + 0x1c, 0x6, 0x0]),
                ]
            }),
            (0x0097_c700, 0x0108_ccb4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xc, u32::MAX]),
                    (0x0097_ade0, vec![t, 0xa, 0x7]),
                    (0x0097_adc0, vec![t, 0xb, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xa, u32::MAX]),
                ]
            }),
            (0x0097_c850, 0x0108_cccc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0xc, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xb, u32::MAX]),
                    (0x0097_adc0, vec![t, 0xa, u32::MAX]),
                ]
            }),
            (0x0097_c9d0, 0x0108_cce4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0x1, 0xe]),
                    (0x0097_adc0, vec![t, 0xe, 0x0]),
                ]
            }),
            (0x0097_ca60, 0x0108_ccfc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x12, 0x11]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0097_adc0, vec![t, 0x3, 0x0]),
                ]
            }),
            (0x0097_cb70, 0x0108_cd14, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0xa, 0xffff_0018]),
                    (0x0098_4390, vec![t, 0x1, 0xf]),
                    (0x0097_adc0, vec![t, 0x3, 0x0]),
                ]
            }),
            (0x0097_cc00, 0x0108_cd2c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0x3]),
                    (0x0097_adc0, vec![t, 0x6, 0x0]),
                ]
            }),
            (0x0097_cc80, 0x0108_cd44, {
                vec![(0x0097_9bd0, vec![t, k]), (0x0097_adc0, vec![t, 0x6, 0x0])]
            }),
            (0x0097_cd30, 0x0108_cd5c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0xc]),
                    (0x0097_adc0, vec![t, 0xf, 0x0]),
                ]
            }),
            (0x0097_cfe0, 0x0108_cd8c, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0xd, u32::MAX]),
                    (0x0097_adc0, vec![t, 0x10, u32::MAX]),
                ]
            }),
            (0x0097_d1c0, 0x0108_cdbc, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0xd]),
                    (0x0097_adc0, vec![t, 0x5, 0x1]),
                ]
            }),
            (0x0097_d250, 0x0108_cdd4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0098_4390, vec![t, 0x1, 0xa]),
                    (0x0097_adc0, vec![t, 0x11, u32::MAX]),
                ]
            }),
            (0x0097_d130, 0x0108_cda4, {
                vec![
                    (0x0097_9bd0, vec![t, k]),
                    (0x0097_ade0, vec![t, 0x3, 0x0]),
                    (0x0098_4390, vec![t, 0x1, 0x0]),
                    (0x0097_adc0, vec![t, 0x6, 0x0]),
                    (0x0097_adc0, vec![t, 0x5, 0x1]),
                ]
            }),
        ]
    }

    #[test]
    fn each_constructor_builds_its_base_sets_its_vtable_and_fills_its_lists() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        let kind = 0x17;
        let addresses: Vec<u32> = cases(this, kind).iter().map(|c| c.0).collect();
        assert_eq!(addresses.len(), 25);
        for (address, vtable, expected) in cases(this, kind) {
            e.call_log = Some(vec![]);
            let back = e.call(address, &args![this, kind]).u32();
            assert_eq!(back, this, "{address:08x}");
            assert_eq!(e.mem.u32(this), vtable, "{address:08x}");
            let log: Calls = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != address)
                .collect();
            assert_eq!(log, expected, "{address:08x}");
        }
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    const PROCEDURES: [u32; 13] = [
        PROCEDURE_009D0890,
        BE_IN_COVER,
        ENGAGE_TARGET,
        PROCEDURE_009CC0C0,
        ATTACK_GRENADE,
        PROCEDURE_009CAEE0,
        PROCEDURE_009D5AD0,
        PROCEDURE_009D69F0,
        SWITCH_WEAPON,
        SEARCH,
        HIDE_FROM_TARGET,
        ACTIVATE_OBJECT,
        USE_COMBAT_ITEM,
    ];
    const SETTERS: [u32; 3] = [
        SET_ACTION_PROCEDURE,
        SET_MOVEMENT_PROCEDURE,
        ADD_BACKGROUND_PROCEDURE,
    ];
    const SET_WEAPON: u32 = 0x0071_4d70;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The calls to the procedure constructors and setters, in order.
    fn procedure_calls(e: &Engine) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| PROCEDURES.contains(a) || SETTERS.contains(a) || *a == SET_WEAPON)
            .cloned()
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        procedure_calls(e).iter().map(|(a, _)| *a).collect()
    }

    fn sizes(e: &Engine) -> Vec<u32> {
        calls(e, OPERATOR_NEW).iter().map(|w| w[0]).collect()
    }

    fn ret_f64(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// The action (+0x30: its base cost), the controller (+0x40 + 8 * i: its
    /// attack cost `i` as a `double`; +0x9C its object; +0xBC its melee-style
    /// object, whose +4 byte answers `00566950`, +8 `double` is its style
    /// value and +0x10 its current weapon) and the controller's object.
    fn world() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0108_c000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x011a_5000, 0x1000);
        e.map(0x0118_a000, 0x1000);
        e.set_global(NO_COST, f32::MAX);
        e.set_global(ONE, 1.0f64);
        let this = e.mem.alloc(0x40);
        let controller = e.mem.alloc(0x100);
        let state = e.mem.alloc(0x200);
        e.mem.set_u32(controller + 0x9c, state);
        e.register(CONTROLLER_STATE, |e, a| e.mem.u32(a[0] + 0x9c).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for constructor in PROCEDURES {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        for addr in SETTERS {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in [SET_WEAPON, PRINT_TEXT, 0x0097_ce70] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(BASE_GET_COST, |e, a| {
            ret_f64(f64::from(e.mem.f32(a[0] + 0x30)))
        });
        e.register(GET_ATTACK_COST, |e, a| {
            ret_f64(e.mem.f64(a[0] + 0x40 + 8 * a[1]))
        });
        e.register(GET_NAME, |_, _| 0x0123_4567u32.into_ret());
        e.register(FORCE_NEXT_UPDATE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(STYLE_VALUE, |e, a| ret_f64(e.mem.f64(a[0] + 8)));
        e.register(CURRENT_WEAPON, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.call_log = Some(vec![]);
        (e, this, controller, state)
    }

    /// Gives the controller a melee-style object answering `00566950` with
    /// `forced` and with style value `style`.
    fn melee_object(e: &mut Engine, controller: u32, forced: bool, style: f64) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u8(object + 4, forced as u8);
        e.mem.set_f64(object + 8, style);
        e.mem.set_u32(controller + 0xbc, object);
        object
    }

    fn cost(e: &mut Engine, addr: u32, this: u32, controller: u32, word: u32) -> f32 {
        e.call(addr, &args![this, controller, word]).f32()
    }

    #[test]
    fn summed_costs_add_the_attack_cost_of_their_index() {
        let (mut e, this, controller, _) = world();
        e.mem.set_f32(this + 0x30, 1.5);
        for index in 0..7u32 {
            e.mem
                .set_f64(controller + 0x40 + 8 * index, f64::from(index) * 10.0);
        }
        let cases = [
            (0x0097_ae00, 11.5),
            (0x0097_b070, 11.5),
            (0x0097_b240, 1.5),
            (0x0097_b4a0, 21.5),
            (0x0097_b5a0, 61.5),
            (0x0097_bb90, 31.5),
        ];
        for (addr, expected) in cases {
            assert_eq!(
                cost(&mut e, addr, this, controller, 9),
                expected,
                "{addr:08x}"
            );
        }
        assert_eq!(calls(&e, BASE_GET_COST)[0], vec![this, controller, 9]);
        assert_eq!(calls(&e, GET_ATTACK_COST)[0], vec![controller, 1]);
    }

    #[test]
    fn melee_style_and_hand_to_hand_weapon_cost_nothing_to_choose() {
        let (mut e, this, controller, _) = world();
        e.mem.set_f32(this + 0x30, 1.5);
        let object = melee_object(&mut e, controller, true, 1.0);
        assert_eq!(cost(&mut e, 0x0097_ae00, this, controller, 0), f32::MAX);
        assert_eq!(calls(&e, STYLE_VALUE), vec![vec![object, MELEE_STYLE_TEXT]]);
        // A style value other than 1, or no forced update: the sum.
        e.mem.set_f64(object + 8, 0.5);
        assert_eq!(cost(&mut e, 0x0097_ae00, this, controller, 0), 1.5);
        e.mem.set_f64(object + 8, 1.0);
        e.mem.set_u8(object + 4, 0);
        assert_eq!(cost(&mut e, 0x0097_ae00, this, controller, 0), 1.5);
        // Hand to hand: a current weapon on a forced object.
        e.mem.set_u8(object + 4, 1);
        assert_eq!(cost(&mut e, 0x0097_b5a0, this, controller, 0), 1.5);
        e.mem.set_u32(object + 0x10, 0x7000);
        assert_eq!(cost(&mut e, 0x0097_b5a0, this, controller, 0), f32::MAX);
    }

    #[test]
    fn capped_costs_give_up_at_float_max() {
        let (mut e, this, controller, _) = world();
        e.mem.set_f32(this + 0x30, 2.0);
        e.mem.set_f64(controller + 0x40 + 8 * 3, 0.5);
        assert_eq!(cost(&mut e, 0x0097_b980, this, controller, 0), 2.5);
        assert_eq!(cost(&mut e, 0x0097_c6b0, this, controller, 0), 2.0);
        e.mem
            .set_f64(controller + 0x40 + 8 * 3, f64::from(f32::MAX) * 2.0);
        assert_eq!(cost(&mut e, 0x0097_b980, this, controller, 0), f32::MAX);
        assert_eq!(cost(&mut e, 0x0097_c6b0, this, controller, 0), f32::MAX);
        // NaN is below the cap.
        e.mem.set_f64(controller + 0x40, f64::NAN);
        assert_eq!(cost(&mut e, 0x0097_c4d0, this, controller, 0), 2.0);
        e.mem.set_f64(controller + 0x40, f64::from(f32::MAX));
        assert_eq!(cost(&mut e, 0x0097_c4d0, this, controller, 0), f32::MAX);
        // Move and attack ranged: the cap, then the melee style.
        assert_eq!(cost(&mut e, 0x0097_c310, this, controller, 0), 2.0);
        e.mem.set_f64(controller + 0x48, f64::from(f32::MAX));
        assert_eq!(cost(&mut e, 0x0097_c310, this, controller, 0), f32::MAX);
        e.mem.set_f64(controller + 0x48, 0.0);
        melee_object(&mut e, controller, true, 1.0);
        assert_eq!(cost(&mut e, 0x0097_c310, this, controller, 0), f32::MAX);
    }

    #[test]
    fn move_cost_adds_the_planner_value() {
        let (mut e, this, controller, _) = world();
        e.mem.set_f32(this + 0x30, 1.25);
        let planner = e.mem.alloc(0x100);
        e.mem.set_f32(planner + 0xb8 + 8, 2.5);
        e.register_double(0x005e_3fc0, move |_, _| planner.into_ret());
        assert_eq!(cost(&mut e, 0x0097_bcc0, this, controller, 2), 3.75);
        assert_eq!(calls(&e, 0x005e_3fc0), vec![vec![controller]]);
        assert_eq!(calls(&e, BASE_GET_COST), vec![vec![this, controller, 2]]);
        assert_eq!(e.call(0x0097_bd00, &args![planner, 2u32]).f32(), 2.5);
    }

    #[test]
    fn switch_weapon_cost_tests_the_weapon() {
        let (mut e, this, controller, state) = world();
        e.mem.set_f32(this + 0x30, 1.0);
        e.register(WEAPON_TEST, |e, a| e.mem.u8(a[0]).into_ret());
        let weapon = e.mem.alloc(4);
        e.mem.set_u32(state + 0xc + 8, weapon);
        assert_eq!(cost(&mut e, 0x0097_c8e0, this, controller, 2), 1.0);
        melee_object(&mut e, controller, true, 1.0);
        assert_eq!(cost(&mut e, 0x0097_c8e0, this, controller, 2), f32::MAX);
        assert_eq!(calls(&e, WEAPON_TEST), vec![vec![weapon]]);
        e.mem.set_u8(weapon, 1);
        assert_eq!(cost(&mut e, 0x0097_c8e0, this, controller, 2), 1.0);
        // Index 6 has no weapon.
        assert_eq!(cost(&mut e, 0x0097_c8e0, this, controller, 6), 1.0);
        assert_eq!(calls(&e, WEAPON_TEST).len(), 2);
    }

    #[test]
    fn executes_set_their_procedures() {
        let (mut e, this, c, state) = world();
        e.mem.set_u32(state + 0xe4, 0x6600);
        e.call(0x0097_af70, &args![this, c, 0u32]);
        let log = procedure_calls(&e);
        assert_eq!(log[0].0, PROCEDURE_009D0890);
        assert_eq!(log[1], (SET_ACTION_PROCEDURE, vec![c, log[0].1[0]]));
        assert_eq!(log[2], (BE_IN_COVER, vec![log[2].1[0], 0x6600]));
        assert_eq!(log[3], (SET_MOVEMENT_PROCEDURE, vec![c, log[2].1[0]]));
        assert_eq!(sizes(&e), vec![0x48, 0x6c]);
        let cases: [(u32, Vec<u32>, Vec<u32>); 6] = [
            (
                0x0097_b170,
                vec![0x48, 0x64],
                vec![
                    PROCEDURE_009D0890,
                    SET_ACTION_PROCEDURE,
                    ENGAGE_TARGET,
                    SET_MOVEMENT_PROCEDURE,
                ],
            ),
            (
                0x0097_b410,
                vec![0x88],
                vec![PROCEDURE_009CC0C0, SET_ACTION_PROCEDURE],
            ),
            (
                0x0097_b6d0,
                vec![0x3c, 0x64],
                vec![
                    ATTACK_GRENADE,
                    SET_ACTION_PROCEDURE,
                    ENGAGE_TARGET,
                    SET_MOVEMENT_PROCEDURE,
                ],
            ),
            (
                0x0097_bab0,
                vec![0x3c, 0x6c],
                vec![
                    ATTACK_GRENADE,
                    SET_ACTION_PROCEDURE,
                    BE_IN_COVER,
                    SET_MOVEMENT_PROCEDURE,
                ],
            ),
            (
                0x0097_caf0,
                vec![0x40],
                vec![SEARCH, SET_MOVEMENT_PROCEDURE],
            ),
            (
                0x0097_cdb0,
                vec![0x40],
                vec![HIDE_FROM_TARGET, SET_MOVEMENT_PROCEDURE],
            ),
        ];
        e.register(0x0040_3e20, |_, a| (a[0] + 0x10).into_ret());
        e.map(0x011c_e000, 0x1000);
        e.mem.set_u32(0x011c_e6f4, 0x7f80_0001);
        for (addr, expected_sizes, expected) in cases {
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, c, 0u32]);
            assert_eq!(sizes(&e), expected_sizes, "{addr:08x}");
            assert_eq!(order(&e), expected, "{addr:08x}");
        }
        // Flee: 0097ce70(2), then the setting's float (quietened), the cover
        // word and 1, 1.
        assert_eq!(calls(&e, 0x0097_ce70), vec![vec![state, 2]]);
        let hide = calls(&e, HIDE_FROM_TARGET);
        assert_eq!(&hide[0][1..], &[0x6600, 0x7fc0_0001, 1, 1]);
        // Hide: the count as a float and 0, 0.
        e.call_log = Some(vec![]);
        e.call(0x0097_d2d0, &args![this, c, 3u32]);
        let hide = calls(&e, HIDE_FROM_TARGET);
        assert_eq!(&hide[0][1..], &[0x6600, 3.0f32.to_bits(), 0, 0]);
        // A failed allocation sets a null procedure.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0097_d2d0, &args![this, c, 3u32]);
        assert!(calls(&e, HIDE_FROM_TARGET).is_empty());
        assert_eq!(calls(&e, SET_MOVEMENT_PROCEDURE), vec![vec![c, 0]]);
    }

    #[test]
    fn flush_target_execute_passes_the_target_and_flag() {
        let (mut e, this, controller, state) = world();
        e.mem.set_u8(state + 0x76, 1);
        e.register(STATE_MOVE_TARGET, |_, _| 0x7700u32.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.call(0x0097_b880, &args![this, controller, 0u32]);
        let procedure = calls(&e, PROCEDURE_009CAEE0);
        assert_eq!(&procedure[0][1..], &[0x7700, 1]);
        assert_eq!(
            calls(&e, SET_ACTION_PROCEDURE),
            vec![vec![controller, procedure[0][0]]]
        );
        assert_eq!(calls(&e, STATE_MOVE_TARGET), vec![vec![state]]);
        assert_eq!(e.call(0x0097_b960, &args![state]).u32(), 1);
    }

    #[test]
    fn move_executes_start_the_movement_procedure() {
        let (mut e, this, controller, state) = world();
        let target = e.mem.alloc(0x40);
        e.register_double(STATE_MOVE_TARGET, move |_, _| target.into_ret());
        e.register(0x004a_7bd0, |_, _| ret_f64(0.1));
        // Flag 1 clear: 009d69f0 with the target, its value and the word.
        e.call(0x0097_bca0, &args![this, controller, 5u32]);
        let procedure = calls(&e, PROCEDURE_009D69F0);
        assert_eq!(&procedure[0][1..], &[target, 0.1f32.to_bits(), 5]);
        assert_eq!(
            calls(&e, SET_MOVEMENT_PROCEDURE),
            vec![vec![controller, procedure[0][0]]]
        );
        assert_eq!(sizes(&e), vec![0x7c]);
        // Flag 1 set: 009d5ad0 with the word.
        e.mem.set_u32(target + 0x1c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0097_bed0, &args![controller, 6u32]);
        assert_eq!(&calls(&e, PROCEDURE_009D5AD0)[0][1..], &[6]);
        assert_eq!(sizes(&e), vec![0x18]);
        // 0097c280 and 0097c620: an action procedure first.
        for (addr, first) in [
            (0x0097_c280, PROCEDURE_009D0890),
            (0x0097_c620, ATTACK_GRENADE),
        ] {
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, controller, 6u32]);
            assert_eq!(
                order(&e),
                vec![
                    first,
                    SET_ACTION_PROCEDURE,
                    PROCEDURE_009D5AD0,
                    SET_MOVEMENT_PROCEDURE
                ]
            );
        }
        // Switch to melee: the move, then weapon 2 as a background procedure.
        e.mem.set_u32(state + 0xc + 8, 0x8800);
        e.call_log = Some(vec![]);
        e.call(0x0097_c090, &args![this, controller, 6u32]);
        let log = procedure_calls(&e);
        assert_eq!(log[2].0, SWITCH_WEAPON);
        assert_eq!(log[2].1[1], 0x8800);
        assert_eq!(
            log[3],
            (ADD_BACKGROUND_PROCEDURE, vec![controller, log[2].1[0]])
        );
        assert_eq!(log[4], (SET_WEAPON, vec![controller, 0x8800]));
        // 0097c790: weapon `index`, none for 6.
        e.call_log = Some(vec![]);
        e.call(0x0097_c790, &args![this, controller, 2u32]);
        e.call(0x0097_c790, &args![this, controller, 6u32]);
        assert_eq!(
            calls(&e, SET_WEAPON),
            vec![vec![controller, 0x8800], vec![controller, 0]]
        );
        assert_eq!(e.call(0x0097_c150, &args![state, 2u32]).u32(), 0x8800);
    }

    #[test]
    fn combat_items_need_an_item() {
        let (mut e, this, controller, state) = world();
        e.call(0x0097_cf10, &args![this, controller, 0x12u32]);
        e.call(0x0097_d060, &args![this, controller, 0x12u32]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 0x11);
        e.mem.set_u32(item + 4, 0x22);
        e.mem.set_u32(state + 0x190 + 4, item);
        e.mem.set_u32(state + 0x1a4 + 4, 0x9900);
        e.call(0x0097_cf10, &args![this, controller, 0x12u32]);
        let activate = calls(&e, ACTIVATE_OBJECT);
        assert_eq!(&activate[0][1..], &[0x11, 0x22]);
        e.call(0x0097_d060, &args![this, controller, 0x12u32]);
        assert_eq!(&calls(&e, USE_COMBAT_ITEM)[0][1..], &[0x9900]);
        assert_eq!(calls(&e, SET_ACTION_PROCEDURE).len(), 2);
        assert_eq!(sizes(&e), vec![0x1c, 0x18]);
        assert_eq!(e.call(0x0097_cfc0, &args![state, 1u32]).u32(), item);
        assert_eq!(e.call(0x0097_d110, &args![state, 1u32]).u32(), 0x9900);
    }

    #[test]
    fn small_accessors() {
        let (mut e, this, controller, state) = world();
        e.mem.set_u32(state + 0xe4, 0x4321);
        assert_eq!(e.call(0x0097_b050, &args![state]).u32(), 0x4321);
        e.mem.set_u32(state + 0x1c, 0b100);
        assert!(e.call(0x0097_be60, &args![state, 0b110u32]).bool());
        assert!(!e.call(0x0097_be60, &args![state, 1u32]).bool());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        for (i, word) in [1u32, 2, 3].into_iter().enumerate() {
            e.mem.set_u32(state + 0x14 + 4 * i as u32, word);
        }
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x0097_be80, &args![state, out]).u32(), out);
        assert_eq!(e.mem.bytes(out, 12), e.mem.bytes(state + 0x14, 12));
        e.mem.set_u8(state + 0x72, 9);
        e.call(0x0097_ccf0, &args![this, controller, 0u32]);
        assert_eq!(e.mem.u8(state + 0x72), 0);
        e.call(0x0097_cd10, &args![state, 7u32]);
        assert_eq!(e.mem.u8(state + 0x72), 7);
    }

    #[test]
    fn debug_texts_print_the_name() {
        let (mut e, this, _, _) = world();
        e.mem.set_u32(MOVE_TEXTS + 4 * 3, 0x3333);
        e.mem.set_u32(WEAPON_TEXTS + 4 * 2, 0x2222);
        e.call(
            0x0097_c520,
            &args![this, 0x5000u32, 0x40u32, 0x0001_0003u32],
        );
        e.call(0x0097_c990, &args![this, 0x5000u32, 0x40u32, 2u32]);
        e.call(0x0097_d370, &args![this, 0x5000u32, 0x40u32, 7u32]);
        assert_eq!(
            calls(&e, PRINT_TEXT),
            vec![
                vec![0x5000, 0x40, NAME_AND_TEXT, 0x0123_4567, 0x3333],
                vec![0x5000, 0x40, NAME_AND_TEXT, 0x0123_4567, 0x2222],
                vec![0x5000, 0x40, NAME_AND_NUMBER, 0x0123_4567, 7],
            ]
        );
    }

    #[test]
    fn can_executes_check_the_base_then_a_location() {
        let (mut e, this, controller, state) = world();
        e.register(BASE_CAN_EXECUTE, |_, a| (a[2] != 0).into_ret());
        e.register(FIND_COMBAT_LOCATION, |_, a| {
            (a[1] == 0xb || a[1] == 4).into_ret()
        });
        let flush = |e: &mut Engine, w: u32| {
            e.call(0x0097_b840, &args![this, controller, w, 1u32])
                .bool()
        };
        assert!(!flush(&mut e, 0));
        assert!(calls(&e, FIND_COMBAT_LOCATION).is_empty());
        assert!(flush(&mut e, 1));
        assert_eq!(calls(&e, FIND_COMBAT_LOCATION), vec![vec![state, 0xb]]);
        assert_eq!(calls(&e, BASE_CAN_EXECUTE)[1], vec![this, controller, 1, 1]);
        let ranged = |e: &mut Engine, w: u32, d: u32| {
            e.call(0x0097_c490, &args![this, controller, w, d]).bool()
        };
        assert!(!ranged(&mut e, 0, 4));
        assert!(ranged(&mut e, 1, 4));
        assert!(!ranged(&mut e, 1, 5));
        assert!(e.call(0x0097_beb0, &args![controller, 0u32, 4u32]).bool());
    }

    #[test]
    fn check_location_identical_compares_the_procedure() {
        let (mut e, this, controller, _) = world();
        const PROCEDURE_VTABLE: u32 = 0x0200_0000;
        let mut slots = [0u32; 14];
        slots[0x34 / 4] = 0x0300_0034;
        e.put_vtable(PROCEDURE_VTABLE, &slots);
        e.register(0x0300_0034, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let procedure = e.mem.alloc(0x40);
        e.mem.set_u32(procedure, PROCEDURE_VTABLE);
        let target = e.mem.alloc(0x40);
        // Words: +0x20 location (00825c00), +0x24 index (0044edb0) and +0x30
        // point (00439090) on the target; +0x20 location, +0x24 00639b40,
        // +0x28 009e32d0 and +0x14 point on the procedure.
        e.register(0x0082_5c00, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(0x0044_edb0, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(0x0063_9b40, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(0x009e_32d0, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0043_9090, |e, a| {
            (e.mem.u32(a[0] + 0x30) != e.mem.u32(a[1])).into_ret()
        });
        e.register(0x004f_d3e0, |e, _| e.mem.u32(0x0108_c000).into_ret());
        e.register_double(STATE_MOVE_TARGET, move |_, _| target.into_ret());
        let check = |e: &mut Engine| {
            e.call(0x0097_bc80, &args![this, controller, 0x77u32])
                .bool()
        };
        // No procedure.
        assert!(!check(&mut e));
        e.mem.set_u32(0x0108_c000, procedure);
        // The target's location differs.
        assert!(!check(&mut e));
        e.mem.set_u32(target + 0x20, 0x77);
        // Kind 6: 00639b40 must match; then 0044edb0 against 009e32d0.
        e.mem.set_u32(procedure + 4, 6);
        assert!(!check(&mut e));
        e.mem.set_u32(procedure + 0x24, 0x77);
        e.mem.set_u32(target + 0x24, 3);
        assert!(!check(&mut e));
        e.mem.set_u32(procedure + 0x28, 3);
        assert!(check(&mut e));
        e.mem.set_u32(target + 0x1c, 1);
        assert!(!check(&mut e));
        e.mem.set_u32(target + 0x1c, 0);
        // Without 0044edb0: the points.
        e.mem.set_u32(target + 0x24, 0);
        e.mem.set_u32(target + 0x30, 5);
        assert!(!check(&mut e));
        e.mem.set_u32(procedure + 0x14, 5);
        assert!(check(&mut e));
        // Kind 4: flag 1 must be set and the locations equal.
        e.mem.set_u32(procedure + 4, 4);
        assert!(!check(&mut e));
        e.mem.set_u32(target + 0x1c, 1);
        assert!(!check(&mut e));
        e.mem.set_u32(procedure + 0x20, 0x77);
        assert!(check(&mut e));
        assert!(e.call(0x0097_bd20, &args![controller, 0x77u32]).bool());
        // Any other kind.
        e.mem.set_u32(procedure + 4, 5);
        assert!(!check(&mut e));
    }
}
