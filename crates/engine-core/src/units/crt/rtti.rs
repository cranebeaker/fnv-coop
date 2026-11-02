//! The C++ runtime's `dynamic_cast` (MSVC 2008 LIBCMT/rtti.cpp, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: `__RTDynamicCast` (`00ec43fb`) and its
//! helpers `FindCompleteObject` (`00ec40ec`), `FindSITargetTypeInstance` (`00ec4102`),
//! `PMDtoOffset` (`00ec4179`), `FindMITargetTypeInstance` (`00ec419d`) and
//! `FindVITargetTypeInstance` (`00ec42a5`), with the `std::bad_cast` constructor it throws
//! (`00ec79c4`, libcpmt/xthrow.cpp in the engine map).
//!
//! `__RTDynamicCast` (`00ec43fb`) replaces the platform stand-in `crt.rs` had for it (taken
//! out, with its test).
//!
//! The helpers take some inputs in registers: in the uniform form the register inputs come
//! first (`EAX`, then `ECX`), then the stack words. The records are the game's own RTTI in
//! mapped memory: the complete object locator (`+4` the subobject's offset, `+8` the
//! constructor displacement offset, `+0x10` the class hierarchy descriptor), the class
//! hierarchy descriptor (`+4` its attributes: bit 0 multiple, bit 1 virtual inheritance; `+8`
//! the base count, `+0xc` the base class array), and each base class descriptor (`+0` its type
//! descriptor, `+4` the bases it contains, `+8` the `PMD`: `mdisp`, `pdisp`, `vdisp`; `+0x14`
//! its attributes; `+0x18` its own class hierarchy descriptor). Type descriptors are the same
//! when their addresses are equal or their decorated names (`+8`) compare equal (`strcmp`).
//! Every call is made by address, as the exe makes it, so a test can double each one.

use super::*;

/// `strcmp` (`00ec6da0`, cdecl), `std::exception::exception(char const * const &)`
/// (`00ec78f1`, thiscall, one stack word: the address of the message pointer) and
/// `_CxxThrowException` (`00ec7894`, stdcall: the object and its throw information).
const STRCMP: u32 = 0x00ec_6da0;
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The helpers, called by address.
const FIND_COMPLETE_OBJECT: u32 = 0x00ec_40ec;
const FIND_SI_TARGET: u32 = 0x00ec_4102;
const PMD_TO_OFFSET: u32 = 0x00ec_4179;
const FIND_MI_TARGET: u32 = 0x00ec_419d;
const FIND_VI_TARGET: u32 = 0x00ec_42a5;
const BAD_CAST_FROM_MESSAGE: u32 = 0x00ec_79c4;
/// `std::bad_cast`'s vtable (`010f1fc4`; its complete object locator names
/// `.?AVbad_cast@std@@`), the message `"Bad dynamic_cast!"` (`010f1f08`) and the throw
/// information `__RTDynamicCast` passes (`0116cdb8`).
const BAD_CAST_VTABLE: u32 = 0x010f_1fc4;
const BAD_CAST_MESSAGE: u32 = 0x010f_1f08;
const BAD_CAST_THROW_INFO: u32 = 0x0116_cdb8;

/// Whether the type descriptors at `a` and `b` are the same type, as the exe's `CMP EAX,ECX;
/// JZ` then `strcmp(a + 8, b + 8)` decide (a zero answer is a match).
fn same_type(e: &mut Engine, a: u32, b: u32) -> bool {
    a == b
        || e.call(STRCMP, &args![a.wrapping_add(8), b.wrapping_add(8)])
            .u32()
            == 0
}

// Translated from 00ec40ec (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindCompleteObject` (LIBCMT/rtti.cpp; unnamed in the engine map, named from its body and
/// its one caller), the object in `ECX`, the answer in `EAX`: the object's complete object
/// locator (the word before its vtable) gives the complete object, the object minus the
/// locator's offset (`+4`); with a constructor displacement offset (`+8`, nonzero) the word at
/// the object minus it is taken off too.
pub fn find_complete_object(e: &mut Engine, object: u32) -> u32 {
    let vtable = e.mem.u32(object);
    let locator = e.mem.u32(vtable.wrapping_sub(4));
    let mut complete = object.wrapping_sub(e.mem.u32(locator.wrapping_add(4)));
    let displacement = e.mem.u32(locator.wrapping_add(8));
    if displacement != 0 {
        let at = object.wrapping_sub(displacement);
        complete = complete.wrapping_sub(e.mem.u32(at));
    }
    complete
}

