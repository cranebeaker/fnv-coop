//! `Havok SDK/hkpworldagentutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The agent storage the functions walk (offsets, from the code): a block at +0x5c holds an
//! array of pointers to 0x200-byte chunks of agent entries, +0x60 is the number of chunks
//! and +0x58 the used size of the last chunk; each entry's byte at +3 is its size.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgentNnMachine_WarpTimeInAgent` (Xbox PDB, `00cfdc90`, cdecl `(entry, float, float,
/// word)`).
const WARP_TIME_IN_AGENT: u32 = 0x00cf_dc90;
/// `hkpWorldAgentUtil::removeAgent` (Xbox PDB, `00cc10b0`, cdecl `(agent)`).
const REMOVE_AGENT: u32 = 0x00cc_10b0;
/// `hkAgentNnMachine_InvalidateTimInAgent` (Xbox PDB, `00cfdbe0`, cdecl `(agent, tim)`).
const INVALIDATE_TIM_IN_AGENT: u32 = 0x00cf_dbe0;

// Translated from 00cc0e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::getIslandFromAgentEntry` (Xbox PDB, cdecl): of the two blocks `first`
/// and `second`, searches the one with the smaller (or equal, for `first`) chunk count for a
/// chunk `[start, start + 0x200)` that contains `entry`, and returns `first` when the answer
/// "found" equals "the search was done in `first`", else `second`.
pub fn hkp_world_agent_util_get_island_from_agent_entry(
    e: &mut Engine,
    entry: u32,
    first: Ptr,
    second: Ptr,
) -> Ptr {
    let searched_first = e.mem.i32(first.addr() + 0x60) <= e.mem.i32(second.addr() + 0x60);
    let searched = if searched_first { first } else { second };
    let count = e.mem.i32(searched.addr() + 0x60);
    let mut found = false;
    if count > 0 {
        let chunks = e.mem.u32(searched.addr() + 0x5c);
        for index in 0..count as u32 {
            let start = e.mem.u32(chunks + 4 * index);
            if start <= entry && entry < start.wrapping_add(0x200) {
                found = true;
                break;
            }
        }
    }
    if found == searched_first {
        first
    } else {
        second
    }
}

// Translated from 00cc0ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::warpTime` (Xbox PDB, cdecl): walks every agent entry of every chunk
/// (a full 0x200 bytes, except the last chunk, which holds the size at +0x58) and calls
/// `hkAgentNnMachine_WarpTimeInAgent(entry, first, second, third)` on each, moving on by the
/// entry's size byte at +3. The two floats go through the x87 (a signalling NaN comes out
/// quiet).
pub fn hkp_world_agent_util_warp_time(
    e: &mut Engine,
    island: Ptr,
    first: f32,
    second: f32,
    third: u32,
) {
    let mut chunk = 0u32;
    if e.mem.i32(island.addr() + 0x60) > 0 {
        loop {
            let chunks = e.mem.u32(island.addr() + 0x5c);
            let mut entry = e.mem.u32(chunks + 4 * chunk);
            chunk += 1;
            let end = if chunk as i32 == e.mem.i32(island.addr() + 0x60) {
                e.mem.u32(island.addr() + 0x58).wrapping_add(entry)
            } else {
                entry.wrapping_add(0x200)
            };
            while entry < end {
                e.call(
                    WARP_TIME_IN_AGENT,
                    &args![entry, x87_quiet(first), x87_quiet(second), third],
                );
                entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
            }
            if chunk as i32 >= e.mem.i32(island.addr() + 0x60) {
                break;
            }
        }
    }
}

// Translated from 00cc1160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::removeAgentAndItsToiEvents` (Xbox PDB, cdecl): finds the object
/// `[[[agent + 0x10] + (signed byte at +0x10) + 0xCC] + 0x18] + 8`, calls its virtual slot
/// `0x28` with the agent, then `removeAgent` (`00cc10b0`).
pub fn hkp_world_agent_util_remove_agent_and_its_toi_events(e: &mut Engine, agent: Ptr) {
    let entity = e.mem.u32(agent.addr() + 0x10);
    let offset = i32::from(e.mem.i8(entity + 0x10)) as u32;
    let holder = e.mem.u32(entity.wrapping_add(offset).wrapping_add(0xcc));
    let inner = e.mem.u32(holder + 0x18);
    let object = e.mem.u32(inner + 8);
    e.vcall(object, 0x28, &args![agent]);
    e.call(REMOVE_AGENT, &args![agent]);
}

