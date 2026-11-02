//! `(unplaced)/run_0044d000` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0044dee0` (`thiscall`, no stack words): the constructor of the iterator's two members.
const MEMBER_CONSTRUCT: u32 = 0x0044_dee0;
/// The scope `00404eb0` builds on a stack local (`thiscall`, four words) and `00404ee0`
/// ends (`thiscall`); unit `fallout shared/magic/effectitem.cpp` in the engine map.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
/// The allocation and free used here (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `0044dec0` (`thiscall` on `this` +0x14, no stack words): answers the current entry.
const CURRENT_ENTRY: u32 = 0x0044_dec0;
/// `006ecba0` (`thiscall`, two words; unit `fallout shared/pathfinding/pathmanager.cpp`): the
/// constructor of the two 4-byte stack locals, (0, pointer).
const HANDLE_CONSTRUCT: u32 = 0x006e_cba0;
/// `00559450` (`thiscall`, no stack words): answers the word a local holds.
const HANDLE_GET: u32 = 0x0055_9450;
/// `004491c0` (`cdecl`, three words; unit `fallout shared/modelloader.cpp`): the
/// compare-and-swap of the new entry into the bucket (by its use here), answering success.
const SWAP_IN: u32 = 0x0044_91c0;
/// `00529000` (`thiscall` on `this`, no stack words): run after the loop.
const FINISH: u32 = 0x0052_9000;
/// The first variant's lookup (`006662a0`) and entry constructor (`006ecbd0`), the second's
/// (`00529030`, `0044df40`): `thiscall`, two words each.
const FIND_FIRST: u32 = 0x0066_62a0;
const ENTRY_CONSTRUCT_FIRST: u32 = 0x006e_cbd0;
const FIND_SECOND: u32 = 0x0052_9030;
const ENTRY_CONSTRUCT_SECOND: u32 = 0x0044_df40;
/// The second variant's entry destructor (`0044ed50`, `thiscall`, the flags 1) and value
/// assignment (`0092c820`, `thiscall`, the address of the value).
const ENTRY_DELETE_SECOND: u32 = 0x0044_ed50;
const VALUE_ASSIGN_SECOND: u32 = 0x0092_c820;

// Translated from 0044d000 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words, `RET 0x10`; the 0x24-byte iterator
/// built by `00906400`): runs `0044dee0` on +0x14 and +0x18, stores the four words at +0x0
/// to +0xC and clears +0x10, +0x1C and +0x20. Answers `this`.
pub fn fn_0044d000(
    e: &mut Engine,
    this: u32,
    map: u32,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    e.call(MEMBER_CONSTRUCT, &args![this + 0x14]);
    e.call(MEMBER_CONSTRUCT, &args![this + 0x18]);
    e.mem.set_u32(this, map);
    e.mem.set_u32(this + 0x4, first);
    e.mem.set_u32(this + 0x8, second);
    e.mem.set_u32(this + 0xc, third);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x1c, 0);
    e.mem.set_u32(this + 0x20, 0);
    this
}

/// What differs between `fn_0044d240` and `fn_0044d070`.
struct Variant {
    find: u32,
    entry_construct: u32,
    /// The second variant destroys its unused entry and assigns through `0092c820`; the first
    /// frees it and stores the value word.
    second: bool,
}

