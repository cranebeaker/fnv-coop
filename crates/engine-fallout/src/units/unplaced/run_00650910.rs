//! `(unplaced)/run_00650910` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPointer` temporary's constructor (`00633c90`; `thiscall`: object) and destructor
/// (`0045cec0`; `thiscall`).
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `00559450`: answers the pointer an `NiPointer` holds (`thiscall`).
const POINTER_GET: u32 = 0x0055_9450;
/// The lock pair around the map walks (`00652140` takes it; `00652160` tries and answers
/// whether it got it; `00652190` releases it); no arguments.
const LOCK: u32 = 0x0065_2140;
const TRY_LOCK: u32 = 0x0065_2160;
const UNLOCK: u32 = 0x0065_2190;
/// The map at `this + 4`: its first position (`004b9ba0`; `thiscall`), the next entry
/// (`00559120`; `thiscall`: addresses of position, key and value) and `RemoveAt`
/// (`00405430`; `thiscall`: key).
const MAP_FIRST: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x0055_9120;
const MAP_REMOVE: u32 = 0x0040_5430;
/// The per-entry size functions of kind 0 (`0065c110`) and kind 1 (`0065c490`), the
/// entry's reference count (`00726070`), its unload test (`0065c090`) and its update
/// (`0065c3a0`), all `thiscall` on the entry's model.
const SIZE_OF_KIND_0: u32 = 0x0065_c110;
const SIZE_OF_KIND_1: u32 = 0x0065_c490;
const REFERENCE_COUNT: u32 = 0x0072_6070;
const TRY_UNLOAD: u32 = 0x0065_c090;
const UPDATE_ENTRY: u32 = 0x0065_c3a0;
/// The release step of `00650ba0` (`00650cb0`; `thiscall`: kind, the argument).
const RELEASE_STEP: u32 = 0x0065_0cb0;

// Translated from 00650910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the word at `this + 0x18 + 4 * kind`.
pub fn fn_00650910(e: &mut Engine, this: Ptr, kind: u32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(0x18)
            .wrapping_add(kind.wrapping_mul(4)),
    )
}

/// The model behind a map entry's value: `00559450` on the value holder, then on the
/// holder at `+8` of what it answers.
fn entry_model(e: &mut Engine, value: Ptr) -> u32 {
    let entry = e.call(POINTER_GET, &args![value]).u32();
    e.call(POINTER_GET, &args![entry.wrapping_add(8)]).u32()
}

// Translated from 00650930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650930 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`): under the lock (`00652140` ..
/// `00652190`) walks the map at `this + 4` and sums every entry's size of `kind` (0:
/// `0065c110`, 1: `0065c490`, other kinds add nothing). Key and value go through stack
/// words, the value an `NiPointer` temporary (`00633c90` .. `0045cec0`).
pub fn fn_00650930(e: &mut Engine, this: Ptr, kind: u32) -> u32 {
    e.with_stack(12, |e, frame| {
        let position = Ptr::<()>::new(frame.addr());
        let key = Ptr::<()>::new(frame.addr() + 4);
        let value = Ptr::new(frame.addr() + 8);
        e.mem.set_u32(key.addr(), 0);
        e.call(POINTER_CONSTRUCTOR, &args![value, 0u32]);
        let mut total = 0u32;
        e.call(LOCK, &args![]);
        let map = this.addr() + 4;
        let first = e.call(MAP_FIRST, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.call(MAP_NEXT, &args![map, position, key, value]);
            let size = match kind {
                0 => {
                    let model = entry_model(e, value);
                    e.call(SIZE_OF_KIND_0, &args![model]).u32()
                }
                1 => {
                    let model = entry_model(e, value);
                    e.call(SIZE_OF_KIND_1, &args![model]).u32()
                }
                _ => continue,
            };
            total = size.wrapping_add(total);
        }
        e.call(UNLOCK, &args![]);
        e.call(POINTER_DESTRUCTOR, &args![value]);
        total
    })
}

