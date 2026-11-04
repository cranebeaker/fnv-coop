//! `fallout shared/run_005ec4d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ed680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed680 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x63 (0x148 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_70ec`) is `BGSRagdoll`, so this is the constructor of `BGSRagdoll`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005ed680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0104_70ec);
    e.mem.set_u32(base + 0x18, 0x0104_70c4);
    e.call(0x005e_d760, &args![base + 0x40]);
    e.call(0x005e_d850, &args![base + 0x90]);
    e.call(0x005e_d910, &args![base + 0xd0]);
    e.call(0x005e_d980, &args![base + 0xf4]);
    e.call(0x005e_d9f0, &args![base + 0x118]);
    e.call(0x004f_15a0, &args![this, 0x63u32]);
    e.call(0x005e_dd00, &args![this]);
    this
}

/// The function `fn_005edaf0` calls on the member (`005eee60`).
const CALLEE_005EEE60: u32 = 0x005e_ee60;

// Translated from 005edaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `005eee60` with `ECX` = `this + 0x40` and passes on nothing (the callee returns no value).
pub fn fn_005edaf0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_005EEE60, &args![this.byte_add(0x40)]);
}

// Translated from 005ec4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0060ffe0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005ec4d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0060_ffe0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005edac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005edc30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005edac0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_dc30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The player character singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The list helpers on a node: its item's slot (`006815c0`) and the next
/// node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The address of the quest's flag byte (`005a8080`, on the quest).
const QUEST_FLAGS: u32 = 0x005a_8080;
/// `HUDMainMenu::ShowQuestObjective` (Xbox PDB, `0077a5b0`, cdecl, three
/// words) and `HUDMainMenu::ShowQuestName` (`0077a480`, cdecl, one word).
const SHOW_QUEST_OBJECTIVE: u32 = 0x0077_a5b0;
const SHOW_QUEST_NAME: u32 = 0x0077_a480;
/// A test on the quest (`0059e400`) that skips the start and the completion.
const QUEST_TEST_0059E400: u32 = 0x0059_e400;
/// `005ec330` on the objective: the first word `ShowQuestObjective` takes.
const OBJECTIVE_TEXT: u32 = 0x005e_c330;
/// The array helpers the ragdoll uses: its element count (`0044ddc0`),
/// element `i`'s address (`00877a30`) and the clear (`008454f0`, one
/// word); `00401030` frees a block.
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_AT: u32 = 0x0087_7a30;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const FREE: u32 = 0x0040_1030;

// Translated from 005ec500 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `007af430` answers 1, walks the list `007d6bb0` answers (until a
/// node's item is null): for each item whose condition (`00717e50` on
/// the item) holds (`TESCondition::IsTrue` with `006101b0(0)` on the item
/// and 0), `00952d60` on the player (`PlayerCharacter::BuildPathToTarget`
/// in the engine map) with `006101b0(1)`, `00460140` and the byte
/// `00610190` of the item, and the item is added to `targets` (`00905820`,
/// given the item's address).
pub fn fn_005ec500(e: &mut Engine, this: Ptr, targets: u32) {
    if e.call(0x007a_f430, &args![this]).u32() != 1 {
        return;
    }
    let mut node = e.call(0x007d_6bb0, &args![this]).u32();
    e.with_stack(4, |e, target_slot| loop {
        if node == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let target = e.mem.u32(slot);
        e.mem.set_u32(target_slot.addr(), target);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let reference = e.call(0x0061_01b0, &args![target, 0u32]).u32();
        let condition = e.call(0x0071_7e50, &args![target]).u32();
        if e.call(0x0068_0c30, &args![condition, reference, 0u32])
            .bool()
        {
            let flag = e.call(0x0061_0190, &args![target]).u8();
            let second = e.call(0x0046_0140, &args![target]).u32();
            let reference = e.call(0x0061_01b0, &args![target, 1u32]).u32();
            let player = e.global::<u32>(PLAYER);
            e.call(
                0x0095_2d60,
                &args![player, reference, second, u32::from(flag)],
            );
            e.call(0x0090_5820, &args![targets, target_slot]);
        }
    });
}

// Translated from 005ec5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets an objective's state (`RET 4`; the engine map has no name):
/// marks its quest (`0044edb0`) changed (vtable +0x48 with 0x20000000).
/// For state 1, or any state while +0x20 is 0: with state 1, unless
/// `0059e400` answers true for the quest, `0060c9c0(1)` on it and, when
/// `005cbb50` on the player answers 0, `PlayerCharacter::SetActiveQuest`;
/// then `HUDMainMenu::ShowQuestObjective(text, 0, 0)` with `005ec330`'s
/// text and, unless bit 0x20 of the quest's flags is set (`005ec6e0`),
/// `HUDMainMenu::ShowQuestName` and the bit set (`005ec700(1)`); then
/// `00952a20` on the player. For state 3 while +0x20 is 1 and `0059e400`
/// answers false: `ShowQuestObjective(text, 1, 0)`. The state is stored at
/// +0x20.
pub fn fn_005ec5d0(e: &mut Engine, this: Ptr, state: u32) {
    let base = this.addr();
    let quest = e.call(0x0044_edb0, &args![this]).u32();
    e.vcall(quest, 0x48, &args![0x2000_0000u32]);
    if state == 1 || e.mem.u32(base + 0x20) == 0 {
        if state == 1 {
            if !e.call(QUEST_TEST_0059E400, &args![quest]).bool() {
                e.call(0x0060_c9c0, &args![quest, 1u32]);
                let player = e.global::<u32>(PLAYER);
                if e.call(0x005c_bb50, &args![player]).u32() == 0 {
                    let player = e.global::<u32>(PLAYER);
                    e.call(0x0095_29d0, &args![player, quest]);
                }
            }
            let text = e.call(OBJECTIVE_TEXT, &args![this]).u32();
            e.call(SHOW_QUEST_OBJECTIVE, &args![text, 0u32, 0u32]);
            if !fn_005ec6e0(e, Ptr::new(quest)) {
                e.call(SHOW_QUEST_NAME, &args![quest]);
                fn_005ec700(e, Ptr::new(quest), 1);
            }
        }
        let player = e.global::<u32>(PLAYER);
        e.call(0x0095_2a20, &args![player, this]);
    }
    if state == 3
        && e.mem.u32(base + 0x20) == 1
        && !e.call(QUEST_TEST_0059E400, &args![quest]).bool()
    {
        let text = e.call(OBJECTIVE_TEXT, &args![this]).u32();
        e.call(SHOW_QUEST_OBJECTIVE, &args![text, 1u32, 0u32]);
    }
    e.mem.set_u32(base + 0x20, state);
}

// Translated from 005ec6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 5 (0x20) of the quest's flag byte (`005a8080`) is set.
pub fn fn_005ec6e0(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.call(QUEST_FLAGS, &args![this]).u32();
    e.mem.u8(flags) & 0x20 != 0
}

// Translated from 005ec700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 5 (0x20) of the quest's flag byte (`005a8080`).
pub fn fn_005ec700(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.call(QUEST_FLAGS, &args![this]).u32();
    let value = e.mem.u8(flags);
    if set != 0 {
        e.mem.set_u8(flags, value | 0x20);
    } else {
        e.mem.set_u8(flags, value & 0xdf);
    }
}

// Translated from 005ed760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the ragdoll's member at +0x40: a cleared byte, the
/// values at +4 (`005ed790`) and `005eee30` on +0x40. Returns `this`.
pub fn fn_005ed760(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    fn_005ed790(e, Ptr::new(this.addr() + 4));
    e.call(0x005e_ee30, &args![this.addr() + 0x40]);
    this
}

/// A `float` constant copied with FLD/FSTP.
fn store_constant(e: &mut Engine, at: u32, constant: u32) {
    let value = e.global::<f32>(constant);
    e.mem.set_f32(at, x87_quiet(value));
}

