//! `Havok SDK/hkdisplaycapsule.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkGeometryUtils::createCapsuleGeometry` (`00da16d0`, cdecl, seven words: the start
/// and the end of the capsule, the radius as a float, the two counts, the transform and
/// the geometry pointer to fill).
const CREATE_CAPSULE_GEOMETRY: u32 = 0x00da_16d0;
/// The vtable of `hkDisplayCapsule`.
const VTABLE: u32 = 0x010c_e2cc;

/// The exe moves 16-byte vectors with `MOVAPS`, which faults on an address that is not 16-byte
/// aligned.
fn assert_aligned(address: u32) {
    assert_eq!(
        address & 0xf,
        0,
        "MOVAPS address {address:#010x} is not 16-byte aligned"
    );
}

/// A `MOVAPS` store of four words.
fn store_vector(e: &mut Engine, address: u32, lanes: [u32; 4]) {
    assert_aligned(address);
    for (index, lane) in lanes.iter().enumerate() {
        e.mem.set_u32(address + 4 * index as u32, *lane);
    }
}

/// A `MOVAPS` copy of 16 bytes.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    assert_aligned(from);
    assert_aligned(to);
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 00d22d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCapsule::buildGeometry` (Xbox PDB; `this` = the display capsule, nothing else):
/// calls `hkGeometryUtils::createCapsuleGeometry` (`00da16d0`, cdecl) with the start (`this +
/// 0x60`), the end (`this + 0x70`), the radius (the float at `+0x80`, loaded and stored again
/// so a signalling NaN comes out quiet), the words at `+0x88` and `+0x84`, the transform
/// (`this + 0x10`) and the geometry pointer slot (`this + 8`).
pub fn hk_display_capsule_build_geometry(e: &mut Engine, this: Ptr) {
    let capsule = this.addr();
    let radius = x87_quiet(e.mem.f32(capsule + 0x80));
    let count_88 = e.mem.u32(capsule + 0x88);
    let count_84 = e.mem.u32(capsule + 0x84);
    e.call(
        CREATE_CAPSULE_GEOMETRY,
        &args![
            capsule + 0x60,
            capsule + 0x70,
            radius.to_bits(),
            count_88,
            count_84,
            capsule + 0x10,
            capsule + 8
        ],
    );
}

