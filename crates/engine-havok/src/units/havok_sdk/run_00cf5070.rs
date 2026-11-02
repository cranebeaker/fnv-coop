//! `Havok SDK/run_00cf5070` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These are `hkpListShape` methods (the vtable at `010cc3d8` and the `hkpShapeContainer`
//! table before it, `010cc3b8`, hold them at the slots of the Xbox PDB's class). The list
//! shape keeps its `m_childInfo` array (`hkArray<ChildInfo>`: data pointer `+0x18`, size
//! `+0x1C`) of 16-byte entries `(m_shape +0, m_collisionFilterInfo +4, m_shapeSize +8,
//! m_numChildShapes +0xC)`, and `m_enabledChildren` (`+0x50`), a bit set over the first 256
//! children (children from 256 on are always enabled). The two container methods take the
//! `hkpShapeContainer` base at `+0x10` as `this`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot, the TLS index of the profiling record stream, and the
/// labels the cast functions write: `"TtrcList"` and the shared end label.
pub(crate) const TLS_GET_VALUE: u32 = 0x00fd_f044;
pub(crate) const PROFILE_SLOT: u32 = 0x0126_835c;
const LABEL_LIST: u32 = 0x010c_c398;
const LABEL_END: u32 = 0x010c_41f8;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
pub(crate) fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte profiling record `(label, time stamp)` to the thread's record stream
/// (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`) is below its limit
/// (`+0xC`).
pub(crate) fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Whether child `index` is enabled: indices from 256 on always are, the others by their bit
/// in `m_enabledChildren`.
fn child_enabled(e: &Engine, this: Ptr, index: u32) -> bool {
    index > 0xff || {
        let word = e.mem.u32(this.addr() + 0x50 + (index >> 5) * 4);
        (1u32 << (index & 0x1f)) & word != 0
    }
}

/// The child shape pointer of entry `index`.
fn child_shape(e: &Engine, this: Ptr, index: u32) -> u32 {
    let data = e.mem.u32(this.addr() + 0x18);
    e.mem.u32(data.wrapping_add(index << 4))
}

// Translated from 00cf5070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape`'s `hkpShapeContainer::getChildShape` (slot `+0x14` of the container table;
/// `this` is the container at the shape's `+0x10`): the `m_shape` of child `key`. The second
/// argument (the Xbox PDB's buffer) is not read.
pub fn fn_00cf5070(e: &mut Engine, this: Ptr, key: u32, _unused_1: u32) -> u32 {
    let data = e.mem.u32(this.addr() + 8);
    e.mem.u32(data.wrapping_add(key << 4))
}

// Translated from 00cf5080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape`'s `hkpShapeContainer::getCollisionFilterInfo` (slot `+0x10` of the
/// container table; `this` is the container at the shape's `+0x10`): the
/// `m_collisionFilterInfo` of child `key`.
pub fn fn_00cf5080(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let data = e.mem.u32(this.addr() + 8);
    e.mem.u32(data.wrapping_add(key << 4).wrapping_add(4))
}

// Translated from 00cf50c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::calcSizeForSpu` (vtable slot `+0x18`): fails with -1 when the shape has
/// 0xFFF or more children, or 0xFD or more while the byte at `input` is set; otherwise asks
/// every child (its slot `+0x18`, with `input` and 0x100) for its size, stores it in the
/// child's entry at `+8` and the child count at `+0xC`, failing with -1 as soon as a child
/// does. Returns 0x70, the size of the list shape. The second stack argument is not read.
pub fn fn_00cf50c0(e: &mut Engine, this: Ptr, input: Ptr, _unused_1: u32) -> i32 {
    let count = e.mem.u32(this.addr() + 0x1c) as i32;
    if input_flag_blocks(e, input, count) || count >= 0xfff {
        return -1;
    }
    let mut index = 0u32;
    while (index as i32) < e.mem.u32(this.addr() + 0x1c) as i32 {
        let child = child_shape(e, this, index);
        let size = e.vcall(child, 0x18, &args![input, 0x100u32]).u32();
        let entry = e.mem.u32(this.addr() + 0x18).wrapping_add(index << 4);
        e.mem.set_u32(entry + 8, size);
        let count_now = e.mem.u32(this.addr() + 0x1c);
        let entry = e.mem.u32(this.addr() + 0x18).wrapping_add(index << 4);
        e.mem.set_u32(entry + 0xc, count_now);
        let entry = e.mem.u32(this.addr() + 0x18).wrapping_add(index << 4);
        if e.mem.u32(entry + 8) == u32::MAX {
            return -1;
        }
        index += 1;
    }
    0x70
}

