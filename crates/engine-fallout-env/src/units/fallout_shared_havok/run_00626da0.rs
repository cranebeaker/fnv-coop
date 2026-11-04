//! `fallout shared/havok/run_00626da0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c82870` (unnamed in the engine map): `cdecl`, a layer and a 64-bit mask (two words).
const SET_LAYER_MASK: u32 = 0x00c8_2870;
/// `00553300` (`fallout shared/tesobjectcell.cpp` in the engine map): `cdecl`, the size; the
/// allocation.
const ALLOCATE: u32 = 0x0055_3300;
/// `005dbe10` (`fallout shared/tesscriptfunctions.cpp` in the engine map): no arguments.
const CASTER_FILTER: u32 = 0x005d_be10;
/// `00403e20` (`fallout shared/magic/effectitem.cpp` in the engine map): `thiscall` on a setting,
/// plain `RET`; answers the address of its `float`.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `006268b0` (`run_00626790.rs`): `thiscall`, `RET 8`, a `float` and a word; the caster's
/// constructor.
const CASTER_CONSTRUCT: u32 = 0x0062_68b0;
/// `00626930` (`run_00626790.rs`): `thiscall`, plain `RET`; the caster's destructor.
const CASTER_DESTRUCT: u32 = 0x0062_6930;
/// `0087e9b0` (`fallout/ai/actor.cpp` in the engine map): `cdecl`, one word; the delete.
const CASTER_DELETE: u32 = 0x0087_e9b0;
/// The cached caster of kind 0.
const FIRST_CASTER: u32 = 0x011c_c2f8;
/// The cached caster of kind 1.
const SECOND_CASTER: u32 = 0x011c_c2fc;
/// The setting whose `float` is the radius of the kind 0 caster.
const FIRST_RADIUS: u32 = 0x011c_c30c;
/// The setting whose `float` is the radius of the kind 1 caster.
const SECOND_RADIUS: u32 = 0x011c_c320;
/// `004968b0` (`fallout shared/animation.cpp` in the engine map): `thiscall`, plain `RET`; the
/// base constructor.
const BASE_CONSTRUCT: u32 = 0x0049_68b0;
/// `LoadedAreaBound`'s vtable (RTTI `.?AVLoadedAreaBound@@`).
const LOADED_AREA_BOUND_VTABLE: u32 = 0x0104_e130;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `RET 0x14`): array, element
/// size, count, constructor, destructor.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `006694e0` (`fallout shared/speedtree/bstreemodel.cpp` in the engine map): the element
/// constructor handed to the iterator.
const POINTER_ELEMENT_CONSTRUCT: u32 = 0x0066_94e0;
/// `0045cec0` (unnamed in the engine map): the element destructor handed to the iterator.
const POINTER_ELEMENT_DESTRUCT: u32 = 0x0045_cec0;
/// `006299f0` (`fallout shared/havok/loadedareabound.cpp` in the engine map): `thiscall`,
/// `RET 4`.
const MEMBER_CONSTRUCT: u32 = 0x0062_99f0;
/// `0066b0d0` (unnamed in the engine map): `thiscall` on a smart pointer, `RET 4`; assigns a
/// word.
const POINTER_SET: u32 = 0x0066_b0d0;

// Translated from 00626da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall` (`this` is not read): sets the collision mask of layer
/// 0x27 to `0x6016777e` (`00c82870`, high word 0).
pub fn fn_00626da0(e: &mut Engine, _this: Ptr) {
    e.call(SET_LAYER_MASK, &args![0x27u32, 0x6016_777eu32, 0u32]);
}

// Translated from 00626dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626dd0 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `cdecl`, a word and a byte. Answers the cached caster of `kind` 0
/// (`011cc2f8`) or 1 (`011cc2fc`), 0 for any other kind. When the cache is empty and `create` is
/// set, allocates 0x3b0 bytes (`00553300`) and constructs a caster in them (`006268b0` with the
/// `float` of the setting `011cc30c` or `011cc320` and what `005dbe10` answers), or stores 0 when
/// the allocation fails.
pub fn fn_00626dd0(e: &mut Engine, kind: u32, create: u8) -> u32 {
    let (cache, radius) = match kind {
        0 => (FIRST_CASTER, FIRST_RADIUS),
        1 => (SECOND_CASTER, SECOND_RADIUS),
        _ => return 0,
    };
    if e.mem.u32(cache) == 0 && create != 0 {
        let block = e.call(ALLOCATE, &args![0x3b0u32]).u32();
        let caster = if block == 0 {
            0
        } else {
            let filter = e.call(CASTER_FILTER, &args![]).u32();
            let value = e.call(SETTING_VALUE, &args![radius]).u32();
            let radius = x87_quiet_bits(e.mem.u32(value));
            e.call(CASTER_CONSTRUCT, &args![block, radius, filter])
                .u32()
        };
        e.mem.set_u32(cache, caster);
    }
    e.mem.u32(cache)
}

