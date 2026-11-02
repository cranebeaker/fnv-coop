//! `Havok SDK/hkpconstraintatomutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions work on an `hkpSimpleContactConstraintAtom` (Xbox PDB, 0x30 bytes, type
//! `0x16`): `m_sizeOfAllAtoms` (+2), `m_numContactPoints` (+4), `m_numReservedContactPoints`
//! (+6), `m_numUserDatasForBodyA` (+8), `m_numUserDatasForBodyB` (+9),
//! `m_contactPointPropertiesStriding` (+0xa), `m_maxNumContactPoints` (+0xc) and
//! `m_info` (+0x10, 0x20 bytes). After the atom come the reserved contact points (0x20 bytes
//! each) and then, per reserved point, a block of `striding` bytes of contact point properties.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator; slot `4` allocates a
/// block of a given size, slot `8` frees a block given its pointer and size.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// The allocator behind the memory router.
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00d92440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `atomSizeRoundUp` (Xbox PDB), cdecl: the allocation size for an atom of `size` bytes, the
/// first of 0x10, 0x20, 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280,
/// 0x400, 0x800, 0x1000 and 0x2000 that is not smaller (signed compares); larger sizes stay
/// as they are.
pub fn atom_size_round_up(_e: &mut Engine, size: i32) -> i32 {
    for limit in [
        0x10, 0x20, 0x30, 0x40, 0x60, 0x80, 0xa0, 0xc0, 0x100, 0x140, 0x200, 0x280, 0x400, 0x800,
        0x1000, 0x2000,
    ] {
        if size <= limit {
            return limit;
        }
    }
    size
}

// Translated from 00d92510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintAtomUtil::allocateAtom` (Xbox PDB), cdecl, four words: the
/// number of contact points to reserve, the two user-data counts and the maximum number of
/// points. Allocates `atomSizeRoundUp(((a + b) * 4 + 0x34) * count + 0x3f & ~0xf)` bytes
/// through the allocator's slot 4 and fills the atom header: type `0x16`, size (+2),
/// zero points (+4), reserved count (+6), user-data counts (+8, +9), striding
/// `(a + b) * 4 + 0x14` (+0xa), maximum (+0xc) and an `m_info` of zeros with the word `3` at
/// +0x12. Returns the atom.
pub fn hkp_simple_contact_constraint_atom_util_allocate_atom(
    e: &mut Engine,
    count: u32,
    user_datas_a: u32,
    user_datas_b: u32,
    max_points: u32,
) -> Ptr {
    let striding = user_datas_a
        .wrapping_add(user_datas_b)
        .wrapping_mul(4)
        .wrapping_add(0x14);
    let size = striding
        .wrapping_add(0x20)
        .wrapping_mul(count)
        .wrapping_add(0x3f)
        & 0xffff_fff0;
    let allocator = memory_allocator(e);
    let rounded = atom_size_round_up(e, size as i32);
    let atom = e.vcall(allocator, 4, &args![rounded]).u32();
    e.mem.set_u8(atom + 8, user_datas_a as u8);
    e.mem.set_u16(atom + 0xc, max_points as u16);
    e.mem.set_u8(atom + 9, user_datas_b as u8);
    e.mem.set_u16(atom + 2, size as u16);
    e.mem.set_u16(atom + 6, count as u16);
    e.mem.set_u8(atom + 0xa, striding as u8);
    e.mem.set_u16(atom + 0x12, 3);
    e.mem.set_u16(atom + 0x10, 0);
    for offset in (0x14..0x30).step_by(4) {
        e.mem.set_u32(atom + offset, 0);
    }
    e.mem.set_u16(atom, 0x16);
    e.mem.set_u16(atom + 4, 0);
    Ptr::new(atom)
}

/// A `MOVAPS` copy of 16 bytes (read first, then written).
fn copy_16(e: &mut Engine, from: u32, to: u32) {
    let words: [u32; 4] = std::array::from_fn(|word| e.mem.u32(from + 4 * word as u32));
    for (word, value) in words.into_iter().enumerate() {
        e.mem.set_u32(to + 4 * word as u32, value);
    }
}

