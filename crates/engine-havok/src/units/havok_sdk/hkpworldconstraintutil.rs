//! `Havok SDK/hkpworldconstraintutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkReferencedObject::addReference` / `removeReference` (`00c90510`, `00c905b0`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpWorldOperationUtil::mergeIslands` (Xbox PDB, `00cb5570`, cdecl `(world, bodyA,
/// bodyB)`).
const MERGE_ISLANDS: u32 = 0x00cb_5570;
/// `hkpConstraintOwner::addConstraintInfo` (Xbox PDB, `00cb74d0`, `this`, `(instance,
/// info)`).
const ADD_CONSTRAINT_INFO: u32 = 0x00cb_74d0;
/// `hkpModifierConstraintAtom::addAllModifierDataToConstraintInfo` (Xbox PDB, `00d22120`,
/// cdecl `(atom, info, flagByte)`, answers a byte).
const MODIFIER_DATA_TO_INFO: u32 = 0x00d2_2120;
/// The array insertion `00d27ec0` (`this` = the array, `(index, source, count)`).
const ARRAY_INSERT: u32 = 0x00d2_7ec0;
/// `hkString::memMove` (Xbox PDB, `00cb1270`, cdecl `(destination, source, bytes)`).
const MEM_MOVE: u32 = 0x00cb_1270;
/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, count,
/// elementSize)`) and `_reserveMore` (`00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// A table of 16-bit sizes indexed by the bodies' flag bits (`010cb7a8`).
const SIZE_TABLE: u32 = 0x010c_b7a8;

/// Size of a constraint record (`hkpConstraintInternal`) in a body's array.
const RECORD_SIZE: u32 = 0x30;

// Translated from 00d27f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts one element: `00d27ec0(this, index, source, 1)`. The array is `this`; its elements
/// are 0x30 bytes.
pub fn fn_00d27f50(e: &mut Engine, this: Ptr, index: u32, source: Ptr) {
    e.call(ARRAY_INSERT, &args![this, index, source, 1u32]);
}

// Translated from 00d27820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::removeConstraint` (Xbox PDB): detaches the constraint instance
/// from its world. The owner (+8) gets its flag bit 0 set and bit 1 cleared at +0x25 and its
/// three counters (+0xC, +0x10, +0x14) reduced by the record's three 16-bit sizes (+0x14,
/// +0x16, +0x18), which are also subtracted from the record; the owner's count at +0x1C
/// drops. The record is taken out of the other body's slave list (+0xB4, the last entry
/// moves into its slot), its runtime data is cut out of the master body's block (+0xC0, used
/// size +0xC4) with `memMove`, and the records after it in the master's array (+0xAC, 0x30
/// bytes each, count at +0xB0) shift down one place with their back pointers and runtime
/// pointers fixed. Finally the instance loses a reference (`00c905b0`).
pub fn hkp_world_constraint_util_remove_constraint(e: &mut Engine, instance: Ptr) {
    let inst = instance.addr();
    let owner = e.mem.u32(inst + 8);
    let flags = e.mem.u8(owner + 0x25);
    e.mem.set_u8(owner + 0x25, (flags & 0xfd) | 1);
    let record = e.mem.u32(inst + 0x30);
    let size_a = u32::from(e.mem.u16(record + 0x14));
    let size_c = u32::from(e.mem.u16(record + 0x18));
    let size_b = u32::from(e.mem.u16(record + 0x16));
    if e.mem.u32(inst + 0x30) != 0 {
        let owner_now = e.mem.u32(inst + 8);
        for (offset, amount) in [(0xc, size_a), (0x10, size_b), (0x14, size_c)] {
            let value = e.mem.u32(owner_now + offset);
            e.mem
                .set_u32(owner_now + offset, value.wrapping_sub(amount));
        }
        let record_now = e.mem.u32(inst + 0x30);
        for (offset, amount) in [(0x16, size_b), (0x18, size_c), (0x14, size_a)] {
            let value = e.mem.u16(record_now + offset);
            e.mem
                .set_u16(record_now + offset, value.wrapping_sub(amount as u16));
        }
    }
    let count = e.mem.u32(owner + 0x1c);
    e.mem.set_u32(owner + 0x1c, count.wrapping_sub(1));
    e.mem.set_u32(inst + 8, 0);
    let slot = e.mem.u16(record + 0x22);
    let which = u32::from(e.mem.u8(record + 0x1a));
    let slave_body = e.mem.u32(record + (2 - which) * 4);
    let slave_data = e.mem.u32(slave_body + 0xb4);
    let slave_size = e.mem.u32(slave_body + 0xb8);
    let last = e.mem.u32(
        slave_data
            .wrapping_add(slave_size.wrapping_mul(4))
            .wrapping_sub(4),
    );
    e.mem.set_u32(slave_data + u32::from(slot) * 4, last);
    e.mem.set_u32(
        slave_body + 0xb8,
        e.mem.u32(slave_body + 0xb8).wrapping_sub(1),
    );
    let last_record = e.mem.u32(last + 0x30);
    e.mem.set_u16(last_record + 0x22, slot);
    let which = u32::from(e.mem.u8(record + 0x1a));
    let master = e.mem.u32(record + which * 4 + 4);
    let runtime = e.mem.u32(record + 0x1c);
    let mut shift = 0u32;
    if runtime != 0 {
        let size = u32::from(e.mem.u16(record + 0x20));
        shift = 0u32.wrapping_sub(size);
        let source = size.wrapping_add(runtime);
        let bytes = e
            .mem
            .u32(master + 0xc4)
            .wrapping_sub(source)
            .wrapping_add(e.mem.u32(master + 0xc0));
        e.call(MEM_MOVE, &args![runtime, source, bytes]);
        let used = e.mem.u32(master + 0xc4);
        e.mem.set_u32(
            master + 0xc4,
            used.wrapping_sub(u32::from(e.mem.u16(record + 0x20))),
        );
        e.mem.set_u32(record + 0x1c, 0);
    }
    e.mem.set_u32(inst + 0x30, 0);
    let count = u32::from(e.mem.u16(master + 0xb0));
    let array = e.mem.u32(master + 0xac);
    let end = count
        .wrapping_mul(RECORD_SIZE)
        .wrapping_sub(RECORD_SIZE)
        .wrapping_add(array);
    let mut cursor = record;
    if cursor < end {
        loop {
            let next = e.mem.bytes(cursor + RECORD_SIZE, RECORD_SIZE);
            e.mem.write(cursor, &next);
            let moved = e.mem.u32(cursor);
            e.mem.set_u32(moved + 0x30, cursor);
            let runtime = e.mem.u32(cursor + 0x1c);
            let fixed = if runtime == 0 {
                0
            } else {
                runtime.wrapping_add(shift)
            };
            e.mem.set_u32(cursor + 0x1c, fixed);
            cursor += RECORD_SIZE;
            if cursor >= end {
                break;
            }
        }
    }
    let count = e.mem.u16(master + 0xb0);
    e.mem.set_u16(master + 0xb0, count.wrapping_add(0xffff));
    e.call(REMOVE_REFERENCE, &args![inst]);
}

