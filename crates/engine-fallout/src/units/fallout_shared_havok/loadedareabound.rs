//! `fallout shared/havok/loadedareabound.cpp` (Xbox PDB source unit),
//! translated from FalloutNV.exe 1.4.0.525.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `LoadedAreaBound` (Xbox PDB), 0x44 bytes in the prototype and PC code.
    pub struct LoadedAreaBound: 0x44 {
        /// `pCurrentCell` (Xbox PDB); PC access at `006296fa`.
        0x20 pCurrentCell: Ptr,
    }
}

const LOADED_BOUND_VTABLE: u32 = 0x0104_e130;
const HIT_MAP_VTABLE: u32 = 0x0104_e150;
const MAP_BASE_VTABLE: u32 = 0x0104_e170;
const SIMPLE_ARRAY_VTABLE: u32 = 0x0104_e190;
const TES_SINGLETON: u32 = 0x011d_ea10;

fn copy_point(e: &mut Engine, dst: u32, src: u32) {
    for offset in [0, 4, 8] {
        e.mem.set_u32(dst + offset, e.mem.u32(src + offset));
    }
}

// Translated from 006270c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedAreaBound::_scalar_deleting_destructor_` (Xbox PDB).
pub fn loaded_area_bound_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LoadedAreaBound>,
    flags: u32,
) -> Ptr<LoadedAreaBound> {
    fn_006270f0(e, this);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x44u32]);
    }
    this
}

// Translation incomplete: 006270f0 - MSVC SEH exception cleanup is omitted.
// Translated from 006270f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normal destruction of `LoadedAreaBound`; exception cleanup remains open.
pub fn fn_006270f0(e: &mut Engine, this: Ptr<LoadedAreaBound>) {
    e.mem.set_u32(this.addr(), LOADED_BOUND_VTABLE);
    for index in 0..6 {
        e.call(0x0066_b0d0, &args![this.addr() + 8 + index * 4, 0u32]);
    }
    e.call(0x0043_8af0, &args![this.addr() + 0x24]);
    fn_00629e50(e, Ptr::new(this.addr() + 0x24));
    e.call(
        0x00ec_5fce,
        &args![this.addr() + 8, 4u32, 6u32, 0x0045_cec0u32],
    );
    e.call(0x0049_6910, &args![this]);
}

// Translated from 006295e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedAreaBound::WarpActor` (Xbox PDB): finds and applies a warp point.
pub fn loaded_area_bound_warp_actor(e: &mut Engine, this: Ptr<LoadedAreaBound>, actor: Ptr) {
    e.with_stack(12, |e, point| {
        e.call(0x0068_15c0, &args![point]);
        // The pending `point` push belongs to FindWarpLocation: slot 0x1f4
        // has no stack arguments.
        let actor_point = e.vcall(actor.addr(), 0x1f4, &[]).u32();
        loaded_area_bound_find_warp_location(e, this, actor, Ptr::new(actor_point), point);
        e.vcall(actor.addr(), 0x2a8, &args![point]);
        let controller = e.call(0x0093_06d0, &args![actor]).u32();
        if controller != 0 {
            let controller = e.call(0x0093_06d0, &args![actor]).u32();
            fn_00629650(e, Ptr::new(controller + 0x410), false);
        }
    });
}

// Translated from 00629650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears controller flag 0x20.
pub fn fn_00629650(e: &mut Engine, this: Ptr, enabled: bool) {
    fn_00629670(e, this, 0x20, enabled);
}

// Translated from 00629670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears a controller flag mask.
pub fn fn_00629670(e: &mut Engine, this: Ptr, mask: u32, enabled: bool) {
    if enabled {
        e.mem
            .set_u32(this.addr() + 4, e.mem.u32(this.addr() + 4) | mask);
    } else {
        fn_006296b0(e, this, mask);
    }
}

// Translated from 006296b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a controller flag mask.
pub fn fn_006296b0(e: &mut Engine, this: Ptr, mask: u32) {
    e.mem
        .set_u32(this.addr() + 4, e.mem.u32(this.addr() + 4) & !mask);
}