// Translated from 00650a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650a30 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`). With `wait` set the lock is taken
/// (`00652140`), else only tried (`00652160`; nothing more happens without it). Then every
/// entry of the map at `this + 4` whose model's reference count (`00726070`) is 1 is
/// removed (`00405430`); the others with an entry and a model are asked to unload
/// (`0065c090`, a refusal remembered) and updated (`0065c3a0`). Without a refusal the byte
/// at `this + 0x20` is cleared. The lock is released (`00652190`).
pub fn fn_00650a30(e: &mut Engine, this: Ptr, wait: u8) {
    e.with_stack(12, |e, frame| {
        let position = Ptr::<()>::new(frame.addr());
        let key = Ptr::<()>::new(frame.addr() + 4);
        let value = Ptr::new(frame.addr() + 8);
        e.mem.set_u32(key.addr(), 0);
        e.call(POINTER_CONSTRUCTOR, &args![value, 0u32]);
        let locked = if wait == 0 {
            e.call(TRY_LOCK, &args![]).u8()
        } else {
            e.call(LOCK, &args![]);
            wait
        };
        if locked != 0 {
            let mut refused = false;
            let map = this.addr() + 4;
            let first = e.call(MAP_FIRST, &args![map]).u32();
            e.mem.set_u32(position.addr(), first);
            while e.mem.u32(position.addr()) != 0 {
                e.call(MAP_NEXT, &args![map, position, key, value]);
                let model = entry_model(e, value);
                if e.call(REFERENCE_COUNT, &args![model]).u32() == 1 {
                    let removed = e.mem.u32(key.addr());
                    e.call(MAP_REMOVE, &args![map, removed]);
                    continue;
                }
                if e.call(POINTER_GET, &args![value]).u32() == 0 {
                    continue;
                }
                if entry_model(e, value) == 0 {
                    continue;
                }
                let model = entry_model(e, value);
                if !e.call(TRY_UNLOAD, &args![model]).bool() {
                    refused = true;
                }
                let model = entry_model(e, value);
                e.call(UPDATE_ENTRY, &args![model]);
            }
            if !refused {
                e.mem.set_u8(this.addr() + 0x20, 0);
            }
            e.call(UNLOCK, &args![]);
        }
        e.call(POINTER_DESTRUCTOR, &args![value]);
    });
}

// Translated from 00650ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`). Under the lock, for kinds 0 and 1:
/// with a non-null `model` whose size of that kind (`0065c110` / `0065c490` on its holder
/// at `+8`) is not zero the kind is skipped; otherwise (margin 10 for a zero size, none for
/// a null `model`) while the kind's total ([`fn_00650930`]) plus the margin is above its
/// limit ([`fn_00650910`]) the release step `00650cb0` runs, until the total stops
/// changing.
pub fn fn_00650ba0(e: &mut Engine, this: Ptr, model: u32) {
    let mut margin = 0u32;
    e.call(LOCK, &args![]);
    for kind in 0..2u32 {
        if model != 0 {
            let size_function = if kind == 0 {
                SIZE_OF_KIND_0
            } else {
                SIZE_OF_KIND_1
            };
            let holder = e.call(POINTER_GET, &args![model.wrapping_add(8)]).u32();
            if e.call(size_function, &args![holder]).u32() != 0 {
                continue;
            }
            margin = 10;
        }
        let mut current = fn_00650930(e, this, kind);
        loop {
            let wanted = current.wrapping_add(margin);
            if wanted <= fn_00650910(e, this, kind) {
                break;
            }
            let before = current;
            e.call(RELEASE_STEP, &args![this, kind, model]);
            current = fn_00650930(e, this, kind);
            if current == before {
                break;
            }
        }
    }
    e.call(UNLOCK, &args![]);
}