// Translated from 00ec4102 (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindSITargetTypeInstance` (Ghidra's function ID), single inheritance: the complete object
/// locator in `EAX`, then (source type, target type) on the stack. The base class array is
/// searched for the target type; from the entry after it on, for the source type, which must
/// come before any entry whose attributes have bit 2 set (that answers null). Answers the
/// target's base class descriptor when the source follows it, else null.
pub fn find_si_target_type_instance(e: &mut Engine, locator: u32, source: u32, target: u32) -> u32 {
    let hierarchy = e.mem.u32(locator.wrapping_add(0x10));
    let count = e.mem.u32(hierarchy.wrapping_add(8));
    let bases = e.mem.u32(hierarchy.wrapping_add(0xc));
    let mut i = 0u32;
    if count == 0 {
        return 0;
    }
    // 00ec411a: the target.
    let found = loop {
        let base = e.mem.u32(bases.wrapping_add(4 * i));
        let td = e.mem.u32(base);
        if same_type(e, td, target) {
            break base;
        }
        i = i.wrapping_add(1);
        if i >= count {
            return 0;
        }
    };
    // 00ec416d: INC ESI; CMP ESI,EBX; JC 00ec4148.
    loop {
        i = i.wrapping_add(1);
        if i >= count {
            return 0;
        }
        let base = e.mem.u32(bases.wrapping_add(4 * i));
        if e.mem.u8(base.wrapping_add(0x14)) & 4 != 0 {
            return 0;
        }
        let td = e.mem.u32(base);
        if same_type(e, td, source) {
            return found;
        }
    }
}

// Translated from 00ec4179 (disassembled, FalloutNV.exe 1.4.0.525)
/// `PMDtoOffset` (Ghidra's function ID), the `PMD` in `EAX`, then the complete object on the
/// stack: `mdisp`, plus, when `pdisp` (`+4`) is not negative (a signed test), `pdisp` and the
/// word at `vdisp` (`+8`) in the virtual base table the object holds at `pdisp`.
pub fn pmd_to_offset(e: &mut Engine, pmd: u32, object: u32) -> u32 {
    let pdisp = e.mem.u32(pmd.wrapping_add(4));
    let mut offset = 0u32;
    if (pdisp as i32) >= 0 {
        offset = pdisp;
        let table = e.mem.u32(offset.wrapping_add(object));
        let vdisp = e.mem.u32(pmd.wrapping_add(8));
        offset = offset.wrapping_add(e.mem.u32(table.wrapping_add(vdisp)));
    }
    e.mem.u32(pmd).wrapping_add(offset)
}

/// `PMDtoOffset` called by address, as the exe calls it.
fn call_pmd_to_offset(e: &mut Engine, base: u32, object: u32) -> u32 {
    e.call(PMD_TO_OFFSET, &args![base.wrapping_add(8), object])
        .u32()
}

// Translated from 00ec419d (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindMITargetTypeInstance` (Ghidra's function ID), multiple inheritance: the complete object
/// locator in `EAX`, then (complete object, source type, the source subobject's offset, target
/// type) on the stack. One walk of the base class array keeps the target found (its index, its
/// descriptor and the bases it contains) and the source found at the given offset
/// (`PMDtoOffset`); the target is only looked for at an index more than its contained bases
/// past the last one found (unsigned). Finding the target after the source: null when the
/// target's attributes have bit 0 or 1 set or the source's bit 0, else the target. Finding the
/// source after the target: when the source lies outside the target's bases, null when the
/// target's attributes have bit 0 or 1 set; inside them, a target with its own hierarchy
/// (attribute 0x40) answers null when the source's entry in that hierarchy (at the index
/// difference) has attribute bit 0 set, else the target; otherwise a target at index 0 with a
/// source whose attribute bit 0 is set answers null, else the target. Null when the walk ends.
pub fn find_mi_target_type_instance(
    e: &mut Engine,
    locator: u32,
    complete: u32,
    source: u32,
    offset: u32,
    target: u32,
) -> u32 {
    let hierarchy = e.mem.u32(locator.wrapping_add(0x10));
    let count = e.mem.u32(hierarchy.wrapping_add(8));
    let bases = e.mem.u32(hierarchy.wrapping_add(0xc));
    let mut target_index = 0xffff_ffffu32;
    let mut target_base = 0u32;
    let mut source_base = 0u32;
    let mut contained = 0u32;
    let mut i = 0u32;
    if count == 0 {
        return 0;
    }
    loop {
        let base = e.mem.u32(bases.wrapping_add(4 * i));
        // SUB EAX,[EBP-8]; CMP EAX,[EBP-0xc]; JBE: unsigned.
        if i.wrapping_sub(target_index) > contained {
            let td = e.mem.u32(base);
            if same_type(e, td, target) {
                if source_base != 0 {
                    // 00ec424c.
                    if e.mem.u8(base.wrapping_add(0x14)) & 3 != 0 {
                        return 0;
                    }
                    if e.mem.u8(source_base.wrapping_add(0x14)) & 1 != 0 {
                        return 0;
                    }
                    return base;
                }
                target_index = i;
                target_base = base;
                contained = e.mem.u32(base.wrapping_add(4));
            }
        }
        // 00ec420b: the source.
        let td = e.mem.u32(base);
        if same_type(e, td, source) && call_pmd_to_offset(e, base, complete) == offset {
            if target_base != 0 {
                // 00ec425c.
                let difference = i.wrapping_sub(target_index);
                if difference > contained {
                    // 00ec4298.
                    if e.mem.u8(target_base.wrapping_add(0x14)) & 3 != 0 {
                        return 0;
                    }
                    return source_public(e, base, target_base);
                }
                if e.mem.u8(target_base.wrapping_add(0x14)) & 0x40 != 0 {
                    // 00ec427d: the source's entry in the target's own hierarchy.
                    let own = e.mem.u32(target_base.wrapping_add(0x18));
                    let own_bases = e.mem.u32(own.wrapping_add(0xc));
                    let entry = e
                        .mem
                        .u32(own_bases.wrapping_add(difference.wrapping_mul(4)));
                    return if e.mem.u8(entry.wrapping_add(0x14)) & 1 != 0 {
                        0
                    } else {
                        target_base
                    };
                }
                if target_index != 0 {
                    return target_base;
                }
                return source_public(e, base, target_base);
            }
            source_base = base;
        }
        i = i.wrapping_add(1);
        if i >= count {
            return 0;
        }
    }
}