// Translated from 006296d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedAreaBound::FindWarpLocation` (Xbox PDB): tries navmesh Z, the
/// actor's current point, an alternate cell point, then collision validation.
pub fn loaded_area_bound_find_warp_location(
    e: &mut Engine,
    this: Ptr<LoadedAreaBound>,
    actor: Ptr,
    requested: Ptr,
    out: Ptr,
) {
    e.with_stack(0x80, |e, stack| {
        let base = stack.addr();
        let transform = base;
        let shape = base + 0x20;
        let alternate = base + 0x3c;
        let nav_z = base + 0x48;
        let candidate = base + 0x4c;
        let normal = base + 0x5c;
        let auxiliary = base + 0x74;
        e.mem.set_u32(auxiliary, 0);
        let cell = e.get(this, LoadedAreaBound::pCurrentCell);
        let eligible = !actor.is_null() && e.vcall(actor.addr(), 0x100, &[]).bool();
        e.call(0x0068_15c0, &args![normal]);
        copy_point(e, candidate, requested.addr());
        let mut valid = false;
        if !actor.is_null() {
            // The pending nav_z pointer is for Pathing::GetNavMeshZForLocation.
            let world = e.call(0x0054_ddd0, &args![cell]).u32();
            if e.call(0x006d_97b0, &args![candidate, cell, world, nav_z])
                .bool()
            {
                // FLD/FSTP copies the float and quiets a signaling NaN.
                let mut bits = e.mem.u32(nav_z);
                if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
                    bits |= 0x0040_0000;
                }
                e.mem.set_u32(candidate + 8, bits);
                copy_point(e, out.addr(), candidate);
                valid = true;
            }
        }
        if !valid && eligible {
            let current_cell = e.vcall(actor.addr(), 0x298, &[]).u32();
            // The comparison reloads this->pCurrentCell after the callback.
            if current_cell == e.get(this, LoadedAreaBound::pCurrentCell).addr() {
                let point = e.vcall(actor.addr(), 0x29c, &args![alternate]).u32();
                copy_point(e, out.addr(), point);
                valid = true;
            }
        }
        if !valid
            && !eligible
            && !actor.is_null()
            && e.vcall(
                actor.addr(),
                0x138,
                &args![candidate, normal, auxiliary, 0u32],
            )
            .bool()
        {
            let tes: u32 = e.global(TES_SINGLETON);
            let found = e
                .call(
                    0x0045_19d0,
                    &args![
                        tes,
                        e.mem.u32(candidate),
                        e.mem.u32(candidate + 4),
                        e.mem.u32(candidate + 8)
                    ],
                )
                .u32();
            if found != 0 && e.call(0x0045_0fd0, &args![found]).u32() == 6 {
                copy_point(e, out.addr(), candidate);
                valid = true;
            }
        }
        if valid {
            e.call(0x0068_15c0, &args![shape]);
            e.call(0x004a_3e00, &args![shape, out]);
            e.call(0x0043_7c10, &args![transform]);
            e.call(0x0062_95a0, &args![transform]);
            for (offset, addend) in [(0x10, 0x10), (0x1c, 0)] {
                let body = e.call(0x0055_9450, &args![this.addr() + offset]).u32();
                let body = e.call(0x0062_0b80, &args![body]).u32();
                let body = e.call(0x0098_4230, &args![body]).u32();
                e.call(0x0062_9560, &args![transform, body + addend]);
            }
            valid = fn_00629930(e, Ptr::new(transform), Ptr::new(shape)) != 0;
        }
        if !valid {
            e.call(0x0054_cfd0, &args![cell, out, normal]);
        }
    });
}

// Translated from 00629930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transforms two collision shapes, compares them and tests mode 7.
pub fn fn_00629930(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.with_stack(0x40, |e, stack| {
        let base = stack.addr();
        e.call(0x0045_8a60, &args![this, base + 0x20, other]);
        e.call(0x0045_8a60, &args![other, base + 0x10, this.addr() + 0x10]);
        e.call(0x0062_8cc0, &args![base, base + 0x20, base + 0x10]);
        e.call(0x0045_8990, &args![base, 7u32]).u32()
    })
}

// Translated from 006299a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the embedded Havok pointer array.
pub fn fn_006299a0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00629a80(e, this);
    this
}

// Translated from 006299c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts a body handle and appends the supplied pointer.
pub fn fn_006299c0(e: &mut Engine, this: Ptr, value: Ptr) {
    e.with_stack(4, |e, temp| {
        e.mem.set_u8(temp.addr(), 0);
        let allocator = e.call(0x004a_4840, &args![temp, this]).u32();
        fn_00629ad0(e, this, Ptr::new(allocator), value);
    });
}

// Translated from 006299f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the rigid-body hit map.
pub fn fn_006299f0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_00629ba0(e, this, hash_size);
    e.mem.set_u32(this.addr(), HIT_MAP_VTABLE);
    this
}

// Translated from 00629a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts a body handle and reserves array space.
pub fn fn_00629a20(e: &mut Engine, this: Ptr, size: i32) {
    e.with_stack(4, |e, temp| {
        e.mem.set_u8(temp.addr(), 0);
        let allocator = e.call(0x004a_4840, &args![temp, this]).u32();
        fn_00629f70(e, this, Ptr::new(allocator), size);
    });
}

// Translated from 00629a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<bhkRigidBody *,unsigned short>` scalar destructor (Xbox PDB).
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00629e50(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00629a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes an `hkArray` header: empty, with the static-capacity flag.
pub fn fn_00629a80(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0x8000_0000);
    this
}

// Translated from 00629ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of an array entry, including out-of-range indices.
pub fn fn_00629ab0(e: &mut Engine, this: Ptr, index: i32) -> Ptr {
    Ptr::new(
        e.mem
            .u32(this.addr())
            .wrapping_add((index as u32).wrapping_mul(4)),
    )
}

