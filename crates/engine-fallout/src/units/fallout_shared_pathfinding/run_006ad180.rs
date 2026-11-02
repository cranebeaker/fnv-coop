//! `fallout shared/pathfinding/run_006ad180` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSScrapArray<NavMeshSearchNode, 1024>`'s and
/// `BSScrapArray<NavMeshSearchLink, 1024>`'s vtables (by their RTTI).
const VTABLE_SCRAP_ARRAY_NODE: u32 = 0x0106_b88c;
const VTABLE_SCRAP_ARRAY_LINK: u32 = 0x0106_b8b4;
/// `PathSmootherPOVSearch`'s vtable (`0106b9cc`, by its RTTI).
const VTABLE_PATH_SMOOTHER_POV_SEARCH: u32 = 0x0106_b9cc;
/// The two arrays' base constructors (`thiscall`).
const NODE_ARRAY_BASE_CONSTRUCT: u32 = 0x006a_bf60;
const LINK_ARRAY_BASE_CONSTRUCT: u32 = 0x006a_bf90;
/// `00401020`: the memory manager (no arguments).
const MEMORY_MANAGER: u32 = 0x0040_1020;
/// `MemoryManager::GetThreadScrapHeap` (engine map), `thiscall`.
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// The arrays' set-up (`thiscall`: reserve, size): `009a3d00` for nodes,
/// `006b3f40` for links.
const NODE_ARRAY_SETUP: u32 = 0x009a_3d00;
const LINK_ARRAY_SETUP: u32 = 0x006b_3f40;
/// The scale `006ad3b0` applies first (`float`).
const SCALE_011D7054: u32 = 0x011d_7054;
/// `0.25` (`double`).
const QUARTER: u32 = 0x0102_90b0;
/// `00406cc0` and `004e4430`: `cdecl`, one float, ST0 (rounding it down
/// and up, by their use here).
const ROUND_DOWN_00406CC0: u32 = 0x0040_6cc0;
const ROUND_UP_004E4430: u32 = 0x004e_4430;
/// `006b31c0`: `PathSmootherPOVSearch`'s base constructor (`thiscall`).
const POV_SEARCH_BASE_CONSTRUCT: u32 = 0x006b_31c0;
/// `006815c0`: returns `this` (the trivial constructor of two members).
const SELF: u32 = 0x0068_15c0;
/// Member constructors (`thiscall`).
const MEMBER_CONSTRUCT_006B3570: u32 = 0x006b_3570;
const MEMBER_CONSTRUCT_006B35C0: u32 = 0x006b_35c0;
const MEMBER_CONSTRUCT_006B3610: u32 = 0x006b_3610;
const MEMBER_CONSTRUCT_006B36E0: u32 = 0x006b_36e0;
/// The zero point (`011f426c`, three words).
const POINT_011F426C: u32 = 0x011f_426c;