/// The shared body of `fn_0044d240` and `fn_0044d070` (both lock-free-map inserts).
fn insert(
    e: &mut Engine,
    this: u32,
    bucket: u32,
    key: u32,
    value: u32,
    replace: u32,
    variant: &Variant,
) -> bool {
    e.with_stack(0x10, |e, frame| {
        let scope = frame.addr();
        let entry_handle = scope + 0x8;
        let node_handle = scope + 0xc;
        e.call(
            SCOPE_BEGIN,
            &args![scope, 6u32, 1u32, 0x0101_73f8u32, 0x187u32],
        );
        let mut result = true;
        let mut node = 0u32;
        // The loop's `MOV EAX,1; TEST EAX,EAX` condition is always true.
        loop {
            if e.call(variant.find, &args![this, bucket, key]).bool() {
                if variant.second {
                    if node != 0 {
                        e.call(ENTRY_DELETE_SECOND, &args![node, 1u32]);
                    }
                } else {
                    e.call(FREE, &args![node]);
                }
                if replace as u8 != 0 {
                    let entry = e.call(CURRENT_ENTRY, &args![this + 0x14]).u32();
                    if variant.second {
                        e.call(VALUE_ASSIGN_SECOND, &args![entry + 4, value]);
                    } else {
                        let word = e.mem.u32(value);
                        e.mem.set_u32(entry + 4, word);
                    }
                } else {
                    result = false;
                }
                break;
            }
            if node == 0 {
                let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                node = if block != 0 {
                    let owner = e.mem.u32(this);
                    let made_key = e.vcall(owner, 0x2c, &args![key]).u32();
                    e.call(variant.entry_construct, &args![block, made_key, value])
                        .u32()
                } else {
                    0
                };
            }
            let current = e.call(CURRENT_ENTRY, &args![this + 0x14]).u32();
            e.call(HANDLE_CONSTRUCT, &args![entry_handle, 0u32, current]);
            e.call(HANDLE_CONSTRUCT, &args![node_handle, 0u32, node]);
            let held = e.mem.u32(entry_handle);
            e.mem.set_u32(node + 8, held);
            let expected = e.call(HANDLE_GET, &args![entry_handle]).u32();
            let replacement = e.call(HANDLE_GET, &args![node_handle]).u32();
            let slot = e.mem.u32(this + 0x10);
            if e.call(SWAP_IN, &args![slot, replacement, expected]).bool() {
                let owner = e.mem.u32(this);
                e.vcall(owner, 0x3c, &[]);
                break;
            }
        }
        e.call(FINISH, &args![this]);
        e.call(SCOPE_END, &args![scope]);
        result
    })
}

// Translated from 0044d240 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words, `RET 0x10`; a lock-free map insert,
/// inside the scope `00404eb0(6, 1, 010173f8, 0x187)`): loops until done. When the key is
/// found (`006662a0(bucket, key)`), the unused new entry is freed (`00401030`, also when
/// null) and, with `replace`, the word at `value` is stored into the current entry's +4
/// (else the answer becomes false). Otherwise a new 12-byte entry is made once (`00401000`,
/// `006ecbd0(<slot +0x2C of the map's owner for the key>, value)`; the owner's slot takes
/// one word, the pushed `value` belongs to `006ecbd0`), linked to the current entry (+8)
/// through two handle locals (`006ecba0`) and swapped in (`004491c0(<word +0x10>, new,
/// current)`); a successful swap runs the owner's slot +0x3C and ends the loop. Then
/// `00529000`. Answers the flag (true unless an existing key was kept).
// Translation incomplete: 0044d240 - the FS:[0] exception frame (ending the scope on unwinding) is not translated
pub fn fn_0044d240(
    e: &mut Engine,
    this: u32,
    bucket: u32,
    key: u32,
    value: u32,
    replace: u32,
) -> bool {
    let variant = Variant {
        find: FIND_FIRST,
        entry_construct: ENTRY_CONSTRUCT_FIRST,
        second: false,
    };
    insert(e, this, bucket, key, value, replace, &variant)
}

