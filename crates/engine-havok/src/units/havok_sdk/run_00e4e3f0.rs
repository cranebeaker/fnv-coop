//! `Havok SDK/run_00e4e3f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl, `(allocator, array, count, element size)`.
const RESERVE: u32 = 0x00c9_08c0;
/// `hkStringPtr::hkStringPtr` (Xbox PDB, `00cb0a60`): thiscall on the string pointer, `this` only.
const STRING_CONSTRUCTOR: u32 = 0x00cb_0a60;
/// `hkStringPtr` copy constructor (`00cb0bd0`): thiscall on the new string pointer, one word (the
/// address of the source string pointer), `RET 4`.
const STRING_COPY_CONSTRUCTOR: u32 = 0x00cb_0bd0;
/// `hkStringPtr` destructor (`00cb0c00`): thiscall, `this` only.
const STRING_DESTRUCTOR: u32 = 0x00cb_0c00;
/// `hkStringPtr::operator=` (`00cb0c30`): thiscall on the target, one word (the address of the
/// source string pointer), `RET 4`.
const STRING_ASSIGN: u32 = 0x00cb_0c30;

// Translated from 00e4e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00c85750` (the memory router lookup, which calls `TlsGetValue`): no arguments,
/// returns its result.
pub fn fn_00e4e3f0(e: &mut Engine) -> u32 {
    e.call(MEMORY_ROUTER, &[]).u32()
}

// Translated from 00e4e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: builds the string pointer at `this` (`00cb0a60`) and an empty unowned
/// `hkArray` (`data` 0, `size` 0, `capacity | flags` `0x80000000`) at `this + 4`. Returns `this`.
pub fn fn_00e4e400(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.call(STRING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.wrapping_add(4), 0);
    e.mem.set_u32(this.wrapping_add(8), 0);
    e.mem.set_u32(this.wrapping_add(0xc), 0x8000_0000);
    this
}

