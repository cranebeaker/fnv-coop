//! `fallout shared/run_00513fa0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00514500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00514500 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x19 (0xc4 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00514500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_30a0, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x84]);
    e.call(0x0048_eac0, &args![base + 0x8c]);
    e.call(0x0048_2c50, &args![base + 0x94]);
    e.call(0x0047_5350, &args![base + 0x9c]);
    e.call(0x0047_a130, &args![base + 0xa4]);
    e.call(0x0047_b4f0, &args![base + 0xb4]);
    e.mem.set_u32(base, 0x0102_a9c4);
    e.mem.set_u32(base + 0x30, 0x0102_a9b0);
    e.mem.set_u32(base + 0x3c, 0x0102_a98c);
    e.mem.set_u32(base + 0x5c, 0x0102_a96c);
    e.mem.set_u32(base + 0x68, 0x0102_a958);
    e.mem.set_u32(base + 0x74, 0x0102_a940);
    e.mem.set_u32(base + 0x84, 0x0102_a920);
    e.mem.set_u32(base + 0x8c, 0x0102_a90c);
    e.mem.set_u32(base + 0x94, 0x0102_a8f4);
    e.mem.set_u32(base + 0x9c, 0x0102_a8e0);
    e.mem.set_u32(base + 0xa4, 0x0102_a8cc);
    e.mem.set_u32(base + 0xb4, 0x0102_a8b8);
    e.call(0x004f_15a0, &args![this, 0x19]);
    e.call(0x0051_4830, &args![this]);
    this
}

// Translated from 005144f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00514470`. `00514470` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005144f0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_4470,
        &args![this.addr().wrapping_sub(0x58), argument],
    )
    .u16()
}

// Translated from 005146a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005146d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005146a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_46d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The component destructors `005146d0` runs, in its order: the offset in
/// the object and the destructor.
const COMPONENT_DESTRUCTORS: [(u32, u32); 8] = [
    (0xb4, 0x0047_b520),
    (0xa4, 0x0047_a1a0),
    (0x9c, 0x0047_5380),
    (0x8c, 0x0048_eaf0),
    (0x84, 0x0048_e7d0),
    (0x5c, 0x0040_2c40),
    (0x3c, 0x0048_a490),
    (0x30, 0x0059_1300),
];

// Translated from 00514410 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: 1 when `004c0bd0` on the component at `+0x70` answers
/// true, else 2 when bit 8 of that component's byte at `+8` is set
/// (`00514450`), else 0.
pub fn fn_00514410(e: &mut Engine, this: Ptr) -> u8 {
    let component = this.addr() + 0x70;
    if e.call(0x004c_0bd0, &args![component]).bool() {
        1
    } else if fn_00514450(e, Ptr::new(component)) {
        2
    } else {
        0
    }
}

// Translated from 00514450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 8 of the byte at `this + 8` is set.
pub fn fn_00514450(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 8 != 0
}

// Translated from 005146d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005146d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `005146a0` runs (vtable `0102a9c4`, RTTI
/// `.?AVTESObjectBOOK@@`): stores the object's and its components'
/// vtables as the constructor `00514500` does, runs the components'
/// destructors (`COMPONENT_DESTRUCTORS`, last component first) and then
/// `0050e9f0` on the object.
pub fn fn_005146d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00, 0x0102_a9c4),
        (0x30, 0x0102_a9b0),
        (0x3c, 0x0102_a98c),
        (0x5c, 0x0102_a96c),
        (0x68, 0x0102_a958),
        (0x74, 0x0102_a940),
        (0x84, 0x0102_a920),
        (0x8c, 0x0102_a90c),
        (0x94, 0x0102_a8f4),
        (0x9c, 0x0102_a8e0),
        (0xa4, 0x0102_a8cc),
        (0xb4, 0x0102_a8b8),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    for (offset, destructor) in COMPONENT_DESTRUCTORS {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 00514830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor `00514500`: 2 zero bytes at
/// `+0xC0` (`00403d30`, a `memset` wrapper), then the byte at `+0xC1` set
/// to 0xFF, and `0041fd00` on `+0x74` with 0 (the engine map's
/// `NonActorMagicCaster::SetCurrentSpell` name is a folded body's).
pub fn fn_00514830(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_3d30, &args![base + 0xc0, 0u32, 2u32]);
    e.mem.set_u8(base + 0xc1, 0xff);
    e.call(0x0041_fd00, &args![base + 0x74, 0u32]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectARMO@@` (`01183a34`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_ARMO: u32 = 0x0118_3a34;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `memcmp` (`00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;
/// The list at +0x188 (a list object whose first node is the object
/// itself): `006815c0` (`thiscall` on a node) answers the address of the
/// node's item, `00726070` (`thiscall`) the next node.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `00470470` (`thiscall` on the list), the list's deleting destructor
/// `004702f0` (`thiscall`, one word), the list constructor `0096a2d0`
/// (`thiscall`) and the list's add `005ae3d0` (`thiscall`, one word: the
/// address of the item).
const LIST_00470470: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_ADD: u32 = 0x005a_e3d0;
/// The allocator (`00401000`, `cdecl`: size), its free (`00401030`,
/// `cdecl`) and the 0xC-byte entry constructor `00513d60`
/// (`tesobjectarmo.rs`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
const ENTRY_CONSTRUCT: u32 = 0x0051_3d60;

/// `form` cast to `TESObjectARMO`, or 0.
fn cast_to_armo(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMO, 0u32],
    )
    .u32()
}