// Translated from 00629ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows an `hkArray` if full, copies one pointer, and increments its count.
pub fn fn_00629ad0(e: &mut Engine, this: Ptr, allocator: Ptr, value: Ptr) {
    let capacity = e.call(0x0062_a100, &args![this]).u32();
    let count = e.mem.u32(this.addr() + 4);
    if count == capacity {
        e.call(0x00c9_0940, &args![allocator, this, 4u32]);
    }
    let destination = e
        .mem
        .u32(this.addr())
        .wrapping_add(e.mem.u32(this.addr() + 4).wrapping_mul(4));
    e.call(0x0055_9a00, &args![destination, 1u32, value, 0u32]);
    e.mem
        .set_u32(this.addr() + 4, e.mem.u32(this.addr() + 4).wrapping_add(1));
}

// Translation incomplete: 00629b40 - MSVC SEH exception cleanup is omitted.
// Translated from 00629b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normal destruction of a helper object; exception cleanup remains open.
pub fn fn_00629b40(e: &mut Engine, this: Ptr) {
    fn_0062a080(e, this);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 00629ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a hash map and clears its bucket array.
pub fn fn_00629ba0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let bytes = hash_size.wrapping_mul(4);
    let table = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, table);
    // The allocator callback can change m_uiHashSize before this reload.
    let clear_bytes = e.mem.u32(this.addr() + 4).wrapping_mul(4);
    e.call(0x0040_3d30, &args![table, 0u32, clear_bytes]);
    this
}

// Translated from 00629c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a body-to-hit-count entry or updates its existing value.
pub fn fn_00629c10(e: &mut Engine, this: Ptr, key: Ptr, value: u16) {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.mem.u32(this.addr() + 8);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let equal = e
            .vcall(this.addr(), 8, &args![key, e.mem.u32(item + 4)])
            .bool();
        if equal {
            e.mem.set_u16(item + 8, value);
            return;
        }
        item = e.mem.u32(item);
    }
    item = e.vcall(this.addr(), 0x14, &[]).u32();
    e.vcall(this.addr(), 0xc, &args![item, key, value]);
    // Both callbacks can relocate the table; the PC code reloads it.
    let table = e.mem.u32(this.addr() + 8);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(item, e.mem.u32(slot));
    e.mem.set_u32(slot, item);
    e.mem.set_u32(
        this.addr() + 0xc,
        e.mem.u32(this.addr() + 0xc).wrapping_add(1),
    );
}

// Translated from 00629cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up a body key, writing the 16-bit hit count only when found.
pub fn fn_00629cf0(e: &mut Engine, this: Ptr, key: Ptr, out: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.mem.u32(this.addr() + 8);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        if e.vcall(this.addr(), 8, &args![key, e.mem.u32(item + 4)])
            .bool()
        {
            e.mem.set_u16(out.addr(), e.mem.u16(item + 8));
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 00629d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads an iterator entry and advances to the next chain or bucket.
pub fn fn_00629d70(e: &mut Engine, this: Ptr, iterator: Ptr, key_out: Ptr, value_out: Ptr) {
    let item = e.mem.u32(iterator.addr());
    e.mem.set_u32(key_out.addr(), e.mem.u32(item + 4));
    e.mem.set_u16(value_out.addr(), e.mem.u16(item + 8));
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let bucket = e.vcall(this.addr(), 4, &args![e.mem.u32(item + 4)]).u32();
    let count = e.mem.u32(this.addr() + 4);
    let table = e.mem.u32(this.addr() + 8);
    for index in bucket.wrapping_add(1)..count {
        let next = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if next != 0 {
            e.mem.set_u32(iterator.addr(), next);
            return;
        }
    }
    e.mem.set_u32(iterator.addr(), 0);
}

// Translated from 00629e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::SetValue` (Xbox PDB): stores key and 16-bit value.
pub fn ni_t_map_base_set_value(e: &mut Engine, _this: Ptr, item: Ptr, key: Ptr, value: u16) {
    e.mem.set_u32(item.addr() + 4, key.addr());
    e.mem.set_u16(item.addr() + 8, value);
}

// Translation incomplete: 00629e50 - MSVC SEH exception cleanup is omitted.
// Translated from 00629e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normal destruction of the hit map; exception cleanup remains open.
pub fn fn_00629e50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HIT_MAP_VTABLE);
    e.call(0x0043_8af0, &args![this]);
    fn_00629eb0(e, this);
}

// Translated from 00629eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a map base and frees its bucket table.
pub fn fn_00629eb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(0x0043_8af0, &args![this]);
    e.call(0x00aa_10f0, &args![e.mem.u32(this.addr() + 8)]);
}

// Translated from 00629ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<bhkRigidBody *,unsigned short>::DeleteItem` (Xbox PDB).
pub fn ni_t_map_delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u16(item.addr() + 8, 0);
    e.call(0x006b_8310, &args![this.addr() + 0xc, item]);
}