/// `GetTickCount`'s wrapper (`00457fe0`, no arguments), `NiPointer` equality with a plain
/// pointer (`00822510`; `thiscall`: the pointer) and `NiPointer` assignment from another
/// (`006e5cc0`; `thiscall`: the other's address).
const TICK_COUNT: u32 = 0x0045_7fe0;
const POINTER_EQUALS: u32 = 0x0082_2510;
const POINTER_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 00650cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00650cb0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 8`; the release step of [`fn_00650ba0`]).
/// Under the lock, walks the map at `this + 4` (key and value through stack words, the
/// value an `NiPointer` temporary) and, skipping the entry holding `model` (`00822510`)
/// and those whose size of `kind` is zero (0: `0065c110`, 1: `0065c490`, other kinds:
/// all skipped), keeps the entry whose word `+0xC` is the oldest against the tick count
/// read at the start (`now - word` largest, unsigned; the first found when none is kept
/// yet). The kept entry's model then gets `0065c090` (kind 0) or `0065c3a0` (kind 1).
/// The key of the kept entry is copied to a local the function never reads again.
pub fn fn_00650cb0(e: &mut Engine, this: Ptr, kind: u32, model: u32) {
    e.with_stack(16, |e, frame| {
        let position = Ptr::<()>::new(frame.addr());
        let key = Ptr::<()>::new(frame.addr() + 4);
        let value = Ptr::new(frame.addr() + 8);
        let best = Ptr::new(frame.addr() + 12);
        e.mem.set_u32(key.addr(), 0);
        e.call(POINTER_CONSTRUCTOR, &args![value, 0u32]);
        e.call(POINTER_CONSTRUCTOR, &args![best, 0u32]);
        let now = e.call(TICK_COUNT, &args![]).u32();
        e.call(LOCK, &args![]);
        let map = this.addr() + 4;
        let first = e.call(MAP_FIRST, &args![map]).u32();
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            e.call(MAP_NEXT, &args![map, position, key, value]);
            if e.call(POINTER_EQUALS, &args![value, model]).u8() != 0 {
                continue;
            }
            let size = match kind {
                0 => {
                    let model = entry_model(e, value);
                    e.call(SIZE_OF_KIND_0, &args![model]).u32()
                }
                1 => {
                    let model = entry_model(e, value);
                    e.call(SIZE_OF_KIND_1, &args![model]).u32()
                }
                _ => 0,
            };
            if size == 0 {
                continue;
            }
            if e.call(POINTER_GET, &args![best]).u32() == 0 {
                e.call(POINTER_ASSIGN, &args![best, value]);
            } else {
                let entry = e.call(POINTER_GET, &args![value]).u32();
                let age = now.wrapping_sub(e.mem.u32(entry.wrapping_add(0xc)));
                let kept = e.call(POINTER_GET, &args![best]).u32();
                let kept_age = now.wrapping_sub(e.mem.u32(kept.wrapping_add(0xc)));
                if age > kept_age {
                    e.call(POINTER_ASSIGN, &args![best, value]);
                }
            }
        }
        if e.call(POINTER_GET, &args![best]).u32() != 0 {
            match kind {
                0 => {
                    let model = entry_model(e, best);
                    e.call(TRY_UNLOAD, &args![model]);
                }
                1 => {
                    let model = entry_model(e, best);
                    e.call(UPDATE_ENTRY, &args![model]);
                }
                _ => {}
            }
        }
        e.call(UNLOCK, &args![]);
        e.call(POINTER_DESTRUCTOR, &args![best]);
        e.call(POINTER_DESTRUCTOR, &args![value]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_0910, fn_00650910(Ptr, u32) -> u32),
        entry!(0x0065_0930, fn_00650930(Ptr, u32) -> u32),
        entry!(0x0065_0a30, fn_00650a30(Ptr, u8)),
        entry!(0x0065_0ba0, fn_00650ba0(Ptr, u32)),
        entry!(0x0065_0cb0, fn_00650cb0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine whose map walk yields the given (key, model) entries; each value holder
    /// is a word holding an entry whose `+8` word holds the model.
    fn engine(entries: &[(u32, u32)]) -> Engine {
        let mut e = Engine::new();
        let mut holders = vec![];
        for (key, model) in entries {
            let entry = e.mem.alloc(0x10);
            e.mem.set_u32(entry + 8, *model);
            holders.push((*key, entry));
        }
        let list = Rc::new(RefCell::new(holders));
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(POINTER_DESTRUCTOR, |_, _| Ret::default());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        for address in [LOCK, UNLOCK, MAP_REMOVE, UPDATE_ENTRY, RELEASE_STEP] {
            e.register(address, |_, _| Ret::default());
        }
        let first = list.clone();
        e.register_double(MAP_FIRST, move |_, _| {
            u32::from(!first.borrow().is_empty()).into_ret()
        });
        let next = list;
        e.register_double(MAP_NEXT, move |e, a| {
            let (key, entry) = next.borrow_mut().remove(0);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], entry);
            e.mem.set_u32(a[1], u32::from(!next.borrow().is_empty()));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn kind_word_reads_the_table() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1c, 0x55);
        assert_eq!(e.call(0x0065_0910, &args![this, 1u32]).u32(), 0x55);
    }

    #[test]
    fn total_sums_the_kind_sizes() {
        let mut e = engine(&[(1, 0x100), (2, 0x200)]);
        e.register(SIZE_OF_KIND_0, |_, a| (a[0] / 0x10).into_ret());
        e.register(SIZE_OF_KIND_1, |_, a| (a[0] / 0x100).into_ret());
        let this = e.mem.alloc(0x24);
        assert_eq!(e.call(0x0065_0930, &args![this, 0u32]).u32(), 0x30);
        assert_eq!(calls(&e, LOCK).len(), 1);
        assert_eq!(calls(&e, UNLOCK).len(), 1);
        assert_eq!(calls(&e, MAP_FIRST), vec![vec![this + 4]]);
        let mut e = engine(&[(1, 0x100), (2, 0x200)]);
        e.register(SIZE_OF_KIND_1, |_, a| (a[0] / 0x100).into_ret());
        assert_eq!(e.call(0x0065_0930, &args![this, 1u32]).u32(), 3);
        let mut e = engine(&[(1, 0x100)]);
        assert_eq!(e.call(0x0065_0930, &args![this, 2u32]).u32(), 0);
        assert_eq!(calls(&e, POINTER_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn cleanup_removes_single_references_and_unloads_the_rest() {
        let mut e = engine(&[(7, 0x100), (8, 0x200), (9, 0x300)]);
        e.register(REFERENCE_COUNT, |_, a| u32::from(a[0] == 0x100).into_ret());
        e.register(TRY_UNLOAD, |_, a| u32::from(a[0] != 0x300).into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x20, 1);
        e.call(0x0065_0a30, &args![this, 1u32]);
        assert_eq!(calls(&e, MAP_REMOVE), vec![vec![this + 4, 7]]);
        assert_eq!(calls(&e, UPDATE_ENTRY), vec![vec![0x200], vec![0x300]]);
        // One refusal keeps the byte.
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(calls(&e, LOCK).len(), 1);
    }

    #[test]
    fn cleanup_without_refusals_clears_the_byte_and_skips_without_the_lock() {
        let mut e = engine(&[(8, 0x200)]);
        e.register(REFERENCE_COUNT, |_, _| 2u32.into_ret());
        e.register(TRY_UNLOAD, |_, _| 1u32.into_ret());
        e.register(TRY_LOCK, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x20, 1);
        e.call(0x0065_0a30, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x20), 0);
        assert!(calls(&e, LOCK).is_empty());
        let mut e = engine(&[(8, 0x200)]);
        e.register(TRY_LOCK, |_, _| Ret::default());
        e.call(0x0065_0a30, &args![this, 0u32]);
        assert!(calls(&e, MAP_FIRST).is_empty());
        assert!(calls(&e, UNLOCK).is_empty());
        assert_eq!(calls(&e, POINTER_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn trim_releases_until_the_total_stops_changing() {
        let mut e = engine(&[]);
        // The model's kind-0 size is 0 (margin 10), its kind-1 size is not (kind skipped).
        e.register(SIZE_OF_KIND_0, |_, _| Ret::default());
        e.register(SIZE_OF_KIND_1, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x18, 8);
        e.mem.set_u32(this + 0x1c, 100);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model + 8, 0x4444);
        e.call(0x0065_0ba0, &args![this, model]);
        // The total (no entries) stays 0; 0 + 10 is above the limit 8, one release changes
        // nothing, so the loop stops.
        assert_eq!(calls(&e, RELEASE_STEP), vec![vec![this, 0, model]]);
        assert_eq!(calls(&e, SIZE_OF_KIND_0), vec![vec![0x4444]]);
        assert_eq!(calls(&e, SIZE_OF_KIND_1), vec![vec![0x4444]]);
        // The lock of this function and of the two totals.
        assert_eq!(calls(&e, LOCK).len(), 3);
        assert_eq!(calls(&e, UNLOCK).len(), 3);
    }

    #[test]
    fn trim_without_a_model_checks_both_kinds() {
        let mut e = engine(&[]);
        let this = e.mem.alloc(0x24);
        e.call(0x0065_0ba0, &args![this, 0u32]);
        assert!(calls(&e, RELEASE_STEP).is_empty());
        assert_eq!(calls(&e, MAP_FIRST).len(), 2);
    }

    #[test]
    fn release_step_frees_the_oldest_other_entry() {
        let mut e = engine(&[]);
        // Entries (model, time stamp at +0xC): the oldest is skipped when it holds `model`.
        let mut entries = vec![];
        for (model, stamp) in [(0x100u32, 50u32), (0x200, 10), (0x300, 5), (0x400, 1)] {
            let entry = e.mem.alloc(0x10);
            e.mem.set_u32(entry + 8, model);
            e.mem.set_u32(entry + 0xc, stamp);
            entries.push(entry);
        }
        let skipped = entries[3];
        let list = Rc::new(RefCell::new(entries));
        let first = list.clone();
        e.register_double(MAP_FIRST, move |_, _| {
            u32::from(!first.borrow().is_empty()).into_ret()
        });
        e.register_double(MAP_NEXT, move |e, a| {
            let entry = list.borrow_mut().remove(0);
            e.mem.set_u32(a[2], entry);
            e.mem.set_u32(a[3], entry);
            e.mem.set_u32(a[1], u32::from(!list.borrow().is_empty()));
            Ret::default()
        });
        e.register(TICK_COUNT, |_, _| 100u32.into_ret());
        e.register(POINTER_EQUALS, |e, a| (e.mem.u32(a[0]) == a[1]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            let other = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], other);
            a[0].into_ret()
        });
        // Kind 0: the third entry has no size.
        e.register(SIZE_OF_KIND_0, |_, a| u32::from(a[0] != 0x300).into_ret());
        e.register(TRY_UNLOAD, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        // `engine` doubles the release step for the callers' tests: call it directly.
        fn_00650cb0(&mut e, Ptr::new(this), 0, skipped);
        assert_eq!(calls(&e, TRY_UNLOAD), vec![vec![0x200]]);
        assert!(calls(&e, UPDATE_ENTRY).is_empty());
        assert_eq!(calls(&e, POINTER_ASSIGN).len(), 2);
        assert_eq!(calls(&e, LOCK).len(), 1);
        assert_eq!(calls(&e, UNLOCK).len(), 1);
        assert_eq!(calls(&e, POINTER_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn release_step_of_another_kind_frees_nothing() {
        let mut e = engine(&[(1, 0x100)]);
        e.register(TICK_COUNT, |_, _| Ret::default());
        e.register(POINTER_EQUALS, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        fn_00650cb0(&mut e, Ptr::new(this), 2, 0);
        assert!(calls(&e, TRY_UNLOAD).is_empty());
        assert!(calls(&e, UPDATE_ENTRY).is_empty());
        assert_eq!(calls(&e, MAP_NEXT).len(), 1);
    }
}