/// The two scrap array constructors' shared body.
/// `kind` is (base constructor, vtable, set-up).
fn scrap_array_construct(
    e: &mut Engine,
    this: Ptr,
    kind: [u32; 3],
    reserve: u32,
    size: u32,
    heap: u32,
) -> Ptr {
    let [base_construct, vtable, setup] = kind;
    let base = this.addr();
    e.call(base_construct, &args![base]);
    e.mem.set_u32(base, vtable);
    let heap = if heap != 0 {
        heap
    } else {
        let manager = e.call(MEMORY_MANAGER, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(base + 0x10, heap);
    e.call(setup, &args![base, reserve, size]);
    this
}

// Translated from 006ad1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ad1b0 - the C++ exception frame (FS:[0]) is not translated
/// The `BSScrapArray<NavMeshSearchNode, 1024>` constructor (by the vtable
/// it sets): the base, the vtable, the scrap heap at `+0x10` (`heap`, or the
/// thread's when null), then the set-up for `size` items with `reserve`
/// room (`009a3d00`). Returns `this`.
pub fn fn_006ad1b0(e: &mut Engine, this: Ptr, reserve: u32, size: u32, heap: u32) -> Ptr {
    scrap_array_construct(
        e,
        this,
        [
            NODE_ARRAY_BASE_CONSTRUCT,
            VTABLE_SCRAP_ARRAY_NODE,
            NODE_ARRAY_SETUP,
        ],
        reserve,
        size,
        heap,
    )
}

// Translated from 006ad250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ad250 - the C++ exception frame (FS:[0]) is not translated
/// The `BSScrapArray<NavMeshSearchLink, 1024>` constructor: as
/// [`fn_006ad1b0`] with its own base, vtable and set-up (`006b3f40`).
pub fn fn_006ad250(e: &mut Engine, this: Ptr, reserve: u32, size: u32, heap: u32) -> Ptr {
    scrap_array_construct(
        e,
        this,
        [
            LINK_ARRAY_BASE_CONSTRUCT,
            VTABLE_SCRAP_ARRAY_LINK,
            LINK_ARRAY_SETUP,
        ],
        reserve,
        size,
        heap,
    )
}

// Translated from 006ad2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores a side's 6-bit field in the word at `+0xc`: for
/// `side` other than 2, the bits from `side * 6 + 0x10` (the shift taken
/// modulo 32, as `SHL` does) are cleared and get `count` (at most 15) and,
/// for a count above 1, `first` at 4 bits above and `second` at 5 bits
/// above.
pub fn fn_006ad2f0(e: &mut Engine, this: Ptr, side: u16, count: u16, first: u8, second: u8) {
    if side == 2 {
        return;
    }
    let count = if count > 0xf { 0xf } else { count };
    let mut shift = u32::from(side).wrapping_mul(6).wrapping_add(0x10);
    let field = this.addr() + 0xc;
    let cleared = !(0x3fu32 << (shift & 0x1f)) & e.mem.u32(field);
    e.mem.set_u32(field, cleared);
    if count == 0 {
        return;
    }
    let mut bits = u32::from(count) << (shift & 0x1f);
    if count > 1 {
        shift = shift.wrapping_add(4);
        bits |= u32::from(first) << (shift & 0x1f);
        shift = shift.wrapping_add(1);
        bits |= u32::from(second) << (shift & 0x1f);
    }
    let value = e.mem.u32(field) | bits;
    e.mem.set_u32(field, value);
}

/// `FISTP qword` with chop rounding (the code sets the control word's
/// rounding bits for the store): the low word.
fn truncate(value: f64) -> u32 {
    x87_fistp64_chop(X87Ext::load64(value)) as u32
}

// Translated from 006ad3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: `value` times the scale at `011d7054`, rounded to
/// an integer of at most 15: with `nearest`, down (`00406cc0`) unless the
/// fraction is 0.25 or more (or NaN), then up (`004e4430`); else down when
/// `down` is set, otherwise up.
pub fn fn_006ad3b0(e: &mut Engine, value: f32, down: bool, nearest: bool) -> u32 {
    let scaled = x87_store32(x87_mul(
        x87_load32(value.to_bits()),
        x87_m32(e.global::<u32>(SCALE_011D7054)),
    ));
    let result = if nearest {
        let low = e.call(ROUND_DOWN_00406CC0, &args![scaled]).f64();
        let low = x87_store32(low);
        let fraction = x87_store32(x87_sub(x87_load32(scaled), x87_m32(low)));
        let quarter: f64 = e.global(QUARTER);
        if x87_load32(fraction) < quarter {
            truncate(x87_load32(low))
        } else {
            let high = e.call(ROUND_UP_004E4430, &args![scaled]).f64();
            truncate(high)
        }
    } else if down {
        let low = e.call(ROUND_DOWN_00406CC0, &args![scaled]).f64();
        truncate(low)
    } else {
        let high = e.call(ROUND_UP_004E4430, &args![scaled]).f64();
        truncate(high)
    };
    if result > 0xf {
        0xf
    } else {
        result
    }
}

// Translated from 006ad4c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ad4c0 - the C++ exception frame (FS:[0]) is not translated
/// The `PathSmootherPOVSearch` constructor (by the vtable it sets): the
/// base, the vtable, its members (`+0x2060`, `+0x206c`: `006815c0`;
/// `+0x2084`, `+0x2094`, `+0x20a4`, `+0x20b4`: `006b3570`; `+0x20cc`:
/// `006b35c0`; `+0x20dc`: `006b3610`; `+0x20ec`, `+0x20fc`: `006b36e0`),
/// and its fields cleared. Two copies of the zero point it makes on its
/// stack are not used. Returns `this`.
pub fn fn_006ad4c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(POV_SEARCH_BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_PATH_SMOOTHER_POV_SEARCH);
    e.call(SELF, &args![base + 0x2060]);
    e.call(SELF, &args![base + 0x206c]);
    for offset in [0x2084u32, 0x2094, 0x20a4, 0x20b4] {
        e.call(MEMBER_CONSTRUCT_006B3570, &args![base + offset]);
    }
    e.call(MEMBER_CONSTRUCT_006B35C0, &args![base + 0x20cc]);
    e.call(MEMBER_CONSTRUCT_006B3610, &args![base + 0x20dc]);
    e.call(MEMBER_CONSTRUCT_006B36E0, &args![base + 0x20ec]);
    e.call(MEMBER_CONSTRUCT_006B36E0, &args![base + 0x20fc]);
    e.mem.set_u32(base + 0x2058, 0);
    e.with_stack(0x18, |e, points| {
        for copy in 0..2u32 {
            for i in 0..3u32 {
                let word = e.global::<u32>(POINT_011F426C + 4 * i);
                e.mem.set_u32(points.addr() + 0xc * copy + 4 * i, word);
            }
        }
    });
    e.mem.set_u32(base + 0x207c, 0);
    e.mem.set_u32(base + 0x2080, 0);
    e.mem.set_u32(base + 0x205c, 0);
    e.mem.set_u32(base + 0x20c4, 0);
    e.mem.set_u32(base + 0x20c8, 0);
    e.mem.set_u8(base + 0x2078, 0);
    e.mem.set_u32(base + 0x210c, 0);
    this
}

// Translated from 006ad180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the 2D point at `this` by `scale` in place (`thiscall`; one
/// `float`; `RET 4`): each coordinate is `FLD`, `FMUL` by the argument and
/// `FSTP`. Returns `this`. The engine map gives it no name.
pub fn fn_006ad180(e: &mut Engine, this: Ptr, scale: f32) -> Ptr {
    let base = this.addr();
    let x = x87_mul(x87_load32(e.mem.u32(base)), x87_m32(scale.to_bits()));
    e.mem.set_u32(base, x87_store32(x));
    let y = x87_mul(x87_load32(e.mem.u32(base + 4)), x87_m32(scale.to_bits()));
    e.mem.set_u32(base + 4, x87_store32(y));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_d1b0, fn_006ad1b0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006a_d250, fn_006ad250(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006a_d2f0, fn_006ad2f0(Ptr, u16, u16, u8, u8)),
        entry!(0x006a_d3b0, fn_006ad3b0(f32, bool, bool) -> u32),
        entry!(0x006a_d4c0, fn_006ad4c0(Ptr) -> Ptr),
        entry!(0x006a_d180, fn_006ad180(Ptr, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn scrap_arrays_take_a_heap() {
        let mut e = Engine::new();
        for addr in [
            NODE_ARRAY_BASE_CONSTRUCT,
            LINK_ARRAY_BASE_CONSTRUCT,
            NODE_ARRAY_SETUP,
            LINK_ARRAY_SETUP,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER, |_, _| 0x4400u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        let r = e.call(0x006a_d1b0, &args![this, 4u32, 2u32, 0x7700u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_SCRAP_ARRAY_NODE);
        assert_eq!(e.mem.u32(this + 0x10), 0x7700);
        assert_eq!(calls_to(&e, NODE_ARRAY_SETUP), vec![vec![this, 4, 2]]);
        assert!(calls_to(&e, MEMORY_MANAGER).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x006a_d250, &args![this, 4u32, 2u32, 0u32]);
        assert_eq!(e.mem.u32(this), VTABLE_SCRAP_ARRAY_LINK);
        assert_eq!(e.mem.u32(this + 0x10), 0x4401);
        assert_eq!(calls_to(&e, LINK_ARRAY_SETUP), vec![vec![this, 4, 2]]);
        assert_eq!(calls_to(&e, LINK_ARRAY_BASE_CONSTRUCT), vec![vec![this]]);
    }

    #[test]
    fn side_fields_pack_count_and_flags() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        // Side 0: bits 16..21; count 3, first 1, second 0.
        e.call(0x006a_d2f0, &args![this, 0u16, 3u16, 1u8, 0u8]);
        assert_eq!(e.mem.u32(this + 0xc), 0xffc0_ffff | 3 << 16 | 1 << 20);
        // Side 1: bits 22..27; a count above 15 is 15; first is 0, second 1.
        e.call(0x006a_d2f0, &args![this, 1u16, 20u16, 0u8, 1u8]);
        let expected = (0xffc0_ffff | 3 << 16 | 1 << 20) & !(0x3f << 22) | 15 << 22 | 1 << 27;
        assert_eq!(e.mem.u32(this + 0xc), expected);
        // Count 1: the flags are not stored; count 0: just cleared.
        e.call(0x006a_d2f0, &args![this, 1u16, 1u16, 1u8, 1u8]);
        assert_eq!(e.mem.u32(this + 0xc) >> 22 & 0x3f, 1);
        e.call(0x006a_d2f0, &args![this, 1u16, 0u16, 1u8, 1u8]);
        assert_eq!(e.mem.u32(this + 0xc) >> 22 & 0x3f, 0);
        // Side 2: nothing.
        let before = e.mem.u32(this + 0xc);
        e.call(0x006a_d2f0, &args![this, 2u16, 5u16, 1u8, 1u8]);
        assert_eq!(e.mem.u32(this + 0xc), before);
        // Side 3: the shift 34 wraps to 2.
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x006a_d2f0, &args![this, 3u16, 2u16, 0u8, 0u8]);
        assert_eq!(e.mem.u32(this + 0xc), 2 << 2);
    }

    fn rounding(e: &mut Engine) {
        e.map(0x011d_7000, 0x1000);
        e.mem.set_f32(SCALE_011D7054, 2.0);
        e.map(0x0102_9000, 0x1000);
        e.set_global(QUARTER, 0.25f64);
        e.register(ROUND_DOWN_00406CC0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).floor(),
            ..Ret::default()
        });
        e.register(ROUND_UP_004E4430, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).ceil(),
            ..Ret::default()
        });
    }

    #[test]
    fn rounding_modes() {
        let mut e = Engine::new();
        rounding(&mut e);
        let round = |e: &mut Engine, v: f32, down: bool, nearest: bool| {
            e.call(0x006a_d3b0, &args![v, down, nearest]).u32()
        };
        // 1.05 * 2 = 2.1: fraction below 0.25 rounds down.
        assert_eq!(round(&mut e, 1.05, false, true), 2);
        // 1.2 * 2 = 2.4: up.
        assert_eq!(round(&mut e, 1.2, false, true), 3);
        assert_eq!(round(&mut e, 1.2, true, false), 2);
        assert_eq!(round(&mut e, 1.2, false, false), 3);
        // At most 15 (unsigned: a negative result is large too).
        assert_eq!(round(&mut e, 40.0, true, false), 15);
        assert_eq!(round(&mut e, -3.0, true, false), 15);
    }

    #[test]
    fn pov_search_constructor() {
        let mut e = Engine::new();
        for addr in [
            POV_SEARCH_BASE_CONSTRUCT,
            MEMBER_CONSTRUCT_006B3570,
            MEMBER_CONSTRUCT_006B35C0,
            MEMBER_CONSTRUCT_006B3610,
            MEMBER_CONSTRUCT_006B36E0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SELF, |_, a| a[0].into_ret());
        e.map(0x011f_4000, 0x1000);
        let this = e.mem.alloc(0x2110);
        e.mem.write(this + 0x2050, &[0xee; 0xc0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_d4c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_PATH_SMOOTHER_POV_SEARCH);
        for offset in [0x2058u32, 0x207c, 0x2080, 0x205c, 0x20c4, 0x20c8, 0x210c] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u8(this + 0x2078), 0);
        assert_eq!(e.mem.u8(this + 0x2079), 0xee);
        assert_eq!(calls_to(&e, MEMBER_CONSTRUCT_006B3570).len(), 4);
        assert_eq!(
            calls_to(&e, MEMBER_CONSTRUCT_006B36E0),
            vec![vec![this + 0x20ec], vec![this + 0x20fc]]
        );
    }

    #[test]
    fn fn_006ad180_scales_both_coordinates() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_f32(this, 1.5);
        e.mem.set_f32(this + 4, -3.0);
        e.mem.set_f32(this + 8, 7.0);
        assert_eq!(e.call(0x006a_d180, &args![this, 2.0f32]).u32(), this);
        assert_eq!(e.mem.f32(this), 3.0);
        assert_eq!(e.mem.f32(this + 4), -6.0);
        assert_eq!(e.mem.f32(this + 8), 7.0);
        // The product rounds to a float at the store.
        e.mem.set_f32(this, f32::MAX);
        e.call(0x006a_d180, &args![this, 2.0f32]);
        assert_eq!(e.mem.f32(this), f32::INFINITY);
        // A NaN coordinate stays a NaN, quieted.
        e.mem.set_u32(this + 4, 0x7f80_0001);
        e.call(0x006a_d180, &args![this, 2.0f32]);
        assert_eq!(e.mem.u32(this + 4), 0x7fc0_0001);
    }
}