/// `FindMITargetTypeInstance`'s `00ec4273`: null when the source's attributes (`+0x14`) have
/// bit 0 set, else the target.
fn source_public(e: &Engine, source_base: u32, target_base: u32) -> u32 {
    if e.mem.u8(source_base.wrapping_add(0x14)) & 1 != 0 {
        0
    } else {
        target_base
    }
}

// Translated from 00ec42a5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `FindVITargetTypeInstance` (Ghidra's function ID), virtual inheritance: the complete object
/// locator in `EAX`, then (complete object, source type, the source subobject's offset, target
/// type) on the stack. One walk of the base class array: a target (looked for, as in the
/// multiple-inheritance search, more than its contained bases past the last one found) becomes
/// the current target, and also the public target when its attributes have neither bit 0 nor 1.
/// A source at the given offset outside the current target's bases becomes the public source
/// when its attributes have neither bit 0 nor 2. Inside them, while the cast is still
/// unambiguous: through a target with its own hierarchy (attribute 0x40) the source's entry
/// there decides (bit 0 set makes the cast ambiguous; bit 2 clear lets it through), otherwise
/// a target at index 0 with a source whose bit 0 is set makes it ambiguous and the source goes
/// through; a source let through records the target with its offset (`PMDtoOffset`), and a
/// second target at another offset answers null. At the end: the recorded target while the
/// cast is unambiguous; else, with a public source, the public target; else null.
pub fn find_vi_target_type_instance(
    e: &mut Engine,
    locator: u32,
    complete: u32,
    source: u32,
    offset: u32,
    target: u32,
) -> u32 {
    let hierarchy = e.mem.u32(locator.wrapping_add(0x10));
    let bases = e.mem.u32(hierarchy.wrapping_add(0xc));
    let count = e.mem.u32(hierarchy.wrapping_add(8));
    let mut target_index = 0xffff_ffffu32;
    let mut result_offset = 0xffff_ffffu32;
    let mut target_base = 0u32;
    let mut result = 0u32;
    let mut public_source = 0u32;
    let mut public_target = 0u32;
    let mut contained = 0u32;
    let mut unambiguous = true;
    let mut i = 0u32;
    if count == 0 {
        return 0;
    }
    loop {
        let base = e.mem.u32(bases.wrapping_add(4 * i));
        if i.wrapping_sub(target_index) > contained {
            let td = e.mem.u32(base);
            if same_type(e, td, target) {
                if e.mem.u8(base.wrapping_add(0x14)) & 3 == 0 {
                    public_target = base;
                }
                target_index = i;
                contained = e.mem.u32(base.wrapping_add(4));
                target_base = base;
            }
        }
        // 00ec4325: the source.
        let td = e.mem.u32(base);
        if same_type(e, td, source) && call_pmd_to_offset(e, base, complete) == offset {
            let difference = i.wrapping_sub(target_index);
            if difference > contained {
                // 00ec43c5.
                if e.mem.u8(base.wrapping_add(0x14)) & 5 == 0 {
                    public_source = base;
                }
            } else if unambiguous {
                let through = if e.mem.u8(target_base.wrapping_add(0x14)) & 0x40 != 0 {
                    // 00ec4381: the dword attributes of the source's entry in the target's
                    // own hierarchy; bit 0 makes the cast ambiguous, AL = !(attributes >> 2)
                    // & 1.
                    let own = e.mem.u32(target_base.wrapping_add(0x18));
                    let own_bases = e.mem.u32(own.wrapping_add(0xc));
                    let entry = e
                        .mem
                        .u32(own_bases.wrapping_add(difference.wrapping_mul(4)));
                    let attributes = e.mem.u32(entry.wrapping_add(0x14));
                    if attributes & 1 != 0 {
                        unambiguous = false;
                    }
                    (attributes >> 2) & 1 == 0
                } else {
                    if target_index == 0 && e.mem.u8(base.wrapping_add(0x14)) & 1 != 0 {
                        unambiguous = false;
                    }
                    true
                };
                // 00ec439c.
                if unambiguous && through {
                    let at = call_pmd_to_offset(e, target_base, complete);
                    if result != 0 && result_offset != at {
                        return 0;
                    }
                    result = target_base;
                    result_offset = at;
                }
            }
        }
        i = i.wrapping_add(1);
        if i >= count {
            break;
        }
    }
    if unambiguous && result != 0 {
        return result;
    }
    if public_source == 0 {
        return 0;
    }
    public_target
}