// Translated from 00cc0ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::invalidateTim` (Xbox PDB, cdecl): for every 8-byte element of the array
/// at `container + 0x60` (count at `+0x64`, read again after each call) calls
/// `hkAgentNnMachine_InvalidateTimInAgent(first word of the element, tim)`.
pub fn hkp_world_agent_util_invalidate_tim(e: &mut Engine, container: Ptr, tim: u32) {
    let c = container.addr();
    let mut index = 0u32;
    if e.mem.i32(c + 0x64) > 0 {
        loop {
            let elements = e.mem.u32(c + 0x60);
            let agent = e.mem.u32(elements.wrapping_add(index.wrapping_mul(8)));
            e.call(INVALIDATE_TIM_IN_AGENT, &args![agent, tim]);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(c + 0x64) {
                break;
            }
        }
    }
}

// Translated from 00cc0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::addAgent` (Xbox PDB, cdecl, three words `collidable_a`, `collidable_b`,
/// `input`; the roles of the fields are descriptions, not confirmed). The owner of each
/// collidable is the collidable plus the signed byte at its `+0x10`; an owner's word at `+0xcc`
/// is its group object (a 16-bit word `0xffff` at the group's `+0x20` marks the fixed one), the
/// byte at `+0xe8` its kind (5 is skipped by the first step) and the signed byte at `+0x8c`
/// picks the agent function.
///
/// 1. When neither owner's kind is 5 and their groups differ, `00cb5570` runs (cdecl) with
///    `([owner_a + 8], owner_a, owner_b)`; the groups are read again afterwards.
/// 2. The group that is not the fixed one is chosen: `owner_a`'s, unless it is fixed and
///    `owner_b`'s is not.
/// 3. A collidable whose first word points to a shape with type word `0xf` (at the shape's
///    `+0xc`) is replaced by the answer of `00cfe1d0` (cdecl, `(collidable, buffer, buffer, 4,
///    flag byte)` with two buffers in the 0x630-byte frame and the one flag byte, cleared first).
/// 4. `00cfdf60` (cdecl, `(first, second, input, out, swap)`) answers into two frame words; when
///    the second (`swap`) is non-zero the two collidables and the two replaced values exchange
///    their places.
/// 5. The object at `[[input] + 4 * (8 * byte_a + byte_b) + 0xc]` (the bytes at `+0x8c` of the
///    owners, in the order of the arguments before any exchange) is asked through its slot
///    `0xc` with `(first, second, input)` for a type number.
/// 6. `00cfe8a0` (cdecl, nine words) runs with `(chosen group + 0x58, first, first value, second,
///    second value, the flag word, the first output word, input, type)` and its answer is the
///    answer of the function (the new agent entry).
pub fn hkp_world_agent_util_add_agent(
    e: &mut Engine,
    collidable_a: Ptr,
    collidable_b: Ptr,
    input: Ptr,
) -> u32 {
    let (mut first, mut second, input) = (collidable_a.addr(), collidable_b.addr(), input.addr());
    let owner_a = first.wrapping_add(i32::from(e.mem.i8(first.wrapping_add(0x10))) as u32);
    let owner_b = second.wrapping_add(i32::from(e.mem.i8(second.wrapping_add(0x10))) as u32);
    if e.mem.u8(owner_a.wrapping_add(0xe8)) != 5
        && e.mem.u8(owner_b.wrapping_add(0xe8)) != 5
        && e.mem.u32(owner_a.wrapping_add(0xcc)) != e.mem.u32(owner_b.wrapping_add(0xcc))
    {
        let world = e.mem.u32(owner_a.wrapping_add(8));
        e.call(0x00cb_5570, &args![world, owner_a, owner_b]);
    }
    let group_a = e.mem.u32(owner_a.wrapping_add(0xcc));
    let group_b = e.mem.u32(owner_b.wrapping_add(0xcc));
    let mut chosen = group_a;
    if e.mem.u16(group_a.wrapping_add(0x20)) == 0xffff {
        chosen = group_b;
        if e.mem.u16(group_b.wrapping_add(0x20)) == 0xffff {
            chosen = group_a;
        }
    }
    e.with_stack(0x630, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u8(frame.wrapping_add(0x18), 0);
        let mut first_value = first;
        let mut second_value = second;
        let shape = e.mem.u32(first);
        if e.mem.u32(shape.wrapping_add(0xc)) == 0xf {
            first_value = e
                .call(
                    0x00cf_e1d0,
                    &args![
                        first,
                        frame.wrapping_add(0x30),
                        frame.wrapping_add(0xb0),
                        4u32,
                        frame.wrapping_add(0x18)
                    ],
                )
                .u32();
        }
        let shape = e.mem.u32(second);
        if e.mem.u32(shape.wrapping_add(0xc)) == 0xf {
            second_value = e
                .call(
                    0x00cf_e1d0,
                    &args![
                        second,
                        frame.wrapping_add(0x70),
                        frame.wrapping_add(0x370),
                        4u32,
                        frame.wrapping_add(0x18)
                    ],
                )
                .u32();
        }
        e.call(
            0x00cf_df60,
            &args![
                first_value,
                second_value,
                input,
                frame.wrapping_add(0x2c),
                frame.wrapping_add(0x28)
            ],
        );
        if e.mem.u32(frame.wrapping_add(0x28)) != 0 {
            std::mem::swap(&mut first, &mut second);
            std::mem::swap(&mut first_value, &mut second_value);
        }
        let byte_a = i32::from(e.mem.i8(owner_a.wrapping_add(0x8c)));
        let byte_b = i32::from(e.mem.i8(owner_b.wrapping_add(0x8c)));
        let index = byte_b.wrapping_add(byte_a.wrapping_mul(8)) as u32;
        let table = e.mem.u32(input);
        let object = e
            .mem
            .u32(table.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xc));
        let type_number = e.vcall(object, 0xc, &args![first, second, input]).u32();
        let output = e.mem.u32(frame.wrapping_add(0x2c));
        let flag = e.mem.u32(frame.wrapping_add(0x18));
        e.call(
            0x00cf_e8a0,
            &args![
                chosen.wrapping_add(0x58),
                first,
                first_value,
                second,
                second_value,
                flag,
                output,
                input,
                type_number
            ],
        )
        .u32()
    })
}

