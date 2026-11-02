//! `BSSystemUtilities/bsawardssystemutility_xenon.cpp` (Xbox PDB source unit), subsystem
//! `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The PC build keeps the unlocked awards in a simple list (two
//! words a node: the item, the next node) and hands them to Steam (`SteamUserStats`) in its
//! idle step.

#[allow(unused_imports)]
use crate::prelude::*;

/// The singleton (`011f7178`), the memory manager (`011f6238`) and its allocation and
/// release (`00aa3e40`, `00aa4060`, `thiscall`, one word).
const INSTANCE: u32 = 0x011f_7178;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
/// The simple list's clear (`00ae83e0`, `thiscall`, no stack words) and its tail add
/// (`00af25b0`, `thiscall`, one word: the address of the item word).
const LIST_CLEAR: u32 = 0x00ae_83e0;
const LIST_ADD_TAIL: u32 = 0x00af_25b0;

// Translated from 00af22d0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af22d0 - the SEH frame (FS:[0] handler 00f22f1b, state 0 around the constructor call) is not translated.
/// `BSAwardsSystemUtility::QInstance` (engine map; no arguments): when the singleton
/// (`011f7178`) is null, an 8-byte block is allocated (`00aa3e40` on the memory manager) and
/// constructed ([`fn_00af23d0`]; a failed allocation gives null), and the result kept.
/// Answers the singleton.
pub fn bs_awards_system_utility_q_instance(e: &mut Engine) -> u32 {
    if e.global::<u32>(INSTANCE) == 0 {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
        let object = if block != 0 { fn_00af23d0(e, block) } else { 0 };
        e.set_global::<u32>(INSTANCE, object);
    }
    e.global::<u32>(INSTANCE)
}

// Translated from 00af2360 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSAwardsSystemUtility::Initialize` (engine map; no arguments): makes the singleton
/// ([`bs_awards_system_utility_q_instance`]) and passes its answer on (EAX).
pub fn bs_awards_system_utility_initialize(e: &mut Engine) -> u32 {
    bs_awards_system_utility_q_instance(e)
}

// Translated from 00af2370 (disassembled, FalloutNV.exe 1.4.0.525)
/// The singleton's release (no label; no arguments): when the singleton is not null, its list
/// is cleared ([`fn_00af2400`]) and the block freed (`00aa4060`; the inlined deleting
/// destructor's flag is the constant 1, so the free always runs), and the singleton set to
/// null. The inner null test repeats the outer one, so its null branch cannot run; it is kept
/// as the exe has it.
pub fn fn_00af2370(e: &mut Engine) {
    if e.global::<u32>(INSTANCE) != 0 {
        let object = e.global::<u32>(INSTANCE);
        if object != 0 {
            fn_00af2400(e, object);
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, object]);
        }
        e.set_global::<u32>(INSTANCE, 0);
    }
}

// Translated from 00af23d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The constructor (no label; `thiscall`, plain `RET`): the list's first node (item +0, next
/// +4) cleared. Answers `this`.
pub fn fn_00af23d0(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, 0);
    e.mem.set_u32(this.wrapping_add(4), 0);
    this
}

// Translated from 00af2400 (disassembled, FalloutNV.exe 1.4.0.525)
/// The destructor body (no label; `thiscall`, plain `RET`): the list cleared (`00ae83e0` on
/// `this`).
pub fn fn_00af2400(e: &mut Engine, this: u32) {
    e.call(LIST_CLEAR, &args![this]);
}

// Translated from 00af2420 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSAwardsSystemUtility::Unlock` (engine map; `thiscall`, one word: the award, `RET 4`): a
/// 4-byte block is allocated (`00aa3e40`) and given the award (a failed allocation gives
/// null); the block's address goes through a local word to the list's tail add (`00af25b0`).
/// Answers 1 (`AL`).
pub fn bs_awards_system_utility_unlock(e: &mut Engine, this: u32, award: u32) -> u8 {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let item = if block != 0 {
        e.mem.set_u32(block, award);
        block
    } else {
        0
    };
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), item);
        e.call(LIST_ADD_TAIL, &args![this, local.addr()]);
    });
    1
}

// Translated from 00af2470 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, plain `RET`): answers 0 (`AL`).
pub fn fn_00af2470(_e: &mut Engine, _this: u32) -> u8 {
    0
}

