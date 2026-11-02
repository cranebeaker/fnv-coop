//! `BSHavok/bhkconvexshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConvexShape` is a `bhkShape` (0x14 bytes): its Havok shape is at `+8` and its
//! construction info at `+0xC`. The construction info that virtual slot `0xB4` answers holds
//! the radius at `+4`; the Havok shape keeps its radius at `+0x10`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{add_viewer_string, enter_scope, leave_scope};

/// `bhkSerializable::Kill` (`00c876a0`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// The `bhkSphereRepShape` destructor body (`00ca0540`) and `GetViewerStrings` (`00ca05b0`).
const SPHERE_REP_SHAPE_DESTRUCT: u32 = 0x00ca_0540;
const SPHERE_REP_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00ca_05b0;
/// The `bhkShape` methods `CopyCinfo` (`00c84e20`), `LoadBinary` (`00c84d00`) and
/// `SaveBinary` (`00c84d50`).
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
/// `NiGetViewerString` for a text (`00a7d330`) and for a float under a name (`00a7d550`);
/// both cdecl.
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The vtable written first by the destructor (RTTI-confirmed `bhkConvexShape`).
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
/// The live-object count of `bhkConvexShape`.
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_8254;
/// The float `1.0` default radius at `010c72bc` and the scale at `011b15c0`.
const DEFAULT_RADIUS: u32 = 0x010c_72bc;
const RADIUS_SCALE: u32 = 0x011b_15c0;
/// The name the radius viewer string is made under.
const RADIUS_NAME: u32 = 0x0101_5a9c;

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The bits of the radius of `this`: the Havok shape's float at `+0x10` through the x87
/// stack, or the default when there is no `this` or no Havok shape.
fn radius_bits(e: &Engine, this: Ptr) -> u32 {
    let shape = if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if shape == 0 {
        x87_quiet_bits(e.mem.u32(DEFAULT_RADIUS))
    } else {
        x87_quiet_bits(e.mem.u32(shape + 0x10))
    }
}

// Translated from 00c9fb00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9fb00 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexShape::~bhkConvexShape` (Ghidra name): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `01268250` and runs the
/// `bhkSphereRepShape` destructor body (`00ca0540`).
pub fn bhk_convex_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_CONVEX_SHAPE);
    e.set_global(COUNT_CONVEX_SHAPE, count.wrapping_sub(1));
    e.call(SPHERE_REP_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00c9fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape::CopyCinfo` (Ghidra name): `bhkShape::CopyCinfo`, then the radius into the
/// cinfo's `+4`: the Havok shape's float at `+0x10`, or the default at `010c72bc` when
/// there is no `this` or no Havok shape (both through the x87 stack).
pub fn bhk_convex_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(SHAPE_COPY_CINFO, &args![this, cinfo]);
    let radius = radius_bits(e, this);
    e.mem.set_u32(cinfo + 4, radius);
}

// Translated from 00c9fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape::LoadBinary` (Ghidra name): `bhkShape::LoadBinary`, then, when the
/// stream's byte at `+0x1E5` is not 1, the cinfo's radius (`+4`) is byte-swapped.
pub fn bhk_convex_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let cinfo = cinfo_of(e, this);
        let radius = e.mem.u32(cinfo + 4);
        e.mem.set_u32(cinfo + 4, radius.swap_bytes());
    }
}

// Translated from 00c9fc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape::SaveBinary` (Ghidra name): takes the cinfo, byte-swaps its radius
/// (`+4`) when the stream's byte at `+0x1E4` is not 1, runs `bhkShape::SaveBinary`, and
/// swaps the radius back under the same condition.
pub fn bhk_convex_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let radius = e.mem.u32(cinfo + 4);
        e.mem.set_u32(cinfo + 4, radius.swap_bytes());
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let radius = e.mem.u32(cinfo + 4);
        e.mem.set_u32(cinfo + 4, radius.swap_bytes());
    }
}

