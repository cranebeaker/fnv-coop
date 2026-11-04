//! `(unplaced)/run_0044edd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a float value (`0040e130`).
const SETTING_CONSTRUCT_FLOAT: u32 = 0x0040_e130;
/// The INI setting vtable.
const INI_SETTING_VTABLE: u32 = 0x0101_7720;
/// The INI setting collection (`0044f560`: created on first use).
const INI_SETTINGS: u32 = 0x0044_f560;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0044f440 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a float INI setting (`thiscall`, `RET 8`): the base
/// constructor (`0040e130`) with the name and the value (passed on through
/// the x87 stack), the INI setting vtable (`01017720`), then adds itself to
/// the INI setting collection (`0044f560`, vtable slot +4). Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn fn_0044f440(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(
        SETTING_CONSTRUCT_FLOAT,
        &args![this, name, x87_quiet_bits(value)],
    );
    e.mem.set_u32(this.addr(), INI_SETTING_VTABLE);
    let collection = e.call(INI_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0044f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044f400` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044f3b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_f400, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0044ed50` (`thiscall`, one word: the flags, 1 here): the entries' deleting destructor.
const ENTRY_DELETE: u32 = 0x0044_ed50;
/// `00559450` (`thiscall`, no stack words): answers the word the object holds.
const HOLDER_GET: u32 = 0x0055_9450;
/// The allocation and free used here (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `0044e360` (`cdecl`, one word): run on the word at +4 by `fn_0044ee40`.
const RELEASE_0044E360: u32 = 0x0044_e360;
/// `00401460` (`cdecl`, (destination, source, size)): the copy of the bucket heads.
const COPY: u32 = 0x0040_1460;
/// The scope `00404eb0` builds on a stack local (`thiscall`, four words) and `00404ee0`
/// ends (`thiscall`); unit `fallout shared/magic/effectitem.cpp` in the engine map.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
/// `00726070` (`thiscall`, no stack words): answers the next entry of a retired list.
const ENTRY_NEXT: u32 = 0x0072_6070;
/// `006ecd40` (`thiscall`, one word): sets an entry's link (the engine map's name for it,
/// `CombatProcedureAttackMelee::Initialize`, is folded code).
const ENTRY_SET_NEXT: u32 = 0x006e_cd40;
/// `0044dde0` (`thiscall`, one word; another unit): `fn_0044f220`'s release of an entry.
const RELEASE_0044DDE0: u32 = 0x0044_dde0;

// Translated from 0044edd0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): calls vtable slot +0x28 with
/// the entry's first word, then deletes the entry (`0044ed50(1)`) when it is not null (the
/// first read is made on it regardless, as in the exe).
pub fn fn_0044edd0(e: &mut Engine, this: u32, entry: u32) {
    let first = e.mem.u32(entry);
    e.vcall(this, 0x28, &args![first]);
    if entry != 0 {
        e.call(ENTRY_DELETE, &args![entry, 1u32]);
    }
}

// Translated from 0044ee20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): the address of word `index` of
/// the array at +0xC.
pub fn fn_0044ee20(e: &mut Engine, this: u32, index: u32) -> u32 {
    e.mem.u32(this + 0xc).wrapping_add(index.wrapping_mul(4))
}

// Translated from 0044ee40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a destructor body): for each of
/// the `00559450` (`this`) pairs of the array at +0x8, deletes the object in the pair's
/// second word (`fn_0044f3b0(1)`) when it is not null; then frees the array (`00401030`)
/// and passes the word at +0x4 to `0044e360`.
pub fn fn_0044ee40(e: &mut Engine, this: u32) {
    let mut index = 0u32;
    while index < e.call(HOLDER_GET, &args![this]).u32() {
        let pairs = e.mem.u32(this + 8);
        let object = e
            .mem
            .u32(pairs.wrapping_add(index.wrapping_mul(8)).wrapping_add(4));
        if object != 0 {
            fn_0044f3b0(e, Ptr::new(object), 1);
        }
        index += 1;
    }
    let pairs = e.mem.u32(this + 8);
    e.call(FREE, &args![pairs]);
    let other = e.mem.u32(this + 4);
    e.call(RELEASE_0044E360, &args![other]);
}

// Translated from 0044f3e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `00559450` of the object at
/// +0x14 (the map's bucket count, by its use in the rebuilds).
pub fn fn_0044f3e0(e: &mut Engine, this: u32) -> u32 {
    let holder = e.mem.u32(this + 0x14);
    e.call(HOLDER_GET, &args![holder]).u32()
}

/// How a rebuild drops an entry that is no longer referenced from a bucket.
enum Drop {
    /// `0044ed50(1)` when the entry is not null (`fn_0044eed0`).
    Delete,
    /// `fn_0044edd0` on the map (`fn_0044f070`).
    Release,
    /// `0044dde0` on the map (`fn_0044f220`).
    Other,
}

/// The shape shared by `fn_0044eed0`, `fn_0044f070` and `fn_0044f220`: inside a scope
/// (`00404eb0(6, 1, 010173f8, line)`), copies two or three words per bucket (`fn_0044f3e0` of the map) of the map's
/// array at `source` (`00401000`, `00401460`), then takes every entry off the retired list
/// at `list` (`00726070` gives the next): an entry still in the copy is linked back into a
/// new list (`006ecd40`) and counted, the others are unlinked (`006ecd40(0)`) and dropped.
/// The copy is freed and the new list and its count stored at `list` and `counter`.
fn rebuild(
    e: &mut Engine,
    this: u32,
    line: u32,
    triple: bool,
    source: u32,
    list: u32,
    counter: u32,
    drop: Drop,
) {
    e.with_stack(4, |e, scope| {
        let scope = scope.addr();
        e.call(SCOPE_BEGIN, &args![scope, 6u32, 1u32, 0x0101_73f8u32, line]);
        // Two words per bucket (`SHL 1`) or three (`IMUL 3`).
        let map = e.mem.u32(this);
        let buckets = fn_0044f3e0(e, map);
        let words = if triple {
            buckets.wrapping_mul(3)
        } else {
            buckets << 1
        };
        let size = words.checked_mul(4).unwrap_or(u32::MAX);
        let copy = e.call(ALLOCATE, &args![size]).u32();
        let map = e.mem.u32(this);
        let from = e.mem.u32(map + source);
        e.call(COPY, &args![copy, from, words << 2]);
        let mut kept_list = 0u32;
        let mut kept = 0u32;
        while e.mem.u32(this + list) != 0 {
            let entry = e.mem.u32(this + list);
            let next = e.call(ENTRY_NEXT, &args![entry]).u32();
            e.mem.set_u32(this + list, next);
            let mut found = false;
            let mut j = 0u32;
            while j < words {
                if entry == e.mem.u32(copy.wrapping_add(j.wrapping_mul(4))) {
                    found = true;
                    break;
                }
                j += 1;
            }
            if found {
                e.call(ENTRY_SET_NEXT, &args![entry, kept_list]);
                kept_list = entry;
                kept = kept.wrapping_add(1);
                continue;
            }
            e.call(ENTRY_SET_NEXT, &args![entry, 0u32]);
            match drop {
                Drop::Delete => {
                    if entry != 0 {
                        e.call(ENTRY_DELETE, &args![entry, 1u32]);
                    }
                }
                Drop::Release => {
                    let map = e.mem.u32(this);
                    fn_0044edd0(e, map, entry);
                }
                Drop::Other => {
                    let map = e.mem.u32(this);
                    e.call(RELEASE_0044DDE0, &args![map, entry]);
                }
            }
        }
        e.call(FREE, &args![copy]);
        e.mem.set_u32(this + list, kept_list);
        e.mem.set_u32(this + counter, kept);
        e.call(SCOPE_END, &args![scope]);
    });
}

// Translated from 0044eed0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the rebuild (see `rebuild`) of the
/// retired list at +0x10 (count +0xC) against the map's array at +0x10, two words per bucket
/// (`fn_0044f3e0 << 1`), scope line 0xD0; dropped entries are deleted (`0044ed50(1)`).
// Translation incomplete: 0044eed0 - the FS:[0] exception frame (ending the scope on unwinding) is not translated
pub fn fn_0044eed0(e: &mut Engine, this: u32) {
    rebuild(e, this, 0xd0, false, 0x10, 0x10, 0xc, Drop::Delete);
}

// Translated from 0044f070 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the rebuild of the retired list
/// at +0x1C (count +0x20) against the map's array at +0x4, three words per bucket, scope
/// line 0x362; dropped entries go through `fn_0044edd0` on the map.
// Translation incomplete: 0044f070 - the FS:[0] exception frame (ending the scope on unwinding) is not translated
pub fn fn_0044f070(e: &mut Engine, this: u32) {
    rebuild(e, this, 0x362, true, 0x4, 0x1c, 0x20, Drop::Release);
}

// Translated from 0044f220 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): as `fn_0044f070`, the dropped
/// entries going through `0044dde0` on the map instead.
// Translation incomplete: 0044f220 - the FS:[0] exception frame (ending the scope on unwinding) is not translated
pub fn fn_0044f220(e: &mut Engine, this: u32) {
    rebuild(e, this, 0x362, true, 0x4, 0x1c, 0x20, Drop::Other);
}

// Translated from 0044f200 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs `fn_0044f220` while the
/// count at +0x20 is not zero.
pub fn fn_0044f200(e: &mut Engine, this: u32) {
    while e.mem.u32(this + 0x20) != 0 {
        fn_0044f220(e, this);
    }
}

// Translated from 0044f400 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the body `fn_0044f3b0` runs): runs
/// `fn_0044eed0` while the count at +0xC is not zero.
pub fn fn_0044f400(e: &mut Engine, this: u32) {
    while e.mem.u32(this + 0xc) != 0 {
        fn_0044eed0(e, this);
    }
}

// Translated from 0044f420 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs `fn_0044f070` while the
/// count at +0x20 is not zero.
pub fn fn_0044f420(e: &mut Engine, this: u32) {
    while e.mem.u32(this + 0x20) != 0 {
        fn_0044f070(e, this);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_edd0, fn_0044edd0(u32, u32)),
        entry!(0x0044_ee20, fn_0044ee20(u32, u32) -> u32),
        entry!(0x0044_ee40, fn_0044ee40(u32)),
        entry!(0x0044_f3e0, fn_0044f3e0(u32) -> u32),
        entry!(0x0044_eed0, fn_0044eed0(u32)),
        entry!(0x0044_f070, fn_0044f070(u32)),
        entry!(0x0044_f220, fn_0044f220(u32)),
        entry!(0x0044_f200, fn_0044f200(u32)),
        entry!(0x0044_f400, fn_0044f400(u32)),
        entry!(0x0044_f420, fn_0044f420(u32)),
        entry!(0x0044f440, fn_0044f440(Ptr, u32, u32) -> Ptr),
        entry!(0x0044_f3b0, fn_0044f3b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_float_ini_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(INI_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), INI_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044f440, &[s, 0x77, 0x7f80_0002]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (SETTING_CONSTRUCT_FLOAT, vec![s, 0x77, 0x7fc0_0002])
        );
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
    }
    #[test]
    fn at_0044f3b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_f400, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_f3b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_f3b0)
                .collect();
            let mut expected = vec![(0x0044_f400, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b018_tests {
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
    fn fn_0044edd0_calls_the_slot_then_deletes() {
        let mut e = Engine::new();
        e.put_vtable(0x0090_0000, &[0; 11]);
        e.mem.set_u32(0x0090_0028, 0x0090_1000);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0090_0000);
        e.register(0x0090_1000, |_, _| Ret::default());
        e.register(ENTRY_DELETE, |_, _| Ret::default());
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0044_edd0, &[this, entry]);
        assert_eq!(calls_to(&e, 0x0090_1000), vec![vec![this, 0x55]]);
        assert_eq!(calls_to(&e, ENTRY_DELETE), vec![vec![entry, 1]]);
    }

    #[test]
    fn fn_0044ee20_and_fn_0044f3e0_answer_their_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x1000);
        assert_eq!(e.call(0x0044_ee20, &[this, 3]).u32(), 0x100c);
        e.register(HOLDER_GET, |_, a| (a[0] + 1).into_ret());
        e.mem.set_u32(this + 0x14, 0x2000);
        assert_eq!(e.call(0x0044_f3e0, &[this]).u32(), 0x2001);
    }

    #[test]
    fn fn_0044ee40_deletes_the_objects_and_frees() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        let pairs = e.mem.alloc(0x18);
        e.mem.set_u32(pairs + 4, 0x7000);
        e.mem.set_u32(pairs + 0x14, 0x7100);
        e.mem.set_u32(this + 8, pairs);
        e.mem.set_u32(this + 4, 0x6666);
        e.register(HOLDER_GET, |_, _| 3u32.into_ret());
        e.register(0x0044_f400, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(RELEASE_0044E360, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_ee40, &[this]);
        assert_eq!(calls_to(&e, 0x0044_f400), vec![vec![0x7000], vec![0x7100]]);
        assert_eq!(
            calls_to(&e, FREE),
            vec![vec![0x7000], vec![0x7100], vec![pairs]]
        );
        assert_eq!(calls_to(&e, RELEASE_0044E360), vec![vec![0x6666]]);
    }

    /// A rebuild setup: the map (at `this`) has `buckets` buckets and its array at +0x4 and
    /// +0x10 holds `heads`; entries link through their word +0 (the doubles of `00726070`
    /// and `006ecd40`).
    fn setup(e: &mut Engine, heads: &[u32], buckets: u32) -> (u32, u32) {
        let map = e.mem.alloc(0x18);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, buckets);
        e.mem.set_u32(map + 0x14, holder);
        let array = e.mem.alloc(0x40);
        for (i, head) in (0u32..).zip(heads) {
            e.mem.set_u32(array + i * 4, *head);
        }
        e.mem.set_u32(map + 4, array);
        e.mem.set_u32(map + 0x10, array);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this, map);
        e.register(HOLDER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SCOPE_BEGIN, |_, _| Ret::default());
        e.register(SCOPE_END, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(COPY, |e, a| {
            for i in (0..a[2]).step_by(4) {
                let word = e.mem.u32(a[1] + i);
                e.mem.set_u32(a[0] + i, word);
            }
            Ret::default()
        });
        e.register(ENTRY_NEXT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(ENTRY_DELETE, |_, _| Ret::default());
        e.register(RELEASE_0044DDE0, |_, _| Ret::default());
        (this, map)
    }

    /// Three entries linked a -> b -> c.
    fn entries(e: &mut Engine) -> [u32; 3] {
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        let c = e.mem.alloc(8);
        e.mem.set_u32(a, b);
        e.mem.set_u32(b, c);
        [a, b, c]
    }

    #[test]
    fn fn_0044eed0_keeps_the_referenced_entries() {
        let mut e = Engine::new();
        let [a, b, c] = entries(&mut e);
        // One bucket, two words: b is referenced.
        let (this, _) = setup(&mut e, &[b, 0], 1);
        e.mem.set_u32(this + 0x10, a);
        e.mem.set_u32(this + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x0044_eed0, &[this]);
        assert_eq!(e.mem.u32(this + 0x10), b);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(calls_to(&e, ENTRY_DELETE), vec![vec![a, 1], vec![c, 1]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls_to(&e, SCOPE_BEGIN)[0][4], 0xd0);
        // fn_0044f400 runs it until the count is 0: c referenced, then nothing.
        let (this, _) = setup(&mut e, &[0, 0], 1);
        e.mem.set_u32(a, b);
        e.mem.set_u32(b, 0);
        e.mem.set_u32(this + 0x10, a);
        e.mem.set_u32(this + 0xc, 2);
        e.call(0x0044_f400, &[this]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn fn_0044f070_and_fn_0044f220_release_through_the_map() {
        let mut e = Engine::new();
        let [a, b, c] = entries(&mut e);
        // One bucket, three words: c is referenced.
        let (this, map) = setup(&mut e, &[0, c, 0], 1);
        e.put_vtable(0x0090_0000, &[0; 11]);
        e.mem.set_u32(0x0090_0028, 0x0090_1000);
        e.mem.set_u32(map, 0x0090_0000);
        e.register(0x0090_1000, |_, _| Ret::default());
        e.mem.set_u32(this + 0x1c, a);
        e.mem.set_u32(this + 0x20, 3);
        e.call_log = Some(vec![]);
        e.call(0x0044_f070, &[this]);
        assert_eq!(e.mem.u32(this + 0x1c), c);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        assert_eq!(calls_to(&e, ENTRY_DELETE), vec![vec![a, 1], vec![b, 1]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![12]]);
        assert_eq!(calls_to(&e, SCOPE_BEGIN)[0][4], 0x362);

        e.mem.set_u32(a, b);
        e.mem.set_u32(b, 0);
        e.mem.set_u32(this + 0x1c, a);
        e.mem.set_u32(this + 0x20, 2);
        e.call_log = Some(vec![]);
        e.call(0x0044_f220, &[this]);
        assert_eq!(
            calls_to(&e, RELEASE_0044DDE0),
            vec![vec![map, a], vec![map, b]]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0);

        // The loops: fn_0044f420 and fn_0044f200 stop once nothing is left.
        e.mem.set_u32(a, 0);
        e.mem.set_u32(this + 0x1c, a);
        e.mem.set_u32(this + 0x20, 1);
        e.call(0x0044_f420, &[this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        e.mem.set_u32(this + 0x1c, a);
        e.mem.set_u32(this + 0x20, 1);
        e.call(0x0044_f200, &[this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }
}