/// The word at the item address `006815c0` answers for `node`.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(item)
}

// Translated from 00513fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00513fa0 - the FS:[0] exception frame (freeing a new list or entry when its constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; the form's
/// copy): when `source` casts to `TESObjectARMO`,
/// `TESForm::CopyAllComponents(source)`, the three words at +0x178 copied,
/// and the list at +0x188 replaced. When the byte at +0x18C says the form
/// owns its list (non-zero) and the list is set, every item is freed
/// (`00401030`, walking to the first node without an item), `00470470` runs
/// on the list and its deleting destructor (`004702f0(1)`). The byte is then
/// copied; without it the list pointer is copied as is, with it a new list
/// (`0096a2d0` on 8 allocated bytes, null when the allocation fails) gets a
/// new 0xC-byte copy (`00513d60`, then the three words) of every non-null
/// entry with a non-zero first word of the source's list (`005ae3d0`).
pub fn fn_00513fa0(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_armo(e, source);
    if other == 0 {
        return;
    }
    let at = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    for offset in [0x178, 0x17c, 0x180] {
        let word = e.mem.u32(other.wrapping_add(offset));
        e.mem.set_u32(at + offset, word);
    }
    if e.mem.u8(at + 0x18c) != 0 && e.mem.u32(at + 0x188) != 0 {
        let mut node = e.mem.u32(at + 0x188);
        while node != 0 && node_item(e, node) != 0 {
            let item = node_item(e, node);
            e.call(FREE, &args![item]);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let list = e.mem.u32(at + 0x188);
        e.call(LIST_00470470, &args![list]);
        let list = e.mem.u32(at + 0x188);
        if list != 0 {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
    }
    let owns = e.mem.u8(other.wrapping_add(0x18c));
    e.mem.set_u8(at + 0x18c, owns);
    if e.mem.u8(at + 0x18c) == 0 {
        let list = e.mem.u32(other.wrapping_add(0x188));
        e.mem.set_u32(at + 0x188, list);
        return;
    }
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    let list = if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    };
    e.mem.set_u32(at + 0x188, list);
    if e.mem.u32(other.wrapping_add(0x188)) == 0 {
        return;
    }
    let mut node = e.mem.u32(other.wrapping_add(0x188));
    while node != 0 && node_item(e, node) != 0 {
        let entry = node_item(e, node);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if entry == 0 || e.mem.u32(entry) == 0 {
            continue;
        }
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let copy = if block == 0 {
            0
        } else {
            e.call(ENTRY_CONSTRUCT, &args![block]).u32()
        };
        for offset in [0, 4, 8] {
            let word = e.mem.u32(entry + offset);
            e.mem.set_u32(copy.wrapping_add(offset), word);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), copy);
            let list = e.mem.u32(at + 0x188);
            e.call(LIST_ADD, &args![list, slot]);
        });
    }
}

