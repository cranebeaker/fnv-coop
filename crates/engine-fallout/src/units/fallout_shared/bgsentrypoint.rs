//! `fallout shared/bgsentrypoint.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player reference (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The actor test that makes the player's perks apply (`00566950`; the
/// engine map's name `MiddleHighProcess::GetForceNextUpdate` belongs to
/// folded code).
const ACTOR_TEST_00566950: u32 = 0x0056_6950;
/// A list node's item address (`006815c0`, the node itself) and next node
/// (`00726070`, the word at +4).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The entry point table: 16 bytes per entry point, its name (+0), the
/// filter argument count (byte +4) and the function type (byte +0xC).
const ENTRY_POINT_TABLE: u32 = 0x0119_6ee0;
const ENTRY_POINT_SIZE: u32 = 0x10;
/// The number of entry points the code accepts (below 0x4A).
const ENTRY_POINT_COUNT: i32 = 0x4a;
/// The function argument count per function type (a byte every 4 bytes).
const FUNCTION_ARGUMENT_COUNTS: u32 = 0x0119_7400;
/// The function table: 12 bytes per function kind, the handler at +0.
const FUNCTION_TABLE: u32 = 0x0119_7390;
/// The debug message function (`005b5e40`, cdecl; its body only answers 0)
/// and its strings: `"DEFAULT: Invalid %s to entry point function %s,
/// expected %i."`, `"parameters"` and `"arguments"`.
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
const MESSAGE_FORMAT: u32 = 0x0104_58a4;
const MESSAGE_PARAMETERS: u32 = 0x0104_58f0;
const MESSAGE_ARGUMENTS: u32 = 0x0104_58e4;

// Translated from 005e5b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of function arguments of a function type (cdecl; the byte of
/// the 4-byte table entry at `01197400`).
pub fn fn_005e5b30(e: &mut Engine, function_type: u32) -> u8 {
    e.mem
        .u8(FUNCTION_ARGUMENT_COUNTS.wrapping_add(function_type.wrapping_mul(4)))
}

// Translated from 005e5b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the handler of a function kind (cdecl): when the 12-byte entry of
/// the table at `01197390` has a handler, it gets the five other words
/// (the third as its low byte).
pub fn fn_005e5b40(
    e: &mut Engine,
    kind: u32,
    actor: u32,
    function_type: u32,
    count: u32,
    arguments: u32,
    function: u32,
) {
    let handler = e
        .mem
        .u32(FUNCTION_TABLE.wrapping_add(kind.wrapping_mul(0xc)));
    if handler != 0 {
        e.call(
            handler,
            &args![actor, function_type, count & 0xff, arguments, function],
        );
    }
}

