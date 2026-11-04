//! `fallout/ai/run_00979510` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatAction`'s vtable (`0108cab8`, named by its RTTI locator).
const COMBAT_ACTION_VTABLE: u32 = 0x0108_cab8;
/// The list head constructor (`00680890`, `thiscall`).
const LIST_CONSTRUCT: u32 = 0x0068_0890;
/// The 12-byte table indexed by the action kind; its first word goes to
/// +0x28 (`011a4288`).
const KIND_TABLE: u32 = 0x011a_4288;
/// Returns the address of a list head's item (`006815c0`, `thiscall`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// Removes the head of a list (`0063f7b0`, `thiscall`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// The list destructor (`007b9000`, `thiscall`).
const LIST_DESTRUCT: u32 = 0x007b_9000;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The four list heads of a `CombatAction`.
const LISTS: [u32; 4] = [0x04, 0x0c, 0x14, 0x1c];

// Translated from 009795c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at +0xD4 of `this` (`FLD`/`FSTP`, so a signalling NaN is
/// stored quiet).
pub fn fn_009795c0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xd4, x87_quiet_bits(value.to_bits()));
}

// Translated from 00979bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979bd0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction` constructor (by the vtable it sets, `0108cab8`): the four
/// list heads at +0x04, +0x0C, +0x14, +0x1C (`00680890`), `kind` at +0x24
/// and the first word of entry `kind` of the 12-byte table at `011a4288` at
/// +0x28. Returns `this`.
pub fn fn_00979bd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_ACTION_VTABLE);
    for offset in LISTS {
        e.call(LIST_CONSTRUCT, &args![base + offset]);
    }
    e.mem.set_u32(base + 0x24, kind);
    let index = e.mem.u32(base + 0x24);
    let word = e.mem.u32(index.wrapping_mul(0xc).wrapping_add(KIND_TABLE));
    e.mem.set_u32(base + 0x28, word);
    this
}

/// Frees the entry at the head of the list at `list` and removes the head
/// until the list is empty.
fn drain(e: &mut Engine, list: u32) {
    // The exe tests `list != 0` before every step; the address never
    // changes, so one test is the same.
    if list == 0 {
        return;
    }
    loop {
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(NODE_ITEM, &args![list]).u32();
        let entry = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![entry]);
        e.call(LIST_REMOVE_HEAD, &args![list]);
    }
}