// Translated from 00d27ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::addConstraint` (Xbox PDB): attaches the constraint `instance`
/// (the first word is not read). It takes a reference, asks the instance's data (+0xC,
/// vtable slot 0x3c) for its constraint info and builds a 0x30-byte record on the stack: the
/// instance, the two bodies (+0x14, +0x18), the atoms (the instance's, or the info's) with
/// their size, the priority byte (+0x1C of the instance), the master flag at +0x1A (set when
/// body A is fixed, type 5 at +0xE8; the islands of the two bodies are merged first when
/// neither is fixed and they differ), the flags at +0x12 (2 for atom types 1 and 0x14, 4 when
/// the data's slot 0x34 says so, plus what the modifier atoms add) and at +0x1B (the
/// instance's slot 0x18 is zero, for atom type 1). The instance joins the master body's
/// island (+8 of the instance, whose count at +0x1C rises); for atom type 0x16 the info's size
/// grows by a table entry chosen by the bodies' flag bytes (+0xD2). The record is inserted
/// into the master body's sorted array (+0xAC) by priority, the instance's order (+0x34)
/// and the other body's value at +0xD4; back pointers of the moved records are fixed. The
/// data's slot 0x24 gives the runtime size (rounded to 16); room is made for it in the
/// master's block (+0xC0), the pointers of the other records are shifted, the data's slot
/// 0x2C initializes the runtime data, and the instance is appended to the other body's slave
/// list (+0xB4) with its index kept in the record at +0x22.
pub fn hkp_world_constraint_util_add_constraint(e: &mut Engine, _unused_0: u32, instance: Ptr) {
    let inst = instance.addr();
    e.call(ADD_REFERENCE, &args![inst]);
    // The exe's 0x74-byte frame: scratch bytes at +0xE and +0xF, the 8-byte answer at +0x24,
    // the constraint info at +0x2C and the record at +0x44.
    e.with_stack(0x74, |e, frame| {
        let frame = frame.addr();
        let info = frame + 0x2c;
        let record = frame + 0x44;
        let flag_byte = frame + 0xe;
        let answer_byte = frame + 0xf;
        let runtime_size_slot = frame + 0x24;
        let data = e.mem.u32(inst + 0xc);
        e.vcall(data, 0x3c, &args![info]);
        let atoms = e.mem.u32(inst + 0x10);
        let body_a = e.mem.u32(inst + 0x14);
        let body_b = e.mem.u32(inst + 0x18);
        e.mem.set_u32(record, inst);
        e.mem.set_u32(record + 4, body_a);
        e.mem.set_u32(record + 8, body_b);
        if atoms != 0 {
            e.mem.set_u32(record + 0xc, atoms);
            let size = e.mem.u16(atoms + 0x10);
            e.mem.set_u16(record + 0x10, size);
        } else {
            let from_info = e.mem.u32(info + 0x10);
            e.mem.set_u32(record + 0xc, from_info);
            let size = e.mem.u16(info + 0x14);
            e.mem.set_u16(record + 0x10, size);
        }
        e.mem.set_u8(record + 0x13, e.mem.u8(inst + 0x1c));
        let a_is_fixed = if e.mem.u8(body_a + 0xe8) == 5 {
            true
        } else {
            if e.mem.u8(body_b + 0xe8) != 5 && e.mem.u32(body_a + 0xcc) != e.mem.u32(body_b + 0xcc)
            {
                let world = e.mem.u32(body_a + 8);
                e.call(MERGE_ISLANDS, &args![world, body_a, body_b]);
            }
            e.mem.u8(body_a + 0xe8) == 5
        };
        e.mem.set_u8(record + 0x1a, u8::from(a_is_fixed));
        let master = e.mem.u32(record + 4 + 4 * u32::from(a_is_fixed));
        let island = e.mem.u32(master + 0xcc);
        e.mem.set_u32(inst + 8, island);
        e.mem
            .set_u32(island + 0x1c, e.mem.u32(island + 0x1c).wrapping_add(1));
        let atom_type = e.mem.u16(e.mem.u32(record + 0xc));
        e.mem.set_u8(record + 0x1b, 1);
        e.mem.set_u8(record + 0x12, 0);
        if atom_type == 1 {
            let data = e.mem.u32(inst + 0xc);
            e.mem.set_u8(record + 0x12, 2);
            let reported = e.vcall(data, 0x34, &args![answer_byte]).u32();
            if e.mem.u8(reported) != 0 {
                e.mem.set_u8(record + 0x12, e.mem.u8(record + 0x12) | 4);
            }
            let verdict = e.vcall(inst, 0x18, &args![]).u32();
            e.mem.set_u8(record + 0x1b, u8::from(verdict == 0));
        } else if atom_type == 0x14 {
            e.mem.set_u8(record + 0x12, 2);
        }
        e.mem.set_u8(flag_byte, 0);
        let modifier = e.mem.u32(inst + 0x10);
        if modifier != 0 {
            let added = e
                .call(MODIFIER_DATA_TO_INFO, &args![modifier, info, flag_byte])
                .u8();
            e.mem.set_u8(record + 0x12, e.mem.u8(record + 0x12) | added);
        }
        if atom_type == 0x16 {
            let flags =
                e.mem.u8(e.mem.u32(inst + 0x18) + 0xd2) | e.mem.u8(e.mem.u32(inst + 0x14) + 0xd2);
            let masked = flags.wrapping_sub(flags & e.mem.u8(flag_byte));
            let extra = u32::from(e.mem.u16(SIZE_TABLE + u32::from(masked & 7) * 2));
            e.mem
                .set_u32(info + 4, e.mem.u32(info + 4).wrapping_add(extra));
        }
        e.mem.set_u16(record + 0x16, 0);
        e.mem.set_u16(record + 0x18, 0);
        e.mem.set_u32(inst + 0x30, record);
        e.mem.set_u16(record + 0x14, 0);
        let island_owner = e.mem.u32(inst + 8);
        e.call(ADD_CONSTRAINT_INFO, &args![island_owner, inst, info]);

        // Find the place in the master's sorted array.
        let list = master + 0xac;
        let count = u32::from(e.mem.u16(master + 0xb0));
        let mut index = 0u32;
        let mut cursor = e.mem.u32(list);
        if count > 0 {
            loop {
                let priority = e.mem.u8(record + 0x13);
                let other_priority = e.mem.u8(cursor + 0x13);
                if priority < other_priority {
                    break;
                }
                if priority == other_priority {
                    let order = e.mem.u32(e.mem.u32(record) + 0x34);
                    let other_order = e.mem.u32(e.mem.u32(cursor) + 0x34);
                    if order < other_order {
                        break;
                    }
                    if order == other_order {
                        let their_other = e.mem.u32(cursor + 8) ^ e.mem.u32(cursor + 4) ^ master;
                        let my_other = master ^ e.mem.u32(record + 8) ^ e.mem.u32(record + 4);
                        if e.mem.u32(my_other + 0xd4) < e.mem.u32(their_other + 0xd4) {
                            break;
                        }
                    }
                }
                index += 1;
                cursor += RECORD_SIZE;
                if index >= count {
                    break;
                }
            }
        }
        let capacity = e.mem.u16(list + 6) & 0x3fff;
        let first_to_fix = if (count as u16) < capacity { index } else { 0 };
        fn_00d27f50(e, Ptr::new(list), index, Ptr::new(record));
        let mut fix = first_to_fix;
        while fix < u32::from(e.mem.u16(list + 4)) {
            let slot = e.mem.u32(list) + fix * RECORD_SIZE;
            let owner_instance = e.mem.u32(slot);
            e.mem.set_u32(owner_instance + 0x30, slot);
            fix += 1;
        }

        // The runtime data.
        let placed = e.mem.u32(inst + 0x30);
        let placed_instance = e.mem.u32(placed);
        let placed_data = e.mem.u32(placed_instance + 0xc);
        let flags = u32::from(e.mem.u8(inst + 0x1d));
        e.vcall(placed_data, 0x24, &args![flags, runtime_size_slot]);
        let size = e.mem.u32(runtime_size_slot).wrapping_add(0xf) & 0xffff_fff0;
        e.mem.set_u32(runtime_size_slot, size);
        e.mem.set_u16(placed + 0x20, size as u16);
        if size == 0 {
            e.mem.set_u32(placed + 0x1c, 0);
        } else {
            let block = master + 0xc0;
            let used = e.mem.u32(master + 0xc4);
            let old_block = e.mem.u32(master + 0xc0);
            let new_used = used.wrapping_add(size);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(block + 8) & 0x3fff_ffff;
            if (capacity as i32) < (new_used as i32) {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE, &args![allocator, block, new_used, 1u32]);
            }
            e.mem.set_u32(block + 4, new_used);
            let mut end_of_runtime = e.mem.u32(block);
            let delta = end_of_runtime.wrapping_sub(old_block);
            if (index as i32) > 0 {
                let mut offset = 0u32;
                let mut remaining = index;
                loop {
                    let slot = e.mem.u32(list).wrapping_add(offset);
                    let runtime = e.mem.u32(slot + 0x1c);
                    if runtime != 0 {
                        let slot_size = u32::from(e.mem.u16(slot + 0x20));
                        let moved = runtime.wrapping_add(delta);
                        e.mem.set_u32(slot + 0x1c, moved);
                        end_of_runtime = moved.wrapping_add(slot_size);
                    }
                    offset = offset.wrapping_add(RECORD_SIZE);
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
            let rounded = u32::from(e.mem.u16(placed + 0x20));
            let tail = e
                .mem
                .u32(block + 4)
                .wrapping_sub(rounded)
                .wrapping_sub(end_of_runtime)
                .wrapping_add(e.mem.u32(block));
            e.call(
                MEM_MOVE,
                &args![rounded.wrapping_add(end_of_runtime), end_of_runtime, tail],
            );
            let shift = delta.wrapping_add(u32::from(e.mem.u16(placed + 0x20)));
            e.mem.set_u32(placed + 0x1c, end_of_runtime);
            let mut next = index.wrapping_add(1);
            if (next as i32) < i32::from(e.mem.u16(list + 4)) {
                loop {
                    let slot = e.mem.u32(list).wrapping_add(next.wrapping_mul(RECORD_SIZE));
                    let runtime = e.mem.u32(slot + 0x1c);
                    let fixed = if runtime != 0 {
                        runtime.wrapping_add(shift)
                    } else {
                        0
                    };
                    e.mem.set_u32(slot + 0x1c, fixed);
                    next = next.wrapping_add(1);
                    if (next as i32) >= i32::from(e.mem.u16(list + 4)) {
                        break;
                    }
                }
            }
        }
        let placed_instance = e.mem.u32(placed);
        let placed_data = e.mem.u32(placed_instance + 0xc);
        let rounded = u32::from(e.mem.u16(placed + 0x20));
        let runtime = e.mem.u32(placed + 0x1c);
        e.vcall(placed_data, 0x2c, &args![inst, runtime, rounded]);

        // Join the other body's slave list.
        let other_index = 2 - u32::from(e.mem.u8(placed + 0x1a));
        let other = e.mem.u32(placed + other_index * 4);
        e.mem.set_u16(placed + 0x22, e.mem.u16(other + 0xb8));
        let slaves = other + 0xb4;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(slaves + 8) & 0x3fff_ffff;
        if e.mem.u32(slaves + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, slaves, 4u32]);
        }
        let used = e.mem.u32(slaves + 4);
        let data = e.mem.u32(slaves);
        e.mem.set_u32(data.wrapping_add(used.wrapping_mul(4)), inst);
        e.mem.set_u32(slaves + 4, used.wrapping_add(1));
    });
}

