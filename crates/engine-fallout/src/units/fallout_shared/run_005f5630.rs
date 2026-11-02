//! `fallout shared/run_005f5630` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f5760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address 0x54 bytes into the object (`ECX`, no stack words). The
/// engine map gives it no name. `TESQuest`'s `InitItem` (`0060cba0`),
/// `Save` and `Load` (`0060ceb0`) use it for the quest's condition list
/// (`TESCondition` at `quest + 0x54`); other functions call it as well.
pub fn fn_005f5760(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x54)
}

// Translated from 005f65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the singly linked list starting at node `this` (value word at
/// `+0`, next node at `+4`; `this` may be null) contains a node whose value
/// equals the word at `wanted`. `__thiscall`, one stack word (`RET 4`).
pub fn fn_005f65d0(e: &mut Engine, this: Ptr, wanted: Ptr) -> bool {
    let mut node = this.addr();
    while node != 0 && e.mem.u32(node) != e.mem.u32(wanted.addr()) {
        node = e.mem.u32(node + 4);
    }
    node != 0
}

// Translated from 005f6610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f6610 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 7 (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005f6610(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_e270, &args![base + 0x2c]);
    e.call(0x0047_ff70, &args![base + 0x38]);
    e.mem.set_u32(base, 0x0104_8bb4);
    e.mem.set_u32(base + 0x18, 0x0104_8ba0);
    e.mem.set_u32(base + 0x24, 0x0104_8b88);
    e.mem.set_u32(base + 0x2c, 0x0104_8b68);
    e.mem.set_u32(base + 0x38, 0x0104_8b54);
    e.call(0x004f_15a0, &args![this, 0x7]);
    e.call(0x005f_67c0, &args![this]);
    this
}

/// `memset` (cdecl: destination, value, count).
const MEMSET: u32 = 0x0040_3d30;
/// `TESClass::SetTagSkill` (engine map): `__thiscall`, index and skill.
const TES_CLASS_SET_TAG_SKILL: u32 = 0x005f_6e80;
/// The 14 lists (8 bytes each) `fn_005f5880` puts the forms of each kind
/// into when their checks pass, and the 14 it puts the others into.
const ACTIVE_LISTS: u32 = 0x011c_b3e0;
const OTHER_LISTS: u32 = 0x011c_b450;
/// Number of kinds (lists in each group).
const KINDS: i32 = 0xe;
/// Empties a list (`__thiscall`, no stack words).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Adds the word at the pointer argument to a list (`__thiscall`, `RET 4`).
const LIST_ADD: u32 = 0x005a_e3d0;
/// The global object whose form list `fn_005f5880` walks, and the function
/// that returns that list's first node (`__thiscall` on it).
const FORM_LIST_OWNER: u32 = 0x011c_3f2c;
const FIRST_FORM_NODE: u32 = 0x0046_1150;
/// The address of a list node's data word.
const NODE_DATA: u32 = 0x0068_15c0;
/// A list node's next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// Form tests both walks make (`__thiscall`, no stack words; each reads
/// the word at `form + 0x70`).
const FORM_TEST_005A6150: u32 = 0x005a_6150;
const FORM_TEST_005A60D0: u32 = 0x005a_60d0;
/// The word at `form + 0x54`: the form's kind.
const FORM_KIND: u32 = 0x006a_1b60;

// Translated from 005f5930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the 16-bit word at `this + 0x5c` (`ECX`, no stack words). The
/// engine map gives it no name.
pub fn fn_005f5930(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 0x5c)) & 1
}

// Translated from 005f5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x60` (`ECX`, no stack words). The engine map gives
/// it no name.
pub fn fn_005f5f80(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x60)
}

// Translated from 005f5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x64`, zero-extended (`ECX`, no stack
/// words). The engine map gives it no name.
pub fn fn_005f5fa0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x64)
}

// Translated from 005f5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x66`, zero-extended (`ECX`, no stack
/// words). The engine map gives it no name.
pub fn fn_005f5fc0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x66)
}

// Translated from 005f5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x68`, zero-extended (`ECX`, no stack
/// words). The engine map gives it no name.
pub fn fn_005f5fe0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x68)
}

// Translated from 005f5800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's virtual slot +0x48 with 2 (`ECX`, no stack words).
/// The engine map gives it no name.
pub fn fn_005f5800(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 005f60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the word at `this + 0x6c` (wrapping, as `ADD`), then
/// runs `fn_005f5800`. `__thiscall`, `RET 4`. The engine map gives it no
/// name.
pub fn fn_005f60e0(e: &mut Engine, this: Ptr, amount: u32) {
    let at = this.addr() + 0x6c;
    let value = e.mem.u32(at).wrapping_add(amount);
    e.mem.set_u32(at, value);
    fn_005f5800(e, this);
}

// Translated from 005f5780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object into `buffer`: `00484d60` (tesform.cpp) on `this` with
/// the buffer, then the words at `this + 0x6c` and `this + 0x70` through
/// `00865e50` (bgssavegamebuffer.cpp: `__thiscall` on the buffer, address,
/// size 4, 0). `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_005f5780(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0048_4d60, &args![this, buffer]);
    e.call(0x0086_5e50, &args![buffer, this.addr() + 0x6c, 4u32, 0u32]);
    e.call(0x0086_5e50, &args![buffer, this.addr() + 0x70, 4u32, 0u32]);
}

// Translated from 005f67c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the 0x1c bytes at `this + 0x44` (`memset`) and sets the four tag
/// skills to -1 (`TESClass::SetTagSkill(i, -1)` for `i` in 0..4). `ECX`, no
/// stack words. The engine map gives it no name; `fn_005f6610` (the type-7
/// constructor) runs it last.
pub fn fn_005f67c0(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + 0x44, 0u32, 0x1cu32]);
    for index in 0..4u32 {
        e.call(TES_CLASS_SET_TAG_SKILL, &args![this, index, 0xffff_ffffu32]);
    }
}