// Translated from 00cc10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldAgentUtil::removeAgent` (Xbox PDB, cdecl, one word `entry`; the roles of the fields
/// are descriptions, not confirmed). The entry's words at `+0x10` and `+0x14` are its two
/// collidables; an owner is a collidable plus the signed byte at its `+0x10`, its group object
/// the word at the owner's `+0xcc`, its kind the byte at `+0xe8`.
///
/// - Equal groups: the byte at the group's `+0x25` gets bit 1 cleared and bit 0 set; that group
///   is chosen.
/// - Otherwise, when the first owner's kind is 5 the second group is chosen, when the second
///   owner's kind is 5 the first group; with neither, `getIslandFromAgentEntry` (`00cc0e20`)
///   picks the group for `(entry, group_a, group_b)`, and the byte at `+0x25` of group A and of
///   the second owner's group (read again) get bit 1 cleared and bit 0 set.
///
/// Then `00cfea30` runs (cdecl) with `(chosen + 0x58, entry, [[chosen + 0x18] + 0x80], chosen)`
/// and the function ends in a tail jump to the virtual slot `0x24` of the object at
/// `[entry + 8]`; the entry stays the first stack word then, so it is passed on. Answers what
/// that answers.
pub fn hkp_world_agent_util_remove_agent(e: &mut Engine, entry: Ptr) -> u32 {
    let entry = entry.addr();
    let collidable_a = e.mem.u32(entry.wrapping_add(0x10));
    let owner_a =
        collidable_a.wrapping_add(i32::from(e.mem.i8(collidable_a.wrapping_add(0x10))) as u32);
    let collidable_b = e.mem.u32(entry.wrapping_add(0x14));
    let group_a = e.mem.u32(owner_a.wrapping_add(0xcc));
    let owner_b =
        collidable_b.wrapping_add(i32::from(e.mem.i8(collidable_b.wrapping_add(0x10))) as u32);
    let group_b = e.mem.u32(owner_b.wrapping_add(0xcc));
    let chosen = if group_a == group_b {
        let flags = e.mem.u8(group_a.wrapping_add(0x25));
        e.mem.set_u8(group_a.wrapping_add(0x25), (flags & 0xfd) | 1);
        group_a
    } else if e.mem.u8(owner_a.wrapping_add(0xe8)) == 5 {
        group_b
    } else if e.mem.u8(owner_b.wrapping_add(0xe8)) == 5 {
        group_a
    } else {
        let island = hkp_world_agent_util_get_island_from_agent_entry(
            e,
            entry,
            Ptr::new(group_a),
            Ptr::new(group_b),
        );
        let flags = e.mem.u8(group_a.wrapping_add(0x25));
        e.mem.set_u8(group_a.wrapping_add(0x25), (flags & 0xfd) | 1);
        let second_group = e.mem.u32(owner_b.wrapping_add(0xcc));
        let flags = e.mem.u8(second_group.wrapping_add(0x25));
        e.mem
            .set_u8(second_group.wrapping_add(0x25), (flags & 0xfd) | 1);
        island.addr()
    };
    let holder = e.mem.u32(chosen.wrapping_add(0x18));
    let value = e.mem.u32(holder.wrapping_add(0x80));
    let manager = e.mem.u32(entry.wrapping_add(8));
    e.call(
        0x00cf_ea30,
        &args![chosen.wrapping_add(0x58), entry, value, chosen],
    );
    e.vcall(manager, 0x24, &args![entry]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc0f40, hkp_world_agent_util_add_agent(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00cc10b0, hkp_world_agent_util_remove_agent(Ptr) -> u32),
        entry!(
            0x00cc_0e20,
            hkp_world_agent_util_get_island_from_agent_entry(u32, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cc_0ea0, hkp_world_agent_util_invalidate_tim(Ptr, u32)),
        entry!(
            0x00cc_0ed0,
            hkp_world_agent_util_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(
            0x00cc_1160,
            hkp_world_agent_util_remove_agent_and_its_toi_events(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What `addAgent` and `removeAgent` read: two collidables inside owner blocks (offset bytes
    /// `0xa0` and `0xb0`, that is -0x60 and -0x50), the owners' kind, table-index and group
    /// words, two groups, an input with the table object and the manager.
    struct Setup00cc0f40 {
        e: Engine,
        a: u32,
        b: u32,
        owner_a: u32,
        owner_b: u32,
        group_a: u32,
        group_b: u32,
        input: u32,
        object: u32,
    }

    fn setup_00cc0f40() -> Setup00cc0f40 {
        let mut e = Engine::new();
        let owner_a = e.mem.alloc(0x200);
        let owner_b = e.mem.alloc(0x200);
        let a = owner_a + 0x60;
        let b = owner_b + 0x50;
        e.mem.set_i8(a + 0x10, -0x60);
        e.mem.set_i8(b + 0x10, -0x50);
        let group_a = e.mem.alloc(0x100);
        let group_b = e.mem.alloc(0x100);
        e.mem.set_u16(group_a + 0x20, 3);
        e.mem.set_u16(group_b + 0x20, 4);
        for (owner, group, world) in [(owner_a, group_a, 0x1111), (owner_b, group_b, 0x2222)] {
            e.mem.set_u32(owner + 8, world);
            e.mem.set_u32(owner + 0xcc, group);
        }
        e.mem.set_i8(owner_a + 0x8c, 2);
        e.mem.set_i8(owner_b + 0x8c, 3);
        for collidable in [a, b] {
            let shape = e.mem.alloc(0x20);
            e.mem.set_u32(shape + 0xc, 7);
            e.mem.set_u32(collidable, shape);
        }
        // The table object for index 2 * 8 + 3 = 19, with a logging slot 0xc.
        e.put_vtable(0x0900_0000, &[0, 0, 0, 0x0900_0100]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0900_0000);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 19 * 4 + 0xc, object);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, table);
        e.register(0x0900_0100, |_, _| Ret {
            eax: 0x66,
            ..Ret::default()
        });
        e.register(0x00cb_5570, |_, _| Ret::default());
        e.register(0x00cf_df60, |_, _| Ret::default());
        e.register(0x00cf_e8a0, |_, _| Ret {
            eax: 0xe47,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        Setup00cc0f40 {
            e,
            a,
            b,
            owner_a,
            owner_b,
            group_a,
            group_b,
            input,
            object,
        }
    }

    fn logged_00cc0f40(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn add_agent_merges_different_groups_and_creates_the_agent() {
        let mut s = setup_00cc0f40();
        let (a, b, input) = (s.a, s.b, s.input);
        let answer = s.e.call(0x00cc_0f40, &args![a, b, input]).u32();
        assert_eq!(answer, 0xe47);
        assert_eq!(
            logged_00cc0f40(&s.e, 0x00cb_5570),
            vec![vec![0x1111, s.owner_a, s.owner_b]]
        );
        assert!(logged_00cc0f40(&s.e, 0x00cf_e1d0).is_empty());
        let process = logged_00cc0f40(&s.e, 0x00cf_df60);
        assert_eq!(process.len(), 1);
        assert_eq!(process[0][..3], [a, b, input]);
        // The two outputs are words of the frame, 4 apart, the second one below.
        let frame_words = (process[0][3], process[0][4]);
        assert_eq!(frame_words.0, frame_words.1 + 4);
        assert_eq!(
            logged_00cc0f40(&s.e, 0x0900_0100),
            vec![vec![s.object, a, b, input]]
        );
        // The first group is chosen (its id word is not 0xffff); no output was written, the flag
        // word is clear.
        assert_eq!(
            logged_00cc0f40(&s.e, 0x00cf_e8a0),
            vec![vec![s.group_a + 0x58, a, a, b, b, 0, 0, input, 0x66]]
        );
    }

    #[test]
    fn add_agent_skips_the_merge_for_equal_groups_and_for_kind_5() {
        let mut s = setup_00cc0f40();
        let (a, b, input) = (s.a, s.b, s.input);
        s.e.mem.set_u32(s.owner_b + 0xcc, s.group_a);
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        assert!(logged_00cc0f40(&s.e, 0x00cb_5570).is_empty());
        s.e.mem.set_u32(s.owner_b + 0xcc, s.group_b);
        s.e.mem.set_u8(s.owner_a + 0xe8, 5);
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        assert!(logged_00cc0f40(&s.e, 0x00cb_5570).is_empty());
        s.e.mem.set_u8(s.owner_a + 0xe8, 0);
        s.e.mem.set_u8(s.owner_b + 0xe8, 5);
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        assert!(logged_00cc0f40(&s.e, 0x00cb_5570).is_empty());
    }

    #[test]
    fn add_agent_chooses_the_group_that_is_not_fixed() {
        let mut s = setup_00cc0f40();
        let (a, b, input) = (s.a, s.b, s.input);
        s.e.mem.set_u16(s.group_a + 0x20, 0xffff);
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        s.e.mem.set_u16(s.group_b + 0x20, 0xffff);
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        let created = logged_00cc0f40(&s.e, 0x00cf_e8a0);
        assert_eq!(created[0][0], s.group_b + 0x58);
        assert_eq!(created[1][0], s.group_a + 0x58);
    }

    #[test]
    fn add_agent_exchanges_the_collidables_when_the_second_output_says_so() {
        let mut s = setup_00cc0f40();
        let (a, b, input) = (s.a, s.b, s.input);
        s.e.register_double(0x00cf_df60, |e, args| {
            e.mem.set_u32(args[3], 0x7001);
            e.mem.set_u32(args[4], 1);
            Ret::default()
        });
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        // The table index still uses the owners in the order of the arguments, the object is
        // asked with the exchanged collidables.
        assert_eq!(
            logged_00cc0f40(&s.e, 0x0900_0100),
            vec![vec![s.object, b, a, input]]
        );
        assert_eq!(
            logged_00cc0f40(&s.e, 0x00cf_e8a0),
            vec![vec![s.group_a + 0x58, b, b, a, a, 0, 0x7001, input, 0x66]]
        );
    }

    #[test]
    fn add_agent_replaces_a_collidable_whose_shape_has_type_0xf() {
        let mut s = setup_00cc0f40();
        let (a, b, input) = (s.a, s.b, s.input);
        let shape_a = s.e.mem.u32(a);
        let shape_b = s.e.mem.u32(b);
        s.e.mem.set_u32(shape_a + 0xc, 0xf);
        s.e.mem.set_u32(shape_b + 0xc, 0xf);
        s.e.register_double(0x00cf_e1d0, |e, args| {
            e.mem.set_u8(args[4], 1);
            Ret {
                eax: args[0] + 0x1000,
                ..Ret::default()
            }
        });
        s.e.call(0x00cc_0f40, &args![a, b, input]);
        let replaced = logged_00cc0f40(&s.e, 0x00cf_e1d0);
        assert_eq!(replaced.len(), 2);
        // Two buffers 0x80 and 0x300 bytes long in the frame; the same flag byte, 0x18 in it.
        assert_eq!(replaced[0][0], a);
        assert_eq!(replaced[0][2], replaced[0][1] + 0x80);
        assert_eq!(replaced[0][3], 4);
        assert_eq!(replaced[0][4], replaced[0][1] - 0x18);
        assert_eq!(replaced[1][0], b);
        assert_eq!(replaced[1][1], replaced[0][1] + 0x40);
        assert_eq!(replaced[1][2], replaced[1][1] + 0x300);
        assert_eq!(replaced[1][4], replaced[0][4]);
        let process = logged_00cc0f40(&s.e, 0x00cf_df60);
        assert_eq!(process[0][..3], [a + 0x1000, b + 0x1000, input]);
        // The flag byte the helper set reaches the last call as a word.
        let created = logged_00cc0f40(&s.e, 0x00cf_e8a0);
        assert_eq!(created[0][1..6], [a, a + 0x1000, b, b + 0x1000, 1]);
    }

    /// A removal setup: the entry's collidables are `a` and `b` of the add setup, its manager
    /// is an object whose slot 0x24 answers 0x4242.
    fn removal_00cc10b0() -> (Setup00cc0f40, u32, u32) {
        let mut s = setup_00cc0f40();
        let entry = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(entry + 0x10, s.a);
        s.e.mem.set_u32(entry + 0x14, s.b);
        let mut slots = vec![0u32; 10];
        slots[9] = 0x0900_0200;
        s.e.put_vtable(0x0900_1000, &slots);
        let manager = s.e.mem.alloc(8);
        s.e.mem.set_u32(manager, 0x0900_1000);
        s.e.mem.set_u32(entry + 8, manager);
        s.e.register(0x0900_0200, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        s.e.register(0x00cf_ea30, |_, _| Ret::default());
        // The chosen group reaches `[[group + 0x18] + 0x80]`.
        for group in [s.group_a, s.group_b] {
            let holder = s.e.mem.alloc(0x100);
            s.e.mem.set_u32(holder + 0x80, group ^ 0x5a5a);
            s.e.mem.set_u32(group + 0x18, holder);
        }
        (s, entry, manager)
    }

    #[test]
    fn remove_agent_with_equal_groups_marks_the_group_and_runs_the_manager() {
        let (mut s, entry, manager) = removal_00cc10b0();
        s.e.mem.set_u32(s.owner_b + 0xcc, s.group_a);
        s.e.mem.set_u8(s.group_a + 0x25, 0xfe);
        let answer = s.e.call(0x00cc_10b0, &args![entry]).u32();
        assert_eq!(answer, 0x4242);
        assert_eq!(s.e.mem.u8(s.group_a + 0x25), 0xfd);
        assert_eq!(
            logged_00cc0f40(&s.e, 0x00cf_ea30),
            vec![vec![s.group_a + 0x58, entry, s.group_a ^ 0x5a5a, s.group_a]]
        );
        assert_eq!(
            logged_00cc0f40(&s.e, 0x0900_0200),
            vec![vec![manager, entry]]
        );
    }

    #[test]
    fn remove_agent_picks_the_other_group_for_kind_5_owners() {
        let (mut s, entry, _) = removal_00cc10b0();
        s.e.mem.set_u8(s.owner_a + 0xe8, 5);
        s.e.call(0x00cc_10b0, &args![entry]);
        s.e.mem.set_u8(s.owner_a + 0xe8, 0);
        s.e.mem.set_u8(s.owner_b + 0xe8, 5);
        s.e.call(0x00cc_10b0, &args![entry]);
        let runs = logged_00cc0f40(&s.e, 0x00cf_ea30);
        assert_eq!(runs[0][3], s.group_b);
        assert_eq!(runs[1][3], s.group_a);
        // Neither group was marked.
        assert_eq!(s.e.mem.u8(s.group_a + 0x25), 0);
        assert_eq!(s.e.mem.u8(s.group_b + 0x25), 0);
    }

    #[test]
    fn remove_agent_marks_both_groups_and_asks_for_the_island() {
        let (mut s, entry, _) = removal_00cc10b0();
        s.e.mem.set_u8(s.group_a + 0x25, 0x02);
        s.e.mem.set_u8(s.group_b + 0x25, 0x06);
        // The entry lies in a chunk of group A: its block has one chunk, group B's has two.
        for (group, chunks) in [(s.group_a, vec![entry - 0x10]), (s.group_b, vec![0, 0x100])] {
            let array = s.e.mem.alloc(16);
            for (index, start) in chunks.iter().enumerate() {
                s.e.mem.set_u32(array + 4 * index as u32, *start);
            }
            s.e.mem.set_u32(group + 0x5c, array);
            s.e.mem.set_i32(group + 0x60, chunks.len() as i32);
        }
        s.e.call(0x00cc_10b0, &args![entry]);
        assert_eq!(s.e.mem.u8(s.group_a + 0x25), 0x01);
        assert_eq!(s.e.mem.u8(s.group_b + 0x25), 0x05);
        let runs = logged_00cc0f40(&s.e, 0x00cf_ea30);
        assert_eq!(runs[0][3], s.group_a);
    }

    /// A block with the given chunk start addresses.
    fn block(e: &mut Engine, chunks: &[u32]) -> u32 {
        let b = e.mem.alloc(0x80);
        let array = e.mem.alloc(4 * chunks.len().max(1) as u32);
        for (i, c) in chunks.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.mem.set_u32(b + 0x5c, array);
        e.mem.set_i32(b + 0x60, chunks.len() as i32);
        b
    }

    #[test]
    fn island_search_uses_the_block_with_fewer_chunks() {
        let mut e = Engine::new();
        let small = block(&mut e, &[0x1000]);
        let big = block(&mut e, &[0x5000, 0x6000, 0x7000]);
        let call = |e: &mut Engine, entry: u32, a: u32, b: u32| {
            e.call(0x00cc_0e20, &args![entry, a, b]).u32()
        };
        // first has fewer chunks and contains the entry: found == searched-in-first -> first
        assert_eq!(call(&mut e, 0x1100, small, big), small);
        // first has fewer chunks and does not contain it -> second
        assert_eq!(call(&mut e, 0x6100, small, big), big);
        // second has fewer chunks (searched in second): found there -> not first -> second
        assert_eq!(call(&mut e, 0x1100, big, small), small);
        // second has fewer chunks and does not contain it -> first
        assert_eq!(call(&mut e, 0x6100, big, small), big);
        // the chunk end is exclusive, the start inclusive
        assert_eq!(call(&mut e, 0x1200, small, big), big);
        assert_eq!(call(&mut e, 0x1000, small, big), small);
        // equal counts search the first block
        let other = block(&mut e, &[0x9000]);
        assert_eq!(call(&mut e, 0x9010, small, other), other);
        assert_eq!(call(&mut e, 0x1010, small, other), small);
    }

    #[test]
    fn island_search_with_no_chunks_finds_nothing() {
        let mut e = Engine::new();
        let empty = block(&mut e, &[]);
        let other = block(&mut e, &[0x1000]);
        // first (0 chunks) is searched and nothing found -> second
        let got = e.call(0x00cc_0e20, &args![0x1000u32, empty, other]).u32();
        assert_eq!(got, other);
    }

    #[test]
    fn warp_time_visits_every_entry_of_every_chunk() {
        let mut e = Engine::new();
        let first_chunk = e.mem.alloc(0x200);
        let last_chunk = e.mem.alloc(0x200);
        // first chunk: 0x80-byte entries (4 of them fill 0x200); last chunk: used 0x30 bytes
        for i in 0..4 {
            e.mem.set_u8(first_chunk + 0x80 * i + 3, 0x80);
        }
        e.mem.set_u8(last_chunk + 3, 0x10);
        e.mem.set_u8(last_chunk + 0x10 + 3, 0x20);
        let island = block(&mut e, &[first_chunk, last_chunk]);
        e.mem.set_u32(island + 0x58, 0x30);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(WARP_TIME_IN_AGENT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(
            0x00cc_0ed0,
            &args![island, f32::from_bits(0x7f80_0001), 2.5f32, 0x77u32],
        );
        let seen = seen.borrow();
        let entries: Vec<u32> = seen.iter().map(|c| c[0]).collect();
        let want: Vec<u32> = (0..4)
            .map(|i| first_chunk + 0x80 * i)
            .chain([last_chunk, last_chunk + 0x10])
            .collect();
        assert_eq!(entries, want);
        assert_eq!(seen[0][1], 0x7fc0_0001);
        assert_eq!(seen[0][2], 2.5f32.to_bits());
        assert_eq!(seen[0][3], 0x77);
    }

    #[test]
    fn warp_time_without_chunks_calls_nothing() {
        let mut e = Engine::new();
        let island = block(&mut e, &[]);
        e.register(WARP_TIME_IN_AGENT, |_, _| panic!("no entries"));
        e.call(0x00cc_0ed0, &args![island, 1.0f32, 2.0f32, 3u32]);
    }

    #[test]
    fn remove_agent_calls_the_toi_slot_then_remove_agent() {
        let mut e = Engine::new();
        const VTABLE: u32 = 0x0a00_0000;
        const SLOT: u32 = 0x0a10_0001;
        let mut slots = vec![0u32; 0x30 / 4];
        slots[0x28 / 4] = SLOT;
        e.put_vtable(VTABLE, &slots);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, VTABLE);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(inner + 8, object);
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 0x18, inner);
        let entity = e.mem.alloc(0x200);
        e.mem.set_i8(entity + 0x10, 4);
        e.mem.set_u32(entity + 4 + 0xcc, holder);
        let agent = e.mem.alloc(0x20);
        e.mem.set_u32(agent + 0x10, entity);
        e.register(SLOT, |_, _| Ret::default());
        e.register(REMOVE_AGENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cc_1160, &args![agent]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| c.0 == SLOT || c.0 == REMOVE_AGENT)
            .collect();
        assert_eq!(
            order,
            vec![(SLOT, vec![object, agent]), (REMOVE_AGENT, vec![agent])]
        );
    }

    #[test]
    fn invalidate_tim_visits_every_element_and_rereads_the_count() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x80);
        let elements = e.mem.alloc(0x20);
        for (i, agent) in [0x11u32, 0x22, 0x33].iter().enumerate() {
            e.mem.set_u32(elements + 8 * i as u32, *agent);
            e.mem.set_u32(elements + 8 * i as u32 + 4, 0xdead);
        }
        e.mem.set_u32(container + 0x60, elements);
        e.mem.set_u32(container + 0x64, 3);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(INVALIDATE_TIM_IN_AGENT, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            if a[0] == 0x22 {
                e.mem.set_u32(container + 0x64, 2);
            }
            Ret::default()
        });
        e.call(0x00cc_0ea0, &args![container, 0x99u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x11, 0x99], vec![0x22, 0x99]]);
    }

    #[test]
    fn invalidate_tim_with_no_elements_calls_nothing() {
        let mut e = Engine::new();
        let container = e.mem.alloc(0x80);
        e.mem.set_u32(container + 0x64, 0);
        e.register(INVALIDATE_TIM_IN_AGENT, |_, _| panic!("no elements"));
        e.call(0x00cc_0ea0, &args![container, 1u32]);
        e.mem.set_u32(container + 0x64, 0xffff_ffff);
        e.call(0x00cc_0ea0, &args![container, 1u32]);
    }
}