// Translated from 0044d070 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words, `RET 0x10`): as `fn_0044d240` with the
/// lookup `00529030` and the entry constructor `0044df40`; a found key destroys the unused
/// entry (`0044ed50(1)`, only when it is not null) and, with `replace`, assigns the value
/// through `0092c820` on the current entry's +4.
// Translation incomplete: 0044d070 - the FS:[0] exception frame (ending the scope on unwinding) is not translated
pub fn fn_0044d070(
    e: &mut Engine,
    this: u32,
    bucket: u32,
    key: u32,
    value: u32,
    replace: u32,
) -> bool {
    let variant = Variant {
        find: FIND_SECOND,
        entry_construct: ENTRY_CONSTRUCT_SECOND,
        second: true,
    };
    insert(e, this, bucket, key, value, replace, &variant)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_d000, fn_0044d000(u32, u32, u32, u32, u32) -> u32),
        entry!(0x0044_d240, fn_0044d240(u32, u32, u32, u32, u32) -> bool),
        entry!(0x0044_d070, fn_0044d070(u32, u32, u32, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0044d000_builds_the_iterator() {
        let mut e = Engine::new();
        e.register(MEMBER_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        for offset in (0..0x24).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_d000, &[this, 1, 2, 3, 4]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMBER_CONSTRUCT),
            vec![vec![this + 0x14], vec![this + 0x18]]
        );
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(this + i * 4)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 0, 0xffff_ffff, 0xffff_ffff, 0, 0]);
    }

    /// A map whose owner (at +0) has slots +0x2C (answers the key + 0x100) and +0x3C, and the
    /// swap target word at +0x10. The handle locals keep their pointer word.
    fn map(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x20);
        let owner = e.mem.alloc(8);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x2c, 0x0090_1000);
        e.mem.set_u32(table + 0x3c, 0x0090_2000);
        e.mem.set_u32(owner, table);
        e.mem.set_u32(this, owner);
        e.mem.set_u32(this + 0x10, 0x3330);
        e.register(0x0090_1000, |_, a| (a[1] + 0x100).into_ret());
        e.register(0x0090_2000, |_, _| Ret::default());
        e.register(SCOPE_BEGIN, |_, _| Ret::default());
        e.register(SCOPE_END, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(FINISH, |_, _| Ret::default());
        e.register(HANDLE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            Ret::default()
        });
        e.register(HANDLE_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_DELETE_SECOND, |_, _| Ret::default());
        e.register(VALUE_ASSIGN_SECOND, |_, _| Ret::default());
        this
    }

    #[test]
    fn fn_0044d240_inserts_after_a_failed_swap() {
        let mut e = Engine::new();
        let this = map(&mut e);
        let node = e.mem.alloc(0xc);
        e.register_double(ALLOCATE, move |_, _| node.into_ret());
        e.register(FIND_FIRST, |_, _| Ret::default());
        e.register(ENTRY_CONSTRUCT_FIRST, |_, a| a[0].into_ret());
        e.register(CURRENT_ENTRY, |_, _| 0x7770u32.into_ret());
        // The first swap fails, the second succeeds.
        let mut tries = 0;
        e.register_double(SWAP_IN, move |_, _| {
            tries += 1;
            u32::from(tries == 2).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_d240, &[this, 5, 6, 0x8880, 0]).bool());
        // One allocation for both tries.
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(
            calls_to(&e, ENTRY_CONSTRUCT_FIRST),
            vec![vec![node, 0x106, 0x8880]]
        );
        assert_eq!(calls_to(&e, FIND_FIRST), vec![vec![this, 5, 6]; 2]);
        assert_eq!(calls_to(&e, SWAP_IN), vec![vec![0x3330, node, 0x7770]; 2]);
        assert_eq!(e.mem.u32(node + 8), 0x7770);
        assert_eq!(calls_to(&e, 0x0090_2000).len(), 1);
        assert_eq!(calls_to(&e, FINISH), vec![vec![this]]);
        assert_eq!(calls_to(&e, SCOPE_END).len(), 1);
    }

    #[test]
    fn fn_0044d240_keeps_or_replaces_an_existing_key() {
        let mut e = Engine::new();
        let this = map(&mut e);
        e.register(FIND_FIRST, |_, _| 1u32.into_ret());
        let entry = e.mem.alloc(8);
        e.register_double(CURRENT_ENTRY, move |_, _| entry.into_ret());
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x4242);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_d240, &[this, 5, 6, value, 0]).bool());
        assert_eq!(calls_to(&e, FREE), vec![vec![0]]);
        assert_eq!(e.mem.u32(entry + 4), 0);
        assert!(e.call(0x0044_d240, &[this, 5, 6, value, 1]).bool());
        assert_eq!(e.mem.u32(entry + 4), 0x4242);
    }

    #[test]
    fn fn_0044d070_uses_its_own_entry_calls() {
        let mut e = Engine::new();
        let this = map(&mut e);
        let node = e.mem.alloc(0xc);
        e.register_double(ALLOCATE, move |_, _| node.into_ret());
        // Not found first, then found after the failed swap: the made entry is destroyed.
        let mut lookups = 0;
        e.register_double(FIND_SECOND, move |_, _| {
            lookups += 1;
            u32::from(lookups == 2).into_ret()
        });
        e.register(ENTRY_CONSTRUCT_SECOND, |_, a| a[0].into_ret());
        e.register(CURRENT_ENTRY, |_, _| 0x7770u32.into_ret());
        e.register(SWAP_IN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_d070, &[this, 5, 6, 0x8880, 1]).bool());
        assert_eq!(
            calls_to(&e, ENTRY_CONSTRUCT_SECOND),
            vec![vec![node, 0x106, 0x8880]]
        );
        assert_eq!(calls_to(&e, ENTRY_DELETE_SECOND), vec![vec![node, 1]]);
        assert_eq!(
            calls_to(&e, VALUE_ASSIGN_SECOND),
            vec![vec![0x7774, 0x8880]]
        );
        assert!(calls_to(&e, FREE).is_empty());
        // Found at once without replacing: no entry to destroy, the answer is false.
        e.register(FIND_SECOND, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_d070, &[this, 5, 6, 0x8880, 0]).bool());
        assert!(calls_to(&e, ENTRY_DELETE_SECOND).is_empty());
        assert!(calls_to(&e, VALUE_ASSIGN_SECOND).is_empty());
    }
}