// Translated from 00c9fc50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9fc50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexShape::GetViewerStrings` (Ghidra name): in the Havok allocation scope runs
/// `bhkSphereRepShape::GetViewerStrings`, then appends the viewer string of the text in the
/// global at `01268254` and the viewer string made under the name at `01015a9c` from the
/// radius (see `CopyCinfo`) times the float at `011b15c0`.
pub fn bhk_convex_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(SPHERE_REP_SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    let radius = f32::from_bits(radius_bits(e, this));
    let scaled = radius * e.global::<f32>(RADIUS_SCALE);
    let text = e
        .call(VIEWER_STRING_FLOAT, &args![RADIUS_NAME, scaled.to_bits()])
        .u32();
    add_viewer_string(e, list, text);
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9fb00, bhk_convex_shape_destructor(Ptr)),
        entry!(0x00c9fb70, bhk_convex_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00c9fbc0, bhk_convex_shape_load_binary(Ptr, u32)),
        entry!(0x00c9fc00, bhk_convex_shape_save_binary(Ptr, u32)),
        entry!(0x00c9fc50, bhk_convex_shape_get_viewer_strings(Ptr, u32)),
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
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A shape whose virtual slot `0xB4` answers `cinfo`.
    fn shape_with_cinfo(e: &mut Engine, cinfo: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0xb4, 0x0900_0100);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0xc, cinfo);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        this
    }

    #[test]
    fn destructor_sets_the_vtable_and_counts_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_CONVEX_SHAPE, 4u32);
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(SPHERE_REP_SHAPE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00c9fb00, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_CONVEX_SHAPE), 3);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![0x00c9fb00, SERIALIZABLE_KILL, SPHERE_REP_SHAPE_DESTRUCT]
        );
        assert_eq!(calls_to(&e, SPHERE_REP_SHAPE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn copy_cinfo_takes_the_havok_radius_quieted_or_the_default() {
        let mut e = Engine::new();
        e.map(DEFAULT_RADIUS, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x3f80_0000);
        e.register(SHAPE_COPY_CINFO, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        let havok = e.mem.alloc(0x20);
        let cinfo = e.mem.alloc(0x20);
        // A signalling NaN radius comes out quiet.
        e.mem.set_u32(havok + 0x10, 0x7f80_0001);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00c9fb70, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x7fc0_0001);
        assert_eq!(calls_to(&e, SHAPE_COPY_CINFO), vec![vec![this, cinfo]]);
        // No Havok shape: the default; no shape object at all: also the default.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c9fb70, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x3f80_0000);
        e.mem.set_u32(cinfo + 4, 0);
        e.call(0x00c9fb70, &args![0u32, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x3f80_0000);
    }

    #[test]
    fn load_binary_swaps_the_radius_unless_the_stream_byte_is_one() {
        let mut e = Engine::new();
        e.register(SHAPE_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, 0x0102_0304);
        let this = shape_with_cinfo(&mut e, cinfo);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00c9fbc0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x0102_0304);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00c9fbc0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x0403_0201);
        assert_eq!(calls_to(&e, SHAPE_LOAD_BINARY), vec![vec![this, stream]; 2]);
    }

    #[test]
    fn save_binary_swaps_the_radius_around_the_base_save() {
        let mut e = Engine::new();
        // The base save sees the swapped value.
        e.register(SHAPE_SAVE_BINARY, |e, a| {
            let cinfo = e.mem.u32(a[0] + 0xc);
            let seen = e.mem.u32(cinfo + 4);
            e.mem.set_u32(cinfo + 8, seen);
            Ret::default()
        });
        let cinfo = e.mem.alloc(0x20);
        e.mem.set_u32(cinfo + 4, 0x0102_0304);
        let this = shape_with_cinfo(&mut e, cinfo);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00c9fc00, &args![this, stream]);
        assert_eq!(
            (e.mem.u32(cinfo + 4), e.mem.u32(cinfo + 8)),
            (0x0102_0304, 0x0102_0304)
        );
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00c9fc00, &args![this, stream]);
        assert_eq!(
            (e.mem.u32(cinfo + 4), e.mem.u32(cinfo + 8)),
            (0x0102_0304, 0x0403_0201)
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text_and_the_scaled_radius() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(DEFAULT_RADIUS, 4);
        e.map(RADIUS_SCALE, 4);
        e.mem.set_u32(DEFAULT_RADIUS, 0x3f80_0000);
        e.set_global(RADIUS_SCALE, 2.0f32);
        e.set_global(CLASS_TEXT, 0x1111u32);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        e.register(SPHERE_REP_SHAPE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VIEWER_STRING_FLOAT, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(0x0096_ad30, |_, _| Ret::default());
        // The list stores each added text at its index.
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00c9fc50, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(e.mem.u32(list + 0x104), 2.0f32.to_bits());
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![vec![RADIUS_NAME, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, SPHERE_REP_SHAPE_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // With a Havok radius of 3: 3 * 2 = 6.
        let havok = e.mem.alloc(0x20);
        e.mem.set_f32(havok + 0x10, 3.0);
        e.mem.set_u32(this + 8, havok);
        e.mem.set_u16(list + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c9fc50, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x104), 6.0f32.to_bits());
    }
}