// Translated from 005ed790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default values of a block of thirteen floats and two words (part of the
/// ragdoll's member at +0x40): the floats from the exe's constants (0.0 at
/// +0x18), then 10000 and 30000. Returns `this`.
pub fn fn_005ed790(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for (offset, constant) in [
        (0x00u32, 0x0101_77e8u32),
        (0x04, 0x0103_0ff0),
        (0x08, 0x0102_31a8),
        (0x0c, 0x0101_df44),
        (0x10, 0x0101_e2bc),
        (0x14, 0x0101_7870),
    ] {
        store_constant(e, base + offset, constant);
    }
    e.mem.set_f32(base + 0x18, 0.0);
    for (offset, constant) in [
        (0x1cu32, 0x0101_b268u32),
        (0x20, 0x0101_b268),
        (0x24, 0x0101_81e8),
        (0x28, 0x0101_81e8),
        (0x2c, 0x0101_b268),
        (0x30, 0x0101_b268),
    ] {
        store_constant(e, base + offset, constant);
    }
    e.mem.set_u32(base + 0x34, 10000);
    e.mem.set_u32(base + 0x38, 30000);
    this
}

// Translated from 005ed850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed850 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of the ragdoll's member at +0x90: a cleared byte, seven
/// zero floats at +4 (`005ed8c0`), `005eede0` on +0x20 and `005eefc0` on
/// +0x30. Returns `this`.
pub fn fn_005ed850(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    fn_005ed8c0(e, Ptr::new(this.addr() + 4));
    e.call(0x005e_ede0, &args![this.addr() + 0x20]);
    e.call(0x005e_efc0, &args![this.addr() + 0x30]);
    this
}

// Translated from 005ed8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Seven floats set to 0.0. Returns `this`.
pub fn fn_005ed8c0(e: &mut Engine, this: Ptr) -> Ptr {
    for i in 0..7 {
        e.mem.set_f32(this.addr() + 4 * i, 0.0);
    }
    this
}

// Translated from 005ed910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed910 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of the ragdoll's member at +0xD0: a cleared byte,
/// `005eede0` on +4 and `005ef1a0` on +0x14. Returns `this`.
pub fn fn_005ed910(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.call(0x005e_ede0, &args![this.addr() + 4]);
    e.call(0x005e_f1a0, &args![this.addr() + 0x14]);
    this
}

// Translated from 005ed980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed980 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of the ragdoll's member at +0xF4: a cleared byte,
/// `005eede0` on +4 and `005ef380` on +0x14. Returns `this`.
pub fn fn_005ed980(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.call(0x005e_ede0, &args![this.addr() + 4]);
    e.call(0x005e_f380, &args![this.addr() + 0x14]);
    this
}

// Translated from 005ed9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ed9f0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of the ragdoll's member at +0x118: a cleared byte, a
/// string at +4 (`004037b0`) and the values at +0xC (`005eda60`). Returns
/// `this`.
pub fn fn_005ed9f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.call(0x0040_37b0, &args![this.addr() + 4]);
    fn_005eda60(e, Ptr::new(this.addr() + 0xc));
    this
}

// Translated from 005eda60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default values: three `u16` set to 0xFFFF, a cleared byte at +6, 0.0 at
/// +8 and +0xC, the constant at `0101e2bc` at +0x10 and 0.0 at +0x14.
/// Returns `this`.
pub fn fn_005eda60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base, 0xffff);
    e.mem.set_u16(base + 2, 0xffff);
    e.mem.set_u16(base + 4, 0xffff);
    e.mem.set_u8(base + 6, 0);
    e.mem.set_f32(base + 8, 0.0);
    e.mem.set_f32(base + 0xc, 0.0);
    store_constant(e, base + 0x10, 0x0101_e2bc);
    e.mem.set_f32(base + 0x14, 0.0);
    this
}

// Translated from 005edb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005edb10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor of the ragdoll's member at +0x90 (built by `005ed850`):
/// `005eeff0` on +0x30, then `005eee10` on +0x20.
pub fn fn_005edb10(e: &mut Engine, this: Ptr) {
    e.call(0x005e_eff0, &args![this.addr() + 0x30]);
    e.call(0x005e_ee10, &args![this.addr() + 0x20]);
}

// Translated from 005edb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005edb70 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor of the ragdoll's member at +0xD0 (built by `005ed910`):
/// `005ef1d0` on +0x14, then `005eee10` on +4.
pub fn fn_005edb70(e: &mut Engine, this: Ptr) {
    e.call(0x005e_f1d0, &args![this.addr() + 0x14]);
    e.call(0x005e_ee10, &args![this.addr() + 4]);
}

// Translated from 005edbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005edbd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor of the ragdoll's member at +0xF4 (built by `005ed980`):
/// `005ef3b0` on +0x14, then `005eee10` on +4.
pub fn fn_005edbd0(e: &mut Engine, this: Ptr) {
    e.call(0x005e_f3b0, &args![this.addr() + 0x14]);
    e.call(0x005e_ee10, &args![this.addr() + 4]);
}

// Translated from 005edc30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005edc30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `BGSRagdoll` (the vtables `005ed680` stores): puts
/// back the vtables, frees the arrays (`005edfb0`), destroys the members
/// at +0x118 (`00591300`), +0xF4, +0xD0, +0x90 and +0x40 (`005edaf0`), the
/// model at +0x18 (`00489070`) and the `TESForm` base (`00483630`).
pub fn fn_005edc30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_70ec);
    e.mem.set_u32(base + 0x18, 0x0104_70c4);
    fn_005edfb0(e, this);
    e.call(0x0059_1300, &args![base + 0x118]);
    fn_005edbd0(e, Ptr::new(base + 0xf4));
    fn_005edb70(e, Ptr::new(base + 0xd0));
    fn_005edb10(e, Ptr::new(base + 0x90));
    fn_005edaf0(e, Ptr::new(base + 0x40));
    e.call(0x0048_9070, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// Frees every element of the array at `array` (`00401030` on each word),
/// then clears the array at `other` and the array itself (`008454f0(1)`).
fn free_elements_and_clear(e: &mut Engine, array: u32, other: u32) {
    let mut i = 0u32;
    while i < e.call(ARRAY_COUNT, &args![array]).u32() {
        let slot = e.call(ARRAY_AT, &args![array, i]).u32();
        let element = e.mem.u32(slot);
        e.call(FREE, &args![element]);
        i = i.wrapping_add(1);
    }
    e.call(ARRAY_CLEAR, &args![other, 1u32]);
    e.call(ARRAY_CLEAR, &args![array, 1u32]);
}

// Translated from 005edd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll`'s defaults (the constructor `005ed680` ends with it): the
/// floats from +0x44 to +0x74 from the exe's constants (1.0 at +0x54, 0.0
/// at +0x5C), the arrays emptied as `005edfb0` does (the six floats at
/// +0x94..+0xAC cleared between the first and second groups), the three
/// `u16` at +0x124 set to 0xFFFF, the members' flag bytes and the words at
/// +0x140 and +0x144 cleared.
pub fn fn_005edd00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, constant) in [
        (0x44u32, 0x0103_0ff0u32),
        (0x48, 0x0104_722c),
        (0x4c, 0x0101_81bc),
        (0x50, 0x0101_8180),
    ] {
        store_constant(e, base + offset, constant);
    }
    e.mem.set_f32(base + 0x54, 1.0);
    store_constant(e, base + 0x58, 0x0101_e2bc);
    e.mem.set_f32(base + 0x5c, 0.0);
    for (offset, constant) in [
        (0x60u32, 0x0101_7718u32),
        (0x64, 0x0101_7870),
        (0x68, 0x0101_7870),
        (0x6c, 0x0101_7718),
        (0x70, 0x0104_7228),
        (0x74, 0x0104_7224),
    ] {
        store_constant(e, base + offset, constant);
    }
    e.call(ARRAY_CLEAR, &args![base + 0x80, 1u32]);
    free_elements_and_clear(e, base + 0xb0, base + 0xc0);
    for offset in [0x94u32, 0x98, 0x9c, 0xa0, 0xa8, 0xac] {
        e.mem.set_f32(base + offset, 0.0);
    }
    free_elements_and_clear(e, base + 0xd4, base + 0xe4);
    free_elements_and_clear(e, base + 0xf8, base + 0x108);
    e.mem.set_u16(base + 0x124, 0xffff);
    e.mem.set_u16(base + 0x126, 0xffff);
    e.mem.set_u16(base + 0x128, 0xffff);
    for offset in [0x40u32, 0x90, 0xd0, 0xf4, 0x118] {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u32(base + 0x144, 0);
    e.mem.set_u32(base + 0x140, 0);
}