/// The first test of `calcSizeForSpu`: the byte at `input` is set and the child count is
/// above 0xFC (signed).
fn input_flag_blocks(e: &Engine, input: Ptr, count: i32) -> bool {
    e.mem.u8(input.addr()) != 0 && count > 0xfc
}

// Translated from 00cf5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::castRayImpl` (vtable slot `+0x20`; the `hkBool` result is returned through
/// the hidden pointer `result`, which is also returned). Casts the ray at every enabled
/// child (through the input's filter at `input + 0x24` when it has one) in index order,
/// with the output's child depth (`output + 0x40`) raised during the casts; the last child
/// that reports a hit is stored at `output + 0x20 + depth * 4` (after the depth is lowered
/// again) and the result is whether any did. Brackets the work with the `"TtrcList"` and
/// end profiling records.
pub fn fn_00cf5140(e: &mut Engine, this: Ptr, result: Ptr, input: Ptr, output: Ptr) -> Ptr {
    profile_record(e, LABEL_LIST);
    let depth = e.mem.u32(output.addr() + 0x40);
    e.mem.set_u32(output.addr() + 0x40, depth.wrapping_add(1));
    // The child casts write their `hkBool` into the stack slot of `input` and the filter's
    // answer comes back through a separate byte. The exe's frame (`SUB ESP,0x8`, four pushes)
    // taken with the arguments: the filter byte at +0x13 (00cf5222), the return address at
    // +0x18 and `input`'s own slot at +0x20 (00cf51cc).
    let best = e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let mut best = u32::MAX;
        let hit_slot = frame + 0x20;
        e.mem.set_u32(frame + 0x1c, result.addr());
        e.mem.set_u32(hit_slot, input.addr());
        e.mem.set_u32(frame + 0x24, output.addr());
        let filter_slot = frame + 0x13;
        let filter = e.mem.u32(input.addr() + 0x24);
        let mut index = 0u32;
        while (index as i32) < e.mem.u32(this.addr() + 0x1c) as i32 {
            if child_enabled(e, this, index) {
                let admitted = if filter == 0 {
                    true
                } else {
                    let answer = e
                        .vcall(
                            filter,
                            0,
                            &args![filter_slot, input, this, this.addr() + 0x10, index],
                        )
                        .u32();
                    e.mem.u8(answer) != 0
                };
                if admitted {
                    let child = e
                        .mem
                        .u32(e.mem.u32(this.addr() + 0x18).wrapping_add(index << 4));
                    e.vcall(child, 0x20, &args![hit_slot, input, output]);
                    if e.mem.u8(hit_slot) != 0 {
                        best = index;
                    }
                }
            }
            index = index.wrapping_add(1);
        }
        best
    });
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    if best != u32::MAX {
        e.mem
            .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), best);
    }
    profile_record(e, LABEL_END);
    e.mem.set_u8(result.addr(), u8::from(best != u32::MAX));
    result
}