// Translated from 00e4e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall copy constructor, `RET 4`: a string pointer at `+0` (copied from the one at
/// `other`, `00cb0bd0`) and an `hkArray` of 8-byte elements (a `float` and a string pointer) at
/// `+4` (`data`, `size` at `+8`, `capacity | flags` at `+0xc`), built empty and unowned and then
/// filled from the array of `other` (`size` at `other + 8`): room is reserved through the
/// allocator at `+0x10` of the router (`00c908c0`, the larger of twice the capacity and the
/// count) when the capacity is below the count; elements beyond the count are destroyed, the
/// first `min(old size, count)` are assigned (the `float` through the x87 stack, the string
/// pointer with `00cb0c30`) and the rest constructed in place (`00cb0bd0`). Returns `this`. The
/// array is empty at that point, so only the construction loop runs in practice; the others are
/// translated as the exe has them.
pub fn fn_00e4e420(e: &mut Engine, this: Ptr, other: u32) -> u32 {
    let this = this.addr();
    e.call(STRING_COPY_CONSTRUCTOR, &args![this, other]);
    let array = this.wrapping_add(4);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let old_size = e.mem.i32(array.wrapping_add(4));
    let count = e.mem.i32(other.wrapping_add(8));
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let kept = if count > old_size { old_size } else { count };
    let capacity = (e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let data = e.mem.u32(array);
    let surplus = old_size.wrapping_sub(count);
    if surplus > 0 {
        let first = data
            .wrapping_add((count as u32).wrapping_mul(8))
            .wrapping_add(4);
        for index in 0..surplus as u32 {
            e.call(STRING_DESTRUCTOR, &args![first.wrapping_add(8 * index)]);
        }
    }
    let source = e.mem.u32(other.wrapping_add(4));
    let target = e.mem.u32(array);
    for index in 0..kept.max(0) as u32 {
        let bits = e.mem.u32(source.wrapping_add(8 * index));
        e.mem
            .set_u32(target.wrapping_add(8 * index), x87_quiet_bits(bits));
        e.call(
            STRING_ASSIGN,
            &args![
                target.wrapping_add(8 * index).wrapping_add(4),
                source.wrapping_add(8 * index).wrapping_add(4)
            ],
        );
    }
    let source = e
        .mem
        .u32(other.wrapping_add(4))
        .wrapping_add((kept as u32).wrapping_mul(8));
    let target = e.mem.u32(array).wrapping_add((kept as u32).wrapping_mul(8));
    for index in 0..count.wrapping_sub(kept).max(0) as u32 {
        let to = target.wrapping_add(8 * index);
        // The exe skips a null destination (a placement-new check).
        if to != 0 {
            let bits = e.mem.u32(source.wrapping_add(8 * index));
            e.mem.set_u32(to, x87_quiet_bits(bits));
            e.call(
                STRING_COPY_CONSTRUCTOR,
                &args![
                    to.wrapping_add(4),
                    source.wrapping_add(8 * index).wrapping_add(4)
                ],
            );
        }
    }
    e.mem.set_i32(array.wrapping_add(4), count);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_e3f0, fn_00e4e3f0() -> u32),
        entry!(0x00e4_e400, fn_00e4e400(Ptr) -> u32),
        entry!(0x00e4_e420, fn_00e4e420(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0010_0000);
        e.map(0x0126_0000, 0x0000_1000);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x4444);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [
            STRING_CONSTRUCTOR,
            STRING_COPY_CONSTRUCTOR,
            STRING_DESTRUCTOR,
            STRING_ASSIGN,
        ] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let log = Rc::clone(&calls);
        e.register_double(RESERVE, move |e, a| {
            log.borrow_mut().push((RESERVE, a.to_vec()));
            let data = e.mem.alloc(8 * a[2] + 8);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        (e, calls)
    }

    #[test]
    fn router_thunk_returns_the_router() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00e4_e3f0, &[]).u32(), 0x0126_0000);
    }

    #[test]
    fn constructor_builds_the_string_and_the_empty_array() {
        let (mut e, calls) = engine();
        let this = e.mem.alloc(0x20);
        for word in 0..8u32 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e4_e400, &args![this]).u32(), this);
        assert_eq!(*calls.borrow(), vec![(STRING_CONSTRUCTOR, vec![this])]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
    }

    /// A source object holding `count` elements, the float of element `k` being `k + 1`.
    fn source(e: &mut Engine, floats: &[u32]) -> u32 {
        let source = e.mem.alloc(0x20);
        let data = e.mem.alloc(8 * floats.len() as u32 + 8);
        for (index, bits) in floats.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *bits);
        }
        e.mem.set_u32(source + 4, data);
        e.mem.set_u32(source + 8, floats.len() as u32);
        source
    }

    #[test]
    fn copy_constructor_reserves_and_constructs_every_element() {
        let (mut e, calls) = engine();
        let floats = [0x3f80_0000, 0x7f80_0001, 0x4000_0000];
        let other = source(&mut e, &floats);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00e4_e420, &args![this, other]).u32(), this);
        let data = e.mem.u32(this + 4);
        let seen = calls.borrow();
        assert_eq!(seen[0], (STRING_COPY_CONSTRUCTOR, vec![this, other]));
        assert_eq!(seen[1], (RESERVE, vec![0x4444, this + 4, 3, 8]));
        let constructed: Vec<Vec<u32>> = seen[2..].iter().map(|c| c.1.clone()).collect();
        let source_data = e.mem.u32(other + 4);
        assert_eq!(
            constructed,
            vec![
                vec![data + 4, source_data + 4],
                vec![data + 12, source_data + 12],
                vec![data + 20, source_data + 20]
            ]
        );
        assert!(seen[2..].iter().all(|c| c.0 == STRING_COPY_CONSTRUCTOR));
        // The floats go through the x87 stack: the signalling NaN is quieted.
        let copied: Vec<u32> = (0..3).map(|index| e.mem.u32(data + 8 * index)).collect();
        assert_eq!(copied, vec![0x3f80_0000, 0x7fc0_0001, 0x4000_0000]);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 3);
    }

    #[test]
    fn copy_constructor_of_an_empty_array_reserves_nothing() {
        let (mut e, calls) = engine();
        let other = source(&mut e, &[]);
        let this = e.mem.alloc(0x20);
        e.call(0x00e4_e420, &args![this, other]);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x8000_0000);
    }
}