// Translated from 00d925b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintAtomUtil::copyContents` (Xbox PDB), cdecl `(dst, src)`: copies
/// the eight words of `m_info` (+0x10) and the point count (+4), then for each of the source's
/// points its 0x20 bytes and its `striding / 4` words of properties. The contact points start
/// at +0x30; the properties start after the reserved points of each atom (`+0x30 + reserved
/// * 0x20`).
pub fn hkp_simple_contact_constraint_atom_util_copy_contents(e: &mut Engine, dst: Ptr, src: Ptr) {
    let (dst, src) = (dst.addr(), src.addr());
    for word in 0..8 {
        let value = e.mem.u32(src + 0x10 + 4 * word);
        e.mem.set_u32(dst + 0x10 + 4 * word, value);
    }
    let count = e.mem.u16(src + 4);
    e.mem.set_u16(dst + 4, count);
    let striding = u32::from(e.mem.u8(src + 0xa));
    let mut dst_point = dst + 0x30;
    let mut dst_properties = dst + 0x30 + (u32::from(e.mem.u16(dst + 6)) << 5);
    let mut src_properties = src + 0x30 + (u32::from(e.mem.u16(src + 6)) << 5);
    let mut src_point = src + 0x30;
    if e.mem.u16(src + 4) != 0 {
        let mut index = 0;
        loop {
            copy_16(e, src_point, dst_point);
            copy_16(e, src_point + 0x10, dst_point + 0x10);
            src_point += 0x20;
            dst_point += 0x20;
            for word in 0..striding >> 2 {
                let value = e.mem.u32(src_properties + 4 * word);
                e.mem.set_u32(dst_properties + 4 * word, value);
            }
            index += 1;
            dst_properties += striding;
            src_properties += striding;
            if index >= u32::from(e.mem.u16(src + 4)) {
                break;
            }
        }
    }
}

// Translated from 00d92690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintAtomUtil::expandOne` (Xbox PDB), cdecl `(atom, replaced)`:
/// makes room for one more contact point. When all reserved points are used (the count is not
/// below the reserved count) it allocates a new atom for `min(2 * max(count, 2), maximum)`
/// points (`allocateAtom`), copies the old contents over (`copyContents`), frees the old atom
/// through the allocator's slot 8 with `atomSizeRoundUp(m_sizeOfAllAtoms)` and stores `1` in
/// the byte `replaced` points to. Then raises the point count, zeroes the new point's
/// properties (`striding / 4` words, found with the signed 16-bit old count) and returns the
/// atom in use.
pub fn hkp_simple_contact_constraint_atom_util_expand_one(
    e: &mut Engine,
    atom: Ptr,
    replaced: Ptr,
) -> Ptr {
    let old = atom.addr();
    let count = u32::from(e.mem.u16(old + 4));
    let mut current = old;
    if count >= u32::from(e.mem.u16(old + 6)) {
        let doubled = count.max(2) * 2;
        let maximum = u32::from(e.mem.u16(old + 0xc));
        let wanted = if doubled < maximum { doubled } else { maximum };
        let user_datas_b = u32::from(e.mem.u8(old + 9));
        let user_datas_a = u32::from(e.mem.u8(old + 8));
        let grown = hkp_simple_contact_constraint_atom_util_allocate_atom(
            e,
            wanted,
            user_datas_a,
            user_datas_b,
            maximum,
        );
        hkp_simple_contact_constraint_atom_util_copy_contents(e, grown, atom);
        let size = i32::from(e.mem.u16(old + 2));
        let rounded = atom_size_round_up(e, size);
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![old, rounded]);
        e.mem.set_u8(replaced.addr(), 1);
        current = grown.addr();
    }
    let striding = u32::from(e.mem.u8(current + 0xa));
    e.mem.set_u16(current + 4, (count as u16).wrapping_add(1));
    let offset = (striding as i32).wrapping_mul(i32::from(count as u16 as i16)) as u32;
    let mut at = (u32::from(e.mem.u16(current + 6)) << 5)
        .wrapping_add(current.wrapping_add(offset))
        .wrapping_add(0x30);
    for _ in 0..striding >> 2 {
        e.mem.set_u32(at, 0);
        at = at.wrapping_add(4);
    }
    Ptr::new(current)
}