// Translated from 00626fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. The caster's deleting destructor:
/// `00626930`, then with bit 0 of `flags` the delete `0087e9b0`. Answers `this`.
pub fn fn_00626fa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CASTER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(CASTER_DELETE, &args![this]);
    }
    this
}

// Translated from 00626f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. No arguments. Deletes the cached kind 1 caster (`011cc2fc`), then
/// the kind 0 one (`011cc2f8`), each through `00626fa0` with 1 when set, and clears both.
pub fn fn_00626f10(e: &mut Engine) {
    for cache in [SECOND_CASTER, FIRST_CASTER] {
        let caster = e.mem.u32(cache);
        if caster != 0 {
            fn_00626fa0(e, Ptr::new(caster), 1);
        }
        e.mem.set_u32(cache, 0);
    }
}

// Translated from 00626fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626fd0 - C++ exception unwinding (FS:[0] frame) not translated
/// `LoadedAreaBound`'s constructor (RTTI `.?AVLoadedAreaBound@@`, vtable `0104e130`).
/// `thiscall`, `RET 0x10`, four `float`s. The base (`004968b0`), the vtable, six smart pointers
/// at `+8` (`eh vector constructor iterator` with `006694e0` / `0045cec0`), 0 at `+0x20`, the
/// member at `+0x24` with 0x25 (`006299f0`), the four `float`s at `+0x34` to `+0x40`, then each
/// of the six pointers set to 0 (`0066b0d0`). Answers `this`.
pub fn fn_00626fd0(e: &mut Engine, this: Ptr, a: f32, b: f32, c: f32, d: f32) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, LOADED_AREA_BOUND_VTABLE);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base.wrapping_add(8),
            4u32,
            6u32,
            POINTER_ELEMENT_CONSTRUCT,
            POINTER_ELEMENT_DESTRUCT
        ],
    );
    e.mem.set_u32(base.wrapping_add(0x20), 0);
    e.call(MEMBER_CONSTRUCT, &args![base.wrapping_add(0x24), 0x25u32]);
    for (offset, value) in [(0x34u32, a), (0x38, b), (0x3c, c), (0x40, d)] {
        e.mem
            .set_u32(base.wrapping_add(offset), x87_quiet_bits(value.to_bits()));
    }
    let mut index = 0i32;
    while index < 6 {
        let pointer = base.wrapping_add(8).wrapping_add(index as u32 * 4);
        e.call(POINTER_SET, &args![pointer, 0u32]);
        index += 1;
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_6da0, fn_00626da0(Ptr)),
        entry!(0x0062_6dd0, fn_00626dd0(u32, u8) -> u32),
        entry!(0x0062_6f10, fn_00626f10()),
        entry!(0x0062_6fa0, fn_00626fa0(Ptr, u32) -> Ptr),
        entry!(0x0062_6fd0, fn_00626fd0(Ptr, f32, f32, f32, f32) -> Ptr),
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
        e.map(0x011c_c000, 0x1000);
        e
    }

    #[test]
    fn at_00626da0_sets_the_layer_mask() {
        let mut e = engine();
        e.register(SET_LAYER_MASK, |_, _| Ret::default());
        e.call(0x0062_6da0, &args![0x4000u32]);
        assert_eq!(calls(&e, SET_LAYER_MASK), vec![vec![0x27, 0x6016_777e, 0]]);
    }

    fn caster_doubles(e: &mut Engine, block: u32) {
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(CASTER_FILTER, |_, _| 0x66u32.into_ret());
        e.register(SETTING_VALUE, |_, a| (a[0] + 0x40).into_ret());
        e.set_global(FIRST_RADIUS + 0x40, 5.0f32);
        e.set_global(SECOND_RADIUS + 0x40, 0x7f80_0001u32);
        e.register(CASTER_CONSTRUCT, |_, a| a[0].into_ret());
    }

    #[test]
    fn at_00626dd0_creates_each_kind_once() {
        let mut e = engine();
        caster_doubles(&mut e, 0x8000);
        assert_eq!(e.call(0x0062_6dd0, &args![0u32, 0u32]).u32(), 0);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(e.call(0x0062_6dd0, &args![0u32, 1u32]).u32(), 0x8000);
        assert_eq!(e.call(0x0062_6dd0, &args![0u32, 1u32]).u32(), 0x8000);
        assert_eq!(
            calls(&e, CASTER_CONSTRUCT),
            vec![vec![0x8000, 5.0f32.to_bits(), 0x66]]
        );
        assert_eq!(e.call(0x0062_6dd0, &args![1u32, 1u32]).u32(), 0x8000);
        assert_eq!(
            calls(&e, CASTER_CONSTRUCT)[1],
            vec![0x8000, 0x7fc0_0001, 0x66]
        );
        assert_eq!(e.mem.u32(FIRST_CASTER), 0x8000);
        assert_eq!(e.mem.u32(SECOND_CASTER), 0x8000);
        assert_eq!(e.call(0x0062_6dd0, &args![2u32, 1u32]).u32(), 0);
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
    }

    #[test]
    fn at_00626dd0_stores_null_when_the_allocation_fails() {
        let mut e = engine();
        caster_doubles(&mut e, 0);
        assert_eq!(e.call(0x0062_6dd0, &args![1u32, 1u32]).u32(), 0);
        assert!(calls(&e, CASTER_FILTER).is_empty());
        assert!(calls(&e, CASTER_CONSTRUCT).is_empty());
    }

    #[test]
    fn at_00626f10_deletes_and_clears_both_casters() {
        let mut e = engine();
        e.register(CASTER_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_DELETE, |_, _| Ret::default());
        e.mem.set_u32(FIRST_CASTER, 0x8000);
        e.mem.set_u32(SECOND_CASTER, 0x9000);
        e.call(0x0062_6f10, &args![]);
        assert_eq!(calls(&e, CASTER_DELETE), vec![vec![0x9000], vec![0x8000]]);
        assert_eq!(e.mem.u32(FIRST_CASTER), 0);
        assert_eq!(e.mem.u32(SECOND_CASTER), 0);
        e.call(0x0062_6f10, &args![]);
        assert_eq!(calls(&e, CASTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn at_00626fa0_deletes_only_with_bit_0() {
        let mut e = engine();
        e.register(CASTER_DESTRUCT, |_, _| Ret::default());
        e.register(CASTER_DELETE, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_6fa0, &args![0x8000u32, 0u32]).u32(), 0x8000);
        assert_eq!(calls(&e, CASTER_DESTRUCT), vec![vec![0x8000]]);
        assert!(calls(&e, CASTER_DELETE).is_empty());
    }

    #[test]
    fn at_00626fd0_constructs_a_loaded_area_bound() {
        let mut e = engine();
        for addr in [
            BASE_CONSTRUCT,
            VECTOR_CONSTRUCTOR_ITERATOR,
            MEMBER_CONSTRUCT,
            POINTER_SET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + 0x20, 0x55);
        let nan = f32::from_bits(0x7f80_0001);
        let answer = e.call(0x0062_6fd0, &args![this, 1.0f32, 2.0f32, 3.0f32, nan]);
        assert_eq!(answer.u32(), this);
        assert_eq!(e.mem.u32(this), LOADED_AREA_BOUND_VTABLE);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                this + 8,
                4,
                6,
                POINTER_ELEMENT_CONSTRUCT,
                POINTER_ELEMENT_DESTRUCT
            ]]
        );
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(calls(&e, MEMBER_CONSTRUCT), vec![vec![this + 0x24, 0x25]]);
        assert_eq!(e.mem.f32(this + 0x34), 1.0);
        assert_eq!(e.mem.f32(this + 0x3c), 3.0);
        assert_eq!(e.mem.u32(this + 0x40), 0x7fc0_0001);
        let set: Vec<u32> = calls(&e, POINTER_SET).iter().map(|a| a[0]).collect();
        assert_eq!(set, (0..6).map(|i| this + 8 + 4 * i).collect::<Vec<_>>());
    }
}
