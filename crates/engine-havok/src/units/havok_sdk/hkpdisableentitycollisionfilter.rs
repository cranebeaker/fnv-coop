//! `Havok SDK/hkpdisableentitycollisionfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dff960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00dff930`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff960(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f930, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00dff980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00dff930`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff980(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f930, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00e46780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDisableEntityCollisionFilter::isCollisionEnabled_ov3` (Ghidra name): `ECX` = filter, six
/// stack words (`RET 0x18`); only the first, the address of a
/// byte, is read. Sets that byte to 1 and returns its address.
pub fn hkp_disable_entity_collision_filter_is_collision_enabled_ov3(
    e: &mut Engine,
    _this: Ptr,
    result: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> Ptr {
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00e46790 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra names `hkpDisableEntityCollisionFilter::_vector_deleting_destructor__ov2`
/// (the name is unconfirmed; the code is not a destructor): `ECX` = filter, five stack words,
/// only the first, the address of a byte, is read. Sets that byte to 1 and returns its address.
pub fn fn_00e46790(
    e: &mut Engine,
    _this: Ptr,
    result: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> Ptr {
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00e467a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDisableEntityCollisionFilter::isCollisionEnabled_ov4` (Ghidra name): `ECX` = filter,
/// eight stack words, only the first, the address of a byte, is read. Sets that byte to 1 and
/// returns its address.
pub fn hkp_disable_entity_collision_filter_is_collision_enabled_ov4(
    e: &mut Engine,
    _this: Ptr,
    result: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
) -> Ptr {
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00e467d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra names `hkpDisableEntityCollisionFilter::_vector_deleting_destructor__ov3`
/// (the name is unconfirmed; the code is not a destructor): `ECX` = filter, three stack words:
/// the address of a result byte and two addresses. Walks the array of pointers (data `+0x2c`,
/// size `+0x30`); when an entry plus 0x10 equals either address the byte is set to 0 and the
/// walk ends, otherwise it is set to 1 after the whole array. Returns the byte's address.
pub fn fn_00e467d0(e: &mut Engine, this: Ptr, result: Ptr, first: u32, second: u32) -> Ptr {
    let data = e.mem.u32(this.addr() + 0x2c);
    let count = e.mem.i32(this.addr() + 0x30);
    let mut enabled = 1;
    for index in 0..count.max(0) as u32 {
        let entry = e.mem.u32(data + 4 * index).wrapping_add(0x10);
        if entry == first || entry == second {
            enabled = 0;
            break;
        }
    }
    e.mem.set_u8(result.addr(), enabled);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f960, fn_00dff960(Ptr, u32) -> u32),
        entry!(0x00df_f980, fn_00dff980(Ptr, u32) -> u32),
        entry!(
            0x00e4_6780,
            hkp_disable_entity_collision_filter_is_collision_enabled_ov3(
                Ptr, Ptr, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x00e4_6790, fn_00e46790(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00e4_67a0,
            hkp_disable_entity_collision_filter_is_collision_enabled_ov4(
                Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x00e4_67d0, fn_00e467d0(Ptr, Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dff960() {
        assert_eq!(check(0x00df_f960, 0x00df_f930, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00dff980() {
        assert_eq!(check(0x00df_f980, 0x00df_f930, 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Calls `address` with `this = 0`, the result byte, then `extra` unread words, and
    /// checks the byte is set to 1 and its address comes back.
    fn check_constant_result(address: u32, extra: usize) {
        let mut e = engine();
        let result = e.mem.alloc(8);
        let mut words = vec![0u32, result];
        words.resize(2 + extra, 0x77);
        assert_eq!(e.call(address, &words).u32(), result);
        assert_eq!(e.mem.u8(result), 1);
    }

    #[test]
    fn the_constant_answers_set_the_result_byte() {
        check_constant_result(0x00e4_6780, 5);
        check_constant_result(0x00e4_6790, 4);
        check_constant_result(0x00e4_67a0, 7);
    }

    /// Runs `00e467d0` on `this` and returns the result byte (preset to 0x55).
    fn walk(e: &mut Engine, this: u32, first: u32, second: u32) -> u8 {
        let result = e.mem.alloc(8);
        e.mem.set_u8(result, 0x55);
        let returned = e.call(0x00e4_67d0, &args![this, result, first, second]);
        assert_eq!(returned.u32(), result);
        e.mem.u8(result)
    }

    #[test]
    fn entity_list_walk_disables_a_match_on_either_address() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x1000);
        e.mem.set_u32(data + 4, 0x2000);
        e.mem.set_u32(this + 0x2c, data);
        e.mem.set_u32(this + 0x30, 2);
        assert_eq!(walk(&mut e, this, 0x1010, 0), 0);
        assert_eq!(walk(&mut e, this, 0, 0x2010), 0);
        assert_eq!(walk(&mut e, this, 0x1000, 0x3010), 1);
        // An empty list never matches.
        e.mem.set_u32(this + 0x30, 0);
        assert_eq!(walk(&mut e, this, 0x1010, 0x2010), 1);
    }
}
