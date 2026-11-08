//! The typed family `topic_order` (docs/ENGINE_CRATE.md, "Typed code"): the order of a topic list,
//! the compare the sorted insert (`0061a380`) and the sort (`0061a5a0`) ask of each node, and the
//! functions it calls.
//!
//! Each topic record a plugin loads is put into the data handler's topic list by the sorted insert,
//! which walks the list from the start and asks the compare (`0061a450`) of every node until the
//! new topic belongs before one: about 21,000 inserts into a list of up to 21,000 topics with the
//! base game and its add-ons, more with every mod that adds dialogue. Each compare asks both topics
//! for their editor ids (vtable slot 0x130), with priorities both topics' priorities and a float
//! check, and on equal first letters the case-insensitive compare: in the translations every one of
//! those is a call by address; here they are Rust calls once the family is on.
//!
//! Same results, the same calls in the same order and the same reads as the exe, and nothing here
//! writes guest memory. `NV_TYPED=topic_order:off` runs the translations. The callees outside this
//! family are the `leaves` family's (`engine_core::typed::leaf`); with that family off they go by
//! address.

#![deny(clippy::cast_possible_truncation)]

use crate::{entry, AbiFn, Engine};
use engine_core::args;
use engine_core::typed::leaf;
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::typed_call;
use engine_core::x87::{x87_load32, x87_quiet, x87_quiet_bits, x87_store32, x87_sub};

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(6);

/// `NV_TYPED=topic_order`. Past its gate (2026-11-08, release build, fixes preset, two runs each): about 0.5 s
/// off the start-up data load, clear with the five add-ons alone (the load's last sort from the process start
/// 3.18-3.46 s off against 2.67-2.96 s on; the first main frame 4.71-4.90 against 3.93-4.29 s), with YUP and
/// SR556 beside them within the noise of two runs (3.53-4.40 against 3.27-3.53 s). On unless `NV_TYPED` says
/// otherwise (`NV_TYPED=off` or `topic_order:off`).
///
/// Its members are game-wide helpers, not the topic list's alone: the case-insensitive compare `00404dc0`
/// (which animation, scripts, models and AI call too), the float check `0040eb10`, the priority `00526ae0`
/// and the editor id `00618660` run typed for every caller. So `topic_order:off` changes more than the topic
/// order, and while the family is on a trace shows no CRT calls (`00ec68e4`, `00ec67d6`) under `00404dc0`.
pub static TOPIC_ORDER: Family = Family {
    name: "topic_order",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// `TESTopic`'s editor id, the function its vtable holds at slot 0x130.
const EDITOR_ID: u32 = 0x0061_8660;
/// The empty string the editor id answers for a topic without one.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The priority difference above which a topic sorts first (a `double`), and the `float` the
/// float check is given as its epsilon.
const PRIORITY_LIMIT: u32 = 0x0103_0f28;
const PRIORITY_EPSILON: u32 = 0x0101_7d00;
/// The `double` 0.0 the float check compares its epsilon with.
const ZERO: u32 = 0x0101_2060;
/// The CRT's `_stricmp` (`00ec68e4`) and its "the locale was changed" word.
const CRT_STRICMP: u32 = 0x00ec_68e4;
const LOCALE_CHANGED: u32 = 0x0126_fdf4;

// Typed from 00618660
/// `TESTopic`'s editor id (its vtable slot 0x130): the text of the string at +0x40 (`00559450`,
/// asked again for the answer), or the empty string at `01011584` when that is null.
#[inline]
pub fn editor_id(e: &mut Engine, this: u32) -> u32 {
    let text = this.wrapping_add(0x40);
    if typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, text)) == 0 {
        EMPTY_STRING
    } else {
        typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, text))
    }
}

// Typed from 00526ae0
/// A topic's priority: the `float` at +0x28, as `FLD` loads it (a signalling NaN quieted).
#[inline]
pub fn priority(e: &mut Engine, this: u32) -> f32 {
    x87_quiet(e.mem.f32(this.wrapping_add(0x28)))
}