// Translated from 005f6710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f6710 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The destructor body of the type-7 form `fn_005f6610` constructs: it
/// stores the vtables back (`01048bb4`, and the components' at +0x18,
/// +0x24, +0x2c, +0x38), then runs the component destructors `0047ffa0`
/// (+0x38), `0048e2e0` (+0x2c), `00591300` (+0x18) and
/// `TESForm::~TESForm` (`00483630`, engine map) on `this`. Nothing runs for
/// the component at +0x24. `ECX`, no stack words. The exception-unwinding
/// frame is not translated.
pub fn fn_005f6710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_8bb4);
    e.mem.set_u32(base + 0x18, 0x0104_8ba0);
    e.mem.set_u32(base + 0x24, 0x0104_8b88);
    e.mem.set_u32(base + 0x2c, 0x0104_8b68);
    e.mem.set_u32(base + 0x38, 0x0104_8b54);
    e.call(0x0047_ffa0, &args![base + 0x38]);
    e.call(0x0048_e2e0, &args![base + 0x2c]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005f66e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type-7 form: runs `fn_005f6710` on
/// `this`, then frees `this` with `00401030` when bit 0 of `flags` is set.
/// Returns `this`. The engine map gives it no name.
pub fn fn_005f66e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005f6710(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005f5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the 14 lists at `011cb3e0` and the 14 at `011cb450` (8 bytes
/// each; `00470470`), pairwise, kind 0 first. No arguments. The engine map
/// gives it no name.
pub fn fn_005f5840(e: &mut Engine) {
    for kind in 0..KINDS as u32 {
        e.call(LIST_CLEAR, &args![ACTIVE_LISTS + kind * 8]);
        e.call(LIST_CLEAR, &args![OTHER_LISTS + kind * 8]);
    }
}

/// Adds `form` to the list of its kind (`fn_005f5880`): `005ae3d0` on
/// `lists + kind * 8` with the address of a word holding the form.
fn add_to_kind_list(e: &mut Engine, lists: u32, form: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        let kind = e.call(FORM_KIND, &args![form]).u32();
        let list = lists.wrapping_add(kind.wrapping_mul(8));
        e.call(LIST_ADD, &args![list, slot]);
    });
}

// Translated from 005f5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the 28 kind lists: empties them (`fn_005f5840`), then walks
/// the form list of the object at `[011c3f2c]` (`00461150` its first node)
/// and adds each form to the list of its kind (`006a1b60`, the word at
/// `form + 0x54`): to the lists at `011cb3e0` when `005a6150` answers 0
/// and either `fn_005f5930` answers 0 or `005a60d0` answers non-zero,
/// otherwise to the lists at `011cb450`. Null entries are skipped. No
/// arguments. The engine map gives it no name.
pub fn fn_005f5880(e: &mut Engine) {
    fn_005f5840(e);
    let owner = e.global::<u32>(FORM_LIST_OWNER);
    let mut node = e.call(FIRST_FORM_NODE, &args![owner]).u32();
    while node != 0 {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let form = e.mem.u32(data);
        let active = form != 0
            && e.call(FORM_TEST_005A6150, &args![form]).u32() == 0
            && (fn_005f5930(e, Ptr::new(form)) == 0
                || e.call(FORM_TEST_005A60D0, &args![form]).u32() != 0);
        if active {
            add_to_kind_list(e, ACTIVE_LISTS, form);
        } else if form != 0 {
            add_to_kind_list(e, OTHER_LISTS, form);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// `sprintf_s`-style formatter (`00406d00`, cdecl: buffer, size, format, ...).
const FORMAT: u32 = 0x0040_6d00;
/// `"%s   %d\\%d\n%s"`.
const PROGRESS_FORMAT: u32 = 0x0104_8a9c;
/// `MapMarkerData::GetLocationName` (engine map) on `form + 0x18`: the
/// form's name (null when none).
const FORM_NAME: u32 = 0x0040_8da0;
/// The form's virtual slot +0x10 on `form + 0x24`, called with
/// `(0, 'DESC')`: its description text.
const DESCRIPTION_SLOT: u32 = 0x10;
const CHUNK_DESC: u32 = 0x4353_4544;
/// A global string the messages need to be non-empty (`00408d60` gives its
/// text).
const MESSAGE_SWITCH: u32 = 0x011c_b4d4;
const STRING_TEXT: u32 = 0x0040_8d60;
/// Called with `(form, 0)`; its result is passed to the message function.
const MESSAGE_ICON: u32 = 0x0048_e730;
/// Shows a message (interface.cpp; cdecl: text, 0, icon, 0, float, 0).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// The float passed to the message function.
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// Two counts of the form: `005e3fa0` (its progress) and `006286d0`
/// (`LowProcess::GetNumberOfItemsActivated` in the engine map; the body
/// reads the form, so the name is a folded one).
const FORM_PROGRESS: u32 = 0x005e_3fa0;
const FORM_TARGET: u32 = 0x0062_86d0;

/// The description check and text the progress messages use.
fn description(e: &mut Engine, form: u32) -> u32 {
    e.vcall(form + 0x24, DESCRIPTION_SLOT, &args![0u32, CHUNK_DESC])
        .u32()
}

/// Whether the progress message can be shown: the form has a name and a
/// description, and the string at `011cb4d4` is not empty.
fn message_allowed(e: &mut Engine, form: u32) -> bool {
    if e.call(FORM_NAME, &args![form + 0x18]).u32() == 0 {
        return false;
    }
    if description(e, form) == 0 {
        return false;
    }
    let text = e.call(STRING_TEXT, &args![MESSAGE_SWITCH]).u32();
    e.mem.u8(text) != 0
}

/// Formats the progress message into `buffer` and shows it.
fn show_progress(e: &mut Engine, form: u32, buffer: Ptr, first: u32, second: u32, text: u32) {
    let name = e.call(FORM_NAME, &args![form + 0x18]).u32();
    e.call(
        FORMAT,
        &args![buffer, 0x200u32, PROGRESS_FORMAT, name, second, first, text],
    );
    let icon = e.call(MESSAGE_ICON, &args![form, 0u32]).u32();
    // FLD/FSTP of the float constant into the pushed word.
    let time = x87_quiet_bits(e.global::<u32>(MESSAGE_TIME));
    e.call(SHOW_MESSAGE, &args![buffer, 0u32, icon, 0u32, time, 0u32]);
}

// Translated from 005f5950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f5950 - the /GS stack cookie check (00ec408c) is not translated.
/// Counts `amount` towards every form of kind `kind` whose conditions
/// match (cdecl, seven words; the last three are 16-bit values). Answers
/// false for a kind outside 0..14, true otherwise. The engine map gives it
/// no name.
///
/// It walks the list of the kind at `011cb3e0 + kind * 8`. A form is
/// skipped when `005a6150` answers non-zero and `005f5f60` zero, or
/// `fn_005f5930` non-zero and `005a60d0` zero. It must then match:
/// - when `004fd3a0` gives a value: `first` must be non-zero, and for kinds
///   0, 9 and 10 be in the list (`00500940`, `fn_005f65d0`) of that value,
///   for kinds 1, 3, 4, 5, 6, 7, 8 and 12 equal it (kinds 2, 11, 13: any);
/// - when `00407820` gives a value: `second` must be non-zero, and for
///   kinds 0..=4 and 8 be in that value's list;
/// - the 16-bit values at +0x64, +0x66, +0x68 (`fn_005f5fa0`, ...) must
///   equal `value_a`, `value_b`, `value_c` and those be non-zero; with no
///   value at +0x64, a form of kind 11 needs `value_a` to be 0; with none
///   at +0x66, a form of kind 3 needs `value_b` to be 0 or 1.
///
/// A matching form gets `amount` (`fn_005f60e0`). If its progress
/// (`005e3fa0`) is below its target (`006286d0`), the message
/// `"<name>   <progress>\<target>\n<description>"` is shown when the
/// progress reaches 0 counting down from it in steps of `fn_005f5f80` (100
/// when 0). Otherwise it is complete: its script (`00726070` on
/// `form + 0x2c`) runs `Script::RunScriptEffectStart` with `[011dea3c]`
/// and 0; the miscellaneous stat 0x1b is incremented unless the form is of
/// kind 11 with the value 0x1b at +0x64; the completion message is shown
/// (with menu sound 0x15); then either the form restarts (`005f5820`,
/// `005f6060(1)`, and the excess counted again) when `005f5f60` answers
/// non-zero and `005f60c0` zero, or it is marked done (`005f6000(1)`) and
/// the lists are rebuilt (`fn_005f5880`) after the walk.
pub fn fn_005f5950(
    e: &mut Engine,
    kind: i32,
    amount: u32,
    first: u32,
    second: u32,
    value_a: u16,
    value_b: u16,
    value_c: u16,
) -> bool {
    if !(0..KINDS).contains(&kind) {
        return false;
    }
    let mut node = ACTIVE_LISTS + kind as u32 * 8;
    let mut completed = false;
    while node != 0 {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let form = e.mem.u32(data);
        let mut matches = true;
        if form == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        let this = Ptr::new(form);
        if e.call(FORM_TEST_005A6150, &args![form]).u32() != 0
            && e.call(0x005f_5f60, &args![form]).u32() == 0
        {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if fn_005f5930(e, this) != 0 && e.call(FORM_TEST_005A60D0, &args![form]).u32() == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if e.call(0x004f_d3a0, &args![form]).u32() != 0 {
            if first == 0 {
                matches = false;
            } else {
                match kind {
                    0 | 9 | 10 => {
                        let owner = e.call(0x004f_d3a0, &args![form]).u32();
                        if owner != 0 && !in_list_of(e, owner, first) {
                            matches = false;
                        }
                    }
                    1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 => {
                        if first != e.call(0x004f_d3a0, &args![form]).u32() {
                            matches = false;
                        }
                    }
                    _ => {}
                }
            }
        }
        if !matches {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if e.call(0x0040_7820, &args![form]).u32() != 0 {
            if second == 0 {
                matches = false;
            } else if (0..=4).contains(&kind) || kind == 8 {
                let owner = e.call(0x0040_7820, &args![form]).u32();
                if owner != 0 && !in_list_of(e, owner, second) {
                    matches = false;
                }
            }
        }
        if !matches {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if fn_005f5fa0(e, this) != 0 {
            if value_a == 0 {
                matches = false;
            }
            if value_a != fn_005f5fa0(e, this) {
                matches = false;
            }
        } else if e.call(FORM_KIND, &args![form]).u32() == 0xb && value_a != fn_005f5fa0(e, this) {
            matches = false;
        }
        if fn_005f5fc0(e, this) != 0 {
            if value_b == 0 {
                matches = false;
            }
            if value_b != fn_005f5fc0(e, this) {
                matches = false;
            }
        } else if e.call(FORM_KIND, &args![form]).u32() == 3 && value_b != 0 && value_b != 1 {
            matches = false;
        }
        if fn_005f5fe0(e, this) != 0 {
            if value_c == 0 {
                matches = false;
            }
            if value_c != fn_005f5fe0(e, this) {
                matches = false;
            }
        }
        if !matches {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        fn_005f60e0(e, this, amount);
        let progress = e.call(FORM_PROGRESS, &args![form]).i32();
        let target = e.call(FORM_TARGET, &args![form]).i32();
        if progress < target {
            let start = e.call(FORM_PROGRESS, &args![form]).i32();
            let step = if fn_005f5f80(e, this) != 0 {
                fn_005f5f80(e, this) as i32
            } else {
                100
            };
            let mut count = start;
            while count >= 0 {
                if count == 0 && message_allowed(e, form) {
                    let text = description(e, form);
                    let target = e.call(FORM_TARGET, &args![form]).u32();
                    let progress = e.call(FORM_PROGRESS, &args![form]).u32();
                    e.with_stack(0x200, |e, buffer| {
                        show_progress(e, form, buffer, target, progress, text);
                    });
                }
                count = count.wrapping_sub(step);
            }
        } else {
            if e.call(NODE_NEXT, &args![form + 0x2c]).u32() != 0 {
                let player = e.global::<u32>(0x011d_ea3c);
                let script = e.call(NODE_NEXT, &args![form + 0x2c]).u32();
                e.call(0x005a_c340, &args![script, player, 0u32]);
            }
            if e.call(FORM_KIND, &args![form]).u32() != 0xb || fn_005f5fa0(e, this) != 0x1b {
                e.call(0x004d_5c60, &args![0x1bu32]);
            }
            if message_allowed(e, form) {
                let text = description(e, form);
                let target_first = e.call(FORM_TARGET, &args![form]).u32();
                let target_second = e.call(FORM_TARGET, &args![form]).u32();
                e.with_stack(0x200, |e, buffer| {
                    show_progress(e, form, buffer, target_first, target_second, text);
                });
                e.call(0x0070_6f30, &args![0x15u32]);
            }
            if e.call(0x005f_5f60, &args![form]).u32() != 0
                && e.call(0x005f_60c0, &args![form]).u32() == 0
            {
                let progress = e.call(FORM_PROGRESS, &args![form]).u32();
                let target = e.call(FORM_TARGET, &args![form]).u32();
                let excess = progress.wrapping_sub(target);
                e.call(0x005f_5820, &args![form]);
                e.call(0x005f_6060, &args![form, 1u32]);
                fn_005f60e0(e, this, excess);
            } else {
                e.call(0x005f_6000, &args![form, 1u32]);
                completed = true;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if completed {
        fn_005f5880(e);
    }
    true
}

/// Whether `wanted` is in the list of `owner` (`00500940` gives the list,
/// `fn_005f65d0` searches it for the word at the address of `wanted`).
fn in_list_of(e: &mut Engine, owner: u32, wanted: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), wanted);
        let list = e.call(0x0050_0940, &args![owner]).u32();
        fn_005f65d0(e, Ptr::new(list), slot)
    })
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the target `005f5630` and `005f56b0` pass (`011864c8`, the type
/// descriptor named `.?AVTESChallenge@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const CHALLENGE_TYPE: u32 = 0x0118_64c8;
/// The challenge's two values `005f5630` copies and `005f56b0` compares:
/// `004fd3a0` (set by `004febb0`) and `00407820` (set by `00442a80`), all
/// thiscall.
const FIRST_VALUE: u32 = 0x004f_d3a0;
const SET_FIRST_VALUE: u32 = 0x004f_ebb0;
const SECOND_VALUE: u32 = 0x0040_7820;
const SET_SECOND_VALUE: u32 = 0x0044_2a80;

// Translated from 005f5630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChallenge`'s copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 011864c8, 0)`; nothing without it. Otherwise the 0x18 bytes at +0x54
/// (`00401460`), the source's `004fd3a0` set through `004febb0`, its
/// `00407820` set through `00442a80`, then `TESForm::CopyAllComponents`
/// given the uncast source.
pub fn fn_005f5630(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, CHALLENGE_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0040_1460, &args![base + 0x54, other + 0x54, 0x18u32]);
    let value = e.call(FIRST_VALUE, &args![other]).u32();
    e.call(SET_FIRST_VALUE, &args![this, value]);
    let value = e.call(SECOND_VALUE, &args![other]).u32();
    e.call(SET_SECOND_VALUE, &args![this, value]);
    e.call(0x0048_51b0, &args![this, source]);
}

// Translated from 005f56b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChallenge`'s compare (`RET 4`: the other form; true when they
/// differ; the engine map has no name): true when the cast as in
/// `005f5630` fails, when `_memcmp` of the 0x18 bytes at +0x54 (the
/// other's through [`fn_005f5760`]) is not 0, when `00407820` or
/// `004fd3a0` answer differently, otherwise
/// `TESForm::CompareAllComponents`.
pub fn fn_005f56b0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let base = this.addr();
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, CHALLENGE_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    let data = fn_005f5760(e, Ptr::new(other)).addr();
    if e.call(0x00ec_4835, &args![base + 0x54, data, 0x18u32])
        .u32()
        != 0
    {
        return true;
    }
    let mine = e.call(SECOND_VALUE, &args![this]).u32();
    if mine != e.call(SECOND_VALUE, &args![other]).u32() {
        return true;
    }
    let mine = e.call(FIRST_VALUE, &args![this]).u32();
    if mine != e.call(FIRST_VALUE, &args![other]).u32() {
        return true;
    }
    e.call(0x0048_5270, &args![this, other]).bool()
}

// Translated from 005f57c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the object from `buffer` (`RET 4`; the engine map has no name),
/// the counterpart of [`fn_005f5780`]: `00484da0` on `this` with the
/// buffer, then the words at +0x6C and +0x70 through `00864980` (thiscall
/// on the buffer: address, size 4).
pub fn fn_005f57c0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    e.call(0x0086_4980, &args![buffer, this.addr() + 0x6c, 4u32]);
    e.call(0x0086_4980, &args![buffer, this.addr() + 0x70, 4u32]);
}

// Translated from 005f5820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +0x6C (no stack words; the engine map has no name).
pub fn fn_005f5820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x6c, 0);
}

// Translated from 005f5f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of the 16-bit word at +0x5C (no stack words; the engine map has
/// no name).
pub fn fn_005f5f60(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 0x5c)) & 2
}

/// Sets (`set` non-zero) or clears `bit` in the word at +0x70, then runs
/// [`fn_005f5800`].
fn set_flag(e: &mut Engine, this: Ptr, set: u8, bit: u32) {
    let at = this.addr() + 0x70;
    let flags = e.mem.u32(at);
    if set == 0 {
        e.mem.set_u32(at, flags & !bit);
    } else {
        e.mem.set_u32(at, flags | bit);
    }
    fn_005f5800(e, this);
}

// Translated from 005f6000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 1 of the word at +0x70 by the byte `set` (`RET 4`;
/// the engine map has no name), then runs [`fn_005f5800`].
pub fn fn_005f6000(e: &mut Engine, this: Ptr, set: u8) {
    set_flag(e, this, set, 2);
}

// Translated from 005f6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 2 of the word at +0x70 by the byte `set` (`RET 4`;
/// the engine map has no name), then runs [`fn_005f5800`].
pub fn fn_005f6060(e: &mut Engine, this: Ptr, set: u8) {
    set_flag(e, this, set, 4);
}

// Translated from 005f60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 3 of the word at +0x70 (no stack words; the engine map has no
/// name).
pub fn fn_005f60c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x70) & 8
}