/// `hkpModifierConstraintAtom::addModifierDataToConstraintInfo` (Xbox PDB, `00d22090`,
/// `this` = the atom, `RET 8`: `(info, flagByte)`).
const ATOM_ADD_MODIFIER_DATA: u32 = 0x00d2_2090;
/// `00cb7480` (`this` = the owner, `RET 8`: `(instance word, info)`): the call that follows
/// the accounting of a removed modifier's data.
const REMOVE_CONSTRAINT_INFO: u32 = 0x00cb_7480;
/// The smallest atom type that still belongs to a constraint's modifier chain: a next atom
/// of a smaller type ends the chain (the code compares the 16-bit type with `0x17`).
const FIRST_MODIFIER_TYPE: u16 = 0x17;
/// The modifier type the code links after the last atom (`0x1a`) and the one it keeps first
/// (`0x1b`).
const LAST_PLACED_TYPE: u16 = 0x1a;
const FIRST_PLACED_TYPE: u16 = 0x1b;

// Translated from 00d275a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::updateFatherOfMovedAtom` (Xbox PDB), cdecl `(instance, old, new,
/// size)`: a modifier atom of the constraint instance moved from `old` to `new`. The link
/// that held `old` is rewritten: with no first atom (+0x10) only the record's (+0x30)
/// first-modifier pointer (+0xC) and size (+0x10) are set to `new` and `size`; when the
/// first atom is `old` the instance's first-atom pointer is set to `new` as well; otherwise
/// the chain is walked (next pointer at +0x14) and the pointer (+0x14) and 16-bit size
/// (+0x12) of the atom before `old` are set. Only the low 16 bits of the last word are read.
pub fn hkp_world_constraint_util_update_father_of_moved_atom(
    e: &mut Engine,
    instance: Ptr,
    old: u32,
    new: u32,
    size: u16,
) {
    let inst = instance.addr();
    let first = e.mem.u32(inst + 0x10);
    let record = e.mem.u32(inst + 0x30);
    if first == 0 {
        if record != 0 {
            e.mem.set_u32(record + 0xc, new);
            e.mem.set_u16(record + 0x10, size);
        }
    } else if first == old {
        e.mem.set_u32(inst + 0x10, new);
        if record != 0 {
            e.mem.set_u32(record + 0xc, new);
            e.mem.set_u16(record + 0x10, size);
        }
    } else {
        let mut previous = first;
        let mut current = e.mem.u32(first + 0x14);
        while current != old {
            previous = current;
            current = e.mem.u32(current + 0x14);
        }
        e.mem.set_u32(previous + 0x14, new);
        e.mem.set_u16(previous + 0x12, size);
    }
}

// Translated from 00d27610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(bodyA, bodyB)`: finds the constraint record that joins the two bodies. For each
/// body in turn (`bodyA`, then `bodyB`) the body's record array (+0xAC, 16-bit count at
/// +0xB0, 0x30 bytes each) is searched for a record whose two words at +4 and +8 xor with
/// the body itself to the other body. Answers the first word of the matching record (the
/// instance), or 0.
pub fn fn_00d27610(e: &mut Engine, body_a: u32, body_b: u32) -> u32 {
    let bodies = [body_a, body_b];
    for (side, &body) in bodies.iter().enumerate() {
        let other = bodies[1 - side];
        let count = u32::from(e.mem.u16(body + 0xb0));
        let array = e.mem.u32(body + 0xac);
        for index in 0..count {
            let record = array.wrapping_add(index * 0x30);
            let first = e.mem.u32(record + 4);
            let second = e.mem.u32(record + 8);
            if second ^ first ^ body == other {
                return e.mem.u32(record);
            }
        }
    }
    0
}