// Translated from 005edfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll`'s array release: clears +0x80, frees the elements of the
/// arrays at +0xB0, +0xD4 and +0xF8 and clears each of them with its
/// companion (+0xC0, +0xE4, +0x108).
pub fn fn_005edfb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(ARRAY_CLEAR, &args![base + 0x80, 1u32]);
    free_elements_and_clear(e, base + 0xb0, base + 0xc0);
    free_elements_and_clear(e, base + 0xd4, base + 0xe4);
    free_elements_and_clear(e, base + 0xf8, base + 0x108);
}

/// The string compare the ragdoll parts use on their name arrays
/// (`00408b20`, cdecl: two texts; non-zero when they differ).
const TEXT_COMPARE: u32 = 0x0040_8b20;
/// `strlen` (`0044a670`), allocate (`00401000`) and `strcpy_s`
/// (`00406d30`: destination, size, source), all cdecl.
const TEXT_LENGTH: u32 = 0x0044_a670;
const ALLOCATE: u32 = 0x0040_1000;
const TEXT_COPY: u32 = 0x0040_6d30;
/// The name arrays' resize (`006f2290`, the engine map's
/// `BSSimpleArray<TESObjectWEAP_P_1024>::SetSize`, a folded body;
/// thiscall: count, 1).
const STRING_ARRAY_SET_SIZE: u32 = 0x006f_2290;

/// Whether two `float`s at `offset` differ: `FLD`, `FLD`, `FUCOMPP`,
/// `TEST AH,0x44`: only equal and ordered values count as the same.
fn float_differs(e: &Engine, this: u32, other: u32, offset: u32) -> bool {
    let mine = x87_load32(e.mem.u32(this + offset));
    let theirs = x87_load32(e.mem.u32(other + offset));
    theirs != mine
}

/// The array part of the ragdoll parts' compares: `differs` set when the
/// counts (`0044ddc0`) of the element arrays at `elements` differ, then
/// (while still clear) each element pair (`element_at` on the other's,
/// then on this one's) through `element_differs`; then set when the
/// counts of the name arrays at `names` differ, and (while still clear)
/// each name pair through `00408b20(this one's, the other's)`.
fn arrays_differ(
    e: &mut Engine,
    this: u32,
    other: u32,
    elements: u32,
    element_at: u32,
    element_differs: fn(&mut Engine, Ptr, u32) -> bool,
    names: u32,
) -> bool {
    let mut differs = false;
    let mine = e.call(ARRAY_COUNT, &args![this + elements]).u32();
    if mine != e.call(ARRAY_COUNT, &args![other + elements]).u32() {
        differs = true;
    }
    if !differs {
        let mut i = 0u32;
        while i < e.call(ARRAY_COUNT, &args![this + elements]).u32() && !differs {
            let theirs = e.call(element_at, &args![other + elements, i]).u32();
            let mine = e.call(element_at, &args![this + elements, i]).u32();
            differs = element_differs(e, Ptr::new(mine), theirs);
            i = i.wrapping_add(1);
        }
    }
    let mine = e.call(ARRAY_COUNT, &args![this + names]).u32();
    if mine != e.call(ARRAY_COUNT, &args![other + names]).u32() {
        differs = true;
    }
    if !differs {
        let mut i = 0u32;
        while i < e.call(ARRAY_COUNT, &args![this + names]).u32() && !differs {
            let slot = e.call(ARRAY_AT, &args![other + names, i]).u32();
            let theirs = e.mem.u32(slot);
            let slot = e.call(ARRAY_AT, &args![this + names, i]).u32();
            let mine = e.mem.u32(slot);
            differs = e.call(TEXT_COMPARE, &args![mine, theirs]).u32() != 0;
            i = i.wrapping_add(1);
        }
    }
    differs
}

/// The array part of the ragdoll parts' copies: this one's element array
/// cleared (`008454f0(1)`), its names freed and the name array cleared,
/// both resized to the other's counts (`set_size(count, 1)`,
/// `006f2290(count, 1)`), each element copied by `copy_element(source,
/// destination)` (the addresses from `element_at`), and each name
/// duplicated (`strlen` + 1 bytes from `00401000`, `strcpy_s`).
fn copy_arrays(
    e: &mut Engine,
    this: u32,
    other: u32,
    elements: u32,
    element_at: u32,
    set_size: u32,
    copy_element: fn(&mut Engine, u32, u32),
    names: u32,
) {
    e.call(ARRAY_CLEAR, &args![this + elements, 1u32]);
    let mut i = 0u32;
    while i < e.call(ARRAY_COUNT, &args![this + names]).u32() {
        let slot = e.call(ARRAY_AT, &args![this + names, i]).u32();
        let text = e.mem.u32(slot);
        e.call(FREE, &args![text]);
        i = i.wrapping_add(1);
    }
    e.call(ARRAY_CLEAR, &args![this + names, 1u32]);
    let count = e.call(ARRAY_COUNT, &args![other + elements]).u32();
    e.call(set_size, &args![this + elements, count, 1u32]);
    let count = e.call(ARRAY_COUNT, &args![other + names]).u32();
    e.call(STRING_ARRAY_SET_SIZE, &args![this + names, count, 1u32]);
    let mut i = 0u32;
    while i < e.call(ARRAY_COUNT, &args![other + elements]).u32() {
        let source = e.call(element_at, &args![other + elements, i]).u32();
        let destination = e.call(element_at, &args![this + elements, i]).u32();
        copy_element(e, source, destination);
        i = i.wrapping_add(1);
    }
    let mut i = 0u32;
    while i < e.call(ARRAY_COUNT, &args![other + names]).u32() {
        let slot = e.call(ARRAY_AT, &args![other + names, i]).u32();
        let text = e.mem.u32(slot);
        let size = e.call(TEXT_LENGTH, &args![text]).u32().wrapping_add(1);
        let copy = e.call(ALLOCATE, &args![size]).u32();
        let slot = e.call(ARRAY_AT, &args![other + names, i]).u32();
        let text = e.mem.u32(slot);
        e.call(TEXT_COPY, &args![copy, size, text]);
        let slot = e.call(ARRAY_AT, &args![this + names, i]).u32();
        e.mem.set_u32(slot, copy);
        i = i.wrapping_add(1);
    }
}

