//! `fallout/interface/run_007da070` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VVideoRes@StartMenu@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01077200: u32 = 0x0107_7200;
/// `007d8e20` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007D8E20: u32 = 0x007d_8e20;

// Translated from 007da0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VVideoRes@StartMenu@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007d8e20(this, 1)`.
pub fn fn_007da0f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01077200);
    e.call(ARRAY_RELEASE_007D8E20, &args![this, 1u32]);
}

/// The constructor body `007da110` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_007DA110: u32 = 0x007d_a110;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<StartMenu::VideoRes,...>`).
const VTABLE_007DA090: u32 = 0x0107_7200;

// Translated from 007da090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `01077200` (RTTI class
/// `BSSimpleArray<StartMenu::VideoRes,...>`), then calls `007da110(this, 0, 0)`. Returns
/// `this`.
pub fn fn_007da090(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_007DA090);
    e.call(CONSTRUCTOR_BODY_007DA110, &args![this, 0u32, 0u32]);
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@PAVUserOption@StartMenu@@$0EAA@@@`.
const USER_OPTION_ARRAY_VTABLE: u32 = 0x0107_71ec;
/// Thiscall (word), `RET 4` (`run_00845230.rs`): the `BSSimpleArray`
/// release body the destructors call with 1.
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// Thiscall on the array (place, count), `RET 8`: constructs `count`
/// elements from `place`.
const ELEMENTS_CONSTRUCT_007DA1A0: u32 = 0x007d_a1a0;
/// `ActorValue::GetActorValueName` (engine map; the body may be folded),
/// cdecl (word): a string.
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
/// Cdecl (text, text): `EAX`; a two-argument wrapper around `00ec6da0`.
const COMPARE_NAMES_00408B20: u32 = 0x0040_8b20;

// Translated from 007da070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@PAVUserOption@StartMenu@@$0EAA@@@` (`thiscall`):
/// stores the instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_007da070(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), USER_OPTION_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 007da110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of the `BSSimpleArray<StartMenu::VideoRes>` instance:
/// clears the buffer (+4), size (+8) and reserved size (+0xC); reserves
/// `max(reserve, size)` elements through vtable +4 when that is non-zero,
/// then constructs `size` elements (`007da1a0`) and sets the size.
pub fn fn_007da110(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ELEMENTS_CONSTRUCT_007DA1A0, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 007da270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a cdecl compare function of two objects: the
/// strings `0066e950` gives for the word at +4 of each, compared by
/// `00408b20` (its result).
pub fn fn_007da270(e: &mut Engine, first: u32, second: u32) -> i32 {
    let first_word = e.mem.u32(first + 4);
    let first_name = e.call(ACTOR_VALUE_NAME, &args![first_word]).u32();
    let second_word = e.mem.u32(second + 4);
    let second_name = e.call(ACTOR_VALUE_NAME, &args![second_word]).u32();
    e.call(COMPARE_NAMES_00408B20, &args![first_name, second_name])
        .i32()
}

/// `MemoryManager` free (`00401030`, cdecl), the placement step (`006e6da0`, cdecl: the
/// size and the address) and `BSString`'s constructor (`004037b0`).
const FREE: u32 = 0x0040_1030;
const PLACE_006E6DA0: u32 = 0x006e_6da0;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;

// Translated from 007da0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `BSSimpleArray<StartMenu::VideoRes>` instance
/// (`thiscall`, `RET 4`): the destructor (`007da0f0`), then the block freed when bit 0
/// of `flags` is set. Answers `this`.
pub fn fn_007da0c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007da0f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007da1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007da1a0 - the C++ exception-unwinding frame (FS:[0] around each construction) is not translated.
/// Unnamed (`thiscall` on the array, `RET 8`: a place and a count; `this` is not read):
/// constructs `count` 12-byte `StartMenu::VideoRes` elements from `place`
/// (`006e6da0(0xc, element)`, then `007da240` on what it answers when non-zero).
pub fn fn_007da1a0(e: &mut Engine, _this: Ptr, place: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        let element = e
            .call(
                PLACE_006E6DA0,
                &args![0xcu32, i.wrapping_mul(0xc).wrapping_add(place)],
            )
            .u32();
        if element != 0 {
            fn_007da240(e, Ptr::new(element));
        }
        i += 1;
    }
}