// Translated from 00d27690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::removeAndDeleteModifier` (Xbox PDB), cdecl `(instance, owner,
/// type)`: runs the owner's virtual function at +0x18 (no arguments), then takes the
/// modifier atom of `type` out of the instance's chain (+0x10, atoms linked through +0x14,
/// 16-bit type at +0, the size of the next atom at +0x12; a next atom of a type below 0x17
/// ends the chain, and the first atom is compared before that test) and frees it through
/// the memory router's allocator (slot 8, the atom's 16-bit size at +0x10). When the atom
/// was first, the instance's first pointer becomes its next atom, or 0 when that one's type
/// is below 0x17. With a record at +0x30 the record's first-modifier pointer (+0xC) and
/// size (+0x10) are also set from the removed first atom, and the atom's modifier data
/// (`00d22090`) is accounted: the owner counter (+0xC) and the record's 16-bit +0x14 grow
/// by the table entry (`010cb7a8`) selected by the flag byte anded with the two bodies'
/// bytes at +0xD2, and `00cb7480` is called with the record's first word and the info.
pub fn hkp_world_constraint_util_remove_and_delete_modifier(
    e: &mut Engine,
    instance: Ptr,
    owner: Ptr,
    modifier_type: u32,
) {
    let inst = instance.addr();
    let owner_addr = owner.addr();
    e.vcall(owner_addr, 0x18, &args![]);
    let first = e.mem.u32(inst + 0x10);
    if first == 0 {
        return;
    }
    let record = e.mem.u32(inst + 0x30);
    let removed = if u32::from(e.mem.u16(first)) == modifier_type {
        let next = e.mem.u32(first + 0x14);
        if record != 0 {
            e.mem.set_u32(record + 0xc, next);
            let size = e.mem.u16(first + 0x12);
            e.mem.set_u16(record + 0x10, size);
        }
        // `SBB` / `NOT` / `AND`: zero when the next atom's type is below 0x17.
        let kept = if e.mem.u16(next) < FIRST_MODIFIER_TYPE {
            0
        } else {
            next
        };
        e.mem.set_u32(inst + 0x10, kept);
        first
    } else {
        let mut previous = first;
        let mut current = e.mem.u32(first + 0x14);
        if e.mem.u16(current) < FIRST_MODIFIER_TYPE {
            return;
        }
        loop {
            if u32::from(e.mem.u16(current)) == modifier_type {
                let after = e.mem.u32(current + 0x14);
                e.mem.set_u32(previous + 0x14, after);
                let size = e.mem.u16(current + 0x12);
                e.mem.set_u16(previous + 0x12, size);
                break current;
            }
            previous = current;
            current = e.mem.u32(current + 0x14);
            if e.mem.u16(current) < FIRST_MODIFIER_TYPE {
                return;
            }
        }
    };
    if record != 0 {
        e.with_stack(0x14, |e, buffer| {
            // The 16-byte info (zeroed) and, after it, the flag byte (zero), as in the exe.
            let info = buffer.addr();
            let flag = info + 0x10;
            e.call(ATOM_ADD_MODIFIER_DATA, &args![removed, info, flag]);
            let body_b = e.mem.u32(inst + 0x18);
            let body_a = e.mem.u32(inst + 0x14);
            let bodies = e.mem.u8(body_b + 0xd2) | e.mem.u8(body_a + 0xd2);
            let combined = e.mem.u8(flag) & bodies;
            e.mem.set_u8(flag, combined);
            let amount = e.mem.u16(SIZE_TABLE + u32::from(combined & 7) * 2);
            let counter = e.mem.u32(owner_addr + 0xc);
            e.mem
                .set_u32(owner_addr + 0xc, counter.wrapping_add(u32::from(amount)));
            let own = e.mem.u32(inst + 0x30);
            let total = e.mem.u16(own + 0x14);
            e.mem.set_u16(own + 0x14, total.wrapping_add(amount));
            let first_word = e.mem.u32(record);
            e.call(REMOVE_CONSTRAINT_INFO, &args![owner, first_word, info]);
        });
    }
    let size = u32::from(e.mem.u16(removed + 0x10));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![removed, size]);
}

// Translated from 00d277c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::findModifier` (Xbox PDB), cdecl `(instance, type)`: the modifier
/// atom with the 16-bit type `type` in the instance's chain (+0x10, next at +0x14), or 0.
/// The first atom is compared before its type is looked at; every later atom ends the
/// search when its type is below 0x17.
pub fn hkp_world_constraint_util_find_modifier(
    e: &mut Engine,
    instance: Ptr,
    modifier_type: u32,
) -> u32 {
    let mut atom = e.mem.u32(instance.addr() + 0x10);
    if atom == 0 {
        return 0;
    }
    if u32::from(e.mem.u16(atom)) == modifier_type {
        return atom;
    }
    loop {
        atom = e.mem.u32(atom + 0x14);
        if e.mem.u16(atom) < FIRST_MODIFIER_TYPE {
            return 0;
        }
        if u32::from(e.mem.u16(atom)) == modifier_type {
            return atom;
        }
    }
}

// Translated from 00d277f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(instance)`: the last atom of the instance's modifier chain (+0x10, next at
/// +0x14): the walk goes on while the next atom's type is at least 0x17. 0 for an instance
/// with no atom.
pub fn fn_00d277f0(e: &mut Engine, instance: Ptr) -> u32 {
    let mut atom = e.mem.u32(instance.addr() + 0x10);
    if atom == 0 {
        return 0;
    }
    let mut next = e.mem.u32(atom + 0x14);
    while e.mem.u16(next) >= FIRST_MODIFIER_TYPE {
        atom = next;
        next = e.mem.u32(next + 0x14);
    }
    atom
}

