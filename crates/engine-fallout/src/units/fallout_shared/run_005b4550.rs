//! `fallout shared/run_005b4550` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The script command `ShowMessage` (`005b4630`, named in docs/OPENING.md and
//! docs/HUD_MESSAGES.md; the engine map has no name for it) and the three
//! small functions beside it: the reset of the message-box answer
//! (`005b4940`), the formatter that builds the text from the script's
//! arguments (`005b4960`) and the box's answer callback (`005b4a70`).
//!
//! Notes on the exe's code:
//! - `ShowMessage` is a script command body: the eight words of
//!   [`ScriptArgs`], answer in `AL`.
//! - The text comes from `TESDescription::GetDescription` (`00482e90`, the
//!   Xbox PDB name the engine map gives it; its vtable is the one the message
//!   form installs at `+0x24`) called through slot `0x10` with (0, `'DESC'`).
//!   The caller also pushes the address of the string it formats into; the
//!   callee (`RET 8`) leaves that word on the stack, where `005b4960` reads it
//!   as its last parameter.
//! - `005b4960` keeps its values in a stack array of ten `double`s but loops
//!   `count` times with a 16-bit counter, so a message with more than ten
//!   arguments overruns the array in the game (undefined behaviour). Here the
//!   array is large enough for any count and only the first ten are
//!   formatted.
//! - The callee that evaluates a condition is `00680c60`, which the engine map
//!   names `TESCondition::IsTrue_ov2`; its body matches (a condition list, the
//!   reference, an out flag).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_shared::tesscriptfunctions::ScriptArgs;

/// The message-box answer, a byte (`0xff` when no button has been pressed).
const MESSAGE_ANSWER: u32 = 0x0118_c684;
/// The reference or script a message box's answer belongs to.
const MESSAGE_OWNER: u32 = 0x011c_ac64;
/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The empty text used for a condition that is false (`""`).
const EMPTY_TEXT: u32 = 0x0102_0770;
/// The text used when no condition gave one.
const NO_TEXT: u32 = 0x0103_934c;
/// `'DESC'` as a little-endian four-character code.
const DESCRIPTION_CHUNK: u32 = 0x4353_4544;

/// `memset` wrapper (`00403d30`: destination, value, count).
const MEMSET_WRAPPER: u32 = 0x0040_3d30;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The `BSStringT<char>` destructor (`004037d0`).
const STRING_DESTROY: u32 = 0x0040_37d0;
/// The string's text pointer (`00559450`: the word at `this`).
const TEXT_POINTER: u32 = 0x0055_9450;
/// The string's length (`004048e0`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// A function that answers its argument (`00464f30`).
const IDENTITY: u32 = 0x0046_4f30;
/// The text of a name-bearing part (`00408da0`): the string's text, or the
/// empty string.
const TEXT_OR_EMPTY: u32 = 0x0040_8da0;
/// A list node's item word address (`006815c0`: answers `this`).
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// The next list node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// The head node of the message's item list (`00436aa0`, `this + 0x30`).
const ITEM_LIST_HEAD: u32 = 0x0043_6aa0;
/// `TESCondition::IsTrue_ov2` (`00680c60`, `RET 0x10`): conditions at
/// `this`, (reference, 0, out flag, 0).
const CONDITIONS_ARE_TRUE: u32 = 0x0068_0c60;
/// `Script::ParseParameters` is called through [`ScriptArgs::parse_into`].
/// One value of the script's data (`005ac7a0`).
const READ_NUMBER: u32 = 0x005a_c7a0;
/// The printf-style formatter (`00406f60`: string, format, arguments).
const FORMAT_INTO_STRING: u32 = 0x0040_6f60;
/// `Interface::GetMessageMenuResult` (`00703fa0`).
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// The flag test of a script reference (`004077c0`: bit 0x4000, temporary).
const IS_TEMPORARY: u32 = 0x0040_77c0;
/// The owner id of a reference (`0084e3a0`).
const OWNER_ID: u32 = 0x0084_e3a0;
/// The message form's flag test (`00590d00`, bit 0 of `+0x38`: a message box).
const IS_MESSAGE_BOX: u32 = 0x0059_0d00;
/// A form's second part, used for the title (`0055b980`: the word at `+0x2c`).
const TITLE_PART: u32 = 0x0055_b980;
/// A word getter (`00639b40`: the word at `+0x3c`).
const DISPLAY_TIME: u32 = 0x0063_9b40;
/// The box (`00703e80`) and the plain message (`007052f0`) display functions.
const SHOW_BOX: u32 = 0x0070_3e80;
const SHOW_PLAIN: u32 = 0x0070_52f0;