/// `REP MOVSD` of `words` words.
fn copy_words(e: &mut Engine, source: u32, destination: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// `005ecb30`'s element copy: 0xE words.
fn copy_foot_element(e: &mut Engine, source: u32, destination: u32) {
    copy_words(e, source, destination, 0xe);
}

/// `005ecfd0`'s element copy: 0xA words.
fn copy_look_element(e: &mut Engine, source: u32, destination: u32) {
    copy_words(e, source, destination, 0xa);
}

/// `005ed360`'s element copy: the word at +0 and the `u16` at +4.
fn copy_grab_element(e: &mut Engine, source: u32, destination: u32) {
    let word = e.mem.u32(source);
    e.mem.set_u32(destination, word);
    let half = e.mem.u16(source + 4);
    e.mem.set_u16(destination + 4, half);
}

// Translated from 005ec760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two 0x38-byte entries of the foot IK data's array (+0x30) are
/// equal (`RET 4`: the other; the engine map has no name): the three
/// `i16` at +0, +2, +4, the byte at +6 and the twelve `float`s at +8 ..
/// +0x34 (unordered counts as different).
pub fn fn_005ec760(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    for offset in [0u32, 2, 4] {
        if e.mem.i16(base + offset) != e.mem.i16(other + offset) {
            return false;
        }
    }
    if e.mem.u8(base + 6) != e.mem.u8(other + 6) {
        return false;
    }
    !(0..12).any(|i| float_differs(e, base, other, 8 + 4 * i))
}

// Translated from 005ec8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two foot IK entries differ (`RET 4`): the opposite of
/// [`fn_005ec760`].
pub fn fn_005ec8f0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    !fn_005ec760(e, this, other)
}

// Translated from 005ec920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ragdoll's foot IK data (its member at +0x90) differs from
/// another's (`RET 4`; `Compare`'s field compare, see `005eeb70`; the
/// engine map has no name): the entry array at +0x30 (`005eedc0`,
/// [`fn_005ec8f0`]) and the name array at +0x20, then the byte at +0 and
/// the seven `float`s at +4 .. +0x1C.
pub fn fn_005ec920(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    if arrays_differ(e, base, other, 0x30, 0x005e_edc0, fn_005ec8f0, 0x20) {
        return true;
    }
    if e.mem.u8(base) != e.mem.u8(other) {
        return true;
    }
    (0..7).any(|i| float_differs(e, base, other, 4 + 4 * i))
}

// Translated from 005ecb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another foot IK data into this one (`RET 4`; `Copy`'s field
/// copy, see `005eeb70`; the engine map has no name): the arrays (entries
/// at +0x30 through `005eedc0`, resized by `005ef010`, 0xE words each;
/// names at +0x20), then the seven `float`s +4 .. +0x1C (FLD/FSTP) and the
/// byte at +0.
pub fn fn_005ecb30(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    copy_arrays(
        e,
        base,
        other,
        0x30,
        0x005e_edc0,
        0x005e_f010,
        copy_foot_element,
        0x20,
    );
    for i in 0..7 {
        let value = x87_quiet_bits(e.mem.u32(other + 4 + 4 * i));
        e.mem.set_u32(base + 4 + 4 * i, value);
    }
    let byte = e.mem.u8(other);
    e.mem.set_u8(base, byte);
}

// Translated from 005ecd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two 0x28-byte entries of the look IK data's array (+0x14) are
/// equal (`RET 4`: the other; the engine map has no name): the `i16` at
/// +0 and +2 and the `float`s at +0x1C, +0x20, +0x24, +0x10, +0x14, +0x18,
/// +4, +8, +0xC (unordered counts as different).
pub fn fn_005ecd10(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    for offset in [0u32, 2] {
        if e.mem.i16(base + offset) != e.mem.i16(other + offset) {
            return false;
        }
    }
    ![0x1cu32, 0x20, 0x24, 0x10, 0x14, 0x18, 4, 8, 0xc]
        .into_iter()
        .any(|offset| float_differs(e, base, other, offset))
}

// Translated from 005ece20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two look IK entries differ (`RET 4`): the opposite of
/// [`fn_005ecd10`].
pub fn fn_005ece20(e: &mut Engine, this: Ptr, other: u32) -> bool {
    !fn_005ecd10(e, this, other)
}

// Translated from 005ece50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ragdoll's look IK data (its member at +0xD0) differs from
/// another's (`RET 4`; see `005eeb70`; the engine map has no name): the
/// entry array at +0x14 (`00691750`, [`fn_005ece20`]) and the name array
/// at +4, then the byte at +0.
pub fn fn_005ece50(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    arrays_differ(e, base, other, 0x14, 0x0069_1750, fn_005ece20, 4)
        || e.mem.u8(base) != e.mem.u8(other)
}

// Translated from 005ecfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another look IK data into this one (`RET 4`; see `005eeb70`;
/// the engine map has no name): the arrays (entries at +0x14 through
/// `00691750`, resized by `005ef1f0`, 0xA words each; names at +4), then
/// the byte at +0.
pub fn fn_005ecfd0(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    copy_arrays(
        e,
        base,
        other,
        0x14,
        0x0069_1750,
        0x005e_f1f0,
        copy_look_element,
        4,
    );
    let byte = e.mem.u8(other);
    e.mem.set_u8(base, byte);
}

// Translated from 005ed160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two 6-byte entries of the grab IK data's array (+0x14) are
/// equal (`RET 4`: the other; the engine map has no name): the `i16` at
/// +0, +2 and +4.
pub fn fn_005ed160(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    [0u32, 2, 4]
        .into_iter()
        .all(|offset| e.mem.i16(base + offset) == e.mem.i16(other + offset))
}

// Translated from 005ed1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two grab IK entries differ (`RET 4`): the opposite of
/// [`fn_005ed160`].
pub fn fn_005ed1b0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    !fn_005ed160(e, this, other)
}

// Translated from 005ed1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ragdoll's grab IK data (its member at +0xF4) differs from
/// another's (`RET 4`; see `005eeb70`; the engine map has no name): the
/// entry array at +0x14 (`0069ccf0`, [`fn_005ed1b0`]) and the name array
/// at +4, then the byte at +0.
pub fn fn_005ed1e0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    arrays_differ(e, base, other, 0x14, 0x0069_ccf0, fn_005ed1b0, 4)
        || e.mem.u8(base) != e.mem.u8(other)
}

// Translated from 005ed360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another grab IK data into this one (`RET 4`; see `005eeb70`;
/// the engine map has no name): the arrays (entries at +0x14 through
/// `0069ccf0`, resized by `005ef3d0`, a word and a `u16` each; names at
/// +4), then the byte at +0.
pub fn fn_005ed360(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    copy_arrays(
        e,
        base,
        other,
        0x14,
        0x0069_ccf0,
        0x005e_f3d0,
        copy_grab_element,
        4,
    );
    let byte = e.mem.u8(other);
    e.mem.set_u8(base, byte);
}

// Translated from 005ed4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the ragdoll's pose matching data (its member at +0x118)
/// differs from another's (`RET 4`; see `005eeb70`; the engine map has no
/// name): the strings at +4 (`00501c90(other's, 1)`, non-zero when they
/// differ), then the byte at +0, the `i16` at +0xC, +0xE, +0x10, the
/// `float`s at +0x14 and +0x1C, the byte at +0x12 and the `float`s at
/// +0x20 and +0x18 (unordered counts as different).
pub fn fn_005ed4f0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    if e.call(0x0050_1c90, &args![base + 4, other + 4, 1u32]).u32() != 0 {
        return true;
    }
    if e.mem.u8(base) != e.mem.u8(other) {
        return true;
    }
    for offset in [0xcu32, 0xe, 0x10] {
        if e.mem.i16(base + offset) != e.mem.i16(other + offset) {
            return true;
        }
    }
    if float_differs(e, base, other, 0x14) || float_differs(e, base, other, 0x1c) {
        return true;
    }
    if e.mem.u8(base + 0x12) != e.mem.u8(other + 0x12) {
        return true;
    }
    float_differs(e, base, other, 0x20) || float_differs(e, base, other, 0x18)
}