// Translated from 00d92740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintAtomUtil::optimizeCapacity` (Xbox PDB), cdecl `(atom, extra,
/// replaced)`: when `(count + extra) * 2` (signed) is at most the reserved count it allocates
/// a new atom for half the reserved points (`allocateAtom` with the old user-data counts and
/// maximum), copies the contents (`copyContents`), frees the old atom through the allocator's
/// slot 8 with `atomSizeRoundUp(m_sizeOfAllAtoms)`, stores `1` in the byte `replaced` points
/// to and returns the new atom; otherwise it returns `atom`.
pub fn hkp_simple_contact_constraint_atom_util_optimize_capacity(
    e: &mut Engine,
    atom: Ptr,
    extra: i32,
    replaced: Ptr,
) -> Ptr {
    let old = atom.addr();
    let reserved = u32::from(e.mem.u16(old + 6));
    let count = i32::from(e.mem.u16(old + 4));
    if count.wrapping_add(extra).wrapping_mul(2) > reserved as i32 {
        return atom;
    }
    let maximum = u32::from(e.mem.u16(old + 0xc));
    let user_datas_b = u32::from(e.mem.u8(old + 9));
    let user_datas_a = u32::from(e.mem.u8(old + 8));
    let shrunk = hkp_simple_contact_constraint_atom_util_allocate_atom(
        e,
        reserved >> 1,
        user_datas_a,
        user_datas_b,
        maximum,
    );
    hkp_simple_contact_constraint_atom_util_copy_contents(e, shrunk, atom);
    let size = i32::from(e.mem.u16(old + 2));
    let rounded = atom_size_round_up(e, size);
    let allocator = memory_allocator(e);
    e.vcall(allocator, 8, &args![old, rounded]);
    e.mem.set_u8(replaced.addr(), 1);
    shrunk
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_2440, atom_size_round_up(i32) -> i32),
        entry!(0x00d9_2510, hkp_simple_contact_constraint_atom_util_allocate_atom(u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00d9_25b0,
            hkp_simple_contact_constraint_atom_util_copy_contents(Ptr, Ptr)
        ),
        entry!(0x00d9_2690, hkp_simple_contact_constraint_atom_util_expand_one(Ptr, Ptr) -> Ptr),
        entry!(0x00d9_2740, hkp_simple_contact_constraint_atom_util_optimize_capacity(Ptr, i32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_SLOT: u32 = 0x0900_0001;
    const FREE_SLOT: u32 = 0x0900_0002;

    type Allocations = Rc<RefCell<Vec<u32>>>;

    /// A memory router whose allocator hands out blocks pre-filled with `0xcc` (logging the
    /// requested sizes) and logs the `(pointer, size)` pairs it is asked to free.
    fn install_allocator(e: &mut Engine) -> (Allocations, Allocations) {
        let allocated: Allocations = Rc::new(RefCell::new(Vec::new()));
        let freed: Allocations = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::clone(&allocated);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            sizes.borrow_mut().push(a[1]);
            let block = e.mem.alloc(a[1]);
            for offset in 0..a[1] {
                e.mem.set_u8(block + offset, 0xcc);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&freed);
        e.register_double(FREE_SLOT, move |_, a| {
            log.borrow_mut().extend([a[1], a[2]]);
            Ret::default()
        });
        (allocated, freed)
    }

    #[test]
    fn sizes_round_up_to_the_allocator_steps() {
        let mut e = Engine::new();
        for (size, expected) in [
            (-5, 0x10),
            (0, 0x10),
            (0x10, 0x10),
            (0x11, 0x20),
            (0x31, 0x40),
            (0x41, 0x60),
            (0x61, 0x80),
            (0xc1, 0x100),
            (0x141, 0x200),
            (0x281, 0x400),
            (0x1001, 0x2000),
            (0x2000, 0x2000),
            (0x2001, 0x2001),
        ] {
            assert_eq!(e.call(0x00d9_2440, &args![size]).i32(), expected);
        }
    }

    #[test]
    fn allocate_atom_fills_the_header() {
        let mut e = Engine::new();
        let (allocated, _) = install_allocator(&mut e);
        let atom = e.call(0x00d9_2510, &args![3u32, 1u32, 2u32, 9u32]).u32();
        // striding (1 + 2) * 4 + 0x14 = 0x20; size (0x20 + 0x20) * 3 + 0x3f -> 0xf0, rounded 0x100.
        assert_eq!(*allocated.borrow(), [0x100]);
        assert_eq!(e.mem.u16(atom), 0x16);
        assert_eq!(e.mem.u16(atom + 2), 0xf0);
        assert_eq!(e.mem.u16(atom + 4), 0);
        assert_eq!(e.mem.u16(atom + 6), 3);
        assert_eq!(e.mem.u8(atom + 8), 1);
        assert_eq!(e.mem.u8(atom + 9), 2);
        assert_eq!(e.mem.u8(atom + 0xa), 0x20);
        assert_eq!(e.mem.u16(atom + 0xc), 9);
        assert_eq!(e.mem.u16(atom + 0x10), 0);
        assert_eq!(e.mem.u16(atom + 0x12), 3);
        for offset in (0x14..0x30).step_by(4) {
            assert_eq!(e.mem.u32(atom + offset), 0);
        }
        // Past the header the block is left as the allocator gave it.
        assert_eq!(e.mem.u8(atom + 0x30), 0xcc);
    }

    /// A contact atom with `reserved` points of which `count` are in use, one user-data word
    /// each side (striding 0x1c); point `i` holds the words `0x100 * i + n` and its properties
    /// the words `0x1000 * i + n`.
    fn make_atom(e: &mut Engine, reserved: u16, count: u16) -> u32 {
        let striding = 0x1cu32;
        let atom = e
            .mem
            .alloc(0x30 + 0x20 * u32::from(reserved) + striding * u32::from(reserved));
        e.mem.set_u16(atom, 0x16);
        e.mem.set_u16(atom + 2, 0x200);
        e.mem.set_u16(atom + 4, count);
        e.mem.set_u16(atom + 6, reserved);
        e.mem.set_u8(atom + 8, 1);
        e.mem.set_u8(atom + 9, 1);
        e.mem.set_u8(atom + 0xa, striding as u8);
        e.mem.set_u16(atom + 0xc, 8);
        for word in 0..8 {
            e.mem.set_u32(atom + 0x10 + 4 * word, 0x10 + word);
        }
        let properties = atom + 0x30 + 0x20 * u32::from(reserved);
        for index in 0..u32::from(count) {
            for word in 0..8 {
                e.mem
                    .set_u32(atom + 0x30 + 0x20 * index + 4 * word, 0x100 * index + word);
            }
            for word in 0..striding / 4 {
                e.mem.set_u32(
                    properties + striding * index + 4 * word,
                    0x1000 * index + word,
                );
            }
        }
        atom
    }

    fn assert_contents(e: &Engine, atom: u32, reserved: u16, count: u16, checked: u16) {
        let striding = 0x1cu32;
        assert_eq!(e.mem.u16(atom + 4), count);
        for word in 0..8 {
            assert_eq!(e.mem.u32(atom + 0x10 + 4 * word), 0x10 + word);
        }
        let properties = atom + 0x30 + 0x20 * u32::from(reserved);
        for index in 0..u32::from(checked) {
            for word in 0..8 {
                assert_eq!(
                    e.mem.u32(atom + 0x30 + 0x20 * index + 4 * word),
                    0x100 * index + word
                );
            }
            for word in 0..striding / 4 {
                assert_eq!(
                    e.mem.u32(properties + striding * index + 4 * word),
                    0x1000 * index + word
                );
            }
        }
    }

    #[test]
    fn copy_contents_moves_points_and_properties_between_different_capacities() {
        let mut e = Engine::new();
        let source = make_atom(&mut e, 4, 3);
        let target = make_atom(&mut e, 6, 0);
        for word in 0..8 {
            e.mem.set_u32(target + 0x10 + 4 * word, 0);
        }
        e.call(0x00d9_25b0, &args![target, source]);
        assert_contents(&e, target, 6, 3, 3);
        // An atom without points only gets its info and count.
        let empty = make_atom(&mut e, 2, 0);
        let other = make_atom(&mut e, 2, 1);
        e.call(0x00d9_25b0, &args![other, empty]);
        assert_eq!(e.mem.u16(other + 4), 0);
        assert_eq!(e.mem.u32(other + 0x34), 1);
    }

    #[test]
    fn expand_one_zeroes_the_new_properties_when_there_is_room() {
        let mut e = Engine::new();
        let (allocated, freed) = install_allocator(&mut e);
        let atom = make_atom(&mut e, 4, 2);
        let flag = e.mem.alloc(4);
        let properties = atom + 0x30 + 0x20 * 4;
        for word in 0..7 {
            e.mem.set_u32(properties + 0x1c * 2 + 4 * word, 0xdead_beef);
        }
        let got = e.call(0x00d9_2690, &args![atom, flag]).u32();
        assert_eq!(got, atom);
        assert_eq!(e.mem.u16(atom + 4), 3);
        for word in 0..7 {
            assert_eq!(e.mem.u32(properties + 0x1c * 2 + 4 * word), 0);
        }
        assert_eq!(e.mem.u8(flag), 0);
        assert!(allocated.borrow().is_empty() && freed.borrow().is_empty());
    }

    #[test]
    fn expand_one_replaces_a_full_atom_by_a_larger_one() {
        let mut e = Engine::new();
        let (allocated, freed) = install_allocator(&mut e);
        let atom = make_atom(&mut e, 2, 2);
        let flag = e.mem.alloc(4);
        let got = e.call(0x00d9_2690, &args![atom, flag]).u32();
        assert_ne!(got, atom);
        // New reserved count min(2 * max(2, 2), 8) = 4: striding 0x1c, size (0x1c + 0x20) * 4 + 0x3f
        // -> 0x120, rounded to 0x140; the old size 0x200 frees as 0x200.
        assert_eq!(*allocated.borrow(), [0x140]);
        assert_eq!(*freed.borrow(), [atom, 0x200]);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u16(got + 6), 4);
        assert_contents(&e, got, 4, 3, 2);
        let properties = got + 0x30 + 0x20 * 4;
        for word in 0..7 {
            assert_eq!(e.mem.u32(properties + 0x1c * 2 + 4 * word), 0);
        }
    }

    #[test]
    fn expand_one_stops_at_the_maximum() {
        let mut e = Engine::new();
        install_allocator(&mut e);
        let atom = make_atom(&mut e, 6, 6);
        e.mem.set_u16(atom + 0xc, 7);
        let flag = e.mem.alloc(4);
        let got = e.call(0x00d9_2690, &args![atom, flag]).u32();
        assert_eq!(e.mem.u16(got + 6), 7);
    }

    #[test]
    fn optimize_capacity_halves_the_reservation_only_when_the_count_allows() {
        let mut e = Engine::new();
        let (allocated, freed) = install_allocator(&mut e);
        let atom = make_atom(&mut e, 8, 3);
        let flag = e.mem.alloc(4);
        // (3 + 2) * 2 = 10 > 8: unchanged.
        assert_eq!(e.call(0x00d9_2740, &args![atom, 2i32, flag]).u32(), atom);
        assert_eq!(e.mem.u8(flag), 0);
        assert!(allocated.borrow().is_empty());
        // (3 + 1) * 2 = 8 <= 8: a new atom reserving 4 points.
        let got = e.call(0x00d9_2740, &args![atom, 1i32, flag]).u32();
        assert_ne!(got, atom);
        assert_eq!(e.mem.u16(got + 6), 4);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(*freed.borrow(), [atom, 0x200]);
        assert_contents(&e, got, 4, 3, 3);
        // A negative extra count counts too: (1 - 5) * 2 = -8 <= 4.
        e.mem.set_u16(got + 4, 1);
        let again = e.call(0x00d9_2740, &args![got, -5i32, flag]).u32();
        assert_ne!(again, got);
        assert_eq!(e.mem.u16(again + 6), 2);
    }
}