// Translated from 00514220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether the
/// form differs from `form`. True when it does not cast to
/// `TESObjectARMO`, when `TESForm::CompareAllComponents` answers true,
/// when `memcmp` finds the 0xC bytes at +0x178 different, or when the bytes
/// at +0x18C differ. Without that byte the list pointers at +0x188 decide.
/// With it, both lists must be set and have the same first item, and every
/// entry of this form's list (up to the first node without an item) must
/// match one of the other's by its first word, its byte at +4 and its word
/// at +8; anything else differs.
pub fn fn_00514220(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_armo(e, form);
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    let at = this.addr();
    let order = e
        .call(
            MEMCMP,
            &args![at + 0x178, other.wrapping_add(0x178), 0xcu32],
        )
        .u32();
    if order != 0 {
        return true;
    }
    if e.mem.u8(at + 0x18c) != e.mem.u8(other.wrapping_add(0x18c)) {
        return true;
    }
    if e.mem.u8(at + 0x18c) == 0 {
        return e.mem.u32(at + 0x188) != e.mem.u32(other.wrapping_add(0x188));
    }
    let theirs = e.mem.u32(other.wrapping_add(0x188));
    if theirs == 0 || e.mem.u32(at + 0x188) == 0 {
        return true;
    }
    let first_theirs = e.call(NODE_ITEM, &args![theirs]).u32();
    let mine = e.mem.u32(at + 0x188);
    let first_mine = e.call(NODE_ITEM, &args![mine]).u32();
    if e.mem.u32(first_theirs) != e.mem.u32(first_mine) {
        return true;
    }
    let mut node = e.mem.u32(at + 0x188);
    while node != 0 && node_item(e, node) != 0 {
        let mut found = false;
        let mut candidate = e.mem.u32(other.wrapping_add(0x188));
        while candidate != 0 && node_item(e, candidate) != 0 {
            let entry = node_item(e, node);
            let match_entry = node_item(e, candidate);
            if e.mem.u32(entry) == e.mem.u32(match_entry) {
                let entry = node_item(e, node);
                let flag = e.mem.u8(entry + 4);
                let match_entry = node_item(e, candidate);
                if flag == e.mem.u8(match_entry + 4) {
                    let entry = node_item(e, node);
                    let match_entry = node_item(e, candidate);
                    if e.mem.u32(entry + 8) == e.mem.u32(match_entry + 8) {
                        found = true;
                        break;
                    }
                }
            }
            candidate = e.call(NODE_NEXT, &args![candidate]).u32();
        }
        if !found {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 00514470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a save-size sum): the 16-bit sum of `00484bf0` on `this`
/// and `0048e9b0` on `this + 0x58`, each given `argument`. `RET 4`.
pub fn fn_00514470(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0048_4bf0, &args![this, argument]).u16());
    let component = this.addr().wrapping_add(0x58);
    size.wrapping_add(e.call(0x0048_e9b0, &args![component, argument]).u16())
}