// Translated from 005ed5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies another pose matching data into this one (`RET 4`; see
/// `005eeb70`; the engine map has no name): the string at +4
/// (`00501c10`), the byte at +0, the `u16` at +0xC, +0xE, +0x10, the
/// `float`s at +0x14 and +0x1C (FLD/FSTP), the byte at +0x12 and the
/// `float`s at +0x18 and +0x20.
pub fn fn_005ed5f0(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    e.call(0x0050_1c10, &args![base + 4, other + 4]);
    let byte = e.mem.u8(other);
    e.mem.set_u8(base, byte);
    for offset in [0xcu32, 0xe, 0x10] {
        let half = e.mem.u16(other + offset);
        e.mem.set_u16(base + offset, half);
    }
    for offset in [0x14u32, 0x1c] {
        let value = x87_quiet_bits(e.mem.u32(other + offset));
        e.mem.set_u32(base + offset, value);
    }
    let byte = e.mem.u8(other + 0x12);
    e.mem.set_u8(base + 0x12, byte);
    for offset in [0x18u32, 0x20] {
        let value = x87_quiet_bits(e.mem.u32(other + offset));
        e.mem.set_u32(base + offset, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_c760, fn_005ec760(Ptr, u32) -> bool),
        entry!(0x005e_c8f0, fn_005ec8f0(Ptr, u32) -> bool),
        entry!(0x005e_c920, fn_005ec920(Ptr, u32) -> bool),
        entry!(0x005e_cb30, fn_005ecb30(Ptr, u32)),
        entry!(0x005e_cd10, fn_005ecd10(Ptr, u32) -> bool),
        entry!(0x005e_ce20, fn_005ece20(Ptr, u32) -> bool),
        entry!(0x005e_ce50, fn_005ece50(Ptr, u32) -> bool),
        entry!(0x005e_cfd0, fn_005ecfd0(Ptr, u32)),
        entry!(0x005e_d160, fn_005ed160(Ptr, u32) -> bool),
        entry!(0x005e_d1b0, fn_005ed1b0(Ptr, u32) -> bool),
        entry!(0x005e_d1e0, fn_005ed1e0(Ptr, u32) -> bool),
        entry!(0x005e_d360, fn_005ed360(Ptr, u32)),
        entry!(0x005e_d4f0, fn_005ed4f0(Ptr, u32) -> bool),
        entry!(0x005e_d5f0, fn_005ed5f0(Ptr, u32)),
        entry!(0x005e_d680, fn_005ed680(Ptr) -> Ptr),
        entry!(0x005e_daf0, fn_005edaf0(Ptr)),
        entry!(0x005e_c4d0, fn_005ec4d0(Ptr, u32) -> Ptr),
        entry!(0x005e_dac0, fn_005edac0(Ptr, u32) -> Ptr),
        entry!(0x005e_c500, fn_005ec500(Ptr, u32)),
        entry!(0x005e_c5d0, fn_005ec5d0(Ptr, u32)),
        entry!(0x005e_c6e0, fn_005ec6e0(Ptr) -> bool),
        entry!(0x005e_c700, fn_005ec700(Ptr, u8)),
        entry!(0x005e_d760, fn_005ed760(Ptr) -> Ptr),
        entry!(0x005e_d790, fn_005ed790(Ptr) -> Ptr),
        entry!(0x005e_d850, fn_005ed850(Ptr) -> Ptr),
        entry!(0x005e_d8c0, fn_005ed8c0(Ptr) -> Ptr),
        entry!(0x005e_d910, fn_005ed910(Ptr) -> Ptr),
        entry!(0x005e_d980, fn_005ed980(Ptr) -> Ptr),
        entry!(0x005e_d9f0, fn_005ed9f0(Ptr) -> Ptr),
        entry!(0x005e_da60, fn_005eda60(Ptr) -> Ptr),
        entry!(0x005e_db10, fn_005edb10(Ptr)),
        entry!(0x005e_db70, fn_005edb70(Ptr)),
        entry!(0x005e_dbd0, fn_005edbd0(Ptr)),
        entry!(0x005e_dc30, fn_005edc30(Ptr)),
        entry!(0x005e_dd00, fn_005edd00(Ptr)),
        entry!(0x005e_dfb0, fn_005edfb0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005ed680_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_d680,
            size: 0x148,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x005e_d760, &[A::T(0x40)]),
                (0x005e_d850, &[A::T(0x90)]),
                (0x005e_d910, &[A::T(0xd0)]),
                (0x005e_d980, &[A::T(0xf4)]),
                (0x005e_d9f0, &[A::T(0x118)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x63)]),
                (0x005e_dd00, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0104_70ec), (0x18, 4, 0x0104_70c4)],
        });
    }

    #[test]
    fn l68d_005edaf0_passes_this_plus_40_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_005EEE60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_daf0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_005EEE60).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x40]);
    }
    #[test]
    fn at_005ec4d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0060_ffe0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_c4d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_c4d0)
                .collect();
            let mut expected = vec![(0x0060_ffe0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005edac0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_dc30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_dac0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_dac0)
                .collect();
            let mut expected = vec![(0x005e_dc30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod quest_and_ragdoll_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn player(e: &mut Engine) {
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER, 0x0bbb_0000u32);
    }

    #[test]
    fn fn_005ec500_adds_the_targets_whose_condition_holds() {
        let mut e = Engine::new();
        player(&mut e);
        // Nodes: three words (targets 0x100, 0x200, then a null item).
        let nodes = e.mem.alloc(12);
        e.mem.set_u32(nodes, 0x100);
        e.mem.set_u32(nodes + 4, 0x200);
        e.register(0x007a_f430, |_, _| ret(1));
        e.register_double(0x007d_6bb0, move |_, _| ret(nodes));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |_, a| ret(a[0] + 4));
        e.register(0x0061_01b0, |_, a| ret(a[0] + 0x10 + a[1]));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 0x20));
        // Only the second target's condition holds.
        e.register(0x0068_0c30, |_, a| ret(u32::from(a[0] == 0x220)));
        e.register(0x0061_0190, |_, _| ret(0x1ff));
        e.register(0x0046_0140, |_, a| ret(a[0] + 0x30));
        quiet(&mut e, &[0x0095_2d60]);
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(0x0090_5820, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005e_c500, &args![0x4000u32, 0x9000u32]);
        assert_eq!(
            calls(&e, 0x0068_0c30),
            vec![vec![0x120, 0x110, 0], vec![0x220, 0x210, 0]]
        );
        assert_eq!(
            calls(&e, 0x0095_2d60),
            vec![vec![0x0bbb_0000, 0x211, 0x230, 0xff]]
        );
        assert_eq!(*added.borrow(), vec![(0x9000, 0x200)]);

        // Another state: nothing.
        e.register(0x007a_f430, |_, _| ret(2));
        e.call_log = Some(vec![]);
        e.call(0x005e_c500, &args![0x4000u32, 0x9000u32]);
        assert!(calls(&e, 0x007d_6bb0).is_empty());
    }

    /// An objective (+0x20 its state) of a quest with a vtable whose +0x48
    /// is a double and whose flag byte is at +0x40.
    fn objective(e: &mut Engine, state: u32, hidden: bool, active: u32) -> (u32, u32) {
        player(e);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x20, state);
        let quest = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.mem.set_u32(quest, vtable);
        quiet(e, &[0x7000_0048]);
        e.register_double(0x0044_edb0, move |_, _| ret(quest));
        e.register(QUEST_FLAGS, |_, a| ret(a[0] + 0x40));
        if hidden {
            e.register(QUEST_TEST_0059E400, |_, _| ret(1));
        } else {
            e.register(QUEST_TEST_0059E400, |_, _| ret(0));
        }
        e.register_double(0x005c_bb50, move |_, _| ret(active));
        e.register(OBJECTIVE_TEXT, |_, _| ret(0x7777));
        quiet(
            e,
            &[
                0x0060_c9c0,
                0x0095_29d0,
                SHOW_QUEST_OBJECTIVE,
                SHOW_QUEST_NAME,
                0x0095_2a20,
            ],
        );
        (this, quest)
    }

    #[test]
    fn fn_005ec5d0_displays_a_new_objective() {
        let mut e = Engine::new();
        let (this, quest) = objective(&mut e, 0, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_c5d0, &args![this, 1u32]);
        assert_eq!(calls(&e, 0x7000_0048), vec![vec![quest, 0x2000_0000]]);
        assert_eq!(calls(&e, 0x0060_c9c0), vec![vec![quest, 1]]);
        assert_eq!(calls(&e, 0x0095_29d0), vec![vec![0x0bbb_0000, quest]]);
        assert_eq!(calls(&e, SHOW_QUEST_OBJECTIVE), vec![vec![0x7777, 0, 0]]);
        assert_eq!(calls(&e, SHOW_QUEST_NAME), vec![vec![quest]]);
        assert_eq!(e.mem.u8(quest + 0x40), 0x20);
        assert_eq!(calls(&e, 0x0095_2a20), vec![vec![0x0bbb_0000, this]]);
        assert_eq!(e.mem.u32(this + 0x20), 1);

        // Again, with the name shown and an active quest: neither.
        e.call_log = Some(vec![]);
        e.register(0x005c_bb50, |_, _| ret(5));
        e.call(0x005e_c5d0, &args![this, 1u32]);
        assert!(calls(&e, 0x0095_29d0).is_empty());
        assert!(calls(&e, SHOW_QUEST_NAME).is_empty());
        assert_eq!(calls(&e, SHOW_QUEST_OBJECTIVE).len(), 1);
    }

    #[test]
    fn fn_005ec5d0_other_states() {
        // A hidden quest: no start, no completion display.
        let mut e = Engine::new();
        let (this, _) = objective(&mut e, 0, true, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_c5d0, &args![this, 1u32]);
        assert!(calls(&e, 0x0060_c9c0).is_empty());
        assert_eq!(calls(&e, SHOW_QUEST_OBJECTIVE).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x005e_c5d0, &args![this, 3u32]);
        assert!(calls(&e, SHOW_QUEST_OBJECTIVE).is_empty());
        assert!(calls(&e, 0x0095_2a20).is_empty());
        assert_eq!(e.mem.u32(this + 0x20), 3);

        // Completing a displayed objective of a shown quest.
        let mut e = Engine::new();
        let (this, _) = objective(&mut e, 1, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_c5d0, &args![this, 3u32]);
        assert_eq!(calls(&e, SHOW_QUEST_OBJECTIVE), vec![vec![0x7777, 1, 0]]);
        assert!(calls(&e, 0x0095_2a20).is_empty());

        // From state 0 to 2: only the player is told.
        let mut e = Engine::new();
        let (this, _) = objective(&mut e, 0, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_c5d0, &args![this, 2u32]);
        assert!(calls(&e, SHOW_QUEST_OBJECTIVE).is_empty());
        assert_eq!(calls(&e, 0x0095_2a20), vec![vec![0x0bbb_0000, this]]);
    }

    #[test]
    fn fn_005ec6e0_and_005ec700_use_bit_0x20() {
        let mut e = Engine::new();
        let quest = e.mem.alloc(0x80);
        e.register(QUEST_FLAGS, |_, a| ret(a[0] + 0x40));
        e.mem.set_u8(quest + 0x40, 0xdf);
        assert!(!e.call(0x005e_c6e0, &args![quest]).bool());
        e.call(0x005e_c700, &args![quest, 0x100u32 + 1]);
        assert_eq!(e.mem.u8(quest + 0x40), 0xff);
        assert!(e.call(0x005e_c6e0, &args![quest]).bool());
        // Only the low byte of the flag word counts.
        e.call(0x005e_c700, &args![quest, 0x100u32]);
        assert_eq!(e.mem.u8(quest + 0x40), 0xdf);
    }

    fn constants(e: &mut Engine) {
        for page in [
            0x0101_7000u32,
            0x0101_8000,
            0x0101_b000,
            0x0101_d000,
            0x0101_e000,
            0x0102_3000,
            0x0103_0000,
            0x0104_7000,
        ] {
            e.map(page, 0x1000);
        }
        let values = [
            (0x0101_77e8u32, 1.0f32),
            (0x0103_0ff0, 2.0),
            (0x0102_31a8, 3.0),
            (0x0101_df44, 4.0),
            (0x0101_e2bc, 5.0),
            (0x0101_7870, 6.0),
            (0x0101_b268, 7.0),
            (0x0101_81e8, 8.0),
            (0x0104_722c, 9.0),
            (0x0101_81bc, 10.0),
            (0x0101_8180, 11.0),
            (0x0101_7718, 12.0),
            (0x0104_7228, 13.0),
            (0x0104_7224, 14.0),
        ];
        for (address, value) in values {
            e.set_global(address, value);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn fn_005ed760_and_005ed790_set_the_defaults() {
        let mut e = Engine::new();
        constants(&mut e);
        quiet(&mut e, &[0x005e_ee30]);
        let this = e.mem.alloc(0x50);
        e.mem.set_u8(this, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_d760, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(
            floats(&e, this + 4, 13),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 7.0, 7.0, 8.0, 8.0, 7.0, 7.0]
        );
        assert_eq!(e.mem.u32(this + 4 + 0x34), 10000);
        assert_eq!(e.mem.u32(this + 4 + 0x38), 30000);
        assert_eq!(calls(&e, 0x005e_ee30), vec![vec![this + 0x40]]);
        assert_eq!(e.call(0x005e_d790, &args![this]).u32(), this);
    }

    #[test]
    fn the_member_constructors_build_their_parts() {
        let mut e = Engine::new();
        constants(&mut e);
        quiet(
            &mut e,
            &[
                0x005e_ede0,
                0x005e_efc0,
                0x005e_f1a0,
                0x005e_f380,
                0x0040_37b0,
            ],
        );
        let this = e.mem.alloc(0x40);
        for i in 0..0x10 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_d850, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(floats(&e, this + 4, 7), vec![0.0; 7]);
        assert_eq!(calls(&e, 0x005e_ede0), vec![vec![this + 0x20]]);
        assert_eq!(calls(&e, 0x005e_efc0), vec![vec![this + 0x30]]);
        assert_eq!(e.call(0x005e_d8c0, &args![this]).u32(), this);

        e.call_log = Some(vec![]);
        e.call(0x005e_d910, &args![this]);
        assert_eq!(calls(&e, 0x005e_ede0), vec![vec![this + 4]]);
        assert_eq!(calls(&e, 0x005e_f1a0), vec![vec![this + 0x14]]);
        e.call_log = Some(vec![]);
        e.call(0x005e_d980, &args![this]);
        assert_eq!(calls(&e, 0x005e_ede0), vec![vec![this + 4]]);
        assert_eq!(calls(&e, 0x005e_f380), vec![vec![this + 0x14]]);

        for i in 0..0x10 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_d9f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(calls(&e, 0x0040_37b0), vec![vec![this + 4]]);
        let values = this + 0xc;
        assert_eq!(
            e.mem.bytes(values, 7),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0]
        );
        assert_eq!(e.mem.u8(values + 7), 0xff);
        assert_eq!(floats(&e, values + 8, 4), vec![0.0, 0.0, 5.0, 0.0]);
        assert_eq!(e.call(0x005e_da60, &args![values]).u32(), values);
    }

    #[test]
    fn the_member_destructors_destroy_their_parts() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[0x005e_eff0, 0x005e_ee10, 0x005e_f1d0, 0x005e_f3b0],
        );
        e.call_log = Some(vec![]);
        e.call(0x005e_db10, &args![0x4000u32]);
        e.call(0x005e_db70, &args![0x5000u32]);
        e.call(0x005e_dbd0, &args![0x6000u32]);
        assert_eq!(calls(&e, 0x005e_eff0), vec![vec![0x4030]]);
        assert_eq!(calls(&e, 0x005e_f1d0), vec![vec![0x5014]]);
        assert_eq!(calls(&e, 0x005e_f3b0), vec![vec![0x6014]]);
        assert_eq!(
            calls(&e, 0x005e_ee10),
            vec![vec![0x4020], vec![0x5004], vec![0x6004]]
        );
    }

    /// Array doubles: every array holds two elements, its low twelve
    /// address bits times 0x100 and one more.
    fn arrays(e: &mut Engine) {
        e.register(ARRAY_COUNT, |_, _| ret(2));
        e.register(ARRAY_AT, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, (a[0] & 0xfff) * 0x100 + a[1]);
            ret(slot)
        });
        quiet(e, &[ARRAY_CLEAR, FREE]);
    }

    #[test]
    fn fn_005edfb0_frees_and_clears_the_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        arrays(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_dfb0, &args![this]);
        let low = this & 0xfff;
        let freed: Vec<u32> = calls(&e, FREE).iter().map(|c| c[0]).collect();
        let expected: Vec<u32> = [0xb0u32, 0xd4, 0xf8]
            .iter()
            .flat_map(|o| [((low + o) & 0xfff) * 0x100, ((low + o) & 0xfff) * 0x100 + 1])
            .collect();
        assert_eq!(freed, expected);
        let cleared: Vec<u32> = calls(&e, ARRAY_CLEAR).iter().map(|c| c[0] - this).collect();
        assert_eq!(cleared, vec![0x80, 0xc0, 0xb0, 0xe4, 0xd4, 0x108, 0xf8]);
        assert!(calls(&e, ARRAY_CLEAR).iter().all(|c| c[1] == 1));
    }

    #[test]
    fn fn_005edd00_sets_the_ragdoll_defaults() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = e.mem.alloc(0x150);
        for i in 0..(0x150 / 4) {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        arrays(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_dd00, &args![this]);
        assert_eq!(
            floats(&e, this + 0x44, 13),
            vec![2.0, 9.0, 10.0, 11.0, 1.0, 5.0, 0.0, 12.0, 6.0, 6.0, 12.0, 13.0, 14.0]
        );
        assert_eq!(floats(&e, this + 0x94, 4), vec![0.0; 4]);
        assert_eq!(e.mem.u32(this + 0xa4), 0xffff_ffff);
        assert_eq!(floats(&e, this + 0xa8, 2), vec![0.0; 2]);
        assert_eq!(calls(&e, FREE).len(), 6);
        let cleared: Vec<u32> = calls(&e, ARRAY_CLEAR).iter().map(|c| c[0] - this).collect();
        assert_eq!(cleared, vec![0x80, 0xc0, 0xb0, 0xe4, 0xd4, 0x108, 0xf8]);
        assert_eq!(e.mem.bytes(this + 0x124, 6), vec![0xff; 6]);
        for offset in [0x40u32, 0x90, 0xd0, 0xf4, 0x118] {
            assert_eq!(e.mem.u8(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x41), 0xff);
        assert_eq!(e.mem.u32(this + 0x140), 0);
        assert_eq!(e.mem.u32(this + 0x144), 0);
    }

    #[test]
    fn fn_005edc30_destroys_the_ragdoll() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        arrays(&mut e);
        quiet(
            &mut e,
            &[
                0x0059_1300,
                0x005e_f3b0,
                0x005e_f1d0,
                0x005e_eff0,
                0x005e_ee10,
                CALLEE_005EEE60,
                0x0048_9070,
                0x0048_3630,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x005e_dc30, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0104_70ec);
        assert_eq!(e.mem.u32(this + 0x18), 0x0104_70c4);
        assert_eq!(calls(&e, FREE).len(), 6);
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x118]]);
        assert_eq!(calls(&e, 0x005e_f3b0), vec![vec![this + 0xf4 + 0x14]]);
        assert_eq!(calls(&e, 0x005e_f1d0), vec![vec![this + 0xd0 + 0x14]]);
        assert_eq!(calls(&e, 0x005e_eff0), vec![vec![this + 0x90 + 0x30]]);
        assert_eq!(calls(&e, CALLEE_005EEE60), vec![vec![this + 0x80]]);
        assert_eq!(calls(&e, 0x0048_9070), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
        let log = e.call_log.as_ref().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [0x0059_1300, 0x005e_f3b0, 0x0048_3630].contains(a))
            .collect();
        assert_eq!(order, vec![0x0059_1300, 0x005e_f3b0, 0x0048_3630]);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Arrays as `{count, data}`; the element getters index `data` by
    /// their stride; resizes allocate fresh data; a text is a block whose
    /// first word is its id (`strlen` answers 3).
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(0x005e_edc0, |e, a| ret(e.mem.u32(a[0] + 4) + 0x38 * a[1]));
        e.register(0x0069_1750, |e, a| ret(e.mem.u32(a[0] + 4) + 0x28 * a[1]));
        e.register(0x0069_ccf0, |e, a| ret(e.mem.u32(a[0] + 4) + 6 * a[1]));
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        for (addr, stride) in [
            (0x005e_f010u32, 0x38u32),
            (0x005e_f1f0, 0x28),
            (0x005e_f3d0, 6),
            (STRING_ARRAY_SET_SIZE, 4),
        ] {
            e.register_double(addr, move |e, a| {
                assert_eq!(a[2], 1);
                let data = e.mem.alloc((a[1] * stride).max(4));
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(a[0] + 4, data);
                Ret::default()
            });
        }
        e.register(TEXT_COMPARE, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])))
        });
        e.register(TEXT_LENGTH, |_, _| ret(3));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(TEXT_COPY, |e, a| {
            assert_eq!(a[1], 4);
            let id = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], id);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
    }

    /// An array at `array` with `count` elements of `stride` bytes, each
    /// filled with `fill` words.
    fn array(e: &mut Engine, array: u32, count: u32, stride: u32, fill: u32) {
        let data = e.mem.alloc((count * stride).max(4));
        for i in 0..count * stride / 4 {
            e.mem.set_u32(data + 4 * i, fill);
        }
        e.mem.set_u32(array, count);
        e.mem.set_u32(array + 4, data);
    }

    /// A name array with texts whose ids are `ids`.
    fn names(e: &mut Engine, array_at: u32, ids: &[u32]) {
        array(e, array_at, ids.len() as u32, 4, 0);
        let data = e.mem.u32(array_at + 4);
        for (i, &id) in ids.iter().enumerate() {
            let text = e.mem.alloc(4);
            e.mem.set_u32(text, id);
            e.mem.set_u32(data + 4 * i as u32, text);
        }
    }

    #[test]
    fn entry_compares() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x38);
        let b = e.mem.alloc(0x38);
        assert!(e.call(0x005e_c760, &args![a, b]).bool());
        assert!(!e.call(0x005e_c8f0, &args![a, b]).bool());
        assert!(e.call(0x005e_cd10, &args![a, b]).bool());
        assert!(e.call(0x005e_d160, &args![a, b]).bool());
        for (offset, value) in [(2u32, 1u32), (6, 1), (0x34, 0x3f80_0000)] {
            e.mem.set_u32(b + offset, value);
            assert!(!e.call(0x005e_c760, &args![a, b]).bool(), "{offset:x}");
            assert!(e.call(0x005e_c8f0, &args![a, b]).bool());
            e.mem.set_u32(b + offset, 0);
        }
        // Negative zero is equal, NaN never.
        e.mem.set_u32(b + 0x24, 0x8000_0000);
        assert!(e.call(0x005e_cd10, &args![a, b]).bool());
        e.mem.set_u32(a + 0x24, 0x7fc0_0000);
        e.mem.set_u32(b + 0x24, 0x7fc0_0000);
        assert!(!e.call(0x005e_cd10, &args![a, b]).bool());
        assert!(e.call(0x005e_ce20, &args![a, b]).bool());
        // 005ecd10 ignores the bytes after +0x28, 005ed160 after +6.
        e.mem.set_u32(a + 0x24, 0);
        e.mem.set_u32(b + 0x24, 0);
        e.mem.set_u32(b + 0x30, 5);
        assert!(e.call(0x005e_cd10, &args![a, b]).bool());
        e.mem.set_u16(b + 4, 5);
        assert!(!e.call(0x005e_d160, &args![a, b]).bool());
        assert!(e.call(0x005e_d1b0, &args![a, b]).bool());
    }

    /// Two foot IK data blocks alike: two entries and two names each.
    fn foot_pair(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        for block in [this, other] {
            array(e, block + 0x30, 2, 0x38, 0);
            names(e, block + 0x20, &[7, 8]);
        }
        (this, other)
    }

    #[test]
    fn fn_005ec920_compares_foot_ik_data() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let (this, other) = foot_pair(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_c920, &args![this, other]).bool());
        let other_data = e.mem.u32(other + 0x34);
        let this_data = e.mem.u32(this + 0x34);
        assert_eq!(
            calls(&e, 0x005e_edc0)[..2],
            [vec![other + 0x30, 0], vec![this + 0x30, 0]]
        );
        assert_eq!(e.mem.u32(other_data), 0);
        assert_eq!(e.mem.u32(this_data), 0);
        assert_eq!(calls(&e, TEXT_COMPARE).len(), 2);
        // An entry, a name, the byte or a float differs.
        e.mem.set_u32(other_data + 0x38 + 8, 0x3f80_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        assert!(calls(&e, TEXT_COMPARE).is_empty());
        let (this, other) = foot_pair(&mut e);
        names(&mut e, other + 0x20, &[7, 9]);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        let (this, other) = foot_pair(&mut e);
        names(&mut e, other + 0x20, &[7]);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        let (this, other) = foot_pair(&mut e);
        e.mem.set_u8(other, 1);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        let (this, other) = foot_pair(&mut e);
        e.mem.set_u32(other + 0x1c, 0x3f80_0000);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        // Different entry counts: the name counts are still compared.
        let (this, other) = foot_pair(&mut e);
        array(&mut e, other + 0x30, 1, 0x38, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_c920, &args![this, other]).bool());
        assert_eq!(calls(&e, ARRAY_COUNT).len(), 4);
    }

    #[test]
    fn fn_005ecb30_copies_foot_ik_data() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        array(&mut e, this + 0x30, 1, 0x38, 0);
        names(&mut e, this + 0x20, &[1, 2, 3]);
        array(&mut e, other + 0x30, 2, 0x38, 0x1111);
        names(&mut e, other + 0x20, &[5, 6]);
        e.mem.set_u8(other, 9);
        e.mem.set_u32(other + 0x1c, 0x7f80_0001);
        e.mem.set_f32(other + 4, 1.5);
        e.call_log = Some(vec![]);
        e.call(0x005e_cb30, &args![this, other]);
        assert_eq!(calls(&e, FREE).len(), 3);
        assert_eq!(e.mem.u32(this + 0x30), 2);
        let data = e.mem.u32(this + 0x34);
        assert_eq!(e.mem.u32(data + 0x38 + 0x34), 0x1111);
        assert_eq!(e.mem.u32(this + 0x20), 2);
        let texts = e.mem.u32(this + 0x24);
        let first = e.mem.u32(texts);
        assert_eq!(e.mem.u32(first), 5);
        assert_eq!(e.mem.u32(e.mem.u32(texts + 4)), 6);
        assert_ne!(first, e.mem.u32(e.mem.u32(other + 0x24)));
        assert_eq!(e.mem.u8(this), 9);
        assert_eq!(e.mem.u32(this + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 4), 1.5);
        assert_eq!(
            calls(&e, ARRAY_CLEAR),
            vec![vec![this + 0x30, 1], vec![this + 0x20, 1]]
        );
    }

    #[test]
    fn look_and_grab_ik_data() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x30);
        let other = e.mem.alloc(0x30);
        array(&mut e, other + 0x14, 2, 0x28, 0x2222);
        names(&mut e, other + 4, &[4]);
        e.mem.set_u8(other, 3);
        array(&mut e, this + 0x14, 0, 0x28, 0);
        names(&mut e, this + 4, &[]);
        assert!(e.call(0x005e_ce50, &args![this, other]).bool());
        e.call(0x005e_cfd0, &args![this, other]);
        let data = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(data + 0x4c), 0x2222);
        assert_eq!(e.mem.u8(this), 3);
        assert!(!e.call(0x005e_ce50, &args![this, other]).bool());
        e.mem.set_u8(this, 4);
        assert!(e.call(0x005e_ce50, &args![this, other]).bool());

        let this = e.mem.alloc(0x30);
        let other = e.mem.alloc(0x30);
        array(&mut e, other + 0x14, 2, 6, 0);
        let source = e.mem.u32(other + 0x18);
        e.mem.set_u32(source + 6, 0x0003_0002);
        e.mem.set_u16(source + 10, 4);
        names(&mut e, other + 4, &[4]);
        array(&mut e, this + 0x14, 0, 6, 0);
        names(&mut e, this + 4, &[]);
        e.call(0x005e_d360, &args![this, other]);
        let data = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(data + 6), 0x0003_0002);
        assert_eq!(e.mem.u16(data + 10), 4);
        assert!(!e.call(0x005e_d1e0, &args![this, other]).bool());
        e.mem.set_u16(data + 10, 5);
        assert!(e.call(0x005e_d1e0, &args![this, other]).bool());
    }

    #[test]
    fn pose_matching_data_compare_and_copy() {
        let mut e = Engine::new();
        e.register(0x0050_1c90, |e, a| {
            assert_eq!(a[2], 1);
            ret(u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])))
        });
        e.register(0x0050_1c10, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let this = e.mem.alloc(0x24);
        let other = e.mem.alloc(0x24);
        for (offset, value) in [
            (0u32, 0x0000_0007u32),
            (4, 0x55),
            (0xc, 0x0002_0001),
            (0x10, 0x0009_0003),
            (0x14, 0x3f80_0000),
            (0x18, 0x7f80_0001),
            (0x1c, 0x4000_0000),
            (0x20, 0x4040_0000),
        ] {
            e.mem.set_u32(other + offset, value);
        }
        assert!(e.call(0x005e_d4f0, &args![this, other]).bool());
        e.call(0x005e_d5f0, &args![this, other]);
        assert_eq!(e.mem.u8(this), 7);
        assert_eq!(e.mem.u32(this + 4), 0x55);
        assert_eq!(e.mem.u32(this + 0xc), 0x0002_0001);
        assert_eq!(e.mem.u16(this + 0x10), 3);
        assert_eq!(e.mem.u8(this + 0x12), 9);
        assert_eq!(e.mem.u32(this + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x20), 0x4040_0000);
        // The NaN at +0x18 is never equal.
        assert!(e.call(0x005e_d4f0, &args![this, other]).bool());
        e.mem.set_u32(this + 0x18, 0);
        e.mem.set_u32(other + 0x18, 0);
        assert!(!e.call(0x005e_d4f0, &args![this, other]).bool());
        for (offset, value) in [(0u32, 8u32), (0xe, 1), (0x12, 1), (0x1d, 1)] {
            let old = e.mem.u32(other + offset);
            e.mem.set_u8(other + offset, value as u8);
            assert!(
                e.call(0x005e_d4f0, &args![this, other]).bool(),
                "{offset:x}"
            );
            e.mem.set_u32(other + offset, old);
        }
    }
}