// Typed from 0040eb10
/// Whether `a` and `b` differ by less than `epsilon` (cdecl, three floats): false when `epsilon`
/// is below the `double` 0.0 at `01012060`, else `epsilon > |a - b|` in doubles, the difference
/// taken as `a - b` when `b < a` and as `b - a` otherwise (also with a NaN). A NaN gives false.
#[inline]
pub fn approx_equal(e: &mut Engine, a: f32, b: f32, epsilon: f32) -> bool {
    let zero: f64 = e.global(ZERO);
    if f64::from(epsilon) < zero {
        return false;
    }
    let (a, b) = (f64::from(a), f64::from(b));
    let difference = if b < a { a - b } else { b - a };
    f64::from(epsilon) > difference
}

// Typed from 00404dc0
/// The case-insensitive compare of two strings (cdecl): the CRT's `_stricmp` (`00ec68e4`). With
/// the locale unchanged and both strings there, that is the ASCII compare (`00ec67d6`), done here:
/// each byte with A-Z folded to a-z, until a zero or a difference, whose signed value it answers.
/// Otherwise (a changed locale, a null string, which the CRT reports) the CRT's by address.
pub fn stricmp(e: &mut Engine, left: u32, right: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 || left == 0 || right == 0 {
        return e.call(CRT_STRICMP, &args![left, right]).i32();
    }
    let (mut a, mut b) = (left, right);
    loop {
        let mut x = u32::from(e.mem.u8(a));
        a = a.wrapping_add(1);
        if x.wrapping_sub(0x41) <= 0x19 {
            x += 0x20;
        }
        let mut y = u32::from(e.mem.u8(b));
        b = b.wrapping_add(1);
        if y.wrapping_sub(0x41) <= 0x19 {
            y += 0x20;
        }
        if x == 0 || x != y {
            return i32::from_ne_bytes(x.wrapping_sub(y).to_ne_bytes());
        }
    }
}

/// A form's editor id through its vtable slot 0x130: `TESTopic`'s (`00618660`) as a typed call,
/// anything else as the exe calls it.
#[inline]
fn name(e: &mut Engine, form: u32) -> u32 {
    let vtable = e.mem.u32(form);
    if e.mem.u32(vtable.wrapping_add(0x130)) == EDITOR_ID {
        typed_call!(FAMILY, 0x0061_8660, editor_id(e, form))
    } else {
        e.vcall(form, 0x130, &[]).u32()
    }
}

// Typed from 0061a450
/// Whether topic `a` sorts before topic `b` (cdecl: a, b, by priority as a byte): false when either
/// is null. Both editor ids are asked for first (`a`'s, then `b`'s). With `by_priority`: the
/// difference of the priorities `b - a` (`00526ae0`, `b`'s first; x87, stored as a `float`) above
/// the `double` at `01030f28` gives true; one that the float check (`0040eb10` with 0.0 and the
/// `float` at `01017d00`) calls equal goes on to the names; any other gives false. By name: true
/// when `a`'s first character (signed) is below `b`'s, else when the case-insensitive compare
/// (`00404dc0`, the ids asked for again, `b`'s first) of `a`'s id with `b`'s is above 0.
pub fn compare(e: &mut Engine, a: u32, b: u32, by_priority: u8) -> bool {
    if a == 0 || b == 0 {
        return false;
    }
    let name_a = name(e, a);
    let name_b = name(e, b);
    if by_priority != 0 {
        let priority_b = f64::from(typed_call!(FAMILY, 0x0052_6ae0, priority(e, b)));
        let priority_a = f64::from(typed_call!(FAMILY, 0x0052_6ae0, priority(e, a)));
        let difference = x87_store32(x87_sub(priority_b, priority_a));
        let limit: f64 = e.global(PRIORITY_LIMIT);
        // FCOMP, then `TEST AH,0x41` (C0 or C3): below, equal or unordered go on.
        if x87_load32(difference) > limit {
            return true;
        }
        let epsilon = x87_quiet_bits(e.global::<u32>(PRIORITY_EPSILON));
        let equal = typed_call!(
            FAMILY,
            0x0040_eb10,
            approx_equal(
                e,
                f32::from_bits(difference),
                f32::from_bits(0),
                f32::from_bits(epsilon)
            )
        );
        if !equal {
            return false;
        }
    }
    let first_a = i32::from(e.mem.i8(name_a));
    let first_b = i32::from(e.mem.i8(name_b));
    if first_a < first_b {
        return true;
    }
    let name_b = name(e, b);
    let name_a = name(e, a);
    typed_call!(FAMILY, 0x0040_4dc0, stricmp(e, name_a, name_b)) > 0
}