// Translated from 005144c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484d60` on `this`, then `0048ea40` on `this + 0x58`,
/// each given `argument`. `RET 4`.
pub fn fn_005144c0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4d60, &args![this, argument]);
    let component = this.addr().wrapping_add(0x58);
    e.call(0x0048_ea40, &args![component, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_4500, fn_00514500(Ptr) -> Ptr),
        entry!(0x0051_44f0, fn_005144f0(Ptr, u32) -> u16),
        entry!(0x0051_46a0, fn_005146a0(Ptr, u32) -> Ptr),
        entry!(0x0051_4410, fn_00514410(Ptr) -> u8),
        entry!(0x0051_4450, fn_00514450(Ptr) -> bool),
        entry!(0x0051_46d0, fn_005146d0(Ptr)),
        entry!(0x0051_4830, fn_00514830(Ptr)),
        entry!(0x0051_3fa0, fn_00513fa0(Ptr, u32)),
        entry!(0x0051_4220, fn_00514220(Ptr, u32) -> bool),
        entry!(0x0051_4470, fn_00514470(Ptr, u32) -> u16),
        entry!(0x0051_44c0, fn_005144c0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != skip)
            .collect()
    }

    #[test]
    fn kind_from_the_component_at_0x70() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.register(0x004c_0bd0, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_4410, &[this]).u8(), 0);
        e.mem.set_u8(this + 0x78, 8);
        assert_eq!(e.call(0x0051_4410, &[this]).u8(), 2);
        assert!(e.call(0x0051_4450, &[this + 0x70]).bool());
        e.mem.set_u8(this + 0x78, 0xf7);
        assert!(!e.call(0x0051_4450, &[this + 0x70]).bool());
        e.register(0x004c_0bd0, |_, a| Ret {
            eax: u32::from(a[0] % 2 == 0),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_4410, &[this]).u8(), 1);
        assert_eq!(
            calls(&mut e, 0x0051_4410),
            vec![(0x004c_0bd0, vec![this + 0x70])]
        );
    }

    #[test]
    fn destructor_body_and_start_values() {
        let mut e = Engine::new();
        for (_, d) in COMPONENT_DESTRUCTORS {
            e.register(d, |_, _| Ret::default());
        }
        for a in [0x0050_e9f0, 0x0040_3d30, 0x0041_fd00] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x0051_46d0, &[this]);
        let mut expected: Vec<(u32, Vec<u32>)> = COMPONENT_DESTRUCTORS
            .iter()
            .map(|(o, d)| (*d, vec![this + o]))
            .collect();
        expected.push((0x0050_e9f0, vec![this]));
        assert_eq!(calls(&mut e, 0x0051_46d0), expected);
        assert_eq!(e.mem.u32(this), 0x0102_a9c4);
        assert_eq!(e.mem.u32(this + 0xb4), 0x0102_a8b8);

        e.call_log = Some(vec![]);
        e.call(0x0051_4830, &[this]);
        assert_eq!(
            calls(&mut e, 0x0051_4830),
            vec![
                (0x0040_3d30, vec![this + 0xc0, 0, 2]),
                (0x0041_fd00, vec![this + 0x74, 0]),
            ]
        );
        assert_eq!(e.mem.u8(this + 0xc1), 0xff);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00514500_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_4500,
            size: 0xc4,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_30a0, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x84)]),
                (0x0048_eac0, &[A::T(0x8c)]),
                (0x0048_2c50, &[A::T(0x94)]),
                (0x0047_5350, &[A::T(0x9c)]),
                (0x0047_a130, &[A::T(0xa4)]),
                (0x0047_b4f0, &[A::T(0xb4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x19)]),
                (0x0051_4830, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a9c4),
                (0x30, 4, 0x0102_a9b0),
                (0x3c, 4, 0x0102_a98c),
                (0x5c, 4, 0x0102_a96c),
                (0x68, 4, 0x0102_a958),
                (0x74, 4, 0x0102_a940),
                (0x84, 4, 0x0102_a920),
                (0x8c, 4, 0x0102_a90c),
                (0x94, 4, 0x0102_a8f4),
                (0x9c, 4, 0x0102_a8e0),
                (0xa4, 4, 0x0102_a8cc),
                (0xb4, 4, 0x0102_a8b8),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_005144f0() {
        assert_eq!(check(0x0051_44f0, 0x0051_4470, 0x58, &[0x77]).u16(), 0xbeef);
    }
    #[test]
    fn at_005146a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_46d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_46a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_46a0)
                .collect();
            let mut expected = vec![(0x0051_46d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    /// Lists as nodes of two words (item, next): `006815c0` answers the
    /// node itself (its item word) and `00726070` the next word. The cast
    /// answers the form when its word at +0xc is zero.
    fn setup(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMO, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
    }

    /// A list of the given entries (each first word, flag byte, last word),
    /// ending with an empty node.
    fn list(e: &mut Engine, entries: &[(u32, u8, u32)]) -> u32 {
        let mut next = e.mem.alloc(8);
        for &(first, flag, last) in entries.iter().rev() {
            let entry = e.mem.alloc(0xc);
            e.mem.set_u32(entry, first);
            e.mem.set_u8(entry + 4, flag);
            e.mem.set_u32(entry + 8, last);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fn_00513fa0_copies_words_and_rebuilds_the_list() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(LIST_00470470, |_, _| Ret::default());
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(ENTRY_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 4, item);
            Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let source = e.mem.alloc(0x200);
        // Not armour: nothing.
        e.mem.set_u32(source + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_3fa0, &[this, source]);
        assert!(calls_to(&mut e, &[COPY_ALL_COMPONENTS]).is_empty());
        e.mem.set_u32(source + 0xc, 0);
        // An owned old list is freed; the source does not own its list.
        let old = list(&mut e, &[(1, 0, 0), (2, 0, 0)]);
        let first_old = e.mem.u32(old);
        e.mem.set_u32(this + 0x188, old);
        e.mem.set_u8(this + 0x18c, 1);
        e.mem.set_u32(source + 0x178, 0x11);
        e.mem.set_u32(source + 0x180, 0x33);
        e.mem.set_u32(source + 0x188, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0051_3fa0, &[this, source]);
        let log = calls_to(
            &mut e,
            &[COPY_ALL_COMPONENTS, FREE, LIST_00470470, LIST_DELETE],
        );
        assert_eq!(log.len(), 5);
        assert_eq!(log[0], (COPY_ALL_COMPONENTS, vec![this, source]));
        assert_eq!(log[1], (FREE, vec![first_old]));
        assert_eq!(log[3], (LIST_00470470, vec![old]));
        assert_eq!(log[4], (LIST_DELETE, vec![old, 1]));
        assert_eq!(e.mem.u32(this + 0x178), 0x11);
        assert_eq!(e.mem.u32(this + 0x180), 0x33);
        assert_eq!(e.mem.u8(this + 0x18c), 0);
        assert_eq!(e.mem.u32(this + 0x188), 0x4444);
        // An owned source list is copied entry by entry (zero entries skipped).
        let theirs = list(&mut e, &[(5, 6, 7), (0, 1, 1), (8, 9, 10)]);
        e.mem.set_u32(source + 0x188, theirs);
        e.mem.set_u8(source + 0x18c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_3fa0, &[this, source]);
        let adds = calls_to(&mut e, &[LIST_ADD]);
        assert_eq!(adds.len(), 2);
        assert_eq!(e.mem.u8(this + 0x18c), 1);
        let new_list = e.mem.u32(this + 0x188);
        assert_eq!(adds[0].1[0], new_list);
        // The double keeps the last added entry at +4 of the list.
        let last = e.mem.u32(new_list + 4);
        assert_eq!(
            (e.mem.u32(last), e.mem.u8(last + 4), e.mem.u32(last + 8)),
            (8, 9, 10)
        );
    }

    #[test]
    fn fn_00514220_compares_the_lists() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        e.register(MEMCMP, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let other = e.mem.alloc(0x200);
        e.mem.set_u32(other + 0xc, 1);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        assert!(!e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 0);
        e.mem.set_u32(other + 0x178, 1);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u32(other + 0x178, 0);
        e.mem.set_u32(other + 0x188, 5);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u8(other + 0x18c, 1);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u8(this + 0x18c, 1);
        // Owned lists: one missing differs.
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        // The same first item, the other entry a copy: equal.
        let mine = list(&mut e, &[(1, 2, 3), (4, 5, 6)]);
        let theirs = list(&mut e, &[(0, 0, 0), (4, 5, 6)]);
        let shared = e.mem.u32(mine);
        e.mem.set_u32(theirs, shared);
        e.mem.set_u32(this + 0x188, mine);
        e.mem.set_u32(other + 0x188, theirs);
        assert!(!e.call(0x0051_4220, &[this, other]).bool());
        // An entry of mine without a match differs.
        let second = e.mem.u32(e.mem.u32(mine + 4));
        e.mem.set_u8(second + 4, 9);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        e.mem.set_u8(second + 4, 5);
        e.mem.set_u32(second + 8, 9);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
        // Different first items differ.
        let theirs = list(&mut e, &[(1, 2, 3), (4, 5, 6)]);
        e.mem.set_u32(other + 0x188, theirs);
        e.mem.set_u32(second + 8, 6);
        assert!(e.call(0x0051_4220, &[this, other]).bool());
    }

    #[test]
    fn fn_00514470_and_005144c0_use_the_component_at_0x58() {
        let mut e = Engine::new();
        e.register(0x0048_4bf0, |_, _| Ret {
            eax: 0xfff0,
            ..Ret::default()
        });
        e.register(0x0048_e9b0, |_, _| Ret {
            eax: 0x1_0011,
            ..Ret::default()
        });
        e.register(0x0048_4d60, |_, _| Ret::default());
        e.register(0x0048_ea40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_4470, &[0x1000, 3]).u16(), 1);
        e.call(0x0051_44c0, &[0x1000, 4]);
        assert_eq!(
            calls_to(
                &mut e,
                &[0x0048_4bf0, 0x0048_e9b0, 0x0048_4d60, 0x0048_ea40]
            ),
            vec![
                (0x0048_4bf0, vec![0x1000, 3]),
                (0x0048_e9b0, vec![0x1058, 3]),
                (0x0048_4d60, vec![0x1000, 4]),
                (0x0048_ea40, vec![0x1058, 4]),
            ]
        );
    }
}