// Translated from 00d22db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCapsule::getWireframeGeometry` (Xbox PDB; `this` = the display capsule, then
/// two words, `RET 8`): does nothing, a capsule has no wireframe.
pub fn hk_display_capsule_get_wireframe_geometry(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00d22dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayCapsule::hkDisplayCapsule` (Xbox PDB; `this` = the display capsule, then `(start,
/// end, radius, field_84, field_88)`, `RET 0x14`; start and end are pointers to 16-byte
/// aligned vectors): word `0` at `+8`, 8 at `+0x50`, 1 at `+6` (a word), the 4x4 transform at
/// `+0x10` as the identity (zero vectors with 1.0 stored at `+0x10`, `+0x24` and `+0x38`), a
/// zero vector at `+0x40`, the words `field_84` and `field_88` to `+0x84` and `+0x88`, the
/// vtable `0x010CE2CC`, the start and end vectors (`MOVAPS`) to `+0x60` and `+0x70` and the
/// radius (loaded as a float, so a signalling NaN is quieted) to `+0x80`. Answers `this`.
pub fn hk_display_capsule_construct(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    end: Ptr,
    radius: f32,
    field_84: u32,
    field_88: u32,
) -> Ptr {
    let capsule = this.addr();
    assert_aligned(capsule);
    e.mem.set_u32(capsule + 8, 0);
    e.mem.set_u32(capsule + 0x50, 8);
    e.mem.set_u16(capsule + 6, 1);
    for offset in [0x10, 0x20, 0x30] {
        store_vector(e, capsule + offset, [0; 4]);
    }
    for offset in [0x10, 0x24, 0x38] {
        e.mem.set_u32(capsule + offset, 1.0f32.to_bits());
    }
    store_vector(e, capsule + 0x40, [0; 4]);
    e.mem.set_u32(capsule + 0x84, field_84);
    e.mem.set_u32(capsule + 0x88, field_88);
    e.mem.set_u32(capsule, VTABLE);
    copy_vector(e, start.addr(), capsule + 0x60);
    copy_vector(e, end.addr(), capsule + 0x70);
    e.mem.set_f32(capsule + 0x80, x87_quiet(radius));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_2d70, hk_display_capsule_build_geometry(Ptr)),
        entry!(
            0x00d2_2db0,
            hk_display_capsule_get_wireframe_geometry(Ptr, u32, u32)
        ),
        entry!(0x00d2_2dc0, hk_display_capsule_construct(Ptr, Ptr, Ptr, f32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 16-byte aligned block of `size` zeroed bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    fn floats(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// A capsule block at an aligned address, its words dirtied.
    fn dirty_capsule(e: &mut Engine) -> u32 {
        let this = aligned_block(e, 0x90);
        for offset in (0..0x90).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        this
    }

    /// Two aligned vectors, `(1, 2, 3, 4)` and `(5, 6, 7, 8)`.
    fn endpoints(e: &mut Engine) -> (u32, u32) {
        let start = aligned_block(e, 16);
        let end = aligned_block(e, 16);
        for i in 0..4 {
            e.mem.set_f32(start + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(end + 4 * i, 5.0 + i as f32);
        }
        (start, end)
    }

    #[test]
    fn construct_sets_up_the_capsule() {
        let mut e = Engine::new();
        let this = dirty_capsule(&mut e);
        let (start, end) = endpoints(&mut e);
        let got = e
            .call(
                0x00d2_2dc0,
                &args![this, start, end, 0.5f32, 0x1111u32, 0x2222u32],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_e2cc);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 8);
        assert_eq!(words(&e, this + 0x10, 4), floats([1.0, 0.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x20, 4), floats([0.0, 1.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x30, 4), floats([0.0, 0.0, 1.0, 0.0]));
        assert_eq!(words(&e, this + 0x40, 4), vec![0; 4]);
        assert_eq!(words(&e, this + 0x60, 4), floats([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(words(&e, this + 0x70, 4), floats([5.0, 6.0, 7.0, 8.0]));
        assert_eq!(e.mem.f32(this + 0x80), 0.5);
        assert_eq!(e.mem.u32(this + 0x84), 0x1111);
        assert_eq!(e.mem.u32(this + 0x88), 0x2222);
        // Words the constructor does not touch.
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x8c), 0xdead_beef);
    }

    #[test]
    fn construct_quiets_a_signalling_nan_radius() {
        let mut e = Engine::new();
        let this = dirty_capsule(&mut e);
        let (start, end) = endpoints(&mut e);
        e.call(
            0x00d2_2dc0,
            &args![this, start, end, f32::from_bits(0x7f80_0001), 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(this + 0x80), 0x7fc0_0001);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn construct_faults_on_an_unaligned_vector() {
        let mut e = Engine::new();
        let this = dirty_capsule(&mut e);
        let (start, end) = endpoints(&mut e);
        e.call(
            0x00d2_2dc0,
            &args![this, start + 4, end, 0.5f32, 0u32, 0u32],
        );
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn construct_faults_on_an_unaligned_capsule() {
        let mut e = Engine::new();
        let this = dirty_capsule(&mut e) + 8;
        let (start, end) = endpoints(&mut e);
        e.call(0x00d2_2dc0, &args![this, start, end, 0.5f32, 0u32, 0u32]);
    }

    #[test]
    fn build_geometry_passes_the_capsule_parts_to_the_geometry_builder() {
        let mut e = Engine::new();
        e.register(CREATE_CAPSULE_GEOMETRY, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        let this = dirty_capsule(&mut e);
        e.mem.set_f32(this + 0x80, 0.25);
        e.mem.set_u32(this + 0x84, 0x1111);
        e.mem.set_u32(this + 0x88, 0x2222);
        e.call(0x00d2_2d70, &args![this]);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == CREATE_CAPSULE_GEOMETRY)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            calls,
            vec![vec![
                this + 0x60,
                this + 0x70,
                0.25f32.to_bits(),
                0x2222,
                0x1111,
                this + 0x10,
                this + 8
            ]]
        );
    }

    #[test]
    fn build_geometry_hands_over_a_quieted_radius() {
        let mut e = Engine::new();
        e.register(CREATE_CAPSULE_GEOMETRY, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        let this = dirty_capsule(&mut e);
        e.mem.set_u32(this + 0x80, 0x7f80_0001);
        e.call(0x00d2_2d70, &args![this]);
        let radius = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == CREATE_CAPSULE_GEOMETRY)
            .map(|(_, w)| w[2])
            .unwrap();
        assert_eq!(radius, 0x7fc0_0001);
        // The slot in the capsule itself is left as it was.
        assert_eq!(e.mem.u32(this + 0x80), 0x7f80_0001);
    }

    #[test]
    fn get_wireframe_geometry_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        let this = dirty_capsule(&mut e);
        e.call(0x00d2_2db0, &args![this, 0x1234u32, 0x5678u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 8), 0xdead_beef);
    }
}