/// The `steam_api.dll` import slot `SteamUserStats` (`00fdf490`, no arguments), the
/// out-of-range message (`005b5e40`, `cdecl`: format, value), the bounded print (`00406d00`,
/// `cdecl`: buffer, size, format, value), the strings `010a4244` ("Achievement %d is out of
/// range and will not be awarded!") and `010a427c` ("A%02d"), and the `ISteamUserStats` slots
/// the exe calls (`thiscall`): +0x1C (one word, the name) and +0x28 (no words).
const STEAM_USER_STATS: u32 = 0x00fd_f490;
const OUT_OF_RANGE_MESSAGE: u32 = 0x005b_5e40;
const PRINT_BOUNDED: u32 = 0x0040_6d00;
const OUT_OF_RANGE_TEXT: u32 = 0x010a_4244;
const AWARD_NAME_FORMAT: u32 = 0x010a_427c;
const STATS_SET_ACHIEVEMENT: u32 = 0x1c;
const STATS_STORE: u32 = 0x28;

// Translated from 00af2480 (disassembled, FalloutNV.exe 1.4.0.525)
/// `BSAwardsSystemUtility::Idle` (engine map; `thiscall`, plain `RET`). With an empty list
/// (first node's item and next both 0) nothing happens. Otherwise the nodes are walked from the
/// first one until a null node or an empty one (item and next 0): the item block's award is read
/// and the block freed (`00aa4060`); an award above 100 (signed) gives the out-of-range message
/// (`005b5e40` with the award) and the walk goes on at the same node, not the next one (as the
/// exe jumps back); otherwise, when `SteamUserStats()` is not null, the award is printed as
/// "A%02d" into a 4-byte buffer (`00406d00`, size 4), its last byte zeroed, and passed to a
/// second `SteamUserStats()`'s slot +0x1C; the walk moves to the next node. After the walk, when
/// `SteamUserStats()` is not null, a second one's slot +0x28 is called; then the list is cleared
/// (`00ae83e0`).
pub fn bs_awards_system_utility_idle(e: &mut Engine, this: u32) {
    let empty =
        |e: &Engine, node: u32| e.mem.u32(node.wrapping_add(4)) == 0 && e.mem.u32(node) == 0;
    if empty(e, this) {
        return;
    }
    let mut node = this;
    while node != 0 && !empty(e, node) {
        let item = e.mem.u32(node);
        let award = e.mem.u32(item);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, item]);
        if (award as i32) > 100 {
            e.call(OUT_OF_RANGE_MESSAGE, &args![OUT_OF_RANGE_TEXT, award]);
            continue;
        }
        if e.call(STEAM_USER_STATS, &args![]).u32() != 0 {
            e.with_stack(4, |e, name| {
                e.call(
                    PRINT_BOUNDED,
                    &args![name.addr(), 4u32, AWARD_NAME_FORMAT, award],
                );
                e.mem.set_u8(name.addr() + 3, 0);
                let stats = e.call(STEAM_USER_STATS, &args![]).u32();
                e.vcall(stats, STATS_SET_ACHIEVEMENT, &args![name.addr()]);
            });
        }
        node = e.mem.u32(node.wrapping_add(4));
    }
    if e.call(STEAM_USER_STATS, &args![]).u32() != 0 {
        let stats = e.call(STEAM_USER_STATS, &args![]).u32();
        e.vcall(stats, STATS_STORE, &args![]);
    }
    e.call(LIST_CLEAR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_2480, bs_awards_system_utility_idle(u32)),
        entry!(0x00af_22d0, bs_awards_system_utility_q_instance() -> u32),
        entry!(0x00af_2360, bs_awards_system_utility_initialize() -> u32),
        entry!(0x00af_2370, fn_00af2370()),
        entry!(0x00af_23d0, fn_00af23d0(u32) -> u32),
        entry!(0x00af_2400, fn_00af2400(u32)),
        entry!(0x00af_2420, bs_awards_system_utility_unlock(u32, u32) -> u8),
        entry!(0x00af_2470, fn_00af2470(u32) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(INSTANCE, 4);
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_instance_is_made_once_and_released() {
        let mut e = engine();
        let made = e.call(0x00af_2360, &args![]).u32();
        assert_ne!(made, 0);
        assert_eq!((e.mem.u32(made), e.mem.u32(made + 4)), (0, 0));
        assert_eq!(e.call(0x00af_22d0, &args![]).u32(), made);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 8]]);
        e.call(0x00af_2370, &args![]);
        assert_eq!(e.global::<u32>(INSTANCE), 0);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![made]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, made]]);
        // Released again: nothing to do.
        e.call(0x00af_2370, &args![]);
        assert_eq!(calls(&e, DEALLOCATE).len(), 1);
    }

    #[test]
    fn a_failed_allocation_keeps_the_instance_null() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(bs_awards_system_utility_q_instance(&mut e), 0);
        assert_eq!(bs_awards_system_utility_q_instance(&mut e), 0);
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
    }

    #[test]
    fn unlock_adds_a_block_holding_the_award() {
        let mut e = engine();
        let list = e.mem.alloc(8);
        e.register_double(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        assert_eq!(e.call(0x00af_2420, &args![list, 0x37u32]).u8(), 1);
        let block = e.mem.u32(list);
        assert_eq!(e.mem.u32(block), 0x37);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 4]]);
        // Without memory the item word is null.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        bs_awards_system_utility_unlock(&mut e, list, 0x38);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.call(0x00af_2470, &args![list]).u8(), 0);
    }

    /// A list of two nodes (the first inside the object) holding blocks with `awards`.
    fn award_list(e: &mut Engine, awards: [u32; 2]) -> (u32, [u32; 2]) {
        let items = awards.map(|a| {
            let block = e.mem.alloc(4);
            e.mem.set_u32(block, a);
            block
        });
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, items[1]);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, second);
        (list, items)
    }

    #[test]
    fn idle_without_steam_frees_the_items_and_clears_the_list() {
        let mut e = engine();
        e.register(STEAM_USER_STATS, |_, _| 0u32.into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let (list, items) = award_list(&mut e, [3, 100]);
        e.call(0x00af_2480, &args![list]);
        assert_eq!(
            calls(&e, DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, items[0]],
                vec![MEMORY_MANAGER, items[1]]
            ]
        );
        assert_eq!(calls(&e, STEAM_USER_STATS).len(), 3);
        assert!(calls(&e, PRINT_BOUNDED).is_empty());
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![list]]);
        // An empty list: nothing at all, not even the clear.
        let empty = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00af_2480, &args![empty]);
        assert!(e.call_log.as_ref().unwrap().len() <= 1);
        assert!(calls(&e, LIST_CLEAR).is_empty());
    }

    #[test]
    fn idle_with_steam_sets_each_award_by_name_and_stores() {
        let mut e = engine();
        const SET: u32 = 0x0e00_0010;
        const STORE: u32 = 0x0e00_0020;
        let stats = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stats, vtable);
        e.mem.set_u32(vtable + STATS_SET_ACHIEVEMENT, SET);
        e.mem.set_u32(vtable + STATS_STORE, STORE);
        e.register_double(STEAM_USER_STATS, move |_, _| stats.into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register_double(PRINT_BOUNDED, |e, a| {
            assert_eq!((a[1], a[2]), (4, AWARD_NAME_FORMAT));
            let text = format!("A{:02}", a[3]);
            e.mem.write(a[0], &text.as_bytes()[..4.min(text.len())]);
            Ret::default()
        });
        let names = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = names.clone();
        e.register_double(SET, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        e.register(STORE, |_, _| Ret::default());
        let (list, _) = award_list(&mut e, [7, 42]);
        e.call(0x00af_2480, &args![list]);
        assert_eq!(
            *names.borrow(),
            vec![(stats, b"A07".to_vec()), (stats, b"A42".to_vec())]
        );
        assert_eq!(calls(&e, STORE), vec![vec![stats]]);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![list]]);
    }

    #[test]
    fn an_award_above_100_gives_the_message_and_stays_on_the_node() {
        let mut e = engine();
        e.register(STEAM_USER_STATS, |_, _| 0u32.into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let (list, items) = award_list(&mut e, [101, 5]);
        // The walk stays on the first node: the message double empties it to end the walk.
        e.register_double(OUT_OF_RANGE_MESSAGE, move |e, _| {
            e.mem.set_u32(list, 0);
            e.mem.set_u32(list + 4, 0);
            Ret::default()
        });
        e.call(0x00af_2480, &args![list]);
        assert_eq!(
            calls(&e, OUT_OF_RANGE_MESSAGE),
            vec![vec![OUT_OF_RANGE_TEXT, 101]]
        );
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, items[0]]]);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![list]]);
    }
}