// Translated from 005f6570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 0 of the word at +0x70 by the byte `set` (`RET 4`;
/// the engine map has no name), then runs [`fn_005f5800`].
pub fn fn_005f6570(e: &mut Engine, this: Ptr, set: u8) {
    set_flag(e, this, set, 1);
}

// Translated from 005f6110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f6110 - the /GS stack cookie check (00ec408c) is not translated.
/// Counts `amount` towards the forms of kind 13 that name `form` (cdecl:
/// the form, the amount; the engine map has no name). It walks the list
/// at `011cb448` (kind 13 of `011cb3e0`) and skips null entries and the
/// forms [`fn_005f5950`] skips (`005a6150` / [`fn_005f5f60`],
/// [`fn_005f5930`] / `005a60d0`), and those whose form id (`0084e3a0`)
/// differs from `form`'s. The rest is [`fn_005f5950`]'s: `amount` counted
/// ([`fn_005f60e0`]); below the target the progress message on reaching 0
/// in steps of [`fn_005f5f80`] (100 when 0); at the target the script, the
/// stat 0x1B, the completion message with menu sound 0x15, then a restart
/// ([`fn_005f5820`], [`fn_005f6060`]`(1)`, the excess counted again) when
/// [`fn_005f5f60`] answers non-zero and [`fn_005f60c0`] zero, or done
/// ([`fn_005f6000`]`(1)`) with the lists rebuilt ([`fn_005f5880`]) after
/// the walk. Answers true.
pub fn fn_005f6110(e: &mut Engine, form: u32, amount: u32) -> bool {
    let mut node = ACTIVE_LISTS + 0xd * 8;
    let mut completed = false;
    while node != 0 {
        let data = e.call(NODE_DATA, &args![node]).u32();
        let challenge = e.mem.u32(data);
        if challenge == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        let this = Ptr::new(challenge);
        if e.call(FORM_TEST_005A6150, &args![challenge]).u32() != 0 && fn_005f5f60(e, this) == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if fn_005f5930(e, this) != 0 && e.call(FORM_TEST_005A60D0, &args![challenge]).u32() == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        let wanted = e.call(0x0084_e3a0, &args![form]).u32();
        if wanted != e.call(0x0084_e3a0, &args![challenge]).u32() {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        fn_005f60e0(e, this, amount);
        let progress = e.call(FORM_PROGRESS, &args![challenge]).i32();
        let target = e.call(FORM_TARGET, &args![challenge]).i32();
        if progress < target {
            let start = e.call(FORM_PROGRESS, &args![challenge]).i32();
            let step = if fn_005f5f80(e, this) != 0 {
                fn_005f5f80(e, this) as i32
            } else {
                100
            };
            let mut count = start;
            while count >= 0 {
                if count == 0 && message_allowed(e, challenge) {
                    let text = description(e, challenge);
                    let target = e.call(FORM_TARGET, &args![challenge]).u32();
                    let progress = e.call(FORM_PROGRESS, &args![challenge]).u32();
                    e.with_stack(0x200, |e, buffer| {
                        show_progress(e, challenge, buffer, target, progress, text);
                    });
                }
                count = count.wrapping_sub(step);
            }
        } else {
            if e.call(NODE_NEXT, &args![challenge + 0x2c]).u32() != 0 {
                let player = e.global::<u32>(0x011d_ea3c);
                let script = e.call(NODE_NEXT, &args![challenge + 0x2c]).u32();
                e.call(0x005a_c340, &args![script, player, 0u32]);
            }
            if e.call(FORM_KIND, &args![challenge]).u32() != 0xb || fn_005f5fa0(e, this) != 0x1b {
                e.call(0x004d_5c60, &args![0x1bu32]);
            }
            if message_allowed(e, challenge) {
                let text = description(e, challenge);
                let target_first = e.call(FORM_TARGET, &args![challenge]).u32();
                let target_second = e.call(FORM_TARGET, &args![challenge]).u32();
                e.with_stack(0x200, |e, buffer| {
                    show_progress(e, challenge, buffer, target_first, target_second, text);
                });
                e.call(0x0070_6f30, &args![0x15u32]);
            }
            if fn_005f5f60(e, this) != 0 && fn_005f60c0(e, this) == 0 {
                let progress = e.call(FORM_PROGRESS, &args![challenge]).u32();
                let target = e.call(FORM_TARGET, &args![challenge]).u32();
                let excess = progress.wrapping_sub(target);
                fn_005f5820(e, this);
                fn_005f6060(e, this, 1);
                fn_005f60e0(e, this, excess);
            } else {
                fn_005f6000(e, this, 1);
                completed = true;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if completed {
        fn_005f5880(e);
    }
    true
}

// Translated from 005f64f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activates a challenge form (cdecl: the form; the engine map has no
/// name): nothing for a null form; otherwise its kind (`006a1b60`), bit 0
/// of its flags set ([`fn_005f6570`]`(1)`), the form added to its kind's
/// list at `011cb3e0` (`005ae3d0`) unless that list holds it
/// ([`fn_005f65d0`]), and removed from the one at `011cb450` (`00905330`)
/// when that one holds it (each given the address of the argument word).
/// Answers true.
pub fn fn_005f64f0(e: &mut Engine, form: u32) -> bool {
    if form == 0 {
        return true;
    }
    let kind = e.call(FORM_KIND, &args![form]).u32();
    fn_005f6570(e, Ptr::new(form), 1);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), form);
        let active = kind.wrapping_mul(8).wrapping_add(ACTIVE_LISTS);
        if !fn_005f65d0(e, Ptr::new(active), argument) {
            e.call(LIST_ADD, &args![active, argument]);
        }
        let other = kind.wrapping_mul(8).wrapping_add(OTHER_LISTS);
        if fn_005f65d0(e, Ptr::new(other), argument) {
            e.call(0x0090_5330, &args![other, argument]);
        }
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_5630, fn_005f5630(Ptr, u32)),
        entry!(0x005f_56b0, fn_005f56b0(Ptr, u32) -> bool),
        entry!(0x005f_57c0, fn_005f57c0(Ptr, u32)),
        entry!(0x005f_5820, fn_005f5820(Ptr)),
        entry!(0x005f_5f60, fn_005f5f60(Ptr) -> u32),
        entry!(0x005f_6000, fn_005f6000(Ptr, u8)),
        entry!(0x005f_6060, fn_005f6060(Ptr, u8)),
        entry!(0x005f_60c0, fn_005f60c0(Ptr) -> u32),
        entry!(0x005f_6110, fn_005f6110(u32, u32) -> bool),
        entry!(0x005f_64f0, fn_005f64f0(u32) -> bool),
        entry!(0x005f_6570, fn_005f6570(Ptr, u8)),
        entry!(0x005f5760, fn_005f5760(Ptr) -> Ptr),
        entry!(0x005f65d0, fn_005f65d0(Ptr, Ptr) -> bool),
        entry!(0x005f_6610, fn_005f6610(Ptr) -> Ptr),
        entry!(0x005f_67c0, fn_005f67c0(Ptr)),
        entry!(0x005f_5880, fn_005f5880()),
        entry!(0x005f_5840, fn_005f5840()),
        entry!(0x005f_5930, fn_005f5930(Ptr) -> u32),
        entry!(0x005f_5f80, fn_005f5f80(Ptr) -> u32),
        entry!(0x005f_5950, fn_005f5950(i32, u32, u32, u32, u16, u16, u16) -> bool),
        entry!(0x005f_5fa0, fn_005f5fa0(Ptr) -> u16),
        entry!(0x005f_5fc0, fn_005f5fc0(Ptr) -> u16),
        entry!(0x005f_5fe0, fn_005f5fe0(Ptr) -> u16),
        entry!(0x005f_60e0, fn_005f60e0(Ptr, u32)),
        entry!(0x005f_5800, fn_005f5800(Ptr)),
        entry!(0x005f_5780, fn_005f5780(Ptr, Ptr)),
        entry!(0x005f_66e0, fn_005f66e0(Ptr, u32) -> Ptr),
        entry!(0x005f_6710, fn_005f6710(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn word(e: &mut Engine, value: u32) -> Ptr {
        let p: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(p.addr(), value);
        p
    }

    #[test]
    fn the_condition_list_is_0x54_bytes_in() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(0x0040_0000);
        assert_eq!(e.call(0x005f_5760, &args![object]).u32(), 0x0040_0054);
    }

    #[test]
    fn finds_a_value_anywhere_in_the_list() {
        let mut e = Engine::new();
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 22);
        let first: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(first.addr(), 11);
        e.mem.set_u32(first.addr() + 4, second);
        let eleven = word(&mut e, 11);
        let twenty_two = word(&mut e, 22);
        let missing = word(&mut e, 33);
        assert!(e.call(0x005f_65d0, &args![first, eleven]).bool());
        assert!(e.call(0x005f_65d0, &args![first, twenty_two]).bool());
        assert!(!e.call(0x005f_65d0, &args![first, missing]).bool());
    }

    #[test]
    fn an_empty_list_contains_nothing() {
        let mut e = Engine::new();
        let any = word(&mut e, 0);
        assert!(!e.call(0x005f_65d0, &args![Ptr::<()>::NULL, any]).bool());
    }

    #[test]
    fn fn_005f6610_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_6610,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_e270, &[A::T(0x2c)]),
                (0x0047_ff70, &[A::T(0x38)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x7)]),
                (0x005f_67c0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_8bb4),
                (0x18, 4, 0x0104_8ba0),
                (0x24, 4, 0x0104_8b88),
                (0x2c, 4, 0x0104_8b68),
                (0x38, 4, 0x0104_8b54),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    const VTABLE: u32 = 0x0200_0000;
    const NOTIFY: u32 = 0x0300_0000;
    const DESCRIBE_VTABLE: u32 = 0x0200_0100;
    const DESCRIBE: u32 = 0x0300_0010;
    const NAME: u32 = 0x0400_0000;
    const SWITCH_TEXT: u32 = 0x0400_0100;
    const DESCRIPTION_TEXT: u32 = 0x0400_0200;

    // Where the fake form keeps the answers of the doubles below.
    const TEST_A6150: u32 = 0x100;
    const TEST_F5F60: u32 = 0x104;
    const TEST_A60D0: u32 = 0x108;
    const FIRST_VALUE: u32 = 0x10c;
    const SECOND_VALUE: u32 = 0x110;
    const PROGRESS: u32 = 0x114;
    const TARGET: u32 = 0x118;
    const TEST_F60C0: u32 = 0x11c;

    type Double = fn(&mut Engine, &[u32]) -> Ret;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn field(offset: u32) -> Double {
        match offset {
            TEST_A6150 => |e, a| ret(e.mem.u32(a[0] + TEST_A6150)),
            TEST_F5F60 => |e, a| ret(e.mem.u32(a[0] + TEST_F5F60)),
            TEST_A60D0 => |e, a| ret(e.mem.u32(a[0] + TEST_A60D0)),
            FIRST_VALUE => |e, a| ret(e.mem.u32(a[0] + FIRST_VALUE)),
            SECOND_VALUE => |e, a| ret(e.mem.u32(a[0] + SECOND_VALUE)),
            PROGRESS => |e, a| ret(e.mem.u32(a[0] + PROGRESS)),
            TARGET => |e, a| ret(e.mem.u32(a[0] + TARGET)),
            _ => |e, a| ret(e.mem.u32(a[0] + TEST_F60C0)),
        }
    }

    /// An engine with every callee doubled; nodes are `{data, next}` pairs
    /// (the node data function answers the node itself).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_KIND, |e, a| ret(e.mem.u32(a[0] + 0x54)));
        e.register(FORM_TEST_005A6150, field(TEST_A6150));
        e.register(0x005f_5f60, field(TEST_F5F60));
        e.register(FORM_TEST_005A60D0, field(TEST_A60D0));
        e.register(0x004f_d3a0, field(FIRST_VALUE));
        e.register(0x0040_7820, field(SECOND_VALUE));
        e.register(FORM_PROGRESS, field(PROGRESS));
        e.register(FORM_TARGET, field(TARGET));
        e.register(0x005f_60c0, field(TEST_F60C0));
        // The value lists: `00500940` answers the value itself as a node.
        e.register(0x0050_0940, |_, a| ret(a[0]));
        e.register(FORM_NAME, |_, _| ret(NAME));
        e.register(STRING_TEXT, |_, _| ret(SWITCH_TEXT));
        e.register(MESSAGE_ICON, |_, _| ret(0x0777));
        e.register(DESCRIBE, |_, _| ret(DESCRIPTION_TEXT));
        for callee in [
            NOTIFY,
            FORMAT,
            SHOW_MESSAGE,
            0x005a_c340,
            0x004d_5c60,
            0x0070_6f30,
            0x005f_5820,
            0x005f_6060,
            0x005f_6000,
            LIST_CLEAR,
            LIST_ADD,
            FIRST_FORM_NODE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &[0; 0x13]);
        e.mem.set_u32(VTABLE + 0x48, NOTIFY);
        e.put_vtable(DESCRIBE_VTABLE, &[0, 0, 0, 0, DESCRIBE]);
        e.map(NAME, 0x400);
        e.mem.set_u8(SWITCH_TEXT, 0x78);
        e.map(ACTIVE_LISTS, 0x200);
        e.map(FORM_LIST_OWNER, 4);
        e.map(MESSAGE_TIME, 4);
        e.mem.set_u32(MESSAGE_TIME, 0x4000_0000);
        e.map(0x011d_ea3c, 4);
        e.mem.set_u32(0x011d_ea3c, 0x0abc);
        e
    }

    /// A form of `kind` that matches every condition, alone in its list.
    fn form(e: &mut Engine, kind: u32) -> u32 {
        let form = e.mem.alloc(0x120);
        e.mem.set_u32(form, VTABLE);
        e.mem.set_u32(form + 0x24, DESCRIBE_VTABLE);
        e.mem.set_u32(form + 0x54, kind);
        e.mem.set_u32(form + PROGRESS, 1);
        e.mem.set_u32(form + TARGET, 5);
        e.mem.set_u32(ACTIVE_LISTS + kind * 8, form);
        form
    }

    fn run(
        e: &mut Engine,
        kind: i32,
        amount: u32,
        first: u32,
        second: u32,
        values: [u16; 3],
    ) -> bool {
        e.call_log = Some(vec![]);
        e.call(
            0x005f_5950,
            &args![kind, amount, first, second, values[0], values[1], values[2]],
        )
        .bool()
    }

    #[test]
    fn small_getters_read_their_fields() {
        let mut e = Engine::new();
        let p: Ptr = Ptr::new(e.mem.alloc(0x80));
        e.mem.set_u16(p.addr() + 0x5c, 0xfff3);
        e.mem.set_u32(p.addr() + 0x60, 0x1234_5678);
        e.mem.set_u16(p.addr() + 0x64, 0x8001);
        e.mem.set_u16(p.addr() + 0x66, 0x0002);
        e.mem.set_u16(p.addr() + 0x68, 0xffff);
        assert_eq!(e.call(0x005f_5930, &args![p]).u32(), 1);
        assert_eq!(e.call(0x005f_5f80, &args![p]).u32(), 0x1234_5678);
        assert_eq!(e.call(0x005f_5fa0, &args![p]).u32(), 0x8001);
        assert_eq!(e.call(0x005f_5fc0, &args![p]).u32(), 2);
        assert_eq!(e.call(0x005f_5fe0, &args![p]).u32(), 0xffff);
        e.mem.set_u16(p.addr() + 0x5c, 0xfffe);
        assert_eq!(e.call(0x005f_5930, &args![p]).u32(), 0);
    }

    #[test]
    fn adding_progress_wraps_and_notifies_the_form() {
        let mut e = engine();
        let f = form(&mut e, 0);
        e.mem.set_u32(f + 0x6c, 0xffff_fffe);
        e.call_log = Some(vec![]);
        e.call(0x005f_60e0, &args![f, 3u32]);
        assert_eq!(e.mem.u32(f + 0x6c), 1);
        assert_eq!(calls_to(&e, NOTIFY), vec![vec![f, 2]]);
        e.call(0x005f_5800, &args![f]);
        assert_eq!(calls_to(&e, NOTIFY).len(), 2);
    }

    #[test]
    fn saving_writes_the_two_counters() {
        let mut e = Engine::new();
        e.register(0x0048_4d60, |_, _| Ret::default());
        e.register(0x0086_5e50, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_5780, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, 0x0048_4d60), vec![vec![0x1000, 0x2000]]);
        assert_eq!(
            calls_to(&e, 0x0086_5e50),
            vec![vec![0x2000, 0x106c, 4, 0], vec![0x2000, 0x1070, 4, 0]]
        );
    }

    #[test]
    fn the_class_reset_clears_and_untags_four_skills() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, _| Ret::default());
        e.register(TES_CLASS_SET_TAG_SKILL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_67c0, &args![0x1000u32]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x1044, 0, 0x1c]]);
        assert_eq!(
            calls_to(&e, TES_CLASS_SET_TAG_SKILL),
            (0..4)
                .map(|i| vec![0x1000, i, 0xffff_ffff])
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_destructor_restores_vtables_and_runs_component_destructors() {
        let mut e = Engine::new();
        for callee in [
            0x0047_ffa0,
            0x0048_e2e0,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x60);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005f_66e0, &args![this, flags]).u32(), this);
            let log: Vec<(u32, Vec<u32>)> =
                e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![
                (0x0047_ffa0, vec![this + 0x38]),
                (0x0048_e2e0, vec![this + 0x2c]),
                (0x0059_1300, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ];
            if flags == 1 {
                expected.push((0x0040_1030, vec![this]));
            }
            assert_eq!(log, expected);
            assert_eq!(e.mem.u32(this), 0x0104_8bb4);
            assert_eq!(e.mem.u32(this + 0x18), 0x0104_8ba0);
            assert_eq!(e.mem.u32(this + 0x24), 0x0104_8b88);
            assert_eq!(e.mem.u32(this + 0x2c), 0x0104_8b68);
            assert_eq!(e.mem.u32(this + 0x38), 0x0104_8b54);
        }
    }

    #[test]
    fn rebuilding_sorts_the_forms_into_the_two_groups() {
        let mut e = engine();
        // Three nodes: an active form of kind 2, a null entry, and a form of
        // kind 5 that fails `005a6150`.
        let active = e.mem.alloc(0x120);
        e.mem.set_u32(active + 0x54, 2);
        let other = e.mem.alloc(0x120);
        e.mem.set_u32(other + 0x54, 5);
        e.mem.set_u32(other + TEST_A6150, 1);
        let n3 = e.mem.alloc(8);
        e.mem.set_u32(n3, other);
        let n2 = e.mem.alloc(8);
        e.mem.set_u32(n2 + 4, n3);
        let n1 = e.mem.alloc(8);
        e.mem.set_u32(n1, active);
        e.mem.set_u32(n1 + 4, n2);
        e.mem.set_u32(FORM_LIST_OWNER, 0x0555);
        e.register_double(FIRST_FORM_NODE, move |_, _| ret(n1));
        let added = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005f_5880, &args![]);
        let clears = calls_to(&e, LIST_CLEAR);
        assert_eq!(clears.len(), 28);
        assert_eq!(clears[0], vec![ACTIVE_LISTS]);
        assert_eq!(clears[1], vec![OTHER_LISTS]);
        assert_eq!(clears[27], vec![OTHER_LISTS + 13 * 8]);
        assert_eq!(calls_to(&e, FIRST_FORM_NODE), vec![vec![0x0555]]);
        assert_eq!(
            *added.borrow(),
            vec![(ACTIVE_LISTS + 2 * 8, active), (OTHER_LISTS + 5 * 8, other)]
        );
    }

    #[test]
    fn a_kind_outside_the_range_answers_false() {
        let mut e = engine();
        assert!(!run(&mut e, -1, 1, 0, 0, [0; 3]));
        assert!(!run(&mut e, 14, 1, 0, 0, [0; 3]));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn progress_below_the_target_shows_the_message_at_zero() {
        let mut e = engine();
        let f = form(&mut e, 2);
        e.mem.set_u32(f + PROGRESS, 0);
        assert!(run(&mut e, 2, 7, 0, 0, [0; 3]));
        assert_eq!(e.mem.u32(f + 0x6c), 7);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(
            &formats[0][1..],
            &[0x200, PROGRESS_FORMAT, NAME, 0, 5, DESCRIPTION_TEXT]
        );
        let shown = calls_to(&e, SHOW_MESSAGE);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][0], formats[0][0]);
        assert_eq!(&shown[0][1..], &[0, 0x0777, 0, 0x4000_0000, 0]);
        assert!(calls_to(&e, 0x0070_6f30).is_empty());
        assert!(calls_to(&e, 0x005f_6000).is_empty());
    }

    #[test]
    fn progress_not_on_a_step_shows_nothing() {
        let mut e = engine();
        let f = form(&mut e, 2);
        e.mem.set_u32(f + PROGRESS, 150);
        e.mem.set_u32(f + TARGET, 500);
        assert!(run(&mut e, 2, 1, 0, 0, [0; 3]));
        assert!(calls_to(&e, FORMAT).is_empty());
        // With a step of 50 it lands on zero.
        e.mem.set_u32(f + 0x60, 50);
        assert!(run(&mut e, 2, 1, 0, 0, [0; 3]));
        assert_eq!(calls_to(&e, FORMAT).len(), 1);
    }

    #[test]
    fn reaching_the_target_completes_the_form_and_rebuilds_the_lists() {
        let mut e = engine();
        let f = form(&mut e, 4);
        e.mem.set_u32(f + PROGRESS, 5);
        e.mem.set_u32(f + 0x30, 0x0999);
        assert!(run(&mut e, 4, 1, 0, 0, [0; 3]));
        assert_eq!(calls_to(&e, 0x005a_c340), vec![vec![0x0999, 0x0abc, 0]]);
        assert_eq!(calls_to(&e, 0x004d_5c60), vec![vec![0x1b]]);
        let formats = calls_to(&e, FORMAT);
        assert_eq!(
            &formats[0][1..],
            &[0x200, PROGRESS_FORMAT, NAME, 5, 5, DESCRIPTION_TEXT]
        );
        assert_eq!(calls_to(&e, 0x0070_6f30), vec![vec![0x15]]);
        assert_eq!(calls_to(&e, 0x005f_6000), vec![vec![f, 1]]);
        assert_eq!(calls_to(&e, LIST_CLEAR).len(), 28);
    }

    #[test]
    fn a_repeatable_form_restarts_with_the_excess() {
        let mut e = engine();
        let f = form(&mut e, 4);
        e.mem.set_u32(f + PROGRESS, 8);
        e.mem.set_u32(f + TEST_F5F60, 1);
        // The empty switch string hides the message.
        e.mem.set_u8(SWITCH_TEXT, 0);
        assert!(run(&mut e, 4, 2, 0, 0, [0; 3]));
        assert!(calls_to(&e, 0x005a_c340).is_empty());
        assert!(calls_to(&e, FORMAT).is_empty());
        assert_eq!(calls_to(&e, 0x005f_5820), vec![vec![f]]);
        assert_eq!(calls_to(&e, 0x005f_6060), vec![vec![f, 1]]);
        assert_eq!(e.mem.u32(f + 0x6c), 5);
        assert!(calls_to(&e, 0x005f_6000).is_empty());
        assert!(calls_to(&e, LIST_CLEAR).is_empty());
    }

    #[test]
    fn kind_11_with_value_0x1b_skips_the_stat() {
        let mut e = engine();
        let f = form(&mut e, 11);
        e.mem.set_u32(f + PROGRESS, 5);
        e.mem.set_u16(f + 0x64, 0x1b);
        assert!(run(&mut e, 11, 1, 0, 0, [0x1b, 0, 0]));
        assert!(calls_to(&e, 0x004d_5c60).is_empty());
        assert_eq!(calls_to(&e, 0x005f_6000).len(), 1);
    }

    /// Whether the form got the amount (it matched).
    fn matched(
        kind: u32,
        setup: impl FnOnce(&mut Engine, u32),
        first: u32,
        second: u32,
        values: [u16; 3],
    ) -> bool {
        let mut e = engine();
        let f = form(&mut e, kind);
        setup(&mut e, f);
        assert!(run(&mut e, kind as i32, 1, first, second, values));
        e.mem.u32(f + 0x6c) == 1
    }

    #[test]
    fn the_skip_tests_come_first() {
        assert!(!matched(
            2,
            |e, f| e.mem.set_u32(f + TEST_A6150, 1),
            0,
            0,
            [0; 3]
        ));
        assert!(matched(
            2,
            |e, f| {
                e.mem.set_u32(f + TEST_A6150, 1);
                e.mem.set_u32(f + TEST_F5F60, 1);
            },
            0,
            0,
            [0; 3]
        ));
        assert!(!matched(2, |e, f| e.mem.set_u16(f + 0x5c, 1), 0, 0, [0; 3]));
        assert!(matched(
            2,
            |e, f| {
                e.mem.set_u16(f + 0x5c, 1);
                e.mem.set_u32(f + TEST_A60D0, 1);
            },
            0,
            0,
            [0; 3]
        ));
    }

    #[test]
    fn the_first_value_is_checked_per_kind() {
        let set = |e: &mut Engine, f: u32| e.mem.set_u32(f + FIRST_VALUE, 0x77);
        // A value on the form needs a non-zero argument, for any kind.
        assert!(!matched(2, set, 0, 0, [0; 3]));
        assert!(matched(2, set, 0x55, 0, [0; 3]));
        assert!(matched(11, set, 0x55, 0, [0; 3]));
        // Kinds 1, 3, 4, 5, 6, 7, 8, 12: equal.
        for kind in [1, 3, 4, 5, 6, 7, 8, 12] {
            assert!(!matched(kind, set, 0x55, 0, [0; 3]));
            assert!(matched(kind, set, 0x77, 0, [0; 3]));
        }
        // Kinds 0, 9, 10: in the list of the value.
        let in_list = |e: &mut Engine, f: u32| {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, 0x55);
            e.mem.set_u32(f + FIRST_VALUE, node);
        };
        for kind in [0, 9, 10] {
            assert!(matched(kind, in_list, 0x55, 0, [0; 3]));
            assert!(!matched(kind, in_list, 0x56, 0, [0; 3]));
        }
    }

    #[test]
    fn the_second_value_is_checked_for_kinds_up_to_4_and_8() {
        let in_list = |e: &mut Engine, f: u32| {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, 0x66);
            e.mem.set_u32(f + SECOND_VALUE, node);
        };
        assert!(!matched(2, in_list, 0, 0, [0; 3]));
        for kind in [0, 2, 4, 8] {
            assert!(matched(kind, in_list, 0, 0x66, [0; 3]));
            assert!(!matched(kind, in_list, 0, 0x67, [0; 3]));
        }
        // Other kinds take any non-zero argument.
        assert!(matched(5, in_list, 0, 0x67, [0; 3]));
    }

    #[test]
    fn the_word_values_must_match() {
        let a = |e: &mut Engine, f: u32| e.mem.set_u16(f + 0x64, 9);
        assert!(matched(2, a, 0, 0, [9, 0, 0]));
        assert!(!matched(2, a, 0, 0, [8, 0, 0]));
        let b = |e: &mut Engine, f: u32| e.mem.set_u16(f + 0x66, 9);
        assert!(matched(2, b, 0, 0, [0, 9, 0]));
        assert!(!matched(2, b, 0, 0, [0, 8, 0]));
        let c = |e: &mut Engine, f: u32| e.mem.set_u16(f + 0x68, 9);
        assert!(matched(2, c, 0, 0, [0, 0, 9]));
        assert!(!matched(2, c, 0, 0, [0, 0, 0]));
        // Without a value at +0x64, kind 11 needs 0; without one at +0x66,
        // kind 3 needs 0 or 1.
        let none = |_: &mut Engine, _: u32| {};
        assert!(!matched(11, none, 0, 0, [1, 0, 0]));
        assert!(matched(11, none, 0, 0, [0, 0, 0]));
        assert!(matched(2, none, 0, 0, [1, 0, 0]));
        assert!(matched(3, none, 0, 0, [0, 1, 0]));
        assert!(!matched(3, none, 0, 0, [0, 2, 0]));
        assert!(matched(2, none, 0, 0, [0, 2, 0]));
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

    #[test]
    fn challenge_copy_and_compare() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| ret(if a[0] == 1 { 0 } else { a[0] }));
        for a in [
            0x0040_1460u32,
            SET_FIRST_VALUE,
            SET_SECOND_VALUE,
            0x0048_51b0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(FIRST_VALUE, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(SECOND_VALUE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        let this = e.mem.alloc(0x80);
        let other = e.mem.alloc(0x80);
        e.mem.set_u32(other + 0x40, 0x41);
        e.mem.set_u32(other + 0x44, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x005f_5630, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, CHALLENGE_TYPE, 0]]
        );
        assert_eq!(
            calls(&e, 0x0040_1460),
            vec![vec![this + 0x54, other + 0x54, 0x18]]
        );
        assert_eq!(calls(&e, SET_FIRST_VALUE), vec![vec![this, 0x41]]);
        assert_eq!(calls(&e, SET_SECOND_VALUE), vec![vec![this, 0x42]]);
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
        e.call_log = Some(vec![]);
        e.call(0x005f_5630, &args![this, 1u32]);
        assert!(calls(&e, 0x0040_1460).is_empty());

        e.register(0x00ec_4835, |_, _| ret(0));
        e.register(0x0048_5270, |_, _| ret(0));
        e.mem.set_u32(this + 0x40, 0x41);
        e.mem.set_u32(this + 0x44, 0x42);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_56b0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, 0x00ec_4835),
            vec![vec![this + 0x54, other + 0x54, 0x18]]
        );
        assert!(e.call(0x005f_56b0, &args![this, 1u32]).bool());
        e.mem.set_u32(this + 0x44, 0);
        assert!(e.call(0x005f_56b0, &args![this, other]).bool());
        e.mem.set_u32(this + 0x44, 0x42);
        e.mem.set_u32(this + 0x40, 0);
        assert!(e.call(0x005f_56b0, &args![this, other]).bool());
        e.mem.set_u32(this + 0x40, 0x41);
        e.register(0x00ec_4835, |_, _| ret(1));
        assert!(e.call(0x005f_56b0, &args![this, other]).bool());
        e.register(0x00ec_4835, |_, _| ret(0));
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005f_56b0, &args![this, other]).bool());
    }

    #[test]
    fn load_and_flag_helpers() {
        let mut e = Engine::new();
        e.register(0x0048_4da0, |_, _| Ret::default());
        e.register(0x0086_4980, |_, _| Ret::default());
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x005f_57c0, &args![this, 0x4444u32]);
        assert_eq!(calls(&e, 0x0048_4da0), vec![vec![this, 0x4444]]);
        assert_eq!(
            calls(&e, 0x0086_4980),
            vec![vec![0x4444, this + 0x6c, 4], vec![0x4444, this + 0x70, 4]]
        );
        e.mem.set_u32(this + 0x6c, 9);
        e.call(0x005f_5820, &args![this]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        e.mem.set_u16(this + 0x5c, 0xffff);
        assert_eq!(e.call(0x005f_5f60, &args![this]).u32(), 2);
        e.mem.set_u16(this + 0x5c, 0xfffd);
        assert_eq!(e.call(0x005f_5f60, &args![this]).u32(), 0);

        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.register(0x7000_0048, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x70, 0xf0);
        e.call_log = Some(vec![]);
        for (addr, bit) in [(0x005f_6570u32, 1u32), (0x005f_6000, 2), (0x005f_6060, 4)] {
            e.call(addr, &args![this, 0x100u32]);
            assert_eq!(e.mem.u32(this + 0x70) & bit, 0, "{addr:x}");
            e.call(addr, &args![this, 1u32]);
            assert_eq!(e.mem.u32(this + 0x70) & bit, bit, "{addr:x}");
        }
        assert_eq!(e.mem.u32(this + 0x70), 0xf7);
        assert_eq!(calls(&e, 0x7000_0048), vec![vec![this, 2]; 6]);
        assert_eq!(e.call(0x005f_60c0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x70, 0xff);
        assert_eq!(e.call(0x005f_60c0, &args![this]).u32(), 8);
    }

    #[test]
    fn fn_005f64f0_moves_the_form_to_its_active_list() {
        let mut e = Engine::new();
        e.map(0x011c_b000, 0x1000);
        e.register(FORM_KIND, |e, a| ret(e.mem.u32(a[0] + 0x54)));
        e.register(LIST_ADD, |e, a| {
            let form = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], form);
            Ret::default()
        });
        e.register(0x0090_5330, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.register(0x7000_0048, |_, _| Ret::default());
        let form = e.mem.alloc(0x80);
        e.mem.set_u32(form, vtable);
        e.mem.set_u32(form + 0x54, 3);
        e.mem.set_u32(OTHER_LISTS + 3 * 8, form);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_64f0, &args![form]).bool());
        assert_eq!(e.mem.u32(form + 0x70), 1);
        assert_eq!(e.mem.u32(ACTIVE_LISTS + 3 * 8), form);
        assert_eq!(e.mem.u32(OTHER_LISTS + 3 * 8), 0);
        // Already active and not in the other list: no calls.
        e.call_log = Some(vec![]);
        e.call(0x005f_64f0, &args![form]);
        assert!(calls(&e, LIST_ADD).is_empty());
        assert!(calls(&e, 0x0090_5330).is_empty());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_64f0, &args![0u32]).bool());
        assert!(calls(&e, FORM_KIND).is_empty());
    }

    /// A kind-13 challenge in the list at `011cb448` (nodes `{data, next}`):
    /// form id at +0xC, progress at +0x6C, target 10.
    fn challenge_setup() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011c_b000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.register(NODE_DATA, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_TEST_005A6150, |_, _| ret(0));
        e.register(FORM_TEST_005A60D0, |_, _| ret(0));
        e.register(0x0084_e3a0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_PROGRESS, |e, a| ret(e.mem.u32(a[0] + 0x6c)));
        e.register(FORM_TARGET, |_, _| ret(10));
        e.register(FORM_KIND, |_, _| ret(0xd));
        e.register(FORM_NAME, |_, _| ret(0));
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.register(FIRST_FORM_NODE, |_, _| ret(0));
        for a in [
            0x004d_5c60u32,
            0x0070_6f30,
            FORMAT,
            SHOW_MESSAGE,
            MESSAGE_ICON,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.register(0x7000_0048, |_, _| Ret::default());
        let challenge = e.mem.alloc(0x80);
        e.mem.set_u32(challenge, vtable);
        e.mem.set_u32(challenge + 0xc, 0x77);
        e.mem.set_u32(ACTIVE_LISTS + 0xd * 8, challenge);
        (e, challenge)
    }

    #[test]
    fn fn_005f6110_completes_or_restarts_a_matching_challenge() {
        // Restart: bit 1 at +0x5C and bit 3 at +0x70 clear.
        let (mut e, challenge) = challenge_setup();
        e.mem.set_u16(challenge + 0x5c, 2);
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form + 0xc, 0x77);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_6110, &args![form, 12u32]).bool());
        assert_eq!(e.mem.u32(challenge + 0x6c), 2);
        assert_eq!(e.mem.u32(challenge + 0x70), 4);
        assert_eq!(calls(&e, 0x004d_5c60), vec![vec![0x1b]]);
        assert!(calls(&e, LIST_CLEAR).is_empty());

        // Done: marked and the lists rebuilt.
        let (mut e, challenge) = challenge_setup();
        e.call_log = Some(vec![]);
        let form = form_with_id(&mut e, 0x77);
        e.call(0x005f_6110, &args![form, 10u32]);
        assert_eq!(e.mem.u32(challenge + 0x70), 2);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 28);

        // Another form id: nothing counted.
        let (mut e, challenge) = challenge_setup();
        let form = form_with_id(&mut e, 0x78);
        e.call(0x005f_6110, &args![form, 10u32]);
        assert_eq!(e.mem.u32(challenge + 0x6c), 0);
    }

    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x10);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    #[test]
    fn fn_005f6110_shows_the_progress_at_zero() {
        let (mut e, challenge) = challenge_setup();
        e.register(FORM_NAME, |_, _| ret(0x5555));
        let text = e.mem.alloc(4);
        e.mem.set_u8(text, b'x');
        e.register_double(STRING_TEXT, move |_, _| ret(text));
        let description_vtable = e.mem.alloc(0x14);
        e.mem.set_u32(description_vtable + 0x10, 0x7000_0010);
        e.register(0x7000_0010, |_, _| ret(0x6666));
        e.mem.set_u32(challenge + 0x24, description_vtable);
        e.call_log = Some(vec![]);
        let form = form_with_id(&mut e, 0x77);
        e.call(0x005f_6110, &args![form, 0u32]);
        let format = calls(&e, FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(
            &format[0][1..],
            &[0x200, PROGRESS_FORMAT, 0x5555, 0, 10, 0x6666]
        );
        assert_eq!(calls(&e, SHOW_MESSAGE).len(), 1);
        assert!(calls(&e, 0x0070_6f30).is_empty());
    }
}