// Translated from 00979c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979c80 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatAction` destructor body (by its vtable, `0108cab8`): frees every
/// entry of the lists at +0x14, +0x04, +0x0C and +0x1C (in that order),
/// then destroys the lists (+0x1C, +0x14, +0x0C, +0x04, `007b9000`).
pub fn fn_00979c80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_ACTION_VTABLE);
    for offset in [0x14u32, 0x04, 0x0c, 0x1c] {
        drain(e, base.wrapping_add(offset));
    }
    for offset in [0x1cu32, 0x14, 0x0c, 0x04] {
        e.call(LIST_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
}

/// The three-float vector at `011f426c` (zero in the file's data).
const ZERO_POINT: u32 = 0x011f_426c;
/// The `double` 0.0 the code compares with (`.rdata`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `0043b300` (cdecl `(type, object)`): whether `object` is of the kind of
/// the `NiRTTI` record at `type`; the records at `01202558`, `012024e0`,
/// `01202508` and `011f4acc` are used here.
const IS_KIND: u32 = 0x0043_b300;
/// `005495f0` (`thiscall`): the geometry data of a node; `004a8a90`
/// (`thiscall`): its skin data; `00a67360` (`thiscall(word)`) / `00a67410`
/// (`thiscall(out)`) / `00a673d0` (`thiscall`) lock, read and unlock it;
/// `00534550` / `00534570` (`thiscall(index, out)`) / `0065b390`
/// (`thiscall(index, source)`) construct, read and write a vertex accessor;
/// `0047d370` (`thiscall`): the 16-bit vertex count of the skin data;
/// `007af430` (`thiscall`): the vertex array (12-byte entries), `004a8ae0`
/// (`thiscall`) its 16-bit count, `0055b980` (`thiscall`): the second array
/// (8-byte entries).
const GEOMETRY_DATA: u32 = 0x0054_95f0;
const SKIN_DATA: u32 = 0x004a_8a90;
const LOCK_DATA: u32 = 0x00a6_7360;
const READ_DATA: u32 = 0x00a6_7410;
const UNLOCK_DATA: u32 = 0x00a6_73d0;
const ACCESSOR_CONSTRUCT: u32 = 0x0053_4550;
const ACCESSOR_GET: u32 = 0x0053_4570;
const ACCESSOR_SET: u32 = 0x0065_b390;
const SKIN_VERTEX_COUNT: u32 = 0x0047_d370;
const VERTICES: u32 = 0x007a_f430;
const VERTEX_COUNT: u32 = 0x004a_8ae0;
const SECOND_ARRAY: u32 = 0x0055_b980;
/// `004d7d50` (`thiscall`): the number of children of a node; `004d7d70`
/// (`thiscall(index)`): one child; `00639b20` (`thiscall`, a float in ST0) and
/// `004ee4b0` (`thiscall(float)`): a child's length getter and setter.
const CHILD_COUNT: u32 = 0x004d_7d50;
const CHILD_AT: u32 = 0x004d_7d70;
const CHILD_LENGTH: u32 = 0x0063_9b20;
const CHILD_SET_LENGTH: u32 = 0x004e_e4b0;

// Translated from 00979510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AddImpact` of a beam (`RET 0x18`, six words; the `Projectile` one is
/// `Projectile::AddImpact`, `009bf5a0`): only while the list at +0x88 is
/// empty (`008256d0`), `009c2c80(first, fourth, this, 004181e0(this))` allows
/// it and the `Projectile` impact is added; then the length (+0xD4,
/// `fn_009795c0`) becomes the distance from the beam's position (slot 0x1F4)
/// to the point at `second` (`00439ef0`, `00457990`). Answers whether the
/// impact was added.
pub fn fn_00979510(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let at = this.addr();
    if !e.call(0x0082_56d0, &args![at + 0x88]).bool() {
        return false;
    }
    let extra = e.call(0x0041_81e0, &args![this]).u32();
    if !e
        .call(0x009c_2c80, &args![first, fourth, this, extra])
        .bool()
    {
        return false;
    }
    if !e
        .call(
            0x009b_f5a0,
            &args![this, first, second, third, fourth, fifth, sixth],
        )
        .bool()
    {
        return false;
    }
    let position = e.vcall(at, 0x1f4, &[]).u32();
    let length = e.with_stack(0xc, |e, difference| {
        let result = e
            .call(0x0043_9ef0, &args![second, difference, position])
            .u32();
        e.call(0x0045_7990, &args![result]).f64()
    });
    fn_009795c0(e, this, f32::from_bits(x87_store32(length)));
    true
}

// Translated from 009795e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the beam geometry `node` by `scale` (`RET 8`; `this` is not read):
/// the node's scale (`00537b10` +0xC) and the vector `00537b10` gives
/// (`00439180`); then, for geometry data (`005495f0`) of the `01202558` kind,
/// the length of every child of the `01202508` kind of a node of the
/// `012024e0` kind (`00639b20`, `004ee4b0`); for skinned data whose slot
/// 0x8C holds, the y of each vertex through the accessor (`00534570`,
/// `0065b390`) while the data is locked (`00a67360(1)` ... `00a673d0`);
/// otherwise the y of each vertex (+4 of 12-byte entries) and of each entry
/// of the second array (+4 of 8-byte entries).
pub fn fn_009795e0(e: &mut Engine, _this: Ptr, node: u32, scale: f32) {
    let scale_bits = scale.to_bits();
    if node == 0 {
        return;
    }
    let scaled = |value: u32| x87_store32(x87_mul(x87_load32(value), x87_m32(scale_bits)));
    let transform = e.call(0x0053_7b10, &args![node]).u32() + 0xc;
    let value = scaled(e.mem.u32(transform));
    e.mem.set_u32(transform, value);
    let vector = e.call(0x0053_7b10, &args![node]).u32();
    e.call(0x0043_9180, &args![vector, x87_quiet_bits(scale_bits)]);
    let data = e.call(GEOMETRY_DATA, &args![node]).u32();
    if data == 0 {
        return;
    }
    if e.call(IS_KIND, &args![0x0120_2558u32, data]).bool() {
        if e.call(IS_KIND, &args![0x0120_24e0u32, node]).bool() {
            let mut index = 0u32;
            while index < e.call(CHILD_COUNT, &args![node]).u32() {
                let child = e.call(CHILD_AT, &args![node, index]).u32();
                if e.call(IS_KIND, &args![0x0120_2508u32, child]).bool() {
                    let length = e.call(CHILD_LENGTH, &args![child]).f64();
                    let length = x87_store32(x87_mul(length, x87_m32(scale_bits)));
                    e.call(CHILD_SET_LENGTH, &args![child, x87_quiet_bits(length)]);
                }
                index += 1;
            }
        }
        return;
    }
    if e.call(SKIN_DATA, &args![data]).u32() != 0 {
        let skin = e.call(SKIN_DATA, &args![data]).u32();
        if e.vcall(skin, 0x8c, &[]).bool() {
            e.with_stack(0x18, |e, frame| {
                // The accessor (EBP-0x20) and the vertex (EBP-0x34).
                let accessor = frame.addr();
                let vertex = frame.addr() + 0xc;
                e.call(ACCESSOR_CONSTRUCT, &args![accessor]);
                let skin = e.call(SKIN_DATA, &args![data]).u32();
                if !e.call(LOCK_DATA, &args![data, 1u32]).bool() {
                    return;
                }
                e.call(READ_DATA, &args![data, accessor]);
                let mut index = 0i32;
                while index < i32::from(e.call(SKIN_VERTEX_COUNT, &args![skin]).u16()) {
                    for i in 0..3 {
                        let word = e.mem.u32(ZERO_POINT + 4 * i);
                        e.mem.set_u32(vertex + 4 * i, word);
                    }
                    e.call(ACCESSOR_GET, &args![accessor, index, vertex]);
                    let value = scaled(e.mem.u32(vertex + 4));
                    e.mem.set_u32(vertex + 4, value);
                    e.call(ACCESSOR_SET, &args![accessor, index, vertex]);
                    index += 1;
                }
                e.call(UNLOCK_DATA, &args![data]);
            });
            return;
        }
    }
    if e.call(VERTICES, &args![data]).u32() == 0 {
        return;
    }
    let mut index = 0i32;
    while index < i32::from(e.call(VERTEX_COUNT, &args![data]).u16()) {
        let vertices = e.call(VERTICES, &args![data]).u32();
        let y = vertices.wrapping_add((index as u32).wrapping_mul(0xc)) + 4;
        let value = scaled(e.mem.u32(y));
        e.mem.set_u32(y, value);
        let second = e.call(SECOND_ARRAY, &args![data]).u32();
        let y = second.wrapping_add((index as u32).wrapping_mul(8)) + 4;
        let value = scaled(e.mem.u32(y));
        e.mem.set_u32(y, value);
        index += 1;
    }
}

// Translated from 00979840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of the beam geometry `node` (`RET 4`; `this` is not read):
/// 0.0 without geometry data; for data of the `01202558` kind the first
/// nonzero length (`00639b20`) of a child of the `01202508` kind (nodes of
/// the `012024e0` kind only); otherwise the largest vertex y above 0.0,
/// through the accessor while skinned data is locked, else from the vertex
/// array.
pub fn fn_00979840(e: &mut Engine, _this: Ptr, node: u32) -> f32 {
    let data = e.call(GEOMETRY_DATA, &args![node]).u32();
    let mut longest = 0.0f32.to_bits();
    if data == 0 {
        return f32::from_bits(longest);
    }
    if e.call(IS_KIND, &args![0x0120_2558u32, data]).bool() {
        if e.call(IS_KIND, &args![0x0120_24e0u32, node]).bool() {
            let mut index = 0u32;
            // `FCOMP 0.0; TEST AH,0x44; JP`: only while still exactly 0.0.
            while x87_load32(longest) == e.mem.f64(DOUBLE_ZERO)
                && index < e.call(CHILD_COUNT, &args![node]).u32()
            {
                let child = e.call(CHILD_AT, &args![node, index]).u32();
                if e.call(IS_KIND, &args![0x0120_2508u32, child]).bool() {
                    longest = x87_store32(e.call(CHILD_LENGTH, &args![child]).f64());
                }
                index += 1;
            }
        }
        return f32::from_bits(x87_quiet_bits(longest));
    }
    // `FCOMPP; TEST AH,0x41; JNZ`: only a larger y (ordered) is taken.
    let take = |longest: &mut u32, y: u32| {
        if x87_load32(y) > x87_load32(*longest) {
            *longest = x87_quiet_bits(y);
        }
    };
    if e.call(SKIN_DATA, &args![data]).u32() != 0 {
        let skin = e.call(SKIN_DATA, &args![data]).u32();
        if e.vcall(skin, 0x8c, &[]).bool() {
            e.with_stack(0x18, |e, frame| {
                // The accessor (EBP-0x24) and the vertex (EBP-0x38).
                let accessor = frame.addr();
                let vertex = frame.addr() + 0xc;
                e.call(ACCESSOR_CONSTRUCT, &args![accessor]);
                let skin = e.call(SKIN_DATA, &args![data]).u32();
                if !e.call(LOCK_DATA, &args![data, 1u32]).bool() {
                    return;
                }
                let mut index = 0i32;
                while index < i32::from(e.call(SKIN_VERTEX_COUNT, &args![skin]).u16()) {
                    for i in 0..3 {
                        let word = e.mem.u32(ZERO_POINT + 4 * i);
                        e.mem.set_u32(vertex + 4 * i, word);
                    }
                    e.call(READ_DATA, &args![data, accessor]);
                    e.call(ACCESSOR_GET, &args![accessor, index, vertex]);
                    take(&mut longest, e.mem.u32(vertex + 4));
                    index += 1;
                }
                e.call(UNLOCK_DATA, &args![data]);
            });
            return f32::from_bits(x87_quiet_bits(longest));
        }
    }
    if e.call(VERTICES, &args![data]).u32() != 0 {
        let mut index = 0i32;
        while index < i32::from(e.call(VERTEX_COUNT, &args![data]).u16()) {
            let vertices = e.call(VERTICES, &args![data]).u32();
            let entry = vertices.wrapping_add((index as u32).wrapping_mul(0xc));
            take(&mut longest, e.mem.u32(entry + 4));
            index += 1;
        }
    }
    f32::from_bits(x87_quiet_bits(longest))
}

// Translated from 00979a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stretches the beam's 3D `node` to `length` (`RET 0xC`): with a positive
/// `current` length the node's translation is scaled by `length / current`
/// (`0043c490`, `00439180`, `00440460`). A node of the `011f4acc` kind gets
/// its geometry scaled by `length` over `current` (or, when `current` is not
/// positive, over its own length `fn_00979840`) unless that is 0.0
/// (`fn_009795e0`); any other node passes both on to its children (slot 0xC,
/// `0043b480`, `0043b4a0`).
pub fn fn_00979a70(e: &mut Engine, this: Ptr, node: u32, length: f32, current: f32) {
    if node == 0 {
        return;
    }
    let length_bits = length.to_bits();
    let current_bits = current.to_bits();
    let zero = e.mem.f64(DOUBLE_ZERO);
    // `FCOMP 0.0; TEST AH,0x41; JNZ`: only above zero (ordered).
    let positive = x87_load32(current_bits) > zero;
    if positive {
        let ratio = x87_store32(x87_div(x87_load32(length_bits), x87_m32(current_bits)));
        e.with_stack(0xc, |e, translation| {
            let source = e.call(0x0043_c490, &args![node]).u32();
            for i in 0..3 {
                let word = e.mem.u32(source + 4 * i);
                e.mem.set_u32(translation.addr() + 4 * i, word);
            }
            e.call(0x0043_9180, &args![translation, x87_quiet_bits(ratio)]);
            e.call(0x0044_0460, &args![node, translation]);
        });
    }
    if e.call(IS_KIND, &args![0x011f_4accu32, node]).bool() {
        let own = if positive {
            x87_quiet_bits(current_bits)
        } else {
            x87_store32(f64::from(fn_00979840(e, this, node)))
        };
        let own = x87_quiet_bits(own);
        // `FCOMP 0.0; TEST AH,0x44; JNP`: anything but exactly 0.0.
        if x87_load32(own) != zero {
            let scale = x87_store32(x87_div(x87_load32(length_bits), x87_m32(own)));
            fn_009795e0(e, this, node, f32::from_bits(x87_quiet_bits(scale)));
        }
        return;
    }
    let children = e.vcall(node, 0xc, &[]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![children]).u32() {
        let child = e.call(0x0043_b4a0, &args![children, index]).u32();
        fn_00979a70(
            e,
            this,
            child,
            f32::from_bits(x87_quiet_bits(length_bits)),
            f32::from_bits(x87_quiet_bits(current_bits)),
        );
        index += 1;
    }
}

// Translated from 00979bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Projectile::SaveGame` (Xbox PDB name of `009c4ff0`) with the one stack
/// word.
pub fn fn_00979bb0(e: &mut Engine, this: Ptr, flags: u32) {
    e.call(0x009c_4ff0, &args![this, flags]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_9510, fn_00979510(Ptr, u32, u32, u32, u32, u32, u32) -> bool),
        entry!(0x0097_95e0, fn_009795e0(Ptr, u32, f32)),
        entry!(0x0097_9840, fn_00979840(Ptr, u32) -> f32),
        entry!(0x0097_9a70, fn_00979a70(Ptr, u32, f32, f32)),
        entry!(0x0097_9bb0, fn_00979bb0(Ptr, u32)),
        entry!(0x0097_95c0, fn_009795c0(Ptr, f32)),
        entry!(0x0097_9bd0, fn_00979bd0(Ptr, u32) -> Ptr),
        entry!(0x0097_9c80, fn_00979c80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn float_setter_quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd8);
        e.call(0x0097_95c0, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xd4), 2.5);
        e.call(0x0097_95c0, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xd4), 0x7fc0_0001);
    }

    #[test]
    fn constructor_builds_the_lists_and_reads_the_kind_table() {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.map(0x011a_4000, 0x1000);
        e.set_global(KIND_TABLE + 3 * 0xc, 0x5555u32);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_9bd0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), COMBAT_ACTION_VTABLE);
        assert_eq!(e.mem.u32(this + 0x24), 3);
        assert_eq!(e.mem.u32(this + 0x28), 0x5555);
        assert_eq!(
            calls(&e, LIST_CONSTRUCT),
            vec![
                vec![this + 4],
                vec![this + 0xc],
                vec![this + 0x14],
                vec![this + 0x1c]
            ]
        );
    }

    #[test]
    fn destructor_frees_all_four_lists() {
        let mut e = Engine::new();
        let empty = e.mem.alloc(4);
        e.register_double(NODE_ITEM, move |e, a| {
            let node = e.mem.u32(a[0]);
            if node == 0 {
                empty.into_ret()
            } else {
                node.into_ret()
            }
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let node = e.mem.u32(a[0]);
            let next = e.mem.u32(node + 4);
            e.mem.set_u32(a[0], next);
            Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        for (i, offset) in LISTS.iter().enumerate() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, 0x6000 + i as u32);
            e.mem.set_u32(this + offset, node);
        }
        e.call_log = Some(vec![]);
        e.call(0x0097_9c80, &args![this]);
        assert_eq!(e.mem.u32(this), COMBAT_ACTION_VTABLE);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![0x6002], vec![0x6000], vec![0x6001], vec![0x6003]]
        );
        assert_eq!(
            calls(&e, LIST_DESTRUCT),
            vec![
                vec![this + 0x1c],
                vec![this + 0x14],
                vec![this + 0xc],
                vec![this + 4]
            ]
        );
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    fn map_data(e: &mut Engine) {
        e.map(DOUBLE_ZERO, 8);
        e.map(ZERO_POINT, 12);
    }

    #[test]
    fn fn_00979510_adds_the_impact_and_keeps_the_length() {
        let mut e = Engine::new();
        let this = object(&mut e, 0xd8, 0x7100_0000);
        stub(&mut e, 0x0082_56d0, 1);
        stub(&mut e, 0x0041_81e0, 0x0e0e);
        stub(&mut e, 0x009c_2c80, 1);
        stub(&mut e, 0x009b_f5a0, 1);
        stub(&mut e, 0x7100_01f4, 0x0f0f);
        e.register(0x0043_9ef0, |_, a| ret(a[1]));
        e.register(0x0045_7990, |_, _| ret_f(12.5));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0097_9510, &[this, 1, 2, 3, 4, 5, 6]).bool());
        assert_eq!(calls_to(&e, 0x009c_2c80), vec![vec![1, 4, this, 0x0e0e]]);
        assert_eq!(
            calls_to(&e, 0x009b_f5a0),
            vec![vec![this, 1, 2, 3, 4, 5, 6]]
        );
        assert_eq!(calls_to(&e, 0x0043_9ef0)[0][0], 2);
        assert_eq!(calls_to(&e, 0x0043_9ef0)[0][2], 0x0f0f);
        assert_eq!(e.mem.f32(this + 0xd4), 12.5);
        stub(&mut e, 0x009c_2c80, 0);
        assert!(!e.call(0x0097_9510, &[this, 1, 2, 3, 4, 5, 6]).bool());
        stub(&mut e, 0x0082_56d0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0097_9510, &[this, 1, 2, 3, 4, 5, 6]).bool());
        assert!(calls_to(&e, 0x0041_81e0).is_empty());
    }

    /// Geometry data whose vertex array holds `ys` (y of 12-byte entries)
    /// and a second array of the same count.
    fn vertex_data(e: &mut Engine, ys: &[f32]) -> (u32, u32) {
        let vertices = e.mem.alloc(0xc * ys.len() as u32);
        let second = e.mem.alloc(8 * ys.len() as u32);
        for (i, &y) in ys.iter().enumerate() {
            e.mem.set_f32(vertices + 0xc * i as u32 + 4, y);
            e.mem.set_f32(second + 8 * i as u32 + 4, y);
        }
        stub(e, VERTICES, vertices);
        stub(e, SECOND_ARRAY, second);
        stub(e, VERTEX_COUNT, ys.len() as u32);
        (vertices, second)
    }

    #[test]
    fn fn_009795e0_scales_the_vertex_arrays() {
        let mut e = Engine::new();
        map_data(&mut e);
        let transform = e.mem.alloc(0x10);
        e.mem.set_f32(transform + 0xc, 2.0);
        stub(&mut e, 0x0053_7b10, transform);
        stub_all(&mut e, &[0x0043_9180]);
        stub(&mut e, GEOMETRY_DATA, 0xda7a);
        stub(&mut e, IS_KIND, 0);
        stub(&mut e, SKIN_DATA, 0);
        let (vertices, second) = vertex_data(&mut e, &[1.0, 3.0]);
        e.call_log = Some(vec![]);
        e.call(0x0097_95e0, &[0, 0x3d00, 1.5f32.to_bits()]);
        assert_eq!(e.mem.f32(transform + 0xc), 3.0);
        assert_eq!(
            calls_to(&e, 0x0043_9180),
            vec![vec![transform, 1.5f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(vertices + 4), 1.5);
        assert_eq!(e.mem.f32(vertices + 0x10), 4.5);
        assert_eq!(e.mem.f32(second + 0xc), 4.5);
        // A null node does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0097_95e0, &[0, 0, 1.5f32.to_bits()]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_009795e0_scales_the_children_of_a_kind_node() {
        let mut e = Engine::new();
        map_data(&mut e);
        let transform = e.mem.alloc(0x10);
        stub(&mut e, 0x0053_7b10, transform);
        stub_all(&mut e, &[0x0043_9180, CHILD_SET_LENGTH]);
        stub(&mut e, GEOMETRY_DATA, 0xda7a);
        e.register(IS_KIND, |_, a| ret(u32::from(a[1] != 0xc002)));
        stub(&mut e, CHILD_COUNT, 2);
        e.register(CHILD_AT, |_, a| ret(0xc001 + a[1]));
        e.register(CHILD_LENGTH, |_, _| ret_f(4.0));
        e.call_log = Some(vec![]);
        e.call(0x0097_95e0, &[0, 0x3d00, 0.5f32.to_bits()]);
        assert_eq!(
            calls_to(&e, CHILD_SET_LENGTH),
            vec![vec![0xc001, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_009795e0_scales_a_locked_skin() {
        let mut e = Engine::new();
        map_data(&mut e);
        let transform = e.mem.alloc(0x10);
        stub(&mut e, 0x0053_7b10, transform);
        stub_all(
            &mut e,
            &[0x0043_9180, ACCESSOR_CONSTRUCT, READ_DATA, UNLOCK_DATA],
        );
        stub(&mut e, GEOMETRY_DATA, 0xda7a);
        stub(&mut e, IS_KIND, 0);
        let skin = object(&mut e, 0x10, 0x7400_0000);
        stub(&mut e, SKIN_DATA, skin);
        stub(&mut e, 0x7400_008c, 1);
        stub(&mut e, LOCK_DATA, 1);
        stub(&mut e, SKIN_VERTEX_COUNT, 2);
        e.register(ACCESSOR_GET, |e, a| {
            e.mem.set_f32(a[2] + 4, 2.0 + a[1] as f32);
            Ret::default()
        });
        let seen = e.mem.alloc(8);
        e.register_double(ACCESSOR_SET, move |e, a| {
            let y = e.mem.u32(a[2] + 4);
            e.mem.set_u32(seen + 4 * a[1], y);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0097_95e0, &[0, 0x3d00, 2.0f32.to_bits()]);
        assert_eq!(e.mem.f32(seen), 4.0);
        assert_eq!(e.mem.f32(seen + 4), 6.0);
        assert_eq!(calls_to(&e, LOCK_DATA), vec![vec![0xda7a, 1]]);
        assert_eq!(calls_to(&e, UNLOCK_DATA), vec![vec![0xda7a]]);
        assert!(calls_to(&e, VERTICES).is_empty());
    }

    #[test]
    fn fn_00979840_answers_the_largest_y() {
        let mut e = Engine::new();
        map_data(&mut e);
        stub(&mut e, GEOMETRY_DATA, 0xda7a);
        stub(&mut e, IS_KIND, 0);
        stub(&mut e, SKIN_DATA, 0);
        vertex_data(&mut e, &[1.0, 7.0, -3.0]);
        assert_eq!(e.call(0x0097_9840, &[0, 0x3d00]).f32(), 7.0);
        vertex_data(&mut e, &[-1.0, -7.0]);
        assert_eq!(e.call(0x0097_9840, &[0, 0x3d00]).f32(), 0.0);
        stub(&mut e, GEOMETRY_DATA, 0);
        assert_eq!(e.call(0x0097_9840, &[0, 0x3d00]).f32(), 0.0);
    }

    #[test]
    fn fn_00979840_answers_the_first_child_length() {
        let mut e = Engine::new();
        map_data(&mut e);
        stub(&mut e, GEOMETRY_DATA, 0xda7a);
        e.register(IS_KIND, |_, a| ret(u32::from(a[1] != 0xc001)));
        stub(&mut e, CHILD_COUNT, 3);
        e.register(CHILD_AT, |_, a| ret(0xc001 + a[1]));
        e.register(CHILD_LENGTH, |_, a| ret_f(f64::from(a[0] - 0xc000)));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_9840, &[0, 0x3d00]).f32(), 2.0);
        assert_eq!(calls_to(&e, CHILD_LENGTH), vec![vec![0xc002]]);
    }

    #[test]
    fn fn_00979a70_stretches_the_kind_nodes() {
        let mut e = Engine::new();
        map_data(&mut e);
        let root = object(&mut e, 0x10, 0x7100_0000);
        stub(&mut e, 0x7100_000c, 0x1c00);
        stub(&mut e, 0x0043_b480, 1);
        stub(&mut e, 0x0043_b4a0, 0xbea0);
        e.register(IS_KIND, |_, a| ret(u32::from(a[1] == 0xbea0)));
        let translation = e.mem.alloc(12);
        stub(&mut e, 0x0043_c490, translation);
        stub_all(&mut e, &[0x0043_9180, 0x0044_0460]);
        // The scaling of the beam's geometry, recorded through its first call.
        let transform = e.mem.alloc(0x10);
        stub(&mut e, 0x0053_7b10, transform);
        stub(&mut e, GEOMETRY_DATA, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0097_9a70,
            &[0x9000, root, 6.0f32.to_bits(), 2.0f32.to_bits()],
        );
        // Both nodes get the translation scaled by 6 / 2 and the beam its
        // geometry by the same.
        assert_eq!(calls_to(&e, 0x0044_0460).len(), 2);
        let scales = calls_to(&e, 0x0043_9180);
        assert_eq!(scales[0][1], 3.0f32.to_bits());
        assert_eq!(scales[2][1], 3.0f32.to_bits());
        // Without a current length the beam's own length divides.
        e.register(0x0054_95f0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0097_9a70, &[0x9000, 0xbea0, 6.0f32.to_bits(), 0]);
        assert!(calls_to(&e, 0x0044_0460).is_empty());
        assert!(calls_to(&e, 0x0053_7b10).is_empty());
    }

    #[test]
    fn fn_00979bb0_saves_through_the_projectile() {
        let mut e = Engine::new();
        stub_all(&mut e, &[0x009c_4ff0]);
        e.call_log = Some(vec![]);
        e.call(0x0097_9bb0, &[0x9000, 7]);
        assert_eq!(calls_to(&e, 0x009c_4ff0), vec![vec![0x9000, 7]]);
    }
}