// Translated from 005e58f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB; cdecl with variable
/// arguments). For an actor and an entry point below 0x4A: the actor's (or,
/// when it is not the player and passes `00566950`, the player's) perk
/// entries for the entry point (its virtual `+0x4AC`). The extra words are
/// the entry point's filter arguments after the actor, then its function
/// type's arguments (counts from the tables). With every argument set, each
/// entry whose conditions pass (its virtual `+0` with the filters) runs its
/// function kind (virtual `+4`) on the arguments with its value (virtual
/// `+8`, `005e5b40`); otherwise the debug message gets "parameters" and the
/// filter count when the filters are all set, else "arguments" and the
/// function argument count, with the entry point's name.
pub fn bgs_entry_point_handle_entry_point(
    e: &mut Engine,
    entry_point: u32,
    actor: Ptr,
    extra: &[u32],
) {
    if actor.is_null() || entry_point as i32 >= ENTRY_POINT_COUNT {
        return;
    }
    let original = actor.addr();
    let mut owner = actor.addr();
    let mut forced = false;
    if owner != e.global::<u32>(PLAYER) && e.call(ACTOR_TEST_00566950, &args![owner]).bool() {
        forced = true;
        owner = e.global::<u32>(PLAYER);
    }
    let mut list = e
        .vcall(owner, 0x4ac, &args![entry_point & 0xff, u32::from(forced)])
        .u32();
    if list == 0 {
        return;
    }
    let first = e.call(NODE_ITEM, &args![list]).u32();
    if e.mem.u32(first) == 0 {
        return;
    }
    let actor = original;
    let entry = ENTRY_POINT_TABLE.wrapping_add(entry_point.wrapping_mul(ENTRY_POINT_SIZE));
    let filter_count = u32::from(e.mem.u8(entry + 4));
    let function_type = u32::from(e.mem.u8(entry + 0xc));
    let function_count = u32::from(fn_005e5b30(e, function_type));
    // The extra words as the exe's variable argument list, then the two
    // local arrays (filters with the actor first, function arguments).
    let words = (extra.len() as u32).max(1) * 4;
    e.with_stack(words + 0x28 + 0x28, |e, block| {
        let varargs = block.addr();
        for (i, word) in extra.iter().enumerate() {
            e.mem.set_u32(varargs + 4 * i as u32, *word);
        }
        let filters = varargs + words;
        let arguments = filters + 0x28;
        e.mem.set_u32(filters, actor);
        let mut cursor = varargs;
        let mut filters_set = true;
        let mut index = 1u32;
        while index < filter_count {
            let value = e.mem.u32(cursor);
            cursor += 4;
            e.mem.set_u32(filters + 4 * index, value);
            if value == 0 {
                filters_set = false;
            }
            index += 1;
        }
        let mut arguments_set = true;
        let mut index = 0u32;
        while index < function_count {
            let value = e.mem.u32(cursor);
            cursor += 4;
            e.mem.set_u32(arguments + 4 * index, value);
            if value == 0 {
                arguments_set = false;
            }
            index += 1;
        }
        if filters_set && arguments_set {
            while list != 0 {
                let item = e.call(NODE_ITEM, &args![list]).u32();
                let perk_entry = e.mem.u32(item);
                if perk_entry != 0 && e.vcall(perk_entry, 0, &args![filter_count, filters]).bool() {
                    let value = e.vcall(perk_entry, 8, &args![]).u32();
                    let kind = u32::from(e.vcall(perk_entry, 4, &args![]).u8());
                    fn_005e5b40(
                        e,
                        kind,
                        actor,
                        function_type,
                        function_count,
                        arguments,
                        value,
                    );
                }
                list = e.call(NODE_NEXT, &args![list]).u32();
            }
            return;
        }
        let count = if filters_set {
            filter_count
        } else {
            function_count
        };
        let text = if filters_set {
            MESSAGE_PARAMETERS
        } else {
            MESSAGE_ARGUMENTS
        };
        let name = e.mem.u32(entry);
        e.call(DEBUG_MESSAGE, &args![MESSAGE_FORMAT, text, name, count]);
    });
}

