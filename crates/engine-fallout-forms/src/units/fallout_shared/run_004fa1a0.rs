//! `fallout shared/run_004fa1a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Entries of 0x10 bytes (a `char` priority at +0, a flag byte at +1, an
//! owned C string at +4, a member constructed by `00488fd0` at +8) kept in
//! a sorted list at +0x1C of their owner, and a record of floats cleared by
//! its constructor.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `0044a670` (`cdecl`): the length of a C string.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `00406d30` (`cdecl`: destination, size, source): copies a C string.
const STRING_COPY: u32 = 0x0040_6d30;
/// `00408b20` (`cdecl`): compares two C strings.
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `BSSimpleList<ActiveEffect_P>::Insert` (engine map), `007a7eb0`
/// (`thiscall`, `RET 8`: item, compare function): a sorted insert.
const LIST_INSERT_SORTED: u32 = 0x007a_7eb0;

// Translated from 004fa4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Orders two entries (`cdecl`; no Xbox PDB name; the compare function
/// [`fn_004fa610`] inserts with): the higher priority (signed `char` at +0)
/// first (-1 when `a`'s is higher, 1 when lower), otherwise the comparison
/// of their strings (+4, `00408b20`).
pub fn fn_004fa4b0(e: &mut Engine, a: Ptr, b: Ptr) -> i32 {
    let pa = e.mem.i8(a.addr());
    let pb = e.mem.i8(b.addr());
    if pa > pb {
        -1
    } else if pa < pb {
        1
    } else {
        let sa = e.mem.u32(a.addr() + 4);
        let sb = e.mem.u32(b.addr() + 4);
        e.call(STRING_COMPARE, &args![sa, sb]).i32()
    }
}

// Translated from 004fa500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fa500 - the C++ exception unwinding states (FS:[0] frame, handler 00f0b23b) are not translated
/// Adds an entry (no Xbox PDB name; `thiscall`, `RET 0xc`): nothing (null)
/// for a null or empty `name` or a zero `priority`; otherwise a new
/// 0x10-byte entry ([`fn_004fa5e0`]; null when the allocation fails, and
/// then the stores below write at low addresses as in the exe) gets the
/// priority, a copy of the name ([`fn_004fa670`]) and the flag, and goes
/// into the list ([`fn_004fa610`]). Returns the entry.
pub fn fn_004fa500(e: &mut Engine, this: Ptr, priority: i8, name: u32, flag: u8) -> u32 {
    if name == 0 || priority == 0 {
        return 0;
    }
    if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
        return 0;
    }
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let entry = if block == 0 {
        0
    } else {
        fn_004fa5e0(e, Ptr::new(block)).addr()
    };
    e.mem.set_i8(entry, priority);
    fn_004fa670(e, Ptr::new(entry), name);
    e.mem.set_u8(entry.wrapping_add(1), flag);
    fn_004fa610(e, this, Ptr::new(entry));
    entry
}

// Translated from 004fa5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The entry constructor (no Xbox PDB name): priority 10, flag 0, no
/// string, the member at +8 constructed (`00488fd0`). Returns `this`.
pub fn fn_004fa5e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 10);
    e.mem.set_u8(this.addr() + 1, 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(0x0048_8fd0, &args![this.addr() + 8]);
    this
}

// Translated from 004fa610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts an entry into the sorted list at +0x1C (no Xbox PDB name;
/// `RET 4`): only an entry with a nonzero priority and a non-empty string,
/// ordered by [`fn_004fa4b0`]; answers whether it was inserted.
pub fn fn_004fa610(e: &mut Engine, this: Ptr, entry: Ptr) -> bool {
    if entry.addr() == 0 || e.mem.i8(entry.addr()) == 0 || e.mem.u32(entry.addr() + 4) == 0 {
        return false;
    }
    let name = e.mem.u32(entry.addr() + 4);
    if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
        return false;
    }
    e.call(
        LIST_INSERT_SORTED,
        &args![this.addr() + 0x1c, entry, 0x004f_a4b0u32],
    );
    true
}