// Translated from 005b4630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ShowMessage` script command. Parses the message form from the script
/// (false when it does not parse or the form is missing), builds the text of
/// the message from the form's description and the script's arguments
/// (`005b4960`, false when an argument cannot be read), resets the answer
/// (`005b4940`) and picks who the answer belongs to: the running reference
/// (its id, `0084e3a0`) when there is one that is not temporary (flag
/// `0x4000`), otherwise the running script, and false when there is neither.
/// It then collects up to ten texts, one for each item of the form's list
/// whose conditions hold (the empty text `01020770` for an item whose
/// conditions do not, and `0103934c` for the first when none held). A message
/// form that is a box (`00590d00`) is shown with `00703e80` (the answer
/// callback is `005b4a70`); a plain message is shown with `007052f0` when it
/// has any text, with the form's name and display time. True once shown.
// Translation incomplete: 005b4630 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_005b4630(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, _]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    if form == 0 {
        return false;
    }
    let string = e.mem.alloc(8);
    e.call(STRING_CONSTRUCT, &args![string]);
    let shown = show_message(e, a, form, string);
    e.call(STRING_DESTROY, &args![string]);
    e.mem.free(string);
    shown
}

/// The part of `ShowMessage` between building and destroying the text
/// string.
fn show_message(e: &mut Engine, a: ScriptArgs, form: u32, string: u32) -> bool {
    // `TESDescription::GetDescription` of the part at +0x24: (0, 'DESC').
    let description = e
        .vcall(form + 0x24, 0x10, &args![0u32, DESCRIPTION_CHUNK])
        .u32();
    if !fn_005b4960(
        e,
        a.param_info,
        a.script_data,
        a.this_obj.addr(),
        a.containing_obj.addr(),
        a.script_obj.addr(),
        a.event_list,
        a.result.addr(),
        Ptr::new(a.opcode_offset),
        description,
        string,
    ) {
        return false;
    }
    fn_005b4940(e);

    let running_reference = a.this_obj.addr();
    let owner_source =
        if running_reference != 0 && !e.call(IS_TEMPORARY, &args![running_reference]).bool() {
            running_reference
        } else if a.script_obj.addr() != 0 {
            a.script_obj.addr()
        } else {
            return false;
        };
    let owner = e.call(OWNER_ID, &args![owner_source]).u32();
    e.set_global(MESSAGE_OWNER, owner);

    // Up to ten texts, one for each item of the form's list.
    let texts = e.mem.alloc(0x28);
    e.call(MEMSET_WRAPPER, &args![texts, 0u32, 0x28u32]);
    let flag = e.mem.alloc(4);
    let mut count = 0u32;
    let mut all_empty = true;
    let mut node = e.call(ITEM_LIST_HEAD, &args![form]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 || count as i32 >= 10 {
            break;
        }
        let player = e.global::<u32>(PLAYER_POINTER);
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        let holds = e
            .call(
                CONDITIONS_ARE_TRUE,
                &args![item + 8, player, 0u32, flag, 0u32],
            )
            .bool();
        let text = if holds {
            let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
            let item = e.mem.u32(slot);
            all_empty = false;
            e.call(TEXT_POINTER, &args![item]).u32()
        } else {
            EMPTY_TEXT
        };
        e.mem.set_u32(texts + 4 * count, text);
        count += 1;
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if all_empty {
        e.mem.set_u32(texts, NO_TEXT);
    }

    if e.call(IS_MESSAGE_BOX, &args![form]).bool() {
        let title = e.call(TEXT_OR_EMPTY, &args![form + 0x18]).u32();
        let text = e.call(TEXT_POINTER, &args![string]).u32();
        let text = e.call(IDENTITY, &args![text]).u32();
        // Nineteen words: text, 0, title, the answer callback, 0, 0x17, two
        // 0.0 floats, the ten texts, 0.
        let mut words = args![
            text,
            0u32,
            title,
            0x005b_4a70u32,
            0u32,
            0x17u32,
            0.0f32,
            0.0f32
        ];
        for i in 0..10 {
            words.push(e.mem.u32(texts + 4 * i));
        }
        words.push(0);
        e.call(SHOW_BOX, &words);
    } else if e.call(STRING_LENGTH, &args![string]).u32() != 0 {
        let part = e.call(TITLE_PART, &args![form]).u32();
        let name = if part == 0 {
            0
        } else {
            let part = e.call(TITLE_PART, &args![form]).u32();
            e.call(TEXT_OR_EMPTY, &args![part + 0x18]).u32()
        };
        // The value is an unsigned word shown as a `float`.
        let time = e.call(DISPLAY_TIME, &args![form]).u32() as f32;
        let text = e.call(TEXT_POINTER, &args![string]).u32();
        let text = e.call(IDENTITY, &args![text]).u32();
        e.call(SHOW_PLAIN, &args![text, 0u32, name, 0u32, time, 0u32]);
    }
    e.mem.free(flag);
    e.mem.free(texts);
    true
}

// Translated from 005b4940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets the pending message-box answer: the answer byte becomes 0xff (no
/// button) and the owner is cleared.
pub fn fn_005b4940(e: &mut Engine) {
    e.set_global(MESSAGE_ANSWER, 0xffu8);
    e.set_global(MESSAGE_OWNER, 0u32);
}

// Translated from 005b4960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a message's arguments from the script data and formats them into
/// `string`. `script_data` plus `*offset` holds a 16-bit count and then that
/// many values, each read as a `double` with `005ac7a0` (the reference,
/// container, script and locals words are passed on to it, `offset` advances);
/// false when one cannot be read. The text is then made by `00406f60(string,
/// format, ...)` with the first ten values as ten `double` arguments, the
/// unread ones 0.0. The first and seventh words are never read.
#[allow(clippy::too_many_arguments)]
pub fn fn_005b4960(
    e: &mut Engine,
    _unused_0: u32,
    script_data: u32,
    reference: u32,
    container: u32,
    script: u32,
    locals: u32,
    _unused_6: u32,
    offset: Ptr,
    format: u32,
    string: u32,
) -> bool {
    let position = e.mem.u32(offset.addr());
    let count = e.mem.u16(script_data + position) as i16;
    e.mem.set_u32(offset.addr(), position + 2);

    // Ten `double`s (0x50 bytes) in the game; any count fits here.
    let values = e.mem.alloc((8 * count.max(0) as u32).max(0x50));
    e.call(MEMSET_WRAPPER, &args![values, 0u32, 0x50u32]);
    let mut index: i16 = 0;
    while index < count {
        let slot = values + 8 * index as u32;
        e.mem.set_f64(slot, 0.0);
        let read = e
            .call(
                READ_NUMBER,
                &args![
                    slot,
                    script_data,
                    offset,
                    reference,
                    container,
                    script,
                    locals,
                    1u32
                ],
            )
            .bool();
        if !read {
            e.mem.free(values);
            return false;
        }
        index += 1;
    }
    let mut words = args![string, format];
    for i in 0..10 {
        words.push(e.mem.u32(values + 8 * i));
        words.push(e.mem.u32(values + 8 * i + 4));
    }
    e.call(FORMAT_INTO_STRING, &words);
    e.mem.free(values);
    true
}

// Translated from 005b4a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message box's answer callback: stores `Interface::GetMessageMenuResult`
/// (`00703fa0`, the pressed button, which it also resets to 0xff) in the
/// answer byte.
pub fn fn_005b4a70(e: &mut Engine) {
    let answer = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8();
    e.set_global(MESSAGE_ANSWER, answer);
}

/// `-1.0` (`double`), the result when there is no answer for the caller.
const NO_ANSWER: u32 = 0x0101_a6b0;

// Translated from 005b4a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A script command body (no name in the engine map; the eight words of
/// [`ScriptArgs`], answer in `AL`) that reads the message-box answer: the
/// caller is the running reference's owner id (`0084e3a0`) when there is a
/// reference that is not temporary (`004077c0`), otherwise the running
/// script's, and false when there is neither. When an answer is waiting
/// (the signed answer byte above -1) for that caller, the result is the
/// answer and the answer and its owner are reset (0xff, 0); otherwise the
/// result is the `double` at `0101a6b0` (-1.0). True.
pub fn fn_005b4a80(e: &mut Engine, a: ScriptArgs) -> bool {
    let caller = if !a.this_obj.is_null() && !e.call(IS_TEMPORARY, &args![a.this_obj]).bool() {
        e.call(OWNER_ID, &args![a.this_obj]).u32()
    } else if !a.script_obj.is_null() {
        e.call(OWNER_ID, &args![a.script_obj]).u32()
    } else {
        return false;
    };
    let answer = e.global::<i8>(MESSAGE_ANSWER);
    if i32::from(answer) > -1 && caller == e.global::<u32>(MESSAGE_OWNER) {
        let answer = e.global::<i8>(MESSAGE_ANSWER);
        e.mem.set_f64(a.result.addr(), f64::from(answer));
        e.set_global(MESSAGE_ANSWER, 0xffu8);
        e.set_global(MESSAGE_OWNER, 0u32);
    } else {
        let no_answer = x87_load64(e.global::<u64>(NO_ANSWER));
        e.mem.set_u64(a.result.addr(), no_answer.to_bits());
    }
    true
}

// Translated from 005b4550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005b4550 - the /GS stack cookie check (00ec408c) is not translated.
/// A script command body (no name in the engine map; the eight words of
/// [`ScriptArgs`], answer in `AL`): parses its parameters into a 0x204-byte
/// stack buffer (false when they do not parse), then shows that text with
/// `00703e80(text, 0, 0, callback, 1, 0, 0.0, 0.0, <004c69f0 on 011d38b8>,
/// 0)`, the callback `0095ee60` when `005a9460` on `011de45c` answers
/// non-zero and `0095ee70` otherwise. True.
pub fn fn_005b4550(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x204, |e, text| {
        if !a.parse(e, &[text.addr()]) {
            return false;
        }
        let holder = e.global::<u32>(0x011d_e45c);
        let callback = if e.call(0x005a_9460, &args![holder]).u32() != 0 {
            0x0095_ee60u32
        } else {
            0x0095_ee70
        };
        let word = e.call(0x004c_69f0, &args![0x011d_38b8u32]).u32();
        e.call(
            SHOW_BOX,
            &args![text, 0u32, 0u32, callback, 1u32, 0u32, 0.0f32, 0.0f32, word, 0u32],
        );
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005b_4550, fn_005b4550(ScriptArgs) -> bool),
        entry!(0x005b4630, fn_005b4630(ScriptArgs) -> bool),
        entry!(0x005b4940, fn_005b4940()),
        entry!(
            0x005b4960,
            fn_005b4960(u32, u32, u32, u32, u32, u32, u32, Ptr, u32, u32) -> bool
        ),
        entry!(0x005b4a70, fn_005b4a70()),
        entry!(0x005b_4a80, fn_005b4a80(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
    /// Test vtable of the message form's description part (slot 0x10).
    const DESCRIPTION_VTABLE: u32 = 0x00ab_0000;
    const GET_DESCRIPTION: u32 = 0x00aa_0010;
    const FORMAT_TEXT: u32 = 0x00ac_0000;

    /// Everything the command touches outside this file is a double; the
    /// globals live in mapped pages.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            MESSAGE_ANSWER & !0xfff,
            MESSAGE_OWNER & !0xfff,
            PLAYER_POINTER & !0xfff,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(PLAYER_POINTER, 0x7777u32);
        for callee in [
            STRING_CONSTRUCT,
            STRING_DESTROY,
            MEMSET_WRAPPER,
            FORMAT_INTO_STRING,
            SHOW_BOX,
            SHOW_PLAIN,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(IS_TEMPORARY, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x4000 != 0).into_ret()
        });
        e.register(OWNER_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(IDENTITY, |_, a| a[0].into_ret());
        e.register(TEXT_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TEXT_OR_EMPTY, |e, a| {
            let text = e.mem.u32(a[0] + 4);
            (if text == 0 { 0x0101_1584 } else { text }).into_ret()
        });
        e.register(NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        // The list head is stored in the form at +0x30 by the tests.
        e.register(ITEM_LIST_HEAD, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        // An item's conditions hold when the byte at item + 0x20 is set
        // (`this` is item + 8).
        e.register(CONDITIONS_ARE_TRUE, |e, a| {
            (e.mem.u8(a[0] - 8 + 0x20) != 0).into_ret()
        });
        e.register(IS_MESSAGE_BOX, |e, a| {
            (e.mem.u32(a[0] + 0x38) & 1 != 0).into_ret()
        });
        e.register(TITLE_PART, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(DISPLAY_TIME, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| 3u8.into_ret());
        // The description part: slot 0x10 answers a fixed format.
        e.put_vtable(DESCRIPTION_VTABLE, &[GET_DESCRIPTION; 8]);
        e.register(GET_DESCRIPTION, |_, _| FORMAT_TEXT.into_ret());
        e
    }

    /// A message form block: description vtable at +0x24, flags at +0x38.
    fn message_form(e: &mut Engine, is_box: bool) -> u32 {
        let form = e.mem.alloc(0x80);
        e.mem.set_u32(form + 0x24, DESCRIPTION_VTABLE);
        e.mem.set_u32(form + 0x38, is_box as u32);
        form
    }

    /// Builds the form's item list: one node per entry of `items`
    /// (`(text pointer, conditions hold)`).
    fn set_items(e: &mut Engine, form: u32, items: &[(u32, bool)]) {
        let mut next = 0;
        for (text, holds) in items.iter().rev() {
            let item = e.mem.alloc(0x30);
            e.mem.set_u32(item, *text);
            e.mem.set_u8(item + 0x20, *holds as u8);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        e.mem.set_u32(form + 0x30, next);
    }

    /// The script data (count 0), a reference with id 0x55, a script with id
    /// 0x66, and the command's words.
    struct Setup {
        args: ScriptArgs,
        reference: u32,
    }

    fn setup(e: &mut Engine, form: u32, with_reference: bool, with_script: bool) -> Setup {
        // The parameters double returns `form`.
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_u32(a[7], form);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0x10);
        let offset = e.mem.alloc(4);
        let reference = e.mem.alloc(0x20);
        e.mem.set_u32(reference + 0xc, 0x55);
        let script = e.mem.alloc(0x20);
        e.mem.set_u32(script + 0xc, 0x66);
        Setup {
            args: ScriptArgs {
                param_info: 1,
                script_data: data,
                this_obj: Ptr::new(if with_reference { reference } else { 0 }),
                containing_obj: Ptr::NULL,
                script_obj: Ptr::new(if with_script { script } else { 0 }),
                event_list: 0,
                result: Ptr::NULL,
                opcode_offset: offset,
            },
            reference,
        }
    }

    fn run(e: &mut Engine, s: &Setup) -> bool {
        let mut words = vec![];
        s.args.put(&mut words);
        e.call(0x005b_4630, &words).bool()
    }

    #[test]
    fn reset_clears_the_answer_and_owner() {
        let mut e = engine();
        e.set_global(MESSAGE_ANSWER, 4u8);
        e.set_global(MESSAGE_OWNER, 9u32);
        e.call(0x005b_4940, &args![]);
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 0xff);
        assert_eq!(e.global::<u32>(MESSAGE_OWNER), 0);
    }

    #[test]
    fn the_callback_stores_the_pressed_button() {
        let mut e = engine();
        e.call(0x005b_4a70, &args![]);
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 3);
    }

    #[test]
    fn the_formatter_reads_count_values_and_passes_ten_doubles() {
        let mut e = engine();
        let data = e.mem.alloc(0x20);
        // Two values follow the count at offset 0x10.
        e.mem.set_u16(data + 0x10, 2);
        let offset = e.mem.alloc(4);
        e.mem.set_u32(offset, 0x10);
        // The reader stores 1.5 and then 2.5.
        let mut n = 0;
        e.register_double(READ_NUMBER, move |e, a| {
            n += 1;
            e.mem.set_f64(a[0], n as f64 + 0.5);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let ok = e
            .call(
                0x005b_4960,
                &args![0u32, data, 1u32, 2u32, 3u32, 4u32, 0u32, offset, 0x8000u32, 0x9000u32],
            )
            .bool();
        assert!(ok);
        assert_eq!(e.mem.u32(offset), 0x12);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|c| c.0 == READ_NUMBER).collect();
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0].1[1..], [data, offset, 1, 2, 3, 4, 1]);
        let format = log.iter().find(|c| c.0 == FORMAT_INTO_STRING).unwrap();
        assert_eq!(format.1.len(), 2 + 20);
        assert_eq!(format.1[..2], [0x9000, 0x8000]);
        let doubles: Vec<f64> = (0..10)
            .map(|i| {
                f64::from_bits(format.1[2 + 2 * i] as u64 | (format.1[3 + 2 * i] as u64) << 32)
            })
            .collect();
        assert_eq!(doubles, [1.5, 2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn the_formatter_stops_at_a_value_it_cannot_read() {
        let mut e = engine();
        let data = e.mem.alloc(0x20);
        e.mem.set_u16(data, 3);
        let offset = e.mem.alloc(4);
        e.register(READ_NUMBER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let ok = e
            .call(
                0x005b_4960,
                &args![0u32, data, 0u32, 0u32, 0u32, 0u32, 0u32, offset, 1u32, 2u32],
            )
            .bool();
        assert!(!ok);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FORMAT_INTO_STRING));
    }

    #[test]
    fn a_missing_form_or_failed_parse_does_nothing() {
        let mut e = engine();
        let s = setup(&mut e, 0, true, true);
        assert!(!run(&mut e, &s));
        e.register(PARSE_PARAMETERS, |_, _| Ret::default());
        assert!(!run(&mut e, &s));
    }

    #[test]
    fn a_box_gets_nineteen_words_with_a_text_for_each_true_item() {
        let mut e = engine();
        let form = message_form(&mut e, true);
        e.mem.set_u32(form + 0x18 + 4, 0x6000); // the name part's text
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        set_items(
            &mut e,
            form,
            &[(0x4000, true), (0x4100, false), (0x4200, true)],
        );
        // The formatted string holds text 0x5000.
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x5000);
            Ret::default()
        });
        e.set_global(MESSAGE_ANSWER, 1u8);
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_BOX).unwrap();
        // Each true item gives the text at its first word; the false one the
        // empty text; the other slots stay 0.
        let mut expected = vec![0x5000, 0, 0x6000, 0x005b_4a70, 0, 0x17, 0, 0];
        expected.extend([0x4000, EMPTY_TEXT, 0x4200, 0, 0, 0, 0, 0, 0, 0]);
        expected.push(0);
        assert_eq!(shown.1, expected);
        assert!(log.iter().all(|c| c.0 != SHOW_PLAIN));
        // The answer was reset and the owner is the reference's id.
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 0xff);
        assert_eq!(e.global::<u32>(MESSAGE_OWNER), 0x55);
        // The string is destroyed.
        assert!(log.iter().any(|c| c.0 == STRING_DESTROY));
    }

    #[test]
    fn a_box_with_no_true_item_shows_the_stock_text_first() {
        let mut e = engine();
        let form = message_form(&mut e, true);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        set_items(&mut e, form, &[(0x4000, false), (0x4100, false)]);
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_BOX).unwrap();
        assert_eq!(shown.1[8], NO_TEXT);
        assert_eq!(shown.1[9], EMPTY_TEXT);
        assert_eq!(shown.1[10], 0);
    }

    #[test]
    fn a_form_without_items_still_gets_the_stock_text() {
        let mut e = engine();
        let form = message_form(&mut e, true);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_BOX).unwrap();
        assert_eq!(shown.1[8], NO_TEXT);
        assert_eq!(shown.1[9..18], [0; 9]);
    }

    #[test]
    fn only_ten_items_are_collected() {
        let mut e = engine();
        let form = message_form(&mut e, true);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        let items: Vec<(u32, bool)> = (0..12).map(|i| (0x4000 + 0x10 * i, true)).collect();
        set_items(&mut e, form, &items);
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_BOX).unwrap();
        let texts: Vec<u32> = shown.1[8..18].to_vec();
        let want: Vec<u32> = (0..10).map(|i| 0x4000 + 0x10 * i).collect();
        assert_eq!(texts, want);
    }

    #[test]
    fn a_plain_message_with_text_is_shown_with_its_name_and_time() {
        let mut e = engine();
        let form = message_form(&mut e, false);
        // The part at +0x2c is at 0x100 with its text at +0x18+4.
        let part = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x2c, part);
        e.mem.set_u32(part + 0x18 + 4, 0x6100);
        e.mem.set_u32(form + 0x3c, 5);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        // The string's length word is non-zero and its text pointer set.
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x5000);
            e.mem.set_u32(a[0] + 4, 3);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_PLAIN).unwrap();
        assert_eq!(shown.1, [0x5000, 0, 0x6100, 0, 5.0f32.to_bits(), 0]);
        assert!(log.iter().all(|c| c.0 != SHOW_BOX));
    }

    #[test]
    fn a_plain_message_with_no_text_shows_nothing_but_succeeds() {
        let mut e = engine();
        let form = message_form(&mut e, false);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != SHOW_PLAIN && c.0 != SHOW_BOX));
        assert!(log.iter().any(|c| c.0 == STRING_DESTROY));
    }

    #[test]
    fn a_plain_message_without_a_title_part_gets_a_null_name() {
        let mut e = engine();
        let form = message_form(&mut e, false);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        let shown = log.iter().find(|c| c.0 == SHOW_PLAIN).unwrap();
        assert_eq!(shown.1[2], 0);
    }

    #[test]
    fn the_owner_is_the_script_for_a_temporary_reference_and_nothing_fails_without_either() {
        let mut e = engine();
        let form = message_form(&mut e, false);
        let a = setup(&mut e, form, true, true);
        e.register(READ_NUMBER, |_, _| Ret::default());
        // A temporary reference (flag 0x4000): the script's id is the owner.
        e.mem.set_u32(a.reference + 8, 0x4000);
        assert!(run(&mut e, &a));
        assert_eq!(e.global::<u32>(MESSAGE_OWNER), 0x66);

        // No reference: the script again.
        let b = setup(&mut e, form, false, true);
        assert!(run(&mut e, &b));
        assert_eq!(e.global::<u32>(MESSAGE_OWNER), 0x66);

        // A temporary reference and no script, or neither: false, and the
        // string is still destroyed.
        let c = setup(&mut e, form, true, false);
        e.mem.set_u32(c.reference + 8, 0x4000);
        e.call_log = Some(vec![]);
        assert!(!run(&mut e, &c));
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|c| c.0 == STRING_DESTROY));
        let d = setup(&mut e, form, false, false);
        assert!(!run(&mut e, &d));
    }

    #[test]
    fn a_formatting_failure_returns_false_before_the_answer_is_reset() {
        let mut e = engine();
        let form = message_form(&mut e, false);
        let a = setup(&mut e, form, true, true);
        // One value to read, which fails.
        e.mem.set_u16(a.args.script_data, 1);
        e.register(READ_NUMBER, |_, _| Ret::default());
        e.set_global(MESSAGE_ANSWER, 2u8);
        e.call_log = Some(vec![]);
        assert!(!run(&mut e, &a));
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 2);
        assert!(log.iter().any(|c| c.0 == STRING_DESTROY));
        assert!(log.iter().all(|c| c.0 != SHOW_PLAIN && c.0 != SHOW_BOX));
    }
}