// Translated from 00ec43fb (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec43fb - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fc68) and its __except block are not translated: the filter 00ec44a1 takes an access violation (0xc0000005) raised while reading the RTTI records and the handler 00ec44b5 throws std::__non_rtti_object ("Access violation - no RTTI data!", constructor 00ec7a45, throw information 0117fc10); the engine raises no access violations.
/// `__RTDynamicCast` (Ghidra's function ID), cdecl: (object, the vftable's displacement in it,
/// source type, target type, whether the cast is to a reference). A null object answers null.
/// Otherwise the complete object (`FindCompleteObject`) and the source subobject's offset in
/// it (the object minus the displacement minus the complete object) pick the search by the
/// class hierarchy's attributes (`+4` of the descriptor the complete object locator names:
/// bit 0 clear single, bit 1 clear multiple, else virtual inheritance). A target found gives
/// the complete object plus its offset (`PMDtoOffset`). None gives null for a pointer cast; for
/// a reference cast a `std::bad_cast("Bad dynamic_cast!")` is made on the stack and thrown
/// (`_CxxThrowException` with `0116cdb8`), which does not return.
pub fn rt_dynamic_cast(
    e: &mut Engine,
    object: u32,
    vf_delta: u32,
    source: u32,
    target: u32,
    is_reference: u32,
) -> u32 {
    if object == 0 {
        return 0;
    }
    let complete = e.call(FIND_COMPLETE_OBJECT, &args![object]).u32();
    let vtable = e.mem.u32(object);
    let locator = e.mem.u32(vtable.wrapping_sub(4));
    let offset = object.wrapping_sub(vf_delta).wrapping_sub(complete);
    let hierarchy = e.mem.u32(locator.wrapping_add(0x10));
    let attributes = e.mem.u32(hierarchy.wrapping_add(4));
    let found = if attributes & 1 == 0 {
        e.call(FIND_SI_TARGET, &args![locator, source, target])
            .u32()
    } else if attributes & 2 == 0 {
        e.call(
            FIND_MI_TARGET,
            &args![locator, complete, source, offset, target],
        )
        .u32()
    } else {
        e.call(
            FIND_VI_TARGET,
            &args![locator, complete, source, offset, target],
        )
        .u32()
    };
    if found != 0 {
        return call_pmd_to_offset(e, found, complete).wrapping_add(complete);
    }
    if is_reference == 0 {
        return 0;
    }
    // 00ec4486: the std::bad_cast at EBP-0x28 (0xc bytes) and the throw.
    e.with_stack(0xc, |e, exception| {
        e.call(BAD_CAST_FROM_MESSAGE, &args![exception, BAD_CAST_MESSAGE]);
        e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_CAST_THROW_INFO]);
    });
    panic!("__RTDynamicCast: _CxxThrowException returned (std::bad_cast from {object:08x})")
}

