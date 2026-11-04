//! `fallout shared/havok/run_0062a3b0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043b4d0` (unnamed in the engine map): `thiscall`, plain `RET`; answers the shift count.
const SHIFT_OF: u32 = 0x0043_b4d0;
/// The CRT's 64-bit left shift `__allshl` (`00ec7810`): the value in `EDX:EAX`, the count in
/// `CL`.
const SHIFT_LEFT_64: u32 = 0x00ec_7810;
/// The low word of the mask `0062a3b0` combines with the shifted bit.
const BASE_MASK: u32 = 0x0002_2757;
/// The table of 64-bit masks `0062a430` fills, indexed by layer.
const LAYER_MASKS: u32 = 0x0126_7f20;
/// `0061f4d0` (unnamed in the engine map): `thiscall`, `RET 4`; the base constructor.
const BASE_CONSTRUCT: u32 = 0x0061_f4d0;
/// `005545d0` (unnamed in the engine map): `thiscall`, plain `RET`; the destructor `0062a4b0`
/// runs.
const DESTRUCT: u32 = 0x0055_45d0;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete of a deleting
/// destructor.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable `0062a480` stores.
const BASE_VTABLE: u32 = 0x0104_e254;
/// The vtable `0062a460` stores.
const DERIVED_VTABLE: u32 = 0x0104_e228;

// Translated from 0062a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, three words. For a `layer` of 0x23 or more (signed),
/// stores the 64-bit mask `low`, `high` in the table at `01267f20`.
pub fn fn_0062a430(e: &mut Engine, layer: i32, low: u32, high: u32) {
    if layer >= 0x23 {
        let slot = (layer as u32).wrapping_mul(8).wrapping_add(LAYER_MASKS);
        e.mem.set_u32(slot, low);
        e.mem.set_u32(slot.wrapping_add(4), high);
    }
}

// Translated from 0062a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. Clears the `float` at `+0x74`, copies the
/// word at `source` to `+0x70`, and records for `layer` (0x27 when below 0x23) the mask
/// `0x22757 | 1 << n` (`0062a430`), `n` being what `0043b4d0` answers for `+0x70`
/// (`__allshl`).
pub fn fn_0062a3b0(e: &mut Engine, this: Ptr, layer: i32, source: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x74), 0.0f32.to_bits());
    let word = e.mem.u32(source.addr());
    e.mem.set_u32(base.wrapping_add(0x70), word);
    let layer = if layer < 0x23 { 0x27 } else { layer };
    let shift = e.call(SHIFT_OF, &args![base.wrapping_add(0x70)]).u32();
    let bit = e.call(SHIFT_LEFT_64, &args![1u64, shift]).u64();
    let mask = bit | u64::from(BASE_MASK);
    fn_0062a430(e, layer, mask as u32, (mask >> 32) as u32);
}

// Translated from 0062a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the base constructor `0061f4d0` with 0, then the
/// vtable `0104e254`. Answers `this`.
pub fn fn_0062a480(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    this
}

// Translated from 0062a460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: `0062a480`, then the vtable `0104e228`. Answers
/// `this`.
pub fn fn_0062a460(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0062a480(e, this);
    e.mem.set_u32(this.addr(), DERIVED_VTABLE);
    this
}

// Translated from 0062a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. A deleting destructor: `005545d0`, then with
/// bit 0 of `flags` the delete `00401030`. Answers `this`.
pub fn fn_0062a4b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_a3b0, fn_0062a3b0(Ptr, i32, Ptr)),
        entry!(0x0062_a430, fn_0062a430(i32, u32, u32)),
        entry!(0x0062_a460, fn_0062a460(Ptr) -> Ptr),
        entry!(0x0062_a480, fn_0062a480(Ptr) -> Ptr),
        entry!(0x0062_a4b0, fn_0062a4b0(Ptr, u32) -> Ptr),
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(0x0126_7000, 0x2000);
        // `__allshl` lives in another unit: a double that shifts.
        e.register(SHIFT_LEFT_64, |_, a| {
            let value = u64::from(a[0]) | u64::from(a[1]) << 32;
            let count = a[2] & 0xff;
            let shifted = if count >= 64 { 0 } else { value << count };
            shifted.into_ret()
        });
        e
    }

    #[test]
    fn at_0062a430_stores_only_from_layer_0x23() {
        let mut e = engine();
        e.call(0x0062_a430, &args![0x22i32, 1u32, 2u32]);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x22 * 8), 0);
        e.call(0x0062_a430, &args![-1i32, 1u32, 2u32]);
        e.call(0x0062_a430, &args![0x23i32, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x23 * 8), 0x11);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x23 * 8 + 4), 0x22);
    }

    #[test]
    fn at_0062a3b0_records_the_mask_for_the_layer() {
        let mut e = engine();
        e.register(SHIFT_OF, |_, _| 40u32.into_ret());
        let this = e.mem.alloc(0x78);
        e.mem.set_u32(this + 0x74, 0x1234);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x5555);
        e.call(0x0062_a3b0, &args![this, 0x30i32, source]);
        assert_eq!(e.mem.u32(this + 0x70), 0x5555);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(calls(&e, SHIFT_OF), vec![vec![this + 0x70]]);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x30 * 8), 0x2_2757);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x30 * 8 + 4), 0x100);
    }

    #[test]
    fn at_0062a3b0_uses_layer_0x27_below_0x23() {
        let mut e = engine();
        e.register(SHIFT_OF, |_, _| 3u32.into_ret());
        let this = e.mem.alloc(0x78);
        let source = e.mem.alloc(4);
        e.call(0x0062_a3b0, &args![this, 5i32, source]);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x27 * 8), 0x2_275f);
        assert_eq!(e.mem.u32(LAYER_MASKS + 0x27 * 8 + 4), 0);
        assert_eq!(e.mem.u32(LAYER_MASKS + 5 * 8), 0);
    }

    #[test]
    fn at_0062a460_constructs_base_then_derived() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0062_a460, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), DERIVED_VTABLE);
        assert_eq!(calls(&e, BASE_CONSTRUCT), vec![vec![this, 0]]);
        e.call(0x0062_a480, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn at_0062a4b0_deletes_only_with_bit_0() {
        let mut e = engine();
        e.register(DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_a4b0, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0062_a4b0, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
    }
}