// Translated from 00d27960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldConstraintUtil::addModifier` (Xbox PDB), cdecl `(instance, owner, atom)`: runs
/// the owner's virtual function at +0x18, then links the modifier atom into the instance's
/// chain (+0x10, next at +0x14, size of the next at +0x12). An atom of type 0x1a goes after
/// the last atom (`00d277f0`) when there is one; any other goes after a first atom of type
/// 0x1b, and otherwise becomes the first. As the first atom with a record at +0x30 it takes
/// the record's first-modifier link (+0xC, size +0x10) and the record takes it; without a
/// record the virtual function at +0x3C of the object at +0xC fills a 0x18-byte buffer whose
/// word at +0x10 and half-word at +0x14 become the atom's next link and size, and the
/// function ends. With a record the atom's modifier data (`00d22090`) is added to the info
/// for `00cb74d0`, then the owner counter (+0xC) and the record's 16-bit +0x14 lose the
/// table entry (`010cb7a8`) selected by the flag byte anded with the two bodies' bytes at
/// +0xD2, and the record's byte at +0x12 gets the low byte `00d22090` answered or-ed in.
pub fn hkp_world_constraint_util_add_modifier(
    e: &mut Engine,
    instance: Ptr,
    owner: Ptr,
    atom: Ptr,
) {
    let inst = instance.addr();
    let owner_addr = owner.addr();
    let new = atom.addr();
    e.vcall(owner_addr, 0x18, &args![]);
    let last = fn_00d277f0(e, instance);
    let record = e.mem.u32(inst + 0x30);
    let first = e.mem.u32(inst + 0x10);
    let before = if e.mem.u16(new) == LAST_PLACED_TYPE && last != 0 {
        Some(last)
    } else if first != 0 && e.mem.u16(first) == FIRST_PLACED_TYPE {
        Some(first)
    } else {
        None
    };
    if let Some(before) = before {
        let next = e.mem.u32(before + 0x14);
        e.mem.set_u32(new + 0x14, next);
        let size = e.mem.u16(before + 0x12);
        e.mem.set_u16(new + 0x12, size);
        e.mem.set_u32(before + 0x14, new);
        let own_size = e.mem.u16(new + 0x10);
        e.mem.set_u16(before + 0x12, own_size);
    } else {
        e.mem.set_u32(inst + 0x10, new);
        if record == 0 {
            let data = e.mem.u32(inst + 0xc);
            e.with_stack(0x18, |e, buffer| {
                e.vcall(data, 0x3c, &args![buffer]);
                let next = e.mem.u32(buffer.addr() + 0x10);
                let size = e.mem.u16(buffer.addr() + 0x14);
                e.mem.set_u32(new + 0x14, next);
                e.mem.set_u16(new + 0x12, size);
            });
            return;
        }
        let next = e.mem.u32(record + 0xc);
        e.mem.set_u32(new + 0x14, next);
        let size = e.mem.u16(record + 0x10);
        e.mem.set_u16(new + 0x12, size);
        e.mem.set_u32(record + 0xc, new);
        let own_size = e.mem.u16(new + 0x10);
        e.mem.set_u16(record + 0x10, own_size);
    }
    if record != 0 {
        e.with_stack(0x18, |e, info| {
            // The flag byte is a stack slot of its own in the exe.
            e.with_stack(4, |e, flag_slot| {
                let flag = flag_slot.addr();
                let answered = e
                    .call(ATOM_ADD_MODIFIER_DATA, &args![new, info, flag])
                    .u32();
                e.call(ADD_CONSTRAINT_INFO, &args![owner, instance, info]);
                let body_b = e.mem.u32(inst + 0x18);
                let body_a = e.mem.u32(inst + 0x14);
                let bodies = e.mem.u8(body_b + 0xd2) | e.mem.u8(body_a + 0xd2);
                let combined = e.mem.u8(flag) & bodies;
                e.mem.set_u8(flag, combined);
                let amount = e.mem.u16(SIZE_TABLE + u32::from(combined & 7) * 2);
                let counter = e.mem.u32(owner_addr + 0xc);
                e.mem
                    .set_u32(owner_addr + 0xc, counter.wrapping_sub(u32::from(amount)));
                let own = e.mem.u32(inst + 0x30);
                let total = e.mem.u16(own + 0x14);
                e.mem.set_u16(own + 0x14, total.wrapping_sub(amount));
                let own = e.mem.u32(inst + 0x30);
                if own != 0 {
                    let bits = e.mem.u8(own + 0x12);
                    e.mem.set_u8(own + 0x12, bits | answered as u8);
                }
            });
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_7820,
            hkp_world_constraint_util_remove_constraint(Ptr)
        ),
        entry!(
            0x00d2_7ac0,
            hkp_world_constraint_util_add_constraint(u32, Ptr)
        ),
        entry!(0x00d2_7f50, fn_00d27f50(Ptr, u32, Ptr)),
        entry!(
            0x00d2_75a0,
            hkp_world_constraint_util_update_father_of_moved_atom(Ptr, u32, u32, u16)
        ),
        entry!(0x00d2_7610, fn_00d27610(u32, u32) -> u32),
        entry!(
            0x00d2_7690,
            hkp_world_constraint_util_remove_and_delete_modifier(Ptr, Ptr, u32)
        ),
        entry!(
            0x00d2_77c0,
            hkp_world_constraint_util_find_modifier(Ptr, u32) -> u32
        ),
        entry!(0x00d2_77f0, fn_00d277f0(Ptr) -> u32),
        entry!(
            0x00d2_7960,
            hkp_world_constraint_util_add_modifier(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A real `memMove` that also logs.
    fn mem_move_double(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(MEM_MOVE, move |e, a| {
            log.borrow_mut().push((MEM_MOVE, a.to_vec()));
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn insert_one_forwards_with_a_count_of_one() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, ARRAY_INSERT);
        e.call(0x00d2_7f50, &args![0x1000u32, 4u32, 0x2000u32]);
        assert_eq!(calls(&log, ARRAY_INSERT), vec![vec![0x1000, 4, 0x2000, 1]]);
    }

    /// A body: master array at +0xAC (room for 4 records), runtime block at +0xC0, slave
    /// list at +0xB4 (room for 4).
    fn body(e: &mut Engine) -> u32 {
        let body = e.mem.alloc(0x100);
        let records = e.mem.alloc(0x30 * 4);
        e.mem.set_u32(body + 0xac, records);
        e.mem.set_u16(body + 0xb2, 4);
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(body + 0xc0, block);
        e.mem.set_u32(body + 0xc8, 0x100);
        let slaves = e.mem.alloc(0x40);
        e.mem.set_u32(body + 0xb4, slaves);
        e.mem.set_u32(body + 0xbc, 0x8000_0004);
        body
    }

    #[test]
    fn remove_constraint_detaches_the_record_and_shifts_the_rest() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, REMOVE_REFERENCE);
        mem_move_double(&mut e, &log);
        let master = body(&mut e);
        let other = body(&mut e);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(owner + 0x25, 0xfe);
        e.mem.set_u32(owner + 0x1c, 3);
        for (offset, value) in [(0xc, 100u32), (0x10, 200), (0x14, 300)] {
            e.mem.set_u32(owner + offset, value);
        }
        // Three instances with records 0..2; the second is removed. Runtime data: record 0
        // owns 0x10 bytes at the block start, record 1 owns 0x20, record 2 owns 0x10.
        let block = e.mem.u32(master + 0xc0);
        let records = e.mem.u32(master + 0xac);
        e.mem.set_u16(master + 0xb0, 3);
        e.mem.set_u32(master + 0xc4, 0x40);
        let mut instances = Vec::new();
        for i in 0..3u32 {
            let instance = e.mem.alloc(0x40);
            let record = records + 0x30 * i;
            e.mem.set_u32(record, instance);
            e.mem.set_u32(instance + 0x30, record);
            e.mem.set_u32(instance + 8, owner);
            // The record sits with the master at index 1 - (its own slot) and its other body
            // is `other`.
            e.mem.set_u8(record + 0x1a, 0);
            e.mem.set_u32(record + 4, master);
            e.mem.set_u32(record + 8, other);
            instances.push(instance);
        }
        for (i, (offset, size)) in [(0u32, 0x10u32), (0x10, 0x20), (0x30, 0x10)]
            .into_iter()
            .enumerate()
        {
            let record = records + 0x30 * i as u32;
            e.mem.set_u32(record + 0x1c, block + offset);
            e.mem.set_u16(record + 0x20, size as u16);
            e.mem.set_u16(record + 0x22, i as u16);
        }
        // `record + 0x1a` is 0, so the master is `record + 4` and the slave body is
        // `record + 8`. The slave list holds the three instances.
        let slaves = e.mem.u32(other + 0xb4);
        for (i, instance) in instances.iter().enumerate() {
            e.mem.set_u32(slaves + 4 * i as u32, *instance);
        }
        e.mem.set_u32(other + 0xb8, 3);
        // The record sizes the removal subtracts.
        let doomed = instances[1];
        let doomed_record = records + 0x30;
        e.mem.set_u16(doomed_record + 0x14, 1);
        e.mem.set_u16(doomed_record + 0x16, 2);
        e.mem.set_u16(doomed_record + 0x18, 3);
        e.call(0x00d2_7820, &args![doomed]);

        assert_eq!(e.mem.u8(owner + 0x25), 0xfd);
        assert_eq!(e.mem.u32(owner + 0x1c), 2);
        assert_eq!(e.mem.u32(owner + 0xc), 99);
        assert_eq!(e.mem.u32(owner + 0x10), 198);
        assert_eq!(e.mem.u32(owner + 0x14), 297);
        assert_eq!(e.mem.u32(doomed + 8), 0);
        assert_eq!(e.mem.u32(doomed + 0x30), 0);
        // The slave list: the last entry took the removed slot and its record was updated.
        assert_eq!(e.mem.u32(other + 0xb8), 2);
        assert_eq!(e.mem.u32(slaves + 4), instances[2]);
        assert_eq!(e.mem.u16(records + 0x60 + 0x22), 1);
        // The runtime data after the removed record moved down by its 0x20 bytes.
        assert_eq!(
            calls(&log, MEM_MOVE),
            vec![vec![block + 0x10, block + 0x30, 0x40 - 0x30]]
        );
        assert_eq!(e.mem.u32(master + 0xc4), 0x20);
        // The record array: record 2 moved into place 1 and the count dropped.
        assert_eq!(e.mem.u16(master + 0xb0), 2);
        assert_eq!(e.mem.u32(records + 0x30), instances[2]);
        assert_eq!(e.mem.u32(instances[2] + 0x30), records + 0x30);
        assert_eq!(e.mem.u32(records + 0x30 + 0x1c), block + 0x10);
        assert_eq!(e.mem.u32(instances[0] + 0x30), records);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![doomed]]);
    }

    /// The pieces of an `addConstraint`: one instance between two bodies of the same island.
    struct AddRig {
        e: Engine,
        log: Log,
        instance: u32,
        master: u32,
        other: u32,
        island: u32,
        existing_instance: u32,
        runtime_calls: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    fn add_rig(master_fixed: bool) -> AddRig {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            ADD_REFERENCE,
            MERGE_ISLANDS,
            ADD_CONSTRAINT_INFO,
            ARRAY_RESERVE,
        ] {
            log_double(&mut e, &log, address);
        }
        mem_move_double(&mut e, &log);
        // `00d27ec0`: insert one 0x30-byte record at the index of a small array.
        e.register_double(ARRAY_INSERT, |e, a| {
            let (array, index, source) = (a[0], a[1], a[2]);
            let size = u32::from(e.mem.u16(array + 4));
            let data = e.mem.u32(array);
            let tail = e.mem.bytes(data + index * 0x30, (size - index) * 0x30);
            e.mem.write(data + (index + 1) * 0x30, &tail);
            let record = e.mem.bytes(source, 0x30);
            e.mem.write(data + index * 0x30, &record);
            e.mem.set_u16(array + 4, (size + 1) as u16);
            Ret::default()
        });
        let island = e.mem.alloc(0x40);
        let mut bodies = [0u32; 2];
        for body_slot in &mut bodies {
            *body_slot = body(&mut e);
            e.mem.set_u32(*body_slot + 0xcc, island);
            e.mem.set_u8(*body_slot + 0xd2, 0x5);
        }
        let (a, b) = (bodies[0], bodies[1]);
        if master_fixed {
            e.mem.set_u8(a + 0xe8, 5);
        }
        let (master, other) = if master_fixed { (b, a) } else { (a, b) };
        // The data object: slots 0x3c (info), 0x34 (a byte), 0x24 (runtime size), 0x2c.
        let runtime_calls = Rc::new(RefCell::new(Vec::new()));
        let initialized = Rc::clone(&runtime_calls);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u16(atom, 1);
        e.register_double(0x0900_003c, move |e, a| {
            e.mem.set_u32(a[1] + 0x10, atom);
            e.mem.set_u16(a[1] + 0x14, 0x20);
            Ret::default()
        });
        let reported = e.mem.alloc(4);
        e.mem.set_u8(reported, 1);
        e.register_double(0x0900_0034, move |_, _| Ret {
            eax: reported,
            ..Ret::default()
        });
        e.register_double(0x0900_0024, |e, a| {
            e.mem.set_u32(a[2], 0x15);
            Ret::default()
        });
        e.register_double(0x0900_002c, move |_, a| {
            initialized.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let data_vtable = e.mem.alloc(0x80);
        for slot in [0x3c, 0x34, 0x24, 0x2c] {
            e.mem.set_u32(data_vtable + slot, 0x0900_0000 + slot);
        }
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, data_vtable);
        e.register_double(0x0900_0018, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let instance_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(instance_vtable + 0x18, 0x0900_0018);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance, instance_vtable);
        e.mem.set_u32(instance + 0xc, data);
        e.mem.set_u32(instance + 0x14, a);
        e.mem.set_u32(instance + 0x18, b);
        e.mem.set_u8(instance + 0x1c, 3);
        e.mem.set_u8(instance + 0x1d, 9);
        e.mem.set_u32(instance + 0x34, 7);
        // The memory router for `_reserveMore`.
        let router = e.mem.alloc(0x20);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // One record already in the master's array: priority 5, runtime data of 0x10 bytes.
        let records = e.mem.u32(master + 0xac);
        let existing_instance = e.mem.alloc(0x40);
        e.mem.set_u32(records, existing_instance);
        e.mem.set_u32(existing_instance + 0x30, records);
        e.mem.set_u8(records + 0x13, 5);
        e.mem.set_u16(master + 0xb0, 1);
        let block = e.mem.u32(master + 0xc0);
        e.mem.set_u32(records + 0x1c, block);
        e.mem.set_u16(records + 0x20, 0x10);
        e.mem.set_u32(master + 0xc4, 0x10);
        e.mem.write(block, &[0xab; 0x10]);
        AddRig {
            e,
            log,
            instance,
            master,
            other,
            island,
            existing_instance,
            runtime_calls,
        }
    }

    #[test]
    fn add_constraint_inserts_ahead_of_a_later_priority() {
        let mut rig = add_rig(false);
        let (instance, master, other, island) = (rig.instance, rig.master, rig.other, rig.island);
        let e = &mut rig.e;
        e.call(0x00d2_7ac0, &args![0xdeadu32, instance]);
        let records = e.mem.u32(master + 0xac);
        // The new record sits first, the old one second; both instances point at theirs.
        assert_eq!(e.mem.u32(records), instance);
        assert_eq!(e.mem.u32(records + 0x30), rig.existing_instance);
        assert_eq!(e.mem.u32(instance + 0x30), records);
        assert_eq!(e.mem.u32(rig.existing_instance + 0x30), records + 0x30);
        assert_eq!(e.mem.u16(master + 0xb0), 2);
        // Fields of the new record.
        assert_eq!(e.mem.u32(records + 4), e.mem.u32(instance + 0x14));
        assert_eq!(e.mem.u8(records + 0x13), 3);
        assert_eq!(e.mem.u8(records + 0x1a), 0);
        assert_eq!(e.mem.u8(records + 0x12), 2 | 4);
        assert_eq!(e.mem.u8(records + 0x1b), 0);
        assert_eq!(e.mem.u16(records + 0x10), 0x20);
        // The island took the constraint.
        assert_eq!(e.mem.u32(instance + 8), island);
        assert_eq!(e.mem.u32(island + 0x1c), 1);
        // Runtime data: 0x15 rounds to 0x20, opened at the start of the block; the old
        // record's data moved up behind it.
        let block = e.mem.u32(master + 0xc0);
        assert_eq!(e.mem.u16(records + 0x20), 0x20);
        assert_eq!(e.mem.u32(records + 0x1c), block);
        assert_eq!(e.mem.u32(master + 0xc4), 0x30);
        assert_eq!(e.mem.u32(records + 0x30 + 0x1c), block + 0x20);
        assert_eq!(e.mem.bytes(block + 0x20, 0x10), vec![0xab; 0x10]);
        assert_eq!(
            *rig.runtime_calls.borrow(),
            vec![vec![
                e.mem.u32(e.mem.u32(records) + 0xc),
                instance,
                block,
                0x20
            ]]
        );
        // The instance joined the other body's slave list, remembered in the record.
        assert_eq!(e.mem.u32(other + 0xb8), 1);
        assert_eq!(e.mem.u32(e.mem.u32(other + 0xb4)), instance);
        assert_eq!(e.mem.u16(records + 0x22), 0);
        assert_eq!(calls(&rig.log, ADD_REFERENCE), vec![vec![instance]]);
        assert_eq!(
            calls(&rig.log, ADD_CONSTRAINT_INFO)[0][0..2],
            [island, instance]
        );
        assert!(calls(&rig.log, MERGE_ISLANDS).is_empty());
    }

    #[test]
    fn add_constraint_makes_the_other_body_master_when_the_first_is_fixed() {
        let mut rig = add_rig(true);
        let (instance, master, other) = (rig.instance, rig.master, rig.other);
        let e = &mut rig.e;
        e.call(0x00d2_7ac0, &args![0u32, instance]);
        let records = e.mem.u32(master + 0xac);
        assert_eq!(e.mem.u32(instance + 0x30), records);
        assert_eq!(e.mem.u8(records + 0x1a), 1);
        assert_eq!(e.mem.u32(other + 0xb8), 1);
        assert!(calls(&rig.log, MERGE_ISLANDS).is_empty());
    }

    #[test]
    fn add_constraint_merges_different_islands_first() {
        let mut rig = add_rig(false);
        let (instance, master, other) = (rig.instance, rig.master, rig.other);
        let e = &mut rig.e;
        let second_island = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0xcc, second_island);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(master + 8, world);
        e.call(0x00d2_7ac0, &args![0u32, instance]);
        assert_eq!(
            calls(&rig.log, MERGE_ISLANDS),
            vec![vec![world, master, other]]
        );
    }
}

#[cfg(test)]
mod modifier_chain_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OWNER_HOOK: u32 = 0x0900_0001;
    const FREE_HOOK: u32 = 0x0900_0002;
    const BUFFER_HOOK: u32 = 0x0900_0003;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    /// The calls of the allocator hook with the allocator itself (`this`) left out: the block
    /// and its size.
    fn freed(log: &Log) -> Vec<Vec<u32>> {
        calls(log, FREE_HOOK)
            .into_iter()
            .map(|args| args[1..].to_vec())
            .collect()
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An atom of the given type and own size (+0x10).
    fn atom(e: &mut Engine, kind: u16, size: u16) -> u32 {
        let atom = e.mem.alloc(0x20);
        e.mem.set_u16(atom, kind);
        e.mem.set_u16(atom + 0x10, size);
        atom
    }

    /// Links `atoms` through +0x14 and +0x12 (size of the next one) and ends with a plain
    /// atom of type 0x10; the instance's first pointer (+0x10) is the first atom.
    fn chain(e: &mut Engine, instance: u32, atoms: &[u32]) {
        let end = atom(e, 0x10, 0);
        for (i, &a) in atoms.iter().enumerate() {
            let next = atoms.get(i + 1).copied().unwrap_or(end);
            e.mem.set_u32(a + 0x14, next);
            let next_size = e.mem.u16(next + 0x10);
            e.mem.set_u16(a + 0x12, next_size);
        }
        e.mem.set_u32(instance + 0x10, atoms[0]);
    }

    #[test]
    fn update_father_rewrites_the_link_that_held_the_moved_atom() {
        let mut e = engine();
        let instance = e.mem.alloc(0x40);
        let record = e.mem.alloc(0x40);
        // No first atom: only the record.
        e.mem.set_u32(instance + 0x30, record);
        e.call(
            0x00d2_75a0,
            &args![instance, 0x1000u32, 0x2000u32, 0xffff_0007u32],
        );
        assert_eq!(e.mem.u32(record + 0xc), 0x2000);
        assert_eq!(e.mem.u16(record + 0x10), 7);
        assert_eq!(e.mem.u32(instance + 0x10), 0);
        // No first atom and no record: nothing happens.
        let bare = e.mem.alloc(0x40);
        e.call(0x00d2_75a0, &args![bare, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(bare + 0x10), 0);
        // The first atom moved.
        let first = atom(&mut e, 0x1a, 0);
        e.mem.set_u32(instance + 0x10, first);
        e.call(0x00d2_75a0, &args![instance, first, 0x3000u32, 9u32]);
        assert_eq!(e.mem.u32(instance + 0x10), 0x3000);
        assert_eq!(e.mem.u32(record + 0xc), 0x3000);
        assert_eq!(e.mem.u16(record + 0x10), 9);
        // The first atom moved and the instance has no record.
        e.mem.set_u32(bare + 0x10, first);
        e.call(0x00d2_75a0, &args![bare, first, 0x4000u32, 5u32]);
        assert_eq!(e.mem.u32(bare + 0x10), 0x4000);
        // A later atom moved: the atom before it is rewritten, the record is untouched.
        let second = atom(&mut e, 0x1a, 0);
        let third = atom(&mut e, 0x1a, 0);
        e.mem.set_u32(first + 0x14, second);
        e.mem.set_u32(second + 0x14, third);
        e.mem.set_u32(instance + 0x10, first);
        e.call(0x00d2_75a0, &args![instance, third, 0x5000u32, 6u32]);
        assert_eq!(e.mem.u32(second + 0x14), 0x5000);
        assert_eq!(e.mem.u16(second + 0x12), 6);
        assert_eq!(e.mem.u32(record + 0xc), 0x3000);
    }

    #[test]
    fn record_search_xors_the_body_words_with_the_body() {
        let mut e = engine();
        let (a, c) = (0x1000u32, 0x3000u32);
        let body_a = e.mem.alloc(0x100);
        let body_b = e.mem.alloc(0x100);
        // Neither array matches yet.
        assert_eq!(e.call(0x00d2_7610, &args![body_a, body_b]).u32(), 0);
        let records = e.mem.alloc(0x60);
        e.mem.set_u16(body_b + 0xb0, 2);
        e.mem.set_u32(body_b + 0xac, records);
        // Record 0 joins other bodies; record 1 is found from the second body: its words
        // xor with `body_b` to `body_a`.
        e.mem.set_u32(records, 0x77);
        e.mem.set_u32(records + 4, a);
        e.mem.set_u32(records + 8, c);
        e.mem.set_u32(records + 0x30, 0x99);
        e.mem.set_u32(records + 0x34, body_b);
        e.mem.set_u32(records + 0x38, body_a);
        assert_eq!(e.call(0x00d2_7610, &args![body_a, body_b]).u32(), 0x99);
        // Asking from the other side finds it in the first search.
        assert_eq!(e.call(0x00d2_7610, &args![body_b, body_a]).u32(), 0x99);
        // An empty count finds nothing even with a record present.
        e.mem.set_u16(body_b + 0xb0, 0);
        assert_eq!(e.call(0x00d2_7610, &args![body_a, body_b]).u32(), 0);
    }

    #[test]
    fn find_modifier_stops_at_the_end_of_the_chain() {
        let mut e = engine();
        let instance = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x1au32]).u32(), 0);
        let first = atom(&mut e, 0x19, 0);
        let second = atom(&mut e, 0x1a, 0);
        let third = atom(&mut e, 0x1b, 0);
        chain(&mut e, instance, &[first, second, third]);
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x19u32]).u32(), first);
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x1au32]).u32(), second);
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x1bu32]).u32(), third);
        // The end atom (type 0x10) is never compared.
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x10u32]).u32(), 0);
        assert_eq!(e.call(0x00d2_77c0, &args![instance, 0x1cu32]).u32(), 0);
    }

    #[test]
    fn last_modifier_is_the_one_before_the_end_atom() {
        let mut e = engine();
        let instance = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00d2_77f0, &args![instance]).u32(), 0);
        let first = atom(&mut e, 0x19, 0);
        chain(&mut e, instance, &[first]);
        assert_eq!(e.call(0x00d2_77f0, &args![instance]).u32(), first);
        let second = atom(&mut e, 0x1a, 0);
        let third = atom(&mut e, 0x1b, 0);
        chain(&mut e, instance, &[first, second, third]);
        assert_eq!(e.call(0x00d2_77f0, &args![instance]).u32(), third);
    }

    /// An object whose vtable has `hooks` at the given byte offsets.
    fn object_with(e: &mut Engine, hooks: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x80);
        for &(slot, target) in hooks {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// The owner and the shared doubles: the owner's virtual function at +0x18, the memory
    /// router and the allocator's free (slot 8), and the size table.
    fn world(e: &mut Engine, log: &Log) -> u32 {
        let owner = object_with(e, &[(0x18, OWNER_HOOK)]);
        log_double(e, log, OWNER_HOOK, 0);
        let allocator = object_with(e, &[(8, FREE_HOOK)]);
        log_double(e, log, FREE_HOOK, 0);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(e, log, MEMORY_ROUTER, router);
        for index in 0..8u32 {
            e.mem.set_u16(SIZE_TABLE + 2 * index, 0x10 * index as u16);
        }
        owner
    }

    /// An instance with two bodies whose bytes at +0xD2 are `a` and `b`.
    fn instance_with_bodies(e: &mut Engine, a: u8, b: u8) -> u32 {
        let instance = e.mem.alloc(0x40);
        let body_a = e.mem.alloc(0x100);
        let body_b = e.mem.alloc(0x100);
        e.mem.set_u8(body_a + 0xd2, a);
        e.mem.set_u8(body_b + 0xd2, b);
        e.mem.set_u32(instance + 0x14, body_a);
        e.mem.set_u32(instance + 0x18, body_b);
        instance
    }

    #[test]
    fn remove_and_delete_unlinks_and_frees_the_atom() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let owner = world(&mut e, &log);
        let instance = instance_with_bodies(&mut e, 0, 0);
        let first = atom(&mut e, 0x1a, 0x30);
        let second = atom(&mut e, 0x1b, 0x40);
        let third = atom(&mut e, 0x1c, 0x50);
        chain(&mut e, instance, &[first, second, third]);
        // Not in the chain: only the owner's function ran.
        e.call(0x00d2_7690, &args![instance, owner, 0x1du32]);
        assert_eq!(calls(&log, OWNER_HOOK).len(), 1);
        assert!(calls(&log, FREE_HOOK).is_empty());
        // A later atom of the chain is unlinked, its size is passed on.
        let after_third = e.mem.u32(third + 0x14);
        e.call(0x00d2_7690, &args![instance, owner, 0x1cu32]);
        assert_eq!(freed(&log), vec![vec![third, 0x50]]);
        assert_eq!(e.mem.u32(second + 0x14), after_third);
        assert_eq!(e.mem.u16(second + 0x12), 0);
        // The first atom, without a record: the instance points at the second one.
        e.call(0x00d2_7690, &args![instance, owner, 0x1au32]);
        assert_eq!(e.mem.u32(instance + 0x10), second);
        assert_eq!(freed(&log).last().unwrap(), &vec![first, 0x30]);
        // The last modifier, now first: the next atom is of a type below 0x17, so 0.
        e.call(0x00d2_7690, &args![instance, owner, 0x1bu32]);
        assert_eq!(e.mem.u32(instance + 0x10), 0);
        // An empty instance only runs the owner's function.
        let before = calls(&log, FREE_HOOK).len();
        e.call(0x00d2_7690, &args![instance, owner, 0x1bu32]);
        assert_eq!(calls(&log, FREE_HOOK).len(), before);
        assert_eq!(calls(&log, OWNER_HOOK).len(), 5);
        // A chain of one atom of another type is left alone.
        let lone = atom(&mut e, 0x1d, 0x20);
        chain(&mut e, instance, &[lone]);
        e.call(0x00d2_7690, &args![instance, owner, 0x1eu32]);
        assert_eq!(calls(&log, FREE_HOOK).len(), before);
    }

    #[test]
    fn remove_and_delete_accounts_the_modifier_data_of_an_instance_with_a_record() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let owner = world(&mut e, &log);
        e.mem.set_u32(owner + 0xc, 100);
        let instance = instance_with_bodies(&mut e, 0x05, 0x02);
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record, 0xabcd);
        e.mem.set_u16(record + 0x14, 0x200);
        e.mem.set_u32(instance + 0x30, record);
        let first = atom(&mut e, 0x1a, 0x30);
        let second = atom(&mut e, 0x1b, 0x40);
        chain(&mut e, instance, &[first, second]);
        // The data function leaves the flag byte 3; bodies 5 | 2 = 7; 3 & 7 = 3.
        let flag_log = Rc::clone(&log);
        e.register_double(ATOM_ADD_MODIFIER_DATA, move |e, a| {
            flag_log
                .borrow_mut()
                .push((ATOM_ADD_MODIFIER_DATA, a.to_vec()));
            assert_eq!(e.mem.u8(a[2]), 0);
            e.mem.set_u8(a[2], 3);
            Ret::default()
        });
        log_double(&mut e, &log, REMOVE_CONSTRAINT_INFO, 0);
        e.call(0x00d2_7690, &args![instance, owner, 0x1au32]);
        assert_eq!(e.mem.u32(instance + 0x10), second);
        assert_eq!(e.mem.u32(record + 0xc), second);
        assert_eq!(e.mem.u16(record + 0x10), 0x40);
        // Table entry 3 is 0x30.
        assert_eq!(e.mem.u32(owner + 0xc), 100 + 0x30);
        assert_eq!(e.mem.u16(record + 0x14), 0x200 + 0x30);
        let data = calls(&log, ATOM_ADD_MODIFIER_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0][0], first);
        let info = calls(&log, REMOVE_CONSTRAINT_INFO);
        assert_eq!(info, vec![vec![owner, 0xabcd, data[0][1]]]);
        assert_eq!(freed(&log), vec![vec![first, 0x30]]);
    }

    #[test]
    fn add_modifier_places_the_atom_by_type() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let owner = world(&mut e, &log);
        e.mem.set_u32(owner + 0xc, 1000);
        let instance = instance_with_bodies(&mut e, 0x06, 0x03);
        let record = e.mem.alloc(0x40);
        e.mem.set_u16(record + 0x14, 0x400);
        e.mem.set_u32(instance + 0x30, record);
        let first = atom(&mut e, 0x19, 0x30);
        let last = atom(&mut e, 0x1c, 0x40);
        chain(&mut e, instance, &[first, last]);
        e.mem.set_u32(record + 0xc, first);
        e.mem.set_u16(record + 0x10, 0x30);
        e.register_double(ATOM_ADD_MODIFIER_DATA, |e, a| {
            e.mem.set_u8(a[2], 0x0f);
            Ret {
                eax: 0x1234_5610,
                ..Ret::default()
            }
        });
        log_double(&mut e, &log, ADD_CONSTRAINT_INFO, 0);
        // Type 0x1a goes after the last atom; bodies 6 | 3 = 7, flag 0x0f & 7 = 7: 0x70.
        let new = atom(&mut e, 0x1a, 0x20);
        e.call(0x00d2_7960, &args![instance, owner, new]);
        assert_eq!(e.mem.u32(last + 0x14), new);
        assert_eq!(e.mem.u16(last + 0x12), 0x20);
        assert_eq!(e.mem.u16(new + 0x12), 0);
        assert_eq!(e.mem.u32(owner + 0xc), 1000 - 0x70);
        assert_eq!(e.mem.u16(record + 0x14), 0x400 - 0x70);
        assert_eq!(e.mem.u8(record + 0x12), 0x10);
        let added = calls(&log, ADD_CONSTRAINT_INFO);
        assert_eq!(added.len(), 1);
        assert_eq!(&added[0][..2], &[owner, instance]);
        // Any other atom becomes the first one and takes the record's link.
        let other = atom(&mut e, 0x1d, 0x50);
        e.call(0x00d2_7960, &args![instance, owner, other]);
        assert_eq!(e.mem.u32(instance + 0x10), other);
        assert_eq!(e.mem.u32(other + 0x14), first);
        assert_eq!(e.mem.u16(other + 0x12), 0x30);
        assert_eq!(e.mem.u32(record + 0xc), other);
        assert_eq!(e.mem.u16(record + 0x10), 0x50);
        // A first atom of type 0x1b keeps its place: the new atom is linked after it.
        let pinned = atom(&mut e, 0x1b, 0x60);
        let tail = atom(&mut e, 0x1c, 0x70);
        chain(&mut e, instance, &[pinned, tail]);
        let after = atom(&mut e, 0x1e, 0x80);
        e.call(0x00d2_7960, &args![instance, owner, after]);
        assert_eq!(e.mem.u32(instance + 0x10), pinned);
        assert_eq!(e.mem.u32(pinned + 0x14), after);
        assert_eq!(e.mem.u32(after + 0x14), tail);
        assert_eq!(e.mem.u16(pinned + 0x12), 0x80);
        assert_eq!(e.mem.u16(after + 0x12), 0x70);
        assert_eq!(calls(&log, OWNER_HOOK).len(), 3);
    }

    #[test]
    fn add_modifier_without_a_record_asks_the_data_object_for_the_link() {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let owner = world(&mut e, &log);
        let instance = instance_with_bodies(&mut e, 0, 0);
        let data = object_with(&mut e, &[(0x3c, BUFFER_HOOK)]);
        e.mem.set_u32(instance + 0xc, data);
        e.register_double(BUFFER_HOOK, |e, a| {
            e.mem.set_u32(a[1] + 0x10, 0x1234);
            e.mem.set_u16(a[1] + 0x14, 0x56);
            Ret::default()
        });
        let new = atom(&mut e, 0x1d, 0x50);
        e.call(0x00d2_7960, &args![instance, owner, new]);
        assert_eq!(e.mem.u32(instance + 0x10), new);
        assert_eq!(e.mem.u32(new + 0x14), 0x1234);
        assert_eq!(e.mem.u16(new + 0x12), 0x56);
        assert!(calls(&log, ADD_CONSTRAINT_INFO).is_empty());
    }
}