// Translated from 007da240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; the `StartMenu::VideoRes` constructor by its array): the
/// `BSString` at +0 constructed, the 16-bit words at +8 and +0xa cleared. Answers `this`.
pub fn fn_007da240(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.mem.set_u16(this.addr() + 8, 0);
    e.mem.set_u16(this.addr() + 0xa, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007d_a0f0, fn_007da0f0(Ptr)),
        entry!(0x007d_a090, fn_007da090(Ptr) -> Ptr),
        entry!(0x007d_a070, fn_007da070(Ptr)),
        entry!(0x007d_a110, fn_007da110(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x007d_a270, fn_007da270(u32, u32) -> i32),
        entry!(0x007d_a0c0, fn_007da0c0(Ptr, u32) -> Ptr),
        entry!(0x007d_a1a0, fn_007da1a0(Ptr, u32, u32)),
        entry!(0x007d_a240, fn_007da240(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007da0f0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x007d_a0f0, ARRAY_VTABLE_01077200, ARRAY_RELEASE_007D8E20);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_007DA110, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a090, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_007DA110),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_007DA090);
    }
}

#[cfg(test)]
mod start_menu_array_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_007da070_sets_the_vtable_then_releases() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x007d_a070, &args![this]);
        assert_eq!(e.mem.u32(this), USER_OPTION_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![this, 1]]);
    }

    #[test]
    fn fn_007da110_reserves_and_constructs() {
        let mut e = Engine::new();
        e.register(0x0000_7004, |_, a| Ret {
            eax: 0x5000 + a[1],
            ..Ret::default()
        });
        e.register(ELEMENTS_CONSTRUCT_007DA1A0, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        let this: Ptr<BSSimpleArray> = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr(), 0x0f00_0000);
        e.mem.set_u32(this.addr() + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x007d_a110, &args![this, 5u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![this.addr(), 5]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5005);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(
            calls_to(&e, ELEMENTS_CONSTRUCT_007DA1A0),
            vec![vec![this.addr(), 0x5005, 2]]
        );
        assert_eq!(e.get(this, BSSimpleArray::iSize), 2);
        e.call_log = Some(vec![]);
        e.call(0x007d_a110, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn fn_007da270_compares_the_two_names() {
        let mut e = Engine::new();
        e.register(ACTOR_VALUE_NAME, |_, a| Ret {
            eax: 0x9000 + a[0],
            ..Ret::default()
        });
        e.register(COMPARE_NAMES_00408B20, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 3);
        e.mem.set_u32(second + 4, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a270, &args![first, second]).i32(), -1);
        assert_eq!(calls_to(&e, ACTOR_VALUE_NAME), vec![vec![3], vec![7]]);
        assert_eq!(
            calls_to(&e, COMPARE_NAMES_00408B20),
            vec![vec![0x9003, 0x9007]]
        );
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn video_res_array_destruction_and_elements() {
        let mut e = Engine::new();
        for address in [FREE, ARRAY_RELEASE_007D8E20, STRING_CONSTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_a0c0, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_01077200);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ARRAY_RELEASE_007D8E20, vec![array, 1])));
        assert!(log.contains(&(FREE, vec![array])));
        let elements = e.mem.alloc(0x24);
        for i in 0..0x24 {
            e.mem.set_u8(elements + i, 0xff);
        }
        // The middle element gets no place.
        e.register_double(PLACE_006E6DA0, move |_, a| {
            if a[1] == elements + 0xc { 0 } else { a[1] }.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007d_a1a0, &args![array, elements, 3u32]);
        assert_eq!(e.mem.u32(elements + 8), 0);
        assert_eq!(e.mem.u32(elements + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(elements + 0x20), 0);
        let built: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == STRING_CONSTRUCT)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(built, vec![elements, elements + 0x18]);
    }
}