// Translated from 00cf52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::castRayWithCollectorImpl` (vtable slot `+0x24`): for every enabled child
/// (through the input's filter at `input + 0x24` when it has one) builds the 16-byte body
/// record `(child, index, body[+8], body)` on the stack and calls the child's slot `+0x24`
/// with `(input, &record, collector)`. Brackets the work with the `"TtrcList"` and end
/// profiling records.
pub fn fn_00cf52c0(e: &mut Engine, this: Ptr, input: Ptr, body: Ptr, collector: u32) {
    profile_record(e, LABEL_LIST);
    let filter = e.mem.u32(input.addr() + 0x24);
    // The exe's frame (`SUB ESP,0x14`, four pushes) taken with the arguments: the collector
    // record at +0x14 (00cf5349 / 00cf53c4), the return address at +0x24; the filter's byte goes
    // to `input`'s own slot, +0x28 (00cf539e).
    e.with_stack(0x34, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x28, input.addr());
        e.mem.set_u32(frame + 0x2c, body.addr());
        e.mem.set_u32(frame + 0x30, collector);
        let filter_slot = frame + 0x28;
        let record = frame + 0x14;
        let mut index = 0u32;
        while (index as i32) < e.mem.u32(this.addr() + 0x1c) as i32 {
            if child_enabled(e, this, index) {
                let admitted = if filter == 0 {
                    true
                } else {
                    let answer = e
                        .vcall(
                            filter,
                            0,
                            &args![filter_slot, input, this, this.addr() + 0x10, index],
                        )
                        .u32();
                    e.mem.u8(answer) != 0
                };
                if admitted {
                    let child = e
                        .mem
                        .u32(e.mem.u32(this.addr() + 0x18).wrapping_add(index << 4));
                    let body_word = e.mem.u32(body.addr() + 8);
                    e.mem.set_u32(record, child);
                    e.mem.set_u32(record + 4, index);
                    e.mem.set_u32(record + 8, body_word);
                    e.mem.set_u32(record + 12, body.addr());
                    e.vcall(child, 0x24, &args![input, record, collector]);
                }
            }
            index = index.wrapping_add(1);
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00cf50a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as the `m_collisionFilterInfo` (`+4`) of child entry `index` of the shape
/// `this` (`this` is the list shape itself, whose `m_childInfo` data pointer is at `+0x18`; the
/// entries are 16 bytes).
pub fn fn_00cf50a0(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let data = e.mem.u32(this.addr() + 0x18);
    e.mem
        .set_u32(data.wrapping_add(index << 4).wrapping_add(4), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_5070, fn_00cf5070(Ptr, u32, u32) -> u32),
        entry!(0x00cf_5080, fn_00cf5080(Ptr, u32) -> u32),
        entry!(0x00cf_50a0, fn_00cf50a0(Ptr, u32, u32)),
        entry!(0x00cf_50c0, fn_00cf50c0(Ptr, Ptr, u32) -> i32),
        entry!(0x00cf_5140, fn_00cf5140(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00cf_52c0, fn_00cf52c0(Ptr, Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_DOUBLE: u32 = 0x0900_0200;
    const FILTER_DOUBLE: u32 = 0x0900_0300;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x100);
        // Record stream: cursor at +4, limit at +0xC.
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e
    }

    /// A list shape with `count` children whose slot `slot` is `CHILD_DOUBLE`; returns the
    /// shape and the child pointers.
    fn list_shape(e: &mut Engine, count: u32, slot: u32) -> (u32, Vec<u32>) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, CHILD_DOUBLE);
        let data = e.mem.alloc(16 * count.max(1));
        let shape = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0x18, data);
        e.mem.set_u32(shape + 0x1c, count);
        let mut children = Vec::new();
        for i in 0..count {
            let child = e.mem.alloc(0x10);
            e.mem.set_u32(child, vtable);
            e.mem.set_u32(data + 16 * i, child);
            e.mem.set_u32(data + 16 * i + 4, 0x100 + i);
            children.push(child);
        }
        (shape, children)
    }

    #[test]
    fn container_getters_index_the_child_array() {
        let mut e = engine();
        let (shape, children) = list_shape(&mut e, 3, 0x18);
        let container = shape + 0x10;
        assert_eq!(
            e.call(0x00cf_5070, &args![container, 2u32, 0u32]).u32(),
            children[2]
        );
        assert_eq!(e.call(0x00cf_5080, &args![container, 1u32]).u32(), 0x101);
    }

    #[test]
    fn calc_size_for_spu_fills_the_entries() {
        let mut e = engine();
        let (shape, children) = list_shape(&mut e, 2, 0x18);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(CHILD_DOUBLE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x40,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(8);
        let got = e.call(0x00cf_50c0, &args![shape, input, 0u32]).i32();
        assert_eq!(got, 0x70);
        assert_eq!(
            *calls.borrow(),
            vec![
                vec![children[0], input, 0x100],
                vec![children[1], input, 0x100]
            ]
        );
        let data = e.mem.u32(shape + 0x18);
        assert_eq!(e.mem.u32(data + 8), 0x40);
        assert_eq!(e.mem.u32(data + 0xc), 2);
        assert_eq!(e.mem.u32(data + 16 + 8), 0x40);
    }

    #[test]
    fn calc_size_for_spu_fails_on_a_child_failure_and_on_too_many_children() {
        let mut e = engine();
        let (shape, _) = list_shape(&mut e, 2, 0x18);
        e.register_double(CHILD_DOUBLE, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        let input = e.mem.alloc(8);
        assert_eq!(e.call(0x00cf_50c0, &args![shape, input, 0u32]).i32(), -1);
        // 0xFD children with the flag byte set, and 0xFFF without it, are too many.
        e.mem.set_u32(shape + 0x1c, 0xfd);
        e.mem.set_u8(input, 1);
        assert_eq!(e.call(0x00cf_50c0, &args![shape, input, 0u32]).i32(), -1);
        e.mem.set_u8(input, 0);
        e.mem.set_u32(shape + 0x1c, 0xfff);
        assert_eq!(e.call(0x00cf_50c0, &args![shape, input, 0u32]).i32(), -1);
    }

    #[test]
    fn cast_ray_reports_the_last_child_that_hits() {
        let mut e = engine();
        let (shape, children) = list_shape(&mut e, 3, 0x20);
        // Child 1 is disabled; the others are enabled. Children 0 and 2 hit.
        e.mem.set_u32(shape + 0x50, 0b101);
        let order = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&order);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + 0x40, 1);
        let result = e.mem.alloc(4);
        let got = e
            .call(0x00cf_5140, &args![shape, result, input, output])
            .u32();
        assert_eq!(got, result);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(*order.borrow(), vec![children[0], children[2]]);
        assert_eq!(e.mem.u32(output + 0x40), 1);
        assert_eq!(e.mem.u32(output + 0x20 + 4), 2);
    }

    #[test]
    fn cast_ray_without_a_hit_leaves_the_output_alone() {
        let mut e = engine();
        let (shape, _) = list_shape(&mut e, 2, 0x20);
        e.register_double(CHILD_DOUBLE, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x50);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 7);
        e.call(0x00cf_5140, &args![shape, result, input, output]);
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(e.mem.u32(output + 0x40), 0);
        assert_eq!(e.mem.u32(output + 0x20), 0);
    }

    #[test]
    fn cast_ray_consults_the_input_filter() {
        let mut e = engine();
        let (shape, children) = list_shape(&mut e, 2, 0x20);
        e.mem.set_u32(shape + 0x50, 0b11);
        let filter_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(filter_vtable, FILTER_DOUBLE);
        let filter = e.mem.alloc(0x10);
        e.mem.set_u32(filter, filter_vtable);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        // The filter admits only child 1.
        e.register_double(FILTER_DOUBLE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], u8::from(a[5] == 1));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let cast = Rc::new(RefCell::new(Vec::new()));
        let cast_log = Rc::clone(&cast);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            cast_log.borrow_mut().push(a[0]);
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input + 0x24, filter);
        let output = e.mem.alloc(0x50);
        let result = e.mem.alloc(4);
        // The filter byte at +0x13 of the exe's 0x28-byte frame (the next stack local).
        let frame = e.next_stack_local(0x28);
        e.call(0x00cf_5140, &args![shape, result, input, output]);
        assert_eq!(*cast.borrow(), vec![children[1]]);
        assert_eq!(seen.borrow().len(), 2);
        let first = seen.borrow()[0].clone();
        assert_eq!(first[1], frame + 0x13);
        assert_eq!(
            (first[0], first[3], first[4], first[5]),
            (filter, shape, shape + 0x10, 0)
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(output + 0x20), 1);
    }

    #[test]
    fn cast_ray_with_collector_builds_the_body_record() {
        let mut e = engine();
        let (shape, children) = list_shape(&mut e, 3, 0x24);
        e.mem.set_u32(shape + 0x50, 0b110);
        let records = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&records);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            log.borrow_mut().push((a[0], a[1], a[3], record));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 8, 0x77);
        // The record sits at +0x14 of the exe's 0x34-byte frame (the next stack local).
        let frame = e.next_stack_local(0x34);
        let seen = Rc::clone(&records);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            assert_eq!(a[2], frame + 0x14);
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            seen.borrow_mut().push((a[0], a[1], a[3], record));
            Ret::default()
        });
        e.call(0x00cf_52c0, &args![shape, input, body, 0x5555u32]);
        let got = records.borrow().clone();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].0, children[1]);
        assert_eq!(got[0].1, input);
        assert_eq!(got[0].2, 0x5555);
        assert_eq!(got[0].3, vec![children[1], 1, 0x77, body]);
        assert_eq!(got[1].3, vec![children[2], 2, 0x77, body]);
    }
}

#[cfg(test)]
mod tests_00cf50a0 {
    use super::*;

    /// The filter word of the chosen 16-byte entry changes and nothing around it.
    #[test]
    fn stores_the_filter_info_of_one_child_entry() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x40);
        let shape = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0x18, data);
        for i in 0..16 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        e.call(0x00cf_50a0, &args![shape, 2u32, 0xabcd_u32]);
        let words: Vec<u32> = (0..16).map(|i| e.mem.u32(data + 4 * i)).collect();
        let mut expected: Vec<u32> = (0..16).map(|i| 0x100 + i).collect();
        expected[9] = 0xabcd;
        assert_eq!(words, expected);
    }
}