// Translated from 00629f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds an equal pointer in a signed index range, or returns -1.
pub fn fn_00629f10(e: &mut Engine, this: Ptr, needle: Ptr, start: i32, end: i32) -> i32 {
    let end = if end < 0 {
        e.mem.u32(this.addr() + 4) as i32
    } else {
        end
    };
    let mut index = start;
    while index < end {
        let base = e.mem.u32(this.addr());
        let at = base.wrapping_add((index as u32).wrapping_mul(4));
        if e.mem.u32(at) == e.mem.u32(needle.addr()) {
            return index;
        }
        index = index.wrapping_add(1);
    }
    -1
}

// Translated from 00629f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reserves at least `required` elements; doubles capacity when larger.
pub fn fn_00629f70(e: &mut Engine, this: Ptr, allocator: Ptr, required: i32) {
    let capacity = e.call(0x0062_a100, &args![this]).i32();
    if capacity < required {
        let doubled = capacity.wrapping_shl(1);
        let target = if doubled > required {
            doubled
        } else {
            required
        };
        e.call(0x00c9_08c0, &args![allocator, this, target, 4u32]);
    }
}

// Translated from 00629fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a simple pointer array with zero initial size.
pub fn fn_00629fd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_VTABLE);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0062a000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the simple pointer array's contents.
pub fn fn_0062a000(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 0062a020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the map base (Xbox PDB).
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00629eb0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0062a050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `BSSimpleArray<bhkRigidBody *,1024>` (Xbox PDB).
pub fn bs_simple_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0062a000(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0062a080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts an array handle and releases it.
pub fn fn_0062a080(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, temp| {
        e.mem.set_u8(temp.addr(), 0);
        let allocator = e.call(0x004a_4840, &args![temp, this]).u32();
        e.call(0x0062_a0b0, &args![this, allocator]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006270c0,
            loaded_area_bound_scalar_deleting_destructor(
                Ptr<LoadedAreaBound>,
                u32,
            ) -> Ptr<LoadedAreaBound>
        ),
        entry!(0x006270f0, fn_006270f0(Ptr<LoadedAreaBound>)),
        entry!(
            0x006295e0,
            loaded_area_bound_warp_actor(Ptr<LoadedAreaBound>, Ptr)
        ),
        entry!(0x00629650, fn_00629650(Ptr, bool)),
        entry!(0x00629670, fn_00629670(Ptr, u32, bool)),
        entry!(0x006296b0, fn_006296b0(Ptr, u32)),
        entry!(
            0x006296d0,
            loaded_area_bound_find_warp_location(Ptr<LoadedAreaBound>, Ptr, Ptr, Ptr)
        ),
        entry!(0x00629930, fn_00629930(Ptr, Ptr) -> u32),
        entry!(0x006299a0, fn_006299a0(Ptr) -> Ptr),
        entry!(0x006299c0, fn_006299c0(Ptr, Ptr)),
        entry!(0x006299f0, fn_006299f0(Ptr, u32) -> Ptr),
        entry!(0x00629a20, fn_00629a20(Ptr, i32)),
        entry!(0x00629a50, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00629a80, fn_00629a80(Ptr) -> Ptr),
        entry!(0x00629ab0, fn_00629ab0(Ptr, i32) -> Ptr),
        entry!(0x00629ad0, fn_00629ad0(Ptr, Ptr, Ptr)),
        entry!(0x00629b40, fn_00629b40(Ptr)),
        entry!(0x00629ba0, fn_00629ba0(Ptr, u32) -> Ptr),
        entry!(0x00629c10, fn_00629c10(Ptr, Ptr, u16)),
        entry!(0x00629cf0, fn_00629cf0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00629d70, fn_00629d70(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00629e20, ni_t_map_base_set_value(Ptr, Ptr, Ptr, u16)),
        entry!(0x00629e50, fn_00629e50(Ptr)),
        entry!(0x00629eb0, fn_00629eb0(Ptr)),
        entry!(0x00629ee0, ni_t_map_delete_item(Ptr, Ptr)),
        entry!(0x00629f10, fn_00629f10(Ptr, Ptr, i32, i32) -> i32),
        entry!(0x00629f70, fn_00629f70(Ptr, Ptr, i32)),
        entry!(0x00629fd0, fn_00629fd0(Ptr) -> Ptr),
        entry!(0x0062a000, fn_0062a000(Ptr)),
        entry!(0x0062a020, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0062a050, bs_simple_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0062a080, fn_0062a080(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn map_vtable(e: &mut Engine, map: Ptr) {
        e.map(0x0020_0000, 0x1000);
        e.mem.set_u32(map.addr(), 0x0020_0000);
        e.mem.set_u32(0x0020_0004, 0x0020_1000);
        e.mem.set_u32(0x0020_0008, 0x0020_1004);
        e.mem.set_u32(0x0020_000c, 0x0020_1008);
        e.mem.set_u32(0x0020_0014, 0x0020_100c);
        e.register(0x0020_1000, |_, _| Ret {
            eax: 0,
            ..Ret::default()
        });
        e.register(0x0020_1004, |_, a| Ret {
            eax: (a[1] == a[2]) as u32,
            ..Ret::default()
        });
        e.register(0x0020_1008, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u16(a[1] + 8, a[3] as u16);
            Ret::default()
        });
        e.register(0x0020_100c, |e, _| Ret {
            eax: e.mem.alloc(12),
            ..Ret::default()
        });
    }

    fn warp_collision_doubles(e: &mut Engine) {
        noop(
            e,
            &[
                0x0068_15c0,
                0x004a_3e00,
                0x0043_7c10,
                0x0062_95a0,
                0x0055_9450,
                0x0062_0b80,
                0x0098_4230,
                0x0062_9560,
                0x0045_8a60,
                0x0062_8cc0,
                0x0045_8990,
                0x0054_cfd0,
            ],
        );
    }

    #[test]
    fn warp_location_uses_reloaded_current_cell_after_actor_callback() {
        let mut e = Engine::new();
        let bound: Ptr<LoadedAreaBound> = e.new_object();
        let actor: Ptr = Ptr::new(e.mem.alloc(4));
        let requested: Ptr = Ptr::new(e.mem.alloc(12));
        let out: Ptr = Ptr::new(e.mem.alloc(12));
        let actor_point = e.mem.alloc(12);
        let updated_cell = 0x1234u32;
        e.mem.set_u32(actor.addr(), 0x0020_0000);
        e.map(0x0020_0000, 0x1000);
        e.mem.set_u32(0x0020_0100, 0x0020_1000);
        e.mem.set_u32(0x0020_0298, 0x0020_1004);
        e.mem.set_u32(0x0020_029c, 0x0020_1008);
        e.register_double(0x0020_1000, move |e, _| {
            e.mem.set_u32(bound.addr() + 0x20, updated_cell);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(0x0020_1004, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register_double(0x0020_1008, move |_, _| Ret {
            eax: actor_point,
            ..Ret::default()
        });
        e.register(0x0054_ddd0, |_, _| Ret::default());
        e.register(0x006d_97b0, |_, _| Ret::default());
        warp_collision_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0062_96d0, &args![bound, actor, requested, out]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0020_1008));
    }

    #[test]
    fn navmesh_warp_quiets_signaling_nan_and_preserves_pending_arguments() {
        let mut e = Engine::new();
        let bound: Ptr<LoadedAreaBound> = e.new_object();
        let actor: Ptr = Ptr::new(e.mem.alloc(4));
        let requested: Ptr = Ptr::new(e.mem.alloc(12));
        let out: Ptr = Ptr::new(e.mem.alloc(12));
        e.mem.set_u32(actor.addr(), 0x0020_0000);
        e.map(0x0020_0000, 0x1000);
        e.mem.set_u32(0x0020_0100, 0x0020_1000);
        e.register(0x0020_1000, |_, _| Ret::default());
        e.register(0x0054_ddd0, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(0x006d_97b0, |e, a| {
            e.mem.set_u32(a[3], 0x7f80_0001);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        warp_collision_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0062_96d0, &args![bound, actor, requested, out]);
        assert_eq!(e.mem.u32(out.addr() + 8), 0x7fc0_0001);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x0054_ddd0).unwrap().1,
            vec![e.get(bound, LoadedAreaBound::pCurrentCell).addr()]
        );
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x006d_97b0).unwrap().1.len(),
            4
        );
    }

    #[test]
    fn warp_actor_slot_one_f_nine_four_takes_no_pending_point_argument() {
        let mut e = Engine::new();
        let bound: Ptr<LoadedAreaBound> = e.new_object();
        let actor: Ptr = Ptr::new(e.mem.alloc(4));
        let actor_point = e.mem.alloc(12);
        let controller = e.mem.alloc(0x418);
        e.mem.set_u32(controller + 0x414, 0x20);
        e.mem.set_u32(actor.addr(), 0x0020_0000);
        e.map(0x0020_0000, 0x1000);
        for (slot, target) in [
            (0x1f4, 0x0020_1000),
            (0x2a8, 0x0020_1004),
            (0x100, 0x0020_1008),
            (0x138, 0x0020_100c),
        ] {
            e.mem.set_u32(0x0020_0000 + slot, target);
        }
        e.register_double(0x0020_1000, move |_, _| Ret {
            eax: actor_point,
            ..Ret::default()
        });
        e.register(0x0020_1004, |_, _| Ret::default());
        e.register(0x0020_1008, |_, _| Ret::default());
        e.register(0x0020_100c, |_, _| Ret::default());
        e.register(0x0054_ddd0, |_, _| Ret::default());
        e.register(0x006d_97b0, |_, _| Ret::default());
        e.register(0x0054_cfd0, |_, _| Ret::default());
        e.register_double(0x0093_06d0, move |_, _| Ret {
            eax: controller,
            ..Ret::default()
        });
        noop(&mut e, &[0x0068_15c0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_95e0, &args![bound, actor]);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x0020_1000).unwrap().1,
            vec![actor.addr()]
        );
        assert_eq!(e.mem.u32(controller + 0x414) & 0x20, 0);
    }

    #[test]
    fn append_wrapper_keeps_value_pending_for_array_append() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        let values = e.mem.alloc(8);
        e.mem.set_u32(array.addr(), values);
        e.register(0x004a_4840, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(0x0062_a100, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(0x0055_9a00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_99c0, &args![array, 0x5678u32]);
        let log = e.call_log.unwrap();
        let convert = log.iter().find(|(a, _)| *a == 0x004a_4840).unwrap();
        assert_eq!(convert.1.len(), 2);
        assert_eq!(convert.1[1], array.addr());
        let copy = log.iter().find(|(a, _)| *a == 0x0055_9a00).unwrap();
        assert_eq!(copy.1, vec![values, 1, 0x5678, 0]);
    }

    #[test]
    fn reserve_wrapper_keeps_size_pending_for_reserve() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        e.register(0x004a_4840, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(0x0062_a100, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register(0x00c9_08c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_9a20, &args![array, 7i32]);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x004a_4840).unwrap().1.len(),
            2
        );
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x00c9_08c0).unwrap().1,
            vec![0x1234, array.addr(), 7, 4]
        );
    }

    #[test]
    fn map_constructor_reloads_hash_size_after_allocator_callback() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register_double(0x00aa_1070, move |e, a| {
            assert_eq!(a, &[8]);
            e.mem.set_u32(map.addr() + 4, 3);
            Ret {
                eax: e.mem.alloc(12),
                ..Ret::default()
            }
        });
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_9ba0, &args![map, 2u32]);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x0040_3d30).unwrap().1[2],
            12
        );
    }

    #[test]
    fn map_insert_reloads_bucket_table_after_new_item_callback() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let old_table = e.mem.alloc(4);
        let new_table = e.mem.alloc(4);
        let prior = e.mem.alloc(12);
        let new_item = e.mem.alloc(12);
        e.mem.set_u32(map.addr() + 8, old_table);
        e.mem.set_u32(new_table, prior);
        map_vtable(&mut e, map);
        e.register_double(0x0020_100c, move |e, _| {
            e.mem.set_u32(map.addr() + 8, new_table);
            Ret {
                eax: new_item,
                ..Ret::default()
            }
        });
        e.call(0x0062_9c10, &args![map, 0x1234u32, 27u16]);
        assert_eq!(e.mem.u32(old_table), 0);
        assert_eq!(e.mem.u32(new_table), new_item);
        assert_eq!(e.mem.u32(new_item), prior);
        assert_eq!(e.mem.u16(new_item + 8), 27);
    }

    #[test]
    fn scalar_deleting_bound_calls_free_only_with_bit_zero() {
        let mut e = Engine::new();
        let this: Ptr<LoadedAreaBound> = e.new_object();
        noop(
            &mut e,
            &[
                0x0066_b0d0,
                0x0043_8af0,
                0x00ec_5fce,
                0x0049_6910,
                0x00aa_10f0,
                0x00aa_1460,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0062_70c0, &args![this, 0u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x00aa_1460));
        e.call(0x0062_70c0, &args![this, 1u32]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x00aa_1460));
    }

    #[test]
    fn bound_destructor_releases_six_phantoms_then_hit_map() {
        let mut e = Engine::new();
        let this: Ptr<LoadedAreaBound> = e.new_object();
        noop(
            &mut e,
            &[
                0x0066_b0d0,
                0x0043_8af0,
                0x00ec_5fce,
                0x0049_6910,
                0x00aa_10f0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0062_70f0, &args![this]);
        let log = e.call_log.unwrap();
        let releases: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0066_b0d0).collect();
        assert_eq!(releases.len(), 6);
        assert_eq!(releases[0].1, vec![this.addr() + 8, 0]);
        assert_eq!(releases[5].1, vec![this.addr() + 0x1c, 0]);
    }

    #[test]
    fn controller_flag_helper_clears_bit_twenty() {
        let mut e = Engine::new();
        let controller: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(controller.addr() + 4, 0x24);
        e.call(0x0062_9650, &args![controller, false]);
        assert_eq!(e.mem.u32(controller.addr() + 4), 4);
    }

    #[test]
    fn controller_flag_setter_takes_both_branches() {
        let mut e = Engine::new();
        let controller: Ptr = Ptr::new(e.mem.alloc(8));
        e.call(0x0062_9670, &args![controller, 0x40u32, true]);
        assert_eq!(e.mem.u32(controller.addr() + 4), 0x40);
        e.call(0x0062_9670, &args![controller, 0x40u32, false]);
        assert_eq!(e.mem.u32(controller.addr() + 4), 0);
    }

    #[test]
    fn controller_flag_clearer_preserves_other_bits() {
        let mut e = Engine::new();
        let controller: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(controller.addr() + 4, 0x33);
        e.call(0x0062_96b0, &args![controller, 0x12u32]);
        assert_eq!(e.mem.u32(controller.addr() + 4), 0x21);
    }

    #[test]
    fn empty_array_constructor_sets_static_capacity_bit() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        assert_eq!(e.call(0x0062_9a80, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr() + 8), 0x8000_0000);
    }

    #[test]
    fn array_constructor_returns_same_pointer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        assert_eq!(e.call(0x0062_99a0, &args![array]).u32(), array.addr());
        assert_eq!(e.mem.u32(array.addr() + 8), 0x8000_0000);
    }

    #[test]
    fn array_entry_address_scales_signed_index() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        e.mem.set_u32(array.addr(), 0x1000);
        assert_eq!(e.call(0x0062_9ab0, &args![array, 3i32]).u32(), 0x100c);
        assert_eq!(e.call(0x0062_9ab0, &args![array, -1i32]).u32(), 0xffc);
    }

    #[test]
    fn map_set_value_stores_short_at_offset_eight() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let item: Ptr = Ptr::new(e.mem.alloc(12));
        e.call(0x0062_9e20, &args![map, item, 0x1234u32, 0x5678u16]);
        assert_eq!(e.mem.u32(item.addr() + 4), 0x1234);
        assert_eq!(e.mem.u16(item.addr() + 8), 0x5678);
    }

    #[test]
    fn map_lookup_finds_and_misses_without_touching_miss_output() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let table = e.mem.alloc(4);
        let item = e.mem.alloc(12);
        let output: Ptr = Ptr::new(e.mem.alloc(2));
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(table, item);
        e.mem.set_u32(item + 4, 0x1234);
        e.mem.set_u16(item + 8, 42);
        e.mem.set_u16(output.addr(), 77);
        map_vtable(&mut e, map);
        assert!(e.call(0x0062_9cf0, &args![map, 0x1234u32, output]).bool());
        assert_eq!(e.mem.u16(output.addr()), 42);
        assert!(!e.call(0x0062_9cf0, &args![map, 0x5678u32, output]).bool());
        assert_eq!(e.mem.u16(output.addr()), 42);
    }

    #[test]
    fn signed_array_search_uses_size_for_negative_end() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(8));
        let data = e.mem.alloc(12);
        let needle: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(array.addr(), data);
        e.mem.set_u32(array.addr() + 4, 3);
        e.mem.set_u32(data + 4, 17);
        e.mem.set_u32(needle.addr(), 17);
        assert_eq!(
            e.call(0x0062_9f10, &args![array, needle, 0i32, -1i32])
                .i32(),
            1
        );
        assert_eq!(
            e.call(0x0062_9f10, &args![array, needle, 2i32, -1i32])
                .i32(),
            -1
        );
    }

    #[test]
    fn collision_check_calls_two_transforms_before_mode_seven() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(24));
        let other: Ptr = Ptr::new(e.mem.alloc(24));
        noop(&mut e, &[0x0045_8a60, 0x0062_8cc0]);
        e.register(0x0045_8990, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_9930, &args![this, other]).u32(), 7);
        let calls: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(
            calls,
            [
                0x0062_9930,
                0x0045_8a60,
                0x0045_8a60,
                0x0062_8cc0,
                0x0045_8990
            ]
        );
    }

    #[test]
    fn hit_map_constructor_overwrites_base_vtable() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x00aa_1070, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        noop(&mut e, &[0x0040_3d30]);
        assert_eq!(e.call(0x0062_99f0, &args![map, 3u32]).u32(), map.addr());
        assert_eq!(e.mem.u32(map.addr()), HIT_MAP_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 3);
    }

    #[test]
    fn hit_map_scalar_destructor_frees_only_when_requested() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0043_8af0, 0x00aa_10f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9a50, &args![map, 0u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
        e.call(0x0062_9a50, &args![map, 1u32]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
    }

    #[test]
    fn append_grows_only_when_count_equals_capacity() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        let data = e.mem.alloc(12);
        e.mem.set_u32(array.addr(), data);
        e.register(0x0062_a100, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        noop(&mut e, &[0x00c9_0940, 0x0055_9a00]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9ad0, &args![array, 0x1234u32, 0x5678u32]);
        assert_eq!(e.mem.u32(array.addr() + 4), 1);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x00c9_0940));
        e.mem.set_u32(array.addr() + 4, 2);
        e.call(0x0062_9ad0, &args![array, 0x1234u32, 0x5678u32]);
        assert_eq!(e.mem.u32(array.addr() + 4), 3);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x00c9_0940));
    }

    #[test]
    fn helper_destructor_releases_array_before_base() {
        let mut e = Engine::new();
        let helper: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x004a_4840, 0x0062_a0b0, 0x0048_3710]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9b40, &args![helper]);
        let calls: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(calls, [0x0062_9b40, 0x004a_4840, 0x0062_a0b0, 0x0048_3710]);
    }

    #[test]
    fn map_insert_updates_existing_item_without_incrementing_count() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let table = e.mem.alloc(4);
        let item = e.mem.alloc(12);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(table, item);
        e.mem.set_u32(item + 4, 0x1234);
        e.mem.set_u16(item + 8, 2);
        map_vtable(&mut e, map);
        e.call(0x0062_9c10, &args![map, 0x1234u32, 9u16]);
        assert_eq!(e.mem.u16(item + 8), 9);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 0);
    }

    #[test]
    fn map_iterator_uses_next_link_or_next_nonempty_bucket() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let table = e.mem.alloc(8);
        let item = e.mem.alloc(12);
        let next = e.mem.alloc(12);
        let iterator: Ptr = Ptr::new(e.mem.alloc(4));
        let key: Ptr = Ptr::new(e.mem.alloc(4));
        let value: Ptr = Ptr::new(e.mem.alloc(2));
        e.mem.set_u32(map.addr() + 4, 2);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(table + 4, next);
        e.mem.set_u32(item + 4, 0x1234);
        e.mem.set_u16(item + 8, 17);
        e.mem.set_u32(iterator.addr(), item);
        map_vtable(&mut e, map);
        e.call(0x0062_9d70, &args![map, iterator, key, value]);
        assert_eq!(e.mem.u32(key.addr()), 0x1234);
        assert_eq!(e.mem.u16(value.addr()), 17);
        assert_eq!(e.mem.u32(iterator.addr()), next);
        e.mem.set_u32(item, next);
        e.mem.set_u32(iterator.addr(), item);
        e.call(0x0062_9d70, &args![map, iterator, key, value]);
        assert_eq!(e.mem.u32(iterator.addr()), next);
    }

    #[test]
    fn map_destructor_sets_derived_vtable_before_base_release() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0043_8af0, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9e50, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), MAP_BASE_VTABLE);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x0043_8af0)
                .count(),
            2
        );
    }

    #[test]
    fn map_base_destructor_frees_bucket_pointer() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x1234);
        noop(&mut e, &[0x0043_8af0, 0x00aa_10f0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9eb0, &args![map]);
        assert_eq!(
            e.call_log
                .unwrap()
                .iter()
                .find(|(a, _)| *a == 0x00aa_10f0)
                .unwrap()
                .1,
            vec![0x1234]
        );
    }

    #[test]
    fn delete_item_clears_hit_count_before_freeing_node() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        let item: Ptr = Ptr::new(e.mem.alloc(12));
        e.mem.set_u16(item.addr() + 8, 17);
        noop(&mut e, &[0x006b_8310]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9ee0, &args![map, item]);
        assert_eq!(e.mem.u16(item.addr() + 8), 0);
        assert_eq!(
            e.call_log.unwrap().last().unwrap().1,
            vec![map.addr() + 0xc, item.addr()]
        );
    }

    #[test]
    fn reserve_doubles_small_capacity_but_skips_sufficient_capacity() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        e.register(0x0062_a100, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        noop(&mut e, &[0x00c9_08c0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_9f70, &args![array, 0x1234u32, 7i32]);
        e.call(0x0062_9f70, &args![array, 0x1234u32, 3i32]);
        let reserves: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00c9_08c0)
            .collect();
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0].1, vec![0x1234, array.addr(), 8, 4]);
    }

    #[test]
    fn simple_array_constructor_passes_two_zero_words() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x006b_3eb0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_9fd0, &args![array]).u32(), array.addr());
        assert_eq!(
            e.call_log.unwrap().last().unwrap().1,
            vec![array.addr(), 0, 0]
        );
    }

    #[test]
    fn simple_array_destructor_passes_one() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0084_54f0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_a000, &args![array]);
        assert_eq!(e.call_log.unwrap().last().unwrap().1, vec![array.addr(), 1]);
    }

    #[test]
    fn map_base_scalar_destructor_obeys_delete_flag() {
        let mut e = Engine::new();
        let map: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0043_8af0, 0x00aa_10f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        e.call(0x0062_a020, &args![map, 0u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
        e.call(0x0062_a020, &args![map, 1u32]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
    }

    #[test]
    fn simple_array_scalar_destructor_obeys_delete_flag() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(16));
        noop(&mut e, &[0x0084_54f0, 0x0040_1030]);
        e.call_log = Some(vec![]);
        e.call(0x0062_a050, &args![array, 0u32]);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
        e.call(0x0062_a050, &args![array, 1u32]);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == 0x0040_1030));
    }

    #[test]
    fn array_release_wrapper_passes_converted_pointer() {
        let mut e = Engine::new();
        let array: Ptr = Ptr::new(e.mem.alloc(12));
        e.register(0x004a_4840, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        noop(&mut e, &[0x0062_a0b0]);
        e.call_log = Some(vec![]);
        e.call(0x0062_a080, &args![array]);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().find(|(a, _)| *a == 0x004a_4840).unwrap().1.len(),
            2
        );
        assert_eq!(log.last().unwrap().1, vec![array.addr(), 0x1234]);
    }
}
