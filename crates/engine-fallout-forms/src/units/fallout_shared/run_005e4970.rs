//! `fallout shared/run_005e4970` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks `this` through its vtable slot +0xB4 (with the address of a
/// one-byte local) and answers the result moved back by 4 bytes (a base
/// class at +4 cast to the whole object), or null for a null result (the
/// engine map has no name). The byte local, uninitialized in the exe, is
/// zero here.
pub fn fn_005e4ab0(e: &mut Engine, this: Ptr) -> u32 {
    let result = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if result != 0 {
        result.wrapping_sub(4)
    } else {
        0
    }
}

/// `BSDismemberSkinInstance::SetBodyPartEnabled` (`005e4810`, thiscall:
/// the part word, the enabled byte).
const SET_BODY_PART_ENABLED: u32 = 0x005e_4810;

// Translated from 005e4970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows one body part on an object's dismember skins (cdecl: the part
/// index, the object; the engine map has no name): the object's list
/// (vtable slot 0xC); for each of its entries while the index is below
/// the list's count (`0043b480`): the entry (`0043b4a0`) or, when it is
/// null, the object itself answers vtable slot 0x18; its result's
/// `0043fad0` (0 for a null result) goes through `00653270(011f49d8, ..)`
/// (cdecl), and when that answers a skin instance, for `i` from 0 to 14
/// `SetBodyPartEnabled(part + 100, 1)` when `i` is the part, otherwise
/// `(i & 0xFFFF, 0)` and `(i + 200, 0)`, then `(i * 1000, 0)` for each `i`.
pub fn fn_005e4970(e: &mut Engine, part: u32, object: u32) {
    let list = e.vcall(object, 0xc, &args![]).u32();
    let mut index = 0u32;
    while list != 0 && index < e.call(0x0043_b480, &args![list]).u32() {
        let entry = if e.call(0x0043_b4a0, &args![list, index]).u32() != 0 {
            let entry = e.call(0x0043_b4a0, &args![list, index]).u32();
            e.vcall(entry, 0x18, &args![]).u32()
        } else {
            e.vcall(object, 0x18, &args![]).u32()
        };
        index += 1;
        let key = if entry != 0 {
            e.call(0x0043_fad0, &args![entry]).u32()
        } else {
            0
        };
        let skin = e.call(0x0065_3270, &args![0x011f_49d8u32, key]).u32();
        if skin == 0 {
            continue;
        }
        for i in 0..0xfu32 {
            if i == part {
                e.call(
                    SET_BODY_PART_ENABLED,
                    &args![skin, part.wrapping_add(100), 1u32],
                );
            } else {
                e.call(SET_BODY_PART_ENABLED, &args![skin, i & 0xffff, 0u32]);
                e.call(SET_BODY_PART_ENABLED, &args![skin, i + 200, 0u32]);
            }
            e.call(SET_BODY_PART_ENABLED, &args![skin, i * 1000, 0u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_4ab0, fn_005e4ab0(Ptr) -> u32),
        entry!(0x005e_4970, fn_005e4970(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_b4_result_is_moved_back_four_bytes() {
        const TARGET: u32 = 0x0300_0000;
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0xb8);
        e.mem.set_u32(vtable + 0xb4, TARGET);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.register(TARGET, |_, _| 0x1004u32.into_ret());
        assert_eq!(e.call(0x005e_4ab0, &args![this]).u32(), 0x1000);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(0x005e_4ab0, &args![this]).u32(), 0);
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

    /// An object whose list (slot 0xC) holds a null entry and an entry
    /// object; every object answers its own address + 0x100 from slot
    /// 0x18; `0043fad0` answers its argument + 1; `00653270` answers a skin
    /// only for the object's own key.
    #[test]
    fn fn_005e4970_enables_the_part_on_each_skin() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x7000_000c);
        e.mem.set_u32(vtable + 0x18, 0x7000_0018);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, vtable);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list, 2);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, entry);
        e.register_double(0x7000_000c, move |_, _| ret(list));
        e.register(0x7000_0018, |_, a| ret(a[0] + 0x100));
        e.register(0x0043_b480, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_b4a0, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        e.register(0x0043_fad0, |_, a| ret(a[0] + 1));
        e.register_double(0x0065_3270, move |_, a| {
            ret(if a[1] == object + 0x101 { 0x5000 } else { 0 })
        });
        e.register(SET_BODY_PART_ENABLED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_4970, &args![3u32, object]);
        assert_eq!(
            calls(&e, 0x0065_3270),
            vec![
                vec![0x011f_49d8, object + 0x101],
                vec![0x011f_49d8, entry + 0x101]
            ]
        );
        let set = calls(&e, SET_BODY_PART_ENABLED);
        // Fourteen parts off twice, one on, and fifteen thousands.
        assert_eq!(set.len(), 14 * 2 + 1 + 15);
        assert!(set.iter().all(|c| c[0] == 0x5000));
        assert_eq!(
            &set[..3],
            &[vec![0x5000, 0, 0], vec![0x5000, 200, 0], vec![0x5000, 0, 0]]
        );
        assert!(set.contains(&vec![0x5000, 103, 1]));
        assert!(!set.contains(&vec![0x5000, 3, 0]));
        assert!(set.contains(&vec![0x5000, 3000, 0]));
        assert!(set.contains(&vec![0x5000, 14000, 0]));

        // No list: nothing.
        e.register(0x7000_000c, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005e_4970, &args![3u32, object]);
        assert!(calls(&e, 0x0043_b480).is_empty());
    }
}