// Translated from 004fa670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the entry's string (no Xbox PDB name; `RET 4`): the old one is
/// freed, and a non-null `name` is copied into a new block of its length
/// plus one (`00406d30`).
pub fn fn_004fa670(e: &mut Engine, this: Ptr, name: u32) {
    let old = e.mem.u32(this.addr() + 4);
    if old != 0 {
        e.call(FREE, &args![old]);
    }
    e.mem.set_u32(this.addr() + 4, 0);
    if name != 0 {
        let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(1);
        let block = e.call(ALLOCATE, &args![size]).u32();
        e.mem.set_u32(this.addr() + 4, block);
        let destination = e.mem.u32(this.addr() + 4);
        e.call(STRING_COPY, &args![destination, size, name]);
    }
}

// Translated from 004fa970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands every entry with a non-empty string to the object at `011c3b3c`
/// (no Xbox PDB name; `this` is a subobject at +0x18 of the owner whose
/// list `00409140` walks; `RET 8`): `004442c0(string, entry + 8, a, b, 1,
/// 1, 0, 0)` for each.
pub fn fn_004fa970(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let mut node = e
        .call(0x0040_9140, &args![this.addr().wrapping_sub(0x18)])
        .u32();
    while node != 0 {
        let item_address = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(item_address);
        if entry != 0 && e.mem.u32(entry + 4) != 0 {
            let name = e.mem.u32(entry + 4);
            if e.call(STRING_LENGTH, &args![name]).u32() != 0 {
                let target = e.mem.u32(0x011c_3b3c);
                let name = e.mem.u32(entry + 4);
                e.call(
                    0x0044_42c0,
                    &args![target, name, entry + 8, a, b, 1u32, 1u32, 0u32, 0u32],
                );
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 004faa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the float record (no Xbox PDB name): [`fn_004faa20`].
/// Returns `this`.
pub fn fn_004faa00(e: &mut Engine, this: Ptr) -> Ptr {
    fn_004faa20(e, this);
    this
}

// Translated from 004faa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the float record (no Xbox PDB name): the floats at +0, +4, +8,
/// +0x24, +0x28, +0x2C and +0x18 become 0.0, the words at +0xC, +0x10,
/// +0x14, +0x1C and +0x20 become 0, and +0x30 becomes 1, in the exe's
/// order.
pub fn fn_004faa20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let zero = x87_store32(0.0);
    for offset in [0, 4, 8, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [0xc, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x18, zero);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x30, 1);
}

/// `00409140` (`thiscall`): the first node of the entry list.
const FIRST_NODE: u32 = 0x0040_9140;
/// `006815c0` (`thiscall`): the node itself, whose word +0 is its entry.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the next node.
const NODE_NEXT: u32 = 0x0072_6070;

/// The entry a list node holds (`006815c0`, then its word +0).
fn node_entry(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item)
}

// Translated from 004fa1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the entries (`ECX` is the form): `TESForm::StartForm`, then for each entry with a
/// non-empty name (+0x4) a `DATA` chunk of the priority byte, the name with its terminator
/// (`00406d30`) and the flag byte (`operator new` of the name's length + 3,
/// `TESForm::AddChunkArray`, freed) followed by the member at +0x8's `MODT` data
/// (`008d0600` with the name), and `TESForm::CloseForm`.
pub fn fn_004fa1a0(e: &mut Engine, this: Ptr) {
    e.call(0x0048_55a0, &args![this]);
    let mut node = e.call(FIRST_NODE, &args![this]).u32();
    while node != 0 {
        let entry = node_entry(e, node);
        if entry != 0 && e.mem.u32(entry + 0x4) != 0 {
            save_entry(e, entry);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0048_5680, &args![this]);
}

/// One entry of `004fa1a0` (its name non-null): nothing for an empty name, else its `DATA`
/// chunk and its member's `MODT` data.
fn save_entry(e: &mut Engine, entry: u32) {
    let name = e.mem.u32(entry + 0x4);
    if e.call(STRING_LENGTH, &args![name]).u32() != 0 {
        let name = e.mem.u32(entry + 0x4);
        let size = e.call(STRING_LENGTH, &args![name]).u32().wrapping_add(3);
        let block = e.call(ALLOCATE, &args![size]).u32();
        let priority = e.mem.u8(entry);
        e.mem.set_u8(block, priority);
        let name = e.mem.u32(entry + 0x4);
        e.call(STRING_COPY, &args![block.wrapping_add(1), size, name]);
        let name = e.mem.u32(entry + 0x4);
        let length = e.call(STRING_LENGTH, &args![name]).u32();
        let flag = e.mem.u8(entry + 1);
        e.mem.set_u8(
            block.wrapping_add(1).wrapping_add(length).wrapping_add(1),
            flag,
        );
        e.call(0x0048_56f0, &args![0x4154_4144u32, block, size]);
        e.call(FREE, &args![block]);
        let name = e.mem.u32(entry + 0x4);
        e.call(0x008d_0600, &args![entry + 0x8, 0x5444_4f4du32, name]);
    }
}

// Translated from 004fa2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies from `other` (`RET 4`) when it casts (`__RTDynamicCast` from `01183028` to
/// `011861f4`): clears this form through vtable slot 0x18, copies the components
/// (`TESForm::CopyAllComponents`) and adds each of the other's entries (`004fa500` with its
/// priority, name and flag).
pub fn fn_004fa2f0(e: &mut Engine, this: Ptr, other: u32) {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_61f4u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.vcall(this.addr(), 0x18, &args![]);
    e.call(0x0048_51b0, &args![this, cast]);
    let mut node = e.call(FIRST_NODE, &args![cast]).u32();
    while node != 0 {
        let entry = node_entry(e, node);
        if entry != 0 {
            let flag = e.mem.u8(entry + 1);
            let name = e.mem.u32(entry + 0x4);
            let priority = e.mem.u8(entry) as i8;
            fn_004fa500(e, this, priority, name, flag);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 004fa390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `011861f4`), its entry list (`00409140`) has as many entries
/// (`005ae380`) as the list at +0x1C, and walking both together every pair is both null or
/// both set with equal priorities, names (`00408b20`) and flags.
pub fn fn_004fa390(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_61f4u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mut theirs = e.call(FIRST_NODE, &args![cast]).u32();
    let their_count = e.call(0x005a_e380, &args![theirs]).u32();
    let list = this.addr() + 0x1c;
    if their_count != e.call(0x005a_e380, &args![list]).u32() {
        return true;
    }
    let mut ours = list;
    while theirs != 0 {
        let their_entry = node_entry(e, theirs);
        let our_entry = node_entry(e, ours);
        if (their_entry == 0 && our_entry != 0) || (their_entry != 0 && our_entry == 0) {
            return true;
        }
        if their_entry != 0 && our_entry != 0 {
            if e.mem.u8(their_entry) as i8 != e.mem.u8(our_entry) as i8 {
                return true;
            }
            let their_name = e.mem.u32(their_entry + 0x4);
            let our_name = e.mem.u32(our_entry + 0x4);
            if e.call(STRING_COMPARE, &args![their_name, our_name]).u32() != 0 {
                return true;
            }
            if e.mem.u8(their_entry + 1) as i8 != e.mem.u8(our_entry + 1) as i8 {
                return true;
            }
        }
        ours = e.call(NODE_NEXT, &args![ours]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
    false
}

// Translated from 004fa6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fa6f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Spawns the debris (`RET 0x10`: `target`, the position `point`, `count`, `scale`;
/// nothing for a null target): for each entry with a model name (+0x4) and a non-zero
/// percentage (+0x0, signed), `percentage * count / 100` pieces (at least one while fewer
/// than `count` were made), each with a direction of three `RandomFloat`s between the float
/// at `01012054` and 1.0, normalised (`004a0c10`): entries with flag bit 0 (`004fa950`) get
/// a 0x24-byte object (`00aa13e0`, constructor `00682b00` with the target, the float at
/// `010249d8`, the name, 0, `point`, the direction, the world point `011f426c` twice,
/// `scale`, 0, 0), the others a model (`006890b0`, cdecl: the target, the float at
/// `01017b78`, the name, the direction and the point by value, `scale` times the double at
/// `01020758`, 7, 0).
pub fn fn_004fa6f0(e: &mut Engine, this: Ptr, target: u32, point: u32, count: u32, scale: f32) {
    if target == 0 {
        return;
    }
    let scale_bits = scale.to_bits();
    let mut made = 0u32;
    // The frame below EBP: the direction at EBP-0x2C, the world point copy at EBP-0x3C.
    e.with_stack(0x20, |e, frame| {
        let direction = frame.addr() + 0x10;
        let world_point = frame.addr();
        let mut node = e.call(FIRST_NODE, &args![this]).u32();
        while node != 0 {
            let entry = node_entry(e, node);
            if entry != 0 && e.mem.u32(entry + 0x4) != 0 && e.mem.u8(entry) as i8 != 0 {
                let percentage = i32::from(e.mem.u8(entry) as i8) as u32;
                let mut pieces = percentage.wrapping_mul(count) / 100;
                if pieces == 0 && made < count {
                    pieces = 1;
                }
                let mut piece = 0u32;
                while piece < pieces {
                    e.call(NODE_ITEM, &args![direction]);
                    for axis in 0..3 {
                        let low = x87_quiet_bits(e.global::<u32>(0x0101_2054));
                        let value = e.call(0x0047_6b70, &args![low, 0x3f80_0000u32]).f64();
                        e.mem.set_u32(direction + 4 * axis, x87_store32(value));
                    }
                    e.call(0x004a_0c10, &args![direction]);
                    if fn_004fa950(e, Ptr::new(entry)) {
                        for word in 0..3 {
                            let value = e.global::<u32>(0x011f_426c + 4 * word);
                            e.mem.set_u32(world_point + 4 * word, value);
                        }
                        let block = e.call(0x00aa_13e0, &args![0x24u32]).u32();
                        if block != 0 {
                            let angle = x87_quiet_bits(e.global::<u32>(0x0102_49d8));
                            let name = e.mem.u32(entry + 0x4);
                            e.call(
                                0x0068_2b00,
                                &args![
                                    block,
                                    target,
                                    angle,
                                    name,
                                    0u32,
                                    point,
                                    direction,
                                    world_point,
                                    world_point,
                                    x87_quiet_bits(scale_bits),
                                    0u32,
                                    0u32
                                ],
                            );
                        }
                    } else {
                        let scaled = x87_store32(x87_mul(
                            x87_load32(scale_bits),
                            e.global::<f64>(0x0102_0758),
                        ));
                        let position =
                            [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)];
                        let turned = [
                            e.mem.u32(direction),
                            e.mem.u32(direction + 4),
                            e.mem.u32(direction + 8),
                        ];
                        let name = e.mem.u32(entry + 0x4);
                        let size = x87_quiet_bits(e.global::<u32>(0x0101_7b78));
                        e.call(
                            0x0068_90b0,
                            &args![
                                target,
                                size,
                                name,
                                turned[0],
                                turned[1],
                                turned[2],
                                position[0],
                                position[1],
                                position[2],
                                x87_quiet_bits(scaled),
                                7u32,
                                0u32
                            ],
                        );
                    }
                    made = made.wrapping_add(1);
                    piece += 1;
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 004fa950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the entry's flag byte at +0x1 (`MOVSX`, `AND 1`).
pub fn fn_004fa950(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u8(this.addr() + 1) as i8 as i32) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fa4b0, fn_004fa4b0(Ptr, Ptr) -> i32),
        entry!(0x004fa500, fn_004fa500(Ptr, i8, u32, u8) -> u32),
        entry!(0x004fa5e0, fn_004fa5e0(Ptr) -> Ptr),
        entry!(0x004fa610, fn_004fa610(Ptr, Ptr) -> bool),
        entry!(0x004fa670, fn_004fa670(Ptr, u32)),
        entry!(0x004fa970, fn_004fa970(Ptr, u32, u32)),
        entry!(0x004faa00, fn_004faa00(Ptr) -> Ptr),
        entry!(0x004faa20, fn_004faa20(Ptr)),
        entry!(0x004f_a1a0, fn_004fa1a0(Ptr)),
        entry!(0x004f_a2f0, fn_004fa2f0(Ptr, u32)),
        entry!(0x004f_a390, fn_004fa390(Ptr, u32) -> bool),
        entry!(0x004f_a6f0, fn_004fa6f0(Ptr, u32, u32, u32, f32)),
        entry!(0x004f_a950, fn_004fa950(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles for the CRT string helpers and the allocator, working on the
    /// engine's memory.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(STRING_COPY, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(0x0048_8fd0, |_, _| Ret::default());
        e.register(LIST_INSERT_SORTED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        p
    }

    #[test]
    fn entries_order_by_priority_then_name() {
        let mut e = engine();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_i8(a, 5);
        e.mem.set_i8(b, -3);
        assert_eq!(e.call(0x004f_a4b0, &args![a, b]).i32(), -1);
        assert_eq!(e.call(0x004f_a4b0, &args![b, a]).i32(), 1);
        e.mem.set_i8(b, 5);
        e.mem.set_u32(a + 4, 0x111);
        e.mem.set_u32(b + 4, 0x222);
        e.register(STRING_COMPARE, |_, a| Ret {
            eax: (a[0] as i32 - a[1] as i32) as u32,
            ..Ret::default()
        });
        assert_eq!(e.call(0x004f_a4b0, &args![a, b]).i32(), -0x111);
    }

    #[test]
    fn add_builds_and_inserts_an_entry() {
        let mut e = engine();
        let owner = e.mem.alloc(0x20);
        let name = text(&mut e, b"Fx");
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 0u32, name, 1u32]).u32(),
            0
        );
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 3u32, 0u32, 1u32]).u32(),
            0
        );
        let empty = text(&mut e, b"");
        assert_eq!(
            e.call(0x004f_a500, &args![owner, 3u32, empty, 1u32]).u32(),
            0
        );
        assert!(calls(&e, ALLOCATE).is_empty());
        let entry = e.call(0x004f_a500, &args![owner, 3u32, name, 1u32]).u32();
        assert_ne!(entry, 0);
        assert_eq!((e.mem.i8(entry), e.mem.u8(entry + 1)), (3, 1));
        let copy = e.mem.u32(entry + 4);
        assert_ne!(copy, name);
        assert_eq!(e.mem.cstr(copy), b"Fx".to_vec());
        assert_eq!(calls(&e, 0x0048_8fd0), vec![vec![entry + 8]]);
        assert_eq!(
            calls(&e, LIST_INSERT_SORTED),
            vec![vec![owner + 0x1c, entry, 0x004f_a4b0]]
        );
    }

    #[test]
    fn insert_needs_a_priority_and_a_string() {
        let mut e = engine();
        let entry = e.mem.alloc(0x10);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, 0u32]).bool());
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        e.mem.set_i8(entry, 1);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        let empty = text(&mut e, b"");
        e.mem.set_u32(entry + 4, empty);
        assert!(!e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        let name = text(&mut e, b"a");
        e.mem.set_u32(entry + 4, name);
        assert!(e.call(0x004f_a610, &args![0x3000u32, entry]).bool());
        assert_eq!(calls(&e, LIST_INSERT_SORTED).len(), 1);
    }

    #[test]
    fn set_string_frees_the_old_copy() {
        let mut e = engine();
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, 0x7777);
        e.call(0x004f_a670, &args![entry, 0u32]);
        assert_eq!(calls(&e, FREE), vec![vec![0x7777]]);
        assert_eq!(e.mem.u32(entry + 4), 0);
        let name = text(&mut e, b"abc");
        e.call(0x004f_a670, &args![entry, name]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4]]);
        assert_eq!(e.mem.cstr(e.mem.u32(entry + 4)), b"abc".to_vec());
    }

    #[test]
    fn entries_with_strings_go_to_the_handler() {
        let mut e = engine();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(0x011c_3b3c, 0x8800);
        let named = e.mem.alloc(0x10);
        let name = text(&mut e, b"n");
        e.mem.set_u32(named + 4, name);
        let unnamed = e.mem.alloc(0x10);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, unnamed);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, named);
        e.mem.set_u32(first + 4, second);
        e.register_double(0x0040_9140, move |_, _| Ret {
            eax: first,
            ..Ret::default()
        });
        e.register(0x0068_15c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0072_6070, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(0x0044_42c0, |_, _| Ret::default());
        e.call(0x004f_a970, &args![0x5018u32, 7u32, 9u32]);
        assert_eq!(calls(&e, 0x0040_9140), vec![vec![0x5000]]);
        assert_eq!(
            calls(&e, 0x0044_42c0),
            vec![vec![0x8800, name, named + 8, 7, 9, 1, 1, 0, 0]]
        );
    }

    #[test]
    fn record_constructor_clears_floats_and_sets_the_last_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x34);
        e.mem.write(this, &[0xff; 0x34]);
        assert_eq!(e.call(0x004f_aa00, &args![this]).u32(), this);
        for offset in (0..0x30).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x30), 1);
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A one-node list whose node is `node` (the node holds the entry at +0) and the list
    /// helpers: `00409140` answers `node`, `006815c0` the node itself, `00726070` the end.
    fn one_entry_list(e: &mut Engine, entry: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, entry);
        e.register_double(FIRST_NODE, move |_, _| node.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |_, _| Ret::default());
        node
    }

    fn entry(e: &mut Engine, priority: u8, name: u32, flag: u8) -> u32 {
        let entry = e.mem.alloc(0x10);
        e.mem.set_u8(entry, priority);
        e.mem.set_u8(entry + 1, flag);
        e.mem.set_u32(entry + 4, name);
        entry
    }

    #[test]
    fn fn_004fa1a0_saves_named_entries() {
        let mut e = Engine::new();
        let name = e.mem.alloc(8);
        let entry = entry(&mut e, 0x40, name, 1);
        one_entry_list(&mut e, entry);
        e.register(STRING_LENGTH, |_, _| 3u32.into_ret());
        let block = e.mem.alloc(8);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        for address in [
            0x0048_55a0,
            STRING_COPY,
            0x0048_56f0,
            FREE,
            0x008d_0600,
            0x0048_5680,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x004f_a1a0, &args![this]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![6]]);
        assert_eq!((e.mem.u8(block), e.mem.u8(block + 5)), (0x40, 1));
        assert_eq!(calls_to(&e, STRING_COPY), vec![vec![block + 1, 6, name]]);
        assert_eq!(calls_to(&e, 0x0048_56f0), vec![vec![0x4154_4144, block, 6]]);
        assert_eq!(
            calls_to(&e, 0x008d_0600),
            vec![vec![entry + 8, 0x5444_4f4d, name]]
        );
        assert_eq!(calls_to(&e, 0x0048_5680), vec![vec![this]]);
        // An empty name saves nothing.
        e.register(STRING_LENGTH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_a1a0, &args![this]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_004fa2f0_clears_copies_and_adds_entries() {
        let mut e = Engine::new();
        let entry = entry(&mut e, 0x7f, 0x0600_0000, 1);
        one_entry_list(&mut e, entry);
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.put_vtable(0x00da_0000, &[0x00e7_0000; 8]);
        e.register(0x00e7_0000, |_, _| Ret::default());
        e.register(0x0048_51b0, |_, _| Ret::default());
        // `004fa500` is this file's: it allocates the entry through `00401000` and inserts it.
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(STRING_LENGTH, |_, _| 3u32.into_ret());
        for address in [STRING_COPY, 0x0048_8fd0, LIST_INSERT_SORTED] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x00da_0000);
        e.call_log = Some(vec![]);
        e.call(0x004f_a2f0, &args![this, 0x9000u32]);
        assert_eq!(calls_to(&e, 0x00e7_0000), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0048_51b0), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, LIST_INSERT_SORTED).len(), 1);
        e.register(0x00ec_43fb, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_a2f0, &args![this, 0x9000u32]);
        assert!(calls_to(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn fn_004fa390_compares_entry_by_entry() {
        let mut e = Engine::new();
        let theirs = entry(&mut e, 5, 0x0600_0000, 1);
        let ours = entry(&mut e, 5, 0x0600_0010, 1);
        let their_node = e.mem.alloc(8);
        e.mem.set_u32(their_node, theirs);
        e.register_double(FIRST_NODE, move |_, _| their_node.into_ret());
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x005a_e380, |_, _| 1u32.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |_, _| Ret::default());
        e.register(STRING_COMPARE, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x1c, ours);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
        assert_eq!(
            calls_to(&e, STRING_COMPARE),
            vec![vec![0x0600_0000, 0x0600_0010]]
        );
        e.mem.set_u8(ours + 1, 0);
        assert!(e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
        e.mem.set_u8(ours + 1, 1);
        e.mem.set_u8(ours, 6);
        assert!(e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
        e.mem.set_u32(this + 0x1c, 0);
        assert!(e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
        e.register_double(0x005a_e380, move |_, a| (a[0] == their_node).into_ret());
        assert!(e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
        e.register(0x00ec_43fb, |_, _| Ret::default());
        assert!(e.call(0x004f_a390, &args![this, 0x9000u32]).bool());
    }

    #[test]
    fn fn_004fa6f0_spawns_models_and_objects() {
        let mut e = Engine::new();
        let model = entry(&mut e, 50, 0x0600_0000, 0);
        one_entry_list(&mut e, model);
        e.map(0x0101_2054, 4);
        e.set_global::<f32>(0x0101_2054, -1.0);
        e.map(0x0101_7b78, 4);
        e.set_global::<f32>(0x0101_7b78, 10.0);
        e.map(0x0102_0758, 8);
        e.set_global::<f64>(0x0102_0758, 10.0);
        e.map(0x0102_49d8, 4);
        e.set_global::<f32>(0x0102_49d8, 90.0);
        e.map(0x011f_426c, 12);
        e.register(0x0047_6b70, |_, _| 0.5f32.into_ret());
        for address in [0x004a_0c10, 0x0068_90b0, 0x0068_2b00] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x00aa_13e0, |_, _| 0x8000u32.into_ret());
        let point = e.mem.alloc(12);
        e.mem.set_u32(point, 1);
        e.mem.set_u32(point + 4, 2);
        e.mem.set_u32(point + 8, 3);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        // 50% of 3 is one piece.
        e.call(0x004f_a6f0, &args![this, 0x7000u32, point, 3u32, 2.0f32]);
        let half = 0.5f32.to_bits();
        assert_eq!(
            calls_to(&e, 0x0068_90b0),
            vec![vec![
                0x7000,
                10.0f32.to_bits(),
                0x0600_0000,
                half,
                half,
                half,
                1,
                2,
                3,
                20.0f32.to_bits(),
                7,
                0
            ]]
        );
        // Flag bit 0: the object; a null target does nothing.
        e.mem.set_u8(model + 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_a6f0, &args![this, 0x7000u32, point, 1u32, 2.0f32]);
        let made = calls_to(&e, 0x0068_2b00);
        assert_eq!(made.len(), 1);
        assert_eq!(
            made[0][..6],
            [0x8000, 0x7000, 90.0f32.to_bits(), 0x0600_0000, 0, point]
        );
        assert_eq!(made[0][7], made[0][8]);
        e.call_log = Some(vec![]);
        e.call(0x004f_a6f0, &args![this, 0u32, point, 1u32, 2.0f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_004fa950_tests_flag_bit_zero() {
        let mut e = Engine::new();
        let entry = entry(&mut e, 0, 0, 0xff);
        assert!(e.call(0x004f_a950, &args![entry]).bool());
        e.mem.set_u8(entry + 1, 0xfe);
        assert!(!e.call(0x004f_a950, &args![entry]).bool());
    }
}