// Translated from 00ec79c4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_cast::bad_cast(char const *)` (libcpmt/xthrow.cpp in the engine map; unnamed,
/// named from the vtable it stores, whose RTTI names `.?AVbad_cast@std@@`), thiscall, one stack
/// word (`RET 4`): the message. The `std::exception` constructor (`00ec78f1`) is given the
/// address of the message word, then the `std::bad_cast` vtable (`010f1fc4`) is stored. Answers
/// `this`.
pub fn bad_cast_from_message(e: &mut Engine, this: u32, message: u32) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), message);
        e.call(EXCEPTION_FROM_MESSAGE, &args![this, word]);
    });
    e.mem.set_u32(this, BAD_CAST_VTABLE);
    this
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_40ec, find_complete_object(u32) -> u32),
        entry!(0x00ec_4102, find_si_target_type_instance(u32, u32, u32) -> u32),
        entry!(0x00ec_4179, pmd_to_offset(u32, u32) -> u32),
        entry!(
            0x00ec_419d,
            find_mi_target_type_instance(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00ec_42a5,
            find_vi_target_type_instance(u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ec_43fb, rt_dynamic_cast(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ec_79c4, bad_cast_from_message(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with `strcmp` doubled (byte compare of the two strings, -1/0/1) and the call
    /// log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRCMP, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (x.cmp(&y) as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A type descriptor named `name`.
    fn type_descriptor(e: &mut Engine, name: &[u8]) -> u32 {
        let td = e.mem.alloc(0x40);
        e.mem.write(td + 8, name);
        td
    }

    /// A base class descriptor: (type, contained bases, mdisp, pdisp, vdisp, attributes, own
    /// hierarchy).
    fn base(e: &mut Engine, td: u32, contained: u32, pmd: [i32; 3], attributes: u32) -> u32 {
        let b = e.mem.alloc(0x1c);
        e.mem.set_u32(b, td);
        e.mem.set_u32(b + 4, contained);
        e.mem.set_i32(b + 8, pmd[0]);
        e.mem.set_i32(b + 0xc, pmd[1]);
        e.mem.set_i32(b + 0x10, pmd[2]);
        e.mem.set_u32(b + 0x14, attributes);
        b
    }

    /// A class hierarchy descriptor with these attributes and bases.
    fn hierarchy(e: &mut Engine, attributes: u32, bases: &[u32]) -> u32 {
        let array = e.mem.alloc(4 * bases.len() as u32 + 4);
        for (i, b) in bases.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *b);
        }
        let h = e.mem.alloc(0x10);
        e.mem.set_u32(h + 4, attributes);
        e.mem.set_u32(h + 8, bases.len() as u32);
        e.mem.set_u32(h + 0xc, array);
        h
    }

    /// A complete object locator (offset, displacement offset, hierarchy).
    fn locator(e: &mut Engine, offset: u32, displacement: u32, h: u32) -> u32 {
        let l = e.mem.alloc(0x14);
        e.mem.set_u32(l + 4, offset);
        e.mem.set_u32(l + 8, displacement);
        e.mem.set_u32(l + 0x10, h);
        l
    }

    /// An object at `at` whose vtable's preceding word is `locator`.
    fn with_vtable(e: &mut Engine, at: u32, locator: u32) {
        let vtable = e.mem.alloc(0x10) + 4;
        e.mem.set_u32(vtable - 4, locator);
        e.mem.set_u32(at, vtable);
    }

    #[test]
    fn find_complete_object_takes_the_offsets_off() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        let l = locator(&mut e, 0x10, 0, 0);
        with_vtable(&mut e, block + 0x10, l);
        assert_eq!(e.call(0x00ec_40ec, &args![block + 0x10]).u32(), block);
        // With a displacement offset, the word at object - it is taken off as well.
        let l2 = locator(&mut e, 0x10, 4, 0);
        with_vtable(&mut e, block + 0x20, l2);
        e.mem.set_u32(block + 0x1c, 8);
        assert_eq!(e.call(0x00ec_40ec, &args![block + 0x20]).u32(), block + 8);
    }

    #[test]
    fn pmd_to_offset_adds_the_virtual_base_offset_for_a_non_negative_pdisp() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, table);
        e.mem.set_u32(table + 4, 0x20);
        let pmd = e.mem.alloc(0xc);
        e.mem.set_i32(pmd, 4);
        e.mem.set_i32(pmd + 4, -1);
        e.mem.set_i32(pmd + 8, 4);
        assert_eq!(e.call(0x00ec_4179, &args![pmd, object]).u32(), 4);
        e.mem.set_i32(pmd + 4, 8);
        // mdisp 4 + pdisp 8 + the table's word at vdisp 4 (0x20).
        assert_eq!(e.call(0x00ec_4179, &args![pmd, object]).u32(), 0x2c);
    }

    #[test]
    fn find_si_needs_the_source_after_the_target_before_a_bit_2_entry() {
        let mut e = engine();
        let derived = type_descriptor(&mut e, b".?AVDerived@@");
        let middle = type_descriptor(&mut e, b".?AVMiddle@@");
        let root = type_descriptor(&mut e, b".?AVRoot@@");
        let other = type_descriptor(&mut e, b".?AVOther@@");
        let b0 = base(&mut e, derived, 2, [0, -1, 0], 0);
        let b1 = base(&mut e, middle, 1, [0, -1, 0], 0);
        let b2 = base(&mut e, root, 0, [0, -1, 0], 0);
        let h = hierarchy(&mut e, 0, &[b0, b1, b2]);
        let l = locator(&mut e, 0, 0, h);
        // Down from Root to Derived, and to Middle: the target's descriptor.
        assert_eq!(e.call(0x00ec_4102, &args![l, root, derived]).u32(), b0);
        assert_eq!(e.call(0x00ec_4102, &args![l, root, middle]).u32(), b1);
        // A name compare finds a copy of the type descriptor.
        let copy = type_descriptor(&mut e, b".?AVRoot@@");
        assert_eq!(e.call(0x00ec_4102, &args![l, copy, middle]).u32(), b1);
        // The source before the target, a missing target, or a missing source: null.
        assert_eq!(e.call(0x00ec_4102, &args![l, derived, root]).u32(), 0);
        assert_eq!(e.call(0x00ec_4102, &args![l, root, other]).u32(), 0);
        assert_eq!(e.call(0x00ec_4102, &args![l, other, derived]).u32(), 0);
        // An entry with bit 2 between them stops the search.
        e.mem.set_u32(b1 + 0x14, 4);
        assert_eq!(e.call(0x00ec_4102, &args![l, root, derived]).u32(), 0);
        // No bases: null without a compare.
        let empty = hierarchy(&mut e, 0, &[]);
        let l0 = locator(&mut e, 0, 0, empty);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_4102, &args![l0, root, derived]).u32(), 0);
        assert!(calls(&e, STRCMP).is_empty());
    }

    /// `C : A, B` (multiple inheritance): bases C (2 contained), A at 0, B at 8.
    fn mi_hierarchy(e: &mut Engine) -> ([u32; 4], [u32; 3], u32) {
        let c = type_descriptor(e, b".?AVC@@");
        let a = type_descriptor(e, b".?AVA@@");
        let b = type_descriptor(e, b".?AVB@@");
        let x = type_descriptor(e, b".?AVX@@");
        let bc = base(e, c, 2, [0, -1, 0], 0);
        let ba = base(e, a, 0, [0, -1, 0], 0);
        let bb = base(e, b, 0, [8, -1, 0], 0);
        let h = hierarchy(e, 1, &[bc, ba, bb]);
        let l = locator(e, 0, 0, h);
        ([c, a, b, x], [bc, ba, bb], l)
    }

    #[test]
    fn find_mi_finds_down_and_cross_casts_by_the_source_offset() {
        let mut e = engine();
        let ([c, a, b, x], [bc, ba, bb], l) = mi_hierarchy(&mut e);
        let complete = e.mem.alloc(0x10);
        // Down from B (offset 8) to C: target found first, the source inside its bases.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, b, 8u32, c]).u32(),
            bc
        );
        // B at the wrong offset is not the source.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, b, 4u32, c]).u32(),
            0
        );
        // Cross from A (offset 0) to B: the source first, then the target (attributes clear).
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, b]).u32(),
            bb
        );
        // ... null when the target has bit 0 or 1, or the source bit 0.
        e.mem.set_u32(bb + 0x14, 2);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, b]).u32(),
            0
        );
        e.mem.set_u32(bb + 0x14, 0);
        e.mem.set_u32(ba + 0x14, 1);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, b]).u32(),
            0
        );
        // Inside the target's bases at index 0 with a source of bit 0: null; elsewhere the
        // target.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, c]).u32(),
            0
        );
        e.mem.set_u32(ba + 0x14, 0);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, c]).u32(),
            bc
        );
        // A missing target: null after the walk.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, x]).u32(),
            0
        );
    }

    #[test]
    fn find_mi_through_a_target_with_its_own_hierarchy_and_outside_its_bases() {
        let mut e = engine();
        // D : C, E where C : A, B (C has its own hierarchy, attribute 0x40) and E holds an A.
        let d = type_descriptor(&mut e, b".?AVD@@");
        let c = type_descriptor(&mut e, b".?AVC@@");
        let a = type_descriptor(&mut e, b".?AVA@@");
        let b = type_descriptor(&mut e, b".?AVB@@");
        let bd = base(&mut e, d, 4, [0, -1, 0], 0);
        let bc = base(&mut e, c, 2, [0, -1, 0], 0x40);
        let ba = base(&mut e, a, 0, [0, -1, 0], 0);
        let bb = base(&mut e, b, 0, [4, -1, 0], 0);
        let ba2 = base(&mut e, a, 0, [0x10, -1, 0], 0);
        // C's own hierarchy: C, A (attributes 1 at index 1), B.
        let own_a = base(&mut e, a, 0, [0, -1, 0], 1);
        let own = hierarchy(&mut e, 1, &[bc, own_a, bb]);
        e.mem.set_u32(bc + 0x18, own);
        let h = hierarchy(&mut e, 1, &[bd, bc, ba, bb, ba2]);
        let l = locator(&mut e, 0, 0, h);
        let complete = e.mem.alloc(0x20);
        // A at 0 to C: the entry at index 1 of C's hierarchy has bit 0: null.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, c]).u32(),
            0
        );
        e.mem.set_u32(own_a + 0x14, 0);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0u32, c]).u32(),
            bc
        );
        // A at 0x10 (outside C's bases) to C: the target's attributes (0x40 only) let it
        // through, the source's bit 0 decides.
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0x10u32, c])
                .u32(),
            bc
        );
        e.mem.set_u32(ba2 + 0x14, 1);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0x10u32, c])
                .u32(),
            0
        );
        e.mem.set_u32(ba2 + 0x14, 0);
        e.mem.set_u32(bc + 0x14, 0x41);
        assert_eq!(
            e.call(0x00ec_419d, &args![l, complete, a, 0x10u32, c])
                .u32(),
            0
        );
    }

    #[test]
    fn find_vi_records_one_target_and_falls_back_to_the_public_pair() {
        let mut e = engine();
        // V : B (virtual base A inside B), A elsewhere at another offset.
        let v = type_descriptor(&mut e, b".?AVV@@");
        let b = type_descriptor(&mut e, b".?AVB@@");
        let a = type_descriptor(&mut e, b".?AVA@@");
        let x = type_descriptor(&mut e, b".?AVX@@");
        let bv = base(&mut e, v, 2, [0, -1, 0], 0);
        let bb = base(&mut e, b, 1, [4, -1, 0], 0);
        let ba = base(&mut e, a, 0, [8, -1, 0], 0);
        let h = hierarchy(&mut e, 3, &[bv, bb, ba]);
        let l = locator(&mut e, 0, 0, h);
        let complete = e.mem.alloc(0x20);
        // A (offset 8) to V and to B: inside the target's bases, recorded.
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, v]).u32(),
            bv
        );
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, b]).u32(),
            bb
        );
        // The source not at the offset, or a missing target: null.
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 4u32, v]).u32(),
            0
        );
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, x]).u32(),
            0
        );
        // Target V at index 0 with a source of bit 0: ambiguous; no public source: null.
        e.mem.set_u32(ba + 0x14, 1);
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, v]).u32(),
            0
        );
        // B (offset 4) to A: the source outside A's (later) bases is the public source, A the
        // public target.
        e.mem.set_u32(ba + 0x14, 0);
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, b, 4u32, a]).u32(),
            ba
        );
        // ... none when the source has bit 2 (not public).
        e.mem.set_u32(bb + 0x14, 4);
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, b, 4u32, a]).u32(),
            0
        );
    }

    #[test]
    fn find_vi_through_an_own_hierarchy_and_two_offsets() {
        let mut e = engine();
        let v = type_descriptor(&mut e, b".?AVV@@");
        let b = type_descriptor(&mut e, b".?AVB@@");
        let a = type_descriptor(&mut e, b".?AVA@@");
        // V, B (own hierarchy, 1 contained), A at 8, B again (1 contained), A at 8.
        let bv = base(&mut e, v, 4, [0, -1, 0], 0);
        let bb = base(&mut e, b, 1, [4, -1, 0], 0x40);
        let ba = base(&mut e, a, 0, [8, -1, 0], 0);
        let bb2 = base(&mut e, b, 1, [0xc, -1, 0], 0);
        let ba2 = base(&mut e, a, 0, [8, -1, 0], 0);
        let own_a = base(&mut e, a, 0, [0, -1, 0], 0);
        let own = hierarchy(&mut e, 3, &[bb, own_a]);
        e.mem.set_u32(bb + 0x18, own);
        let h = hierarchy(&mut e, 3, &[bv, bb, ba, bb2, ba2]);
        let l = locator(&mut e, 0, 0, h);
        let complete = e.mem.alloc(0x20);
        // A to B: the first B (offset 4) is recorded through its own hierarchy, the second B
        // at offset 0xc differs: null.
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, b]).u32(),
            0
        );
        // With bit 2 on A's entry in B's hierarchy the first is not let through; the second
        // is recorded.
        e.mem.set_u32(own_a + 0x14, 4);
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, b]).u32(),
            bb2
        );
        // Bit 0 there makes the cast ambiguous: no result, no public source: null.
        e.mem.set_u32(own_a + 0x14, 1);
        assert_eq!(
            e.call(0x00ec_42a5, &args![l, complete, a, 8u32, b]).u32(),
            0
        );
    }

    #[test]
    fn rt_dynamic_cast_picks_the_search_by_the_hierarchy_attributes() {
        let mut e = engine();
        let derived = type_descriptor(&mut e, b".?AVDerived@@");
        let root = type_descriptor(&mut e, b".?AVRoot@@");
        let other = type_descriptor(&mut e, b".?AVOther@@");
        let b0 = base(&mut e, derived, 1, [0, -1, 0], 0);
        let b1 = base(&mut e, root, 0, [4, -1, 0], 0);
        let h = hierarchy(&mut e, 0, &[b0, b1]);
        // The object is the Root subobject at +4 of a Derived.
        let complete = e.mem.alloc(0x10);
        let l = locator(&mut e, 4, 0, h);
        with_vtable(&mut e, complete + 4, l);
        assert_eq!(
            e.call(0x00ec_43fb, &args![0u32, 0u32, root, derived, 0u32])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x00ec_43fb, &args![complete + 4, 0u32, root, derived, 0u32])
                .u32(),
            complete
        );
        assert_eq!(calls(&e, 0x00ec_4102), vec![vec![l, root, derived]]);
        assert_eq!(
            e.call(0x00ec_43fb, &args![complete + 4, 0u32, root, other, 0u32])
                .u32(),
            0
        );
        // Multiple and virtual inheritance go to their searches with (complete, source, the
        // offset of the object less the displacement, target).
        for (attributes, search) in [(1u32, FIND_MI_TARGET), (3, FIND_VI_TARGET)] {
            e.mem.set_u32(h + 4, attributes);
            e.register_double(search, move |_, _| b1.into_ret());
            e.call_log = Some(vec![]);
            let r = e
                .call(0x00ec_43fb, &args![complete + 4, 2u32, root, derived, 0u32])
                .u32();
            assert_eq!(r, complete + 4);
            assert_eq!(
                calls(&e, search),
                vec![vec![l, complete, root, 2u32, derived]]
            );
        }
    }

    #[test]
    fn rt_dynamic_cast_throws_bad_cast_for_a_reference() {
        let mut e = engine();
        let root = type_descriptor(&mut e, b".?AVRoot@@");
        let other = type_descriptor(&mut e, b".?AVOther@@");
        let b0 = base(&mut e, root, 0, [0, -1, 0], 0);
        let h = hierarchy(&mut e, 0, &[b0]);
        let object = e.mem.alloc(0x10);
        let l = locator(&mut e, 0, 0, h);
        with_vtable(&mut e, object, l);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register(EXCEPTION_FROM_MESSAGE, |e, a| {
            let message = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, message);
            a[0].into_ret()
        });
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            log.borrow_mut()
                .push((e.mem.u32(a[0]), e.mem.u32(a[0] + 4), a[1]));
            Ret::default()
        });
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00ec_43fb, &args![object, 0u32, root, other, 1u32])
        }));
        // The exe's throw does not return; the engine stops when the double does.
        let payload = r.unwrap_err();
        let message = payload.downcast_ref::<String>().unwrap();
        assert!(message.contains("_CxxThrowException returned"));
        assert_eq!(
            *seen.borrow(),
            vec![(BAD_CAST_VTABLE, BAD_CAST_MESSAGE, BAD_CAST_THROW_INFO)]
        );
    }

    #[test]
    fn bad_cast_from_message_passes_the_message_word_and_sets_its_vtable() {
        let mut e = engine();
        e.register(EXCEPTION_FROM_MESSAGE, |e, a| {
            let message = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], 0x0101_0101);
            e.mem.set_u32(a[0] + 4, message);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xc);
        assert_eq!(
            e.call(0x00ec_79c4, &args![this, 0x0123_4567u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), BAD_CAST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x0123_4567);
    }
}
