//! `Havok SDK/hkpphantomcallbackshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkReferencedObject::calcContentStatistics` (`00c90360`, `this`, collector, class record).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The class record `calcContentStatistics` hands on (`01268910`).
const PHANTOM_CALLBACK_SHAPE_CLASS: u32 = 0x0126_8910;

// Translated from 00c9e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomCallbackShape::castRayWithCollectorImpl` (Xbox PDB): `ECX` = shape and three
/// stack words; the body is empty.
pub fn hkp_phantom_callback_shape_cast_ray_with_collector_impl(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

// Translated from 00c9e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomCallbackShape::calcContentStatistics` (Xbox PDB): `ECX` = shape and two stack
/// words, the collector and a second word the function never reads (it jumps to the base
/// function with the class record `01268910` in its place). Calls
/// `hkReferencedObject::calcContentStatistics` (`00c90360`) with `(this, collector, 01268910)`.
pub fn hkp_phantom_callback_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, PHANTOM_CALLBACK_SHAPE_CLASS],
    );
}

// Translated from 00c9e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomCallbackShape::getAabbImpl` (Xbox PDB): `ECX` = shape and three stack words, of
/// which the third, `out`, is the box: its 32 bytes (two 16-byte stores) are set to zero.
pub fn hkp_phantom_callback_shape_get_aabb_impl(
    e: &mut Engine,
    _this: Ptr,
    _transform: u32,
    _tolerance: u32,
    out: u32,
) {
    for offset in 0..8 {
        e.mem.set_u32(out + offset * 4, 0);
    }
}

// Translated from 00c9e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomCallbackShape::castRayImpl` (Xbox PDB): `ECX` = shape and three stack words, of
/// which the first, `result`, is where a byte 0 (no hit) is stored. Nothing is returned (`EAX`
/// is left as it was).
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_phantom_callback_shape_cast_ray_impl(
    e: &mut Engine,
    _this: Ptr,
    result: u32,
    _input: u32,
    _output: u32,
) -> u32 {
    e.mem.set_u8(result, 0);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9e390,
            hkp_phantom_callback_shape_cast_ray_with_collector_impl(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00c9e3a0,
            hkp_phantom_callback_shape_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(
            0x00c9e3b0,
            hkp_phantom_callback_shape_get_aabb_impl(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00c9e3d0,
            hkp_phantom_callback_shape_cast_ray_impl(Ptr, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_ray_with_collector_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00c9_e390, &args![0x10u32, 1u32, 2u32, 3u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn calc_content_statistics_hands_the_class_record_to_the_base_function() {
        let mut e = Engine::new();
        e.register(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c9_e3a0, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(
            log[1],
            (
                REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                args![0x1000u32, 0x2000u32, PHANTOM_CALLBACK_SHAPE_CLASS].to_vec()
            )
        );
    }

    #[test]
    fn get_aabb_zeroes_the_box() {
        let mut e = Engine::new();
        let out = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u32(out + i * 4, 0xdead_beef);
        }
        e.call(0x00c9_e3b0, &args![0x10u32, 0x20u32, 0x30u32, out]);
        for i in 0..8 {
            assert_eq!(e.mem.u32(out + i * 4), 0);
        }
        // Only the 32 bytes of the box.
        assert_eq!(e.mem.u32(out + 0x20), 0xdead_beef);
    }

    #[test]
    fn cast_ray_stores_no_hit() {
        let mut e = Engine::new();
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 1);
        e.call(0x00c9_e3d0, &args![0x10u32, result, 2u32, 3u32]);
        assert_eq!(e.mem.u8(result), 0);
    }

    #[test]
    fn fn_00c9e3d0_answers_its_result_pointer() {
        let mut e = Engine::new();
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 1);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00c9_e3d0, &args![0x10u32, result, 2u32, 3u32])
                .u32(),
            args![result][0]
        );
    }
}