/// The uniform form of `HandleEntryPoint`: the entry point, the actor, then
/// the extra words.
fn handle_entry_point_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "HandleEntryPoint needs 2 argument words, got {}",
        words.len()
    );
    bgs_entry_point_handle_entry_point(e, words[0], Ptr::new(words[1]), &words[2..]);
    Ret::default()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e5b30, fn_005e5b30(u32) -> u8),
        entry!(0x005e5b40, fn_005e5b40(u32, u32, u32, u32, u32, u32)),
        (0x005e58f0, handle_entry_point_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn function_argument_counts_and_handlers() {
        let mut e = Engine::new();
        e.map(FUNCTION_ARGUMENT_COUNTS, 0x40);
        e.mem.set_u8(FUNCTION_ARGUMENT_COUNTS + 8, 3);
        assert_eq!(e.call(0x005e_5b30, &args![2u32]).u8(), 3);
        e.map(FUNCTION_TABLE, 0x30);
        e.mem.set_u32(FUNCTION_TABLE + 0xc, 0x0650_0000);
        e.register(0x0650_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x005e_5b40,
            &args![1u32, 0x10u32, 0x20u32, 0x1ffu32, 0x40u32, 0x50u32],
        );
        assert_eq!(
            calls_to(&e, 0x0650_0000),
            vec![vec![0x10, 0x20, 0xff, 0x40, 0x50]]
        );
        e.call(0x005e_5b40, &args![2u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls_to(&e, 0x0650_0000).len(), 1);
    }

    /// An entry point table with entry 5: 2 filters, function type 1 of 1
    /// argument; a perk entry list for the actor.
    fn setup(e: &mut Engine, conditions: bool) -> (u32, u32) {
        e.map(PLAYER, 4);
        e.map(ENTRY_POINT_TABLE, 0x100);
        e.mem.set_u32(ENTRY_POINT_TABLE + 0x50, 0x0105_0000);
        e.mem.set_u8(ENTRY_POINT_TABLE + 0x54, 2);
        e.mem.set_u8(ENTRY_POINT_TABLE + 0x5c, 1);
        e.map(FUNCTION_ARGUMENT_COUNTS, 0x10);
        e.mem.set_u8(FUNCTION_ARGUMENT_COUNTS + 4, 1);
        e.map(FUNCTION_TABLE, 0x30);
        e.mem.set_u32(FUNCTION_TABLE + 0x18, 0x0650_0000);
        e.register_double(0x0650_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0x77);
            Ret::default()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ACTOR_TEST_00566950, |_, _| false.into_ret());
        e.register(DEBUG_MESSAGE, |_, _| Ret::default());
        let perk_vtable = e.mem.alloc(12);
        for (slot, target) in [(0u32, 0x0650_0100u32), (4, 0x0650_0104), (8, 0x0650_0108)] {
            e.mem.set_u32(perk_vtable + slot, target);
        }
        e.register_double(0x0650_0100, move |e, a| {
            assert_eq!(a[1], 2);
            assert_eq!(e.mem.u32(a[2] + 4), 0x66);
            conditions.into_ret()
        });
        e.register(0x0650_0104, |_, _| 2u32.into_ret());
        e.register(0x0650_0108, |_, _| 0x99u32.into_ret());
        let perk = e.mem.alloc(4);
        e.mem.set_u32(perk, perk_vtable);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, perk);
        let actor_vtable = e.mem.alloc(0x4b0);
        e.mem.set_u32(actor_vtable + 0x4ac, 0x0650_0200);
        e.register_double(0x0650_0200, move |_, _| list.into_ret());
        let actor = e.mem.alloc(4);
        e.mem.set_u32(actor, actor_vtable);
        (actor, list)
    }

    #[test]
    fn handle_entry_point_runs_matching_entries() {
        let mut e = Engine::new();
        let (actor, _) = setup(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x005e_58f0, &args![5u32, actor, 0x66u32, 0x77u32]);
        assert_eq!(calls_to(&e, 0x0650_0200), vec![vec![actor, 5, 0]]);
        let handled = calls_to(&e, 0x0650_0000);
        assert_eq!(handled.len(), 1);
        assert_eq!(handled[0][..3], [actor, 1, 1]);
        assert_eq!(handled[0][4], 0x99);
        // Conditions failing: nothing runs.
        let mut e = Engine::new();
        let (actor, _) = setup(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x005e_58f0, &args![5u32, actor, 0x66u32, 0x77u32]);
        assert!(calls_to(&e, 0x0650_0000).is_empty());
    }

    #[test]
    fn handle_entry_point_reports_missing_arguments() {
        let mut e = Engine::new();
        let (actor, _) = setup(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x005e_58f0, &args![5u32, actor, 0u32, 0x77u32]);
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGE),
            vec![vec![MESSAGE_FORMAT, MESSAGE_ARGUMENTS, 0x0105_0000, 1]]
        );
        e.call(0x005e_58f0, &args![5u32, actor, 0x66u32, 0u32]);
        assert_eq!(
            calls_to(&e, DEBUG_MESSAGE)[1],
            vec![MESSAGE_FORMAT, MESSAGE_PARAMETERS, 0x0105_0000, 2]
        );
        assert!(calls_to(&e, 0x0650_0000).is_empty());
    }

    #[test]
    fn handle_entry_point_needs_an_actor_and_a_valid_entry_point() {
        let mut e = Engine::new();
        let (actor, list) = setup(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x005e_58f0, &args![0x4au32, actor]);
        e.call(0x005e_58f0, &args![5u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // An empty first entry: nothing more.
        e.mem.set_u32(list, 0);
        e.call(0x005e_58f0, &args![5u32, actor, 0x66u32, 0x77u32]);
        assert!(calls_to(&e, NODE_NEXT).is_empty());
        // The forced player's list.
        e.mem.set_u32(list, 0);
        let player = actor;
        let other = e.mem.alloc(4);
        e.set_global(PLAYER, player);
        e.register(ACTOR_TEST_00566950, |_, _| true.into_ret());
        e.call(0x005e_58f0, &args![5u32, other, 0x66u32, 0x77u32]);
        assert_eq!(
            calls_to(&e, 0x0650_0200).last().unwrap(),
            &vec![player, 5, 1]
        );
    }
}