#[cfg(test)]
mod answer_b039_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0118_c000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.map(0x0101_a000, 0x1000);
        e.set_global(NO_ANSWER, -1.0f64);
        e.register(IS_TEMPORARY, |e, a| word(u32::from(e.mem.u32(a[0]) == 1)));
        e.register(OWNER_ID, |_, a| word(a[0] + 0x100));
        e
    }

    fn script_args(this_obj: u32, script_obj: u32, result: u32) -> ScriptArgs {
        ScriptArgs {
            param_info: 0,
            script_data: 0,
            this_obj: Ptr::new(this_obj),
            containing_obj: Ptr::new(0),
            script_obj: Ptr::new(script_obj),
            event_list: 0,
            result: Ptr::new(result),
            opcode_offset: 0,
        }
    }

    #[test]
    fn the_answer_for_the_reference_is_given_once() {
        let mut e = engine();
        let reference = e.mem.alloc(4);
        let result = e.mem.alloc(8);
        e.set_global(MESSAGE_ANSWER, 2u8);
        e.set_global(MESSAGE_OWNER, reference + 0x100);
        assert!(e
            .call(0x005b_4a80, &args![script_args(reference, 0, result)])
            .bool());
        assert_eq!(e.mem.f64(result), 2.0);
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 0xff);
        assert_eq!(e.global::<u32>(MESSAGE_OWNER), 0);
        assert!(e
            .call(0x005b_4a80, &args![script_args(reference, 0, result)])
            .bool());
        assert_eq!(e.mem.f64(result), -1.0);
    }

    #[test]
    fn a_temporary_reference_uses_the_script_and_no_caller_fails() {
        let mut e = engine();
        let reference = e.mem.alloc(4);
        e.mem.set_u32(reference, 1);
        let script = e.mem.alloc(4);
        let result = e.mem.alloc(8);
        e.set_global(MESSAGE_ANSWER, 0u8);
        e.set_global(MESSAGE_OWNER, script + 0x100);
        assert!(e
            .call(0x005b_4a80, &args![script_args(reference, script, result)])
            .bool());
        assert_eq!(e.mem.f64(result), 0.0);
        // Another owner: -1.
        e.set_global(MESSAGE_ANSWER, 1u8);
        e.set_global(MESSAGE_OWNER, 0x55u32);
        assert!(e
            .call(0x005b_4a80, &args![script_args(0, script, result)])
            .bool());
        assert_eq!(e.mem.f64(result), -1.0);
        assert_eq!(e.global::<u8>(MESSAGE_ANSWER), 1);
        // Neither a usable reference nor a script.
        assert!(!e
            .call(0x005b_4a80, &args![script_args(reference, 0, result)])
            .bool());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;
    use crate::units::fallout_shared::tesscriptfunctions::PARSE_PARAMETERS;

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

    fn script_args() -> ScriptArgs {
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::new(3),
            containing_obj: Ptr::new(4),
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::new(7),
            opcode_offset: 8,
        }
    }

    #[test]
    fn fn_005b4550_shows_the_parsed_text() {
        for (state, callback) in [(1u32, 0x0095_ee60u32), (0, 0x0095_ee70)] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            e.set_global(0x011d_e45c, 0x9999u32);
            e.register(PARSE_PARAMETERS, |_, _| ret(1));
            e.register_double(0x005a_9460, move |_, _| ret(state));
            e.register(0x004c_69f0, |_, _| ret(0x4242));
            e.register(SHOW_BOX, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let mut words = Vec::new();
            script_args().put(&mut words);
            assert!(e.call(0x005b_4550, &words).bool());
            let parse = calls(&e, PARSE_PARAMETERS);
            assert_eq!(parse.len(), 1);
            assert_eq!(&parse[0][..7], &[1, 2, 8, 3, 4, 5, 6]);
            let text = parse[0][7];
            assert_eq!(calls(&e, 0x005a_9460), vec![vec![0x9999]]);
            assert_eq!(calls(&e, 0x004c_69f0), vec![vec![0x011d_38b8]]);
            assert_eq!(
                calls(&e, SHOW_BOX),
                vec![vec![text, 0, 0, callback, 1, 0, 0, 0, 0x4242, 0]]
            );
        }
        let mut e = Engine::new();
        e.register(PARSE_PARAMETERS, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let mut words = Vec::new();
        script_args().put(&mut words);
        assert!(!e.call(0x005b_4550, &words).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