/// The family's functions at their exe addresses.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_8660, editor_id(u32) -> u32),
        entry!(0x0052_6ae0, priority(u32) -> f32),
        entry!(0x0040_eb10, approx_equal(f32, f32, f32) -> bool),
        entry!(0x0040_4dc0, stricmp(u32, u32) -> i32),
        entry!(0x0061_a450, compare(u32, u32, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    //! Each function against its translation (`engine_core::typed::diff`), on topics laid out as the
    //! exe's: a vtable whose slot 0x130 is `TESTopic`'s editor id (or another function), the editor
    //! id's string pointer at +0x40, the priority at +0x28.

    use super::*;
    use engine_core::typed::diff::{Diff, Input};

    /// A vtable of 0x140 bytes whose slot 0x130 is `slot`.
    fn vtable(e: &mut Engine, slot: u32) -> u32 {
        let v = e.mem.alloc(0x140);
        e.mem.set_u32(v + 0x130, slot);
        v
    }

    fn c_string(e: &mut Engine, text: &[u8]) -> u32 {
        let p = e.mem.alloc(u32::try_from(text.len()).unwrap() + 1);
        e.mem.write(p, text);
        e.mem.set_u8(p + u32::try_from(text.len()).unwrap(), 0);
        p
    }

    /// Maps the words the compare and its callees read: the empty string, the limit, the epsilon,
    /// the zero and the locale word (`locale` its value).
    fn globals(e: &mut Engine, locale: u32) {
        for (at, len) in [
            (EMPTY_STRING, 4),
            (PRIORITY_LIMIT, 8),
            (PRIORITY_EPSILON, 4),
            (ZERO, 8),
            (LOCALE_CHANGED, 4),
        ] {
            if !e.mem.is_mapped(at) || !e.mem.is_mapped(at + len - 1) {
                e.map(at, len);
            }
        }
        e.mem.set_u32(EMPTY_STRING, 0);
        e.mem.set_u32(PRIORITY_LIMIT, 0xeb1c_432d);
        e.mem.set_u32(PRIORITY_LIMIT + 4, 0x3f1a_36e2);
        e.mem.set_u32(PRIORITY_EPSILON, 0x3a83_126f);
        e.mem.set_u64(ZERO, 0);
        e.mem.set_u32(LOCALE_CHANGED, locale);
    }

    /// The names the tests compare: case and first letters that differ, a prefix, the empty one; then
    /// the fold's edges and bytes it leaves alone, also as the first character: 'Z' folds to 'z', '@'
    /// (below 'A'), '[' (above 'Z'), '`' and '{' (around the lower case) don't, and neither does a byte
    /// from 0x80 (as a first character, signed, below every ASCII one).
    const NAMES: [&[u8]; 12] = [
        b"GREETING",
        b"greeting",
        b"Goodbye",
        b"NVDLC01Topic",
        b"G",
        b"",
        b"Zebra",
        b"@home",
        b"[x",
        b"`tick",
        b"{brace",
        b"\xe9clair",
    ];

    /// Every name's index, and one past them: a topic without an editor id.
    const NAME_INDEXES: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    const TOPIC_INDEXES: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    /// A topic with `TESTopic`'s vtable slot, editor id `NAMES[name]` (none past them) and priority
    /// `priority`.
    fn topic(e: &mut Engine, name: usize, priority: f32) -> u32 {
        let t = e.mem.alloc(0x50);
        let v = vtable(e, EDITOR_ID);
        e.mem.set_u32(t, v);
        let text = if name < NAMES.len() {
            c_string(e, NAMES[name])
        } else {
            0
        };
        e.mem.set_u32(t + 0x40, text);
        e.mem.set_f32(t + 0x28, priority);
        t
    }

    #[test]
    fn editor_id_as_translated() {
        Diff::new(entry!(0x0061_8660, editor_id(u32) -> u32))
            .family(&TOPIC_ORDER)
            .family(&leaf::LEAVES)
            .inputs(&[Input::Of(&[0, 3, 11, 12])])
            .setup(|e, case| {
                globals(e, 0);
                let index = usize::try_from(case.word(0)).unwrap();
                vec![topic(e, index, 0.0)]
            })
            .run()
            .assert_same();
    }

    #[test]
    fn priority_as_translated() {
        Diff::new(entry!(0x0052_6ae0, priority(u32) -> f32))
            .family(&TOPIC_ORDER)
            .inputs(&[Input::Of(&[
                0,
                0x3f80_0000,
                0xbf80_0000,
                0x7f80_0001,
                0xffc0_0000,
            ])])
            .setup(|e, case| {
                let t = e.mem.alloc(0x30);
                e.mem.set_u32(t + 0x28, case.word(0));
                vec![t]
            })
            .run()
            .assert_same();
    }

    /// Every float the harness makes (NaNs, signalling ones too, zeros, denormals, huge values).
    #[test]
    fn priority_of_any_float_as_translated() {
        Diff::new(entry!(0x0052_6ae0, priority(u32) -> f32))
            .family(&TOPIC_ORDER)
            .inputs(&[Input::F32])
            .setup(|e, case| {
                let t = e.mem.alloc(0x30);
                e.mem.set_u32(t + 0x28, case.f32(0).to_bits());
                vec![t]
            })
            .run()
            .assert_same();
    }

    /// The float check on the harness's floats in every argument.
    #[test]
    fn approx_equal_of_any_floats_as_translated() {
        Diff::new(entry!(0x0040_eb10, approx_equal(f32, f32, f32) -> bool))
            .family(&TOPIC_ORDER)
            .inputs(&[Input::F32, Input::F32, Input::F32])
            .setup(|e, case| {
                globals(e, 0);
                vec![
                    case.f32(0).to_bits(),
                    case.f32(1).to_bits(),
                    case.f32(2).to_bits(),
                ]
            })
            .run()
            .assert_same();
    }

    #[test]
    fn approx_equal_as_translated() {
        let floats = Input::Of(&[
            0,
            0x8000_0000,
            0x3f80_0000,
            0x3a83_126f,
            0xbf80_0000,
            0x7f80_0000,
            0x7fc0_0000,
        ]);
        Diff::new(entry!(0x0040_eb10, approx_equal(f32, f32, f32) -> bool))
            .family(&TOPIC_ORDER)
            .inputs(&[floats, floats, floats])
            .setup(|e, case| {
                globals(e, 0);
                vec![case.word(0), case.word(1), case.word(2)]
            })
            .run()
            .assert_same();
    }

    /// With the locale unchanged and both strings there: the ASCII compare, done in the body (the
    /// CRT's calls are the translation's only).
    #[test]
    fn stricmp_as_translated() {
        Diff::new(entry!(0x0040_4dc0, stricmp(u32, u32) -> i32))
            .family(&TOPIC_ORDER)
            .ignore_calls(&[CRT_STRICMP])
            .inputs(&[Input::Of(NAME_INDEXES), Input::Of(NAME_INDEXES)])
            .setup(|e, case| {
                globals(e, 0);
                let a = c_string(e, NAMES[usize::try_from(case.word(0)).unwrap()]);
                let b = c_string(e, NAMES[usize::try_from(case.word(1)).unwrap()]);
                vec![a, b]
            })
            .run()
            .assert_same();
    }

    /// A changed locale, or a null string with the locale changed or not: the CRT's `_stricmp` by
    /// address on both sides (a double there for both engines: the CRT's own path reaches imports
    /// these engines don't have), nothing dropped.
    #[test]
    fn stricmp_hands_a_changed_locale_and_nulls_to_the_crt() {
        Diff::new(entry!(0x0040_4dc0, stricmp(u32, u32) -> i32))
            .family(&TOPIC_ORDER)
            .inputs(&[Input::Of(&[0, 1, 2, 3, 4])])
            .setup(|e, case| {
                let (locale, left, right) = match case.word(0) {
                    0 => (1, true, true),
                    1 => (1, false, true),
                    2 => (1, true, false),
                    3 => (0, false, true),
                    _ => (0, true, false),
                };
                globals(e, locale);
                e.register(CRT_STRICMP, |_, a| engine_core::abi::Ret {
                    eax: a[0] ^ a[1] ^ 0x5a,
                    ..Default::default()
                });
                let a = if left { c_string(e, b"Topic") } else { 0 };
                let b = if right { c_string(e, b"topic") } else { 0 };
                vec![a, b]
            })
            .run()
            .assert_same();
    }

    /// By name: every pair of the names (and a topic without one), the priorities equal, so with
    /// the flag too the names decide.
    #[test]
    fn compare_by_name_as_translated() {
        Diff::new(entry!(0x0061_a450, compare(u32, u32, u8) -> bool))
            .family(&TOPIC_ORDER)
            .family(&leaf::LEAVES)
            .ignore_calls(&[CRT_STRICMP])
            // Every combination: 13 x 13 names x 3 flags.
            .max_cases(1024)
            .inputs(&[
                Input::Of(TOPIC_INDEXES),
                Input::Of(TOPIC_INDEXES),
                // Only the flag word's low byte counts.
                Input::Of(&[0, 1, 0x100]),
            ])
            .setup(|e, case| {
                globals(e, 0);
                let a = topic(e, usize::try_from(case.word(0)).unwrap(), 50.0);
                let b = topic(e, usize::try_from(case.word(1)).unwrap(), 50.0);
                vec![a, b, case.word(2)]
            })
            .run()
            .assert_same();
    }

    /// By priority: each pair of priorities over names that sort either way, alike, by a first
    /// character from 0x80, or not at all.
    #[test]
    fn compare_by_priority_as_translated() {
        // Priorities: equal, one ahead past the limit, one behind, within the epsilon; near equal
        // either way, under the limit and within the epsilon, so on to the names; a NaN.
        const PRIORITIES: [(f32, f32); 7] = [
            (50.0, 50.0),
            (40.0, 60.0),
            (60.0, 40.0),
            (50.0, 50.0005),
            (50.0005, 50.0),
            (50.0, 50.00005),
            (50.0, f32::NAN),
        ];
        Diff::new(entry!(0x0061_a450, compare(u32, u32, u8) -> bool))
            .family(&TOPIC_ORDER)
            .family(&leaf::LEAVES)
            .ignore_calls(&[CRT_STRICMP])
            // Every combination: 4 x 4 names x 7 priority pairs x 3 flags.
            .max_cases(1024)
            .inputs(&[
                Input::Of(&[0, 6, 11, 12]),
                Input::Of(&[0, 6, 11, 12]),
                Input::Of(&[0, 1, 2, 3, 4, 5, 6]),
                // Only the flag word's low byte counts.
                Input::Of(&[0, 1, 0x100]),
            ])
            .setup(|e, case| {
                globals(e, 0);
                let (pa, pb) = PRIORITIES[usize::try_from(case.word(2)).unwrap()];
                let a = topic(e, usize::try_from(case.word(0)).unwrap(), pa);
                let b = topic(e, usize::try_from(case.word(1)).unwrap(), pb);
                vec![a, b, case.word(3)]
            })
            .run()
            .assert_same();
    }

    /// A null topic, and a form whose slot 0x130 is not `TESTopic`'s: called as the exe calls it.
    #[test]
    fn compare_of_other_forms_as_translated() {
        const OTHER_NAME: u32 = 0x7000_0130;
        Diff::new(entry!(0x0061_a450, compare(u32, u32, u8) -> bool))
            .family(&TOPIC_ORDER)
            .family(&leaf::LEAVES)
            .ignore_calls(&[CRT_STRICMP])
            .inputs(&[Input::Of(&[0, 1, 2]), Input::Of(&[0, 1])])
            .setup(|e, case| {
                globals(e, 0);
                e.register(OTHER_NAME, |e, a| {
                    let text = e.mem.u32(a[0] + 0x44);
                    engine_core::abi::Ret {
                        eax: text,
                        ..Default::default()
                    }
                });
                let other = e.mem.alloc(0x50);
                let v = vtable(e, OTHER_NAME);
                e.mem.set_u32(other, v);
                let text = c_string(e, b"Rumors");
                e.mem.set_u32(other + 0x44, text);
                let t = topic(e, 0, 50.0);
                let a = match case.word(0) {
                    0 => 0,
                    1 => other,
                    _ => t,
                };
                let b = if case.word(1) == 0 { t } else { other };
                vec![a, b, 0]
            })
            .run()
            .assert_same();
    }
}
